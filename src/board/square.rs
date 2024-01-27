use crate::board::Color;

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
    pub fn to_index(&self) -> usize {
        (self.row * 8 + self.col).into()
    }

    pub fn color(&self) -> Color {
        let n = self.row + self.col;
        if n % 2 == 0 {
            Color::Black
        } else {
            Color::White
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn colors_of_squares() {
        assert!(Square::from_coord(0, 0).expect("Should be valid coordinates").color().is_black());
        assert!(Square::from_coord(1, 1).expect("Should be valid coordinates").color().is_black());
        assert!(Square::from_coord(0, 2).expect("Should be valid coordinates").color().is_black());
        assert!(Square::from_coord(0, 7).expect("Should be valid coordinates").color().is_white());
    }

    #[test]
    fn to_index_is_injective() {
        let mut dest = HashSet::new();
        for i in 0..8 {
            for j in 0..8 {
                let square = Square::from_coord(i, j).expect("Should be valid coord");
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
                let square = Square::from_coord(i, j).expect("Should be valid coord");
                let index = square.to_index();
                assert!(index < 64);
            }
        }
    }
}

