use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Compile,
    Test,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub path: String,
    pub mode: Mode,
    pub hint: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub module: String,
}

impl Exercise {
    pub fn find_by_name<'a>(exercises: &'a [Exercise], name: &str) -> Option<&'a Exercise> {
        exercises.iter().find(|exercise| exercise.name == name)
    }

    pub fn read_source(&self) -> std::io::Result<String> {
        std::fs::read_to_string(&self.path)
    }
}

#[derive(Deserialize)]
struct Info {
    levels: Vec<Level>,
}

#[derive(Deserialize)]
struct Level {
    name: String,
    modules: Vec<Module>,
}

#[derive(Deserialize)]
struct Module {
    name: String,
    exercises: Vec<Exercise>,
}

pub fn parse_exercises(json: &str) -> Result<Vec<Exercise>, serde_json::Error> {
    let info: Info = serde_json::from_str(json)?;
    let mut exercises = Vec::new();
    for level in &info.levels {
        for module in &level.modules {
            for exercise in &module.exercises {
                let mut exercise = exercise.clone();
                exercise.level = level.name.clone();
                exercise.module = module.name.clone();
                exercises.push(exercise);
            }
        }
    }
    Ok(exercises)
}

#[derive(Debug, Clone, PartialEq)]
pub struct MissingPaths(pub Vec<String>);

impl std::fmt::Display for MissingPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "info.json declares paths that do not exist on disk:")?;
        for path in &self.0 {
            writeln!(f, " - {path}")?;
        }
        Ok(())
    }
}

pub fn validate_paths(exercises: &[Exercise]) -> Result<(), MissingPaths> {
    let missing: Vec<String> = exercises
        .iter()
        .filter(|exercise| !std::path::Path::new(&exercise.path).is_file())
        .map(|exercise| exercise.path.clone())
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(MissingPaths(missing))
    }
}

pub const DEFAULT_INFO_PATH: &str = "info.json";

#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Corrupt(serde_json::Error),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(err) => write!(f, "{err}"),
            LoadError::Corrupt(err) => write!(f, "info.json is corrupt: {err}"),
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

pub fn load(path: &std::path::Path) -> Result<Vec<Exercise>, LoadError> {
    let contents = std::fs::read_to_string(path).map_err(LoadError::Io)?;
    parse_exercises(&contents).map_err(LoadError::Corrupt)
}

#[cfg(test)]
mod tests {
    use super::{Exercise, LoadError, Mode, load, parse_exercises, validate_paths};

    #[test]
    fn parse_exercises_preserves_file_order() {
        let json = r#"
        {
            "levels": [
                {
                    "name": "01_junior",
                    "modules": [
                        {
                            "name": "01_variables",
                            "exercises": [
                                {
                                    "name": "zzz_first",
                                    "path": "exercises/01_junior/01_variables/zzz_first.lua",
                                    "mode": "compile",
                                    "hint": "primero"
                                },
                                {
                                    "name": "aaa_second",
                                    "path": "exercises/01_junior/01_variables/aaa_second.lua",
                                    "mode": "compile",
                                    "hint": "segundo"
                                }
                            ]
                        },
                        {
                            "name": "02_types",
                            "exercises": [
                                {
                                    "name": "aab_third",
                                    "path": "exercises/01_junior/02_types/aab_third.lua",
                                    "mode": "test",
                                    "hint": "tercero"
                                }
                            ]
                        }
                    ]
                }
            ]
        }
        "#;

        let exercises = parse_exercises(json).unwrap();
        let names: Vec<&str> = exercises.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["zzz_first", "aaa_second", "aab_third"]);
    }

    #[test]
    fn parse_exercises_handles_multiple_modules() {
        let json = r#"
        {
            "levels": [
                {
                    "name": "01_junior",
                    "modules": [
                        {
                            "name": "01_variables",
                            "exercises": [
                                {
                                    "name": "variables1",
                                    "path": "exercises/01_junior/01_variables/variables1.lua",
                                    "mode": "compile",
                                    "hint": "usa local"
                                }
                            ]
                        },
                        {
                            "name": "02_types",
                            "exercises": [
                                {
                                    "name": "types1",
                                    "path": "exercises/01_junior/02_types/types1.lua",
                                    "mode": "compile",
                                    "hint": "usa type()"
                                },
                                {
                                    "name": "types2",
                                    "path": "exercises/01_junior/02_types/types2.lua",
                                    "mode": "test",
                                    "hint": "asserts sobre tipos"
                                }
                            ]
                        }
                    ]
                }
            ]
        }
        "#;

        let exercises = parse_exercises(json).unwrap();
        assert_eq!(exercises.len(), 3);

        assert_eq!(exercises[0].name, "variables1");
        assert_eq!(exercises[0].level, "01_junior");
        assert_eq!(exercises[0].module, "01_variables");
        assert_eq!(
            exercises[0].path,
            "exercises/01_junior/01_variables/variables1.lua"
        );

        assert_eq!(exercises[1].name, "types1");
        assert_eq!(exercises[1].level, "01_junior");
        assert_eq!(exercises[1].module, "02_types");

        assert_eq!(exercises[2].name, "types2");
        assert_eq!(exercises[2].module, "02_types");
    }

    #[test]
    fn parse_exercises_deserializes_both_modes() {
        let json = r#"
        {
            "levels": [
                {
                    "name": "01_junior",
                    "modules": [
                        {
                            "name": "01_variables",
                            "exercises": [
                                {
                                    "name": "compiles",
                                    "path": "exercises/01_junior/01_variables/compiles.lua",
                                    "mode": "compile",
                                    "hint": "hint"
                                },
                                {
                                    "name": "tests",
                                    "path": "exercises/01_junior/01_variables/tests.lua",
                                    "mode": "test",
                                    "hint": "hint"
                                }
                            ]
                        }
                    ]
                }
            ]
        }
        "#;

        let exercises = parse_exercises(json).unwrap();
        assert_eq!(exercises[0].mode, Mode::Compile);
        assert_eq!(exercises[1].mode, Mode::Test);
    }

    #[test]
    fn parse_exercises_fails_on_unknown_mode() {
        let json = r#"
        {
            "levels": [
                {
                    "name": "01_junior",
                    "modules": [
                        {
                            "name": "01_variables",
                            "exercises": [
                                {
                                    "name": "broken",
                                    "path": "exercises/01_junior/01_variables/broken.lua",
                                    "mode": "unknown",
                                    "hint": "hint"
                                }
                            ]
                        }
                    ]
                }
            ]
        }
        "#;

        assert!(parse_exercises(json).is_err());
    }

    fn exercise_with_path(path: &str) -> Exercise {
        Exercise {
            name: "placeholder".to_string(),
            path: path.to_string(),
            mode: Mode::Compile,
            hint: "hint".to_string(),
            level: "01_junior".to_string(),
            module: "01_variables".to_string(),
        }
    }

    #[test]
    fn validate_paths_passes_when_all_exist() {
        let exercises = vec![
            exercise_with_path("Cargo.toml"),
            exercise_with_path("src/lib.rs"),
        ];
        assert_eq!(validate_paths(&exercises), Ok(()));
    }

    #[test]
    fn validate_paths_detects_missing_paths() {
        let exercises = vec![
            exercise_with_path("Cargo.toml"),
            exercise_with_path("exercises/01_junior/01_variables/no_exist.lua"),
        ];

        let err = validate_paths(&exercises).unwrap_err();
        assert_eq!(
            err.0,
            vec!["exercises/01_junior/01_variables/no_exist.lua".to_string()]
        )
    }

    #[test]
    fn validate_paths_error_message_is_readable() {
        let exercises = vec![exercise_with_path(
            "exercises/01_junior/01_variables/no_exist.lua",
        )];

        let err = validate_paths(&exercises).unwrap_err();
        let message = format!("{err}");
        assert!(message.contains("exercises/01_junior/01_variables/no_exist.lua"));
    }

    #[test]
    fn parse_exercises_malformed_json_reports_location() {
        let err = parse_exercises(r#"{ "levels": ["#).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("line"));
        assert!(message.contains("column"));
    }

    #[test]
    fn parse_exercises_missing_field_names_it_and_reports_location() {
        let json = r#"
        {
            "levels": [
                {
                    "name": "01_junior",
                    "modules": [
                        {
                            "name": "01_variables",
                            "exercises": [
                                { "name": "x", "mode": "compile", "hint": "h" }
                            ]
                        }
                    ]
                }
            ]
        }
        "#;
        let err = parse_exercises(json).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("missing field `path`"));
        assert!(message.contains("line"));
        assert!(message.contains("column"));
    }

    #[test]
    fn parse_exercises_wrong_type_reports_location() {
        let json = r#"
        {
            "levels": [
                {
                    "name": "01_junior",
                    "modules": [
                        {
                            "name": "01_variables",
                            "exercises": [
                                { "name": "x", "path": 123, "mode": "compile", "hint": "h" }
                            ]
                        }
                    ]
                }
            ]
        }
        "#;
        let err = parse_exercises(json).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("invalid type"));
        assert!(message.contains("line"));
        assert!(message.contains("column"));
    }

    #[test]
    fn load_reads_valid_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("info.json");
        std::fs::write(
            &path,
            r#"
            {
                "levels": [
                    {
                        "name": "01_junior",
                        "modules": [
                            {
                                "name": "01_variables",
                                "exercises": [
                                    {
                                        "name": "variables1",
                                        "path": "exercises/01_junior/01_variables/variables1.lua",
                                        "mode": "compile",
                                        "hint": "usa local"
                                    }
                                ]
                            }
                        ]
                    }
                ]
            }
            "#,
        )
        .unwrap();

        let exercises = load(&path).unwrap();
        assert_eq!(exercises.len(), 1);
        assert_eq!(exercises[0].name, "variables1");
    }

    #[test]
    fn load_reports_missing_file_as_error_not_empty() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("no_exist.json");

        match load(&path) {
            Err(LoadError::Io(_)) => {}
            other => panic!("expected LoadError::Io, got {other:?}"),
        }
    }

    #[test]
    fn load_reports_corrupt_json_as_error_not_panic() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("info.json");
        std::fs::write(&path, "not json").unwrap();

        match load(&path) {
            Err(LoadError::Corrupt(_)) => {}
            other => panic!("expected LoadError::Corrupt, got {other:?}"),
        }
    }

    #[test]
    fn find_by_name_returns_the_matching_exercise() {
        let mut a = exercise_with_path("a.lua");
        a.name = "variables1".to_string();
        let mut b = exercise_with_path("b.lua");
        b.name = "variables2".to_string();
        let exercises = vec![a, b];

        let found = Exercise::find_by_name(&exercises, "variables2").unwrap();
        assert_eq!(found.path, "b.lua");
    }

    #[test]
    fn find_by_name_returns_none_when_absent() {
        let mut a = exercise_with_path("a.lua");
        a.name = "variables1".to_string();
        let exercises = vec![a];

        assert!(Exercise::find_by_name(&exercises, "no_existe").is_none());
    }

    #[test]
    fn find_by_name_does_not_partial_match() {
        let mut a = exercise_with_path("a.lua");
        a.name = "variables1".to_string();
        let exercises = vec![a];

        assert!(Exercise::find_by_name(&exercises, "variables").is_none());
    }

    #[test]
    fn read_source_reads_the_exercises_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("exercise.lua");
        std::fs::write(&path, "local x = 1").unwrap();

        let exercise = exercise_with_path(path.to_str().unwrap());
        assert_eq!(exercise.read_source().unwrap(), "local x = 1");
    }

    #[test]
    fn read_source_fails_when_file_is_missing() {
        let exercise = exercise_with_path("no_exist.lua");
        assert!(exercise.read_source().is_err());
    }
}
