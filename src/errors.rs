#[derive(Debug)]
pub struct InvalidFenError {
    pub fen: String
}

#[derive(Debug)]
pub struct IllegalMoveError;