use mastodon_async::{
    helpers::{cli, toml},
    prelude::*,
    Result,
};
use std::path::Path;

use crate::consts::{PROGRAM_NAME, CLIENT_SITE};

pub async fn get_mastodon_data<P: AsRef<Path>>(
    pth: P,
    instance_url: &str,
) -> Result<Mastodon> {
    match toml::from_file(&pth) {
        Ok(data) => Ok(Mastodon::from(data)),
        Err(_) => register(&pth, instance_url).await,
    }
}

pub async fn register<P: AsRef<Path>>(
    pth: P,
    instance_url: &str,
) -> Result<Mastodon> {
    let reg = Registration::new(instance_url)
        .client_name(PROGRAM_NAME)
        .scopes(Scopes::all())
        .website(CLIENT_SITE)
        .build()
        .await?;

    let mastodon = cli::authenticate(reg).await?;

    toml::to_file(&mastodon.data, &pth)?;

    Ok(mastodon)
}
