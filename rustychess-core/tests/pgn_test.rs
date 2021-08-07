use rustychess_core::Kind;
use rustychess_core::Rank;
use rustychess_core::Rank;
use rustychess_core::PGN;
use std::str::FromStr;

#[test]
fn test_parse_pawn() {
    let _move = PGN::parse("e2e4");

    assert_eq!(_move.len(), 2);

    let from_move = &_move[0];
    assert_eq!(from_move.piece.kind, Kind::Pawn);
    assert_eq!(from_move.position.file, Rank::from_str("e").unwrap());
    assert_eq!(from_move.position.rank, Rank::from_str("2").unwrap());

    let to_move = &_move[1];
    assert_eq!(to_move.piece.kind, Kind::Pawn);
    assert_eq!(to_move.position.file, Rank::from_str("e").unwrap());
    assert_eq!(to_move.position.rank, Rank::from_str("4").unwrap());
}

#[test]
fn test_parse_queen() {
    let _move = PGN::parse("Qe2e4");

    assert_eq!(_move.len(), 2);

    let from_move = &_move[0];
    assert_eq!(from_move.piece.kind, Kind::Queen);
    assert_eq!(from_move.position.file, Rank::from_str("e").unwrap());
    assert_eq!(from_move.position.rank, Rank::from_str("2").unwrap());

    let to_move = &_move[1];
    assert_eq!(to_move.piece.kind, Kind::Queen);
    assert_eq!(to_move.position.file, Rank::from_str("e").unwrap());
    assert_eq!(to_move.position.rank, Rank::from_str("4").unwrap());
}

#[test]
fn test_parse_all_pieces() {
    let pgns = ["e2e4", "Pe2e4", "Be2e4", "Ne2e4", "Re2e4", "Qe2e4", "Ke2e4"];
    let pieces = [
        Kind::Pawn,
        Kind::Pawn,
        Kind::Bishop,
        Kind::Knight,
        Kind::Rook,
        Kind::Queen,
        Kind::King,
    ];

    for (i, p) in pgns.iter().enumerate() {
        let _move = PGN::parse(p);

        assert_eq!(_move.len(), 2);

        let from_move = &_move[0];
        assert_eq!(from_move.piece.kind, pieces[i]);
        assert_eq!(from_move.position.file, Rank::from_str("e").unwrap());
        assert_eq!(from_move.position.rank, Rank::from_str("2").unwrap());

        let to_move = &_move[1];
        assert_eq!(to_move.piece.kind, pieces[i]);
        assert_eq!(to_move.position.file, Rank::from_str("e").unwrap());
        assert_eq!(to_move.position.rank, Rank::from_str("4").unwrap());
    }
}
