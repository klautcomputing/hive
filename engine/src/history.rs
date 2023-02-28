use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};

use crate::color::Color;
use crate::game_error::GameError;
use crate::game_result::GameResult;
use crate::game_type::GameType;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq)]
pub struct History {
    pub moves: Vec<(String, String)>,
    pub result: GameResult,
    pub game_type: GameType,
}

impl History {
    pub fn new() -> Self {
        History {
            moves: Vec::new(),
            result: GameResult::Unknown,
            game_type: GameType::default(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut his = String::new();
        for (i, (piece, pos)) in self.moves.iter().enumerate() {
            his += &format!("{}. {} {}", i + 1, piece, pos);
        }
        his
    }

    pub fn record_move(&mut self, piece: String, pos: String) {
        self.moves.push((piece, pos));
    }

    pub fn from_filepath(file_path: &str) -> Result<Self, GameError> {
        let mut history = History::new();
        let header = Regex::new(r"\[.*").expect("This regex should compile");
        let turn = Regex::new(r"\d+").expect("This regex should compile");
        let result = Regex::new(r"\[Result").expect("This regex should compile");
        let game_type_line = Regex::new(r"\[GameType.*").expect("This regex should compile");
        let game_type =
            Regex::new(r#"\[GameType "(Base[+MLP]?)"\]"#).expect("This regex should compile");
        match File::open(file_path) {
            Ok(file) => {
                for line in io::BufReader::new(file).lines().flatten() {
                    let tokens = line.split_whitespace().collect::<Vec<&str>>();
                    if line.len() == 0 {
                        continue;
                    }
                    if game_type_line.is_match(&line) {
                        if let Some(caps) = game_type.captures(&line) {
                            if let Some(mtch) = caps.get(1) {
                                history.game_type = GameType::from_str(mtch.as_str())?;
                            }
                        } else {
                            return Err(GameError::ParsingError {
                                found: line.to_string(),
                                typ: "game string".to_string(),
                            });
                        }
                    }
                    if let Some(token) = tokens.first() {
                        if result.is_match(token) {
                            match tokens.get(1) {
                                Some(&"\"1-0\"]") => {
                                    history.result = GameResult::Winner(Color::White)
                                }
                                Some(&"\"0-1\"]") => {
                                    history.result = GameResult::Winner(Color::Black)
                                }
                                Some(&"\"1/2-1/2\"]") => history.result = GameResult::Draw,
                                _ => history.result = GameResult::Unknown,
                            }
                        }
                        if header.is_match(token) {
                            continue;
                        }
                        if turn.is_match(token) {
                            if let Some(piece) = tokens.get(1) {
                                if let Some(position) = tokens.get(2) {
                                    history
                                        .moves
                                        .push((piece.to_string(), position.to_string()));
                                } else {
                                    match *piece {
                                        "pass" => {
                                            history
                                                .moves
                                                .push(("pass".to_string(), "".to_string()));
                                        }
                                        _ if history.moves.is_empty() => {
                                            history
                                                .moves
                                                .push((piece.to_string(), ".".to_string()));
                                        }
                                        any => return Err(GameError::ParsingError { found: any.to_owned(), typ: format!("move, in history on turn {}", token)}),
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Couldn't open file because: {}", e);
            }
        }
        Ok(history)
    }

    pub fn write_move(&self, file_name: &str, turn: usize, board_move: String) {
        let mut file = OpenOptions::new()
            .append(true)
            .open(file_name)
            .expect("game.txt cannot be written to");
        if let Err(e) = write!(file, "{}. {}\n", turn, board_move) {
            //TODO not sure what to do with this one
            panic!("{}", e);
        }
    }

    pub fn write_file(&self, file_name: String) {
        // TODO rewrite this to not open the file for every single line
        for (i, (piece, pos)) in self.moves.iter().enumerate() {
            let mov = format!("{} {}", piece, pos);
            self.write_move(&file_name, i, mov);
        }
    }
}
