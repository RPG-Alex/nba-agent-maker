
use crate::rules::{general_skills::*, investigative_skills::*, modes::GameMode};
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
    pub mos: GeneralSkill,
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
    pub general_points: u16,
    pub investigative_points: u16,
    pub game_modes: Vec<GameMode>,

    // Abilities
    pub general_skills: GeneralSkills,
    pub investigative_abilities: InvestigativeAbilities,
}

impl Character {
    pub fn get_mos(&self) -> &GeneralSkill {
        todo!()
    }
}

impl Default for Character {
    fn default() -> Self {
        Self { agent_name: todo!(), mos: todo!(), drive: todo!(), handler: todo!(), professional_role: todo!(), backgrounds: todo!(), symbol: todo!(), solace: todo!(), safety: todo!(), health: todo!(), stability: todo!(), heat_level: todo!(), general_points: todo!(), investigative_points: todo!(), game_modes: todo!(), general_skills: todo!(), investigative_abilities: todo!() }
    }
}