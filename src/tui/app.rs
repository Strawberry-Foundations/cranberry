use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::net::TcpStream;
use std::sync::mpsc::{channel};

use crossterm::event;
use crossterm::event::{Event, KeyCode};

use ratatui::backend::Backend;
use ratatui::style::{Color, Style};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders};

use stblib::stbm::stbchat::net::{IncomingPacketStream, OutgoingPacketStream};

use tui_textarea::TextArea;

use crate::net;
use crate::tui::app::AppEvent::SendMessage;

#[derive(Default, Clone)]
pub struct App {
    pub cursor_position: usize,
    pub selected: Option<usize>,
    pub current_view: Views,
    pub state: Arc<RwLock<AppState>>,
    pub address: String,
    pub port: u16,
    pub selected_text_field: String,
}

#[derive(Default, Clone)]
pub struct AppState {
    pub messages: Vec<String>,
    pub current_view: Views,
}

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Views {
    #[default]
    MainView,
    Menu,
    Authentication
}

pub enum AppEvent {
    SendMessage(String),
}

impl App {
    pub fn run(mut self, terminal: &mut Terminal<impl Backend>, host: (String, u16)) {
        let (tx_recv, _rx_recv) = channel::<AppEvent>();
        let (tx_send, rx_send) = channel::<AppEvent>();

        let (tx, rx) = channel::<(String, String, String)>();
        let (utx, _urx) = channel::<String>();

        self.state = Arc::new(RwLock::new(AppState::default()));

        let mut input = TextArea::default();
        input.set_block(Block::default().borders(Borders::ALL));
        input.set_cursor_line_style(Style::default());
        input.set_placeholder_text("Message...");

        let mut username_input = TextArea::default();
        username_input.set_block(Block::default().borders(Borders::ALL));
        username_input.set_cursor_line_style(Style::default());
        username_input.set_placeholder_text("Username");

        let mut password_input = TextArea::default();
        password_input.set_block(Block::default().borders(Borders::ALL));
        password_input.set_cursor_line_style(Style::default());
        password_input.set_placeholder_text("Password");
        password_input.set_mask_char('*');
        
        self.address = host.0;
        self.port = host.1;

        let host = (self.address.clone(), self.port);

        let stream = match TcpStream::connect(host) {
            Ok(tcp_stream) => tcp_stream,
            Err(_) => {
                eprintln!("Server unreachable.");
                std::process::exit(1);
            }
        };

        let (r_server, w_server) = (stream.try_clone().unwrap(), stream.try_clone().unwrap());
        
        let keep_alive_stream = OutgoingPacketStream::wrap(stream.try_clone().unwrap());

        let r_server = IncomingPacketStream::wrap(r_server);
        let w_server = OutgoingPacketStream::wrap(w_server);

        let state_clone = self.state.clone();

        std::thread::spawn(|| { net::recv::recv(r_server, tx_recv, utx, state_clone) });
        std::thread::spawn(|| { net::send::send(w_server, rx_send, rx) });

        std::thread::spawn(|| { net::keep_alive::keep_alive(keep_alive_stream) });

        self.selected_text_field = String::from("username");

        loop {
            terminal.draw(|frame| self.main_ui(frame, &mut input, &mut username_input, &mut password_input)).unwrap();

            if !event::poll(Duration::from_millis(100)).expect("Failed to poll event") {
                continue;
            }
            
            if let Event::Key(key) = event::read().expect("Failed to read event") {
                let current_view = self.state.read().unwrap().current_view;

                match current_view {
                    Views::MainView => {
                        match key.code {
                            KeyCode::Enter => {
                                let message = input.lines()[0].clone();
                                tx_send.send(SendMessage(message)).unwrap();

                                input = TextArea::default();
                                input.set_block(Block::default().borders(Borders::ALL));
                                input.set_cursor_line_style(Style::default());
                                input.set_placeholder_text("Message...");

                            }
                            KeyCode::Esc => { self.state.write().unwrap().current_view = Views::Menu },
                            KeyCode::Down => {
                                let mut new_selected = Some(self.selected.unwrap_or(0).saturating_sub(1));
                                if new_selected == Some(0) {
                                    new_selected = None
                                }
                                self.selected = new_selected
                            },
                            KeyCode::Up => {
                                let mut new_selected = Some(self.selected.unwrap_or(0) + 1);

                                if new_selected > Some(self.state.read().unwrap().messages.len()) {
                                    new_selected = None
                                }
                                self.selected = new_selected
                            },
                            KeyCode::PageUp => {
                                self.selected = Some(self.state.read().unwrap().messages.len())
                            }
                            KeyCode::PageDown => {
                                self.selected = Some(1)
                            }
                            _ => {
                                input.input(key);
                            },
                        }
                    }
                    Views::Menu => {
                        match key.code {
                            KeyCode::Char('q') => {
                                tx_send.send(SendMessage("/exit".to_string())).unwrap();
                                return
                            },
                            KeyCode::Esc => { self.state.write().unwrap().current_view = Views::default() },
                            _ => {}
                        }

                    }
                    Views::Authentication => {
                        match key.code {
                            KeyCode::Esc => { self.state.write().unwrap().current_view = Views::default(); },
                            KeyCode::Up => {
                                self.selected_text_field = String::from("username");
                                password_input.set_cursor_style(Style::default());
                                username_input.set_cursor_style(Style::default().bg(Color::White))
                            },
                            KeyCode::Down => {
                                self.selected_text_field = String::from("password");
                                username_input.set_cursor_style(Style::default());
                                password_input.set_cursor_style(Style::default().bg(Color::White))
                            },
                            KeyCode::Tab => {
                                match self.selected_text_field.as_str() {
                                    "username" => {
                                        self.selected_text_field = String::from("password");
                                        username_input.set_cursor_style(Style::default());
                                        password_input.set_cursor_style(Style::default().bg(Color::White))
                                    },
                                    "password" => {
                                        self.selected_text_field = String::from("username");
                                        password_input.set_cursor_style(Style::default());
                                        username_input.set_cursor_style(Style::default().bg(Color::White))
                                    }
                                    _ => {}
                                }
                            }
                            KeyCode::Enter => {
                                let (username, password) = (
                                    username_input.lines()[0].clone(),
                                    password_input.lines()[0].clone()
                                );

                                tx.send(("event.login".to_string(), username, password)).unwrap();

                                self.state.write().unwrap().current_view = Views::default();
                            },
                            _ => {
                                if self.selected_text_field == "username" {
                                    username_input.input(key);
                                }
                                else {
                                    password_input.input(key);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}