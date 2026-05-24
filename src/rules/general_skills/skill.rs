use serde::{Deserialize, Serialize};

use crate::rules::general_skills::{
    cherries::CherryId,
    name::SkillName,
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct GeneralSkill {
    pub skill: SkillName,
    pub rating: i32,
    pub chosen_cherries: Vec<CherryId>,
}

impl GeneralSkill {
    pub fn new(name: SkillName) -> Self {
        Self {
            skill: name,
            rating: 0,
            chosen_cherries: Vec::new(),
        }
    }
}
