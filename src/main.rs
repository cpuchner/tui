use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    P,
}

mod kitty;
mod protos;
mod dev;

const DEFAULT_SOCKET_PATH: &str = "unix:/tmp/kitty";

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::P => protos::bump_protos(),
    }
}
