use super::Piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square {
    piece: Option<Piece>,
}

pub const EMPTY_SQUARE: Square = Square { piece: None };

impl From<Piece> for Square {
    fn from(piece: Piece) -> Self {
        Self { piece: Some(piece) }
    }
}

impl Square {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.piece == None
    }

    #[inline]
    pub fn get_piece(&self) -> Option<Piece> {
        self.piece
    }
}
