use super::Color;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

pub const A1: Position = Position::new(0, 0);
pub const A2: Position = Position::new(1, 0);
pub const A3: Position = Position::new(2, 0);
pub const A4: Position = Position::new(3, 0);
pub const A5: Position = Position::new(4, 0);
pub const A6: Position = Position::new(5, 0);
pub const A7: Position = Position::new(6, 0);
pub const A8: Position = Position::new(7, 0);

pub const B1: Position = Position::new(0, 1);
pub const B2: Position = Position::new(1, 1);
pub const B3: Position = Position::new(2, 1);
pub const B4: Position = Position::new(3, 1);
pub const B5: Position = Position::new(4, 1);
pub const B6: Position = Position::new(5, 1);
pub const B7: Position = Position::new(6, 1);
pub const B8: Position = Position::new(7, 1);

pub const C1: Position = Position::new(0, 2);
pub const C2: Position = Position::new(1, 2);
pub const C3: Position = Position::new(2, 2);
pub const C4: Position = Position::new(3, 2);
pub const C5: Position = Position::new(4, 2);
pub const C6: Position = Position::new(5, 2);
pub const C7: Position = Position::new(6, 2);
pub const C8: Position = Position::new(7, 2);

pub const D1: Position = Position::new(0, 3);
pub const D2: Position = Position::new(1, 3);
pub const D3: Position = Position::new(2, 3);
pub const D4: Position = Position::new(3, 3);
pub const D5: Position = Position::new(4, 3);
pub const D6: Position = Position::new(5, 3);
pub const D7: Position = Position::new(6, 3);
pub const D8: Position = Position::new(7, 3);

pub const E1: Position = Position::new(0, 4);
pub const E2: Position = Position::new(1, 4);
pub const E3: Position = Position::new(2, 4);
pub const E4: Position = Position::new(3, 4);
pub const E5: Position = Position::new(4, 4);
pub const E6: Position = Position::new(5, 4);
pub const E7: Position = Position::new(6, 4);
pub const E8: Position = Position::new(7, 4);

pub const F1: Position = Position::new(0, 5);
pub const F2: Position = Position::new(1, 5);
pub const F3: Position = Position::new(2, 5);
pub const F4: Position = Position::new(3, 5);
pub const F5: Position = Position::new(4, 5);
pub const F6: Position = Position::new(5, 5);
pub const F7: Position = Position::new(6, 5);
pub const F8: Position = Position::new(7, 5);

pub const G1: Position = Position::new(0, 6);
pub const G2: Position = Position::new(1, 6);
pub const G3: Position = Position::new(2, 6);
pub const G4: Position = Position::new(3, 6);
pub const G5: Position = Position::new(4, 6);
pub const G6: Position = Position::new(5, 6);
pub const G7: Position = Position::new(6, 6);
pub const G8: Position = Position::new(7, 6);

pub const H1: Position = Position::new(0, 7);
pub const H2: Position = Position::new(1, 7);
pub const H3: Position = Position::new(2, 7);
pub const H4: Position = Position::new(3, 7);
pub const H5: Position = Position::new(4, 7);
pub const H6: Position = Position::new(5, 7);
pub const H7: Position = Position::new(6, 7);
pub const H8: Position = Position::new(7, 7);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    row: i32,
    col: i32,
}

impl core::fmt::Display for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}{}",
            match self.col {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => '?',
            },
            self.row + 1
        )
    }
}

impl Position {
    /// Return the starting position for a given color's king.
    #[inline]
    pub const fn king_pos(color: Color) -> Self {
        match color {
            Color::White => Self::new(0, 4),
            Color::Black => Self::new(7, 4),
        }
    }

    /// Return the starting position for a given color's queen.
    #[inline]
    pub const fn queen_pos(color: Color) -> Self {
        match color {
            Color::White => Self::new(0, 3),
            Color::Black => Self::new(7, 3),
        }
    }

    /// Create a `Position` from its respective row or column number.
    /// The row and column numbers can be any of 0, 1, 2, 3, 4, 5, 6, or 7.
    /// 
    /// Examples:
    /// - `A1 = Position::new(0, 0)`
    /// - `A8 = Position::new(7, 0)`
    /// - `H1 = Position::new(0, 7)`
    /// - `H8 = Position::new(7, 7)`
    #[inline]
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    /// Parse a position from PGN. This simply just supports positions like
    /// `e4` and `D8`.
    pub fn pgn(s: &str) -> Result<Self, String> {
        let s = s.trim().to_lowercase();
        let col = s.chars().next().ok_or(format!("invalid pgn `{}`", s))?;
        let row = s
            .chars()
            .nth(1)
            .ok_or(format!("invalid pgn `{}`", s))?
            .to_string()
            .parse::<u32>()
            .map_err(|_| format!("invalid pgn `{}`", s))? as i32;
        let c = match col {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err(format!("invalid column character `{}`", col)),
        };

        if 1 <= row || row <= 8 {
            Ok(Self::new(row - 1, c))
        } else {
            Err(format!("invalid row number `{}`", row))
        }
    }

    /// Is this position a valid spot on the board?
    #[inline]
    pub fn is_on_board(&self) -> bool {
        !self.is_off_board()
    }

    /// Is this position NOT a valid spot on the board?
    #[inline]
    pub fn is_off_board(&self) -> bool {
        self.row < 0 || self.row > 7 || self.col < 0 || self.col > 7
    }

    /// Get the row number of the position.
    /// This can be any of 0, 1, 2, 3, 4, 5, 6, or 7.
    #[inline]
    pub fn get_row(&self) -> i32 {
        self.row
    }

    #[inline]
    pub fn get_col(&self) -> i32 {
        self.col
    }

    #[inline]
    fn add_row(&self, drow: i32) -> Self {
        let mut result = *self;
        result.row += drow;
        result
    }

    #[inline]
    fn add_col(&self, dcol: i32) -> Self {
        let mut result = *self;
        result.col += dcol;
        result
    }

    /// Is this position diagonal to another position?
    #[inline]
    pub fn is_diagonal_to(&self, other: Self) -> bool {
        // Algorithm for determining whether or not two squares are diagonal
        // https://math.stackexchange.com/questions/1194565/how-to-know-if-two-points-are-diagonally-aligned
        (self.col - other.col).abs() == (self.row - other.row).abs()
    }

    /// Get the diagonal distance between two positions
    #[inline]
    fn diagonal_distance(&self, other: Self) -> i32 {
        (self.col - other.col).abs()
    }
    
    /// Is this position orthogonal to another position?
    #[inline]
    pub fn is_orthogonal_to(&self, other: Self) -> bool {
        (self.col == other.col) || (self.row == other.row)
    }

    /// Get the orthogonal distance between two positions
    #[inline]
    fn orthogonal_distance(&self, other: Self) -> i32 {
        (self.col - other.col).abs() + (self.row - other.row).abs()
    }

    /// Is this position adjacent to another position?
    /// 
    /// Adjacent positions have either:
    /// 1. A diagonal distance of one from each other
    /// 2. An orthogonal distance of one from each other
    #[inline]
    pub fn is_adjacent_to(&self, other: Self) -> bool {
        if self.is_orthogonal_to(other) {
            self.orthogonal_distance(other) == 1
        } else if self.is_diagonal_to(other) {
            self.diagonal_distance(other) == 1
        } else {
            false
        }
    }

    /// Is this position beneath another position on the board?
    /// Pieces "beneath" other pieces on the board have lower ranks.
    /// 
    /// So, for example, A7 is below A8.
    #[inline]
    pub fn is_below(&self, other: Self) -> bool {
        self.row < other.row
    }

    /// Is this position above another position on the board?
    /// Pieces "above" other pieces on the board have higher ranks.
    /// 
    /// So, for example, A8 is above A8.
    #[inline]
    pub fn is_above(&self, other: Self) -> bool {
        self.row > other.row
    }

    /// Is this position left of another position on the board?
    /// Pieces "left of" other pieces on the board have a lower
    /// lexigraphical column character.
    /// 
    /// So, for example, A8 is left of B8.
    #[inline]
    pub fn is_left_of(&self, other: Self) -> bool {
        self.col < other.col
    }

    /// Is this position right of another position on the board?
    /// Pieces "right of" other pieces on the board have a higher
    /// lexigraphical column character.
    /// 
    /// So, for example, B8 is right of A8.
    #[inline]
    pub fn is_right_of(&self, other: Self) -> bool {
        self.col > other.col
    }

    /// Get the position directly below this position.
    /// 
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_below(&self) -> Self {
        Self::new(self.row - 1, self.col)
    }

    /// Get the position directly above this position.
    /// 
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_above(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    /// Get the next square upwards from a respective player's
    /// pawn.
    /// 
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn pawn_up(&self, ally_color: Color) -> Self {
        match ally_color {
            Color::White => self.next_above(),
            Color::Black => self.next_below(),
        }
    }

    /// Get the next square backwards from a respective player's
    /// pawn.
    /// 
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn pawn_back(&self, ally_color: Color) -> Self {
        self.pawn_up(!ally_color)
    }
    
    /// Get the position directly left of this position.
    /// 
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_left(&self) -> Self {
        Self::new(self.row, self.col - 1)
    }

    /// Get the position directly right of this position.
    /// 
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_right(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

    /// Is this pawn on the starting rank for the respective player?
    #[inline]
    pub fn is_starting_pawn(&self, color: Color) -> bool {
        match color {
            Color::White => self.row == 1,
            Color::Black => self.row == 6,
        }
    }

    /// Is this the starting position of the kingside rook?
    #[inline]
    pub fn is_kingside_rook(&self) -> bool {
        (self.row == 0 || self.row == 7) && self.col == 7
    }
    
    /// Is this the starting position of the queenside rook?
    #[inline]
    pub fn is_queenside_rook(&self) -> bool {
        (self.row == 0 || self.row == 7) && self.col == 0
    }

    /// Get the list of positions from this position to another
    /// position, moving diagonally.
    /// 
    /// This does _not_ include the `from` position, and includes the `to` position.
    pub fn diagonals_to(&self, to: Self) -> Vec<Self> {
        if !self.is_diagonal_to(to) {
            return Vec::new();
        }

        let row_step;
        let col_step;
        if self.is_left_of(to) {
            col_step = 1;
        } else {
            col_step = -1;
        }

        if self.is_below(to) {
            row_step = 1;
        } else {
            row_step = -1;
        }

        let mut acc = *self;
        let mut result = Vec::new();
        for _ in 0..self.diagonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step);
            result.push(acc);
        }

        result
    }

    /// Get the list of positions from this position to another
    /// position, moving orthogonally.
    /// 
    /// This does _not_ include the `from` position, and includes the `to` position.
    pub fn orthogonals_to(&self, to: Self) -> Vec<Self> {
        if !self.is_orthogonal_to(to) {
            return Vec::new();
        }
        let mut row_step = 0;
        let mut col_step = 0;
        if self.is_left_of(to) {
            col_step = 1;
        } else if self.is_right_of(to) {
            col_step = -1;
        } else if self.is_above(to) {
            row_step = -1;
        } else if self.is_below(to) {
            row_step = 1;
        }

        let mut acc = *self;
        let mut result = Vec::new();

        for _ in 0..self.orthogonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step);
            result.push(acc);
        }

        result
    }

    #[inline]
    pub fn is_knight_move(&self, other: Self) -> bool {
        (self.row - other.row).abs() == 2 && (self.col - other.col).abs() == 1
            || (self.row - other.row).abs() == 1 && (self.col - other.col).abs() == 2
    }
}
