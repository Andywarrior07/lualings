use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    #[serde(default)]
    pub completed: BTreeMap<String, bool>,
}

#[cfg(test)]
mod test {
    use super::Progress;
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
}
