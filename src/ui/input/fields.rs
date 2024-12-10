mod int;
mod string;
mod time;

use crate::ui::Component;

pub use int::IntInputField;
pub use string::StrInputField;
pub use time::TimeInputField;

use ratatui::prelude::Style;
use ratatui::widgets::Borders;
type BorderStyle = (Borders, Style);

pub trait InputField: Component {
    fn get_value(&self) -> String;
    fn borders(&mut self, border_style: Option<BorderStyle>);
    fn set_cursor_visibility(&mut self, visible: bool);
}
