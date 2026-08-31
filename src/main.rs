use clap::Parser;
use lualings::cli::{
    self, Cli, Commands, first_pending, render_exercise_list, render_hint, render_run_result,
    render_solution,
};
use lualings::exercise::{self, Exercise, Mode};
use lualings::lua_runner;
use lualings::progress::{self, ProgressStore};
use lualings::watcher;
use std::path::{Path, PathBuf};

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

fn execute_and_report(exercise: &Exercise) -> lua_runner::Outcome {
    let source = match exercise.read_source() {
        Ok(source) => source,
        Err(err) => {
            eprintln!(
                "error: the exercise file for '{}' was not found on disk \
                (expected at:{}): {err}",
                exercise.name, exercise.path
            );
            std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
        }
    };

    let outcome = match exercise.mode {
        Mode::Compile => lua_runner::run_compile(&source),
        Mode::Test => lua_runner::run_compile(&source),
    };

    print!("{}", render_run_result(&exercise.name, &outcome));

    if matches!(outcome, lua_runner::Outcome::Pass) {
        let mut progress = load_progress_or_exit();
        if let Err(err) = progress.mark_done(&exercise.path) {
            eprintln!("error: exercise passed but progress could not be save: {err}");
            std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
        }
    }

    outcome
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

            let outcome = execute_and_report(exercise);

            if matches!(
                outcome,
                lua_runner::Outcome::Fail(_) | lua_runner::Outcome::Timeout
            ) {
                std::process::exit(cli::EXIT_CONTENT_FAILURE);
            }
        }
        Commands::Watch => {
            let exercises = load_exercises_or_exit();

            let (handle, rx) = match watcher::watch(Path::new(exercise::DEFAULT_EXERCISES_DIR)) {
                Ok(watch) => watch,
                Err(err) => {
                    eprintln!("error: could not start watcher: {err}");
                    std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
                }
            };

            loop {
                let progress = load_progress_or_exit();
                let active = match first_pending(&exercises, &progress) {
                    Some(exercise) => exercise,
                    None => {
                        println!("All exercises are complete!");
                        return;
                    }
                };
                handle.set_active(Some(PathBuf::from(&active.path)));

                loop {
                    let outcome = execute_and_report(active);
                    if matches!(outcome, lua_runner::Outcome::Pass) {
                        break;
                    }
                    if rx.recv().is_err() {
                        eprintln!("error: the watcher stopped unexpectedly");
                        std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
                    }
                }
            }
        }
        Commands::Init => todo!("implement `init`"),
        Commands::Hint { name, solution } => {
            let exercises = load_exercises_or_exit();
            let exercise = match Exercise::find_by_name(&exercises, &name) {
                Some(exercise) => exercise,
                None => {
                    eprintln!("error: no exercise named '{name}' was found in info.json");
                    std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
                }
            };

            if solution {
                match exercise.read_source() {
                    Ok(content) => print!("{}", render_solution(&exercise.name, &content)),
                    Err(_) => {
                        eprintln!("No solution is available yet for '{name}'.");
                        std::process::exit(cli::EXIT_OPERATIONAL_ERROR);
                    }
                }
            } else {
                print!("{}", render_hint(&exercise.name, &exercise.hint));
            }
        }
    }
}
