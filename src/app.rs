use crate::cli::Args;
use crate::net_handler::*;
use crate::tui::ui;
use clap::Parser;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::thread::spawn;
use std::time::Duration;

#[derive(Default)]
pub struct App {
    pub input: Vec<char>,
    pub cursor_pos: usize,
    pub messages: Vec<String>,
    pub message_queue: Vec<String>,
    pub input_history: Vec<String>,
    pub debug_messages: Vec<String>,
}

impl App {
    pub fn run(self, term: &mut Terminal<impl Backend>) {
        let app = Arc::new(RwLock::new(self));
        let cli_args = Args::parse();
        let stream =
            TcpStream::connect((cli_args.addr, cli_args.port)).expect("Failed to open stream");
        let stream_r = stream.try_clone().unwrap();
        let stream_w = stream.try_clone().unwrap();
        let c1 = app.clone();
        let c2 = c1.clone();
        spawn(|| handler_c2s(c1, stream_r));
        spawn(|| handler_s2c(c2, stream_w));
        loop {
            term.draw(|f| ui(f, app.clone())).expect("Error drawing");
            if !event::poll(Duration::from_millis(100)).expect("Failed to poll event") {
                continue;
            }
            if let Event::Key(key) = event::read().expect("Failed to read event") {
                match key.code {
                    KeyCode::Enter => app.write().unwrap().send(),
                    KeyCode::Esc => return,
                    KeyCode::Char(c) => app.write().unwrap().enter(c),
                    KeyCode::Backspace => app.write().unwrap().delete(),
                    _ => {}
                }
            }
        }
    }
}
