#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn is_white(&self) -> bool {
        match self {
            Color::White => true,
            Color::Black => false,
        }
    }

    pub fn is_black(&self) -> bool {
        match self {
            Color::White => false,
            Color::Black => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_is_white() {
        assert!(Color::White.is_white());
        assert!(!Color::White.is_black());
    }

    #[test]
    fn black_is_black() {
        assert!(Color::Black.is_black());
        assert!(!Color::Black.is_white());
    }
}
