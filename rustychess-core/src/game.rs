use crate::chessboard::BoardStatus;
use crate::chessboard::Chessboard2;
use crate::pgn::Position;
use crate::pieces::Color;
use crate::pieces::Kind;
use std::error::Error;

pub struct Game {
    pub board: Chessboard2,
    turn: Color,
}

impl Game {
    pub fn new() -> Game {
        let mut r = Game {
            board: Chessboard2::new(),
            turn: Color::White,
        };
        r.board.reset();
        r
    }

    pub fn promote(&mut self, pos: &Position, kind: Kind) {
        self.board.promote(pos, kind, self.turn);
        self.turn = self.turn.switch();
    }

    pub fn move_(&mut self, pgn: &str) -> Result<BoardStatus, Box<dyn Error>> {
        let result = self.board.move_(pgn, self.turn);
        match result {
            Ok(r) => {
                println!("godt");
                match r.1 {
                    BoardStatus::Promote => {
                        return Ok(r.1);
                    }
                    _ => {}
                }

                self.turn = self.turn.switch();
            }
            Err(e) => println!("{}", e),
        }
        println!("{:?}", self.turn);
        Ok(BoardStatus::None)
    }
}
