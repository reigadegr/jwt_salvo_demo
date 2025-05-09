use crate::config::get_cfg;
use nacos_sdk::api::{
    naming::{
        NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder,
        ServiceInstance,
    },
    props::ClientProps,
};
use std::sync::Arc;
use tracing::info;

pub struct MyNamingEventListener;

impl NamingEventListener for MyNamingEventListener {
    fn event(&self, event: Arc<NamingChangeEvent>) {
        info!("subscriber notify: {event:?}");
    }
}

fn create_naming_service() -> NamingService {
    let server_ip = &get_cfg().nacos_cfg.server_ip;
    let server_port = &get_cfg().nacos_cfg.server_port;
    let client_props = ClientProps::new()
        .server_addr(format!("{server_ip}:{server_port}"))
        .auth_username(&get_cfg().nacos_cfg.username)
        .auth_password(&get_cfg().nacos_cfg.password);

    NamingServiceBuilder::new(client_props)
        .enable_auth_plugin_http()
        .build()
        .unwrap()
}

pub async fn init_nacos_service() {
    let listener = Arc::new(MyNamingEventListener);
    let naming_service = create_naming_service();
    let _subscribe_ret = naming_service
        .subscribe(
            get_cfg().nacos_cfg.app_name.clone(),
            Some(get_cfg().nacos_cfg.default_group.clone()),
            Vec::default(),
            listener,
        )
        .await;

    let service_instance1 = ServiceInstance {
        ip: get_cfg().nacos_cfg.app_ip.clone(),
        port: get_cfg().nacos_cfg.app_port,
        ..Default::default()
    };

    let _register_instance_ret = naming_service
        .batch_register_instance(
            get_cfg().nacos_cfg.app_name.clone(),
            Some(get_cfg().nacos_cfg.default_group.clone()),
            vec![service_instance1],
        )
        .await;
}
