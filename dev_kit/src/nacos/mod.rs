pub mod naming_service;
use crate::config::get_cfg;
use nacos_sdk::api::naming::ServiceInstance;
use naming_service::{get_naming_service, start_listener};
use once_cell::sync::Lazy;

static SERVICE_INSTANCE: Lazy<ServiceInstance> = Lazy::new(|| ServiceInstance {
    ip: get_cfg().nacos_cfg.service_ip.clone(),
    port: get_cfg().nacos_cfg.service_port,
    weight: get_cfg().nacos_cfg.weight,
    cluster_name: get_cfg().nacos_cfg.cluster_name.clone(),
    ..Default::default()
});

#[must_use]
pub fn get_service_instance() -> &'static ServiceInstance {
    &SERVICE_INSTANCE
}

pub async fn init_nacos_service() {
    start_listener().await;
    register_instance().await;
}

async fn register_instance() {
    let _register_instance_ret = get_naming_service()
        .register_instance(
            get_cfg().nacos_cfg.service_name.clone(),
            Some(get_cfg().nacos_cfg.group_name.clone()),
            get_service_instance().clone(),
        )
        .await;
}

pub async fn deregister_instance() {
    let _deregister_instance_ret = get_naming_service()
        .deregister_instance(
            get_cfg().nacos_cfg.service_name.clone(),
            Some(get_cfg().nacos_cfg.group_name.clone()),
            get_service_instance().clone(),
        )
        .await;
}
