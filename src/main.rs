pub mod actions;
pub mod command;
pub mod consts;
pub mod log;
pub mod register;
// pub mod tui;

use mastodon_async::{Error, Result};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let md = register::get_mastodon_data("./md.toml", "https://mastodon.ml").await?;
    let mut exit = false;

    actions::welcome(&md).await?;
    while !exit {
        let cmd = command::Command::from_str(
            &command::prompt("").map_err(|err| Error::Other(err.to_string()))?,
        )
        .map_err(|_| Error::Other("Unknown command".to_string()))?;

        match cmd {
            command::Command::NewStatus => actions::create_new_status(&md).await?,
            command::Command::StatusList => actions::get_statuses(&md).await?,
            command::Command::Notifications => actions::get_notifications(&md).await?,
            command::Command::Help => command::Command::print_help(),
            command::Command::Exit => exit = true,
            _ => {}
        }
    }

    println!("Have a good day.");

    // tui::main_win();

    Ok(())
}
