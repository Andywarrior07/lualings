use crate::exercise;
use include_dir::{Dir, DirEntry, include_dir};
use std::path::Path;

const PROJECTS_DIR_NAME: &str = "projects";

static EXERCISES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/exercises");
static SOLUTIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/solutions");
static PROJECTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/projects");
static INFO_JSON: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/info.json"));

fn extract_dir_skip_existing(dir: &Dir<'_>, target: &Path) -> std::io::Result<()> {
    for entry in dir.entries() {
        let path = target.join(entry.path());
        match entry {
            DirEntry::Dir(d) => {
                std::fs::create_dir_all(&path)?;
                extract_dir_skip_existing(d, target)?;
            }
            DirEntry::File(f) => {
                if !path.exists() {
                    std::fs::write(&path, f.contents())?;
                }
            }
        }
    }
    Ok(())
}

pub fn extract_to(target: &Path) -> std::io::Result<()> {
    extract_dir_skip_existing(
        &EXERCISES_DIR,
        &target.join(exercise::DEFAULT_EXERCISES_DIR),
    )?;
    extract_dir_skip_existing(
        &SOLUTIONS_DIR,
        &target.join(exercise::DEFAULT_SOLUTIONS_DIR),
    )?;
    extract_dir_skip_existing(&PROJECTS_DIR, &target.join(PROJECTS_DIR_NAME))?;

    let info_path = target.join("info.json");
    if !info_path.exists() {
        std::fs::write(info_path, INFO_JSON)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::extract_to;

    #[test]
    fn extract_to_populates_an_empty_directory() {
        let dir = tempfile::tempdir().unwrap();
        extract_to(dir.path()).unwrap();

        assert!(dir.path().join("info.json").is_file());
        assert!(
            dir.path()
                .join("exercises/01_junior/01_variables/variables1.lua")
                .is_file()
        );
        assert!(
            dir.path()
                .join("solutions/01_junior/01_variables/variables1.lua")
                .is_file()
        );
        assert!(
            dir.path()
                .join("projects/junior_text_adventure/README.md")
                .is_file()
        );
    }

    #[test]
    fn extract_to_never_overwrites_an_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir
            .path()
            .join("exercises/01_junior/01_variables/variables1.lua");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, "-- user's in-progress edit").unwrap();

        extract_to(dir.path()).unwrap();

        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "-- user's in-progress edit"
        );
    }

    #[test]
    fn extract_to_never_overwrites_an_existing_info_json() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("info.json"), "{\"levels\":[]}").unwrap();

        extract_to(dir.path()).unwrap();

        assert_eq!(
            std::fs::read_to_string(dir.path().join("info.json")).unwrap(),
            "{\"levels\":[]}"
        );
    }

    #[test]
    fn extract_to_twice_does_not_fail_or_change_anything() {
        let dir = tempfile::tempdir().unwrap();
        extract_to(dir.path()).unwrap();
        let first = std::fs::read_to_string(dir.path().join("info.json")).unwrap();

        extract_to(dir.path()).unwrap();
        let second = std::fs::read_to_string(dir.path().join("info.json")).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn extract_to_leaves_an_existing_progress_json_untouched() {
        let dir = tempfile::tempdir().unwrap();
        let cache_dir = dir.path().join(".lualings-cache");
        std::fs::create_dir_all(&cache_dir).unwrap();
        std::fs::write(
            cache_dir.join("progress.json"),
            "{\"completed\":{\"x\":true}}",
        )
        .unwrap();

        extract_to(dir.path()).unwrap();

        assert_eq!(
            std::fs::read_to_string(cache_dir.join("progress.json")).unwrap(),
            "{\"completed\":{\"x\":true}}"
        )
    }
}
