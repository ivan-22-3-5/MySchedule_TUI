use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub selected: Style,
}

pub const THEME: Theme = Theme {
    selected: Style::new()
        .fg(Color::White)
        .bg(Color::DarkGray)
        .add_modifier(Modifier::BOLD),
};
