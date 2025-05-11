use crate::config::get_cfg;
use anyhow::Result;
use nacos_sdk::api::naming::ServiceInstance;
use super::naming_service::{MyNamingEventListener, get_naming_service};
use service_instance::get_service_instance;
use std::sync::Arc;

async fn get_healthy_instance(
    service_name: &str,
    group: &str,
    cluster: Vec<String>,
) -> Result<ServiceInstance> {
    let instance = get_naming_service()
        .select_one_healthy_instance(
            String::from(service_name),
            Some(group.to_string()),
            cluster,
            true,
        )
        .await?;
    Ok(instance)
}
