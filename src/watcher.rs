use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeEvent {
    pub path: PathBuf,
}

fn is_lua_path(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("lua")
}

pub fn watch(dir: &Path) -> notify::Result<(RecommendedWatcher, Receiver<ChangeEvent>)> {
    let (tx, rx) = mpsc::channel::<ChangeEvent>();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        let Ok(event) = res else {
            return;
        };
        for path in event.paths {
            if is_lua_path(&path) {
                let _ = tx.send(ChangeEvent { path });
            }
        }
    })?;

    watcher.watch(dir, RecursiveMode::Recursive)?;

    Ok((watcher, rx))
}

#[cfg(test)]
mod tests {
    use super::{is_lua_path, watch};
    use std::path::Path;
    use std::sync::mpsc::RecvTimeoutError;
    use std::time::Duration;

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
        std::fs::write(&file, "local x = 1");

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
}
