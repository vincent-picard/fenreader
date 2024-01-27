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
