use crate::exercise;
use include_dir::{Dir, DirEntry, include_dir};
use std::path::Path;

const PROJECTS_DIR_NAME: &str = "projects";

static EXERCISES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/exercises");
static SOLUTIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/solutions");
static PROJECTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/projects");
static INFO_JSON: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/info.json"));

fn with_path_context(path: &Path, err: std::io::Error) -> std::io::Error {
    std::io::Error::new(err.kind(), format!("{}: {err}", path.display()))
}

fn extract_dir_skip_existing(dir: &Dir<'_>, target: &Path) -> std::io::Result<()> {
    for entry in dir.entries() {
        let path = target.join(entry.path());
        match entry {
            DirEntry::Dir(d) => {
                std::fs::create_dir_all(&path).map_err(|err| with_path_context(&path, err))?;
                extract_dir_skip_existing(d, target)?;
            }
            DirEntry::File(f) => {
                if !path.exists() {
                    std::fs::write(&path, f.contents())
                        .map_err(|err| with_path_context(&path, err))?;
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
        std::fs::write(&info_path, INFO_JSON).map_err(|err| with_path_context(&info_path, err))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        EXERCISES_DIR, INFO_JSON, PROJECTS_DIR, PROJECTS_DIR_NAME, SOLUTIONS_DIR, extract_to,
    };
    use crate::exercise;
    use include_dir::{Dir, DirEntry};
    use std::path::{Path, PathBuf};

    fn collect_file_names(dir: &Dir<'_>, names: &mut Vec<String>) {
        for entry in dir.entries() {
            match entry {
                DirEntry::Dir(d) => collect_file_names(d, names),
                DirEntry::File(f) => {
                    if let Some(name) = f.path().file_name() {
                        names.push(name.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    fn collect_relative_paths(dir: &Dir<'_>, out: &mut Vec<PathBuf>) {
        for entry in dir.entries() {
            match entry {
                DirEntry::Dir(d) => collect_relative_paths(d, out),
                DirEntry::File(f) => out.push(f.path().to_path_buf()),
            }
        }
    }

    fn collect_disk_files(root: &Path, base: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(root).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                collect_disk_files(&path, base, out);
            } else {
                out.push(path.strip_prefix(base).unwrap().to_path_buf());
            }
        }
    }

    fn assert_tree_matches_embedded(embedded: &Dir<'_>, extracted_root: &Path) {
        let mut embedded_paths = Vec::new();
        collect_relative_paths(embedded, &mut embedded_paths);
        embedded_paths.sort();

        let mut disk_paths = Vec::new();
        collect_disk_files(extracted_root, extracted_root, &mut disk_paths);
        disk_paths.sort();

        assert_eq!(
            embedded_paths, disk_paths,
            "extracted files under {extracted_root:?} don't match the embedded tree"
        );

        for relative_path in &embedded_paths {
            let embedded_contents = embedded.get_file(relative_path).unwrap().contents();
            let disk_contents = std::fs::read(extracted_root.join(relative_path)).unwrap();
            assert_eq!(
                embedded_contents, disk_contents,
                "content mismatch for {relative_path:?}"
            );
        }
    }

    #[test]
    fn extract_to_matches_embedded_structure_exactly() {
        let dir = tempfile::tempdir().unwrap();
        extract_to(dir.path()).unwrap();

        assert_tree_matches_embedded(
            &EXERCISES_DIR,
            &dir.path().join(exercise::DEFAULT_EXERCISES_DIR),
        );
        assert_tree_matches_embedded(
            &SOLUTIONS_DIR,
            &dir.path().join(exercise::DEFAULT_SOLUTIONS_DIR),
        );
        assert_tree_matches_embedded(&PROJECTS_DIR, &dir.path().join(PROJECTS_DIR_NAME));

        assert_eq!(
            std::fs::read_to_string(dir.path().join("info.json")).unwrap(),
            INFO_JSON
        );
    }

    #[test]
    fn embedded_exercises_excludes_test_fixtures() {
        let mut names = Vec::new();
        collect_file_names(&EXERCISES_DIR, &mut names);

        for fixture_only_name in ["passes.lua", "fails.lua", "infinite_loop.lua"] {
            assert!(
                !names.iter().any(|name| name == fixture_only_name),
                "'{fixture_only_name}' only exists under test/fixtures/exercises/ and \
                should never be embedded via EXERCISES_DIR"
            );
        }
    }

    #[test]
    fn extract_to_does_not_create_the_cache_directory() {
        let dir = tempfile::tempdir().unwrap();

        extract_to(dir.path()).unwrap();

        assert!(!dir.path().join(".lualings-cache").exists());
    }

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

    #[cfg(unix)]
    #[test]
    fn extract_to_reports_the_failing_path_on_write_error() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o555)).unwrap();

        let err = extract_to(dir.path()).unwrap_err();

        let expected_path = dir.path().join(exercise::DEFAULT_EXERCISES_DIR);
        assert!(
            err.to_string()
                .contains(&expected_path.display().to_string()),
            "expected the error to mention '{expected_path:?}', got: {err}"
        );

        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o755)).unwrap();
    }
}
