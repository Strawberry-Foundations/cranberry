#![allow(deprecated)]
#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
)]

use std::io::Write;
use crate::command::Command;
use clap::Parser;
use owo_colors::OwoColorize;
use serde_json::Value;
use std::process::exit;
use std::thread::sleep_ms;
use std::time::Duration;
use rayon::prelude::*;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use tokio::io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::time::sleep;

mod cli;
mod command;

async fn s2c_t(mut r_server: ReadHalf<TcpStream>) {
    loop {
        let mut buff = [0u8; 1];
        let mut str_buf = String::new();
        let mut wraps = 0;
        loop {
            let n_bytes = r_server.read(&mut buff).await.expect("Failed to read from stream");
            if n_bytes == 0 {
                println!("Server closed connection, exiting");
                exit(0);
            }
            match buff[0] as char {
                '{' => {
                    wraps += 1;
                    str_buf.push('{');
                }
                '}' => {
                    wraps -= 1;
                    str_buf.push('}');
                }
                c => str_buf.push(c)
            }
            if wraps == 0 {
                break
            }
            //dbg!(wraps, &str_buf);
        }
        let msg: Value = match serde_json::from_str(&str_buf) {
            Ok(ok) => ok,
            Err(e) => {
                println!("{} error desering packet ({str_buf}): {e}", "[err]".red());
                continue;
            }
        };
        match msg["message_type"].as_str() {
            Some("system_message") => println!(
                "{} {}",
                "[sys]".bright_green(),
                msg["message"]["content"].as_str().unwrap()
            ),
            Some("stbchat_backend") => {}
            Some("user_message") => println!(
                "{} {} ({}) -> {}",
                "[msg]".bright_blue(),
                msg["username"].as_str().unwrap(),
                msg["nickname"].as_str().unwrap(),
                msg["message"]["content"].as_str().unwrap()
            ),
            None => unreachable!(),
            m => println!(
                "{} Unimplemented packet {} - full packet: {}",
                "[uimp]".red(),
                m.unwrap(),
                str_buf
            ),
        }
    }
}


async fn c2s_t(mut w_server: WriteHalf<TcpStream>) {
    let cmds = vec![
        Command {
            name: "randchars",
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
            name: "loginspam",
            handler: |a| {
                if a.len() < 2 {
                    println!("Not enough args - 2 required");
                    return vec![];
                }
                let args = cli::Args::parse();
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
        },
    ];
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let line = match rl.readline("") {
            Ok(l) => {
                rl.add_history_entry(&l).unwrap();
                l
            },
            Err(ReadlineError::Eof) => exit(0),
            Err(ReadlineError::Interrupted) => {
                w_server.write_all("/exit".as_bytes()).await.expect("Failed to write to stream");
                sleep(Duration::from_millis(400)).await;
                exit(0);
            },
            Err(why) => panic!("{why}")
        };
        let f = line.split(' ').next().unwrap();
        let cmd_filt = cmds.iter().filter(|c| c.name == f);
        for cmd in cmd_filt.clone() {
            let args: Vec<String> = line.split(' ').map(String::from).skip(1).collect();
            for mut m in (cmd.handler)(args) {
                while m.len() > 4096 {
                    m.pop();
                }
                w_server
                    .write_all(m.as_bytes())
                    .await
                    .expect("Failed to write to stream");
                sleep(Duration::from_millis(50)).await;
            }
        }
        if cmd_filt.count() > 0 {
            continue;
        }
        let read_str = line.trim_end_matches('\n');
        w_server
            .write_all(read_str.as_bytes())
            .await
            .expect("Failed to write to stream");
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();
    rayon::ThreadPoolBuilder::new().num_threads(5).build_global().unwrap();
    let args = cli::Args::parse();
    let stream = TcpStream::connect((args.addr, args.port)).await?;
    let halves = split(stream);
    tokio::spawn(s2c_t(halves.0));
    tokio::spawn(c2s_t(halves.1)).await.expect("Join error");
    Ok(())
}
