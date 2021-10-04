use super::{Board, Color, Move, Position};
use alloc::vec::Vec;

/// A piece on a board.
///
/// Every piece has both a color and a position.
/// These, combined with the type of piece it is,
/// determine things like
/// 1. The validity of legal moves
/// 2. The validity of legal attacks
/// 3. Move generation
/// 4. Material and positional value
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    King(Color, Position),
    Queen(Color, Position),
    Rook(Color, Position),
    Bishop(Color, Position),
    Knight(Color, Position),
    Pawn(Color, Position),
}

const WHITE_KING_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [-1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0],
    [2.0, 3.0, 1.0, 0.0, 0.0, 1.0, 3.0, 2.0],
];

const BLACK_KING_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [2.0, 3.0, 1.0, 0.0, 0.0, 1.0, 3.0, 2.0],
    [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0],
    [-1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0],
    [-2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
    [-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0],
];

const WHITE_QUEEN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, -0.0, -1.0, -0.5, -0.5, -0.5, -1.0, -2.0],
];
const BLACK_QUEEN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-1.0, -0.0, -1.0, -0.5, -0.5, -0.5, -1.0, -2.0],
    [-1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -0.5],
    [-1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0],
];

const WHITE_ROOK_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0],
];

const BLACK_ROOK_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [-0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5],
    [0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

const WHITE_BISHOP_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, -1.0],
    [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0],
    [-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
    [-1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -1.0],
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
];

const BLACK_BISHOP_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
    [-1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -1.0],
    [-1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0],
    [-1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, -1.0],
    [-1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, -1.0],
    [-1.0, 0.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0],
    [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0],
    [-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0],
];

const WHITE_KNIGHT_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0, -4.0],
    [-3.0, 0.0, 1.0, 1.5, 1.5, 1.0, 0.0, -3.0],
    [-3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0],
    [-3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0],
    [-3.0, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -3.0],
    [-4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
];

const BLACK_KNIGHT_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
    [-4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0],
    [-3.0, 0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -3.0],
    [-3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0],
    [-3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0],
    [-3.0, 0.0, 1.0, 1.5, 1.5, 1.0, 0.0, -3.0],
    [-4.0, -2.0, 0.0, 0.0, 0.0, 0.0, -2.0, -4.0],
    [-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0],
];

const WHITE_PAWN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
    [1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0],
    [0.5, 0.5, 1.0, 2.5, 2.5, 1.0, 0.5, 0.5],
    [0.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0, 0.0],
    [0.5, -0.5, -1.0, 0.0, 0.0, -1.0, -0.5, 0.5],
    [0.5, 1.5, -1.0, -2.0, -2.0, 1.0, 1.5, 0.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

const BLACK_PAWN_POSITION_WEIGHTS: [[f64; 8]; 8] = [
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
    /// Get the name of the piece such as `"pawn"` or `"king"`.
    /// All names are lowercase.
    #[inline]
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::King(_, _) => "king",
            Self::Queen(_, _) => "queen",
            Self::Rook(_, _) => "rook",
            Self::Bishop(_, _) => "bishop",
            Self::Knight(_, _) => "knight",
            Self::Pawn(_, _) => "pawn",
        }
    }

    /// Get the material value for a piece.
    /// | Name | Value |
    /// |-|-|
    /// | King | 99999 |
    /// | Queen | 9 |
    /// | Rook | 5 |
    /// | Bishop | 3 |
    /// | Knight | 3 |
    /// | Pawn | 1 |
    #[inline]
    pub fn get_material_value(&self) -> i32 {
        match self {
            Self::King(_, _) => 99999,
            Self::Queen(_, _) => 9,
            Self::Rook(_, _) => 5,
            Self::Bishop(_, _) => 3,
            Self::Knight(_, _) => 3,
            Self::Pawn(_, _) => 1,
        }
    }

    /// Get the weighted value of a piece. This simply factors in position
    /// to the pieces value. For example, a knight that is in the center is
    /// more favorable than a knight on the side of the board. Similarly,
    /// a king in the center of the board is highly unfavorable compared to
    /// a king its respective side.
    ///
    /// Additionally, the weighted value of the piece is 10 times greater than
    /// its material value, plus or minus a weight ranging between 5.0 and -5.0.
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

    /// Get the color of a given piece.
    #[inline]
    pub fn with_color(&self, color: Color) -> Self {
        match *self {
            Self::King(_, pos) => Self::King(color, pos),
            Self::Queen(_, pos) => Self::Queen(color, pos),
            Self::Rook(_, pos) => Self::Rook(color, pos),
            Self::Bishop(_, pos) => Self::Bishop(color, pos),
            Self::Knight(_, pos) => Self::Knight(color, pos),
            Self::Pawn(_, pos) => Self::Pawn(color, pos),
        }
    }

    /// Get the color of a given piece.
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

    /// Get the position of a piece.
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

    /// Is this piece a king?
    #[inline]
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King(_, _))
    }

    /// Is this piece a queen?
    #[inline]
    pub fn is_queen(&self) -> bool {
        matches!(self, Self::Queen(_, _))
    }

    /// Is this piece a rook?
    #[inline]
    pub fn is_rook(&self) -> bool {
        matches!(self, Self::Rook(_, _))
    }

    /// Is this piece a bishop?
    #[inline]
    pub fn is_bishop(&self) -> bool {
        matches!(self, Self::Bishop(_, _))
    }

    /// Is this piece a knight?
    #[inline]
    pub fn is_knight(&self) -> bool {
        matches!(self, Self::Knight(_, _))
    }

    /// Is this piece a pawn?
    #[inline]
    pub fn is_pawn(&self) -> bool {
        matches!(self, Self::Pawn(_, _))
    }

    /// Is this piece a starting pawn?
    ///
    /// A starting pawn is a pawn that has not been pushed
    /// yet whatsoever.
    #[inline]
    pub fn is_starting_pawn(&self) -> bool {
        if let Self::Pawn(c, pos) = self {
            pos.is_starting_pawn(*c)
        } else {
            false
        }
    }

    /// Is this piece in the starting position for the queenside rook?
    ///
    /// This method will only return true for rooks that are in the position
    /// of the queenside rook, not for any particular rook.
    #[inline]
    pub fn is_queenside_rook(&self) -> bool {
        if let Self::Rook(_, pos) = self {
            pos.is_queenside_rook()
        } else {
            false
        }
    }

    /// Is this piece in the starting position for the kingside rook?
    ///
    /// This method will only return true for rooks that are in the position
    /// of the kingside rook, not for any particular rook.
    #[inline]
    pub fn is_kingside_rook(&self) -> bool {
        if let Self::Rook(_, pos) = self {
            pos.is_kingside_rook()
        } else {
            false
        }
    }

    /// Change the position of this piece to a new position.
    ///
    /// For example, `Pawn(Color::White, E4).move_to(E5)` will result in
    /// `Pawn(Color::White, E5)`. This does not check for move legality,
    /// it merely creates a new piece with the same color and type, but
    /// with a new position.
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

    /// Get the exhaustive list of legal moves for a given piece.
    ///
    /// This is used for move generation.
    #[inline]
    pub(crate) fn get_legal_moves(&self, board: &Board) -> Vec<Move> {
        let mut result = Vec::new();
        match *self {
            Self::Pawn(ally_color, pos) => {
                let up = pos.pawn_up(ally_color);
                let next_up = up.pawn_up(ally_color);
                let up_left = up.next_left();
                let up_right = up.next_right();

                if let Some(en_passant) = board.get_en_passant() {
                    if en_passant == up_left || en_passant == up_right {
                        result.push(Move::Piece(pos, en_passant));
                    }
                }

                if next_up.is_on_board()
                    && self.is_starting_pawn()
                    && board.has_no_piece(up)
                    && board.has_no_piece(next_up)
                {
                    result.push(Move::Piece(pos, next_up))
                }

                if up.is_on_board() && board.has_no_piece(up) {
                    result.push(Move::Piece(pos, up))
                }

                if up_left.is_on_board() && board.has_enemy_piece(up_left, ally_color) {
                    result.push(Move::Piece(pos, up.next_left()))
                } else if up_right.is_on_board()
                    && board.has_enemy_piece(up.next_right(), ally_color)
                {
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
                    if p.is_on_board() && !board.has_ally_piece(*p, ally_color) {
                        result.push(Move::Piece(pos, *p))
                    }
                }
                if board.can_kingside_castle(ally_color) {
                    result.push(Move::KingSideCastle);
                } else if board.can_queenside_castle(ally_color) {
                    result.push(Move::QueenSideCastle);
                }
            }
            Self::Queen(ally_color, pos) => {
                for row in 0..8 {
                    let new_pos = Position::new(row, pos.get_col());
                    if new_pos != pos
                        && !board.has_ally_piece(new_pos, ally_color)
                        && new_pos.is_orthogonal_to(pos)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
                for col in 0..8 {
                    let new_pos = Position::new(pos.get_row(), col);
                    if new_pos != pos
                        && !board.has_ally_piece(new_pos, ally_color)
                        && new_pos.is_orthogonal_to(pos)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }

                for row in 0..8 {
                    for col in 0..8 {
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
                for row in 0..8 {
                    let new_pos = Position::new(row, pos.get_col());
                    if new_pos != pos
                        && !board.has_ally_piece(new_pos, ally_color)
                        && new_pos.is_orthogonal_to(pos)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
                for col in 0..8 {
                    let new_pos = Position::new(pos.get_row(), col);
                    if new_pos != pos
                        && !board.has_ally_piece(new_pos, ally_color)
                        && new_pos.is_orthogonal_to(pos)
                    {
                        result.push(Move::Piece(pos, new_pos));
                    }
                }
            }

            Self::Bishop(ally_color, pos) => {
                for row in 0..8 {
                    for col in 0..8 {
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
                    if p.is_on_board() && !board.has_ally_piece(*p, ally_color) {
                        result.push(Move::Piece(pos, *p))
                    }
                }
            }
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

    /// Verify that moving to a new position is a legal move.
    #[inline]
    pub(crate) fn is_legal_move(&self, new_pos: Position, board: &Board) -> bool {
        if board.has_ally_piece(new_pos, self.get_color()) || new_pos.is_off_board() {
            return false;
        }

        match *self {
            Self::Pawn(ally_color, pos) => {
                let up = pos.pawn_up(ally_color);
                let up_left = up.next_left();
                let up_right = up.next_right();

                (if let Some(en_passant) = board.get_en_passant() {
                    (en_passant == up_left || en_passant == up_right) && (new_pos == en_passant)
                } else {
                    false
                }) || (self.is_starting_pawn()
                    && board.has_no_piece(new_pos)
                    && board.has_no_piece(up)
                    && new_pos == up.pawn_up(ally_color))
                    || (board.has_enemy_piece(new_pos, ally_color) && new_pos == up_left)
                    || (board.has_enemy_piece(new_pos, ally_color) && new_pos == up_right)
                    || (board.has_no_piece(new_pos) && new_pos == up)
            }

            Self::King(_, pos) => pos.is_adjacent_to(new_pos),

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

    /// Verify that attacking a given square is a legal move.
    #[inline]
    pub(crate) fn is_legal_attack(&self, new_pos: Position, board: &Board) -> bool {
        if board.has_ally_piece(new_pos, self.get_color()) || new_pos.is_off_board() {
            return false;
        }

        match *self {
            Self::Pawn(ally_color, pos) => {
                let up = pos.pawn_up(ally_color);
                (if let Some(en_passant) = board.get_en_passant() {
                    (en_passant == up.next_left() || en_passant == up.next_right())
                        && (new_pos == en_passant)
                } else {
                    false
                }) || new_pos == up.next_left()
                    || new_pos == up.next_right()
            }

            Self::King(_, pos) => pos.is_adjacent_to(new_pos),

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
