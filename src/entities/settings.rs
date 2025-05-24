use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Settings {
    pub autostart: bool,
    pub early_join_minutes: u16,
}
