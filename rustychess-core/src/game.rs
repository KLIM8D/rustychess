use crate::chessboard::BoardStatus;
use crate::chessboard::Chessboard2;
use crate::pgn::Position;
use crate::pieces::Color;
use crate::pieces::Kind;
use crate::pieces::Piece;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub piece: Piece,
    pub capture: Option<Piece>,
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

pub struct Game {
    pub board: Chessboard2,
    turn: Color,
    moves: VecDeque<Move>,
}

impl Game {
    pub fn new() -> Game {
        let mut r = Game {
            board: Chessboard2::new(),
            turn: Color::White,
            moves: VecDeque::with_capacity(90),
        };
        r.board.reset();
        r
    }

    pub fn promote(&mut self, pos: &Position, kind: Kind) {
        self.board.promote(pos, kind, self.turn);
        self.turn = self.turn.switch();
    }

    pub fn add_move(&mut self, m: Move) {
        self.board.remove(m.from);
        self.board.remove(m.to);
        self.board.set_(m.to, Box::new(m.piece));
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

    pub fn move_(&mut self, pgn: &str) -> Result<BoardStatus, Box<dyn Error>> {
        let prev_move = self.moves.back();
        let result = self.board.move_(prev_move, pgn, self.turn);
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
}
