use std::sync::Arc;

use rustyline::hint::{Hint, Hinter};
use rustyline::Context;
use rustyline::{Completer, Helper, Highlighter, Validator};
//use rustyline::completion::Completer;

pub trait Command: Send + Sync {
    fn name(&self) -> &str;
    fn complete_up_to(&self) -> usize;
    fn run(&self, args: &[&str]);
}

#[derive(Completer, Helper, Highlighter, Validator)]
pub struct DIYHinter {
    pub commands: Vec<Arc<dyn Command>>,
}

pub struct HelpCommand;

impl Command for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn complete_up_to(&self) -> usize {
        self.name().len()
    }

    fn run(&self, _args: &[&str]) {
        println!("This is the help command");
    }
}

pub struct GetCommand;

impl Command for GetCommand {
    fn name(&self) -> &str {
        "get"
    }

    fn complete_up_to(&self) -> usize {
        self.name().len()
    }

    fn run(&self, args: &[&str]) {
        match args {
            [key] => println!("Getting key: {}", key),
            _ => println!("Usage: get <key>"),
        }
    }
}

pub struct DynCommandHint {
    display: String,
    complete_up_to: usize,
}

impl Hint for DynCommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.complete_up_to])
        } else {
            None
        }
    }
}

impl Hinter for DIYHinter {
    type Hint = DynCommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.commands.iter().find_map(|cmd| {
            if cmd.name().starts_with(line) {
                Some(DynCommandHint {
                    display: cmd.name().to_owned(),
                    complete_up_to: cmd.complete_up_to().saturating_sub(0),
                })
            } else {
                None
            }
        })
    }
}
