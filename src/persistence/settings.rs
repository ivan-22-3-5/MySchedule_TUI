use crate::models::Settings;

pub trait SettingsLoader {
    fn load(&mut self) -> Settings;
    fn save(&mut self, settings: Settings);
}

pub struct JsonSettingsLoader {
    base_path: String,
}

impl JsonSettingsLoader {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
        }
    }
}

impl SettingsLoader for JsonSettingsLoader {
    fn load(&mut self) -> Settings {
        todo!()
    }
    fn save(&mut self, settings: Settings) {
        todo!()
    }
}
