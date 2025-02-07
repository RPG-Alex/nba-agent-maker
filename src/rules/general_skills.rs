use crate::rules::{cherries::*, maneuvers::*};

#[derive(Clone, PartialEq)]
pub struct GeneralSkill {
    pub skill: String,
    pub rating: i32,
    pub cherries: Vec<Cherry>,
    pub maneuvers: Vec<Maneuver>,
    pub is_mos: bool,
}

impl GeneralSkill {
    pub fn new(name: &str, cherries: Vec<Cherry>, maneuvers: Vec<Maneuver>) -> Self {
        Self {
            skill: name.to_string(),
            rating: 0,
            cherries,
            maneuvers,
            is_mos: false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct GeneralSkills {
    pub athletics: GeneralSkill,
    pub conceal: GeneralSkill,
    pub cover: GeneralSkill,
    pub digital_intrusion: GeneralSkill,
    pub disguise: GeneralSkill,
    pub driving: GeneralSkill,
    pub explosive_devices: GeneralSkill,
    pub filch: GeneralSkill,
    pub gambling: GeneralSkill,
    pub hand_to_hand: GeneralSkill,
    pub infiltration: GeneralSkill,
    pub mechanics: GeneralSkill,
    pub medic: GeneralSkill,
    pub network: GeneralSkill,
    pub piloting: GeneralSkill,
    pub preparedness: GeneralSkill,
    pub sense_trouble: GeneralSkill,
    pub shooting: GeneralSkill,
    pub shrink: GeneralSkill,
    pub surveillance: GeneralSkill,
    pub weapons: GeneralSkill,
}

impl GeneralSkills {
    pub fn new() -> Self {
        Self {
            athletics: GeneralSkill::new(
                "Athletics",
                vec![
                    Cherry::new(
                        "Hard to Hit (NBA p.27): +1 to your Hit Threshold in Combat (Dust mode)",
                    ),
                    Cherry::new(
                        "Roll Through the Pain (p.38): Spend Health to Succeed at failed Athletics Tests",
                    ),
                    Cherry::new(
                        "Runner's Intuition (p.38): Spend 1 Athletics to judge opponent’s Athletics rating level compared to yours (Dust mode)",
                    ),
                ],
                vec![
                    Maneuver::new("Breakfall (NBA p.80)"),
                    Maneuver::new("Like Smoke (p.51)"),
                    Maneuver::new("Parkour (NBA p.58)"),
                    Maneuver::new("Support Moves (NBA p.76)"),
                ],
            ),
            conceal: GeneralSkill::new(
                "Conceal",
                vec![
                    Cherry::new(
                        "Bug Stasher (p.39): Hide a bug against all but SIGINT-agency search or specialized equipment",
                    ),
                    Cherry::new(
                        "Perfect Holdout (NBA p.27): Hide a small object on your person against all but X-ray or strip search",
                    ),
                ],
                vec![], // No maneuvers for Conceal
            ),
            cover: GeneralSkill::new("Cover", vec![], vec![]),
            digital_intrusion: GeneralSkill::new(
                "Digital Intrusion",
                vec![
                    Cherry::new("Cracker's Crypto (NBA p.28): 1 free rating point in Cryptography"),
                    Cherry::new(
                        "Head of a PIN (p.39): If you see others input passwords, you can guess them later",
                    ),
                    Cherry::new(
                        "Mr. Clean (p.39): Your smartphones and other devices are untraceable (Dust mode)",
                    ),
                ],
                vec![
                    Maneuver::new("Digital Judo (p.81)"),
                    Maneuver::new("m4d sk1llz (p.51)"),
                ],
            ),
            disguise: GeneralSkill::new(
                "Disguise",
                vec![
                    Cherry::new(
                        "Connected Cover (NBA p.28): Use Cover to create an identity already known to an NPC",
                    ),
                    Cherry::new(
                        "Innocent Bystander (p.40): May spend Disguise pool points on Surveillance tests (Dust mode)",
                    ),
                    Cherry::new(
                        "Just the Help (p.40): In servants' or workman’s uniform, lower Disguise Difficulties by 1 (Dust mode)",
                    ),
                ],
                vec![
                    Maneuver::new("Alibi (p.49)"),
                    Maneuver::new("Quick Change (p.51)"),
                ],
            ),
            driving: GeneralSkill::new(
                "Driving",
                vec![
                    Cherry::new(
                        "Defensive Driving (p.40): +1 to Hit Threshold and to Difficulty of ramming, etc. for you, passengers, and vehicle (Dust mode)",
                    ),
                    Cherry::new(
                        "Grand Theft Auto (NBA p.29): Spend 1 Driving to steal any standard vehicle you can drive",
                    ),
                ],
                vec![
                    Maneuver::new("Gear Devil (NBA p.56)"),
                    Maneuver::new("Signature Wheels (p.52)"),
                ],
            ),
            explosive_devices: GeneralSkill::new(
                "Explosive Devices",
                vec![
                    Cherry::new(
                        "Bigger Bang (NBA p.29): Spend 3 Explosive Devices points to add a die of damage to an explosive charge",
                    ),
                    Cherry::new(
                        "Maestro of Destruction (p.41): Your bombs cannot be disarmed without your aid or wiring diagram",
                    ),
                ],
                vec![], // No maneuvers for Explosive Devices
            ),
            filch: GeneralSkill::new(
                "Filch",
                vec![
                    Cherry::new(
                        "A Lift in Time Saves Nine (p.41): Retroactively declare you lifted a small object in a previous scene",
                    ),
                    Cherry::new(
                        "No Slipups (NBA p.29): After failing a Filch test, spend 2 Filch to bump the result by 1",
                    ),
                ],
                vec![], // No maneuvers for Filch
            ),
            gambling: GeneralSkill::new(
                "Gambling",
                vec![
                    Cherry::new(
                        "All In (p.42): Once per session, refresh any General pool by up to 3",
                    ),
                    Cherry::new(
                        "Everybody's Got a Tell (p.42): 1 free rating point in Bullshit Detector (Dust mode)",
                    ),
                    Cherry::new(
                        "Luck of the Devil (NBA p.30): Roll one die at session start; replace any one die result later with that roll",
                    ),
                ],
                vec![Maneuver::new("Card Up the Sleeve (p.50)")],
            ),
            hand_to_hand: GeneralSkill::new(
                "Hand-to-Hand",
                vec![
                    Cherry::new(
                        "Eye of the Tiger (NBA p.31): Spend 1 HtH to gauge opponent’s HtH rating level (Dust mode)",
                    ),
                    Cherry::new(
                        "Haymaker (p.43): Roll two dice for damage; pick the higher, then spend 1 HtH per extra damage point",
                    ),
                ],
                vec![
                    Maneuver::new("Breakfall (NBA p.80)"),
                    Maneuver::new("Extra Attacks (NBA p.74)"),
                    Maneuver::new("Martial Arts (NBA p.75)"),
                    Maneuver::new("Mook Shield (NBA p.76)"),
                    Maneuver::new("One-Two Punch (p.53)"),
                ],
            ),
            infiltration: GeneralSkill::new(
                "Infiltration",
                vec![
                    Cherry::new("Bono Cane (p.43): Spend 1 Infiltration to bypass guard dogs"),
                    Cherry::new(
                        "Escape Artist (p.44): Given enough time unobserved, you can escape any restraint",
                    ),
                    Cherry::new(
                        "Open Sesame (NBA p.31): Open or bypass any normal or commercial lock or alarm without a test",
                    ),
                ],
                vec![
                    Maneuver::new("Like Smoke (p.51)"),
                    Maneuver::new("Mark and Strike (p.53)"),
                    Maneuver::new("Run and Hide (p.84)"),
                ],
            ),
            mechanics: GeneralSkill::new(
                "Mechanics",
                vec![
                    Cherry::new(
                        "Demolition Man (p.44): Spend Mechanics on Explosive Devices tests to rig vehicles or machinery to explode",
                    ),
                    Cherry::new(
                        "Swiss Army Prep (NBA p.31): May spend Mechanics on Preparedness tests with jaunty narration",
                    ),
                    Cherry::new(
                        "Trapmaster (p.44): Spend 2 Mechanics to inflict two instances of damage with a non-explosive booby trap",
                    ),
                ],
                vec![Maneuver::new("Grease Monkey (p.51)")],
            ),
            medic: GeneralSkill::new(
                "Medic",
                vec![
                    Cherry::new(
                        "Medical School of Hard Knocks (NBA p.31): 1 free rating point in Diagnosis (Dust mode)",
                    ),
                    Cherry::new(
                        "On Your Feet (p.45): Spend Medic points to give another agent a bonus to their Consciousness roll",
                    ),
                ],
                vec![Maneuver::new("Verbal Trauma Unit (p.52)")],
            ),
            network: GeneralSkill::new("Network", vec![], vec![]),
            piloting: GeneralSkill::new(
                "Piloting",
                vec![
                    Cherry::new(
                        "Grand Theft Aero or Aqua (NBA p.33): Spend 1 Piloting to steal an operable standard vehicle; forge flight plans/port documents",
                    ),
                    Cherry::new(
                        "Move Around the Cabin (p.45): On a boat/aircraft, -1 Difficulty to Athletics tests and to foe’s Hit Threshold when you attack with Hand to Hand or Weapons (Dust mode)",
                    ),
                ],
                vec![Maneuver::new("Gear Devil (NBA p.56)")],
            ),
            preparedness: GeneralSkill::new(
                "Preparedness",
                vec![
                    Cherry::new(
                        "Check Your Other Left Pocket (p.45): May spend Preparedness on behalf of other players",
                    ),
                    Cherry::new(
                        "Hoarder (p.45): Lower all Cache test (NBA p.94) Difficulties by 1 (Dust mode)",
                    ),
                    Cherry::new(
                        "In the Nick of Time (NBA p.33): Retroactively plan for actions as needed (roll still required)",
                    ),
                ],
                vec![Maneuver::new("Calculated Risk (p.50)")],
            ),
            sense_trouble: GeneralSkill::new(
                "Sense Trouble",
                vec![
                    Cherry::new(
                        "Combat Intuition (NBA p.34): Use Sense Trouble instead of another ability to determine action order",
                    ),
                    Cherry::new("Hawkeye (p.46): 1 free rating point in Notice (Dust mode)"),
                ],
                vec![
                    Maneuver::new("Danger Zone (p.50)"),
                    Maneuver::new("Perfect Drop (p.53)"),
                ],
            ),
            shooting: GeneralSkill::new(
                "Shooting",
                vec![], // No cherries listed
                vec![
                    Maneuver::new("Extra Attacks (NBA p.74)"),
                    Maneuver::new("Sniping (NBA p.76)"),
                    Maneuver::new("Special Weapons Training (NBA p.76)"),
                    Maneuver::new("Suppressive Fire (NBA p.77)"),
                    Maneuver::new("Technothriller Monologue (NBA p.77)"),
                ],
            ),
            shrink: GeneralSkill::new(
                "Shrink",
                vec![
                    Cherry::new(
                        "Anger Management (p.47): Make a Shrink test to anger or enrage a subject known to you",
                    ),
                    Cherry::new(
                        "Talk It Out (NBA p.34): 1 free rating point in Bullshit Detector, Flattery, Interrogation, or Reassurance (Dust mode)",
                    ),
                    Cherry::new(
                        "Talking Cure (p.47): Mental illness tests are at -1 Difficulty; restore 3 Stability for 2 Shrink spent on triage",
                    ),
                ],
                vec![], // No maneuvers for Shrink
            ),
            surveillance: GeneralSkill::new(
                "Surveillance",
                vec![
                    Cherry::new(
                        "Face in the Crowd (p.47): Losing your quarry doesn't blow your cover",
                    ),
                    Cherry::new(
                        "Tail Lights (p.47): May spend Driving on Surveillance tests while in a moving vehicle",
                    ),
                    Cherry::new(
                        "The Wire (NBA p.35): 1 free rating point in Electronic Surveillance",
                    ),
                ],
                vec![
                    Maneuver::new("Blending Agent (p.50)"),
                    Maneuver::new("For Your Eyes Only (p.50)"),
                    Maneuver::new("Watching the Watchers (p.83)"),
                ],
            ),
            weapons: GeneralSkill::new(
                "Weapons",
                vec![
                    Cherry::new(
                        "Quincey Morris' Bowie Knife (NBA p.35): Throw balanced hand weapons within Near range at no penalty",
                    ),
                    Cherry::new(
                        "Riposte (p.48): After an attacker rolls a 1 and misses, spend Weapons points to do damage up to your weapon’s max",
                    ),
                ],
                vec![
                    Maneuver::new("Extra Attacks (NBA p.74)"),
                    Maneuver::new("Martial Arts (NBA p.75)"),
                    Maneuver::new("One-Two Punch (p.53)"),
                    Maneuver::new("Special Weapons Training (NBA p.76)"),
                ],
            ),
        }
    }
    pub fn set_mos(&mut self, skill_name: &str) {
        let mut all_skills = [
            &mut self.athletics,
            &mut self.conceal,
            &mut self.cover,
            &mut self.digital_intrusion,
            &mut self.disguise,
            &mut self.driving,
            &mut self.explosive_devices,
            &mut self.filch,
            &mut self.gambling,
            &mut self.hand_to_hand,
            &mut self.infiltration,
            &mut self.mechanics,
            &mut self.medic,
            &mut self.network,
            &mut self.piloting,
            &mut self.preparedness,
            &mut self.sense_trouble,
            &mut self.shooting,
            &mut self.shrink,
            &mut self.surveillance,
            &mut self.weapons,
        ];

        for skill in &mut all_skills {
            skill.is_mos = skill.skill == skill_name;
        }
    }
}

pub fn gen_skills_list() -> Vec<&'static str> {
    vec![
        "Athletics",
        "Conceal",
        "Cover",
        "Digital Intrusion",
        "Disguise",
        "Driving",
        "Explosive Devices",
        "Filch",
        "Gambling",
        "Hand-to-Hand",
        "Infiltration",
        "Mechanics",
        "Medic",
        "Network",
        "Piloting",
        "Preparedness",
        "Sense Trouble",
        "Shooting",
        "Shrink",
        "Surveillance",
        "Weapons",
    ]
}
