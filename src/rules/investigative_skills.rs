use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct InvestigativeAbility {
    pub name: String,
    pub rating: i32,
}

impl InvestigativeAbility {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rating: 0,
        }
    }
    pub fn freepoint(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rating: 1,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct LanguagesAbility {
    pub ability: InvestigativeAbility,
    pub known_languages: Vec<String>,
}

impl LanguagesAbility {
    pub fn new() -> Self {
        Self {
            ability: InvestigativeAbility::new("Languages"),
            known_languages: vec![],
        }
    }

    pub fn add_language(&mut self, language: &str) {
        self.known_languages.push(language.to_string());
    }

    pub fn remove_language(&mut self, language: &str) {
        self.known_languages.retain(|l| l != language);
    }

    pub fn set_languages(&mut self, languages: Vec<String>) {
        self.known_languages = languages;
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct InvestigativeAbilities {
    pub accounting: InvestigativeAbility,
    pub archeology: InvestigativeAbility,
    pub architecture: InvestigativeAbility,
    pub art_history: InvestigativeAbility,
    pub astronomy: InvestigativeAbility,
    pub bullshit_detector: InvestigativeAbility,
    pub bureaucracy: InvestigativeAbility,
    pub chemistry: InvestigativeAbility,
    pub cop_talk: InvestigativeAbility,
    pub criminology: InvestigativeAbility,
    pub cryptography: InvestigativeAbility,
    pub data_recovery: InvestigativeAbility,
    pub diagnosis: InvestigativeAbility,
    pub electronic_surveillance: InvestigativeAbility,
    pub flattery: InvestigativeAbility,
    pub flirting: InvestigativeAbility,
    pub forensic_pathology: InvestigativeAbility,
    pub forgery: InvestigativeAbility,
    pub high_society: InvestigativeAbility,
    pub history: InvestigativeAbility,
    pub human_terrain: InvestigativeAbility,
    pub interrogation: InvestigativeAbility,
    pub intimidation: InvestigativeAbility,
    pub languages: LanguagesAbility,
    pub law: InvestigativeAbility,
    pub military_science: InvestigativeAbility,
    pub negotiation: InvestigativeAbility,
    pub notice: InvestigativeAbility,
    pub occult_studies: InvestigativeAbility,
    pub outdoor_survival: InvestigativeAbility,
    pub pharmacy: InvestigativeAbility,
    pub photography: InvestigativeAbility,
    pub reassurance: InvestigativeAbility,
    pub research: InvestigativeAbility,
    pub streetwise: InvestigativeAbility,
    pub surveillance: InvestigativeAbility,
    pub tradecraft: InvestigativeAbility,
    pub traffic_analysis: InvestigativeAbility,
    pub urban_survival: InvestigativeAbility,
}

impl InvestigativeAbilities {
    pub fn new() -> Self {
        Self {
            accounting: InvestigativeAbility::new("Accounting"),
            archeology: InvestigativeAbility::new("Archeology"),
            architecture: InvestigativeAbility::new("Architecture"),
            art_history: InvestigativeAbility::new("Art History"),
            astronomy: InvestigativeAbility::new("Astronomy"),
            bullshit_detector: InvestigativeAbility::new("Bullshit Detector"),
            bureaucracy: InvestigativeAbility::new("Bureaucracy"),
            chemistry: InvestigativeAbility::new("Chemistry"),
            cop_talk: InvestigativeAbility::new("Cop Talk"),
            criminology: InvestigativeAbility::new("Criminology"),
            cryptography: InvestigativeAbility::new("Cryptography"),
            data_recovery: InvestigativeAbility::new("Data Recovery"),
            diagnosis: InvestigativeAbility::new("Diagnosis"),
            electronic_surveillance: InvestigativeAbility::new("Electronic Surveillance"),
            flattery: InvestigativeAbility::new("Flattery"),
            flirting: InvestigativeAbility::new("Flirting"),
            forensic_pathology: InvestigativeAbility::new("Forensic Pathology"),
            forgery: InvestigativeAbility::new("Forgery"),
            high_society: InvestigativeAbility::new("High Society"),
            history: InvestigativeAbility::new("History"),
            human_terrain: InvestigativeAbility::new("Human Terrain"),
            interrogation: InvestigativeAbility::new("Interrogation"),
            intimidation: InvestigativeAbility::new("Intimidation"),
            languages: LanguagesAbility::new(),
            law: InvestigativeAbility::new("Law"),
            military_science: InvestigativeAbility::new("Military Science"),
            negotiation: InvestigativeAbility::new("Negotiation"),
            notice: InvestigativeAbility::new("Notice"),
            occult_studies: InvestigativeAbility::new("Occult Studies"),
            outdoor_survival: InvestigativeAbility::new("Outdoor Survival"),
            pharmacy: InvestigativeAbility::new("Pharmacy"),
            photography: InvestigativeAbility::new("Photography"),
            reassurance: InvestigativeAbility::new("Reassurance"),
            research: InvestigativeAbility::new("Research"),
            streetwise: InvestigativeAbility::freepoint("Streetwise"),
            surveillance: InvestigativeAbility::new("Surveillance"),
            tradecraft: InvestigativeAbility::freepoint("Tradecraft"),
            traffic_analysis: InvestigativeAbility::new("Traffic Analysis"),
            urban_survival: InvestigativeAbility::new("Urban Survival"),
        }
    }
}
