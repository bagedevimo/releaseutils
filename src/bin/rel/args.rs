use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long = "build")]
    build_number: u32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start { name: String },
    Finalise {},
}

pub fn parse() -> Args {
    Args::parse()
}
