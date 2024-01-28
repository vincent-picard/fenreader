pub mod color;
pub mod square;
pub mod piece;

use std::iter;
use color::Color;
use square::Square;
use piece::{Piece, ColoredPiece};

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
    
    pub fn set(&mut self, square: &Square, piece: ColoredPiece) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_has_correct_size() {
        let board = Board::new();
        assert!(board.content.len() == 64);
    }

    #[test]
    #[should_panic]
    fn two_pieces_on_same_square() {
        let mut board = Board::new();
        let g1 = Square::from_algebraic("g1").unwrap();
        board.set(&g1, ColoredPiece(Color::White, Piece::Knight));
        board.set(&g1, ColoredPiece(Color::White, Piece::Queen));
    }
}

