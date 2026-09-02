use crate::exercise::Exercise;
use crate::lua_runner::Outcome;
use crate::progress::ProgressStore;
use crossterm::event::KeyCode;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Continue,
    Quit,
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct ExerciseNotFound(pub String);

impl std::fmt::Display for ExerciseNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no exercise named '{}' was found", self.0)
    }
}

impl std::error::Error for ExerciseNotFound {}

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

    fn set_selected(&mut self, idx: usize) {
        if idx == self.selected {
            return;
        }
        self.watched = Some(PathBuf::from(self.exercises[idx].path.clone()));
        self.selected = idx;
        self.last_run = None;
    }

    pub fn next(&mut self) {
        let idx = (self.selected + 1).min(self.exercises.len() - 1);
        self.set_selected(idx);
    }

    pub fn previous(&mut self) {
        let idx = self.selected.saturating_sub(1);
        self.set_selected(idx);
    }

    pub fn select(&mut self, idx: usize) {
        self.set_selected(idx.min(self.exercises.len() - 1));
    }

    pub fn select_by_name(&mut self, name: &str) -> Result<(), ExerciseNotFound> {
        let idx = self
            .exercises
            .iter()
            .position(|exercise| exercise.name == name)
            .ok_or_else(|| ExerciseNotFound(name.to_string()))?;
        self.set_selected(idx);
        Ok(())
    }

    pub fn record_run(&mut self, output: Vec<String>, outcome: Outcome) {
        if matches!(outcome, Outcome::Pass) {
            let path = self.exercises[self.selected].path.clone();
            let _ = self.progress.mark_done(&path);
        }
        self.last_run = Some(LastRun { output, outcome });
    }

    pub fn handle_key(&mut self, key: KeyCode) -> Action {
        match key {
            KeyCode::Char('j') | KeyCode::Down => {
                self.next();
                Action::Continue
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.previous();
                Action::Continue
            }
            KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
            _ => Action::Continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Action, App, EmptyExercises, ExerciseNotFound, LastRun};
    use crate::exercise::{Exercise, Mode};
    use crate::lua_runner::Outcome;
    use crate::progress::ProgressStore;
    use crossterm::event::KeyCode;

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

    fn three_exercises() -> Vec<Exercise> {
        vec![
            exercise("01_junior", "01_variables", "variables1", "p1"),
            exercise("01_junior", "01_variables", "variables2", "p2"),
            exercise("01_junior", "01_variables", "variables3", "p3"),
        ]
    }

    #[test]
    fn next_advances_to_the_next_exercise() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        app.next();

        assert_eq!(app.selected_index(), 1);
        assert_eq!(app.selected_exercise().name, "variables2");
    }

    #[test]
    fn next_stops_at_the_last_exercise_without_wrapping() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        app.next();
        app.next();
        app.next();
        app.next();

        assert_eq!(app.selected_index(), 2);
    }

    #[test]
    fn previous_moves_to_the_previous_exercise() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();
        app.next();
        app.next();

        app.previous();

        assert_eq!(app.selected_index(), 1);
    }

    #[test]
    fn previous_stops_at_the_first_exercise_without_wrapping() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        app.previous();
        app.previous();

        assert_eq!(app.selected_index(), 0);
    }

    #[test]
    fn select_moves_to_the_given_index() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        app.select(2);

        assert_eq!(app.selected_index(), 2);
        assert_eq!(app.selected_exercise().name, "variables3");
    }

    #[test]
    fn select_clamps_an_out_of_range_index_to_the_last_exercise() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        app.select(100);

        assert_eq!(app.selected_index(), 2);
    }

    #[test]
    fn select_by_name_moves_to_the_matching_exercise() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        app.select_by_name("variables3").unwrap();

        assert_eq!(app.selected_index(), 2);
    }

    #[test]
    fn select_by_name_returns_error_and_leaves_state_unchanged_for_unknown_name() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        let err = app.select_by_name("no_exist").unwrap_err();

        assert_eq!(err, ExerciseNotFound("no_exist".to_string()));
        assert_eq!(app.selected_index(), 0);
    }

    #[test]
    fn navigating_resets_last_run_to_none() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();
        app.last_run = Some(LastRun {
            output: vec!["hola".to_string()],
            outcome: Outcome::Pass,
        });

        app.next();

        assert!(app.last_run().is_none());
    }

    #[test]
    fn navigating_updates_watched_to_the_new_exercise_path() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        app.next();

        assert_eq!(app.watched(), Some(std::path::Path::new("p2")));
    }

    #[test]
    fn moving_to_the_current_index_is_a_noop_and_does_not_reset_last_run() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();
        app.last_run = Some(LastRun {
            output: vec![],
            outcome: Outcome::Pass,
        });

        app.select(0);

        assert!(app.last_run().is_some());
    }

    #[test]
    fn record_run_on_pass_marks_the_selected_exercise_done() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        app.record_run(vec!["ok".to_string()], Outcome::Pass);

        assert!(app.is_done("p1"));
        assert_eq!(
            app.last_run(),
            Some(&LastRun {
                output: vec!["ok".to_string()],
                outcome: Outcome::Pass
            })
        );
    }

    #[test]
    fn record_run_on_fail_does_not_mark_done() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        app.record_run(vec![], Outcome::Fail("boom".to_string()));

        assert!(!app.is_done("p1"));
    }

    #[test]
    fn record_run_always_overwrites_last_run() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();
        app.record_run(vec!["first".to_string()], Outcome::Fail("boom".to_string()));

        app.record_run(vec!["second".to_string()], Outcome::Pass);

        assert_eq!(
            app.last_run(),
            Some(&LastRun {
                output: vec!["second".to_string()],
                outcome: Outcome::Pass
            })
        );
    }

    #[test]
    fn handle_key_j_and_down_advance_to_the_next_exercise() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        assert_eq!(app.handle_key(KeyCode::Char('j')), Action::Continue);
        assert_eq!(app.selected_index(), 1);

        assert_eq!(app.handle_key(KeyCode::Down), Action::Continue);
        assert_eq!(app.selected_index(), 2);
    }

    #[test]
    fn handle_key_k_and_up_move_to_the_previous_exercise() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();
        app.select(2);

        assert_eq!(app.handle_key(KeyCode::Char('k')), Action::Continue);
        assert_eq!(app.selected_index(), 1);

        assert_eq!(app.handle_key(KeyCode::Up), Action::Continue);
        assert_eq!(app.selected_index(), 0);
    }

    #[test]
    fn handle_key_q_and_esc_return_quit() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        assert_eq!(app.handle_key(KeyCode::Char('q')), Action::Quit);
        assert_eq!(app.handle_key(KeyCode::Esc), Action::Quit);
    }

    #[test]
    fn handle_key_unknown_key_returns_continue_and_does_not_move() {
        let mut app = App::new(three_exercises(), empty_progress()).unwrap();

        assert_eq!(app.handle_key(KeyCode::Char('x')), Action::Continue);
        assert_eq!(app.selected_index(), 0);
    }
}
