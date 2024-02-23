use std::sync::Arc;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

use stblib::stbm::stbchat::net::IncomingPacketStream;
use stblib::stbm::stbchat::packet::ClientPacket;


use crate::tui::app::{AppEvent, AppState, Views};
use crate::tui::formatter::{badge_handler, MessageFormatter};

pub fn recv(
    mut r_server: IncomingPacketStream<TcpStream>,
    _tx: Sender<AppEvent>,
    _utx: Sender<String>,
    app_state: Arc<std::sync::RwLock<AppState>>,
) {
    let formatter = MessageFormatter::new();

    loop {
        match r_server.read::<ClientPacket>() {
            Ok(ClientPacket::SystemMessage { message}) => {
                app_state.write().unwrap().messages.push(formatter.system(message));
            },

            Ok(ClientPacket::UserMessage { author, message }) => {
                app_state.write().unwrap().messages.push(formatter.user(
                    author.username,
                    author.nickname,
                    author.role_color,
                    badge_handler(author.badge),
                    message
                ));
            },

            Ok(ClientPacket::Event { event_type}) => {
                if event_type == "event.login" {
                    app_state.write().unwrap().current_view = Views::Authentication;
                }
            }
            Err(_) => break,
            _ => ()
        }
    }
}