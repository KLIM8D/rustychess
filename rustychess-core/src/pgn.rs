use crate::file::File;
use crate::my_reader;
use crate::pieces::Piece;
use crate::rank::Rank;
use std::fmt;
use std::str::FromStr;

quick_error! {
    #[derive(Debug)]
    pub enum MyError{
        FailedOperation(s: &'static str, errno: i32) {
            from(errno: i32) -> ("os error", errno)
            from(e: std::io::Error) -> ("io error", e.raw_os_error().unwrap())
        }
        Other(descr: &'static str) {
            display("Error {}", descr)
        }
        String(descr: String) {
            display("Error {}", descr)
        }
        /// Converts from both kinds of utf8 errors
        Utf8(err: std::str::Utf8Error) {
            from()
            from(err: std::string::FromUtf8Error) -> (err.utf8_error())
        }
    }

}

/*impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}*/

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, PartialOrd)]
pub struct Position {
    pub rank: Rank,
    pub file: File,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}", self.rank, self.file,)
    }
}

impl Position {
    pub fn new(rank: &str, file: i8) -> Position {
        Position {
            file: match File::from_str(&file.to_string()) {
                Ok(v) => v,
                Err(e) => panic!("error file {:?}", e),
            },
            rank: match Rank::from_str(&rank.to_string()) {
                Ok(v) => v,
                Err(e) => panic!("error rank {:?}", e),
            },
        }
    }

    pub fn new_(rank: Rank, file: File) -> Position {
        Position {
            file: file,
            rank: rank,
        }
    }
    pub fn diagonals_squares(self, limit: i8) -> Vec<Position> {
        let mut r = Vec::new();
        let mut positions = vec![self.clone(), self.clone(), self.clone(), self.clone()];

        for _ in 0..limit {
            {
                let mut pos = positions[0];
                let new_pos = (pos.rank.right(), pos.file.up());
                if pos.rank < new_pos.0 && pos.file < new_pos.1 {
                    pos.rank = new_pos.0;
                    pos.file = new_pos.1;
                    r.push(pos.clone());
                }

                positions[0] = pos
            }

            {
                let mut pos = positions[1];
                let new_pos = (pos.rank.left(), pos.file.up());
                if pos.rank > new_pos.0 && pos.file < new_pos.1 {
                    pos.rank = new_pos.0;
                    pos.file = new_pos.1;
                    r.push(pos.clone());
                }

                positions[1] = pos
            }
            {
                let mut pos = positions[2];
                let new_pos = (pos.rank.right(), pos.file.down());
                if pos.rank < new_pos.0 && pos.file > new_pos.1 {
                    pos.rank = new_pos.0;
                    pos.file = new_pos.1;
                    r.push(pos.clone());
                }

                positions[2] = pos
            }
            {
                let mut pos = positions[3];
                let new_pos = (pos.rank.left(), pos.file.down());
                if pos.rank > new_pos.0 && pos.file > new_pos.1 {
                    pos.rank = new_pos.0;
                    pos.file = new_pos.1;
                    r.push(pos.clone());
                }

                positions[3] = pos
            }
        }
        return r;
    }

    pub fn side_squares(self, limit: i8) -> Vec<Position> {
        let mut r = Vec::new();
        let mut positions = vec![self.clone(), self.clone(), self.clone(), self.clone()];

        for _ in 0..limit {
            {
                let mut pos = positions[0];
                let rank = pos.rank.left();
                if pos.rank > rank {
                    pos.rank = rank;
                    r.push(pos.clone());
                }

                positions[0] = pos
            }

            {
                let mut pos = positions[1];
                let file = pos.file.up();
                if pos.file < file {
                    pos.file = file;
                    r.push(pos.clone());
                }

                positions[1] = pos
            }
            {
                let mut pos = positions[2];
                let rank = pos.rank.right();
                if pos.rank < rank {
                    pos.rank = rank;
                    r.push(pos.clone());
                }

                positions[2] = pos
            }
            {
                let mut pos = positions[3];
                let file = pos.file.down();
                if pos.file > file {
                    pos.file = file;
                    r.push(pos.clone());
                }

                positions[3] = pos
            }
        }
        return r;
    }

    pub fn shortest_path(self, to: Position) -> Vec<Position> {
        let mut r = Vec::new();
        if self.file == to.file {
            let mut p = self.clone();
            for _ in 0..(self.rank.sub(to.rank)) {
                if self.rank < to.rank {
                    p.rank = p.rank.right();
                    r.push(p.clone());
                } else {
                    p.rank = p.rank.left();
                    r.push(p.clone());
                }
            }
        } else if self.rank == to.rank {
            let mut p = self.clone();
            for _ in 0..(self.file.sub(to.file)) {
                if self.file < to.file {
                    p.file = p.file.up();
                    r.push(p.clone());
                } else {
                    p.file = p.file.down();
                    r.push(p.clone());
                }
            }
        } else {
            //diagonal move
            //
            match to {
                _ if (self.rank < to.rank) && (self.file < to.file) => {
                    let mut p = self.clone();
                    for _ in 0..(self.rank.sub(to.rank)) {
                        p.rank = p.rank.right();
                        p.file = p.file.up();
                        r.push(p.clone());
                    }
                }
                _ if (self.rank < to.rank) && (self.file > to.file) => {
                    let mut p = self.clone();
                    for _ in 0..(self.rank.sub(to.rank)) {
                        p.rank = p.rank.right();
                        p.file = p.file.down();
                        r.push(p.clone());
                    }
                }
                _ if (self.rank > to.rank) && (self.file < to.file) => {
                    let mut p = self.clone();
                    for _ in 0..(self.file.sub(to.file)) {
                        p.rank = p.rank.left();
                        p.file = p.file.up();
                        r.push(p.clone());
                    }
                }
                _ if (self.rank < to.rank) && (self.file > to.file) => {
                    let mut p = self.clone();
                    for _ in 0..(self.file.sub(to.file)) {
                        p.rank = p.rank.left();
                        p.file = p.file.down();
                        r.push(p.clone());
                    }
                }
                _ => {}
            }
        }

        r.into_iter().filter(|pos| *pos != self).collect()
    }

    pub fn squares_around(self, limit: i8) -> Vec<Position> {
        let mut r = Vec::new();
        r.append(&mut self.diagonals_squares(limit));
        r.append(&mut self.side_squares(limit));

        r
    }
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
    pub fn parse(m: &str) -> Result<Vec<Move>, MyError> {
        let mut r = Vec::new();

        let piece = match m.len() {
            0..=4 => 'P',
            5 => m.chars().next().unwrap(),
            _ => return Err(MyError::Other("error in piece part".into())),
        };

        let from_rank = match m.len() {
            0..=4 => m.chars().next().unwrap(),
            5 => m.chars().nth(1).unwrap(),
            _ => return Err(MyError::Other("error in from file")),
        };

        let from_file = match m.len() {
            0..=4 => m.chars().nth(1).unwrap(),
            5 => m.chars().nth(2).unwrap(),
            _ => return Err(MyError::Other("error in from rank".into())),
        };

        let to_rank = match m.len() {
            0..=4 => m.chars().nth(2).unwrap(),
            5 => m.chars().nth(3).unwrap(),
            _ => return Err(MyError::Other("error in to file".into())),
        };
        let to_file = match m.len() {
            0..=4 => m.chars().nth(3).unwrap(),
            5 => m.chars().nth(4).unwrap(),
            _ => return Err(MyError::Other("error in to rank".into())),
        };

        println!(
            "from: {}{} to: {}{}",
            from_rank, from_file, to_rank, to_file
        );

        r.push(Move {
            piece: match Piece::from_str(&piece.to_string()) {
                Ok(v) => v,
                Err(e) => return Err(MyError::String(format!("error piece {:?}", e))),
            },
            position: Position {
                file: match File::from_str(&from_file.to_string()) {
                    Ok(v) => v,
                    Err(e) => return Err(MyError::String(format!("push 0 error file {:?}", e))),
                },
                rank: match Rank::from_str(&from_rank.to_string()) {
                    Ok(v) => v,
                    Err(e) => return Err(MyError::String(format!("error rank {:?}", e))),
                },
            },
        });

        r.push(Move {
            piece: match Piece::from_str(&piece.to_string()) {
                Ok(v) => v,
                Err(e) => return Err(MyError::String(format!("error piece {:?}", e))),
            },
            position: Position {
                file: match File::from_str(&to_file.to_string()) {
                    Ok(v) => v,
                    Err(e) => return Err(MyError::String(format!("error file {:?}", e))),
                },
                rank: match Rank::from_str(&to_rank.to_string()) {
                    Ok(v) => v,
                    Err(e) => return Err(MyError::String(format!("error rank {:?}", e))),
                },
            },
        });
        Ok(r)
    }

    pub fn parse_file(path: &str) -> Result<Vec<Move>, MyError> {
        let r = Vec::new();

        let reader = my_reader::BufReader::open(path);
        let mut buffer = String::new();
        match reader {
            Ok(mut f) => {
                while let Some(line) = f.read_line(&mut buffer) {
                    println!("{}", line?.trim());
                }
            }
            Err(e) => panic!("error parsing file {:?}", e),
        }
        Ok(r)
    }
}
