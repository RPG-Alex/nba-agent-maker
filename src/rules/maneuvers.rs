use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Maneuver {
    pub description: String,
    pub unlocked: bool,
}

impl Maneuver {
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
            unlocked: false,
        }
    }
}
