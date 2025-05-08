use anyhow::anyhow;
use once_cell::sync::OnceCell;
use salvo::server::ServerHandle;

pub static SERVER_HANDLE: OnceCell<ServerHandle> = OnceCell::new();

pub fn init_handle(app_config: ServerHandle) {
    SERVER_HANDLE
        .set(app_config)
        .map_err(|_| anyhow!("Failed to set server handle."))
        .unwrap();
}

pub fn get_handle() -> &'static ServerHandle {
    SERVER_HANDLE.get().unwrap()
}
