use crate::error::Error;
use crate::file::File;
use crate::file::ALL_FILES;
use crate::pgn::Position;
use crate::rank::ALL_RANKS;
use std::collections::HashSet;
use std::fmt::{self, Debug};
use std::str::FromStr;

pub trait PieceMovements {
    fn position(&self) -> u64;
    fn possible_moves(&mut self, position: Position) -> Vec<Position>;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
    Unknown,
}

impl Color {
    pub fn switch(self) -> Color {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
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

    pub fn symbol(&'a self, c: Color) -> &'a str {
        match self {
            Kind::Pawn => {
                if c == Color::White {
                    "♙"
                } else {
                    "♟︎"
                }
            }
            Kind::Bishop => {
                if c == Color::White {
                    "♗"
                } else {
                    "♝"
                }
            }
            Kind::Knight => {
                if c == Color::White {
                    "♘"
                } else {
                    "♞"
                }
            }
            Kind::Rook => {
                if c == Color::White {
                    "♖"
                } else {
                    "♜"
                }
            }
            Kind::Queen => {
                if c == Color::White {
                    "♕"
                } else {
                    "♛"
                }
            }
            Kind::King => {
                if c == Color::White {
                    "♔"
                } else {
                    "♚"
                }
            }
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
    pub number_of_moves: i8,
}

impl Piece {
    pub fn new(k: Kind, c: Color) -> Box<Piece> {
        Box::new(Piece { kind: k, color: c, number_of_moves: 0 })
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind.symbol(self.color))
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Piece {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 1 {
            return Err(Error::InvalidPiece);
        }
        match s.chars().next().unwrap() {
            'P' => Ok(*Piece::new(Kind::Pawn, Color::Unknown)),
            'B' => Ok(*Piece::new(Kind::Bishop, Color::Unknown)),
            'N' => Ok(*Piece::new(Kind::Knight, Color::Unknown)),
            'R' => Ok(*Piece::new(Kind::Rook, Color::Unknown)),
            'Q' => Ok(*Piece::new(Kind::Queen, Color::Unknown)),
            'K' => Ok(*Piece::new(Kind::King, Color::Unknown)),
            _ => Err(Error::InvalidPiece),
        }
    }
}

impl PieceMovements for Piece {
    fn position(&self) -> u64 {
        println!("{:?}", self.kind);
        0
    }

    fn possible_moves(&mut self, position: Position) -> Vec<Position> {
        let mut r = Vec::new();
        match self.kind {
            Kind::Pawn => match self.color {
                Color::White => {
                    if position.file == File::Second {
                        r.push(Position {
                            rank: position.rank,
                            file: File::Fourth,
                        });
                    }

                    r.push(Position {
                        rank: position.rank,
                        file: position.file.up(),
                    });
                }
                Color::Black => {
                    if position.file == File::Seventh {
                        r.push(Position {
                            rank: position.rank,
                            file: File::Fifth,
                        });
                    }

                    r.push(Position {
                        rank: position.rank,
                        file: position.file.down(),
                    });
                }
                Color::Unknown => {}
            },
            Kind::Bishop => {
                let mut positions = vec![
                    position.clone(),
                    position.clone(),
                    position.clone(),
                    position.clone(),
                ];

                for _ in 0..8 {
                    {
                        let mut pos = positions[0];
                        let (rank, file) = (pos.rank.right(), pos.file.up());
                        if rank > pos.rank && file > pos.file {
                            pos.rank = rank;
                            pos.file = file;

                            positions[0] = pos;
                            r.push(pos.clone());
                        }
                    }

                    {
                        let mut pos = positions[1];
                        let (rank, file) = (pos.rank.left(), pos.file.up());
                        if rank < pos.rank && file > pos.file {
                            pos.rank = rank;
                            pos.file = file;

                            positions[1] = pos;
                            r.push(pos.clone());
                        }
                    }

                    {
                        let mut pos = positions[2];
                        let (rank, file) = (pos.rank.right(), pos.file.down());
                        if rank > pos.rank && file < pos.file {
                            pos.rank = rank;
                            pos.file = file;

                            positions[2] = pos;
                            r.push(pos.clone());
                        }
                    }

                    {
                        let mut pos = positions[3];
                        let (rank, file) = (pos.rank.left(), pos.file.down());
                        if rank < pos.rank && file < pos.file {
                            pos.rank = rank;
                            pos.file = file;

                            positions[3] = pos;
                            r.push(pos.clone());
                        }
                    }
                }
            }
            Kind::Knight => {
                let s0 = position.squares_around(1);
                let mut p0 = s0.clone();
                for s in s0.iter() {
                    p0.push(s.clone());
                    p0.append(&mut s.squares_around(1));
                }

                let mut p0: HashSet<Position> = p0.into_iter().collect();
                let p1: HashSet<Position> = position.squares_around(2).into_iter().collect();
                let p2: HashSet<Position> = position.side_squares(2).into_iter().collect();
                let p3: HashSet<Position> = position.diagonals_squares(2).into_iter().collect();
                let p4: HashSet<Position> = position.squares_around(1).into_iter().collect();

                p0.retain(|p| {
                    !(p1.contains(p)
                        || p2.contains(p)
                        || p3.contains(p)
                        || p4.contains(p)
                        || p.eq(&position))
                });
                r.append(&mut p0.into_iter().collect());
            }
            Kind::Rook => {
                for rank in IntoIterator::into_iter(ALL_RANKS) {
                    if rank != position.rank {
                        r.push(Position {
                            rank,
                            file: position.file,
                        });
                    }
                }

                for file in IntoIterator::into_iter(ALL_FILES) {
                    if file != position.file {
                        r.push(Position {
                            rank: position.rank,
                            file,
                        });
                    }
                }
            }
            Kind::Queen => {
                let mut positions = vec![
                    position.clone(),
                    position.clone(),
                    position.clone(),
                    position.clone(),
                ];

                for _ in 0..8 {
                    {
                        let mut pos = positions[0];
                        let (rank, file) = (pos.rank.right(), pos.file.up());
                        if rank > pos.rank && file > pos.file {
                            pos.rank = rank;
                            pos.file = file;

                            positions[0] = pos;
                            r.push(pos.clone());
                        }
                    }

                    {
                        let mut pos = positions[1];
                        let (rank, file) = (pos.rank.left(), pos.file.up());
                        if rank < pos.rank && file > pos.file {
                            pos.rank = rank;
                            pos.file = file;

                            positions[1] = pos;
                            r.push(pos.clone());
                        }
                    }

                    {
                        let mut pos = positions[2];
                        let (rank, file) = (pos.rank.right(), pos.file.down());
                        if rank > pos.rank && file < pos.file {
                            pos.rank = rank;
                            pos.file = file;

                            positions[2] = pos;
                            r.push(pos.clone());
                        }
                    }

                    {
                        let mut pos = positions[3];
                        let (rank, file) = (pos.rank.left(), pos.file.down());
                        if rank < pos.rank && file < pos.file {
                            pos.rank = rank;
                            pos.file = file;

                            positions[3] = pos;
                            r.push(pos.clone());
                        }
                    }
                }

                for rank in IntoIterator::into_iter(ALL_RANKS) {
                    if rank != position.rank {
                        r.push(Position {
                            rank,
                            file: position.file,
                        });
                    }
                }

                for file in IntoIterator::into_iter(ALL_FILES) {
                    if file != position.file {
                        r.push(Position {
                            rank: position.rank,
                            file,
                        });
                    }
                }
            }
            Kind::King => {
                r.append(&mut position.squares_around(1));
            }
        }
        r
    }

    fn capture(&mut self) -> bool {
        false
    }
}
