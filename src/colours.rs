use std::ops::Not;

#[derive(Copy, Clone, Debug)]
pub enum Colour {
    White,
    Black
}

impl Not for Colour {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White
        }
    }
}