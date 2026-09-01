use crate::exercise::Exercise;
use crate::lua_runner::Outcome;
use crate::progress::ProgressStore;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct LastRun {
    pub output: Vec<String>,
    pub outcome: Outcome,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyExercises;

impl std::fmt::Display for EmptyExercises {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no exercises were loaded: info.json declared none")
    }
}

impl std::error::Error for EmptyExercises {}

pub struct App {
    exercises: Vec<Exercise>,
    progress: ProgressStore,
    selected: usize,
    watched: Option<PathBuf>,
    last_run: Option<LastRun>,
}

impl App {
    pub fn new(exercises: Vec<Exercise>, progress: ProgressStore) -> Result<Self, EmptyExercises> {
        if exercises.is_empty() {
            return Err(EmptyExercises);
        }
        Ok(Self {
            exercises,
            progress,
            selected: 0,
            watched: None,
            last_run: None,
        })
    }

    pub fn exercises(&self) -> &[Exercise] {
        &self.exercises
    }

    pub fn selected_exercise(&self) -> &Exercise {
        &self.exercises[self.selected]
    }

    pub fn selected_index(&self) -> usize {
        self.selected
    }

    pub fn is_done(&self, exercise_path: &str) -> bool {
        self.progress.is_done(exercise_path)
    }

    pub fn watched(&self) -> Option<&Path> {
        self.watched.as_deref()
    }

    pub fn last_run(&self) -> Option<&LastRun> {
        self.last_run.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{App, EmptyExercises};
    use crate::exercise::{Exercise, Mode};
    use crate::progress::ProgressStore;

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

    #[test]
    fn new_succeeds_and_selects_the_first_exdercise_by_default() {
        assert!(matches!(
            App::new(vec![], empty_progress()),
            Err(EmptyExercises)
        ));
    }

    #[test]
    fn new_succeeds_and_selects_the_first_exercise_by_default() {
        let exercises = vec![
            exercise("01_junior", "01_variables", "variables1", "p1"),
            exercise("01_junior", "01_variables", "variables2", "p2"),
        ];

        let app = App::new(exercises, empty_progress()).unwrap();

        assert_eq!(app.selected_index(), 0);
        assert_eq!(app.selected_exercise().name, "variables1");
    }

    #[test]
    fn new_starts_with_no_watched_exercise() {
        let exercises = vec![exercise("01_junior", "01_variables", "variables1", "p1")];
        let app = App::new(exercises, empty_progress()).unwrap();

        assert_eq!(app.watched(), None);
    }

    #[test]
    fn new_starts_with_no_last_run() {
        let exercises = vec![exercise("01_junio", "01_variables", "variables1", "p1")];
        let app = App::new(exercises, empty_progress()).unwrap();

        assert!(app.last_run().is_none());
    }

    #[test]
    fn selected_exercise_returns_the_exercise_at_the_selected_index() {
        let exercises = vec![
            exercise("01_junior", "01_variables", "variables1", "p1"),
            exercise("01_junior", "01_variables", "variables2", "p2"),
        ];
        let app = App::new(exercises, empty_progress()).unwrap();

        assert_eq!(app.selected_exercise().path, "p1");
    }

    #[test]
    fn exercises_preserves_the_input_order() {
        let exercises = vec![
            exercise("01_junior", "zeta_module", "z1", "p1"),
            exercise("01_junior", "alpha_module", "a1", "p2"),
        ];
        let app = App::new(exercises, empty_progress()).unwrap();

        let names: Vec<&str> = app.exercises().iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["z1", "a1"]);
    }

    #[test]
    fn is_done_reflects_the_progress_store() {
        let dir = tempfile::tempdir().unwrap();
        let mut progress = ProgressStore::load(&dir.path().join("progress.json")).unwrap();
        progress.mark_done("p1").unwrap();

        let exercises = vec![exercise("01_junior", "01_variables", "variables1", "p1")];
        let app = App::new(exercises, progress).unwrap();

        assert!(app.is_done("p1"));
    }

    #[test]
    fn is_done_is_false_for_an_unmarked_exercise() {
        let exercises = vec![exercise("01_junior", "01_variables", "variables1", "p1")];
        let app = App::new(exercises, empty_progress()).unwrap();

        assert!(!app.is_done("p1"));
    }
}
