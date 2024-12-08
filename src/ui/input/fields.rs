mod int;
mod string;
mod time;
use crate::ui::Component;
pub use int::IntInputField;
use ratatui::prelude::Style;
use ratatui::widgets::Borders;
pub use string::StrInputField;
pub use time::TimeInputField;

type BorderStyle = (Borders, Style);

pub trait InputField: Component {
    fn get_value(&self) -> String;
    fn border_style(&mut self, border_style: Option<BorderStyle>);
    fn set_cursor_visibility(&mut self, visible: bool);
}
