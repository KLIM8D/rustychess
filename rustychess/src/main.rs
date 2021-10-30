use rustychess_core::chessboard::BoardStatus;
use rustychess_core::file::File;
use rustychess_core::game::Game;
use rustychess_core::pieces::Color;
use rustychess_core::pieces::Kind;
use rustychess_core::pieces::Piece;
use rustychess_core::rank::Rank;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::str::FromStr;

fn main() {
    /*println!("Hello, world!");
    let mut pawn = Piece::new(Kind::Pawn);
    println!("{}", pawn.move_p());
    for i in 0..5 {
        pawn.capture();
    }
    println!("{}", pawn.position());

    let mut board = Chessboard::new();
    board.set("A", 1, pawn);

    let a = board.get(&"A", &1);
    println!("{}", a.unwrap().position())*/

    let mut game = Game::new();
    /*
     *
     * [(R),(N),(B),(Q),(K),(B),(N),(R), (P), (P)...]
     */
    //board.set("A", 1, pawn);
    //&mut ShellIO, &mut T, &[&str]

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let s = line.split_whitespace().collect::<Vec<&str>>();
                println!("Line: {}", s[0]);
                match s[0] {
                    "print" => {
                        game.board.clone().print();
                    }
                    "get" => {
                        let (rank, file) = if let Some(r) = s.get(1) {
                            let (rank, v) = r.split_at(1);
                            (Some(rank), v.parse::<i8>().ok())
                        } else {
                            (None, None)
                        };
                        //let file = s.get(2).and_then(|x| x.parse::<i8>().ok());

                        match (rank, file) {
                            (Some(rank), Some(file)) => {
                                let r = Rank::from_str(rank).ok();
                                let f = File::from_i8(file).ok();

                                println!("{} {}", rank, file);
                                match (r, f) {
                                    (Some(r), Some(f)) => {
                                        match game.board.get(r.to_str(), f.to_i8()) {
                                            Some(v) => println!("Piece: {}", v),
                                            None => println!("empty"),
                                        }
                                    }
                                    _ => println!("lol"),
                                }
                            }
                            _ => println!("wolla"),
                        }
                    }
                    "move" => {
                        let from = s[1];
                        let to = s[2];
                        let s = format!("{}{}", from, to);
                        let status = game.move_(s.as_str());
                        match status {
                            Ok(v) => {
                                match v {
                                    BoardStatus::Promote => {
                                        let kind = promote(&mut rl);
                                    }
                                    _ => {}
                                }
                                println!("ok")
                            }
                            Err(e) => println!("{}", e),
                        };
                    }
                    _ => println!("not valid"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    fn promote(rl: &mut Editor<()>) -> Kind {
        println!("Promote pawn, options: Q, R, K, B");
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    let s = line.as_str();
                    match s {
                        "q" | "Q" => return Kind::Queen,
                        "r" | "R" => return Kind::Rook,
                        "k" | "K" => return Kind::Knight,
                        "b" | "B" => return Kind::Bishop,
                        _ => {
                            println!("not valid, options: Q, R, K, B");
                        }
                    }
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
    }
}
