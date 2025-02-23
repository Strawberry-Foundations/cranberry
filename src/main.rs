use std::env;
use std::io::{stdout, Result};

use ratatui::prelude::*;

use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use crossterm::ExecutableCommand;

use crate::tui::app::App;

mod net;
mod tui;
mod util;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let default_addr = String::from("127.0.0.1");

    let address = args.get(1).unwrap_or(&default_addr);
    let port: u16 = args.get(2).map_or(52800, |p| p.parse().unwrap_or(52800));

    let host = (address.clone(), port);

    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    stdout().execute(EnterAlternateScreen)?;
    terminal.clear()?;

    let app = App::default();
    app.run(&mut terminal, host);

    util::terminal::cleanup();
    Ok(())
}
