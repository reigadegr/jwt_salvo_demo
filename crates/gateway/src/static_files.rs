//! Static file serving module for the gateway.
//!
//! Provides functionality to serve static files embedded at compile time,
//! with proper MIME type detection and SPA (Single Page Application) support.

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

use pingora::http::ResponseHeader;
use pingora::prelude::*;
use rust_embed::RustEmbed;
use std::borrow::Cow;

// Use phf (perfect hash function) for O(1) MIME type lookup
use phf::{Map, phf_map};

/// MIME type mapping for common file extensions.
/// Using phf for O(1) compile-time generated perfect hash lookup.
#[allow(clippy::doc_markdown)]
const MIME_TYPES: Map<&'static str, &'static str> = phf_map! {
    "html" => "text/html; charset=utf-8",
    "htm" => "text/html; charset=utf-8",
    "css" => "text/css; charset=utf-8",
    "js" => "application/javascript; charset=utf-8",
    "mjs" => "application/javascript; charset=utf-8",
    "png" => "image/png",
    "jpg" => "image/jpeg",
    "jpeg" => "image/jpeg",
    "gif" => "image/gif",
    "svg" => "image/svg+xml",
    "ico" => "image/x-icon",
    "webp" => "image/webp",
    "woff" => "font/woff",
    "woff2" => "font/woff2",
    "ttf" => "font/ttf",
    "otf" => "font/otf",
    "eot" => "application/vnd.ms-fontobject",
    "json" => "application/json",
    "txt" => "text/plain; charset=utf-8",
    "xml" => "application/xml; charset=utf-8",
    "pdf" => "application/pdf",
    "mp4" => "video/mp4",
    "webm" => "video/webm",
    "mp3" => "audio/mpeg",
    "wav" => "audio/wav",
    "webmanifest" => "application/manifest+json",
};

/// Get MIME type for a file path.
/// O(1) lookup using phf perfect hash function.
#[must_use]
pub fn mime_type(path: &str) -> &'static str {
    path.rsplit('.')
        .next()
        .and_then(|ext| MIME_TYPES.get(ext))
        .copied()
        .unwrap_or("application/octet-stream")
}

/// Embedded static assets from the `dist` directory.
/// The path is relative to the crate root.
#[derive(RustEmbed)]
#[folder = "../../../dist/"]
struct Assets;

/// Get an embedded file by path.
fn get_embedded_file(path: &str) -> Option<Cow<'static, [u8]>> {
    // Remove leading slash if present
    let normalized = path.strip_prefix('/').unwrap_or(path);
    Assets::get(normalized).map(|f| f.data)
}

/// Try to serve a static file for the given request.
///
/// Returns `Ok(true)` if a response was sent, `Ok(false)` if the request
/// should be passed to the upstream, or `Err` on error.
///
/// # Errors
///
/// Returns an error if the response fails to write.
pub async fn serve_static_file(
    session: &mut Session,
    uri: &str,
    config: &StaticFileConfig,
) -> Result<bool> {
    // Skip API requests (match /api, /api/, /api/xxx)
    if uri.starts_with("/api") {
        return Ok(false);
    }

    // Split URI into path and query
    let path_part = uri.split('?').next().unwrap_or(uri);

    // Strip prefix if configured
    let stripped = config
        .path_prefix
        .as_deref()
        .and_then(|p| path_part.strip_prefix(p))
        .unwrap_or(path_part);

    // Remove leading slash for lookup
    let lookup_path = stripped.strip_prefix('/').unwrap_or(stripped);

    // Try to get the embedded file, track if we used fallback
    let (contents, is_fallback) = if let Some(data) = get_embedded_file(lookup_path) {
        (data, false)
    } else if config.spa_fallback {
        // SPA fallback: try index.html
        (
            get_embedded_file("index.html")
                .ok_or_else(|| Error::new(ErrorType::HTTPStatus(404)))?,
            true,
        )
    } else {
        return Ok(false);
    };

    // Determine MIME type - use HTML for fallback or directory requests
    let mime = if is_fallback || stripped.ends_with('/') || stripped.is_empty() {
        "text/html; charset=utf-8"
    } else {
        mime_type(stripped)
    };

    // Build response header
    let mut resp = ResponseHeader::build(200, None)?;
    resp.insert_header("Content-Type", mime)?;
    resp.insert_header("Content-Length", contents.len().to_string())?;
    resp.insert_header("Cache-Control", "public, max-age=3600")?;

    // Write response header
    session.write_response_header(Box::new(resp), false).await?;

    // Write response body - zero-copy conversion from Cow to Bytes
    let body = match contents {
        Cow::Borrowed(slice) => bytes::Bytes::from_static(slice),
        Cow::Owned(vec) => bytes::Bytes::from(vec),
    };
    session.write_response_body(Some(body), true).await?;

    Ok(true)
}

/// Configuration for static file serving.
pub struct StaticFileConfig {
    /// Path prefix to strip from the request URI.
    pub path_prefix: Option<String>,
    /// Enable SPA fallback (serve index.html for non-found paths).
    pub spa_fallback: bool,
}

impl StaticFileConfig {
    /// Create a new static file configuration with default settings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            path_prefix: None,
            spa_fallback: true,
        }
    }

    /// Set the path prefix to strip from request URIs.
    #[must_use]
    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.path_prefix = Some(prefix);
        self
    }

    /// Enable or disable SPA fallback.
    #[must_use]
    pub const fn with_spa_fallback(mut self, enabled: bool) -> Self {
        self.spa_fallback = enabled;
        self
    }
}

impl Default for StaticFileConfig {
    fn default() -> Self {
        Self::new()
    }
}
