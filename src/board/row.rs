use super::square::Square;

#[derive(Clone, Copy, Debug)]
pub struct Row {
    i: u8,
}

impl Row {
    pub fn new(i: u8) -> Result<Self, &'static str> {
        if i >= 8 {
            Err("Unexpected row number")
        } else {
            Ok(Row {i})
        }
    }
}

pub struct RowIterator {
    row: u8, 
    col: u8,
}

impl Iterator for RowIterator { 
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < 8 {
            let sq = Square::from_coord(self.row, self.col).unwrap();
            self.col += 1;
            Some(sq)
        } else {
            None
        }
    }

}

impl IntoIterator for Row {
    type Item = Square;
    type IntoIter = RowIterator;

    fn into_iter(self) -> RowIterator {
        RowIterator {
            row: self.i,
            col: 0,
        }
    }
}

