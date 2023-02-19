use thiserror::Error;

#[derive(Error,Debug)]
pub enum GameErrors {
    #[error("Invalid or illegal move")]
    InvalidMove,
    #[error("Invalid game type")]
    InvalidGameType,
    #[error("Missing result")]
    MissingResult,
}
