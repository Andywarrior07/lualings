use clap::Parser;
use lualings::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => todo!("implement `list`"),
        Commands::Run { name: _ } => todo!("implement `run <name>`"),
        Commands::Watch => todo!("implement `watch`"),
        Commands::Init => todo!("implement `init`"),
        Commands::Hint {
            name: _,
            solution: _,
        } => todo!("implement `hint`"),
    }
}
