use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct AwardConfig {
    a: u32,
}
