use crate::app::{Action, App};
use crate::exercise::{self, Mode};
use crate::lua_runner;
use crate::progress::{self, ProgressStore};
use crate::ui;
use crate::watcher::{self, ChangeEvent, WatchHandle};
use crossterm::event::{self, Event};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

const POLL_TIMEOUT: Duration = Duration::from_millis(100);

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let exercises = exercise::load(Path::new(exercise::DEFAULT_INFO_PATH))?;
    let progress = ProgressStore::load(Path::new(progress::DEFAULT_PROGRESS_PATH))?;
    let mut app = App::new(exercises, progress)?;

    let (handle, rx) = watcher::watch(Path::new(exercise::DEFAULT_EXERCISES_DIR))?;
    sync_watched(&handle, &app);

    ratatui::run(|terminal| event_loop(terminal, &mut app, &handle, &rx)).map_err(Into::into)
}

fn sync_watched(handle: &WatchHandle, app: &App) {
    handle.set_active(Some(PathBuf::from(&app.selected_exercise().path)));
}

fn event_loop(
    terminal: &mut ratatui::prelude::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    handle: &WatchHandle,
    rx: &Receiver<ChangeEvent>,
) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if event::poll(POLL_TIMEOUT)?
            && let Event::Key(key) = event::read()?
        {
            match app.handle_key(key.code) {
                Action::Quit => return Ok(()),
                Action::Continue => sync_watched(handle, app),
            }
        }

        match rx.try_recv() {
            Ok(_change_event) => run_and_record(app),
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                return Err(std::io::Error::other("the watcher stopped unexpectedly"));
            }
        }
    }
}

fn run_and_record(app: &mut App) {
    let exercise = app.selected_exercise();
    let mode = exercise.mode;
    let Ok(source) = exercise.read_source() else {
        return;
    };

    let (output, outcome) = match mode {
        Mode::Compile => lua_runner::run_compile_capturing(&source),
        Mode::Test => lua_runner::run_test_capturing(&source),
    };

    app.record_run(output, outcome);
}
