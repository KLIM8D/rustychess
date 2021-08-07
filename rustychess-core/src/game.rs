use crate::chessboard::Chessboard2;
use crate::pieces::Color;
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

    pub fn move_(&mut self, pgn: &str) -> Result<(), Box<dyn Error>> {
        let m = self.board.move_(pgn, self.turn);
        match m {
            Ok(()) => {
                println!("godt");
                self.turn = self.turn.switch();
            }
            Err(e) => println!("{}", e),
        }
        println!("{:?}", self.turn);
        Ok(())
    }
}
