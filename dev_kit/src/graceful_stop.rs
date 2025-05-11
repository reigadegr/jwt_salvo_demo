use anyhow::anyhow;
use once_cell::sync::OnceCell;
use salvo::server::ServerHandle;

static SERVER_HANDLE: OnceCell<ServerHandle> = OnceCell::new();

pub fn init_handle(handle: ServerHandle) {
    SERVER_HANDLE
        .set(handle)
        .map_err(|_| anyhow!("Failed to set server handle."))
        .unwrap();
}

pub fn get_handle() -> &'static ServerHandle {
    SERVER_HANDLE.get().unwrap()
}
