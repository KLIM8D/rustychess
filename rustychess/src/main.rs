use rustychess_core::chessboard::BoardStatus;
use rustychess_core::file::File;
use rustychess_core::game::Game;
use rustychess_core::pgn::Position;
use rustychess_core::pieces::Kind;
use rustychess_core::rank::Rank;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::history::MemHistory;
use std::str::FromStr;
use chrono::Local;
use std::fs::{self, File as StdFile};
use std::path::Path;
use std::sync::Arc;
use rustyline::{DefaultEditor, Result};
use rustyline::history::DefaultHistory;

mod command;

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

fn main_old() {
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
    //
    //
    // Create a directory named "saves" if it doesn't exist
    let save_directory = "saves";
    if !Path::new(save_directory).exists() {
        fs::create_dir(save_directory).unwrap();
    }

    //let mut rl = Editor::<()>::new();
    let history = rustyline::history::MemHistory::new();
    let config = rustyline::Config::builder().auto_add_history(true).build();
    //let mut rl = rustyline::DefaultEditor::new().unwrap();
    let mut rl: Editor<(), _> = Editor::with_history(config, history).unwrap();
    'readlineLoop: loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let s = line.split_whitespace().collect::<Vec<&str>>();
                println!("Line: {}", s[0]);

                match s[0] {
                    "new" => {
                        //new_command()
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
                        //let mut stdout = stdout();
                        
                        // Get the current date and time in the local timezone
                        let current_datetime = Local::now();

                        // Format the date and time as part of the filename
                        let formatted_datetime = current_datetime.format("%Y-%m-%d_%H-%M-%S");
                        let filename = format!("file_{}.txt", formatted_datetime);

                        let file_path = Path::new(save_directory).join(&filename);
                        // Open a file for writing with the generated filename
                        let file = StdFile::create(&file_path);

                        match file {
                            Ok(mut f) => {
                                let result = game.save(&mut f);
                                match result {
                                    Ok(()) => println!("\nSave operation succeeded!"),
                                    Err(error) => println!("\nSave operation failed with error: {:?}", error),
                                }
                            },
                            Err(error) => println!("\nSave operation failed with error: {:?}", error),
                        }

                    },
                    "load" => {
                        if s.len() < 2 {
                            println!("Wrong format");
                            continue;
                        }

                        let path = s[1];
                        let exist = Path::exists(Path::new(path));
                        if !exist {
                            println!("File not found");
                            continue;
                        }

                        match std::fs::read_to_string(path) {
                            Ok(contents) => {
                                game.load(contents);
                            },
                            Err(err) => {
                                println!("Error reading file: {}", err)
                            }
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

    fn promote(rl: &mut Editor<(), MemHistory>) -> Kind {
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

fn new_command(game: Game) {
    if game.clone().number_of_moves() > 0 {
        println!("Game already started..");
        //continue 'readlineLoop;
    }

    /*if let Some(name) = s.get(1) {
        game.insert_metadata("name".to_string(), name.to_string());
    }*/
}

fn main() -> Result<()> {
    let mut game = Game::new();
    /*
     *
     * [(R),(N),(B),(Q),(K),(B),(N),(R), (P), (P)...]
     */
    //board.set("A", 1, pawn);
    //&mut ShellIO, &mut T, &[&str]
    
    // Create a directory named "saves" if it doesn't exist
    let save_directory = "saves";
    if !Path::new(save_directory).exists() {
        fs::create_dir(save_directory).unwrap();
    }

    let commands = registered_commands();
    let h = command::DIYHinter {
        commands: commands.clone(),
    };


    let mut rl: Editor<command::DIYHinter, DefaultHistory> = Editor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    //rl.set_helper(Some(h));

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                println!("Line: {}", line);

                let tokens: Vec<&str> = line.trim().split_whitespace().collect();

                if let Some((cmd_name, args)) = tokens.split_first() {
                    if let Some(cmd) = commands.iter().find(|c| c.name() == *cmd_name) {
                        cmd.run(args);
                    } else {
                        println!("Unknown command: {}", cmd_name);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    rl.save_history("history.txt");

    Ok(())
}

fn registered_commands() -> Vec<Arc<dyn command::Command>> {
    vec![
        Arc::new(command::HelpCommand),
        Arc::new(command::GetCommand),
        // add more commands as needed
    ]
}
