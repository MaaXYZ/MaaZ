use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub enum ClientType {
    #[default]
    Official,
    Bilibili,
}

impl ClientType {
    pub fn get_package_name(&self) -> String {
        match *self {
            ClientType::Official => "com.hypergryph.arknights/com.u8.sdk.U8UnityContext".to_owned(),
            ClientType::Bilibili => "com.hypergryph.arknights.bilibili".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct StartUpConfig {
    pub client_type: ClientType,
}
