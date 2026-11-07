use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::NbaError;

use super::{general_skills::name::GeneralSkillName, investigative_skills::InvestigativeAbilityName};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct Background {
    pub(crate) name: BackGroundName,
    pub(crate) description: String,
    pub(crate) examples: Vec<String>,
    pub(crate) investigative_point_buys: Vec<InvestigativePointBuy>,
    pub(crate) investigative_alternatives: Vec<InvestigativeAbilityName>,
    pub(crate) general_point_buys: Vec<GeneralPointBuy>,
    pub(crate) general_alternatives: Vec<GeneralSkillName>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct InvestigativePointBuy {
    pub(crate) skill: InvestigativeAbilityName,
    pub(crate) points: u8,
    pub(crate) alternative_skill_choice: Option<InvestigativeAbilityName>,
}
impl InvestigativePointBuy {
    pub(crate) fn new(skill: InvestigativeAbilityName, points: u8, alternative_skill_choice: Option<InvestigativeAbilityName>) -> Self {
        Self { skill, points, alternative_skill_choice }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct GeneralPointBuy {
    pub(crate) skill: GeneralSkillName,
    pub(crate) points: u8,
    pub(crate) alternative_skill_choice: Option<GeneralSkillName>,
}

impl GeneralPointBuy {
    pub(crate) fn new(skill: GeneralSkillName, points: u8, alternative_skill_choice: Option<GeneralSkillName>) -> Self {
        Self { skill, points, alternative_skill_choice }
    }
}

impl Background {
    pub(crate) fn analyst() -> Self {
        Self {
            name: BackGroundName::Analyst,
            description: String::from("Almost all modern spy work involves sifting through the immense amount of data available to come to some kind of useful conclusion. Much of this information isn’t classified or stolen, or even picked up by spy satellites: newspapers, periodicals, and the Internet provide oceans of raw open-source intelligence (OSINT) that a good analyst can turn into useful product forh is government. An analyst might also work for a university, newspaper, or strategic-studies think tank, either under cover or simply providing government- quality intelligence to the private sector. \nAnalysts have good minds for pattern matching; this might manifest in mathematical ability, musical creativity, or just low-level paranoia."),
            examples: vec![
                String::from("MI5 threat analyst"),
                String::from("Deutsche Bank econometrician"),
                String::from("European Parliament staffer"),
                String::from("deputy CIA station chief"),
                String::from("journalist for Dagens Nyheter"),
                String::from("French Air Force operational analyst"),
            ],
            investigative_point_buys: vec![
                InvestigativePointBuy::new(InvestigativeAbilityName::DataRecovery, 1),
                InvestigativePointBuy::new(InvestigativeAbilityName::Languages, 2),
                InvestigativePointBuy::new(InvestigativeAbilityName::Research, 1),
                InvestigativePointBuy::new(InvestigativeAbilityName::TrafficAnalysis, 2),
            ],
            general_point_buys: vec![
                GeneralPointBuy::new(GeneralSkillName::DigitalIntrusion, 2),
                GeneralPointBuy::new(GeneralSkillName::Network, 5),
                GeneralPointBuy::new(GeneralSkillName::Preparedness, 4),
                GeneralPointBuy::new(GeneralSkillName::SenseTrouble, 5),
                GeneralPointBuy::new(GeneralSkillName::Surveillance, 3),
            ]
        }
    }
    pub(crate) fn asset_handler() -> Self {
        Self { name: BackGroundName::AssetHandler , description: String::from("Intelligence agencies get human intelligence (HUMINT) from all manner of sources, willing and coerced, knowing and duped. These sources are called “assets,” and the agent who keeps them all supplying intel is their “runner” or “handler.”Asset handlers have to juggle carrots and sticks, avoid getting involved with their assets’ lives while solving their problems, and always keep an eye out for more potential recruits. Since they actually have to meet their assets on occasion, they have to be better than average at avoiding surveillance."), examples: vec![
            String::from("CIA or MI6 case officer"),
            String::from("SVR journalist under cover"),
            String::from("Italian ROS major running Mafia snitches"),
            String::from("French DRM intelligence officer monitoring warlord activity in Bosnia"),
        ] ,
        investigative_point_buys: vec![
            InvestigativePointBuy::new(InvestigativeAbilityName::BullshitDetector, 2),
            InvestigativePointBuy::new(InvestigativeAbilityName::Intimidation, 2),
            InvestigativePointBuy::new(InvestigativeAbilityName::Negotiation, 1),
            InvestigativePointBuy::new(InvestigativeAbilityName::Reassurance, 1),
        ], general_point_buys: vec![
            
        ] }
    }
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn
    pub(crate) fn 
}

impl Background {
    pub(crate) fn backgrounds() -> &'static [BackGroundName] {
        &[
            BackGroundName::Analyst,
            BackGroundName::AssetHandler,
            BackGroundName::Bagman,
            BackGroundName::BangAndBurner,
            BackGroundName::BlackBagger,
            BackGroundName::Cleaner,
            BackGroundName::Cobbler,
            BackGroundName::Cuckoo,
            BackGroundName::Hacker,
            BackGroundName::Investigator,
            BackGroundName::Medic,
            BackGroundName::Mule,
            BackGroundName::Muscle,
            BackGroundName::Watcher,
            BackGroundName::WetWorker,
            BackGroundName::WheelArtist,
            BackGroundName::WireRat,
        ]
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub(crate) enum BackGroundName {
    Analyst,
    AssetHandler,
    Bagman,
    BangAndBurner,
    BlackBagger,
    Cleaner,
    Cobbler,
    Cuckoo,
    Hacker,
    Investigator,
    Medic,
    Mule,
    Muscle,
    Watcher,
    WetWorker,
    WheelArtist,
    WireRat,
    Custom(String)
}


impl fmt::Display for BackGroundName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Analyst => write!(f, "Analyst"),
            Self::AssetHandler => write!(f, "Asset Handler"),
            Self::Bagman => write!(f, "Bagman"),
            Self::BangAndBurner => write!(f, "Bang and Burner"),
            Self::BlackBagger => write!(f, "Black Bagger"),
            Self::Cleaner => write!(f, "Cleaner"),
            Self::Cobbler => write!(f, "Cobbler"),
            Self::Cuckoo => write!(f, "Cuckoo"),
            Self::Hacker => write!(f, "Hacker"),
            Self::Investigator => write!(f, "Investigator"),
            Self::Medic => write!(f, "Medic"),
            Self::Mule => write!(f, "Mule"),
            Self::Muscle => write!(f, "Muscle"),
            Self::Watcher => write!(f, "Watcher"),
            Self::WetWorker => write!(f, "Wet Worker"),
            Self::WheelArtist => write!(f, "Wheel Artist"),
            Self::WireRat => write!(f, "Wire Rat"),
            Self::Custom(name) => write!(f, "{name}"),
        }
    }
}


impl FromStr for BackGroundName {
    type Err = NbaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let background: BackGroundName = match s {
            "Analyst" => Self::Analyst,
            "Asset Handler" => Self::AssetHandler,
            "Bagman" => Self::Bagman,
            "Bang and Burner" => Self::BangAndBurner,
            "Black Bagger" => Self::BlackBagger,
            "Cleaner" => Self::Cleaner,
            "Cobbler" => Self::Cobbler,
            "Cuckoo" => Self::Cuckoo,
            "Hacker" => Self::Hacker,
            "Investigator" => Self::Investigator,
            "Medic" => Self::Medic,
            "Mule" => Self::Mule,
            "Muscle" => Self::Muscle,
            "Watcher" => Self::Watcher,
            "Wet Worker" => Self::WetWorker,
            "Wheel Artist" => Self::WheelArtist,
            "Wire Rat" => Self::WireRat,
            _ => Self::Custom(s.to_string())
        };

        Ok(background)
    }
}