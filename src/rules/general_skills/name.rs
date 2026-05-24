use core::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum SkillName {
    Athletics,
    Conceal,
    Cover,
    DigitalIntrusion,
    Disguise,
    Driving,
    ExplosiveDevices,
    Filch,
    Gambling,
    HandToHand,
    Infiltration,
    Mechanics,
    Medic,
    Network,
    Piloting,
    Preparedness,
    SenseTrouble,
    Shooting,
    Shrink,
    Surveillance,
    Weapons,
}

impl Display for SkillName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillName::Athletics => write!(f, "Athletics"),
            SkillName::Conceal => write!(f, "Conceal"),
            SkillName::Cover => write!(f, "Cover"),
            SkillName::DigitalIntrusion => write!(f, "Digital Intrusion"),
            SkillName::Disguise => write!(f, "Disguise"),
            SkillName::Driving => write!(f, "Driving"),
            SkillName::ExplosiveDevices => write!(f, "Explosive Devices"),
            SkillName::Filch => write!(f, "Filch"),
            SkillName::Gambling => write!(f, "Gambling"),
            SkillName::HandToHand => write!(f, "Hand to Hand"),
            SkillName::Infiltration => write!(f, "Infiltration"),
            SkillName::Mechanics => write!(f, "Mechanics"),
            SkillName::Medic => write!(f, "Medic"),
            SkillName::Network => write!(f, "Network"),
            SkillName::Piloting => write!(f, "Piloting"),
            SkillName::Preparedness => write!(f, "Preparedness"),
            SkillName::SenseTrouble => write!(f, "Sense Trouble"),
            SkillName::Shooting => write!(f, "Shooting"),
            SkillName::Shrink => write!(f, "Shrink"),
            SkillName::Surveillance => write!(f, "Surveillance"),
            SkillName::Weapons => write!(f, "Weapons"),
        }
    }
}


impl SkillName {
    pub const ALL: [SkillName; 21] = [
            SkillName::Athletics,
            SkillName::Conceal,
            SkillName::Cover,
            SkillName::DigitalIntrusion,
            SkillName::Disguise,
            SkillName::Driving,
            SkillName::ExplosiveDevices,
            SkillName::Filch,
            SkillName::Gambling,
            SkillName::HandToHand,
            SkillName::Infiltration,
            SkillName::Mechanics,
            SkillName::Medic,
            SkillName::Network,
            SkillName::Piloting,
            SkillName::Preparedness,
            SkillName::SenseTrouble,
            SkillName::Shooting,
            SkillName::Shrink,
            SkillName::Surveillance,
            SkillName::Weapons,
    ];

        pub const fn index(self) -> usize {
        match self {
            SkillName::Athletics => 0,
            SkillName::Conceal => 1,
            SkillName::Cover => 2,
            SkillName::DigitalIntrusion => 3,
            SkillName::Disguise => 4,
            SkillName::Driving => 5,
            SkillName::ExplosiveDevices => 6,
            SkillName::Filch => 7,
            SkillName::Gambling => 8,
            SkillName::HandToHand => 9,
            SkillName::Infiltration => 10,
            SkillName::Mechanics => 11,
            SkillName::Medic => 12,
            SkillName::Network => 13,
            SkillName::Piloting => 14,
            SkillName::Preparedness => 15,
            SkillName::SenseTrouble => 16,
            SkillName::Shooting => 17,
            SkillName::Shrink => 18,
            SkillName::Surveillance => 19,
            SkillName::Weapons => 20,
        }
    }
}