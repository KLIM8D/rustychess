use crate::error::Error;
use std::fmt::{self, Debug};
use std::str::FromStr;

pub trait PieceMovements {
    fn position(&self) -> u64;
    fn move_p(&mut self) -> bool;
    fn capture(&mut self) -> bool;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl<'a> Kind {
    pub fn pgn(&'a self) -> &'a str {
        match self {
            Kind::Pawn => "P",
            Kind::Bishop => "B",
            Kind::Knight => "N",
            Kind::Rook => "R",
            Kind::Queen => "Q",
            Kind::King => "K",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub kind: Kind,
}

impl Piece {
    pub fn new(k: Kind) -> Box<Piece> {
        Box::new(Piece { kind: k })
    }
}

impl FromStr for Piece {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidPiece);
        }
        match s.chars().next().unwrap() {
            'P' => Ok(*Piece::new(Kind::Pawn)),
            'B' => Ok(*Piece::new(Kind::Bishop)),
            'N' => Ok(*Piece::new(Kind::Knight)),
            'R' => Ok(*Piece::new(Kind::Rook)),
            'Q' => Ok(*Piece::new(Kind::Queen)),
            'K' => Ok(*Piece::new(Kind::King)),
            _ => Err(Error::InvalidFile),
        }
    }
}

impl PieceMovements for Piece {
    fn position(&self) -> u64 {
        println!("{:?}", self.kind);
        0
    }

    fn move_p(&mut self) -> bool {
        match self.kind {
            Kind::Pawn => println!("PAWN"),
            Kind::Bishop => println!("BISHOP"),
            Kind::Knight => println!("KNIGHT"),
            Kind::Rook => println!("ROOK"),
            Kind::Queen => println!("QUEEN"),
            Kind::King => println!("KING"),
        }
        false
    }

    fn capture(&mut self) -> bool {
        false
    }
}
