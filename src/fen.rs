use crate::board::Board;
use crate::board::color::Color;
use crate::board::piece::{ColoredPiece, Piece};
use crate::board::square::Square;
use std::error::Error;

pub struct Fen {
    board: Board,
    turn: Color,
    white_short_castle: bool,
    white_long_castle: bool,
    black_short_castle: bool,
    black_long_castle: bool,
    en_passant: Option<Square>,
    half_moves: u8,
    moves: u8,
}

pub enum FenParseError {
    NumberOfWords,
    NumberOfRows,
    UnknownPiece(char),
    IncorrectRow(u8),
}

fn letter_to_piece(c: char) -> Result<ColoredPiece, FenParseError> {
    Ok (match c {
        'p' => ColoredPiece(Color::White, Piece::Pawn),
        'n' => ColoredPiece(Color::White, Piece::Knight),
        'b' => ColoredPiece(Color::White, Piece::Bishop),
        'r' => ColoredPiece(Color::White, Piece::Rook),
        'q' => ColoredPiece(Color::White, Piece::Queen),
        'k' => ColoredPiece(Color::White, Piece::King),
        'P' => ColoredPiece(Color::Black, Piece::Pawn),
        'N' => ColoredPiece(Color::Black, Piece::Knight),
        'B' => ColoredPiece(Color::Black, Piece::Bishop),
        'R' => ColoredPiece(Color::Black, Piece::Rook),
        'Q' => ColoredPiece(Color::Black, Piece::Queen),
        'K' => ColoredPiece(Color::Black, Piece::King),
        _ => return Err(FenParseError::UnknownPiece(c)),
    })
}

impl Fen {
    pub fn parse(s: &str) -> Result<Self, FenParseError> {
        let words: Vec<&str> = s.split(' ').collect();
        if words.len() != 5 {
            return Err(FenParseError::NumberOfWords);
        };
        let board_str = words[0]; 
        let rows: Vec<&str> = board_str.split('/').collect();
        if rows.len() != 8 {
            return Err(FenParseError::NumberOfRows);
        };
        let mut board = Board::new();
        for i in 0..7 {
            let row = rows[7 - i];
            let chars = row.chars();
            let mut j = 0;
            while j < 8 {
                let c = chars.next();
                let sq = Square::from_coord(i as u8, j as u8).unwrap();
                let piece = letter_to_piece(c)?;
                board.set(sq, piece);
            }
        };
        Ok()
    }
}


