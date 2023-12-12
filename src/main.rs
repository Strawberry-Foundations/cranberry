#![warn(clippy::all, clippy::nursery)]

mod app;
mod cli;
mod net_handler;
mod tui;
mod commands;

use crate::cli::Args;
use better_panic::Settings;
use clap::Parser;
use crossterm::execute;
use crossterm::terminal::*;
use ratatui::prelude::*;
use std::io;
use std::io::stdout;
use crate::app::App;

pub fn initialize_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        Terminal::new(CrosstermBackend::new(stdout())).unwrap().show_cursor().unwrap();
        Settings::auto()
            .most_recent_first(true)
            .lineno_suffix(true)
            .create_panic_handler()(panic_info);
    }));
}

fn main() -> io::Result<()> {
    initialize_panic_handler();
    Args::parse();
    rayon::ThreadPoolBuilder::new().num_threads(3).build_global().unwrap();
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let app = App::default();
    app.run(&mut terminal);
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}
