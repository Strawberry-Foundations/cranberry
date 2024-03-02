use std::net::TcpStream;
use stblib::colors::{BOLD, C_RESET, RED};
use stblib::stbm::stbchat::net::OutgoingPacketStream;
use stblib::stbm::stbchat::packet::ServerPacket;

pub fn keep_alive(mut server: OutgoingPacketStream<TcpStream>) {
    loop {
        stblib::utilities::sleep(30);
        server.write(ServerPacket::KeepAlive)
            .unwrap_or_else(|_| {
                eprintln!(
                    "{BOLD}{RED}An error occurred when sending Keep Alive to the server.\n\
                Could it be that the connection to the server has been lost?{C_RESET}"
                );
            });
    }
}