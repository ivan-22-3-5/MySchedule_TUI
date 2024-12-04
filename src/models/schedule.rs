use super::Conference;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub monday: Arc<Vec<Conference>>,
    pub tuesday: Arc<Vec<Conference>>,
    pub wednesday: Arc<Vec<Conference>>,
    pub thursday: Arc<Vec<Conference>>,
    pub friday: Arc<Vec<Conference>>,
    pub saturday: Arc<Vec<Conference>>,
    pub sunday: Arc<Vec<Conference>>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            monday: Arc::new(Vec::new()),
            tuesday: Arc::new(Vec::new()),
            wednesday: Arc::new(Vec::new()),
            thursday: Arc::new(Vec::new()),
            friday: Arc::new(Vec::new()),
            saturday: Arc::new(Vec::new()),
            sunday: Arc::new(Vec::new()),
        }
    }
    pub fn clone_into_array(&self) -> [Arc<Vec<Conference>>; 7] {
        [
            Arc::clone(&self.monday),
            Arc::clone(&self.tuesday),
            Arc::clone(&self.wednesday),
            Arc::clone(&self.thursday),
            Arc::clone(&self.friday),
            Arc::clone(&self.saturday),
            Arc::clone(&self.sunday),
        ]
    }
}
