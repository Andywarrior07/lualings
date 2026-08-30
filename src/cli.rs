use crate::exercise::Exercise;
use crate::lua_runner::{self, Outcome};
use crate::progress::ProgressStore;
use clap::{Parser, Subcommand};
use std::fmt::Write as _;

pub const EXIT_CONTENT_FAILURE: i32 = 1;
pub const EXIT_OPERATIONAL_ERROR: i32 = 2;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List,
    Run {
        name: String,
    },
    Watch,
    Init,
    Hint {
        name: String,
        #[arg(long)]
        solution: bool,
    },
}

pub fn render_exercise_list(exercises: &[Exercise], progress: &ProgressStore) -> String {
    let mut out = String::new();
    let mut current_level: Option<&str> = None;
    let mut current_module: Option<&str> = None;

    for exercise in exercises {
        if current_level != Some(exercise.level.as_str()) {
            let _ = writeln!(out, "{}", exercise.level);
            current_level = Some(exercise.level.as_str());
            current_module = None;
        }
        if current_module != Some(exercise.module.as_str()) {
            let _ = writeln!(out, "  {}", exercise.module);
            current_module = Some(exercise.module.as_str());
        }
        let checkbox = if progress.is_done(&exercise.path) {
            "[x]"
        } else {
            "[ ]"
        };
        let _ = writeln!(out, "    {checkbox} {}", exercise.name);
    }

    out
}

pub fn render_run_result(name: &str, outcome: &Outcome) -> String {
    match outcome {
        Outcome::Pass => format!("[PASS] {name}\n"),
        Outcome::Fail(message) => format!("[FAIL] {name}\n  {message}\n"),
        Outcome::Timeout => format!(
            "[TIMEOUT] {name}\n  exceeded the {:?} execution time limit\n",
            lua_runner::DEFAULT_TIMEOUT_BUDGET
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{Cli, Commands, render_exercise_list, render_run_result};
    use crate::exercise::{Exercise, Mode};
    use crate::lua_runner::Outcome;
    use crate::progress::ProgressStore;
    use clap::Parser;

    fn exercise(level: &str, module: &str, name: &str, path: &str) -> Exercise {
        Exercise {
            name: name.to_string(),
            path: path.to_string(),
            mode: Mode::Compile,
            hint: "hint".to_string(),
            level: level.to_string(),
            module: module.to_string(),
        }
    }

    fn empty_progress() -> ProgressStore {
        let dir = tempfile::tempdir().unwrap();
        ProgressStore::load(&dir.path().join("progress.json")).unwrap()
    }

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(std::iter::once("lualings").chain(args.iter().copied()))
    }

    #[test]
    fn no_subcommand_is_an_error() {
        assert!(parse(&[]).is_err());
    }

    #[test]
    fn list_parses_without_arguments() {
        let cli = parse(&["list"]).unwrap();
        assert!(matches!(cli.command, Commands::List));
    }

    #[test]
    fn watch_parses_without_arguments() {
        let cli = parse(&["watch"]).unwrap();
        assert!(matches!(cli.command, Commands::Watch));
    }

    #[test]
    fn init_parses_without_arguments() {
        let cli = parse(&["init"]).unwrap();
        assert!(matches!(cli.command, Commands::Init));
    }

    #[test]
    fn run_requires_name() {
        assert!(parse(&["run"]).is_err());
    }

    #[test]
    fn run_parses_with_name() {
        let cli = parse(&["run", "variables1"]).unwrap();
        match cli.command {
            Commands::Run { name } => assert_eq!(name, "variables1"),
            other => panic!("expected Commands::Run, got {other:?}"),
        }
    }

    #[test]
    fn hint_requires_name() {
        assert!(parse(&["hint"]).is_err());
    }

    #[test]
    fn hint_parses_with_name_and_defaults_solution_to_false() {
        let cli = parse(&["hint", "variables1"]).unwrap();
        match cli.command {
            Commands::Hint { name, solution } => {
                assert_eq!(name, "variables1");
                assert!(!solution);
            }
            other => panic!("expected Commands::Hint, got {other:?}"),
        }
    }

    #[test]
    fn hint_parses_solution_flag_alongside_name() {
        let cli = parse(&["hint", "variables1", "--solution"]).unwrap();
        match cli.command {
            Commands::Hint { name, solution } => {
                assert_eq!(name, "variables1");
                assert!(solution);
            }
            other => panic!("expected Commands::Hint, got {other:?}"),
        }
    }

    #[test]
    fn render_groups_by_level_and_module_without_repeating_headers() {
        let exercises = vec![
            exercise("01_junior", "01_variables", "variables1", "p1"),
            exercise("01_junior", "01_variables", "variables2", "p2"),
            exercise("01_junior", "02_types", "types1", "p3"),
            exercise("02_mid", "01_closures", "closures1", "p4"),
        ];

        let rendered = render_exercise_list(&exercises, &empty_progress());

        assert_eq!(
            rendered,
            "01_junior\n  01_variables\n    [ ] variables1\n    [ ] variables2\n  \
             02_types\n    [ ] types1\n02_mid\n  01_closures\n    [ ] closures1\n"
        );
    }

    #[test]
    fn render_preserves_input_order_even_if_not_alphabetical() {
        let exercises = vec![
            exercise("01_junior", "zeta_module", "z1", "p1"),
            exercise("01_junior", "alpha_module", "a1", "p2"),
        ];

        let rendered = render_exercise_list(&exercises, &empty_progress());
        let zeta_pos = rendered.find("zeta_module").unwrap();
        let alpha_pos = rendered.find("alpha_module").unwrap();
        assert!(zeta_pos < alpha_pos);
    }

    #[test]
    fn render_reflects_progress_per_exercise() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = ProgressStore::load(&dir.path().join("progress.json")).unwrap();
        store.mark_done("p1").unwrap();

        let exercises = vec![
            exercise("01_junior", "01_variables", "variables1", "p1"),
            exercise("01_junior", "01_variables", "variables2", "p2"),
        ];

        let rendered = render_exercise_list(&exercises, &store);

        assert!(rendered.contains("[x] variables1"));
        assert!(rendered.contains("[ ] variables2"));
    }

    #[test]
    fn render_marks_everything_pending_when_progress_is_empty() {
        let exercises = vec![exercise("01_junior", "01_variables", "variables1", "p1")];

        let rendered = render_exercise_list(&exercises, &empty_progress());

        assert!(rendered.contains("[ ] variables1"));
        assert!(!rendered.contains("[x]"));
    }

    #[test]
    fn render_run_result_pass() {
        let rendered = render_run_result("variables1", &Outcome::Pass);
        assert_eq!(rendered, "[PASS] variables1\n");
    }

    #[test]
    fn render_run_result_fail_includes_the_full_message() {
        let rendered = render_run_result("variables3", &Outcome::Fail("boom".to_string()));
        assert_eq!(rendered, "[FAIL] variables3\n  boom\n");
    }

    #[test]
    fn render_run_result_timeout_mentions_the_real_budget() {
        let rendered = render_run_result("loop_infinity", &Outcome::Timeout);
        assert!(rendered.starts_with("[TIMEOUT] loop_infinity\n"));
        assert!(rendered.contains("2s"));
    }
}
