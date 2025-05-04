use once_cell::sync::Lazy;
use serde::Deserialize;

const APP_CONFIG: &str = include_str!("../application.toml");

pub static PROFILE: Lazy<Config> = Lazy::new(|| {
    let profile: Config = toml::from_str(APP_CONFIG).unwrap();
    profile
});

#[derive(Deserialize)]
pub struct Config {
    pub nacos_cfg: NacosCfg,
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
