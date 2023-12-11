use ansi_to_tui::IntoText;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
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

    let messages: Vec<ListItem> = state
        .messages
        .iter()
        .rev()
        .map(|m| {
            let line = m.into_text().unwrap();
            ListItem::new(line)
        })
        .collect();
    drop(state);
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    frame.render_widget(messages, chunks[2]);
}



impl App {
    pub fn send(&mut self, msg: &str) {
        self.message_queue.push(msg.to_string());
    }
}
