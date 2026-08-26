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

#[cfg(test)]
mod tests {
    use super::{Mode, parse_exercises};

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
}
