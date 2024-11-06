use serde::{Deserialize, Serialize};
use super::Conference;

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