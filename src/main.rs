use std::io::{stdout, Result};
use std::env;

use ratatui::prelude::*;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;

use crate::tui::app::App;

mod tui;
mod net;

fn main() -> Result<()>  {
    let args: Vec<String> = env::args().collect();
    
    let default_addr = String::from("127.0.0.1");
    
    let address = args.get(1).unwrap_or(&default_addr);
    let port: u16 = args.get(2).map_or(8080, |p| p.parse().unwrap_or(8080));

    let host = (address.clone(), port);


    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    stdout().execute(EnterAlternateScreen)?;
    terminal.clear()?;

    let app = App::default();
    app.run(&mut terminal, host);

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
