use crate::pieces::Kind;
use crate::pieces::PieceMovements;

pub struct Piece {
    pub kind: Kind
}

impl Piece {
    pub fn new(k: Kind) -> Box<Piece> {
        Box::new(Piece {kind: k})
    }
}

impl PieceMovements for Piece {
    fn position(&self) -> u64 {
        println!("{:?}", self.kind);
        0
    }

    fn move_p(&mut self) -> bool {
        false
    }

    fn attack(&mut self) -> bool {
        false
    }
}
