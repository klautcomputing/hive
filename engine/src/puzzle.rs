use crate::{
    board::Board, game_error::GameError, game_type::GameType, history::History, piece::Piece,
    position::Position, state::State,
};

pub struct Puzzle {
    pub state: State,
    pub next_moves: Vec<(Piece, Position)>,
    pub length: usize,
    pub rating: usize,
    pub current_step: usize,
}

impl Puzzle {
    pub fn check_step_from_notation(&self, piece: &str, position: &str) -> Result<bool, GameError> {
        let piece = Piece::from_string(piece)?;
        let target_position = Position::from_string(position, &self.state.board)?;
        self.check_step(&piece, &target_position)
    }

    pub fn check_step(&self, piece: &Piece, position: &Position) -> Result<bool, GameError> {
        self.state.play_turn(*piece, *position)?;
        if let Some((piece_next, position_next)) = self.next_moves.get(self.current_step) {
            self.current_step += 1;
            return Ok(piece_next == piece && position_next == position);
        }
        Err(GameError::new_invalid_move(
            piece,
            "NA".to_string(),
            position,
            self.current_step,
            "Ran out of moves".to_string(),
        ))
    }

    pub fn new_from_history(history: &History) -> Self {}

    pub fn new_from_json() {}
}
