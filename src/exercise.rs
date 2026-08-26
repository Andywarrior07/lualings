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

#[cfg(test)]
mod tests {
    use super::{Exercise, Mode, parse_exercises, validate_paths};

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
}
