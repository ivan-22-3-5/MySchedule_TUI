use crate::models::Time;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Week {
    #[default]
    Every,
    Even,
    Odd,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Conference {
    pub title: String,
    pub link: String,
    pub start_time: Time,
    pub end_time: Time,
    pub password: Option<String>,
    pub autostart_permission: bool,
    pub week: Week,
}

#[allow(dead_code)]
impl Conference {
    pub fn open(&self) {
        webbrowser::open(&self.link).expect("Browser failed to open");
    }
}
