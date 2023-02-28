use crate::game_result::GameResult;

#[derive(thiserror::Error, Debug)]
pub enum GameError {
    #[error("Invalid or illegal move on turn {turn}, moving piece {piece} from {from} to {to} because {reason}")]
    InvalidMove {
        piece: String,
        from: String,
        to: String,
        turn: usize,
        reason: String,
    },
    #[error("No piece found at position {position}")]
    NoPieceAtPosition { position: String },
    #[error(
        "Invalid spawn of piece {piece} at position {position} on turn {turn} because {reason}"
    )]
    InvalidSpawn {
        piece: String,
        position: String,
        turn: usize,
        reason: String,
    },
    #[error("Found {found:?} which is not a valid {typ}")]
    ParsingError { found: String, typ: String },
    #[error("Result {reported_result:?} doesn't match board endstate {actual_result:?}")]
    ResultMismatch {
        reported_result: GameResult,
        actual_result: GameResult,
    },
    #[error("No .pgn file supplied")]
    NoPgnFile,
    #[error("Invalid direction {direction:?}")]
    InvalidDirection { direction: String },
}

use crate::game_error::GameError::InvalidMove;
use crate::piece::Piece;
use crate::position::{Position, self};
impl GameError {
    pub fn new_invalid_move(
        piece: Piece,
        from: Position,
        to: Position,
        turn: usize,
        reason: String,
    ) -> GameError {
        InvalidMove {
            piece: piece.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            turn,
            reason,
        }
    }
}
