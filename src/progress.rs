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

#[derive(Debug)]
pub enum SaveError {
    Io(std::io::Error),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for SaveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SaveError::Io(err) => Some(err),
        }
    }
}

pub fn save(path: &std::path::Path, progress: &Progress) -> Result<(), SaveError> {
    if let Some(dir) = path.parent()
        && !dir.as_os_str().is_empty()
    {
        std::fs::create_dir_all(dir).map_err(SaveError::Io)?;
    }

    let json = serde_json::to_string_pretty(progress)
        .expect("Progress solo contiene String/bool: no puede fallar al serializar");
    let mut tmp_name = path
        .file_name()
        .expect("Path debe tener nombre de archivo")
        .to_os_string();
    tmp_name.push(".tmp");
    let tmp_path = path.with_file_name(tmp_name);

    std::fs::write(&tmp_path, json).map_err(SaveError::Io)?;
    std::fs::rename(&tmp_path, path).map_err(SaveError::Io)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{LoadError, Progress, load, save};
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

    #[test]
    fn save_creates_missing_directory() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("progress.json");

        save(&path, &Progress::default()).unwrap();

        assert!(path.parent().unwrap().is_dir());
        assert!(path.is_file());
    }

    #[test]
    fn save_then_load_roundtrips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("progress.json");
        let mut completed = BTreeMap::new();
        completed.insert("exercises/01_junior/01_variables/a.lua".to_string(), true);
        completed.insert("exercises/01_junior/01_variables/b.lua".to_string(), false);
        let progress = Progress { completed };
        save(&path, &progress).unwrap();

        assert_eq!(load(&path).unwrap(), progress);
    }

    #[test]
    fn save_twice_does_not_corrupt() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("progress.json");

        let mut completed_a = BTreeMap::new();
        completed_a.insert("exercises/01_junior/01_variables/a.lua".to_string(), true);
        save(
            &path,
            &Progress {
                completed: completed_a,
            },
        )
        .unwrap();

        let mut completed_b = BTreeMap::new();
        completed_b.insert("exercises/01_junior/02_types/b.lua".to_string(), true);
        let progress_b = Progress {
            completed: completed_b,
        };
        save(&path, &progress_b).unwrap();

        assert_eq!(load(&path).unwrap(), progress_b);
    }

    #[test]
    fn save_does_not_leave_temp_file_behind() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("progress.json");

        save(&path, &Progress::default()).unwrap();

        assert!(!dir.path().join("progress.json.tmp").exists());
    }
}
