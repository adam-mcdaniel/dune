use super::Piece;

/// Essentially a container for a single piece on a board.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square {
    piece: Option<Piece>,
}

/// A square containing no piece
pub const EMPTY_SQUARE: Square = Square { piece: None };

impl From<Piece> for Square {
    fn from(piece: Piece) -> Self {
        Self { piece: Some(piece) }
    }
}

impl Square {
    /// Does this square contain a piece?
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.piece == None
    }

    /// Get the piece contained in this square.
    #[inline]
    pub fn get_piece(&self) -> Option<Piece> {
        self.piece
    }
}
