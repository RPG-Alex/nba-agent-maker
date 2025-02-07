#[derive(Clone, PartialEq)]
pub struct Cherry {
    pub description: String,
    pub chosen: bool,
}

impl Cherry {
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
            chosen: false,
        }
    }
}

// ATHLETICS
pub fn athletic_cherries() -> Vec<&'static str> {
    vec![
        "Hard to Hit (NBA p.27): +1 to your Hit Threshold in Combat (Dust mode)",
        "Roll Through the Pain (p.38): Spend Health to Succeed at failed Athletics Tests",
        "Runner's Intuition (p.38): Spend 1 Athletics to judge opponent’s Athletics rating level compared to yours (Dust mode)",
    ]
}

pub fn athletic_maneuvers() -> Vec<&'static str> {
    vec![
        "Breakfall (NBA p.80)",
        "Like Smoke (p.51)",
        "Parkour (NBA p.58)",
        "Support Moves (NBA p.76)",
    ]
}

// CONCEAL
pub fn conceal_cherries() -> Vec<&'static str> {
    vec![
        "Bug Stasher (p.39): Hide a bug against all but SIGINT-agency search or specialized equipment",
        "Perfect Holdout (NBA p.27): Hide a small object on your person against all but X-ray or strip search",
    ]
}

pub fn conceal_maneuvers() -> Vec<&'static str> {
    vec![]
}

// DIGITAL INTRUSION
pub fn digital_intrusion_cherries() -> Vec<&'static str> {
    vec![
        "Cracker's Crypto (NBA p.28): 1 free rating point in Cryptography",
        "Head of a PIN (p.39): If you see others input passwords, you can guess them later",
        "Mr. Clean (p.39): Your smartphones and other devices are untraceable (Dust mode)",
    ]
}

pub fn digital_intrusion_maneuvers() -> Vec<&'static str> {
    vec!["Digital Judo (p.81)", "m4d sk1llz (p.51)"]
}

// DISGUISE
pub fn disguise_cherries() -> Vec<&'static str> {
    vec![
        "Connected Cover (NBA p.28): Use Cover to create an identity already known to an NPC",
        "Innocent Bystander (p.40): May spend Disguise pool points on Surveillance tests (Dust mode)",
        "Just the Help (p.40): In servants' or workman’s uniform, lower Disguise Difficulties by 1 (Dust mode)",
    ]
}

pub fn disguise_maneuvers() -> Vec<&'static str> {
    vec!["Alibi (p.49)", "Quick Change (p.51)"]
}

// DRIVING
pub fn driving_cherries() -> Vec<&'static str> {
    vec![
        "Defensive Driving (p.40): +1 to Hit Threshold and to Difficulty of ramming, etc. for you, passengers, and vehicle (Dust mode)",
        "Grand Theft Auto (NBA p.29): Spend 1 Driving to steal any standard vehicle you can drive",
    ]
}

pub fn driving_maneuvers() -> Vec<&'static str> {
    vec!["Gear Devil (NBA p.56)", "Signature Wheels (p.52)"]
}

// EXPLOSIVE DEVICES
pub fn explosive_devices_cherries() -> Vec<&'static str> {
    vec![
        "Bigger Bang (NBA p.29): Spend 3 Explosive Devices points to add a die of damage to an explosive charge",
        "Maestro of Destruction (p.41): Your bombs cannot be disarmed without your aid or wiring diagram",
    ]
}

pub fn explosive_devices_maneuvers() -> Vec<&'static str> {
    vec![]
}

// FILCH
pub fn filch_cherries() -> Vec<&'static str> {
    vec![
        "A Lift in Time Saves Nine (p.41): Retroactively declare you lifted a small object in a previous scene",
        "No Slipups (NBA p.29): After failing a Filch test, spend 2 Filch to bump the result by 1",
    ]
}

pub fn filch_maneuvers() -> Vec<&'static str> {
    vec![]
}

// GAMBLING
pub fn gambling_cherries() -> Vec<&'static str> {
    vec![
        "All In (p.42): Once per session, refresh any General pool by up to 3",
        "Everybody's Got a Tell (p.42): 1 free rating point in Bullshit Detector (Dust mode)",
        "Luck of the Devil (NBA p.30): Roll one die at session start; replace any one die result later with that roll",
    ]
}

pub fn gambling_maneuvers() -> Vec<&'static str> {
    vec!["Card Up the Sleeve (p.50)"]
}

// HAND-TO-HAND
pub fn hand_to_hand_cherries() -> Vec<&'static str> {
    vec![
        "Eye of the Tiger (NBA p.31): Spend 1 HtH to gauge opponent’s HtH rating level (Dust mode)",
        "Haymaker (p.43): Roll two dice for damage; pick the higher, then spend 1 HtH per extra damage point",
    ]
}

pub fn hand_to_hand_maneuvers() -> Vec<&'static str> {
    vec![
        "Breakfall (NBA p.80)",
        "Extra Attacks (NBA p.74)",
        "Martial Arts (NBA p.75)",
        "Mook Shield (NBA p.76)",
        "One-Two Punch (p.53)",
    ]
}

// INFILTRATION
pub fn infiltration_cherries() -> Vec<&'static str> {
    vec![
        "Bono Cane (p.43): Spend 1 Infiltration to bypass guard dogs",
        "Escape Artist (p.44): Given enough time unobserved, you can escape any restraint",
        "Open Sesame (NBA p.31): Open or bypass any normal or commercial lock or alarm without a test",
    ]
}

pub fn infiltration_maneuvers() -> Vec<&'static str> {
    vec![
        "Like Smoke (p.51)",
        "Mark and Strike (p.53)",
        "Run and Hide (p.84)",
    ]
}

// MECHANICS
pub fn mechanics_cherries() -> Vec<&'static str> {
    vec![
        "Demolition Man (p.44): Spend Mechanics on Explosive Devices tests to rig vehicles or machinery to explode",
        "Swiss Army Prep (NBA p.31): May spend Mechanics on Preparedness tests with jaunty narration",
        "Trapmaster (p.44): Spend 2 Mechanics to inflict two instances of damage with a non-explosive booby trap",
    ]
}

pub fn mechanics_maneuvers() -> Vec<&'static str> {
    vec!["Grease Monkey (p.51)"]
}

// MEDIC
pub fn medic_cherries() -> Vec<&'static str> {
    vec![
        "Medical School of Hard Knocks (NBA p.31): 1 free rating point in Diagnosis (Dust mode)",
        "On Your Feet (p.45): Spend Medic points to give another agent a bonus to their Consciousness roll",
    ]
}

pub fn medic_maneuvers() -> Vec<&'static str> {
    vec!["Verbal Trauma Unit (p.52)"]
}

// PILOTING
pub fn piloting_cherries() -> Vec<&'static str> {
    vec![
        "Grand Theft Aero or Aqua (NBA p.33): Spend 1 Piloting to steal an operable standard vehicle; forge flight plans/port documents",
        "Move Around the Cabin (p.45): On a boat/aircraft, -1 Difficulty to Athletics tests and to foe’s Hit Threshold when you attack with Hand to Hand or Weapons (Dust mode)",
    ]
}

pub fn piloting_maneuvers() -> Vec<&'static str> {
    vec!["Gear Devil (NBA p.56)"]
}

// PREPAREDNESS
pub fn preparedness_cherries() -> Vec<&'static str> {
    vec![
        "Check Your Other Left Pocket (p.45): May spend Preparedness on behalf of other players",
        "Hoarder (p.45): Lower all Cache test (NBA p.94) Difficulties by 1 (Dust mode)",
        "In the Nick of Time (NBA p.33): Retroactively plan for actions as needed (roll still required)",
    ]
}

pub fn preparedness_maneuvers() -> Vec<&'static str> {
    vec!["Calculated Risk (p.50)"]
}

// SENSE TROUBLE
pub fn sense_trouble_cherries() -> Vec<&'static str> {
    vec![
        "Combat Intuition (NBA p.34): Use Sense Trouble instead of another ability to determine action order",
        "Hawkeye (p.46): 1 free rating point in Notice (Dust mode)",
    ]
}

pub fn sense_trouble_maneuvers() -> Vec<&'static str> {
    vec!["Danger Zone (p.50)", "Perfect Drop (p.53)"]
}

// SHOOTING
pub fn shooting_cherries() -> Vec<&'static str> {
    // No direct cherries listed, only 8+ maneuvers
    vec![]
}

pub fn shooting_maneuvers() -> Vec<&'static str> {
    vec![
        "Extra Attacks (NBA p.74)",
        "Sniping (NBA p.76)",
        "Special Weapons Training (NBA p.76)",
        "Suppressive Fire (NBA p.77)",
        "Technothriller Monologue (NBA p.77)",
    ]
}

// SHRINK
pub fn shrink_cherries() -> Vec<&'static str> {
    vec![
        "Anger Management (p.47): Make a Shrink test to anger or enrage a subject known to you",
        "Talk It Out (NBA p.34): 1 free rating point in Bullshit Detector, Flattery, Interrogation, or Reassurance (Dust mode)",
        "Talking Cure (p.47): Mental illness tests are at -1 Difficulty; restore 3 Stability for 2 Shrink spent on triage",
    ]
}

pub fn shrink_maneuvers() -> Vec<&'static str> {
    vec![]
}

// SURVEILLANCE
pub fn surveillance_cherries() -> Vec<&'static str> {
    vec![
        "Face in the Crowd (p.47): Losing your quarry doesn't blow your cover",
        "Tail Lights (p.47): May spend Driving on Surveillance tests while in a moving vehicle",
        "The Wire (NBA p.35): 1 free rating point in Electronic Surveillance",
    ]
}

pub fn surveillance_maneuvers() -> Vec<&'static str> {
    vec![
        "Blending Agent (p.50)",
        "For Your Eyes Only (p.50)",
        "Watching the Watchers (p.83)",
    ]
}

// WEAPONS
pub fn weapons_cherries() -> Vec<&'static str> {
    vec![
        "Quincey Morris' Bowie Knife (NBA p.35): Throw balanced hand weapons within Near range at no penalty",
        "Riposte (p.48): After an attacker rolls a 1 and misses, spend Weapons points to do damage up to your weapon’s max",
    ]
}

pub fn weapons_maneuvers() -> Vec<&'static str> {
    vec![
        "Extra Attacks (NBA p.74)",
        "Martial Arts (NBA p.75)",
        "One-Two Punch (p.53)",
        "Special Weapons Training (NBA p.76)",
    ]
}
