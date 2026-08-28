use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeEvent {
    pub path: PathBuf,
}

fn is_lua_path(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("lua")
}

pub const DEFAULT_DEBOUNCE_WINDOW: Duration = Duration::from_millis(300);

const FLUSH_TICK: Duration = Duration::from_millis(20);

#[derive(Debug, Clone, Eq, PartialEq)]
enum ActiveFilter {
    Unfiltered,
    Filtered(Option<PathBuf>),
}

fn matches_active(canonica_event_path: &Path, filter: &ActiveFilter) -> bool {
    match filter {
        ActiveFilter::Unfiltered => true,
        ActiveFilter::Filtered(None) => false,
        ActiveFilter::Filtered(Some(active_path)) => canonica_event_path == active_path,
    }
}

pub struct WatchHandle {
    _watcher: RecommendedWatcher,
    stop: Arc<AtomicBool>,
    active: Arc<Mutex<ActiveFilter>>,
}

impl Drop for WatchHandle {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl WatchHandle {
    pub fn set_active(&self, path: Option<PathBuf>) {
        let canonical = path.and_then(|p| p.canonicalize().ok());
        *self.active.lock().unwrap() = ActiveFilter::Filtered(canonical);
    }
}

fn watch_with_window(
    dir: &Path,
    window: Duration,
) -> notify::Result<(WatchHandle, Receiver<ChangeEvent>)> {
    let pending: Arc<Mutex<HashMap<PathBuf, Instant>>> = Arc::new(Mutex::new(HashMap::new()));
    let pending_for_callback = Arc::clone(&pending);

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        let Ok(event) = res else {
            return;
        };
        for path in event.paths {
            if is_lua_path(&path) {
                pending_for_callback
                    .lock()
                    .unwrap()
                    .insert(path, Instant::now());
            }
        }
    })?;
    watcher.watch(dir, RecursiveMode::Recursive)?;

    let (tx, rx) = mpsc::channel::<ChangeEvent>();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_for_flusher = Arc::clone(&stop);
    let active = Arc::new(Mutex::new(ActiveFilter::Unfiltered));
    let active_for_flusher = Arc::clone(&active);

    thread::spawn(move || {
        while !stop_for_flusher.load(Ordering::Relaxed) {
            thread::sleep(FLUSH_TICK);

            let due: Vec<PathBuf> = {
                let mut guard = pending.lock().unwrap();
                let now = Instant::now();
                let ready: Vec<PathBuf> = guard
                    .iter()
                    .filter(|(_, seen)| now.duration_since(**seen) >= window)
                    .map(|(path, _)| path.clone())
                    .collect();
                for path in &ready {
                    guard.remove(path);
                }
                ready
            };

            for path in due {
                let filter_snapshot = active_for_flusher.lock().unwrap().clone();
                let should_send = match filter_snapshot {
                    ActiveFilter::Unfiltered => true,
                    ActiveFilter::Filtered(_) => path
                        .canonicalize()
                        .is_ok_and(|canonical| matches_active(&canonical, &filter_snapshot)),
                };

                if should_send && tx.send(ChangeEvent { path }).is_err() {
                    return;
                }
            }
        }
    });

    Ok((
        WatchHandle {
            _watcher: watcher,
            stop,
            active,
        },
        rx,
    ))
}

pub fn watch(dir: &Path) -> notify::Result<(WatchHandle, Receiver<ChangeEvent>)> {
    watch_with_window(dir, DEFAULT_DEBOUNCE_WINDOW)
}

#[cfg(test)]
mod tests {
    use super::{ActiveFilter, is_lua_path, matches_active, watch, watch_with_window};
    use std::path::{Path, PathBuf};
    use std::sync::mpsc::RecvTimeoutError;
    use std::time::{Duration, Instant};

    #[test]
    fn is_lua_path_true_for_lua_extension() {
        assert!(is_lua_path(Path::new("foo.lua")));
        assert!(is_lua_path(Path::new("dir/sub/foo.lua")));
    }

    #[test]
    fn is_lua_path_false_for_non_lua_extension() {
        assert!(!is_lua_path(Path::new("foo.txt")));
        assert!(!is_lua_path(Path::new("foo.LUA")));
    }

    #[test]
    fn is_lua_path_false_for_no_extension() {
        assert!(!is_lua_path(Path::new("foo")));
    }

    const RECV_TIMEOUT: Duration = Duration::from_secs(5);

    #[test]
    fn watch_detects_change_to_lua_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("foo.lua");
        std::fs::write(&file, "local x = 1").unwrap();

        let (_watcher, rx) = watch(dir.path()).unwrap();

        std::fs::write(&file, "local x = 2").unwrap();

        let event = rx
            .recv_timeout(RECV_TIMEOUT)
            .expect("expected a change event");
        assert_eq!(event.path.file_name(), file.file_name());
    }

    #[test]
    fn watch_ignores_change_to_non_lua_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("notes.txt");
        std::fs::write(&file, "hola").unwrap();

        let (_watcher, rx) = watch(dir.path()).unwrap();

        std::fs::write(&file, "chau").unwrap();

        match rx.recv_timeout(RECV_TIMEOUT) {
            Err(RecvTimeoutError::Timeout) => {}
            other => panic!("expected timeout (no event), got {other:?}"),
        }
    }

    #[test]
    fn watch_runs_continuously_across_multiple_writes() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("foo.lua");
        std::fs::write(&file, "local x = 0").unwrap();

        let (_watcher, rx) = watch(dir.path()).unwrap();

        for i in 1..=3 {
            std::fs::write(&file, format!("local x = {i}")).unwrap();
            rx.recv_timeout(RECV_TIMEOUT)
                .unwrap_or_else(|_| panic!("expected event #{i} without recreating the watcher"));
        }
    }

    const TEST_WINDOW: Duration = Duration::from_millis(50);

    #[test]
    fn debounce_collapses_rapid_writes_into_one_event() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("foo.lua");
        std::fs::write(&file, "local x = 0").unwrap();

        let (_watcher, rx) = watch_with_window(dir.path(), TEST_WINDOW).unwrap();

        for i in 1..=5 {
            std::fs::write(&file, format!("local x = {i}")).unwrap();
            std::thread::sleep(Duration::from_millis(2));
        }

        let event = rx
            .recv_timeout(RECV_TIMEOUT)
            .expect("expected exactly one event");
        assert_eq!(event.path.file_name(), file.file_name());

        match rx.recv_timeout(Duration::from_millis(100)) {
            Err(RecvTimeoutError::Timeout) => {}
            other => panic!("expected no second event, got {other:?}"),
        }
    }

    #[test]
    fn debounce_treats_writes_separated_by_more_than_window_as_independent_events() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("foo.lua");
        std::fs::write(&file, "local x = 0").unwrap();

        let (_watcher, rx) = watch_with_window(dir.path(), TEST_WINDOW).unwrap();

        std::fs::write(&file, "local x = 1").unwrap();
        rx.recv_timeout(RECV_TIMEOUT).expect("expected first event");

        std::thread::sleep(TEST_WINDOW * 3);

        std::fs::write(&file, "local x = 2").unwrap();
        rx.recv_timeout(RECV_TIMEOUT)
            .expect("expected second, independent event");
    }

    #[test]
    fn debounce_latency_stays_bounded_by_window() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("foo.lua");
        std::fs::write(&file, "local x = 0").unwrap();

        let (_watcher, rx) = watch_with_window(dir.path(), TEST_WINDOW).unwrap();

        let start = Instant::now();
        std::fs::write(&file, "local x = 1").unwrap();
        rx.recv_timeout(RECV_TIMEOUT).expect("expected an event");
        let elapsed = start.elapsed();

        assert!(
            elapsed < TEST_WINDOW + Duration::from_millis(500),
            "debounce latency was {elapsed:?}, expected roughly TEST_WINDOW ({TEST_WINDOW:?} plus FS/OS overhead)"
        )
    }

    #[test]
    fn matches_active_true_when_unfiltered() {
        assert!(matches_active(
            Path::new("/tmp/foo.lua"),
            &ActiveFilter::Unfiltered
        ));
    }

    #[test]
    fn matches_active_false_when_filtered_with_none() {
        assert!(!matches_active(
            Path::new("/tmp/foo.lua"),
            &ActiveFilter::Filtered(None)
        ));
    }

    #[test]
    fn matches_active_true_when_path_equals_active() {
        let active = PathBuf::from("/tmp/foo.lua");
        assert!(matches_active(
            Path::new("/tmp/foo.lua"),
            &ActiveFilter::Filtered(Some(active))
        ));
    }

    #[test]
    fn matches_active_false_when_path_differs_from_active() {
        let active = PathBuf::from("/tmp/bar.lua");
        assert!(!matches_active(
            Path::new("/tmp/foo.lua"),
            &ActiveFilter::Filtered(Some(active))
        ));
    }

    #[test]
    fn active_exercise_receives_its_own_events() {
        let dir = tempfile::tempdir().unwrap();
        let file_a = dir.path().join("a.lua");
        let file_b = dir.path().join("b.lua");
        std::fs::write(&file_a, "local x = 0").unwrap();
        std::fs::write(&file_b, "local x = 0").unwrap();

        let (handle, rx) = watch_with_window(dir.path(), TEST_WINDOW).unwrap();
        handle.set_active(Some(file_a.clone()));

        std::fs::write(&file_a, "local x = 1").unwrap();

        let event = rx
            .recv_timeout(RECV_TIMEOUT)
            .expect("expected the active exercise's event");
        assert_eq!(event.path.file_name(), file_a.file_name());
    }

    #[test]
    fn inactive_exercise_is_silenced() {
        let dir = tempfile::tempdir().unwrap();
        let file_a = dir.path().join("a.lua");
        let file_b = dir.path().join("b.lua");
        std::fs::write(&file_a, "local x = 0").unwrap();
        std::fs::write(&file_b, "local x = 0").unwrap();

        let (handle, rx) = watch_with_window(dir.path(), TEST_WINDOW).unwrap();
        handle.set_active(Some(file_a));

        std::fs::write(&file_b, "local x = 1").unwrap();

        match rx.recv_timeout(RECV_TIMEOUT) {
            Err(RecvTimeoutError::Timeout) => {}
            other => panic!("expected no event for the inactive exercise, got {other:?}"),
        }
    }

    // AC "cambiar cuál es el ejercicio activo actualiza correctamente qué
    // archivo se está observando": prueba tanto que el nuevo activo empieza
    // a dispar eventos como que el anterior deja de hacerlo, no solo que se
    // sumó uno nuevo.
    #[test]
    fn changing_active_exercise_updates_what_is_watched() {
        let dir = tempfile::tempdir().unwrap();
        let file_a = dir.path().join("a.lua");
        let file_b = dir.path().join("b.lua");
        std::fs::write(&file_a, "local x = 0").unwrap();
        std::fs::write(&file_b, "local x = 0").unwrap();

        let (handle, rx) = watch_with_window(dir.path(), TEST_WINDOW).unwrap();
        handle.set_active(Some(file_a.clone()));

        std::fs::write(&file_b, "local x = 1").unwrap();
        match rx.recv_timeout(RECV_TIMEOUT) {
            Err(RecvTimeoutError::Timeout) => {}
            other => panic!("expected b.lua to still be silenced, got {other:?}"),
        }

        handle.set_active(Some(file_b.clone()));

        std::fs::write(&file_b, "local x = 2").unwrap();
        let event = rx
            .recv_timeout(RECV_TIMEOUT)
            .expect("expected b.lua's event now that it's active");
        assert_eq!(event.path.file_name(), file_b.file_name());

        std::fs::write(&file_a, "local x = 3").unwrap();
        match rx.recv_timeout(RECV_TIMEOUT) {
            Err(RecvTimeoutError::Timeout) => {}
            other => panic!("expected a.lua to be silenced after losing focus, got {other:?}"),
        }
    }

    #[test]
    fn watch_without_set_active_still_receives_all_lua_events() {
        let dir = tempfile::tempdir().unwrap();
        let file_a = dir.path().join("a.lua");
        let file_b = dir.path().join("b.lua");
        std::fs::write(&file_a, "local x = 0").unwrap();
        std::fs::write(&file_b, "local x = 0").unwrap();

        let (_handle, rx) = watch_with_window(dir.path(), TEST_WINDOW).unwrap();

        std::fs::write(&file_a, "local x = 1").unwrap();
        rx.recv_timeout(RECV_TIMEOUT)
            .expect("expected a.lua's event");

        std::fs::write(&file_b, "local x = 1").unwrap();
        rx.recv_timeout(RECV_TIMEOUT)
            .expect("expected b.lua's event too");
    }

    #[test]
    fn set_active_with_nonexistent_path_blocks_everything_safely() {
        let dir = tempfile::tempdir().unwrap();
        let file_a = dir.path().join("a.lua");
        std::fs::write(&file_a, "local x = 0").unwrap();

        let (handle, rx) = watch_with_window(dir.path(), TEST_WINDOW).unwrap();
        handle.set_active(Some(dir.path().join("no_exist.lua")));

        std::fs::write(&file_a, "local x = 1").unwrap();

        match rx.recv_timeout(RECV_TIMEOUT) {
            Err(RecvTimeoutError::Timeout) => {}
            other => panic!("expected everything to be blocked, got {other:?}"),
        }
    }
}
