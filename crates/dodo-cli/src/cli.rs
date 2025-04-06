use clap::Parser;
use crate::commands::Command;

#[derive(Parser)]
#[command(name = "dodo", version = "0.1.0", about = "Dodo CLI")]
pub struct DodoCli {
    #[command(subcommand)]
    pub command: Command,
}
