use anyhow::anyhow;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use toml::from_str;

static PROFILE: OnceCell<Config> = OnceCell::new();

pub fn init_config(app_config: &str) {
    let profile: Config = from_str(app_config).unwrap();
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
    pub client_cfg: ClientCfg,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct ClientCfg {
    pub service_ip: String,
    pub service_port: i32,
}
