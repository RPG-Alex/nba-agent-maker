use reactive_stores::Store;

#[derive(Clone, PartialEq, Store)]
pub struct Character {
    // Agent Info (Personality and Dossier)
    pub agent_name: String,
    pub mos: String,
    pub drive: String,
    pub handler: String,
    pub professional_role: String,
    pub backgrounds: Vec<String>,
    pub symbol: String,
    pub solace: String,
    pub safety: String,
    pub languages_spoken: Vec<String>,
    pub health: i32,
    pub stability: i32,
    pub heat_level: i32,
    pub build_points: i32,
    pub game_modes: Vec<String>,

    // General Abilities
    pub athletics: i32,
    pub conceal: i32,
    pub cover: i32,
    pub digital_intrusion: i32,
    pub disguise: i32,
    pub driving: i32,
    pub explosive_devices: i32,
    pub filch: i32,
    pub gambling: i32,
    pub hand_to_hand: i32,
    pub infiltration: i32,
    pub mechanics: i32,
    pub medic: i32,
    pub network: i32,
    pub piloting: i32,
    pub preparedness: i32,
    pub sense_trouble: i32,
    pub shooting: i32,
    pub shrink: i32,
    pub surveillance: i32,
    pub weapons: i32,

    // Cherries
    pub cherries: Vec<String>,

    // Academic (Investigative) Abilities
    pub accounting: i32,
    pub archaeology: i32,
    pub architecture: i32,
    pub art_history: i32,
    pub criminology: i32,
    pub diagnosis: i32,
    pub history: i32,
    pub human_terrain: i32,
    pub academic_languages: i32,
    pub law: i32,
    pub military_science: i32,
    pub occult_studies: i32,
    pub research: i32,
    pub vampirology: i32,

    // Interpersonal (Investigative) Abilities
    pub bs_detector: i32,
    pub bureaucracy: i32,
    pub cop_talk: i32,
    pub flattery: i32,
    pub flirting: i32,
    pub high_society: i32,
    pub interrogation: i32,
    pub intimidation: i32,
    pub negotiation: i32,
    pub reassurance: i32,
    pub streetwise: i32,
    pub tradecraft: i32,

    // Technical (Investigative) Abilities
    pub astronomy: i32,
    pub chemistry: i32,
    pub cryptography: i32,
    pub data_recovery: i32,
    pub electronic_surveillance: i32,
    pub forensic_pathology: i32,
    pub forgery: i32,
    pub notice: i32,
    pub outdoor_survival: i32,
    pub pharmacy: i32,
    pub photography: i32,
    pub traffic_analysis: i32,
    pub urban_survival: i32,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            // Agent Info (Personality and Dossier)
            agent_name: String::new(),
            mos: String::new(),
            drive: String::new(),
            handler: String::new(),
            professional_role: String::new(),
            backgrounds: Vec::new(),
            symbol: String::new(),
            solace: String::new(),
            safety: String::new(),
            languages_spoken: Vec::new(),
            health: 4,
            stability: 4,
            heat_level: 0,
            build_points: 0,
            game_modes: Vec::new(),

            // General Abilities
            athletics: 0,
            conceal: 0,
            cover: 10,
            digital_intrusion: 0,
            disguise: 0,
            driving: 0,
            explosive_devices: 0,
            filch: 0,
            gambling: 0,
            hand_to_hand: 0,
            infiltration: 0,
            mechanics: 0,
            medic: 0,
            network: 15,
            piloting: 0,
            preparedness: 0,
            sense_trouble: 0,
            shooting: 0,
            shrink: 0,
            surveillance: 0,
            weapons: 0,

            cherries: Vec::new(),

            // Academic (Investigative) Abilities
            accounting: 0,
            archaeology: 0,
            architecture: 0,
            art_history: 0,
            criminology: 0,
            diagnosis: 0,
            history: 0,
            human_terrain: 0,
            academic_languages: 0,
            law: 0,
            military_science: 0,
            occult_studies: 0,
            research: 0,
            vampirology: 0,

            // Interpersonal (Investigative) Abilities
            bs_detector: 0,
            bureaucracy: 0,
            cop_talk: 0,
            flattery: 0,
            flirting: 0,
            high_society: 0,
            interrogation: 0,
            intimidation: 0,
            negotiation: 0,
            reassurance: 0,
            streetwise: 1,
            tradecraft: 1,

            // Technical (Investigative) Abilities
            astronomy: 0,
            chemistry: 0,
            cryptography: 0,
            data_recovery: 0,
            electronic_surveillance: 0,
            forensic_pathology: 0,
            forgery: 0,
            notice: 0,
            outdoor_survival: 0,
            pharmacy: 0,
            photography: 0,
            traffic_analysis: 0,
            urban_survival: 0,
        }
    }
}

pub struct Mos {
    pub mos: String,
}
