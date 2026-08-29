use clap::Parser;
use lualings::cli::{Cli, Commands, render_exercise_list};
use lualings::exercise;
use lualings::progress::{self, ProgressStore};
use std::path::Path;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let exercises = match exercise::load(Path::new(exercise::DEFAULT_INFO_PATH)) {
                Ok(exercises) => exercises,
                Err(err) => {
                    eprintln!("error: could not load info.json: {err}");
                    std::process::exit(1);
                }
            };
            let progress = match ProgressStore::load(Path::new(progress::DEFAULT_PROGRESS_PATH)) {
                Ok(progress) => progress,
                Err(err) => {
                    eprintln!("error: could not load progress: {err}");
                    std::process::exit(1);
                }
            };
            print!("{}", render_exercise_list(&exercises, &progress));
        }
        Commands::Run { name: _ } => todo!("implement `run <name>`"),
        Commands::Watch => todo!("implement `watch`"),
        Commands::Init => todo!("implement `init`"),
        Commands::Hint {
            name: _,
            solution: _,
        } => todo!("implement `hint`"),
    }
}
