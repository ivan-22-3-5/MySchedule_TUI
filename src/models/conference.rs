use crate::models::Time;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Week {
    #[default]
    Every,
    Even,
    Odd,
}
impl Week {
    pub fn variants() -> [&'static str; 3] {
        ["Every", "Even", "Odd"]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Week::Every => "Every",
            Week::Even => "Even",
            Week::Odd => "Odd",
        }
    }

    pub fn parse(s: &str) -> Result<Week, &'static str> {
        match s {
            "Every" => Ok(Week::Every),
            "Even" => Ok(Week::Even),
            "Odd" => Ok(Week::Odd),
            _ => Err("Invalid week: must be 'Every', 'Even', or 'Odd'"),
        }
    }
}

impl FromStr for Week {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Week::parse(s)
    }
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
