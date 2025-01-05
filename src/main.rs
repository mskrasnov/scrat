//pub mod command;
pub mod consts;
pub mod log;

pub mod register;

//use std::time::Duration;

use mastodon_async::{prelude::*, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let md = register::get_mastodon_data("./md.toml", "https://mastodon.ml").await?;
    let status = StatusBuilder::default()
        .status("TEST STATUS")
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