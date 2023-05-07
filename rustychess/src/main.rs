use rustychess_core::chessboard::BoardStatus;
use rustychess_core::file::File;
use rustychess_core::game::Game;
use rustychess_core::pgn::Position;
use rustychess_core::pieces::Kind;
use rustychess_core::rank::Rank;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::str::FromStr;
use std::io::{stdout, Write};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

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
    'readlineLoop: loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let s = line.split_whitespace().collect::<Vec<&str>>();
                println!("Line: {}", s[0]);

                match s[0] {
                    "new" => {
                        if game.clone().number_of_moves() > 0 {
                            println!("Game already started..");
                            continue 'readlineLoop;
                        }

                        if let Some(name) = s.get(1) {
                            game.insert_metadata("name".to_string(), name.to_string());
                        }
                    }
                    "metadata" => {
                        game.clone().print_metadata();
                    }
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
                        if s.len() < 3 {
                            println!("Wrong format");
                            continue;
                        }

                        let from = s[1];
                        let to = s[2];
                        if from.len() != 2 || to.len() != 2 {
                            println!("Wrong format");
                            continue;
                        }

                        let mut is_valid = true;
                        if let Some(r) = s.get(1) {
                            let (rank, file) = r.split_at(1);
                            is_valid = Rank::from_str(rank).is_ok() && File::from_str(file).is_ok();
                        }

                        if let Some(r) = s.get(2) {
                            let (rank, file) = r.split_at(1);
                            is_valid = Rank::from_str(rank).is_ok() && File::from_str(file).is_ok();
                        }

                        if !is_valid {
                            println!("Wrong format 1");
                            continue;
                        }

                        let s = format!("{}{}", from, to);
                        let status = game.move_(s.as_str());
                        match status {
                            Ok(v) => {
                                match v {
                                    BoardStatus::Promote => {
                                        let kind = promote(&mut rl);
                                        game.promote(
                                            &Position::new(
                                                &from[0..1],
                                                from[1..2].parse::<i8>().unwrap(),
                                            ),
                                            kind,
                                        )
                                    }
                                    _ => {}
                                }
                                println!("ok")
                            }
                            Err(e) => println!("{}", e),
                        };
                    }
                    "movelist" => game.printmoves(),
                    "save" => {
                        let mut stdout = stdout();
                        let result = game.save(&mut stdout);
                        match result {
                            Ok(()) => println!("\nSave operation succeeded!"),
                            Err(error) => println!("\nSave operation failed with error: {:?}", error),
                        }
                    },
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

    fn parse_arguments() {
        let args = Args::parse();

        for _ in 0..args.count {
            println!("Hello {}!", args.name)
        }
    }
}
