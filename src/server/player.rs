use serde::{Serialize, Deserialize};

use crate::game::Move;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub what: String,
}

impl Player {
    pub fn from_str(input: &str) -> Move {
        match input.trim() {
            "rock" => Move::Rock,
            "paper" => Move::Paper,
            "scissors" => Move::Scissors,
            _ => return Move::None,
        }
//        Some(Self { what: what })
    }
}