use super::{Evaluate, Color, Move, Piece, Position, Square, EMPTY_SQUARE};
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CastlingRights {
    kingside: bool,
    queenside: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            kingside: true,
            queenside: true,
        }
    }
}

impl CastlingRights {
    fn can_kingside_castle(&self) -> bool {
        self.kingside
    }

    fn can_queenside_castle(&self) -> bool {
        self.queenside
    }

    fn no_kingside(&mut self) {
        self.kingside = false
    }

    fn no_queenside(&mut self) {
        self.queenside = false
    }

    fn invalidate(&mut self) {
        self.no_kingside();
        self.no_queenside()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            squares: [
                Square::from(Piece::Rook(Color::Black, Position::new(7, 0))),
                Square::from(Piece::Knight(Color::Black, Position::new(7, 1))),
                Square::from(Piece::Bishop(Color::Black, Position::new(7, 2))),
                Square::from(Piece::Queen(Color::Black, Position::new(7, 3))),
                Square::from(Piece::King(Color::Black, Position::new(7, 4))),
                Square::from(Piece::Bishop(Color::Black, Position::new(7, 5))),
                Square::from(Piece::Knight(Color::Black, Position::new(7, 6))),
                Square::from(Piece::Rook(Color::Black, Position::new(7, 7))),
                Square::from(Piece::Pawn(Color::Black, Position::new(6, 0))),
                Square::from(Piece::Pawn(Color::Black, Position::new(6, 1))),
                Square::from(Piece::Pawn(Color::Black, Position::new(6, 2))),
                Square::from(Piece::Pawn(Color::Black, Position::new(6, 3))),
                Square::from(Piece::Pawn(Color::Black, Position::new(6, 4))),
                Square::from(Piece::Pawn(Color::Black, Position::new(6, 5))),
                Square::from(Piece::Pawn(Color::Black, Position::new(6, 6))),
                Square::from(Piece::Pawn(Color::Black, Position::new(6, 7))),
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                EMPTY_SQUARE,
                Square::from(Piece::Pawn(Color::White, Position::new(1, 0))),
                Square::from(Piece::Pawn(Color::White, Position::new(1, 1))),
                Square::from(Piece::Pawn(Color::White, Position::new(1, 2))),
                Square::from(Piece::Pawn(Color::White, Position::new(1, 3))),
                Square::from(Piece::Pawn(Color::White, Position::new(1, 4))),
                Square::from(Piece::Pawn(Color::White, Position::new(1, 5))),
                Square::from(Piece::Pawn(Color::White, Position::new(1, 6))),
                Square::from(Piece::Pawn(Color::White, Position::new(1, 7))),
                Square::from(Piece::Rook(Color::White, Position::new(0, 0))),
                Square::from(Piece::Knight(Color::White, Position::new(0, 1))),
                Square::from(Piece::Bishop(Color::White, Position::new(0, 2))),
                Square::from(Piece::Queen(Color::White, Position::new(0, 3))),
                Square::from(Piece::King(Color::White, Position::new(0, 4))),
                Square::from(Piece::Queen(Color::White, Position::new(0, 5))),
                // Square::from(Piece::Bishop(Color::White, Position::new(0, 5))),
                Square::from(Piece::Knight(Color::White, Position::new(0, 6))),
                Square::from(Piece::Rook(Color::White, Position::new(0, 7))),
            ],

            en_passant_square: Position::new(0, 0),

            white_castling_rights: CastlingRights::default(),
            black_castling_rights: CastlingRights::default(),

            turn: Color::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board {
    squares: [Square; 64],

    en_passant_square: Position,

    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,

    turn: Color,
}

impl Evaluate for Board {
    #[inline]
    fn material_value_for(&self, ally_color: Color) -> f64 {
        self.squares.iter().map(|square| match square.get_piece() {
            Some(piece) => if piece.get_color() == ally_color {
                piece.get_weighted_value()
            } else {
                -piece.get_weighted_value()
            },
            None => 0.0
        }).sum()
    }

    #[inline]
    fn get_current_player_color(&self) -> Color {
        self.turn
    }

    #[inline]
    fn play_move(&self, m: Move) -> Self {
        self.apply_move(m).change_turn()
    }

    #[inline]
    fn get_legal_moves(&self) -> Vec<Move> {
        let mut result = vec![];
        let color = self.get_current_player_color();
        for square in &self.squares {
            if let Some(piece) = square.get_piece() {
                if piece.get_color() == color {
                    result.extend(piece.get_legal_moves(self))
                }
            }
        }

        result
    }
}

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(f, "   abcdefgh\n  ╔════════╗")?;
        let mut square_color = !self.turn;

        let height = 8;
        let width = 8;

        for row in 0..height {
            write!(f, "\n")?;

            let print_row = match self.turn {
                Color::White => height - row - 1,
                Color::Black => row,
            };
            write!(f, "{} ║", print_row + 1)?;

            for col in 0..width {
                let pos = Position::new(print_row, col);

                let s = if let Some(piece) = self.get_piece(pos) {
                    piece.to_string()
                } else {
                    String::from(match square_color {
                        Color::White => "░",
                        Color::Black => "▓",
                    })
                };

                if self.is_threatened(pos, self.turn) {
                    write!(f, "\x1b[31m{}\x1b[m\x1b[0m", s)?;
                } else if self.is_threatened(pos, !self.turn) {
                    write!(f, "\x1b[32m{}\x1b[m\x1b[0m", s)?;
                } else {
                    write!(f, "{}", s)?;
                }

                square_color = !square_color;
            }
            write!(f, "║")?;
            square_color = !square_color;
        }

        write!(f, "\n  ╚════════╝\n   abcdefgh\n")?;

        let white_adv = self.get_material_advantage(Color::White);
        let black_adv = self.get_material_advantage(Color::Black);

        if white_adv == black_adv {
            write!(f, "Both sides have equal materials\n")?
        } else if white_adv > black_adv {
            write!(f, "White has a {} point material advantage\n", white_adv)?
        } else {
            write!(f, "Black has a {} point material advantage\n", black_adv)?
        }
        write!(f, "{} to move", self.turn)
    }
}

impl Board {
    pub fn set_turn(&self, color: Color) -> Self {
        let mut result = *self;
        result.turn = color;
        result
    }

    #[inline]
    pub fn get_material_advantage(&self, color: Color) -> i32 {
        self.squares.iter().map(|square| match square.get_piece() {
            Some(piece) => if piece.get_color() == color {
                piece.get_material_value()
            } else {
                -piece.get_material_value()
            },
            None => 0
        }).sum()
    }

    #[inline]
    pub fn get_turn_color(&self) -> Color {
        self.turn
    }

    #[inline]
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        if pos.is_off_board() {
            println!("off board {:?}", pos);
            return None;
        }
        self.squares[((7 - pos.get_row()) * 8 + pos.get_col()) as usize].get_piece()
    }

    #[inline]
    pub fn has_ally_piece(&self, pos: Position, ally_color: Color) -> bool {
        if let Some(piece) = self.get_piece(pos) {
            piece.get_color() == ally_color
        } else {
            false
        }
    }

    #[inline]
    pub fn has_enemy_piece(&self, pos: Position, ally_color: Color) -> bool {
        if let Some(piece) = self.get_piece(pos) {
            piece.get_color() == !ally_color
        } else {
            false
        }
    }

    #[inline]
    pub fn has_piece(&self, pos: Position) -> bool {
        self.get_piece(pos) != None
    }

    #[inline]
    pub fn has_no_piece(&self, pos: Position) -> bool {
        self.get_piece(pos) == None
    }

    #[inline]
    pub fn get_king_pos(&self, color: Color) -> Position {
        let mut king_pos = Position::king_pos(color);
        for square in &self.squares {
            if let Some(Piece::King(c, pos)) = square.get_piece() {
                if c == color {
                    king_pos = pos;
                }
            }
        }
        king_pos
    }

    #[inline]
    pub fn is_threatened(&self, pos: Position, ally_color: Color) -> bool {
        for square in &self.squares {
            if let Some(piece) = square.get_piece() {
                if piece.get_color() == ally_color {
                    continue;
                }
                if piece.is_legal_attack(pos, self) {
                    return true;
                }
            }
        }

        false
    }

    #[inline]
    pub fn is_in_check(&self, color: Color) -> bool {
        let king_pos = self.get_king_pos(color);
        self.is_threatened(king_pos, color)
    }

    pub fn move_piece(&self, from: Position, to: Position) -> Self {
        let mut result = *self;

        if from.is_off_board() || to.is_off_board() {
            return result;
        }

        let from_square = &mut result.squares[((7 - from.get_row()) * 8 + from.get_col()) as usize];
        if let Some(mut piece) = from_square.get_piece() {
            *from_square = EMPTY_SQUARE;

            if piece.is_pawn() && (to.get_row() == 0 || to.get_row() == 7) {
                piece = Piece::Queen(piece.get_color(), piece.get_pos());
            }

            result.squares[((7 - to.get_row()) * 8 + to.get_col()) as usize] =
                Square::from(piece.move_to(to));

            let castling_rights = match piece.get_color() {
                Color::White => &mut result.white_castling_rights,
                Color::Black => &mut result.black_castling_rights,
            };

            if piece.is_king() {
                castling_rights.invalidate();
            } else if piece.is_queenside_rook() {
                castling_rights.no_queenside();
            } else if piece.is_kingside_rook() {
                castling_rights.no_kingside();
            }
        }

        result
    }

    pub fn can_kingside_castle(&self, color: Color) -> bool {
        match color {
            Color::White => {
                self.has_no_piece(Position::new(0, 5))
                    && self.has_no_piece(Position::new(0, 6))
                    && !self.is_in_check(color)
                    && !self.is_threatened(Position::king_pos(color).next_right(), color)
                    && self.white_castling_rights.can_kingside_castle()
            }
            Color::Black => {
                self.has_no_piece(Position::new(7, 5))
                    && self.has_no_piece(Position::new(7, 6))
                    && !self.is_in_check(color)
                    && !self.is_threatened(Position::king_pos(color).next_right(), color)
                    && self.black_castling_rights.can_kingside_castle()
            }
        }
    }

    pub fn can_queenside_castle(&self, color: Color) -> bool {
        match color {
            Color::White => {
                self.has_no_piece(Position::new(0, 1))
                    && self.has_no_piece(Position::new(0, 2))
                    && self.has_no_piece(Position::new(0, 3))
                    && !self.is_threatened(Position::queen_pos(color), color)
                    && self.white_castling_rights.can_queenside_castle()
            }
            Color::Black => {
                self.has_no_piece(Position::new(7, 1))
                    && self.has_no_piece(Position::new(7, 2))
                    && self.has_no_piece(Position::new(7, 3))
                    && !self.is_in_check(color)
                    && !self.is_threatened(Position::queen_pos(color), color)
                    && self.black_castling_rights.can_queenside_castle()
            }
        }
    }

    pub fn is_legal_move(&self, m: Move, player_color: Color) -> bool {
        (match m {
            Move::KingSideCastle => self.can_kingside_castle(player_color),
            Move::QueenSideCastle => self.can_queenside_castle(player_color),
            Move::Piece(from, to) => {
                if let Some(piece) = self.get_piece(from) {
                    piece.is_legal_move(to, self) && piece.get_color() == player_color
                } else {
                    false
                }
            }
        }) && !self.apply_move(m).is_in_check(player_color)
    }

    #[inline]
    pub fn change_turn(mut self) -> Self {
        self.turn = !self.turn;
        self
    }

    pub fn apply_move(&self, m: Move) -> Self {
        match m {
            Move::KingSideCastle => {
                let king_pos = self.get_king_pos(self.turn);
                let rook_pos = match self.turn {
                    Color::White => Position::new(0, 7),
                    Color::Black => Position::new(7, 7),
                };
                self.move_piece(king_pos, rook_pos.next_left())
                    .move_piece(rook_pos, king_pos.next_right())
            }
            Move::QueenSideCastle => {
                let king_pos = self.get_king_pos(self.turn);
                let rook_pos = match self.turn {
                    Color::White => Position::new(0, 0),
                    Color::Black => Position::new(7, 0),
                };
                self.move_piece(king_pos, king_pos.next_left().next_left())
                    .move_piece(rook_pos, king_pos.next_left())
            }

            Move::Piece(from, to) => self.move_piece(from, to),
        }
    }
}
