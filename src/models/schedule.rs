use super::Conference;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub monday: Vec<Conference>,
    pub tuesday: Vec<Conference>,
    pub wednesday: Vec<Conference>,
    pub thursday: Vec<Conference>,
    pub friday: Vec<Conference>,
    pub saturday: Vec<Conference>,
    pub sunday: Vec<Conference>,
}

impl Schedule {
    pub fn to_array(self) -> [Vec<Conference>; 7] {
        [
            self.monday,
            self.tuesday,
            self.wednesday,
            self.thursday,
            self.friday,
            self.saturday,
            self.sunday,
        ]
    }
}
