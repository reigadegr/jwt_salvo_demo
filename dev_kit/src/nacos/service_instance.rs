use crate::config::get_cfg;
use nacos_sdk::api::naming::ServiceInstance;
use once_cell::sync::Lazy;

static SERVICE_INSTANCE: Lazy<ServiceInstance> = Lazy::new(|| ServiceInstance {
    ip: get_cfg().nacos_cfg.service_ip.clone(),
    port: get_cfg().client_cfg.service_port,
    weight: get_cfg().nacos_cfg.weight,
    cluster_name: get_cfg().nacos_cfg.cluster_name.clone(),
    ..Default::default()
});

#[must_use]
pub fn get_service_instance() -> &'static ServiceInstance {
    &SERVICE_INSTANCE
}
