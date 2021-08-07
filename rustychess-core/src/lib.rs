pub mod pgn;
pub use pgn::*;

pub mod pieces;
pub use pieces::*;

pub mod file;
pub use crate::file::*;

pub mod rank;
pub use crate::rank::*;

pub mod error;
pub use crate::error::*;

pub mod chessboard;
pub use crate::chessboard::*;

pub mod game;
pub use crate::game::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
