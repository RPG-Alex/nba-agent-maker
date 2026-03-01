use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Cherry {
    pub description: String,
    pub chosen: bool,
}

impl Cherry {
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
            chosen: false,
        }
    }
}
