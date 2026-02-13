#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness,
    clippy::suspicious,
    clippy::unwrap_used,
    clippy::expect_used
)]

use async_trait::async_trait;
use pingora::http::{RequestHeader, ResponseHeader};
use pingora::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

mod static_files;
pub use static_files::{StaticFileConfig, serve_static_file};

/// Lock-free rate limiter using atomic operations.
///
/// Performance: Only 2 atomic ops in hot path (load + `fetch_add`).
/// Window reset uses CAS but only contends once per second.
struct RateLimiter {
    window_start: AtomicU64, // Unix timestamp of current window (seconds)
    counter: AtomicU64,      // Request count in current window
}

impl RateLimiter {
    const fn new() -> Self {
        Self {
            window_start: AtomicU64::new(0),
            counter: AtomicU64::new(0),
        }
    }

    /// Check if rate limit is exceeded. Returns true if request should be rejected.
    #[inline]
    fn is_rate_limited(&self, max_per_sec: u64) -> bool {
        // Get current timestamp (seconds since Unix epoch)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        // Try to reset window if expired (CAS - only one thread wins per second)
        let ts = self.window_start.load(Ordering::Acquire);
        if now > ts
            && self
                .window_start
                .compare_exchange(ts, now, Ordering::SeqCst, Ordering::Acquire)
                .is_ok()
        {
            // We won: reset counter for new window
            self.counter.store(1, Ordering::Release);
            return false; // Current request is first in new window, allow it
        }
        // Another thread reset it, fall through to increment with new window

        // Normal case: same window, just increment
        let count = self.counter.fetch_add(1, Ordering::Relaxed) + 1;
        count > max_per_sec
    }
}

/// Global rate limiter instance
static RATE_LIMITER: RateLimiter = RateLimiter::new();

/// Maximum requests per second allowed globally
const MAX_REQ_PER_SEC: u64 = 7500;

/// Static 429 response headers - pre-built to avoid per-request allocation
/// Only standard Retry-After header per RFC 6585
#[allow(clippy::unwrap_used)] // Static init: failure indicates programming error
static RATE_LIMITED_HEADERS: std::sync::LazyLock<ResponseHeader> = std::sync::LazyLock::new(|| {
    let mut header = ResponseHeader::build(429, Some(8)).unwrap();
    header.insert_header("Retry-After", "1").unwrap();
    header
});

pub struct Gateway {
    static_config: StaticFileConfig,
}

impl Gateway {
    /// Create a new Gateway with default static file configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self::with_static_config(StaticFileConfig::new())
    }

    /// Create a new Gateway with custom static file configuration.
    #[must_use]
    pub const fn with_static_config(static_config: StaticFileConfig) -> Self {
        Self { static_config }
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        // Use empty static str to avoid allocation - SNI is not used here
        let peer = Box::new(HttpPeer::new("127.0.0.1:3000", false, String::new()));
        Ok(peer)
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        // Check rate limit first - lock-free hot path (2 atomic ops)
        if RATE_LIMITER.is_rate_limited(MAX_REQ_PER_SEC) {
            session.set_keepalive(None);
            session
                .write_response_header(Box::new(RATE_LIMITED_HEADERS.clone()), true)
                .await?;
            return Ok(true);
        }

        let req = session.req_header();
        let method = req.method.as_str();

        if method == "OPTIONS" {
            let mut resp = ResponseHeader::build(204, None)?;
            resp.insert_header("Access-Control-Allow-Origin", "*")?;
            resp.insert_header("Access-Control-Allow-Headers", "*")?;
            resp.insert_header("Access-Control-Allow-Methods", "*")?;
            resp.insert_header("Access-Control-Allow-Credentials", "true")?;

            session.write_response_header(Box::new(resp), false).await?;
            return Ok(true);
        }

        // Try to serve static files for GET and HEAD requests
        // Clone URI path here - this is the minimal allocation needed
        if matches!(method, "GET" | "HEAD") {
            let uri = req.uri.path().to_owned();
            // Immutable borrow of req ends here
            if serve_static_file(session, &uri, &self.static_config).await? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        // Insert client real IP header for upstream
        // Use Display impl to write directly to header without intermediate String
        if let Some(client_addr) = session.client_addr()
            && let Some(inet_addr) = client_addr.as_inet()
        {
            // Format IP directly - avoids intermediate allocation
            let ip = inet_addr.ip();
            upstream_request.insert_header("X-Real-IP", ip.to_string())?;
        }

        // Insert X-Forwarded-For header (standard proxy header)
        if let Some(client_addr) = session.client_addr() {
            upstream_request.insert_header("X-Forwarded-For", client_addr.to_string())?;
        }

        // Insert X-Forwarded-Host and X-Forwarded-Proto headers
        // These are critical for backend to generate correct URLs for API responses
        // The backend uses these headers to construct download URLs that point to the gateway
        if let Some(host) = session.req_header().headers.get("host") {
            let host_str = host.to_str().unwrap_or("localhost");
            upstream_request.insert_header("X-Forwarded-Host", host_str)?;
        }
        upstream_request.insert_header("X-Forwarded-Proto", "http")?;

        Ok(())
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_response.insert_header("Access-Control-Allow-Origin", "*")?;
        upstream_response.insert_header("Access-Control-Allow-Credentials", "true")?;
        Ok(())
    }
}
