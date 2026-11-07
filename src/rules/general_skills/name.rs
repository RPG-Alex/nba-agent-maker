use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum GeneralSkillName {
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

impl Display for GeneralSkillName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Athletics => write!(f, "Athletics"),
            Self::Conceal => write!(f, "Conceal"),
            Self::Cover => write!(f, "Cover"),
            Self::DigitalIntrusion => write!(f, "Digital Intrusion"),
            Self::Disguise => write!(f, "Disguise"),
            Self::Driving => write!(f, "Driving"),
            Self::ExplosiveDevices => write!(f, "Explosive Devices"),
            Self::Filch => write!(f, "Filch"),
            Self::Gambling => write!(f, "Gambling"),
            Self::HandToHand => write!(f, "Hand to Hand"),
            Self::Infiltration => write!(f, "Infiltration"),
            Self::Mechanics => write!(f, "Mechanics"),
            Self::Medic => write!(f, "Medic"),
            Self::Network => write!(f, "Network"),
            Self::Piloting => write!(f, "Piloting"),
            Self::Preparedness => write!(f, "Preparedness"),
            Self::SenseTrouble => write!(f, "Sense Trouble"),
            Self::Shooting => write!(f, "Shooting"),
            Self::Shrink => write!(f, "Shrink"),
            Self::Surveillance => write!(f, "Surveillance"),
            Self::Weapons => write!(f, "Weapons"),
        }
    }
}

impl GeneralSkillName {
    pub const ALL: [GeneralSkillName; 21] = [
        Self::Athletics,
        Self::Conceal,
        Self::Cover,
        Self::DigitalIntrusion,
        Self::Disguise,
        Self::Driving,
        Self::ExplosiveDevices,
        Self::Filch,
        Self::Gambling,
        Self::HandToHand,
        Self::Infiltration,
        Self::Mechanics,
        Self::Medic,
        Self::Network,
        Self::Piloting,
        Self::Preparedness,
        Self::SenseTrouble,
        Self::Shooting,
        Self::Shrink,
        Self::Surveillance,
        Self::Weapons,
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::Athletics => 0,
            Self::Conceal => 1,
            Self::Cover => 2,
            Self::DigitalIntrusion => 3,
            Self::Disguise => 4,
            Self::Driving => 5,
            Self::ExplosiveDevices => 6,
            Self::Filch => 7,
            Self::Gambling => 8,
            Self::HandToHand => 9,
            Self::Infiltration => 10,
            Self::Mechanics => 11,
            Self::Medic => 12,
            Self::Network => 13,
            Self::Piloting => 14,
            Self::Preparedness => 15,
            Self::SenseTrouble => 16,
            Self::Shooting => 17,
            Self::Shrink => 18,
            Self::Surveillance => 19,
            Self::Weapons => 20,
        }
    }
}
