use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub selected_text: Style,
    pub selected_field: Style,
    pub active_field: Style,
    pub input_field: Style,
}

pub const THEME: Theme = Theme {
    selected_text: Style::new()
        .fg(Color::White)
        .bg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
    input_field: Style::new().fg(Color::White),
    selected_field: Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
    active_field: Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
};
