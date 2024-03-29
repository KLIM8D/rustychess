use crate::chessboard::BoardStatus;
use crate::chessboard::Chessboard2;
use crate::pgn::Position;
use crate::pieces::Color;
use crate::pieces::Kind;
use crate::pieces::Piece;
use chrono::{DateTime, Local, Utc};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{self, Debug};
use std::io::{Result as ioResult, Write};

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub piece: Piece,
    pub capture: Option<Piece>,
    pub is_from_orignal_pos: bool,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind_str = if self.piece.kind != Kind::Pawn {
            self.piece.kind.pgn()
        } else {
            " "
        };

        write!(
            f,
            "{}{}{}{}{}     Color: {:?} - Piece: {:?} - Capture: {:?}",
            kind_str,
            self.from.rank.to_str(),
            self.from.file.to_i8(),
            self.to.rank.to_str(),
            self.to.file.to_i8(),
            self.piece.color,
            self.piece.kind,
            self.capture
        )
    }
}

impl Move {
    fn pgn(&self) -> String {
        let kind_str = if self.piece.kind != Kind::Pawn {
            self.piece.kind.pgn()
        } else {
            ""
        };

        let capture_str = match self.capture {
            Some(c) => c.to_string(),
            None => " ".to_string(),
        };

        let has_moved_str = if self.piece.kind != Kind::Pawn && self.is_from_orignal_pos {
            self.from.rank.to_str()
        } else {
            ""
        };

        format!(
            "{}{}{}{}{}",
            kind_str,
            has_moved_str,
            self.to.rank.to_str(),
            self.to.file.to_i8(),
            capture_str
        )
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub board: Chessboard2,
    turn: Color,
    moves: VecDeque<Move>,
    metadata: HashMap<String, String>,
}

impl Game {
    pub fn new() -> Game {
        let mut r = Game {
            board: Chessboard2::new(),
            turn: Color::White,
            moves: VecDeque::with_capacity(90),
            metadata: HashMap::new(),
        };
        r.board.reset();
        r.insert_metadata(
            "created_at".to_string(),
            Local::now().format("%d-%m-%Y %H:%M").to_string(),
        );
        r
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.metadata.clear();
        self.turn = Color::White;
        self.moves.clear();

        self.insert_metadata(
            "created_at".to_string(),
            Local::now().format("%d-%m-%Y %H:%M").to_string(),
        );
    }

    pub fn insert_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn print_metadata(self) {
        for (key, value) in self.metadata {
            println!("{}: {}", key, value);
        }
    }

    pub fn promote(&mut self, pos: &Position, kind: Kind) {
        self.board.promote(pos, kind, self.turn);
        self.turn = self.turn.switch();
    }

    pub fn number_of_moves(self) -> usize {
        return self.moves.len();
    }

    pub fn add_move(&mut self, mut m: Move) {
        self.board.remove(m.from);
        self.board.remove(m.to);
        m.piece.number_of_moves += 1;
        println!("moves: {}", m.piece.number_of_moves);
        self.board.set_(m.to, Box::new(m.piece));
        m.is_from_orignal_pos = m.piece.number_of_moves == 1;
        self.moves.push_back(m);
    }

    pub fn printmoves(&self) {
        for m in self.moves.iter() {
            println!("{}", m)
        }
    }

    pub fn rollback_move(&mut self) {
        let m = self.moves.pop_back();
        println!("{:#?}", m);
        match m {
            Some(mm) => {
                self.board.set_(mm.from, Box::new(mm.piece));
                match mm.capture {
                    Some(c) => self.board.set_(mm.to, Box::new(c)),
                    None => {}
                }
            }
            None => {}
        }
    }

    pub fn count_moves(
        &self,
        piece: &Piece
    ) -> usize {
        self
            .moves
            .iter()
            .filter(|m| m.piece == *piece)
            .count()
    }

    pub fn find_move(
        &self,
        piece: &Piece,
        from: Option<&Position>,
        to: Option<&Position>,
    ) -> Option<Move> {
        let m = self
            .moves
            .iter()
            .filter(|m| m.piece == *piece)
            .find(|m| match (from, to) {
                (Some(from), Some(to)) => m.from == *from && m.to == *to,
                (Some(from), _) => m.from == *from,
                (_, Some(to)) => m.to == *to,
                (_, _) => true,
            });

        match m {
            Some(ma) => Some(*ma),
            _ => None,
        }
    }

    pub fn move_(&mut self, pgn: &str) -> Result<BoardStatus, Box<dyn Error>> {
        let prev_move = self.moves.back();
        let can_castle = self.can_castle(self.turn);
        let result = self.board.move_(prev_move, pgn, self.turn, can_castle);
        match result {
            Ok(r) => {
                self.add_move(r.0);
                let is_checked = self.board.is_checked(self.turn);
                if !is_checked {
                    match r.1 {
                        BoardStatus::Promote => {
                            return Ok(r.1);
                        }
                        _ => {}
                    }

                    self.turn = self.turn.switch();
                } else {
                    self.rollback_move();
                    println!("Invalid move. Move leads to check");
                }
            }
            Err(e) => println!("{}", e),
        }
        println!("{:?}", self.turn);
        Ok(BoardStatus::None)
    }

    fn can_castle(&self, color: Color) -> bool {
        if self.board.is_checked(color) {
            return false;
        }

        let a_rook_pos = if color == Color::White {
            Position::new("A", 1)
        } else {
            Position::new("A", 8)
        };

        let king_pos = if color == Color::White {
            Position::new("E", 1)
        } else {
            Position::new("E", 8)
        };
        let h_rook_pos = if color == Color::White {
            Position::new("H", 1)
        } else {
            Position::new("H", 8)
        };

        let a_rook_has_moved = self.find_move(
            &Piece::new(Kind::Rook, Color::White),
            Some(&a_rook_pos),
            None,
        );

        let king_has_moved =
            self.find_move(&Piece::new(Kind::King, Color::White), Some(&king_pos), None);

        let h_rook_has_moved = self.find_move(
            &Piece::new(Kind::Rook, Color::White),
            Some(&h_rook_pos),
            None,
        );

        if !king_has_moved.is_none() || !a_rook_has_moved.is_none() | !h_rook_has_moved.is_none() {
            return false;
        }

        let short_side: Vec<Position> = king_pos.shortest_path(a_rook_pos);
        let long_side: Vec<Position> = king_pos.shortest_path(h_rook_pos);

        let c: Vec<Position> = short_side.iter().chain(&long_side).copied().collect();

        let any_field_threatened = c.iter().any(|&field| {
            self.clone()
                .board
                .is_field_threatened(color, &field.clone())
        });

        let is_blocking = c.iter().any(|pos| {
            let piece = self.board.get_with_pos(pos);
            match piece {
                Some(_) => true,
                _ => false,
            }
        });

        return any_field_threatened && !is_blocking;
    }

    pub fn save(&mut self, writer: &mut dyn Write) -> ioResult<()> {
        let mut counter = 1;

        let (front_slice, back_slice) = self.moves.as_slices();

        for i in (0..front_slice.len()).step_by(2) {
            if let Some(first) = front_slice.get(i) {
                writer.write_all(format!("{}. ", counter).as_bytes())?;

                writer.write_all(first.pgn().as_bytes())?;
            }

            if let Some(second) = front_slice.get(i + 1) {
                writer.write_all(second.pgn().as_bytes())?;
            } else {
                if let Some(second) = i
                    .checked_sub(front_slice.len())
                    .and_then(|idx| back_slice.get(idx))
                {
                    writer.write_all(second.pgn().as_bytes())?;
                }
            }
            writer.write(b" ")?;

            counter += 1;
        }

        writer.flush()?;

        Ok(())
    }
}
