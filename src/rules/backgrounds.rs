use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct Background {
    pub(crate) name: BackGroundName,
    pub(crate) description: String,
    pub(crate) investigative_point_buys: String,
    pub(crate) general_point_buys: String,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub(crate) struct Backgrounds {
    pub(crate) backgrounds: Vec<Background>
} 

impl Backgrounds {
    pub(crate) fn new() -> Self {
        Self { backgrounds: Vec::new() }
    }
}