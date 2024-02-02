use crate::board::Board;
use crate::board::color::Color;
use crate::board::piece::{ColoredPiece, Piece};
use crate::board::square::Square;
use std::num::ParseIntError;

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
    ParseIntError(ParseIntError),
    ParseTurn,
    ParseCastlePossibilities,
}

impl From<ParseIntError> for FenParseError {
   fn from(e: ParseIntError) -> Self {
       FenParseError::ParseIntError(e)
   }
}

fn letter_to_piece(c: char) -> Result<ColoredPiece, FenParseError> {
    Ok (match c {
        'P' => ColoredPiece(Color::White, Piece::Pawn),
        'N' => ColoredPiece(Color::White, Piece::Knight),
        'B' => ColoredPiece(Color::White, Piece::Bishop),
        'R' => ColoredPiece(Color::White, Piece::Rook),
        'Q' => ColoredPiece(Color::White, Piece::Queen),
        'K' => ColoredPiece(Color::White, Piece::King),
        'p' => ColoredPiece(Color::Black, Piece::Pawn),
        'n' => ColoredPiece(Color::Black, Piece::Knight),
        'b' => ColoredPiece(Color::Black, Piece::Bishop),
        'r' => ColoredPiece(Color::Black, Piece::Rook),
        'q' => ColoredPiece(Color::Black, Piece::Queen),
        'k' => ColoredPiece(Color::Black, Piece::King),
        _ => return Err(FenParseError::UnknownPiece(c)),
    })
}

fn string_into_board(s: &str) -> Result<Board, FenParseError> {
    let mut board = Board::new();

    let rows: Vec<&str> = s.split('/').collect();
    if rows.len() != 8 {
        return Err(FenParseError::NumberOfRows);
    };
    for i in 0..8 {
        let row = rows[7 - i];
        let mut chars = row.chars();
        let mut j = 0;
        while j < 8 {
            let c = chars.next().ok_or_else(|| FenParseError::IncorrectRow(j))?;
            if c.is_digit(10) {
                let d = c.to_digit(10).unwrap();
                if d == 0 || d > 8 {
                    return Err(FenParseError::IncorrectRow(i));
                } else {
                    j += d;
                    continue;
                }
            } else {
                    let sq = Square::from_coord(i as u8, j as u8).unwrap();
                    let piece = letter_to_piece(c)?;
                    board.set(sq, piece);
            }
        }
        if j != 8 {
            return Err(FenParseError::IncorrectRow(i));
        }
    };
    Ok(board)
}

fn word_into_turn(w : &str) -> Result<Color, FenParseError> {
    let mut chars: Vec<char> = w.chars().collect();
    if chars.len() != 1 {
        return Err(FenParseError::ParseTurn);
    };
    match chars[0] {
        'w' => Ok(Color::White),
        'b' => Ok(Color::Black),
    }
}

fn word_into_castles(w: &str) -> Result<(bool, bool, bool, bool), FenParseError> { 
    let mut chars: Vec<char> = w.chars().collect();
    let n = chars.len();
    if n > 5 || n == 0 {
        return Err(FenParseError::ParseCastlePossibilities);
    };
    let mut white_short = false;
    let mut white_long = false;
    let mut black_short = false;
    let mut black_long = false;
    for c in chars {
        match c {
            'K' => white_short = true,
            'Q' => white_long = true,
            'k' => black_short = true,
            'q' => black_long = true,
            '-' if n==1 => return Ok((false, false, false, false)),
            _ => return Err(FenParseError::ParseCastlePossibilities),
        };
    };
    Ok((white_short, white_long, black_short, black_long))
}

impl Fen {
    pub fn parse(s: &str) -> Result<Self, FenParseError> {
        let words: Vec<&str> = s.split(' ').collect();
        if words.len() != 6 {
            return Err(FenParseError::NumberOfWords);
        };
        let board = string_into_board(words[0])?;
        let turn = word_into_turn(words[1])?;
        let (white_short_castle, white_long_castle, black_short_castle, black_long_castle) = word_into_castles(word[2])?;
        let fen = Fen {
            board,
            turn,
            white_short_castle,
            white_long_castle,
            black_short_castle,
            black_long_castle,
        }
        Ok(fen)
    }
}


