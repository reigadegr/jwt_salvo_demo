use once_cell::sync::Lazy;
use serde::Deserialize;

pub static PROFILE: Lazy<Config> = Lazy::new(|| {
    let app_config = include_str!("../application.toml");
    let profile: Config = toml::from_str(app_config).unwrap();
    profile
});

#[derive(Deserialize)]
pub struct Config {
    pub nacos_cfg: NacosCfg,
    pub client_cfg: ClientCfg,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct NacosCfg {
    pub server_ip: String,
    pub app_name: String,
    pub app_ip: String,
    pub app_port: i32,
    pub default_group: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Eq, Hash, PartialEq)]
pub struct ClientCfg {
    pub app_ip: String,
    pub app_port: String,
}
