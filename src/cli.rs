use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The address of the strawberry chat instance
    #[arg(short, long)]
    pub addr: String,

    /// The port of the instance
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
