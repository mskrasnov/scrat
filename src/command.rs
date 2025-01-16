//! Command parser
//!
//! ## Syntax
//!
//! ```no-test
//! :<COMMAND> <SUBCOMMAND> <ARG 1> <ARG 2> ... <ARG N>
//! ```

use anyhow::Result;
use colored::Colorize;
use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
    str::FromStr,
};

pub fn prompt<M: Display>(msg: M) -> Result<String> {
    print!("{}{} ", msg.to_string().bold(), "%".bold().yellow());
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
        println!(
            "{:>width$}: {}",
            "new-status | status".bold(),
            "write new status".dimmed(),
            width = 25
        );
        println!(
            "{:>width$}: {}",
            "status-list".bold(),
            "print 5 my statuses".dimmed(),
            width = 25
        );
        println!(
            "{:>width$}: {}",
            "exit".bold(),
            "exit program".dimmed(),
            width = 25
        );
    }
}
