use crate::rules::{
    general_skills::{name::SkillName, skill::GeneralSkill, *},
    investigative_skills::*,
    modes::GameMode,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq)]
pub(crate) struct MOS;

impl MOS {
    pub fn get_mos(general_skills: &GeneralSkills) -> String {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct Character {
    // Agent Info (Personality and Dossier)
    pub(crate) agent_name: String,
    pub(crate) drive: String,
    pub(crate) handler: String,
    pub(crate) professional_role: String,
    pub(crate) backgrounds: Vec<String>,
    pub(crate) symbol: String,
    pub(crate) solace: String,
    pub(crate) safety: String,
    pub(crate) health: i8,
    pub(crate) stability: i8,
    pub(crate) heat_level: u8,
    pub(crate) general_points_total: u16,
    pub(crate) general_points_spent: u16,
    pub(crate) investigative_points_total: u16,
    pub(crate) investigative_points_spent: u16,
    pub(crate) game_modes: Vec<GameMode>,

    // Abilities
    pub(crate) general_skills: GeneralSkills,
    pub(crate) investigative_abilities: InvestigativeAbilities,

    pub(crate) editable: bool,
}

impl Character {
    pub(crate) fn investigative_points_available(&self) -> u16 {
        self.investigative_points_total - self.investigative_points_spent
    }

    pub(crate) fn general_points_available(&self) -> u16 {
        self.general_points_total - self.general_points_spent
    }

    pub(crate) fn spend_investigate_points(&mut self, spent: u16) -> Result<(), ()> {
        if spent > self.investigative_points_available() {
            Err(())
        } else {
            self.investigative_points_spent += spent;
            Ok(())
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            agent_name: String::new(),
            drive: String::new(),
            handler: String::new(),
            professional_role: String::new(),
            backgrounds: Vec::new(),
            symbol: String::new(),
            solace: String::new(),
            safety: String::new(),
            health: 4,
            stability: 4,
            heat_level: 1,
            general_points_total: 70,
            general_points_spent: 0,
            investigative_points_total: 20, // default to a full table of 5 players
            investigative_points_spent: 0,
            game_modes: Vec::new(),
            general_skills: GeneralSkills::new(),
            investigative_abilities: InvestigativeAbilities::new(),
            editable: false,
        }
    }
}
