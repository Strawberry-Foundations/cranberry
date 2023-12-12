use ansi_to_tui::IntoText;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use std::sync::{Arc, RwLock};
use tui_textarea::TextArea;
use crate::app::{App, SelectServerScreen};

pub fn select_server_ui(frame: &mut Frame, select_server: Arc<RwLock<SelectServerScreen>>) {
    todo!();
    let mut size = frame.size();
    size.height /= 2;
    size.width /= 2;
}

pub fn ui(frame: &mut Frame, app: Arc<RwLock<App>>, text_area: &mut TextArea) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(3),
        ])
        .split(frame.size());
    let text = Text::from(Line::from("Press ESC to exit"));
    let esc_info = Paragraph::new(text);
    frame.render_widget(esc_info, chunks[0]);

    let state = app.read().unwrap();
    let input = text_area.widget();
    frame.render_widget(input, chunks[1]);
    let mut list_state = ListState::default();
    let selected = app.read().unwrap().selected;
    list_state.select(selected);
    let len = state.messages.len();
    let messages: Vec<ListItem> = state
        .messages
        .clone()
        .into_iter()
        .enumerate()
        .rev()
        .map(|(i, mut m)| {
            if selected == Some(len-i) {
                m = format!("> {m}");
            }
            let line = m.into_text().unwrap();
            ListItem::new(line)
        })
        .collect();
    drop(state);

    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    frame.render_stateful_widget(messages, chunks[2], &mut list_state);
}



impl App {
    pub fn send(&mut self, msg: String) {
        self.message_queue.push(msg);
    }
}
