use serde::{Deserialize, Serialize};

use crate::rules::general_skills::{ name::SkillName, skill::GeneralSkill};

pub mod cherries;
pub mod maneuvers;
pub mod name;
pub mod skill;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct GeneralSkills {
    pub general_skills: [GeneralSkill; SkillName::ALL.len()],
    pub mos: SkillName,
}

impl GeneralSkills {
    pub fn new() -> Self {
        Self {
            general_skills: SkillName::ALL.map(GeneralSkill::new),
            mos: SkillName::Athletics,
        }
    }

    pub fn get(&self, skill: SkillName) -> &GeneralSkill {
        &self.general_skills[skill.index()]
    }

    pub fn get_mut(&mut self, skill: SkillName) -> &mut GeneralSkill {
        &mut self.general_skills[skill.index()]
    }

    pub fn iter(&self) -> impl Iterator<Item = &GeneralSkill> {
        self.general_skills.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut GeneralSkill> {
        self.general_skills.iter_mut()
    }

    pub fn mos(&self) -> SkillName {
        self.mos
    }

    pub fn set_mos(&mut self, skill: SkillName) -> SkillName {
        self.mos = skill;
        self.mos
    }
}

impl Default for GeneralSkills {
    fn default() -> Self {
        Self::new()
    }
}