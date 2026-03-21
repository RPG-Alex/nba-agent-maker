use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Maneuver {
    pub description: String,
    pub unlock_rating: u8,
}

impl Maneuver {
    pub fn new(description: &str, unlock_rating: u8) -> Self {
        Self {
            description: description.to_string(),
            unlock_rating,
        }
    }
}
