use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dodo",
    version,
    about = "Scaffold your GitHub workflows automatically."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    Build,
    Add { integration: String },
}
