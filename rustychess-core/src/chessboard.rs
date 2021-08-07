use crate::file::File;
use crate::pgn::Position;
use crate::pgn::PGN;
use crate::pieces::Color;
use crate::pieces::Kind;
use crate::pieces::Piece;
use crate::pieces::PieceMovements;
use crate::rank::Rank;
use crate::rank::ALL_RANKS;
use colored::*;
use hashbrown::HashMap;
use std::collections::HashSet;
use std::error::Error;

/*trait GetSimple {
    fn get(&self, _: &str, _: i8) -> Option<&Box<Piece>>;
}

trait GetEnums {
    fn get(&self, _: Rank, _: File) -> Option<&Box<Piece>>;
}

impl GetSimple for Chessboard2 {}

impl GetEnums for Chessboard2 {
    fn get(&self, rank: Rank, file: File) -> Option<&Box<Piece>> {
        self.board.get(&Position::new(rank.to_str(), file.to_i8()))
    }
}*/

#[derive(Debug, Clone)]
pub struct Chessboard2 {
    board: HashMap<Position, Box<Piece>>,
}

impl Chessboard2 {
    pub fn new() -> Chessboard2 {
        Chessboard2 {
            board: HashMap::new(),
        }
    }

    pub fn set(&mut self, rank: &str, file: i8, v: Box<Piece>) {
        self.board.insert(Position::new(rank, file), v);
    }

    pub fn get(&self, rank: &str, file: i8) -> Option<&Box<Piece>> {
        self.board.get(&Position::new(rank, file))
    }

    pub fn set_position(&mut self, board: Vec<Option<Box<Piece>>>) {
        let mut rank = Rank::A;
        let mut file = File::First;
        for (i, piece) in board.iter().enumerate() {
            if i % 8 == 0 && i > 0 {
                file = file.up();
                rank = Rank::A;
            }

            match piece {
                Some(p) => {
                    self.set(rank.to_str(), file.to_i8(), p.to_owned().clone());
                }
                _ => {}
            }

            rank = rank.right();
        }
    }

    pub fn reset(&mut self) {
        let mut v = Vec::new();
        v.push(Some(Piece::new(Kind::Rook, Color::White)));
        v.push(Some(Piece::new(Kind::Knight, Color::White)));
        v.push(Some(Piece::new(Kind::Bishop, Color::White)));
        v.push(Some(Piece::new(Kind::Queen, Color::White)));
        v.push(Some(Piece::new(Kind::King, Color::White)));
        v.push(Some(Piece::new(Kind::Bishop, Color::White)));
        v.push(Some(Piece::new(Kind::Knight, Color::White)));
        v.push(Some(Piece::new(Kind::Rook, Color::White)));
        for _ in 0..8 {
            v.push(Some(Piece::new(Kind::Pawn, Color::White)));
        }

        for _ in 0..32 {
            v.push(None);
        }

        for _ in 0..8 {
            v.push(Some(Piece::new(Kind::Pawn, Color::Black)));
        }
        v.push(Some(Piece::new(Kind::Rook, Color::Black)));
        v.push(Some(Piece::new(Kind::Knight, Color::Black)));
        v.push(Some(Piece::new(Kind::Bishop, Color::Black)));
        v.push(Some(Piece::new(Kind::Queen, Color::Black)));
        v.push(Some(Piece::new(Kind::King, Color::Black)));
        v.push(Some(Piece::new(Kind::Bishop, Color::Black)));
        v.push(Some(Piece::new(Kind::Knight, Color::Black)));
        v.push(Some(Piece::new(Kind::Rook, Color::Black)));

        self.set_position(v);
    }

    pub fn move_(&mut self, pgn: &str, color: Color) -> Result<(), Box<dyn Error>> {
        let _move = match PGN::parse(pgn, color) {
            Ok(v) => v,
            Err(e) => return Err(Box::new(e)),
        };

        println!("rank: {}", _move[0].position.rank);
        println!("file: {}", _move[0].position.file);
        let from_pos = &Position {
            rank: _move[0].position.rank,
            file: _move[0].position.file,
        };
        let to_pos = &Position {
            rank: _move[1].position.rank,
            file: _move[1].position.file,
        };

        if from_pos == to_pos {
            return Err("Wrong".into());
        }

        let a = self.board.get(from_pos);

        match a {
            Some(ref v) => {
                let mut piece = v.to_owned().clone();
                if piece.color != color {
                    return Err("Wrong color".into());
                }

                println!("Piece: {}", piece);
                let mut moves = piece.possible_moves(*from_pos);
                //println!("({}) moves: {:?}", moves.len(), moves);

                moves = moves
                    .into_iter()
                    .filter(|pos| {
                        let piece = self.board.get(pos);
                        match piece {
                            Some(p) => p.color != color,
                            _ => true,
                        }
                    })
                    .collect();

                let is_valid = moves.contains(to_pos);
                println!("({}) moves: {:?}", moves.len(), moves);
                println!("is_valid: {:?}", is_valid);
                if is_valid {
                    self.board.insert(*to_pos, piece);
                    self.board.remove(from_pos);
                }
            }
            None => println!("empty"),
        }

        Ok(())
    }

    pub fn print(self) {
        for rank in std::array::IntoIter::new(ALL_RANKS) {
            print!("  {}", rank);
        }
        println!();
        for file in (1..9).rev() {
            print!("{}", file);

            for rank in std::array::IntoIter::new(ALL_RANKS) {
                let piece = self.get(rank.to_str(), file);
                match piece {
                    Some(p) => print!("[{}]", p),
                    _ => print!("{}", "[ ]".white()),
                }
            }
            println!();
        }
    }
}
