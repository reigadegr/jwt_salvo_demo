use anyhow::Result;
use nacos_sdk::api::naming::ServiceInstance;
use super::naming_service::get_naming_service;

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
