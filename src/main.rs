#![allow(deprecated)]

use std::io::{Read, Write};
use crate::command::Command;
use clap::Parser;
use owo_colors::OwoColorize;
use serde_json::Value;
use std::process::exit;
use std::thread::sleep_ms;
use std::time::Duration;
use rayon::prelude::*;
use tokio::io::{split, stdin, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::time::sleep;

mod cli;
mod command;

async fn s2c_t(mut r_server: ReadHalf<TcpStream>) {
    let mut buffer = [0u8; 8000];
    loop {
        match r_server.read(&mut buffer).await {
            Ok(n_bytes) => {
                if n_bytes == 0 {
                    println!("Server closed connection, exiting");
                    exit(0);
                }
                let read = &buffer[..n_bytes];
                let mut msg_str = String::from_utf8_lossy(read).to_string();
                while !msg_str.ends_with("}") {
                    let mut tmp = [0u8; 2048];
                    let n = r_server.read(&mut tmp).await.unwrap();
                    msg_str.push_str(&String::from_utf8_lossy(&tmp[..n]).to_string());
                }
                let msg: Value = match serde_json::from_str(&msg_str) {
                    Ok(ok) => ok,
                    Err(e) => {
                        println!("{} error desering packet ({msg_str}): {e}", "[err]".red());
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
                        msg_str
                    ),
                }
            }

            Err(err) => {
                panic!("{err}");
            }
        }
    }
}

async fn c2s_t(mut w_server: WriteHalf<TcpStream>) {
    let cmds = vec![
        Command {
            name: "randchars",
            handler: |a| {
                let num_chars: u32 = match a.get(0) {
                    Some(s) => s.parse().unwrap_or(600),
                    None => 600,
                };
                let num_messages: u64 = match a.get(1) {
                    Some(s) => s.parse().unwrap_or(1),
                    None => 1,
                };
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
                let user = a.get(0).unwrap();
                let pass = a.get(1).unwrap();
                let logins: u64 = a.get(2).unwrap_or(&"10".to_string()).parse().unwrap_or(10);
                (0..logins).into_par_iter().for_each(|_| {
                    let mut stream = std::net::TcpStream::connect((args.addr.clone(), args.port)).unwrap();
                    sleep_ms(500);
                    stream.write(user.as_bytes()).unwrap();
                    sleep_ms(500);
                    stream.write(pass.as_bytes()).unwrap();
                    sleep_ms(500);
                    let mut buf = [0u8; 4096];
                    stream.read(&mut buf).unwrap();
                });
                vec![]
            },
        },
    ];
    let mut buff = [0u8; 2048];
    loop {
        let n = stdin().read(&mut buff).await.expect("Failed to read line");
        let read_str = String::from_utf8(buff[..n].to_vec()).unwrap();
        let read_str = read_str.trim().to_string();
        if read_str.is_empty() {
            continue;
        }
        let f = read_str.split(" ").nth(0).unwrap();
        let cmd_filt = cmds.iter().filter(|c| c.name == f);
        for cmd in cmd_filt.clone() {
            let args: Vec<String> = read_str.split(" ").map(String::from).skip(1).collect();
            for mut m in (cmd.handler)(args) {
                while m.len() > 4096 {
                    m.pop();
                }
                w_server
                    .write_all(&m.as_bytes())
                    .await
                    .expect("Failed to write to stream");
                sleep(Duration::from_millis(50)).await;
            }
        }
        if cmd_filt.count() > 0 {
            continue;
        }
        let read_str = read_str.trim_end_matches("\n");
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
