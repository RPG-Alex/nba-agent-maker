use serde::{Deserialize, Serialize};

use self::InvestigativeAbilityName::Intimidation;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub(crate) struct InvestigativeAbility {
    pub(crate) name: InvestigativeAbilityName,
    pub(crate) rating: i32,
}

impl InvestigativeAbility {
    pub(crate) fn new(name: InvestigativeAbilityName) -> Self {
        Self {
            name: name,
            rating: 0,
        }
    }
    pub(crate) fn freepoint(name: InvestigativeAbilityName) -> Self {
        Self {
            name: name,
            rating: 1,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub(crate) struct LanguagesAbility {
    pub(crate) ability: InvestigativeAbility,
    pub(crate) known_languages: Vec<String>,
}

impl LanguagesAbility {
    pub(crate) fn new() -> Self {
        Self {
            ability: InvestigativeAbility::new(InvestigativeAbilityName::Languages),
            known_languages: vec![],
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub(crate) struct InvestigativeAbilities {
    pub(crate) accounting: InvestigativeAbility,
    pub(crate) archeology: InvestigativeAbility,
    pub(crate) architecture: InvestigativeAbility,
    pub(crate) art_history: InvestigativeAbility,
    pub(crate) astronomy: InvestigativeAbility,
    pub(crate) bullshit_detector: InvestigativeAbility,
    pub(crate) bureaucracy: InvestigativeAbility,
    pub(crate) chemistry: InvestigativeAbility,
    pub(crate) cop_talk: InvestigativeAbility,
    pub(crate) criminology: InvestigativeAbility,
    pub(crate) cryptography: InvestigativeAbility,
    pub(crate) data_recovery: InvestigativeAbility,
    pub(crate) diagnosis: InvestigativeAbility,
    pub(crate) electronic_surveillance: InvestigativeAbility,
    pub(crate) flattery: InvestigativeAbility,
    pub(crate) flirting: InvestigativeAbility,
    pub(crate) forensic_pathology: InvestigativeAbility,
    pub(crate) forgery: InvestigativeAbility,
    pub(crate) high_society: InvestigativeAbility,
    pub(crate) history: InvestigativeAbility,
    pub(crate) human_terrain: InvestigativeAbility,
    pub(crate) interrogation: InvestigativeAbility,
    pub(crate) intimidation: InvestigativeAbility,
    pub(crate) languages: LanguagesAbility,
    pub(crate) law: InvestigativeAbility,
    pub(crate) military_science: InvestigativeAbility,
    pub(crate) negotiation: InvestigativeAbility,
    pub(crate) notice: InvestigativeAbility,
    pub(crate) occult_studies: InvestigativeAbility,
    pub(crate) outdoor_survival: InvestigativeAbility,
    pub(crate) pharmacy: InvestigativeAbility,
    pub(crate) photography: InvestigativeAbility,
    pub(crate) reassurance: InvestigativeAbility,
    pub(crate) research: InvestigativeAbility,
    pub(crate) streetwise: InvestigativeAbility,
    pub(crate) surveillance: InvestigativeAbility,
    pub(crate) tradecraft: InvestigativeAbility,
    pub(crate) traffic_analysis: InvestigativeAbility,
    pub(crate) urban_survival: InvestigativeAbility,
}

impl InvestigativeAbilities {
    pub(crate) fn new() -> Self {
        Self {
            accounting: InvestigativeAbility::new(InvestigativeAbilityName::Accounting),
            archeology: InvestigativeAbility::new(InvestigativeAbilityName::Archeology),
            architecture: InvestigativeAbility::new(InvestigativeAbilityName::Architecture),
            art_history: InvestigativeAbility::new(InvestigativeAbilityName::ArtHistory),
            astronomy: InvestigativeAbility::new(InvestigativeAbilityName::Astronomy),
            bullshit_detector: InvestigativeAbility::new(InvestigativeAbilityName::BullshitDetector),
            bureaucracy: InvestigativeAbility::new(InvestigativeAbilityName::Bureaucracy),
            chemistry: InvestigativeAbility::new(InvestigativeAbilityName::Chemistry),
            cop_talk: InvestigativeAbility::new(InvestigativeAbilityName::CopTalk),
            criminology: InvestigativeAbility::new(InvestigativeAbilityName::Criminology),
            cryptography: InvestigativeAbility::new(InvestigativeAbilityName::Cryptography),
            data_recovery: InvestigativeAbility::new(InvestigativeAbilityName::DataRecovery),
            diagnosis: InvestigativeAbility::new(InvestigativeAbilityName::Diagnosis),
            electronic_surveillance: InvestigativeAbility::new(InvestigativeAbilityName::ElectronicSurveillance),
            flattery: InvestigativeAbility::new(InvestigativeAbilityName::Flattery),
            flirting: InvestigativeAbility::new(InvestigativeAbilityName::Flirting),
            forensic_pathology: InvestigativeAbility::new(InvestigativeAbilityName::ForensicPathology),
            forgery: InvestigativeAbility::new(InvestigativeAbilityName::Forgery),
            high_society: InvestigativeAbility::new(InvestigativeAbilityName::HighSociety),
            history: InvestigativeAbility::new(InvestigativeAbilityName::History),
            human_terrain: InvestigativeAbility::new(InvestigativeAbilityName::HumanTerrain),
            interrogation: InvestigativeAbility::new(InvestigativeAbilityName::Interrogation),
            intimidation: InvestigativeAbility::new(Intimidation),
            languages: LanguagesAbility::new(),
            law: InvestigativeAbility::new(InvestigativeAbilityName::Law),
            military_science: InvestigativeAbility::new(InvestigativeAbilityName::MilitaryScience),
            negotiation: InvestigativeAbility::new(InvestigativeAbilityName::Negotiation),
            notice: InvestigativeAbility::new(InvestigativeAbilityName::Notice),
            occult_studies: InvestigativeAbility::new(InvestigativeAbilityName::OccultStudies),
            outdoor_survival: InvestigativeAbility::new(InvestigativeAbilityName::OutdoorSurvival),
            pharmacy: InvestigativeAbility::new(InvestigativeAbilityName::Pharmacy),
            photography: InvestigativeAbility::new(InvestigativeAbilityName::Photography),
            reassurance: InvestigativeAbility::new(InvestigativeAbilityName::Reassurance),
            research: InvestigativeAbility::new(InvestigativeAbilityName::Research),
            streetwise: InvestigativeAbility::freepoint(InvestigativeAbilityName::Streetwise),
            surveillance: InvestigativeAbility::new(InvestigativeAbilityName::Surveillance),
            tradecraft: InvestigativeAbility::freepoint(InvestigativeAbilityName::TradeCraft),
            traffic_analysis: InvestigativeAbility::new(InvestigativeAbilityName::TrafficAnalysis),
            urban_survival: InvestigativeAbility::new(InvestigativeAbilityName::UrbanSurvival),
        }
    }
}



#[derive(Clone, Deserialize, Serialize, Debug, Eq, PartialEq)]
pub(crate) enum InvestigativeAbilityName {
    Accounting,
    Archeology,
    Architecture,
    ArtHistory,
    Astronomy,
    BullshitDetector,
    Bureaucracy,
    Chemistry,
    CopTalk,
    Criminology,
    Cryptography,
    DataRecovery,
    Diagnosis,
    ElectronicSurveillance,
    Flattery,
    Flirting,
    ForensicPathology,
    Forgery,
    HighSociety,
    History,
    HumanTerrain,
    Interrogation,
    Intimidation,
    Languages,
    Law,
    MilitaryScience,
    Negotiation,
    Notice,
    OccultStudies,
    OutdoorSurvival,
    Pharmacy,
    Photography,
    Reassurance,
    Research,
    Streetwise,
    Surveillance,
    TradeCraft,
    TrafficAnalysis,
    UrbanSurvival,
}

impl InvestigativeAbilityName {
    pub(crate) fn abilities() -> Vec<InvestigativeAbilityName> {
            vec![
            InvestigativeAbilityName::Accounting,
            InvestigativeAbilityName::Archeology,
            InvestigativeAbilityName::Architecture,
            InvestigativeAbilityName::ArtHistory,
            InvestigativeAbilityName::Astronomy,
            InvestigativeAbilityName::BullshitDetector,
            InvestigativeAbilityName::Bureaucracy,
            InvestigativeAbilityName::Chemistry,
            InvestigativeAbilityName::CopTalk,
            InvestigativeAbilityName::Criminology,
            InvestigativeAbilityName::Cryptography,
            InvestigativeAbilityName::DataRecovery,
            InvestigativeAbilityName::Diagnosis,
            InvestigativeAbilityName::ElectronicSurveillance,
            InvestigativeAbilityName::Flattery,
            InvestigativeAbilityName::Flirting,
            InvestigativeAbilityName::ForensicPathology,
            InvestigativeAbilityName::Forgery,
            InvestigativeAbilityName::HighSociety,
            InvestigativeAbilityName::History,
            InvestigativeAbilityName::HumanTerrain,
            InvestigativeAbilityName::Interrogation,
            InvestigativeAbilityName::Intimidation,
            InvestigativeAbilityName::Languages,
            InvestigativeAbilityName::Law,
            InvestigativeAbilityName::MilitaryScience,
            InvestigativeAbilityName::Negotiation,
            InvestigativeAbilityName::Notice,
            InvestigativeAbilityName::OccultStudies,
            InvestigativeAbilityName::OutdoorSurvival,
            InvestigativeAbilityName::Pharmacy,
            InvestigativeAbilityName::Photography,
            InvestigativeAbilityName::Reassurance,
            InvestigativeAbilityName::Research,
            InvestigativeAbilityName::Streetwise,
            InvestigativeAbilityName::Surveillance,
            InvestigativeAbilityName::TradeCraft,
            InvestigativeAbilityName::TrafficAnalysis,
            InvestigativeAbilityName::UrbanSurvival,]
    }
}

impl core::fmt::Display for InvestigativeAbilityName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InvestigativeAbilityName::Accounting => write!(f, "Accounting"),
            InvestigativeAbilityName::Archeology => write!(f, "Archeology"),
            InvestigativeAbilityName::Architecture => write!(f, "Architecture"),
            InvestigativeAbilityName::ArtHistory => write!(f, "Art History"),
            InvestigativeAbilityName::Astronomy => write!(f, "Astronomy"),
            InvestigativeAbilityName::BullshitDetector => write!(f, "Bullshit Detector"),
            InvestigativeAbilityName::Bureaucracy => write!(f, "Bureaucracy"),
            InvestigativeAbilityName::Chemistry => write!(f, "Chemistry"),
            InvestigativeAbilityName::CopTalk => write!(f, "Cop Talk"),
            InvestigativeAbilityName::Criminology => write!(f, "Criminology"),
            InvestigativeAbilityName::Cryptography => write!(f, "Cryptography"),
            InvestigativeAbilityName::DataRecovery => write!(f, "Data Recovery"),
            InvestigativeAbilityName::Diagnosis => write!(f, "Diagnosis"),
            InvestigativeAbilityName::ElectronicSurveillance => write!(f, "Electronic Survival"),
            InvestigativeAbilityName::Flattery => write!(f, "Flattery"),
            InvestigativeAbilityName::Flirting => write!(f, "Flirting"),
            InvestigativeAbilityName::ForensicPathology => write!(f, "Forensic Pathology"),
            InvestigativeAbilityName::Forgery => write!(f, "Forgery"),
            InvestigativeAbilityName::HighSociety => write!(f, "High Society"),
            InvestigativeAbilityName::History => write!(f, "History"),
            InvestigativeAbilityName::HumanTerrain => write!(f, "Human Terrain"),
            InvestigativeAbilityName::Interrogation => write!(f, "Interrogation"),
            InvestigativeAbilityName::Intimidation => write!(f, "Intimidation"),
            InvestigativeAbilityName::Languages => write!(f, "Languages"),
            InvestigativeAbilityName::Law => write!(f, "Law"),
            InvestigativeAbilityName::MilitaryScience => write!(f, "Military Science"),
            InvestigativeAbilityName::Negotiation => write!(f, "Negotiation"),
            InvestigativeAbilityName::Notice => write!(f, "Notice"),
            InvestigativeAbilityName::OccultStudies => write!(f, "Occult Studies"),
            InvestigativeAbilityName::OutdoorSurvival => write!(f, "Outdoor Survival"),
            InvestigativeAbilityName::Pharmacy => write!(f, "Pharmacy"),
            InvestigativeAbilityName::Photography => write!(f, "Photography"),
            InvestigativeAbilityName::Reassurance => write!(f, "Reassurance"),
            InvestigativeAbilityName::Research => write!(f, "Research"),
            InvestigativeAbilityName::Streetwise => write!(f, "Streetwise"),
            InvestigativeAbilityName::Surveillance => write!(f, "Surveillance"),
            InvestigativeAbilityName::TradeCraft => write!(f, "Trade Craft"),
            InvestigativeAbilityName::TrafficAnalysis => write!(f, "Traffic Analysis"),
            InvestigativeAbilityName::UrbanSurvival => write!(f, "Urban Survival"),
        }
    }
}
