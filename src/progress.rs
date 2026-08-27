use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    #[serde(default)]
    pub completed: BTreeMap<String, bool>,
}

pub const DEFAULT_PROGRESS_PATH: &str = ".lualings-cache/progress.json";

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Corrupt(serde_json::Error),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(err) => write!(f, "{err}"),
            LoadError::Corrupt(err) => write!(f, "progress.json is corrupt: {err}"),
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::Io(err) => Some(err),
            LoadError::Corrupt(err) => Some(err),
        }
    }
}

pub fn load(path: &std::path::Path) -> Result<Progress, LoadError> {
    match std::fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str(&contents).map_err(LoadError::Corrupt),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Progress::default()),
        Err(err) => Err(LoadError::Io(err)),
    }
}

#[cfg(test)]
mod test {
    use super::{LoadError, Progress, load};
    use std::collections::BTreeMap;

    #[test]
    fn serializes_and_deserializes_without_custom_logic() {
        let mut completed = BTreeMap::new();
        completed.insert("exercises/01_junior/01_variables/a.lua".to_string(), true);
        completed.insert("exercises/01_junior/01_variables/b.lua".to_string(), false);
        let progress = Progress { completed };

        let json = serde_json::to_string(&progress).unwrap();
        let roundtripped: Progress = serde_json::from_str(&json).unwrap();

        assert_eq!(roundtripped, progress);
    }

    #[test]
    fn empty_json_object_deserializes_to_empty_map() {
        let progress: Progress = serde_json::from_str("{}").unwrap();
        assert!(progress.completed.is_empty());
    }

    #[test]
    fn absent_path_is_distinguishable_from_completed_false() {
        let mut completed = BTreeMap::new();
        completed.insert("exercises/01_junior/01_variables/a.lua".to_string(), false);
        let progress = Progress { completed };

        assert_eq!(
            progress
                .completed
                .get("exercises/01_junior/01_variables/a.lua"),
            Some(&false)
        );
        assert_eq!(
            progress
                .completed
                .get("exercises/01_junior/01_variables/no_existe.lua"),
            None
        )
    }

    #[test]
    fn load_reads_valid_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("progress.json");
        let mut completed = BTreeMap::new();
        completed.insert("exercises/01_junior/01_variables/a.lua".to_string(), true);
        let progress = Progress { completed };
        std::fs::write(&path, serde_json::to_string(&progress).unwrap()).unwrap();

        assert_eq!(load(&path).unwrap(), progress);
    }

    #[test]
    fn load_returns_empty_progress_when_file_missing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("no_existe.json");

        assert_eq!(load(&path).unwrap(), Progress::default());
    }

    #[test]
    fn load_reports_corrupt_json_as_error_not_panic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("progress.json");
        std::fs::write(&path, "not json").unwrap();

        match load(&path) {
            Err(LoadError::Corrupt(_)) => {}
            other => panic!("expected LoadError::Corrupt, got {other:?}"),
        }
    }

    #[test]
    fn load_reports_non_not_found_io_errors_as_io() {
        let dir = tempfile::tempdir().unwrap();

        match load(dir.path()) {
            Err(LoadError::Io(_)) => {}
            other => panic!("expected LoadError::Io, got {other:?}"),
        }
    }
}
