use crate::rules::{
    general_skills::{name::SkillName, skill::GeneralSkill, *},
    investigative_skills::*,
    modes::GameMode,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq)]
pub struct MOS;

impl MOS {
    pub fn get_mos(general_skills: &GeneralSkills) -> String {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Character {
    // Agent Info (Personality and Dossier)
    pub agent_name: String,
    pub drive: String,
    pub handler: String,
    pub professional_role: String,
    pub backgrounds: Vec<String>,
    pub symbol: String,
    pub solace: String,
    pub safety: String,
    pub health: i8,
    pub stability: i8,
    pub heat_level: u8,
    pub general_points_total: u16,
    pub general_points_spent: u16,
    pub investigative_points_total: u16,
    pub investigative_points_spent: u16,
    pub game_modes: Vec<GameMode>,

    // Abilities
    pub general_skills: GeneralSkills,
    pub investigative_abilities: InvestigativeAbilities,
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
        }
    }
}
