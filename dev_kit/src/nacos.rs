use crate::config::PROFILE;
use nacos_sdk::api::{
    naming::{
        NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder,
        ServiceInstance,
    },
    props::ClientProps,
};
use std::sync::{Arc, LazyLock};

pub struct MyNamingEventListener;

impl NamingEventListener for MyNamingEventListener {
    fn event(&self, event: Arc<NamingChangeEvent>) {
        println!("subscriber notify: {event:?}");
    }
}

static CLIENT_PROPS: LazyLock<ClientProps> = LazyLock::new(|| {
    ClientProps::new()
        .server_addr(&PROFILE.nacos_cfg.server_ip)
        .auth_username(&PROFILE.nacos_cfg.username)
        .auth_password(&PROFILE.nacos_cfg.password)
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
            PROFILE.nacos_cfg.app_name.clone(),
            Some(PROFILE.nacos_cfg.default_group.clone()),
            Vec::default(),
            listener,
        )
        .await;

    let service_instance1 = ServiceInstance {
        ip: PROFILE.nacos_cfg.app_ip.clone(),
        port: PROFILE.nacos_cfg.app_port,
        ..Default::default()
    };

    let _register_instance_ret = NAMING_SERVICE
        .batch_register_instance(
            PROFILE.nacos_cfg.app_name.clone(),
            Some(PROFILE.nacos_cfg.default_group.clone()),
            vec![service_instance1],
        )
        .await;
}
