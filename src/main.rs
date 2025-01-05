//pub mod command;
pub mod command;
pub mod consts;
pub mod log;

pub mod register;

use futures_util::StreamExt;
use mastodon_async::{prelude::*, Error, Result, StatusesRequest};
use std::str::FromStr;

async fn create_new_status(md: &Mastodon) -> Result<()> {
    let status_txt =
        command::prompt("Enter your status").map_err(|err| Error::Other(err.to_string()))?;

    let status = StatusBuilder::default()
        .status(status_txt)
        .visibility(Visibility::Public)
        .language(mastodon_async::Language::Rus)
        .build()?;

    let status = md.new_status(status).await?;

    if let Some(url) = status.url {
        println!("You may be read this post at: {url}");
    } else {
        println!("For some reason, the status URL was not returned from the server :(");
    }
    Ok(())
}

async fn get_statuses(md: &Mastodon) -> Result<()> {
    let mut filters = StatusesRequest::new();
    filters.limit(5);

    let me = md.verify_credentials().await?;

    md.statuses(&AccountId::new(me.id.as_ref()), filters)
        .await?
        .items_iter()
        .for_each(|status| async move {
            let created = &status.created_at;
            println!(
                "Status created: {}, {}:{}",
                created.year(),
                created.hour(),
                created.minute(),
            );
            println!("{}\n", &status.content);
        })
        .await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let md = register::get_mastodon_data("./md.toml", "https://mastodon.ml").await?;
    let mut exit = false;

    while !exit {
        let cmd = command::Command::from_str(
            &command::prompt("").map_err(|err| Error::Other(err.to_string()))?,
        )
        .map_err(|_| Error::Other("Unknown command".to_string()))?;

        match cmd {
            command::Command::NewStatus => create_new_status(&md).await?,
            command::Command::StatusList => get_statuses(&md).await?,
            command::Command::Help => command::Command::print_help(),
            command::Command::Exit => exit = true,
            _ => {}
        }
    }

    println!("Have a good day.");

    Ok(())
}
