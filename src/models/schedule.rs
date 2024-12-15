use super::Conference;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Schedule([Vec<Conference>; 7]);

impl Schedule {
    pub fn new() -> Schedule {
        Schedule::default()
    }

    pub fn as_array(&self) -> &[Vec<Conference>; 7] {
        &self.0
    }
}
