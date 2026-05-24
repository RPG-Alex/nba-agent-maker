use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::rules::{general_skills::name::SkillName, modes::GameMode};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum CherryId {
    HardToHit,
    RollThroughThePain,
    RunnersIntuition,

    BugStasher,
    PerfectHoldout,

    CrackersCrypto,
    HeadOfAPin,
    MrClean,

    ConnectedCover,
    InnocentBystander,
    JustTheHelp,

    DefensiveDriving,
    GrandTheftAuto,

    BiggerBang,
    MaestroOfDestruction,

    ALiftInTimeSavesNine,
    NoSlipups,

    AllIn,
    EverybodysGotATell,
    LuckOfTheDevil,

    EyeOfTheTiger,
    Haymaker,

    BonoCane,
    EscapeArtist,
    OpenSesame,

    DemolitionMan,
    SwissArmyPrep,
    Trapmaster,

    MedicalSchoolOfHardKnocks,
    OnYourFeet,

    GrandTheftAeroOrAqua,
    MoveAroundTheCabin,

    CheckYourOtherLeftPocket,
    Hoarder,
    InTheNickOfTime,

    CombatIntuition,
    Hawkeye,

    AngerManagement,
    TalkItOut,
    TalkingCure,

    FaceInTheCrowd,
    TailLights,
    TheWire,

    QuinceyMorris,
    Riposte,
}

impl CherryId {
    pub fn as_str(&self) -> &'static str {
        match self {
            CherryId::HardToHit => "Hard to Hit",
            CherryId::RollThroughThePain => "Roll Through the Pain",
            CherryId::RunnersIntuition => "Runner's Intuition",

            CherryId::BugStasher => "Bug Stasher",
            CherryId::PerfectHoldout => "Perfect Holdout",

            CherryId::CrackersCrypto => "Cracker's Crypto",
            CherryId::HeadOfAPin => "Head of a pin",
            CherryId::MrClean => "Mr.Clean",

            CherryId::ConnectedCover => "Connected Cover",
            CherryId::InnocentBystander => "Innocent Bystander",
            CherryId::JustTheHelp => "Just the Help",

            CherryId::DefensiveDriving => "Defensive Driving",
            CherryId::GrandTheftAuto => "Grand Theft Auto",

            CherryId::BiggerBang => "Bigger Bang",
            CherryId::MaestroOfDestruction => "Maestro of Destruction",

            CherryId::ALiftInTimeSavesNine => "A Lift in Time Saves Nine",
            CherryId::NoSlipups => "No Slipups",

            CherryId::AllIn => "All In",
            CherryId::EverybodysGotATell => "Everybody's Got a Tell",
            CherryId::LuckOfTheDevil => "Luck of the Devil",

            CherryId::EyeOfTheTiger => "Eye of the Tiger",
            CherryId::Haymaker => "Haymaker",

            CherryId::BonoCane => "Bono Cane",
            CherryId::EscapeArtist => "Escape Artist",
            CherryId::OpenSesame => "Open Sesame",

            CherryId::DemolitionMan => "Demolition Man",
            CherryId::SwissArmyPrep => "Swiss Army Prep",
            CherryId::Trapmaster => "Trapmaster",

            CherryId::MedicalSchoolOfHardKnocks => "Medical School of Hard Knocks",
            CherryId::OnYourFeet => "On Your Feet",

            CherryId::GrandTheftAeroOrAqua => "Grand Theft Aero or Aqua",
            CherryId::MoveAroundTheCabin => "Move Around the Cabin",

            CherryId::CheckYourOtherLeftPocket => "Check Your Other Left Pocket",
            CherryId::Hoarder => "Hoarder",
            CherryId::InTheNickOfTime => "In the Nick of Time",

            CherryId::CombatIntuition => "Combat Intuition",
            CherryId::Hawkeye => "Hawkeye",

            CherryId::AngerManagement => "Anger Management",
            CherryId::TalkItOut => "Talk It Out",
            CherryId::TalkingCure => "Talking Cure",

            CherryId::FaceInTheCrowd => "Face in the Crowd",
            CherryId::TailLights => "Tail Lights",
            CherryId::TheWire => "The Wire",

            CherryId::QuinceyMorris => "Quincey Morris",
            CherryId::Riposte => "Riposte",
        }
    }
}

impl Display for CherryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cherry {
    pub game_mode: GameMode,
    pub name: CherryId,
    pub description: &'static str,
}

impl Cherry {
    pub fn game_mode(&self) -> &GameMode {
        &self.game_mode
    }

    pub fn name(&self) -> &'static str {
        self.name.as_str()
    }

    pub fn description(&self) -> &'static str {
        self.description
    }
    pub fn athletics() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::HardToHit,
                description: "Hard to Hit (NBA p.27): +1 to your Hit Threshold in Combat (Dust mode)",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::RollThroughThePain,
                description: "Roll Through the Pain (p.38): Spend Health to Succeed at failed Athletics Tests",
            },
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::RunnersIntuition,
                description: "Runner's Intuition (p.38): Spend 1 Athletics to judge opponent's Athletics rating level compared to yours (Dust mode)",
            },
        ]
    }
    pub fn conceal() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::BugStasher,
                description: "Bug Stasher (p.39): Hide a bug against all but SIGINT-agency search or specialized equipment",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::PerfectHoldout,
                description: "Perfect Holdout (NBA p.27): Hide a small object on your person against all but X-ray or strip search",
            },
        ]
    }
    pub fn cover() -> &'static [Cherry] {
        &[]
    }
    pub fn digital_intrusion() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::CrackersCrypto,
                description: "Cracker's Crypto (NBA p.28): 1 free rating point in Cryptography",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::HeadOfAPin,
                description: "Head of a PIN (p.39): If you see others input passwords, you can guess them later",
            },
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::MrClean,
                description: "Mr. Clean (p.39): Your smartphones and other devices are untraceable (Dust mode)",
            },
        ]
    }
    pub fn disguise() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::ConnectedCover,
                description: "Connected Cover (NBA p.28): Use Cover to create an identity already known to an NPC",
            },
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::InnocentBystander,
                description: "Innocent Bystander (p.40): May spend Disguise pool points on Surveillance tests (Dust mode)",
            },
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::JustTheHelp,
                description: "Just the Help (p.40): In servants' or workman’s uniform, lower Disguise Difficulties by 1 (Dust mode)",
            },
        ]
    }
    pub fn driving() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::DefensiveDriving,
                description: "Defensive Driving (p.40): +1 to Hit Threshold and to Difficulty of ramming, etc. for you, passengers, and vehicle (Dust mode)",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::GrandTheftAeroOrAqua,
                description: "Grand Theft Auto (NBA p.29): Spend 1 Driving to steal any standard vehicle you can drive",
            },
        ]
    }
    pub fn explosive_devices() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::BiggerBang,
                description: "Bigger Bang (NBA p.29): Spend 3 Explosive Devices points to add a die of damage to an explosive charge",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::MaestroOfDestruction,
                description: "Maestro of Destruction (p.41): Your bombs cannot be disarmed without your aid or wiring diagram",
            },
        ]
    }
    pub fn filch() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::ALiftInTimeSavesNine,
                description: "A Lift in Time Saves Nine (p.41): Retroactively declare you lifted a small object in a previous scene",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::NoSlipups,
                description: "No Slipups (NBA p.29): After failing a Filch test, spend 2 Filch to bump the result by 1",
            },
        ]
    }
    pub fn gambling() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::AllIn,
                description: "All In (p.42): Once per session, refresh any General pool by up to 3",
            },
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::EverybodysGotATell,
                description: "Everybody's Got a Tell (p.42): 1 free rating point in Bullshit Detector (Dust mode)",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::LuckOfTheDevil,
                description: "Luck of the Devil (NBA p.30): Roll one die at session start; replace any one die result later with that roll",
            },
        ]
    }
    pub fn hand_to_hand() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::EyeOfTheTiger,
                description: "Eye of the Tiger (NBA p.31): Spend 1 HtH to gauge opponent’s HtH rating level (Dust mode)",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::Haymaker,
                description: "Haymaker (p.43): Roll two dice for damage; pick the higher, then spend 1 HtH per extra damage point",
            },
        ]
    }
    pub fn infiltration() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::BonoCane,
                description: "Bono Cane (p.43): Spend 1 Infiltration to bypass guard dogs",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::EscapeArtist,
                description: "Escape Artist (p.44): Given enough time unobserved, you can escape any restraint",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::OpenSesame,
                description: "Open Sesame (NBA p.31): Open or bypass any normal or commercial lock or alarm without a test",
            },
        ]
    }
    pub fn mechanics() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::DemolitionMan,
                description: "Demolition Man (p.44): Spend Mechanics on Explosive Devices tests to rig vehicles or machinery to explode",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::SwissArmyPrep,
                description: "Swiss Army Prep (NBA p.31): May spend Mechanics on Preparedness tests with jaunty narration",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::Trapmaster,
                description: "Trapmaster (p.44): Spend 2 Mechanics to inflict two instances of damage with a non-explosive booby trap",
            },
        ]
    }
    pub fn medic() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::MedicalSchoolOfHardKnocks,
                description: "Medical School of Hard Knocks (NBA p.31): 1 free rating point in Diagnosis (Dust mode)",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::OnYourFeet,
                description: "On Your Feet (p.45): Spend Medic points to give another agent a bonus to their Consciousness roll",
            },
        ]
    }
    pub fn network() -> &'static [Cherry] {
        &[]
    }
    pub fn piloting() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::GrandTheftAeroOrAqua,
                description: "Grand Theft Aero or Aqua (NBA p.33): Spend 1 Piloting to steal an operable standard vehicle; forge flight plans/port documents",
            },
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::MoveAroundTheCabin,
                description: "Move Around the Cabin (p.45): On a boat/aircraft, -1 Difficulty to Athletics tests and to foe’s Hit Threshold when you attack with Hand to Hand or Weapons (Dust mode)",
            },
        ]
    }
    pub fn preparedness() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::CheckYourOtherLeftPocket,
                description: "Check Your Other Left Pocket (p.45): May spend Preparedness on behalf of other players",
            },
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::Hoarder,
                description: "Hoarder (p.45): Lower all Cache test (NBA p.94) Difficulties by 1 (Dust mode)",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::InTheNickOfTime,
                description: "In the Nick of Time (NBA p.33): Retroactively plan for actions as needed (roll still required)",
            },
        ]
    }
    pub fn sense_trouble() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::CombatIntuition,
                description: "Combat Intuition (NBA p.34): Use Sense Trouble instead of another ability to determine action order",
            },
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::Hawkeye,
                description: "Hawkeye (p.46): 1 free rating point in Notice (Dust mode)",
            },
        ]
    }
    pub fn shooting() -> &'static [Cherry] {
        &[]
    }
    pub fn shrink() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::AngerManagement,
                description: "Anger Management (p.47): Make a Shrink test to anger or enrage a subject known to you",
            },
            Cherry {
                game_mode: GameMode::Dust,
                name: CherryId::TalkItOut,
                description: "Talk It Out (NBA p.34): 1 free rating point in Bullshit Detector, Flattery, Interrogation, or Reassurance (Dust mode)",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::TalkingCure,
                description: "Talking Cure (p.47): Mental illness tests are at -1 Difficulty; restore 3 Stability for 2 Shrink spent on triage",
            },
        ]
    }
    pub fn surveillance() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::FaceInTheCrowd,
                description: "Face in the Crowd (p.47): Losing your quarry doesn't blow your cover",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::TailLights,
                description: "Tail Lights (p.47): May spend Driving on Surveillance tests while in a moving vehicle",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::TheWire,
                description: "The Wire (NBA p.35): 1 free rating point in Electronic Surveillance",
            },
        ]
    }
    pub fn weapons() -> &'static [Cherry] {
        &[
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::QuinceyMorris,
                description: "Quincey Morris' Bowie Knife (NBA p.35): Throw balanced hand weapons within Near range at no penalty",
            },
            Cherry {
                game_mode: GameMode::None,
                name: CherryId::Riposte,
                description: "Riposte (p.48): After an attacker rolls a 1 and misses, spend Weapons points to do damage up to your weapon’s max",
            },
        ]
    }
}

impl SkillName {
    pub fn cherries(self) -> &'static [Cherry] {
        match self {
            SkillName::Athletics => Cherry::athletics(),
            SkillName::Conceal => Cherry::conceal(),
            // cover has no cherries.
            SkillName::Cover => Cherry::cover(),
            SkillName::DigitalIntrusion => Cherry::digital_intrusion(),
            SkillName::Disguise => Cherry::disguise(),
            SkillName::Driving => Cherry::driving(),
            SkillName::ExplosiveDevices => Cherry::explosive_devices(),
            SkillName::Filch => Cherry::filch(),
            SkillName::Gambling => Cherry::gambling(),
            SkillName::HandToHand => Cherry::hand_to_hand(),
            SkillName::Infiltration => Cherry::infiltration(),
            SkillName::Mechanics => Cherry::mechanics(),
            SkillName::Medic => Cherry::medic(),
            SkillName::Network => Cherry::network(),
            SkillName::Piloting => Cherry::piloting(),
            SkillName::Preparedness => Cherry::preparedness(),
            SkillName::SenseTrouble => Cherry::sense_trouble(),
            SkillName::Shooting => Cherry::shooting(),
            SkillName::Shrink => Cherry::shrink(),
            SkillName::Surveillance => Cherry::surveillance(),
            SkillName::Weapons => Cherry::weapons(),
        }
    }
}
