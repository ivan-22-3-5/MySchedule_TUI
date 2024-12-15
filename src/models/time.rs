use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Default)]
pub struct Time(u8, u8);

impl Time {
    pub fn parse(time: &str) -> Result<Time, &'static str> {
        const MAX_HOURS: u8 = 23;
        const MAX_MINUTES: u8 = 59;

        fn validate_hours(hours: u8) -> Result<(), &'static str> {
            if hours <= MAX_HOURS {
                Ok(())
            } else {
                Err("Invalid hour: must be between 0 and 23")
            }
        }

        fn validate_minutes(minutes: u8) -> Result<(), &'static str> {
            if minutes <= MAX_MINUTES {
                Ok(())
            } else {
                Err("Invalid minute: must be between 0 and 59")
            }
        }

        let mut parts = time.split(':').map(|s| s.parse::<u8>());
        match (parts.next(), parts.next()) {
            (Some(Ok(hours)), Some(Ok(minutes))) => {
                validate_hours(hours)?;
                validate_minutes(minutes)?;
                Ok(Self(hours, minutes))
            }
            _ => Err("Invalid time format: must be HH:MM"),
        }
    }

    pub fn hours(&self) -> u8 {
        self.0
    }

    pub fn minutes(&self) -> u8 {
        self.1
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0, self.1)
    }
}

impl FromStr for Time {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Time::parse(s)
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> color_eyre::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let parsed_str = String::deserialize(deserializer)?;
        Time::from_str(&parsed_str).map_err(serde::de::Error::custom)
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> color_eyre::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
