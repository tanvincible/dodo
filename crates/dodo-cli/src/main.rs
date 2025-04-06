use clap::Parser;
use dodo_cli::cli::DodoCli;
use dodo_cli::commands;

fn main() {
    let cli = DodoCli::parse();
    match cli.command {
        commands::Command::Init => commands::init::handle(),
        commands::Command::Build => commands::build::handle(),
        commands::Command::Add { ref name } => commands::add::handle(name),
    }
}
