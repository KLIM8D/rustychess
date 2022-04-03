use crate::file::File;
use crate::game::Move;
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
    status: BoardStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoardStatus {
    None,
    Checkmate,
    Stalemate,
    Promote,
    EnPassant,
}

impl Chessboard2 {
    pub fn new() -> Chessboard2 {
        Chessboard2 {
            board: HashMap::new(),
            status: BoardStatus::None,
        }
    }

    pub fn set_(&mut self, pos: Position, v: Box<Piece>) {
        self.board.insert(pos, v);
    }

    pub fn set(&mut self, rank: &str, file: i8, v: Box<Piece>) {
        self.board.insert(Position::new(rank, file), v);
    }

    pub fn get(&self, rank: &str, file: i8) -> Option<&Box<Piece>> {
        self.board.get(&Position::new(rank, file))
    }

    pub fn remove(&mut self, pos: Position) -> Option<Piece> {
        match self.board.get(&pos) {
            Some(v) => {
                let r = *v.clone();
                self.board.remove(&pos);
                Some(r)
            }
            None => (None),
        }
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

    pub fn promote(&mut self, pos: &Position, kind: Kind, color: Color) {
        let piece = self.board.get(pos);

        match piece {
            Some(_) => {
                self.set(pos.rank.to_str(), pos.file.to_i8(), Piece::new(kind, color));
            }
            None => {}
        }
    }

    pub fn move_(
        &mut self,
        previous_move: Option<&Move>,
        notation: &str,
        color: Color,
    ) -> Result<(Move, BoardStatus), Box<dyn Error>> {
        let mut status = BoardStatus::None;
        let _move = match PGN::parse(notation, color) {
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

                    let mut r = Vec::new();
                    r.append(
                        &mut diagonal
                            .into_iter()
                            .filter(|pos| {
                                let piece = self.board.get(pos);
                                match piece {
                                    Some(p) => p.color != color,
                                    _ => false,
                                }
                            })
                            .collect(),
                    );

                    r
                } else {
                    Vec::new()
                };

                moves.append(&mut possible_captures);

                //let shortest_path = from_pos.shortest_path(to_pos);
                let path: Vec<Position> = from_pos
                    .shortest_path(*to_pos)
                    .into_iter()
                    .filter(|pos| moves.contains(pos))
                    .collect();
                let is_blocking = path.iter().any(|pos| {
                    let piece = self.board.get(pos);
                    match piece {
                        Some(p) => p.color == color,
                        _ => false,
                    }
                });
                println!("shortest_path: {:?}", path);
                println!("is_blocking: {:?}", is_blocking);
                if is_blocking {
                    return Err("Invalid move. Own piece blocking".into());
                }

                let mut m = Move {
                    from: *from_pos,
                    to: *to_pos,
                    piece: *piece,
                    capture: None,
                };

                let is_enpassant = !(previous_move.is_none())
                    && self.is_enpassant(previous_move.unwrap(), *piece, to_pos);
                if is_enpassant {
                    moves.push(*to_pos);

                    let enpassant_piece_pos = if color == Color::White {
                        Position::new_(to_pos.rank, to_pos.file.down())
                    } else {
                        Position::new_(to_pos.rank, to_pos.file.up())
                    };

                    let captured_piece = self.board.get(&enpassant_piece_pos);
                    m.capture = match captured_piece {
                        Some(p) => Some(**p),
                        _ => None,
                    };
                    self.board.remove(&enpassant_piece_pos);
                    println!("en passant possible")
                }

                let to_piece = self.board.get(to_pos);
                let is_capture = match to_piece {
                    Some(p) => {
                        if p.color != color {
                            m.capture = Some(**p);
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

                println!("{:#?}", m);

                let is_valid = moves.contains(to_pos);
                println!("({}) moves: {:?}", moves.len(), moves);
                println!("is_valid: {:?}", is_valid);

                if is_valid {
                    if self.can_promote(color, to_pos) {
                        status = BoardStatus::Promote;
                        println!("can promote")
                    }
                } else {
                    return Err("Invalid move".into());
                }

                self.status = status;
                Ok((m, self.status))
            }
            None => return Err("Invalid move. Field empty!".into()),
        }
    }

    pub fn is_enpassant(&self, previous_move: &Move, piece: Piece, to: &Position) -> bool {
        if previous_move.piece.kind != Kind::Pawn || piece.kind != Kind::Pawn {
            return false;
        }
        let white_pawn_4_file = previous_move.piece.color == Color::White
            && previous_move.from.file == File::Second
            && previous_move.to.file == File::Fourth;

        let black_pawn_6_file = previous_move.piece.color == Color::Black
            && previous_move.from.file == File::Seventh
            && previous_move.to.file == File::Fifth;

        let is_neightbour_file = to.file.sub(previous_move.to.file) == 1;
        let move_to_same_rank = previous_move.to.rank == to.rank;

        println!("white_pawn_4_file: {:?}", white_pawn_4_file);
        println!("black_pawn_6_file: {:?}", black_pawn_6_file);
        println!("is_neightbour_file: {:?}", is_neightbour_file);
        println!("move_to_same_rank: {:?}", move_to_same_rank);
        return (white_pawn_4_file || black_pawn_6_file) && is_neightbour_file && move_to_same_rank;
    }

    pub fn can_promote(&self, color: Color, pos: &Position) -> bool {
        if color == Color::White {
            return pos.file == File::Eighth;
        }

        pos.file == File::First
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
                let mut piece = *p.1.clone();
                let mut possible_moves = piece.possible_moves(*opononent_position);
                let mut possible_captures = if piece.kind == Kind::Pawn {
                    let diagonal = opononent_position.diagonals_squares(1);
                    diagonal
                        .into_iter()
                        .filter(|pos| {
                            let piece = self.board.get(pos);
                            match piece {
                                Some(pp) => pp.color == color,
                                _ => false,
                            }
                        })
                        .collect()
                } else {
                    Vec::new()
                };
                possible_moves.append(&mut possible_captures);

                let shortest_path: Vec<Position> = opononent_position
                    .shortest_path(king_pos)
                    .into_iter()
                    .filter(|pos| possible_moves.contains(pos))
                    .collect();

                let can_capture = shortest_path.iter().any(|&x| x == king_pos);
                let is_blocking = shortest_path.iter().any(|pos| {
                    let piece = self.board.get(pos);
                    if *pos == king_pos {
                        return false;
                    }
                    match piece {
                        Some(_) => true,
                        _ => false,
                    }
                });

                if can_capture && !is_blocking {
                    println!("Kind: {:?}", piece.kind);
                    println!("Path: {:?}", shortest_path);
                    println!("can_capture: {:?}", can_capture);
                    println!("is_blocking: {:?}", is_blocking);
                    result = true;
                    return;
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
