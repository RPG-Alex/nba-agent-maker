use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,  Default,Clone, Copy, Debug, PartialEq)]
pub enum GameMode {
    #[default]
    None,
    Burn,
    Dust,
    Mirror,
    Stakes,
}
