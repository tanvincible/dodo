pub mod init;
pub mod build;
pub mod add;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    Init,
    Build,
    Add {
        #[arg(help = "Name of external integration to add")]
        name: String,
    },
}
