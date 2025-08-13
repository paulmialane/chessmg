#[derive(Debug)]
pub enum ChessMgError {
    InvalidFEN(String),
    InvalidSquare,
}
