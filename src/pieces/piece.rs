#[derive(Debug)]
pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

pub trait PieceMovements {
    fn position(&self) -> u64;
    fn move_p(&mut self) -> bool;
    fn attack(&mut self) -> bool;
}

