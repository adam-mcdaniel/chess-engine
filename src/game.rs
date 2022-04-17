use alloc::string::String;

use crate::board::Board;
use crate::util::{format_fen, parse_fen, parse_san_move};
use crate::{Color, GameResult};

pub enum GameAction {
    // accept draw if previous action was OfferDraw
    AcceptDraw,
    // make move, using san notation
    MakeMove(String),
    // make move and offer draw, using san notiation
    OfferDraw(String),
    // resign
    Resign,
}

// convenience for most common action (MakeMove)
impl From<&str> for GameAction {
    fn from(move_str: &str) -> GameAction {
        GameAction::MakeMove(String::from(move_str))
    }
}

#[derive(Debug, PartialEq)]
pub enum GameError {
    // when san move has multiple options
    AmbiguousMove,
    // no more actions allowed once game is over
    GameAlreadyOver,
    // unable to parse move for current turn
    InvalidMove,
    // unable to parse position
    InvalidPosition,
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
//
// abstractions for two player games, like offering/accepting a draw.
// status is Some when the game is over.
#[derive(Default)]
pub struct Game {
    pub board: Board,
    pub draw_offered: Option<Color>,
    pub status: Option<GameOver>,
}

impl Game {
    pub fn from_fen(
        fen: &str,
        draw_offered: Option<Color>,
        status: Option<GameOver>,
    ) -> Result<Self, GameError> {
        let board = match parse_fen(fen) {
            Ok(board) => board,
            Err(_) => {
                return Err(GameError::InvalidPosition);
            }
        };
        Ok(Game {
            board,
            draw_offered,
            status,
        })
    }

    pub fn to_fen(&self, halfmove_clock: u8, fullmove_number: u8) -> Result<String, String> {
        format_fen(&self.board, halfmove_clock, fullmove_number)
    }

    // convenience accessor for board.get_turn_color
    pub fn get_turn_color(&self) -> Color {
        self.board.get_turn_color()
    }

    // make a move for current turn
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

    // accept draw if previous move was OfferDraw
    fn accept_draw(&mut self) -> Result<&Option<GameOver>, GameError> {
        if let Some(color) = self.draw_offered {
            if color != self.get_turn_color() {
                self.status = Some(GameOver::DrawAccepted);
                return Ok(&self.status);
            }
        }
        Err(GameError::InvalidMove {})
    }

    // move a piece and optionally offer a draw
    fn move_piece(
        &mut self,
        movestr: &str,
        draw_offered: bool,
    ) -> Result<&Option<GameOver>, GameError> {
        let chess_move = match parse_san_move(&self.board, movestr) {
            Ok(chess_move) => chess_move,
            Err(_) => {
                return Err(GameError::InvalidMove {});
            }
        };

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

    // resign
    fn resign(&mut self) -> Result<&Option<GameOver>, GameError> {
        self.status = match self.get_turn_color() {
            Color::Black => Some(GameOver::BlackResigns),
            Color::White => Some(GameOver::WhiteResigns),
        };
        Ok(&self.status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Piece;
    use crate::position::Position;

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

    #[test]
    fn test_promotion() {
        let mut game = Game::default();
        let game_moves = vec![
            "e4", "d5", "exd5", "Nf6", "Bb5", "c6", "dxc6", "Qb6", "cxb7", "Qxb5",
            // bxc8Q is checkmate, but want to test non-queen promotion
            "bxc8R",
        ];
        for game_move in game_moves {
            game.make_move(&GameAction::from(game_move))
                .expect(game_move);
        }
        assert_eq!(game.status, None);
        assert_eq!(
            game.board.get_piece(Position::pgn("c8").unwrap()),
            Some(Piece::Rook(Color::White, Position::pgn("c8").unwrap()))
        )
    }
}
