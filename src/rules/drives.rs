use core::fmt;

use super::modes::GameMode;

pub(crate) enum Drive {
    Altruism,
    Atonement,
    Comradeship,
    Mystery,
    NowhereElseToGo,
    Patriotism,
    INeverLeft,
    Programming,
    Restoration,
    Revenge,
    Slayer,
    ThrillSeeker,
    Transparency,
    Collector,
}

impl Drive {
    pub(crate) fn drives() -> [Drive; 14] {
        [
    Self::Altruism,
    Self::Atonement,
    Self::Comradeship,
    Self::Mystery,
    Self::NowhereElseToGo,
    Self::Patriotism,
    Self::INeverLeft,
    Self::Programming,
    Self::Restoration,
    Self::Revenge,
    Self::Slayer,
    Self::ThrillSeeker,
    Self::Transparency,
    Self::Collector,
    ]
    }

    pub(crate) fn description(&self, modes: &[GameMode]) -> &'static str {
        match self {
            Drive::Altruism => todo!(),
            Drive::Atonement => todo!(),
            Drive::Comradeship => todo!(),
            Drive::Mystery => todo!(),
            Drive::NowhereElseToGo => todo!(),
            Drive::Patriotism => todo!(),
            Drive::INeverLeft => todo!(),
            Drive::Programming => todo!(),
            Drive::Restoration => todo!(),
            Drive::Revenge => todo!(),
            Drive::Slayer => todo!(),
            Drive::ThrillSeeker => todo!(),
            Drive::Transparency => todo!(),
            Drive::Collector => todo!(),
        }
    }

    pub(crate) fn altruism_description(&self, modes: &[GameMode]) -> String {
        let mut description = String::from("You got into the game to protect innocents from terrorists, or disease, or war, or tyranny. You’re no innocent yourself now, but that only lets you know just how much innocence normal people have left to lose. Without a necessarily compromised agency telling you what to do, doing the right thing becomes not just easier, but imperative.");
        if modes.contains(&GameMode::Mirror) {
            description.push_str("Is another agent acting from shadier motives? Perhaps doing the right thing isn’t what everyone wants to do today.");
        }

        description
    }

    pub(crate) fn game_modes(&self) -> &'static [GameMode] {
        match self {
            Self::Altruism | Self::Atonement => &[
                GameMode::Base,
                GameMode::Burn,
                GameMode::Dust,
                GameMode::Mirror,
                GameMode::Stakes
            ],
            Self::Comradeship => todo!(),
            Self::Mystery => todo!(),
            Self::NowhereElseToGo => todo!(),
            Self::Patriotism => todo!(),
            Self::INeverLeft => todo!(),
            Self::Programming => todo!(),
            Self::Restoration => todo!(),
            Self::Revenge => todo!(),
            Self::Slayer => todo!(),
            Self::ThrillSeeker => todo!(),
            Self::Transparency => todo!(),
            Self::Collector => todo!(),
        }
    }
}

impl fmt::Display for Drive {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Drive::Altruism => write!(f, "Altruism"),
            Drive::Atonement => write!(f, "Atonement"),
            Drive::Comradeship => write!(f, "Comradeship"),
            Drive::Mystery => write!(f, "Mystery"),
            Drive::NowhereElseToGo => write!(f, "Nowhere Else To Go"),
            Drive::Patriotism => write!(f, "Patriotism"),
            Drive::INeverLeft => write!(f, "I Never Left"),
            Drive::Programming => write!(f, "Programming"),
            Drive::Restoration => write!(f, "Restoration"),
            Drive::Revenge => write!(f, "Revenge"),
            Drive::Slayer => write!(f, "Slayer"),
            Drive::ThrillSeeker => write!(f, "Thrill Seeker"),
            Drive::Transparency => write!(f, "Transparency"),
            Drive::Collector => write!(f, "Collector"),
        }
    }
}