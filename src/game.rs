use alloc::string::String;

use crate::board::Board;
use crate::piece::Piece;
use crate::position::Position;
use crate::{Color, Evaluate, GameResult, Move};

pub enum GameAction {
    AcceptDraw,
    MakeMove(String),
    OfferDraw(String),
    Resign,
}

impl From<&str> for GameAction {
    fn from(move_str: &str) -> GameAction {
        GameAction::MakeMove(String::from(move_str))
    }
}

#[derive(Debug, PartialEq)]
pub enum GameError {
    AmbiguousMove,
    GameAlreadyOver,
    InvalidMove,
}

#[derive(Debug, PartialEq)]
pub enum GameOver {
    WhiteCheckmates,
    WhiteResigns,
    BlackCheckmates,
    BlackResigns,
    Stalemate,
    DrawAccepted,
}

// wrapper around chess_engine::Board
#[derive(Default)]
pub struct Game {
    pub board: Board,
    pub draw_offered: Option<Color>,
    pub status: Option<GameOver>,
}

impl Game {
    pub fn get_turn_color(&self) -> Color {
        self.board.get_turn_color()
    }

    pub fn make_move(&mut self, action: &GameAction) -> Result<&Option<GameOver>, GameError> {
        if self.status.is_some() {
            return Err(GameError::GameAlreadyOver {});
        }
        match action {
            GameAction::AcceptDraw => self.accept_draw(),
            GameAction::MakeMove(move_str) => self.move_piece(move_str, false),
            GameAction::OfferDraw(move_str) => self.move_piece(move_str, true),
            GameAction::Resign => self.resign(),
        }
    }

    fn accept_draw(&mut self) -> Result<&Option<GameOver>, GameError> {
        if let Some(color) = self.draw_offered {
            if color != self.get_turn_color() {
                self.status = Some(GameOver::DrawAccepted);
                return Ok(&self.status);
            }
        }
        Err(GameError::InvalidMove {})
    }

    fn move_piece(
        &mut self,
        movestr: &str,
        draw_offered: bool,
    ) -> Result<&Option<GameOver>, GameError> {
        let chess_move = parse_san_move(&self.board, movestr)?;

        self.draw_offered = match draw_offered {
            true => Some(self.get_turn_color()),
            false => None,
        };
        self.status = match self.board.play_move(chess_move) {
            GameResult::Continuing(board) => {
                self.board = board;
                None
            }
            GameResult::IllegalMove(_) => {
                return Err(GameError::InvalidMove {});
            }
            GameResult::Stalemate => Some(GameOver::Stalemate),
            GameResult::Victory(color) => match color {
                Color::Black => Some(GameOver::BlackCheckmates),
                Color::White => Some(GameOver::WhiteCheckmates),
            },
        };
        Ok(&self.status)
    }

    fn resign(&mut self) -> Result<&Option<GameOver>, GameError> {
        self.status = match self.get_turn_color() {
            Color::Black => Some(GameOver::BlackResigns),
            Color::White => Some(GameOver::WhiteResigns),
        };
        Ok(&self.status)
    }
}

fn parse_san_move(board: &Board, move_str: &str) -> Result<Move, GameError> {
    if move_str == "0-0" {
        return Ok(Move::KingSideCastle {});
    } else if move_str == "0-0-0" {
        return Ok(Move::QueenSideCastle {});
    }

    // parse in reverse
    let mut chars = move_str.chars();

    // optional pawn promotion
    let mut last = chars.next_back();
    let color = board.get_turn_color();
    let offboard = Position::new(-1, -1);
    let move_promotion = match last {
        Some('Q') => Some(Piece::Queen(color, offboard)),
        Some('K') => Some(Piece::King(color, offboard)),
        Some('N') => Some(Piece::Knight(color, offboard)),
        Some('B') => Some(Piece::Bishop(color, offboard)),
        Some('R') => Some(Piece::Rook(color, offboard)),
        _ => None,
    };
    if move_promotion.is_some() {
        last = chars.next_back();
    }

    // to position
    let to: String = vec![chars.next_back().unwrap_or(' '), last.unwrap_or(' ')]
        .into_iter()
        .collect();
    let move_to = match Position::pgn(&to) {
        Ok(position) => position,
        Err(_) => {
            return Err(GameError::InvalidMove {});
        }
    };

    // chars is now source, with possible 'x' take at end
    let mut source_column = chars.next();

    let piece = match source_column {
        Some('B') => Piece::Bishop(color, offboard),
        Some('K') => Piece::King(color, offboard),
        Some('N') => Piece::Knight(color, offboard),
        Some('Q') => Piece::Queen(color, offboard),
        Some('R') => Piece::Rook(color, offboard),
        _ => Piece::Pawn(color, offboard),
    };
    if piece != Piece::Pawn(color, offboard) {
        // source_column was piece type, next char is source_column
        source_column = chars.next();
    }

    // filter by source column if specified
    let column = match source_column {
        Some('a') => Some(0),
        Some('b') => Some(1),
        Some('c') => Some(2),
        Some('d') => Some(3),
        Some('e') => Some(4),
        Some('f') => Some(5),
        Some('g') => Some(6),
        Some('h') => Some(7),
        // no column specified
        _ => None,
    };

    // if column not specified, may be row
    let mut source_row = source_column;
    if column.is_some() {
        // if column specified, row may be next char
        source_row = chars.next();
    }

    // filter by source row if specified
    let row = match source_row {
        Some('1') => Some(0),
        Some('2') => Some(1),
        Some('3') => Some(2),
        Some('4') => Some(3),
        Some('5') => Some(4),
        Some('6') => Some(5),
        Some('7') => Some(6),
        Some('8') => Some(7),
        _ => None,
    };

    // find moves that end on target square and are correct piece type
    let mut candidates = vec![];
    for legal_move in board.get_legal_moves() {
        if let Move::Piece(from, to) = legal_move {
            if move_to == to {
                if let Some(board_piece) = board.get_piece(from) {
                    // filter based on type
                    let pos = board_piece.get_pos();
                    if board_piece.get_name() == piece.get_name()
                        && (column.is_none() || column == Some(pos.get_col()))
                        && (row.is_none() || row == Some(pos.get_row()))
                    {
                        candidates.push(board_piece);
                    }
                }
            }
        }
    }

    match candidates.len() {
        0 => Err(GameError::InvalidMove {}),
        // todo, use move_promotion
        1 => {
            let move_from = candidates[0].get_pos();
            match move_promotion {
                None => Ok(Move::Piece(move_from, move_to)),
                Some(piece) => Ok(Move::Promotion(move_from, move_to, piece)),
            }
        }
        _ => Err(GameError::AmbiguousMove {}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_parse_san_move() {
        let mut board = Board::default();

        // try valid moves from starting position
        assert_eq!(
            parse_san_move(&board, "d4").expect("d4"),
            Move::Piece(D2, D4)
        );
        assert_eq!(
            parse_san_move(&board, "Nc3").expect("Nc3"),
            Move::Piece(B1, C3)
        );
        // not valid first move for white
        assert_eq!(
            parse_san_move(&board, "d5").expect_err("d5"),
            GameError::InvalidMove {}
        );

        // make first move
        board = match board.play_move(Move::Piece(E2, E4)) {
            GameResult::Continuing(board) => board,
            e => panic!("unexpected error: {:?}", e),
        };
        // valid first move for black
        assert_eq!(
            parse_san_move(&board, "d5").expect("d5"),
            Move::Piece(D7, D5)
        );
        // white moves not valid for black
        assert_eq!(
            parse_san_move(&board, "c4").expect_err("c4"),
            GameError::InvalidMove {}
        );
    }

    #[test]
    fn test_game_moves() {
        let mut game = Game::default();
        let game_moves = vec!["d4", "d5", "c4", "dxc4", "e3", "Nf6", "Bxc4"];
        for game_move in game_moves {
            game.make_move(&GameAction::from(game_move))
                .expect(game_move);
        }
        assert_eq!(game.status, None);
    }

    #[test]
    fn test_fools_mate() {
        let mut game = Game::default();
        let game_moves = vec!["f3", "e5", "g4", "Qh4"];
        for game_move in game_moves {
            game.make_move(&GameAction::from(game_move))
                .expect(game_move);
        }
        assert_eq!(game.status, Some(GameOver::BlackCheckmates));
    }
}
