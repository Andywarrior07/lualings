use clap::{Parser, Subcommand};

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

#[cfg(test)]
mod tests {
    use super::{Cli, Commands};
    use clap::Parser;

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
}
