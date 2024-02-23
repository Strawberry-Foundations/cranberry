use std::io::Write;
use std::net::TcpStream;
use stblib::colors::{BOLD, C_RESET, RED};

pub fn keep_alive(mut stream: TcpStream) {
    loop {
        stblib::utilities::sleep(30);
        stream.write_all(b"[#<keepalive.event.sent>]").unwrap_or_else(|_| {
            eprintln!(
                "{BOLD}{RED}An error occurred when sending Keep Alive to the server.\n\
                Could it be that the connection to the server has been lost?{C_RESET}"
            );
        });
    }
}