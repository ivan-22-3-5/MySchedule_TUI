use crate::models::Schedule;

pub trait ScheduleLoader {
    fn save(&mut self, schedule: Schedule);
    fn load(&mut self, name: &str) -> Schedule;
    fn delete(&mut self, name: &str);
}

pub struct JsonScheduleLoader {
    base_path: String,
}

impl JsonScheduleLoader {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
        }
    }
}

impl ScheduleLoader for JsonScheduleLoader {
    fn save(&mut self, schedule: Schedule) {
        todo!()
    }

    fn load(&mut self, name: &str) -> Schedule {
        todo!()
    }
    fn delete(&mut self, name: &str) {
        todo!()
    }
}
