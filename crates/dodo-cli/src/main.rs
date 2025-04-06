mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => commands::init::handle(),
        Command::Build => commands::build::handle(),
        Command::Add { integration } => commands::add::handle(&integration),
    };
}
