use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::thread::{sleep, sleep_ms};
use std::time::Duration;
use clap::Parser;
use owo_colors::OwoColorize;
use rayon::prelude::*;
use rayon::spawn;
use crate::app::App;
use crate::cli::Args;

pub struct Command {
    pub name: String,
    pub handler: fn(Vec<String>) -> Vec<String>,
}

fn get_commands() -> Vec<Command> {
    vec![
        Command {
        name: "randchars".into(),
        handler: |a| {
            let num_chars = a.first().map_or(600, |s| s.parse().unwrap_or(600));
            let num_messages = a.get(1).map_or(1, |s| s.parse().unwrap_or(1));
            let mut res = vec![];
            for _ in 0..num_messages {
                let mut bytes = String::new();
                for _ in 0..num_chars {
                    bytes.push(fastrand::char(..));
                }
                res.push(bytes);
            }

            res
        },
    },
    Command {
        name: "loginspam".into(),
        handler: |a| {
            if a.len() < 2 {
                println!("Not enough args - 2 required");
                return vec![];
            }
            let args = Args::parse();
            let user = a.first().unwrap();
            let pass = a.get(1).unwrap();
            let logins: u64 = a.get(2).unwrap_or(&"10".to_string()).parse().unwrap_or(10);
            (0..logins).into_par_iter().for_each(|_| {
                let mut stream = std::net::TcpStream::connect((args.addr.clone(), args.port)).unwrap();
                sleep_ms(230);
                stream.write_all(user.as_bytes()).unwrap();
                sleep_ms(230);
                stream.write_all(pass.as_bytes()).unwrap();
                sleep_ms(300);
            });
            vec![]
        },
    }
    ]
}

pub fn run_cmd_threaded(input: String, stream: &TcpStream, app: Arc<RwLock<App>>) {
    let cloned_stream = stream.try_clone().unwrap();
    spawn(|| { command_handler(input, cloned_stream, app) });
}

fn command_handler(input: String, mut stream: TcpStream, app: Arc<RwLock<App>>) {
    let split: Vec<&str> = input.split(' ').collect();
    let cmd = split.first().unwrap();
    let cmd_filt = get_commands().into_iter().filter(|c| c.name == *cmd);
    for cmd in cmd_filt {
        let args: Vec<String> = input.split(' ').map(String::from).skip(1).collect();
        for mut m in (cmd.handler)(args) {
            while m.len() > 4096 {
                m.pop();
            }
            stream
                .write_all(m.as_bytes())
                .expect("Failed to write to stream");
            sleep(Duration::from_millis(50));
        }
        app.write().unwrap().messages.push(format!("{} Command {} finished executing", "[crn]".purple(), cmd.name))
    }
}