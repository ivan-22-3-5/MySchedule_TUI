use crate::action::{Action, Mode as AppMode};
use crate::entities::Schedule;
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
    Add(ConferenceEditForm),
}

pub struct SchedulePage {
    schedule: Rc<RefCell<Schedule>>,
    selector: Selector2D,
    mode: Mode,
}

impl SchedulePage {
    pub fn new(schedule: Rc<RefCell<Schedule>>) -> Self {
        let day_lengths = schedule.borrow().get_conference_count_by_day();
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
        let titles: Vec<String> = self
            .schedule
            .borrow()
            .get_day(selected_day)
            .iter()
            .map(|c| c.title.clone())
            .collect();
        let items = titles.into_iter().map(ListItem::new);
        let list = List::new(items).highlight_style(THEME.selected_text);
        let mut state = ListState::default().with_selected(Option::from(selected_conference));
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn handle_view_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        if key.code == KeyCode::Char('e') {
            let (day, conf) = self.selector.selected();
            // TODO: shouldn't be allowed to edit a conference that doesn't exist
            self.mode = Mode::Edit(ConferenceEditForm::new(
                self.schedule.borrow().get_day(day)[conf].clone().into(),
            ));
            return Ok(Some(Action::ChangeMode(AppMode::Edit)));
        } else if key.code == KeyCode::Char('+') {
            self.mode = Mode::Add(ConferenceEditForm::new(None));
            return Ok(Some(Action::ChangeMode(AppMode::Edit)));
        } else {
            match key.code {
                KeyCode::Up => self.selector.move_left(),
                KeyCode::Down => self.selector.move_right(),
                KeyCode::Left => self.selector.move_up(),
                KeyCode::Right => self.selector.move_down(),
                _ => (),
            };
        }
        Ok(None)
    }
}

impl Component for SchedulePage {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match &mut self.mode {
            Mode::View => Ok(self.handle_view_key_event(key)?),
            Mode::Edit(form) => match key.code {
                KeyCode::Esc => {
                    let (day, conf) = self.selector.selected();
                    self.schedule
                        .borrow_mut()
                        .update_conference(day, conf, form.get_conference())
                        .expect("Failed to update conference, conference not found");
                    self.mode = Mode::View;
                    Ok(Some(Action::ChangeMode(AppMode::Schedule)))
                }
                _ => Ok(form.handle_key_event(key)?),
            },
            Mode::Add(form) => match key.code {
                KeyCode::Esc => {
                    let (day, _) = self.selector.selected();
                    self.schedule
                        .borrow_mut()
                        .add_conference(day, form.get_conference());
                    self.mode = Mode::View;
                    self.selector =
                        Selector2D::new(self.schedule.borrow().get_conference_count_by_day());
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
                    Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(area);
                self.render_days(frame, layout[0]);
                self.render_conferences(frame, layout[1]);
            }
            Mode::Edit(form) => form.draw(frame, area)?,
            Mode::Add(form) => form.draw(frame, area)?,
        }
        Ok(())
    }
}
