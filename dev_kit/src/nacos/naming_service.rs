use crate::config::get_cfg;
use nacos_sdk::api::{
    naming::{NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder},
    props::ClientProps,
};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tracing::info;

pub struct MyNamingEventListener;

impl NamingEventListener for MyNamingEventListener {
    fn event(&self, event: Arc<NamingChangeEvent>) {
        info!("subscriber notify: {event:?}");
    }
}

static NAMING_SERVICE: Lazy<NamingService> = Lazy::new(|| {
    let server_ip = &get_cfg().nacos_cfg.server_ip;
    let server_port = &get_cfg().nacos_cfg.server_port;
    let client_props = ClientProps::new()
        .server_addr(format!("{server_ip}:{server_port}"))
        .namespace(&get_cfg().nacos_cfg.namespace)
        .app_name(&get_cfg().nacos_cfg.service_name)
        .remote_grpc_port(get_cfg().nacos_cfg.grpc_port)
        .auth_username(&get_cfg().nacos_cfg.username)
        .auth_password(&get_cfg().nacos_cfg.password);

    NamingServiceBuilder::new(client_props)
        .enable_auth_plugin_http()
        .build()
        .unwrap()
});

#[must_use]
pub fn get_naming_service() -> &'static NamingService {
    &NAMING_SERVICE
}
