use crossterm::style::Stylize;
use owo_colors::OwoColorize;
use serde_json::Value;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use crate::app::App;

pub fn handler_s2c(app: Arc<RwLock<App>>, stream: TcpStream) {
    let deser = serde_json::Deserializer::from_reader(stream);
    let messages_iter = deser.into_iter::<Value>();
    for message in messages_iter {
        let now = chrono::Local::now();
        let fmt = now.format("- %H:%M:%S |");
        match message {
            Err(e) => app.write().unwrap().messages.push(format!(
                "{} Error deserializing packet - {}",
                "[err]".red(),
                e
            )),
            Ok(msg) => {
                let to_push = match msg["message_type"].as_str() {
                    Some("system_message") => Some(format!(
                        "{} {fmt} {}",
                        "[sys]".bright_green(),
                        msg["message"]["content"].as_str().unwrap()
                    )),
                    Some("stbchat_backend") => None,
                    Some("user_message") => Some(format!(
                        "{} {fmt} {} ({}) -> {}",
                        "[msg]".bright_blue(),
                        msg["username"].as_str().unwrap(),
                        msg["nickname"].as_str().unwrap(),
                        msg["message"]["content"].as_str().unwrap()
                    )),
                    None => None,
                    m => {
                        println!("{} Unimplemented packet {}", "[uimp]".red(), m.unwrap());
                        None
                    }
                };
                if let Some(text) = to_push {
                    app.write().unwrap().messages.push(text);
                }
            }
        }
    }
}

pub fn handler_c2s(app: Arc<RwLock<App>>, mut stream: TcpStream) {
    loop {
        let state = app.read().unwrap();
        let msgs = state.message_queue.clone();
        drop(state);
        if let Some(msg) = msgs.last() {
            stream
                .write_all(msg.as_bytes())
                .expect("Failed to write to stream");
            app.write().unwrap().message_queue.pop();
        }
        sleep(Duration::from_millis(40));
    }
}
