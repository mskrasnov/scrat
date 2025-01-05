//! Command parser
//!
//! ## Syntax
//!
//! ```no-test
//! :<COMMAND> <SUBCOMMAND> <ARG 1> <ARG 2> ... <ARG N>
//! ```

use color_eyre::Result;
use std::{fmt::Display, io::{stdin, stdout, Write}, str::FromStr};

pub fn prompt<M: Display>(msg: M) -> Result<String> {
    print!("{msg}> ");
    stdout().flush()?;

    let mut s = String::new();
    stdin().read_line(&mut s)?;

    Ok(s.trim().to_string())
}

#[derive(Debug, Clone)]
pub enum Command {
    NewStatus,
    StatusList,

    Help,
    Exit,

    ErrorCmd,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "new-status" | "status" => Self::NewStatus,
            "status-list" => Self::StatusList,
            "help" => Self::Help,
            "exit" => Self::Exit,

            _ => Self::ErrorCmd,
        })
    }
}

impl Command {
    pub fn print_help() {
        println!("{:>width$}: {}", "new-status | status", "write new status", width = 25);
        println!("{:>width$}: {}", "status-list", "print 5 my statuses", width = 25);
        println!("{:>width$}: {}", "exit", "exit program", width = 25);
    }
}