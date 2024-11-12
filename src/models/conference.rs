use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conference {
    pub title: String,
    pub link: String,
    pub start_time: String,
    pub duration: u8,
    pub password: Option<String>,
    pub autostart_permission: bool,
    pub week: u8,
}

impl Display for Conference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Conference Details:\nTitle: {}\nLink: {}\nStart Time: {}\nDuration: {} hours\n{}Autostart Permission: {}\nWeek: {}",
            self.title,
            self.link,
            self.start_time,
            self.duration,
            match &self.password {
                Some(p) => format!("Password: {}\n", p),
                None => "Password: not set\n".to_string(),
            },
            if self.autostart_permission { "Yes" } else { "No" },
            self.week
        )
    }
}
