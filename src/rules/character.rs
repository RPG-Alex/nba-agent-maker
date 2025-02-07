use crate::rules::{general_skills::*, investigative_skills::*};
use reactive_stores::Store;

#[derive(Clone, PartialEq)]
pub struct MOS;

impl MOS {
    pub fn get_mos(general_skills: &GeneralSkills) -> String {
        [
            &general_skills.athletics,
            &general_skills.conceal,
            &general_skills.cover,
            &general_skills.digital_intrusion,
            &general_skills.disguise,
            &general_skills.driving,
            &general_skills.explosive_devices,
            &general_skills.filch,
            &general_skills.gambling,
            &general_skills.hand_to_hand,
            &general_skills.infiltration,
            &general_skills.mechanics,
            &general_skills.medic,
            &general_skills.network,
            &general_skills.piloting,
            &general_skills.preparedness,
            &general_skills.sense_trouble,
            &general_skills.shooting,
            &general_skills.shrink,
            &general_skills.surveillance,
            &general_skills.weapons,
        ]
        .iter()
        .find_map(|skill| {
            if skill.is_mos {
                Some(skill.skill.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "None".to_string())
    }
}

#[derive(Clone, PartialEq, Store)]
pub struct Character {
    // Agent Info (Personality and Dossier)
    pub agent_name: String,
    pub mos: String,
    pub drive: String,
    pub handler: String,
    pub professional_role: String,
    pub backgrounds: Vec<String>,
    pub symbol: String,
    pub solace: String,
    pub safety: String,
    pub health: i32,
    pub stability: i32,
    pub heat_level: i32,
    pub general_points: i32,
    pub investigative_points: i32,
    pub game_modes: Vec<String>,

    // Abilities
    pub general_skills: GeneralSkills,
    pub investigative_abilities: InvestigativeAbilities,
}

impl Character {
    pub fn get_mos(&self) -> String {
        MOS::get_mos(&self.general_skills)
    }
}

impl Default for Character {
    fn default() -> Self {
        let general_skills = GeneralSkills::new();
        let mos = MOS::get_mos(&general_skills);
        Self {
            agent_name: String::new(),
            mos,
            drive: String::new(),
            handler: String::new(),
            professional_role: String::new(),
            backgrounds: Vec::new(),
            symbol: String::new(),
            solace: String::new(),
            safety: String::new(),
            health: 4,
            stability: 4,
            heat_level: 0,
            general_points: 70,
            investigative_points: 0,
            game_modes: Vec::new(),

            // General & Investigative Abilities
            general_skills,
            investigative_abilities: InvestigativeAbilities::new(),
        }
    }
}
