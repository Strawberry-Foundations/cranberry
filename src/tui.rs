use ansi_to_tui::IntoText;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use std::sync::{Arc, RwLock};
use crate::app::App;

pub fn ui(frame: &mut Frame, app: Arc<RwLock<App>>) {
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
    let input = Paragraph::new(state.input.as_str())
        .block(Block::default().borders(Borders::ALL).title("Message"));
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
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    frame.render_widget(messages, chunks[2]);
}



impl App {
    pub fn move_cursor_left(&mut self) {
        let moved = self.cursor_pos.saturating_sub(1);
        self.cursor_pos = moved.clamp(0, self.input.len());
    }
    pub fn move_cursor_right(&mut self) {
        let moved = self.cursor_pos + 1;
        self.cursor_pos = moved.clamp(0, self.input.len());
    }

    pub fn enter(&mut self, ch: char) {
        self.input.insert(self.cursor_pos, ch);
        self.move_cursor_right();
    }

    pub fn delete(&mut self) {
        if self.cursor_pos == 0 {
            return;
        }
        let char_to_delete_pos = self.cursor_pos - 1;
        self.input.remove(char_to_delete_pos);
        self.move_cursor_left();
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn send(&mut self) {
        self.message_queue.push(self.input.clone());
        self.input = String::new();
        self.reset_cursor();
    }
}
