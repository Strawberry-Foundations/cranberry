use crate::cli::Args;
use crate::net_handler::*;
use clap::Parser;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::thread::spawn;
use std::time::Duration;
use ratatui::widgets::{Block, Borders};
use tui_textarea::TextArea;
use crate::commands:: run_cmd_threaded;
use crate::tui::ui;

#[derive(Default)]
pub struct SelectServerScreen {
    pub ip: Vec<char>,
    pub port: Vec<char>,
    pub entered: bool,
}

#[derive(Default)]
pub struct App {
    pub cursor_pos: usize,
    pub messages: Vec<String>,
    pub message_queue: Vec<String>,
    pub selected: Option<usize>,
}

impl App {
    pub fn run(self, term: &mut Terminal<impl Backend>) {
        let app = Arc::new(RwLock::new(self));
        let cli_args = Args::parse();
        let mut stream =
            TcpStream::connect((cli_args.addr, cli_args.port)).expect("Failed to open stream");
        let stream_r = stream.try_clone().unwrap();
        let stream_w = stream.try_clone().unwrap();
        let c1 = app.clone();
        let c2 = c1.clone();
        spawn(|| handler_c2s(c1, stream_r));
        spawn(|| handler_s2c(c2, stream_w));
        let mut input = TextArea::default();
        input.set_block(Block::default().borders(Borders::ALL));
        loop {
            term.draw(|f| ui(f, app.clone(), &mut input)).expect("Error drawing");
            if !event::poll(Duration::from_millis(100)).expect("Failed to poll event") {
                continue;
            }
            if let Event::Key(key) = event::read().expect("Failed to read event") {
                match key.code {
                    KeyCode::Enter => {
                        let inp = input.lines()[0].clone();
                        if inp.starts_with('.') && inp.trim() != "." {
                            run_cmd_threaded(inp.strip_prefix('.').unwrap().to_string(), &mut stream, app.clone());
                            input = TextArea::default();
                            input.set_block(Block::default().borders(Borders::ALL));
                            continue;
                        }
                        app.write().unwrap().send(inp);
                        input = TextArea::default();
                        input.set_block(Block::default().borders(Borders::ALL));
                    },
                    KeyCode::Esc => return,
                    KeyCode::Down => {
                        let new_selected = app.write().unwrap().selected.unwrap_or(0) + 1;
                        if new_selected > app.read().unwrap().messages.len() {
                            continue;
                        }
                        app.write().unwrap().selected = Some(new_selected);
                    },
                    KeyCode::Up => {
                        let mut new_selected = Some(app.read().unwrap().selected.unwrap_or(0).saturating_sub(1));
                        if new_selected == Some(0) {
                            new_selected = None
                        }
                        app.write().unwrap().selected = new_selected;
                    },
                    _ => { input.input(key); }
                }
            }
        }
    }
}
