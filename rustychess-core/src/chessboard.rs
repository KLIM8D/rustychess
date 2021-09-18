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

    pub fn find_piece(
        &self,
        kind: Kind,
        color: Color,
    ) -> Result<(Position, Piece), Box<dyn Error>> {
        let a = self
            .board
            .iter()
            .find(|(_, v)| v.color == color && v.kind == kind);

        match a {
            Some(v) => Ok((*v.0, *v.1.clone())),
            None => return Err("piece not found".into()),
        }
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

    pub fn move_(&mut self, pgn: &str, color: Color) -> Result<Vec<Piece>, Box<dyn Error>> {
        let mut r = Vec::new();
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
                println!("({}) moves: {:?}", moves.len(), moves);

                // removes squares (moves) that contains a piece with the same color
                // eg. whites turn, remove all squares from moves that contains a white piece
                moves = moves
                    .into_iter()
                    .filter(|pos| {
                        let square = self.board.get(pos);
                        match square {
                            Some(p) => p.color != color && piece.kind != Kind::Pawn,
                            _ => true,
                        }
                    })
                    .collect();

                let mut possible_captures = if piece.kind == Kind::Pawn {
                    let diagonal = from_pos.diagonals_squares(1);
                    diagonal
                        .into_iter()
                        .filter(|pos| {
                            let piece = self.board.get(pos);
                            match piece {
                                Some(p) => p.color != color,
                                _ => false,
                            }
                        })
                        .collect()
                } else {
                    Vec::new()
                };

                moves.append(&mut possible_captures);

                let path = from_pos.shortest_path(*to_pos);
                let is_blocking = path.iter().any(|pos| {
                    let piece = self.board.get(pos);
                    match piece {
                        Some(_) => true,
                        _ => false,
                    }
                });
                println!("shortest_path: {:?}", path);
                println!("is_blocking: {:?}", is_blocking);
                if is_blocking {
                    return Err("Invalid move. Own piece blocking".into());
                }

                let to_piece = self.board.get(to_pos);
                let is_capture = match to_piece {
                    Some(p) => {
                        if p.color != color {
                            r.push(*p.to_owned());
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                };
                if is_capture {
                    println!("capture")
                }

                let is_valid = moves.contains(to_pos);
                println!("({}) moves: {:?}", moves.len(), moves);
                println!("is_valid: {:?}", is_valid);
                if is_valid {
                    self.board.insert(*to_pos, piece.clone());
                    self.board.remove(from_pos);

                    let is_checked = self.is_checked(color);
                    if is_checked {
                        self.board.insert(*from_pos, piece);
                        self.board.remove(to_pos);

                        return Err("Invalid move".into());
                    }
                } else {
                    return Err("Invalid move".into());
                }
            }
            None => println!("empty"),
        }

        Ok(r)
    }

    pub fn is_checked(&self, color: Color) -> bool {
        let found = self.find_piece(Kind::King, color);
        let king_pos;
        match found {
            Ok(v) => {
                king_pos = v.0;
                //king = v.1;
            }
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }

        let mut result = false;
        self.board
            .iter()
            .filter(|(_, v)| v.color != color)
            .for_each(|p| {
                let opononent_position = p.0;
                //let piece = p.1;
                let path = opononent_position.shortest_path(king_pos);
                let is_blocking = path.iter().any(|pos| {
                    let piece = self.board.get(pos);
                    match piece {
                        Some(_) => true,
                        _ => false,
                    }
                });

                if !is_blocking {
                    result = true
                }
            });

        result
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
