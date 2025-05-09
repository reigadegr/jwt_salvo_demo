use anyhow::anyhow;
use once_cell::sync::OnceCell;
use serde::Deserialize;

static PROFILE: OnceCell<Config> = OnceCell::new();

pub fn init_config(app_config: &str) {
    let profile: Config = toml::from_str(app_config).unwrap();
    PROFILE
        .set(profile)
        .map_err(|_| anyhow!("Failed to set configuration."))
        .unwrap();
}

pub fn get_cfg() -> &'static Config {
    PROFILE.get().unwrap()
}

#[derive(Deserialize)]
pub struct Config {
    pub nacos_cfg: NacosCfg,
    pub client_cfg: ClientCfg,
    pub redis_cfg: RedisCfg,
}

#[derive(Deserialize)]
pub struct NacosCfg {
    pub server_ip: String,
    pub server_port: u16,
    pub service_name: String,
    pub service_ip: String,
    pub service_port: i32,
    pub weight: f64,
    pub default_group: String,
    pub cluster_name: String,
    pub namespace: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct ClientCfg {
    pub service_ip: String,
    pub service_port: u16,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct RedisCfg {
    pub uri: String,
    pub max_size: u32,
    pub min_idle: Option<u32>,
    pub max_lifetime: Option<u64>,
    pub idle_timeout: Option<u64>,
    pub connection_timeout: u64,
    pub test_on_check_out: bool,
}
