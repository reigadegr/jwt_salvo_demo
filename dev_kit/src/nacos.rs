use crate::config::PROFILE;
use nacos_sdk::api::{
    naming::{
        NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder,
        ServiceInstance,
    },
    props::ClientProps,
};
use std::sync::{Arc, LazyLock};

// static SERVER_ADDR: &str = &PROFILE.nacos_cfg.server_addr;
const APPLICATION_NAME: &str = "jwt-salvo-service";
const APPLICATION_IP_ADDR: &str = "127.0.0.1";
const APPLICATION_PORT: i32 = 3000;
const DEFAULT_GROUP: &str = "DEFAULT_GROUP";
const USERNAME: &str = "admin";
const PASSWORD: &str = "admin";
pub struct MyNamingEventListener;

impl NamingEventListener for MyNamingEventListener {
    fn event(&self, event: Arc<NamingChangeEvent>) {
        println!("subscriber notify: {event:?}");
    }
}

static CLIENT_PROPS: LazyLock<ClientProps> = LazyLock::new(|| {
    ClientProps::new()
        .server_addr(&PROFILE.nacos_cfg.server_addr)
        .auth_username(USERNAME)
        .auth_password(PASSWORD)
});

static NAMING_SERVICE: LazyLock<Box<NamingService>> = LazyLock::new(|| {
    let naming_service = NamingServiceBuilder::new(CLIENT_PROPS.clone())
        .enable_auth_plugin_http()
        .build()
        .unwrap();
    Box::new(naming_service)
});

pub async fn init_nacos_service() {
    let listener = Arc::new(MyNamingEventListener);
    let _subscribe_ret = NAMING_SERVICE
        .subscribe(
            APPLICATION_NAME.to_string(),
            Some(DEFAULT_GROUP.to_string()),
            Vec::default(),
            listener,
        )
        .await;

    let service_instance1 = ServiceInstance {
        ip: APPLICATION_IP_ADDR.to_string(),
        port: APPLICATION_PORT,
        ..Default::default()
    };

    let _register_instance_ret = NAMING_SERVICE
        .batch_register_instance(
            APPLICATION_NAME.to_string(),
            Some(DEFAULT_GROUP.to_string()),
            vec![service_instance1],
        )
        .await;
}
