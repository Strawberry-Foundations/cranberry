use std::io::stdout;
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};

pub fn cleanup() {
    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}