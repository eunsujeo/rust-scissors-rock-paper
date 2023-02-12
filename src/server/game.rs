#[derive(Debug)]
pub enum Move {
    Rock,
    Scissors,
    Paper,
    None,
}

pub struct Game {
    pub player1: Move,
    pub player2: Move,
}

impl Game {

    pub fn winner(p1: Move, p2: Move) -> Option<Move> {
        match (p1, p2) {
            (Move::Rock, Move::Scissors) => Some(Move::Rock),
            (Move::Scissors, Move::Paper) => Some(Move::Scissors),
            (Move::Paper, Move::Rock) => {
                let player1 = Move::Paper;
                Some(player1)
            },
            (_, _) => None,
        }
    }
}
