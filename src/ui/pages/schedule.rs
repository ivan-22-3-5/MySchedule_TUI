use crate::action::{Action, Mode as AppMode};
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
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
enum Mode {
    #[default]
    View,
    Edit(ConferenceEditForm),
}

pub struct SchedulePage {
    schedule: Rc<RefCell<Schedule>>,
    selector: Selector2D,
    mode: Mode,
}

impl SchedulePage {
    pub fn new(schedule: Rc<RefCell<Schedule>>) -> Self {
        let day_lengths = schedule.borrow().iter().map(|day| day.len()).collect();
        Self {
            selector: Selector2D::new(day_lengths),
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
        let titles: Vec<String> = self.schedule.borrow()[selected_day]
            .iter()
            .map(|c| c.title.clone())
            .collect();
        let items = titles.into_iter().map(ListItem::new);
        let list = List::new(items).highlight_style(THEME.selected_text);
        let mut state = ListState::default().with_selected(Option::from(selected_conference));
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn handle_view_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Up => {
                self.selector.move_left();
                Ok(None)
            }
            KeyCode::Down => {
                self.selector.move_right();
                Ok(None)
            }
            KeyCode::Left => {
                self.selector.move_up();
                Ok(None)
            }
            KeyCode::Right => {
                self.selector.move_down();
                Ok(None)
            }
            KeyCode::Char('e') => {
                let (day, conf) = self.selector.selected();
                self.mode = Mode::Edit(ConferenceEditForm::new(
                    self.schedule.borrow()[day][conf].clone(),
                ));
                Ok(Some(Action::ChangeMode(AppMode::Edit)))
            }
            _ => Ok(None),
        }
    }
}

impl Component for SchedulePage {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match &mut self.mode {
            Mode::View => Ok(self.handle_view_key_event(key)?),
            Mode::Edit(form) => match key.code {
                KeyCode::Esc => {
                    let (day, conf) = self.selector.selected();
                    self.schedule.borrow_mut()[day][conf] = form.get_conference();
                    self.mode = Mode::View;
                    Ok(Some(Action::ChangeMode(AppMode::Schedule)))
                }
                _ => Ok(form.handle_key_event(key)?),
            },
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        match &mut self.mode {
            Mode::View => {
                let layout: [Rect; 2] =
                    Layout::vertical([Constraint::Percentage(15), Constraint::Percentage(85)])
                        .areas(area);
                self.render_days(frame, layout[0]);
                self.render_conferences(frame, layout[1]);
            }
            Mode::Edit(form) => form.draw(frame, area)?,
        }
        Ok(())
    }
}
