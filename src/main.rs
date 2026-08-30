use clap::Parser;
use lualings::cli::{self, Cli, Commands, render_exercise_list, render_run_result};
use lualings::exercise::{self, Exercise, Mode};
use lualings::lua_runner;
use lualings::progress::{self, ProgressStore};
use std::path::Path;

fn load_exercises_or_exit() -> Vec<Exercise> {
    match exercise::load(Path::new(exercise::DEFAULT_INFO_PATH)) {
        Ok(exercises) => exercises,
        Err(err) => {
            eprintln!("error: could not load info.json: {err}");
            std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
        }
    }
}

fn load_progress_or_exit() -> ProgressStore {
    match ProgressStore::load(Path::new(progress::DEFAULT_PROGRESS_PATH)) {
        Ok(progress) => progress,
        Err(err) => {
            eprintln!("error: could not load progress: {err}");
            std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let exercises = load_exercises_or_exit();
            let progress = load_progress_or_exit();

            print!("{}", render_exercise_list(&exercises, &progress));
        }
        Commands::Run { name } => {
            let exercises = load_exercises_or_exit();
            let exercise = match Exercise::find_by_name(&exercises, &name) {
                Some(exercise) => exercise,
                None => {
                    eprintln!("error: noexercise named '{name}' was found in info.json");
                    std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
                }
            };

            let source = match exercise.read_source() {
                Ok(source) => source,
                Err(err) => {
                    eprintln!(
                        "error: the exercise file for '{name}' was not found on disk \
                        (expected at: {}): {err}",
                        exercise.path
                    );
                    std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
                }
            };

            let outcome = match exercise.mode {
                Mode::Compile => lua_runner::run_compile(&source),
                Mode::Test => lua_runner::run_test(&source),
            };

            print!("{}", render_run_result(&exercise.name, &outcome));

            match outcome {
                lua_runner::Outcome::Pass => {
                    let mut progress = load_progress_or_exit();
                    if let Err(err) = progress.mark_done(&exercise.path) {
                        eprintln!("error: exercise passed but progress could not be saved: {err}");
                        std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
                    }
                }
                lua_runner::Outcome::Fail(_) | lua_runner::Outcome::Timeout => {
                    std::process::exit(cli::EXIT_CONTENT_FAILURE);
                }
            }
        }
        Commands::Watch => todo!("implement `watch`"),
        Commands::Init => todo!("implement `init`"),
        Commands::Hint {
            name: _,
            solution: _,
        } => todo!("implement `hint`"),
    }
}
