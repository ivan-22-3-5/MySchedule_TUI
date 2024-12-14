use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Conference {
    pub title: String,
    pub link: String,
    pub start_time: String,
    pub duration: u8,
    pub password: Option<String>,
    pub autostart_permission: bool,
    pub week: u8,
}

#[allow(dead_code)]
impl Conference {
    pub fn open(&self) {
        webbrowser::open(&self.link).expect("Browser failed to open");
    }
}
