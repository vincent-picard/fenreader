use std::fmt;

pub enum Color {
    White,
    Black,
}

pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn => write!(f, "P"),
            Piece::Knight => write!(f, "N"),
            Piece::Bishop => write!(f, "B"),
            Piece::Rook => write!(f, "R"),
            Piece::Queen => write!(f, "Q"),
            Piece::King => write!(f, "K"),
        }
    }
}

impl Piece {
    pub fn from_algebraic(s: &str) -> Result<Piece, &'static str> {
        match s.chars().next() {
            Some('P') => Ok(Piece::Pawn),
            Some('N') => Ok(Piece::Knight),
            Some('B') => Ok(Piece::Bishop),
            Some('R') => Ok(Piece::Rook),
            Some('Q') => Ok(Piece::Queen),
            Some('K') => Ok(Piece::King),
            _ => Err("Not a valid piece in algebraic notation"),
        }
    }
}

