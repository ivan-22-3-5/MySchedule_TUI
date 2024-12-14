use crate::action::Action;
use crate::models::Schedule;
use crate::theme::THEME;
use crate::ui::components::Selector2D;
use crate::ui::input::forms::ConferenceEditForm;
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Tabs};
use ratatui::Frame;
use std::rc::Rc;

#[derive(Default)]
enum Mode {
    #[default]
    View,
    Edit(ConferenceEditForm),
}

pub struct SchedulePage {
    schedule: Rc<Schedule>,
    selector: Selector2D,
    mode: Mode,
}

impl SchedulePage {
    pub fn new(schedule: Rc<Schedule>) -> Self {
        Self {
            selector: Selector2D::new(schedule.to_array().iter().map(|day| day.len()).collect()),
            schedule,
            mode: Mode::default(),
        }
    }

    fn render_days(&mut self, frame: &mut Frame, area: Rect) {
        let titles =
            ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].map(|day| format!("  {}  ", day));
        let (selected_day, _) = self.selector.selected();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Schedule"))
            .style(Style::default().fg(Color::White))
            .divider("")
            .padding("", "")
            .highlight_style(THEME.selected_text)
            .select(selected_day);

        frame.render_widget(tabs, area);
    }

    fn render_conferences(&mut self, frame: &mut Frame, area: Rect) {
        let (selected_day, selected_conference) = self.selector.selected();
        let titles = self
            .schedule
            .to_array()
            .get(selected_day)
            .unwrap()
            .iter()
            .map(|c| c.title.clone());
        let items = titles.map(ListItem::new);
        let list = List::new(items).highlight_style(THEME.selected_text);
        let mut state = ListState::default().with_selected(Option::from(selected_conference));
        frame.render_stateful_widget(list, area, &mut state);
    }
}

impl Component for SchedulePage {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match &mut self.mode {
            Mode::View => match key.code {
                KeyCode::Up => self.selector.move_left(),
                KeyCode::Down => self.selector.move_right(),
                KeyCode::Left => self.selector.move_up(),
                KeyCode::Right => self.selector.move_down(),
                KeyCode::Char('e') => {
                    let (day, conf) = self.selector.selected();
                    self.mode = Mode::Edit(ConferenceEditForm::new(
                        self.schedule
                            .to_array()
                            .get(day)
                            .unwrap()
                            .get(conf)
                            .unwrap()
                            .clone(),
                    ))
                }
                _ => (),
            },
            Mode::Edit(form) => {
                form.handle_key_event(key)?;
                if key.code == KeyCode::Esc {
                    // let (day, conf) = self.selector.selected();

                    self.mode = Mode::View;
                }
            }
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let layout: [Rect; 2] =
            Layout::vertical([Constraint::Percentage(15), Constraint::Percentage(85)]).areas(area);
        self.render_days(frame, layout[0]);
        self.render_conferences(frame, layout[1]);
        Ok(())
    }
}
