use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::Frame;

use ansi_to_tui::IntoText;
use tui_textarea::TextArea;

use crate::tui::app::{App, Views};

impl App {
    pub fn main_ui(
        &mut self,
        frame: &mut Frame,
        input: &mut TextArea,
        username_input: &mut TextArea,
        password_input: &mut TextArea,
    ) {
        let area = frame.size();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new("Strawberry Chat");

        frame.render_widget(title, layout[0]);
        frame.render_widget(input.widget(), layout[2]);

        let mut list_state = ListState::default();
        let selected = self.selected;
        let messages_len = { self.state.read().unwrap().messages.len() };

        // list_state.select(selected);

        list_state.select(Some(messages_len));

        let view_messages: Vec<ListItem> = self
            .state
            .read()
            .unwrap()
            .messages
            .clone()
            .iter()
            .enumerate()
            .map(|(i, m)| {
                if selected == Some(messages_len - i) {
                    return ListItem::new(format!("* {m}").into_text().unwrap().bold().italic());
                }

                ListItem::new(m.into_text().unwrap())
            })
            .collect();

        let message_list = List::new(view_messages)
            .block(Block::default().borders(Borders::ALL).title("Messages"));

        frame.render_stateful_widget(message_list, layout[1], &mut list_state);

        match self.state.read().unwrap().current_view {
            Views::Menu => {
                let area = Self::centered_rect(50, 45, area);

                let menu = Block::default().title("Menu").borders(Borders::ALL);

                let menu_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(1), Constraint::Min(0)])
                    .split(area);

                let text = Paragraph::new("Press 'q' to exit Strawberry Chat")
                    .style(Style::default().fg(Color::White))
                    .alignment(Alignment::Center);

                frame.render_widget(Clear, area);
                frame.render_widget(menu, area);
                frame.render_widget(text, menu_layout[1]);
            }
            Views::Authentication => {
                let area = Self::centered_rect(50, 45, area);

                let login = Block::default().title("Login").borders(Borders::ALL);

                let login_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Length(1),
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ])
                    .split(area);

                let text = Paragraph::new(format!("Login to Strawberry Chat ({})", self.address))
                    .style(Style::default().fg(Color::White).bold())
                    .alignment(Alignment::Center);

                frame.render_widget(Clear, area);
                frame.render_widget(login, area);
                frame.render_widget(text, login_layout[1]);
                frame.render_widget(username_input.widget(), login_layout[2]);
                frame.render_widget(password_input.widget(), login_layout[3]);
            }
            _ => {}
        }
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }
}
