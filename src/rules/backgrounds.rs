
pub(crate) struct Background {
    name: BackGroundName,
    description: String,
    investigative_point_buys: String,
    general_point_buys: String,
}

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
    WireRate,
    Custom(String)
}

pub(crate) struct Backgrounds {
    backgrounds: Vec<Background>
}