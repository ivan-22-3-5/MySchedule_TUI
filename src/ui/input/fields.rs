mod int;
mod string;
mod time;
use crate::ui::Component;
pub use int::IntInputField;
use ratatui::prelude::Style;
pub use string::StrInputField;
pub use time::TimeInputField;

pub trait InputField: Component {
    fn get_value(&self) -> String;
    fn border_style(&mut self, style: Style);
}
