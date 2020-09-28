use chessboard::Chessboard;
use chessboard::A;
use chessboard::B;
use rustychess_core::pgn::Position;
use rustychess_core::pieces::Kind;
use rustychess_core::pieces::Piece;
use rustychess_core::pieces::PieceMovements;
mod chessboard;

fn main() {
    println!("Hello, world!");
    let mut pawn = Piece::new(Kind::Pawn);
    println!("{}", pawn.move_p());
    for i in 0..5 {
        pawn.capture();
    }
    println!("{}", pawn.position());

    let mut board = Chessboard::new();
    board.set("A", 1, pawn);

    let a = board.get(&"A", &1);
    println!("{}", a.unwrap().position())
}
