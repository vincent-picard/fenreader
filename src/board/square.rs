use crate::board::Color;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Square {
    row: u8,
    col: u8,
}

impl Square {
    pub fn from_coord(row: u8, col: u8) -> Result<Square, &'static str> {
        if row >= 8 {
            Err("Invalid row number")
        } else if col>= 8 {
            Err("Invalid column number")
        } else {
            Ok(Square{row, col})
        }
    }

    pub fn from_algebraic(s: &str) -> Result<Square, &'static str> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return Err("Algebraic coord parse error : string length must be 2");
        }
        let letter = chars[0];
        let digit = chars[1];
        let row_number = match digit {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return Err("Algebraic coord parse error, digit is incorrect"),
        };
        let col_number = match letter {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err("Algebraic coord parse error, letter is incorrect"),
        };
        Self::from_coord(row_number, col_number)
    }

    // Returns an index in [0, 64[, unique to each square
    pub fn to_index(self) -> usize {
        (self.row * 8 + self.col).into()
    }

    pub fn color(self) -> Color {
        let n = self.row + self.col;
        if n % 2 == 0 {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letter = match self.col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("Try to display invalid square"),
        };
        let digit = if self.row < 8 {
            self.row + 1
        } else {
            panic!("Try to display invalid square")
        };
        write!(f, "{}{}", letter, digit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn square_tostring() {
        assert_eq!(Square::from_coord(0, 0).unwrap().to_string(), "a1");
        assert_eq!(Square::from_coord(1, 4).unwrap().to_string(), "e2");
        assert_eq!(Square::from_coord(7, 7).unwrap().to_string(), "h8");
    }

    #[test]
    #[should_panic]
    fn panic_tostring_1() {
        let invalid_square = Square {
            row: 18,
            col: 5,
        };
        println!("{}", invalid_square);
    }

    #[test]
    #[should_panic]
    fn panic_tostring_2() {
        let invalid_square = Square {
            row: 2,
            col: 42,
        };
        println!("{}", invalid_square);
    }

    #[test]
    fn colors_of_squares() {
        assert!(Square::from_coord(0, 0).unwrap().color().is_black());
        assert!(Square::from_coord(1, 1).unwrap().color().is_black());
        assert!(Square::from_coord(0, 2).unwrap().color().is_black());
        assert!(Square::from_coord(0, 7).unwrap().color().is_white());
    }

    #[test]
    fn to_index_is_injective() {
        let mut dest = HashSet::new();
        for i in 0..8 {
            for j in 0..8 {
                let square = Square::from_coord(i, j).unwrap();
                let index = square.to_index();
                assert!(!dest.contains(&index));
                dest.insert(index);
            }
        }
    }

    #[test]
    fn to_index_bounds() {
        for i in 0..8 {
            for j in 0..8 {
                let square = Square::from_coord(i, j).unwrap();
                let index = square.to_index();
                assert!(index < 64);
            }
        }
    }

    #[test]
    fn algebraic_incorrect_input() {
        assert!(Square::from_algebraic("a9").is_err());
        assert!(Square::from_algebraic("A7").is_err());
        assert!(Square::from_algebraic("3a").is_err());
        assert!(Square::from_algebraic("i3").is_err());
        assert!(Square::from_algebraic("k4").is_err());
        assert!(Square::from_algebraic("42").is_err());
        assert!(Square::from_algebraic("h0").is_err());
        assert!(Square::from_algebraic("c-3").is_err());
    }

    #[test]
    fn algebraic_correct_input() {
        assert_eq!(Square::from_algebraic("a1"), Square::from_coord(0, 0));
        assert_eq!(Square::from_algebraic("h1"), Square::from_coord(0, 7));
        assert_eq!(Square::from_algebraic("a8"), Square::from_coord(7, 0));
        assert_eq!(Square::from_algebraic("h8"), Square::from_coord(7, 7));
    }

}

