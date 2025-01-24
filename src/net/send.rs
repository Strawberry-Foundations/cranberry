use std::net::TcpStream;
use std::sync::mpsc::Receiver;

use stblib::stbchat::net::OutgoingPacketStream;
use stblib::stbchat::packet::ServerPacket;

use crate::tui::app::AppEvent;

pub fn send(
    mut w_server: OutgoingPacketStream<TcpStream>,
    rx: Receiver<AppEvent>,
    rx_login: Receiver<(String, String, String)>,
) {
    let tx_data = rx_login.recv().unwrap();

    if tx_data.0 == "event.login" {
        w_server
            .write(ServerPacket::Login {
                username: tx_data.1,
                password: tx_data.2,
            })
            .unwrap();
    }

    loop {
        let message = rx.recv().unwrap_or_else(|_| {
            println!("Client closed connection");
            std::process::exit(0);
        });

        match message {
            AppEvent::SendMessage(message) => {
                w_server.write(ServerPacket::Message { message }).unwrap();
            }
        }
    }
}
