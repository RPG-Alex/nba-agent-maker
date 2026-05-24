use crate::rules::general_skills::name::SkillName;

impl SkillName {
    pub fn maneuvers(self) -> &'static [&'static str] {
        match self {
            SkillName::Athletics => &[
                "Breakfall (NBA p.80)",
                "Like Smoke (p.51)",
                "Parkour (NBA p.58)",
                "Support Moves (NBA p.76)",
            ],

            SkillName::Conceal => &[],

            SkillName::Cover => &[],

            SkillName::DigitalIntrusion => &["Digital Judo (p.81)", "m4d sk1llz (p.51)"],

            SkillName::Disguise => &["Alibi (p.49)", "Quick Change (p.51)"],

            SkillName::Driving => &["Gear Devil (NBA p.56)", "Signature Wheels (p.52)"],

            SkillName::ExplosiveDevices => &[],

            SkillName::Filch => &[],

            SkillName::Gambling => &["Card Up the Sleeve (p.50)"],

            SkillName::HandToHand => &[
                "Breakfall (NBA p.80)",
                "Extra Attacks (NBA p.74)",
                "Martial Arts (NBA p.75)",
                "Mook Shield (NBA p.76)",
                "One-Two Punch (p.53)",
            ],

            SkillName::Infiltration => &[
                "Like Smoke (p.51)",
                "Mark and Strike (p.53)",
                "Run and Hide (p.84)",
            ],

            SkillName::Mechanics => &["Grease Monkey (p.51)"],

            SkillName::Medic => &["Verbal Trauma Unit (p.52)"],

            SkillName::Network => &[],

            SkillName::Piloting => &["Gear Devil (NBA p.56)"],

            SkillName::Preparedness => &["Calculated Risk (p.50)"],

            SkillName::SenseTrouble => &["Danger Zone (p.50)", "Perfect Drop (p.53)"],

            SkillName::Shooting => &[
                "Extra Attacks (NBA p.74)",
                "Sniping (NBA p.76)",
                "Special Weapons Training (NBA p.76)",
                "Suppressive Fire (NBA p.77)",
                "Technothriller Monologue (NBA p.77)",
            ],

            SkillName::Shrink => &[],

            SkillName::Surveillance => &[
                "Blending Agent (p.50)",
                "For Your Eyes Only (p.50)",
                "Watching the Watchers (p.83)",
            ],

            SkillName::Weapons => &[
                "Extra Attacks (NBA p.74)",
                "Martial Arts (NBA p.75)",
                "One-Two Punch (p.53)",
                "Special Weapons Training (NBA p.76)",
            ],
        }
    }
}
