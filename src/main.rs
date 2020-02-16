use pieces::Kind;
use pieces::Piece;
use chessboard::Chessboard;
use chessboard::A;
use chessboard::B;
mod pieces;
mod chessboard;

struct Position<'a> {
    rank: &'a A,
    file: &'a B,
}
const A1: Position = Position{rank: &A::new(&"a"), file: &B::new(&1)};


fn main() {
    println!("Hello, world!");
    let mut pawn = Piece::new(Kind::Pawn);
    println!("{}", pawn.move_p());
    for i in 0..5 {
        pawn.attack();
    }
    println!("{}", pawn.position());

    let mut board = Chessboard::new();
    board.set(A1.rank, A1.file, pawn);

    let a = board.get(&A1.rank, &A1.file);
    println!("{}", a.unwrap().position())
}
