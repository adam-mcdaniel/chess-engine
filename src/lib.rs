// #![no_std]
#[macro_use]
extern crate alloc;

mod board;
pub use board::Board;

mod square;
pub use square::{Square, EMPTY_SQUARE};

mod piece;
pub use piece::Piece;

mod position;
pub use position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::White => "White",
                Self::Black => "Black",
            }
        )
    }
}

impl core::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Move {
    QueenSideCastle,
    KingSideCastle,
    Piece(Position, Position),
}


impl core::fmt::Display for Move {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            Move::Piece(from, to) => write!(f, "{} to {}", from, to),
            Move::KingSideCastle => write!(f, "O-O"),
            Move::QueenSideCastle => write!(f, "O-O-O"),
        }
    }
}

pub trait Evaluate: Sized {
    fn material_value_for(&self, color: Color) -> f64;

    fn get_current_player_color(&self) -> Color;

    fn get_legal_moves(&self) -> Vec<Move>;

    fn play_move(&self, m: Move) -> Self;

    fn get_best_next_move(&self, depth: i32, board_limit: i32) -> Move {
        let legal_moves = self.get_legal_moves();
        let mut best_move_value = -9999.0;
        let mut best_move = legal_moves[0];

        let color = self.get_current_player_color();
        
        let mut board_count = 0;
        for m in &legal_moves {
            let child_board_value = self.play_move(*m).minimax(depth, -10000.0, 10000.0, false, color, &mut board_count, board_limit);
            if child_board_value >= best_move_value {
                best_move = *m;
                best_move_value = child_board_value;
            }
        }

        println!("evaluated {} boards", board_count);
        best_move
    }

    fn get_worst_next_move(&self, depth: i32, board_limit: i32) -> Move {
        let legal_moves = self.get_legal_moves();
        let mut best_move_value = -9999.0;
        let mut best_move = legal_moves[0];
        
        let color = self.get_current_player_color();
        
        let mut board_count = 0;
        for m in &legal_moves {
            let child_board_value = self.play_move(*m).minimax(depth, -10000.0, 10000.0, true, color, &mut board_count, board_limit);
            if child_board_value >= best_move_value {
                best_move = *m;
                best_move_value = child_board_value;
            }
        }

        best_move
    }

    fn minimax(&self, depth: i32, mut alpha: f64, mut beta: f64, is_maximizing: bool, getting_move_for: Color, board_count: &mut i32, board_limit: i32) -> f64 {
        *board_count += 1;
        let legal_moves = self.get_legal_moves();
        if depth == 0 || *board_count >= board_limit {
            return self.material_value_for(getting_move_for);
        }

        let mut best_move_value;

        if is_maximizing {
            best_move_value = -9999.0;

            for m in &legal_moves {
                let child_board_value =
                    self.play_move(*m)
                        .minimax(depth - 1, alpha, beta, !is_maximizing, getting_move_for, board_count, board_limit);
                if child_board_value > best_move_value {
                    best_move_value = child_board_value;
                }

                if best_move_value > alpha {
                    alpha = best_move_value
                }

                if beta <= alpha {
                    return best_move_value;
                }
            }
        } else {
            best_move_value = 9999.0;

            for m in &legal_moves {
                let child_board_value =
                    self.play_move(*m)
                        .minimax(depth - 1, alpha, beta, !is_maximizing, getting_move_for, board_count, board_limit);
                if child_board_value < best_move_value {
                    best_move_value = child_board_value;
                }

                if best_move_value < beta {
                    beta = best_move_value
                }

                if beta <= alpha {
                    return best_move_value;
                }
            }
        }

        best_move_value
    }
}
