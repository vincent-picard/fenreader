use crate::board::color::Color;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct ColoredPiece(Color, Piece);

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn => write!(f, "♙"),
            Piece::Knight => write!(f, "♘"),
            Piece::Bishop => write!(f, "♗"),
            Piece::Rook => write!(f, "♖"),
            Piece::Queen => write!(f, "♕"),
            Piece::King => write!(f, "♔"),
        }
    }
}

impl fmt::Display for ColoredPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColoredPiece(Color::White, Piece::Pawn) => write!(f, "♙"),
            ColoredPiece(Color::White, Piece::Knight) => write!(f, "♘"),
            ColoredPiece(Color::White, Piece::Bishop) => write!(f, "♗"),
            ColoredPiece(Color::White, Piece::Rook) => write!(f, "♖"),
            ColoredPiece(Color::White, Piece::Queen) => write!(f, "♕"),
            ColoredPiece(Color::White, Piece::King) => write!(f, "♔"),
            ColoredPiece(Color::Black, Piece::Pawn) => write!(f, "♟"),
            ColoredPiece(Color::Black, Piece::Knight) => write!(f, "♞"),
            ColoredPiece(Color::Black, Piece::Bishop) => write!(f, "♝"),
            ColoredPiece(Color::Black, Piece::Rook) => write!(f, "♜"),
            ColoredPiece(Color::Black, Piece::Queen) => write!(f, "♛"),
            ColoredPiece(Color::Black, Piece::King) => write!(f, "♚"),
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
