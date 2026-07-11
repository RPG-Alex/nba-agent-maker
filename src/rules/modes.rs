use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq)]
pub enum GameMode {
    #[default]
    Base, // base game
    Burn,
    Dust,
    Mirror,
    Stakes,
}

impl GameMode {
    pub fn info(&self) -> String {
        match self {
            GameMode::Base => String::from("The base game. The default setting of Night's Black Agents is a cinematic thriller."),
            GameMode::Burn => String::from("In burn mode games, psychological damage is more intense; the actions agents must take inevitably burn away their humanity. Your Stability is capped at 12, and degrades faster. Killing is never easy, and  ever free."),
            GameMode::Dust => String::from("In dust mode, the vampires and their agents will be far more challenging and powerful in open combat. Design operations that avoid shootouts unless the team has an overwhelming positional advantage, or some surprising ace in the hole."),
            GameMode::Mirror => String::from("In mirror mode games, your contacts and even your team are unreliable; your partners can help you with Trust, or destroy you with Betrayal. Unlike the other modes, mirror mode games encourage player vs. player story lines or active conflict."),
            GameMode::Stakes => String::from("In stakes mode games, you have Drives that urge you forward; this rule is highly recommended for games in any mode. In burn mode, Drives can force the characters to sacrifice themselves; in mirror mode, conflicting agendas can escalate the drama. Even dust mode agents often aim higher than just getting out from under the looming threat."),
        }
    }
}

impl Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                GameMode::Dust => write!(f, "Dust"),
                GameMode::Mirror => write!(f, "Mirror"),
                GameMode::Stakes => write!(f, "Stakes"),
                GameMode::Base => write!(f, "Base"),
                GameMode::Burn => write!(f, "Burn"),
            }
    }
}