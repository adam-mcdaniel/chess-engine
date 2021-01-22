use super::{Board, Color, Move, Position};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    King(Color, Position),
    Queen(Color, Position),
    Rook(Color, Position),
    Bishop(Color, Position),
    Knight(Color, Position),
    Pawn(Color, Position),
}

pub const WHITE_KING_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [-1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0],
    [2.0, 3.0, 1.0, 0.0, 0.0, 1.0, 3.0, 2.0],
];

pub const BLACK_KING_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [2.0, 3.0, 1.0, 0.0, 0.0, 1.0, 3.0, 2.0],
    [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0],
    [-1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [-2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
];

pub const WHITE_QUEEN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, -0.0, -1.0, -0.5, -0.5, -0.5, -1.0, -2.0],
];
pub const BLACK_QUEEN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-1.0, -0.0, -1.0, -0.5, -0.5, -0.5, -1.0, -2.0],
    [-1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
];

pub const WHITE_ROOK_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0],
];

pub const BLACK_ROOK_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

pub const WHITE_BISHOP_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, -1.0],
    [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0],
    [-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
    [-1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -1.0],
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
];

pub const BLACK_BISHOP_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [-1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -1.0],
    [-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
    [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, -1.0],
    [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
];

pub const WHITE_KNIGHT_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0, -4.0],
    [-3.0, 0.0, 1.0, 1.5, 1.5, 1.0, 0.0, -3.0],
    [-3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0],
    [-3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0],
    [-3.0, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -3.0],
    [-4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
];

pub const BLACK_KNIGHT_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0],
    [-3.0, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -3.0],
    [-3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0],
    [-3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0],
    [-3.0, 0.0, 1.0, 1.5, 1.5, 1.0, 0.0, -3.0],
    [-4.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
];

pub const WHITE_PAWN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
    [1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0],
    [0.5, 0.5, 1.0, 2.5, 2.5, 1.0, 0.5, 0.5],
    [0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 0.0],
    [0.5, -0.5, -1.0, 0.0, 0.0, -1.0, -0.5, 0.5],
    [0.5, 1.5, -1.0, -2.0, -2.0, 1.0, 1.5, 0.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

pub const BLACK_PAWN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 1.5, -1.0, -2.0, -2.0, 1.0, 1.5, 0.5],
    [0.5, -0.5, -1.0, 0.0, 0.0, -1.0, -0.5, 0.5],
    [0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 0.0],
    [0.5, 0.5, 1.0, 2.5, 2.5, 1.0, 0.5, 0.5],
    [1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0],
    [5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self.get_color() {
                Color::Black => match self {
                    Self::King(_, _) => "♔",
                    Self::Queen(_, _) => "♕",
                    Self::Rook(_, _) => "♖",
                    Self::Knight(_, _) => "♘",
                    Self::Bishop(_, _) => "♗",
                    Self::Pawn(_, _) => "♙",
                },
                Color::White => match self {
                    Self::King(_, _) => "♚",
                    Self::Queen(_, _) => "♛",
                    Self::Rook(_, _) => "♜",
                    Self::Knight(_, _) => "♞",
                    Self::Bishop(_, _) => "♝",
                    Self::Pawn(_, _) => "♟︎",
                },
            }
        )
    }
}

impl Piece {
    #[inline]
    pub fn get_material_value(&self) -> i32 {
        match self {
            Self::King(_, _) => 60000,
            Self::Queen(_, _) => 9,
            Self::Rook(_, _) => 5,
            Self::Bishop(_, _) => 3,
            Self::Knight(_, _) => 3,
            Self::Pawn(_, _) => 1,
        }
    }

    #[inline]
    pub fn get_weighted_value(&self) -> f64 {
        let weights = match self {
            Self::King(c, _) => match c {
                Color::White => WHITE_KING_POSITION_WEIGHTS,
                Color::Black => BLACK_KING_POSITION_WEIGHTS,
            },
            Self::Queen(c, _) => match c {
                Color::White => WHITE_QUEEN_POSITION_WEIGHTS,
                Color::Black => BLACK_QUEEN_POSITION_WEIGHTS,
            },
            Self::Rook(c, _) => match c {
                Color::White => WHITE_ROOK_POSITION_WEIGHTS,
                Color::Black => BLACK_ROOK_POSITION_WEIGHTS,
            },
            Self::Bishop(c, _) => match c {
                Color::White => WHITE_BISHOP_POSITION_WEIGHTS,
                Color::Black => BLACK_BISHOP_POSITION_WEIGHTS,
            },
            Self::Knight(c, _) => match c {
                Color::White => WHITE_KNIGHT_POSITION_WEIGHTS,
                Color::Black => BLACK_KNIGHT_POSITION_WEIGHTS,
            },
            Self::Pawn(c, _) => match c {
                Color::White => WHITE_PAWN_POSITION_WEIGHTS,
                Color::Black => BLACK_PAWN_POSITION_WEIGHTS,
            },
        };
        weights[(7 - self.get_pos().get_row()) as usize][self.get_pos().get_col() as usize]
            + (self.get_material_value() * 10) as f64
    }

    #[inline]
    pub fn get_color(&self) -> Color {
        match self {
            Self::King(c, _)
            | Self::Queen(c, _)
            | Self::Rook(c, _)
            | Self::Bishop(c, _)
            | Self::Knight(c, _)
            | Self::Pawn(c, _) => *c,
        }
    }

    #[inline]
    pub fn get_pos(&self) -> Position {
        match self {
            Self::King(_, p)
            | Self::Queen(_, p)
            | Self::Rook(_, p)
            | Self::Bishop(_, p)
            | Self::Knight(_, p)
            | Self::Pawn(_, p) => *p,
        }
    }

    #[inline]
    pub fn is_king(&self) -> bool {
        if let Self::King(_, _) = self {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_queen(&self) -> bool {
        if let Self::Queen(_, _) = self {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_rook(&self) -> bool {
        if let Self::Rook(_, _) = self {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_bishop(&self) -> bool {
        if let Self::Bishop(_, _) = self {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_knight(&self) -> bool {
        if let Self::Knight(_, _) = self {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_pawn(&self) -> bool {
        if let Self::Pawn(_, _) = self {
            true
        } else {
            false
        }
    }

    #[inline]
    fn is_starting_pawn(&self) -> bool {
        if let Self::Pawn(c, pos) = self {
            pos.is_starting_pawn(*c)
        } else {
            false
        }
    }

    #[inline]
    pub fn is_queenside_rook(&self) -> bool {
        if let Self::Rook(_, pos) = self {
            pos.is_queenside_rook()
        } else {
            false
        }
    }

    #[inline]
    pub fn is_kingside_rook(&self) -> bool {
        if let Self::Rook(_, pos) = self {
            pos.is_kingside_rook()
        } else {
            false
        }
    }

    #[inline]
    pub fn move_to(&self, new_pos: Position) -> Self {
        match *self {
            Self::King(c, _) => Self::King(c, new_pos),
            Self::Queen(c, _) => Self::Queen(c, new_pos),
            Self::Rook(c, _) => Self::Rook(c, new_pos),
            Self::Bishop(c, _) => Self::Bishop(c, new_pos),
            Self::Knight(c, _) => Self::Knight(c, new_pos),
            Self::Pawn(c, _) => Self::Pawn(c, new_pos),
        }
    }

    #[inline]
    pub(crate) fn get_legal_moves(&self, board: &Board) -> Vec<Move> {
        let mut result = Vec::new();
        match *self {
            Self::Pawn(ally_color, pos) => {
                let up = pos.pawn_up(ally_color);
                let next_up = up.pawn_up(ally_color);

                if next_up.is_on_board() && self.is_starting_pawn() && board.has_no_piece(up) && board.has_no_piece(next_up)
                {
                    result.push(Move::Piece(pos, next_up))
                }

                if up.is_on_board() && board.has_no_piece(up) {
                    result.push(Move::Piece(pos, up))
                }


                let up_left = up.next_left();
                let up_right = up.next_right();
                if up_left.is_on_board() && board.has_enemy_piece(up_left, ally_color) {
                    result.push(Move::Piece(pos, up.next_left()))
                } else if up_right.is_on_board() && board.has_enemy_piece(up.next_right(), ally_color) {
                    result.push(Move::Piece(pos, up.next_right()))
                }
            }

            Self::King(ally_color, pos) => {
                for p in &[
                    pos.next_left(),
                    pos.next_right(),
                    pos.next_above(),
                    pos.next_below(),
                    pos.next_left().next_above(),
                    pos.next_left().next_below(),
                    pos.next_right().next_above(),
                    pos.next_right().next_below(),
                ] {
                    if p.is_on_board() {
                        if !board.has_ally_piece(*p, ally_color) {
                            result.push(Move::Piece(pos, *p))
                        }
                    }
                }
                if board.can_kingside_castle(ally_color) {
                    result.push(Move::KingSideCastle);
                } else if board.can_queenside_castle(ally_color) {
                    result.push(Move::QueenSideCastle);
                }
            }
            Self::Queen(ally_color, pos) => {
                for row in 0..7 {
                    let new_pos = Position::new(row, pos.get_col());
                    if new_pos != pos && !board.has_ally_piece(new_pos, ally_color) && new_pos.is_orthogonal_to(pos) {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
                for col in 0..7 {
                    let new_pos = Position::new(pos.get_row(), col);
                    if new_pos != pos && !board.has_ally_piece(new_pos, ally_color) && new_pos.is_orthogonal_to(pos) {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }

                for row in 0..7 {
                    for col in 0..7 {
                        let new_pos = Position::new(row, col);
                        if new_pos != pos
                            && !board.has_ally_piece(new_pos, ally_color)
                            && new_pos.is_diagonal_to(pos)
                        {
                            result.push(Move::Piece(pos, new_pos));
                        }
                    }
                }
            }

            Self::Rook(ally_color, pos) => {
                for row in 0..7 {
                    let new_pos = Position::new(row, pos.get_col());
                    if new_pos != pos && !board.has_ally_piece(new_pos, ally_color) && new_pos.is_orthogonal_to(pos) {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
                for col in 0..7 {
                    let new_pos = Position::new(pos.get_row(), col);
                    if new_pos != pos && !board.has_ally_piece(new_pos, ally_color) && new_pos.is_orthogonal_to(pos) {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
            }

            Self::Bishop(ally_color, pos) => {
                for row in 0..7 {
                    for col in 0..7 {
                        let new_pos = Position::new(row, col);
                        if new_pos != pos && !board.has_ally_piece(new_pos, ally_color) && new_pos.is_diagonal_to(pos) {
                            result.push(Move::Piece(pos, new_pos));
                        }
                    }
                }
            }
            Self::Knight(ally_color, pos) => {
                for p in &[
                    pos.next_left().next_left().next_above(),
                    pos.next_left().next_above().next_above(),
                    
                    pos.next_left().next_left().next_below(),
                    pos.next_left().next_below().next_below(),

                    pos.next_right().next_right().next_above(),
                    pos.next_right().next_above().next_above(),
                    
                    pos.next_right().next_right().next_below(),
                    pos.next_right().next_below().next_below(),
                ] {
                    if p.is_on_board() {
                        if !board.has_ally_piece(*p, ally_color) {
                            result.push(Move::Piece(pos, *p))
                        }
                    }
                }
            },
        }
        
        let color = self.get_color();
        result
            .into_iter()
            .filter(|x| match x {
                Move::Piece(from, to) => {
                    if from.is_on_board() && to.is_on_board() {
                        board.is_legal_move(*x, color)
                    } else {
                        false
                    }
                }
                _ => board.is_legal_move(*x, color),
            })
            .collect::<Vec<Move>>()
    }
    
    #[inline]
    pub(crate) fn is_legal_move(&self, new_pos: Position, board: &Board) -> bool {
        if board.has_ally_piece(new_pos, self.get_color()) || new_pos.is_off_board() {
            return false;
        }

        let result = match *self {
            Self::Pawn(ally_color, pos) => {
                let up = pos.pawn_up(ally_color);
                (self.is_starting_pawn()
                    && board.has_no_piece(new_pos)
                    && board.has_no_piece(up)
                    && new_pos == up.pawn_up(ally_color))
                    || (board.has_enemy_piece(new_pos, ally_color) && new_pos == up.next_left())
                    || (board.has_enemy_piece(new_pos, ally_color) && new_pos == up.next_right())
                    || (board.has_no_piece(new_pos) && new_pos == up)
            }

            Self::King(_, pos) => {
                pos.is_adjacent_to(new_pos)
            }

            Self::Queen(_, pos) => {
                if pos.is_orthogonal_to(new_pos) {
                    let mut traveling = pos.orthogonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else if pos.is_diagonal_to(new_pos) {
                    let mut traveling = pos.diagonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Rook(_, pos) => {
                if pos.is_orthogonal_to(new_pos) {
                    let mut traveling = pos.orthogonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Bishop(_, pos) => {
                if pos.is_diagonal_to(new_pos) {
                    let mut traveling = pos.diagonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Knight(_, pos) => pos.is_knight_move(new_pos),
        };

        result
    }

    #[inline]
    pub(crate) fn is_legal_attack(&self, new_pos: Position, board: &Board) -> bool {
        if board.has_ally_piece(new_pos, self.get_color()) || new_pos.is_off_board() {
            return false;
        }

        match *self {
            Self::Pawn(ally_color, pos) => {
                let up = pos.pawn_up(ally_color);
                new_pos == up.next_left() || new_pos == up.next_right()
            }

            Self::King(_, pos) => {
                pos.is_adjacent_to(new_pos)
            }

            Self::Queen(_, pos) => {
                if pos.is_orthogonal_to(new_pos) {
                    let mut traveling = pos.orthogonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else if pos.is_diagonal_to(new_pos) {
                    let mut traveling = pos.diagonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Rook(_, pos) => {
                if pos.is_orthogonal_to(new_pos) {
                    let mut traveling = pos.orthogonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Bishop(_, pos) => {
                if pos.is_diagonal_to(new_pos) {
                    let mut traveling = pos.diagonals_to(new_pos);
                    traveling.pop();

                    for pos in traveling {
                        if board.has_piece(pos) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            Self::Knight(_, pos) => pos.is_knight_move(new_pos),
        }
    }
}
