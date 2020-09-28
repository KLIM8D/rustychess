use crate::file::File;
use crate::pieces::Kind;
use crate::pieces::Piece;
use crate::rank::Rank;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Position {
    pub rank: Rank,
    pub file: File,
}

#[derive(Clone, Copy)]
pub struct PGN {}

pub struct Move {
    pub piece: Piece,
    pub position: Position,
}

impl PGN {
    // Valid formats:
    // Qh4e1 -> move Queen from h4 to e1
    // e2e4 -> move pawn from e2 to e4
    pub fn parse(m: &'static str) -> Vec<Move> {
        let mut r = Vec::new();

        let piece = match m.len() {
            0..=4 => 'P',
            5 => m.chars().next().unwrap(),
            _ => panic!("a"),
        };

        let from_file = match m.len() {
            0..=4 => m.chars().next().unwrap(),
            5 => m.chars().nth(1).unwrap(),
            _ => panic!("a"),
        };

        let from_rank = match m.len() {
            0..=4 => m.chars().nth(1).unwrap(),
            5 => m.chars().nth(2).unwrap(),
            _ => panic!("a"),
        };

        let to_file = match m.len() {
            0..=4 => m.chars().nth(2).unwrap(),
            5 => m.chars().nth(3).unwrap(),
            _ => panic!("a"),
        };
        let to_rank = match m.len() {
            0..=4 => m.chars().nth(3).unwrap(),
            5 => m.chars().nth(4).unwrap(),
            _ => panic!("a"),
        };

        r.push(Move {
            piece: match Piece::from_str(&piece.to_string()) {
                Ok(v) => v,
                Err(e) => panic!("error piece"),
            },
            position: Position {
                file: match File::from_str(&from_file.to_string()) {
                    Ok(v) => v,
                    Err(e) => panic!("error file"),
                },
                rank: match Rank::from_str(&from_rank.to_string()) {
                    Ok(v) => v,
                    Err(e) => panic!("error rank"),
                },
            },
        });

        r.push(Move {
            piece: match Piece::from_str(&piece.to_string()) {
                Ok(v) => v,
                Err(e) => panic!("error piece"),
            },
            position: Position {
                file: match File::from_str(&to_file.to_string()) {
                    Ok(v) => v,
                    Err(e) => panic!("error file"),
                },
                rank: match Rank::from_str(&to_rank.to_string()) {
                    Ok(v) => v,
                    Err(e) => panic!("error rank"),
                },
            },
        });
        r
    }
}
