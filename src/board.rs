pub mod color;
pub mod square;

use std::fmt;
use std::iter;
use color::Color;
use square::Square;

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

pub struct Board {
    content: Vec<Option<ColoredPiece>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            content: iter::repeat(None).take(64).collect(),
        }
    }

    pub fn get(&self, square: &Square) -> &Option<ColoredPiece> {
        &self.content[square.to_index()]
    }
    
    pub fn place(&mut self, square: &Square, piece: ColoredPiece) {
        let s = &mut self.content[square.to_index()];
        if s.is_some() {
            panic!("place : a piece already exists in this square");
        }
        *s = Some(piece);
    }

    pub fn clear_square(&mut self, square: &Square) {
        self.content[square.to_index()] = None;
    }

}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}



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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_to_string() {
        assert_eq!("♙", Piece::Pawn.to_string());
        assert_eq!("♘", Piece::Knight.to_string());
        assert_eq!("♗", Piece::Bishop.to_string());
        assert_eq!("♖", Piece::Rook.to_string());
        assert_eq!("♕", Piece::Queen.to_string());
        assert_eq!("♔", Piece::King.to_string());
    }

    #[test]
    fn colored_piece_to_string() {
        assert_eq!("♙", ColoredPiece(Color::White, Piece::Pawn).to_string());
        assert_eq!("♘", ColoredPiece(Color::White, Piece::Knight).to_string());
        assert_eq!("♗", ColoredPiece(Color::White, Piece::Bishop).to_string());
        assert_eq!("♖", ColoredPiece(Color::White, Piece::Rook).to_string());
        assert_eq!("♕", ColoredPiece(Color::White, Piece::Queen).to_string());
        assert_eq!("♔", ColoredPiece(Color::White, Piece::King).to_string());
        assert_eq!("♟", ColoredPiece(Color::Black, Piece::Pawn).to_string());
        assert_eq!("♞", ColoredPiece(Color::Black, Piece::Knight).to_string());
        assert_eq!("♝", ColoredPiece(Color::Black, Piece::Bishop).to_string());
        assert_eq!("♜", ColoredPiece(Color::Black, Piece::Rook).to_string());
        assert_eq!("♛", ColoredPiece(Color::Black, Piece::Queen).to_string());
        assert_eq!("♚", ColoredPiece(Color::Black, Piece::King).to_string());
    }

    #[test]
    fn colors_of_squares() {
        assert!(Square::from_coord(0, 0).expect("Should be valid coordinates").color().is_black());
        assert!(Square::from_coord(1, 1).expect("Should be valid coordinates").color().is_black());
        assert!(Square::from_coord(0, 2).expect("Should be valid coordinates").color().is_black());
        assert!(Square::from_coord(0, 7).expect("Should be valid coordinates").color().is_white());
    }
        
}
