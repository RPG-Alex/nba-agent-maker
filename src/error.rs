use thiserror::Error;

#[derive(Error,Debug)]
pub enum NbaError {
    #[error("Failed to parse background: {0}")]
    Background(String)
}