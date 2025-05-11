pub mod naming_service;
pub mod rpc;
pub mod service_instance;

use crate::config::get_cfg;
use naming_service::{MyNamingEventListener, get_naming_service};
use service_instance::get_service_instance;
use std::sync::Arc;

pub async fn start_listener() {
    let listener = Arc::new(MyNamingEventListener);
    let _subscribe_ret = get_naming_service()
        .subscribe(
            get_cfg().nacos_cfg.service_name.clone(),
            Some(get_cfg().nacos_cfg.group_name.clone()),
            Vec::default(),
            listener,
        )
        .await;
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
