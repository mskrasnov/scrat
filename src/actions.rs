use colored::Colorize;
use futures_util::StreamExt;
use mastodon_async::entities::notification::NotificationType;
use mastodon_async::{prelude::*, Error, Result, StatusesRequest};

use crate::command;
use crate::consts::PROGRAM_NAME;

pub async fn welcome(md: &Mastodon) -> Result<()> {
    let me = md.verify_credentials().await?;
    println!(
        "Welcome to {}, {}! Input 'help' command for list all supported actions.",
        PROGRAM_NAME.bold(),
        &me.acct,
    );

    Ok(())
}

pub async fn create_new_status(md: &Mastodon) -> Result<()> {
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
        println!(
            "{} For some reason, the status URL was not returned from the server :(",
            "E:".bold().red()
        );
    }
    Ok(())
}

fn date_str(y: i32, m: String, d: u8, h: u8, min: u8) -> String {
    let date = format!("{y} {m}, {}{d}", if d < 10 { "0" } else { "" })
        .cyan()
        .dimmed();
    let time = format!(
        "{}{}:{}{}",
        if h < 10 { "0" } else { "" },
        h,
        if min < 10 { "0" } else { "" },
        min
    )
    .cyan()
    .bold();

    format!("{date} {time}")
}

pub async fn get_notifications(md: &Mastodon) -> Result<()> {
    md.notifications()
        .await?
        .items_iter()
        .for_each(|n| async move {
            print!(
                "{} {} {} ",
                match n.notification_type {
                    NotificationType::Mention => "Mention  ",
                    NotificationType::Reblog => "Reblog   ",
                    NotificationType::Favourite => "Favourite",
                    NotificationType::Follow => "Follow   ",
                    NotificationType::FollowRequest => "FollowReq",
                    NotificationType::Poll => "Poll     ",
                },
                format!("#{}", n.id).yellow(),
                date_str(
                    n.created_at.year(),
                    n.created_at.month().to_string(),
                    n.created_at.day(),
                    n.created_at.hour(),
                    n.created_at.minute()
                ),
            );
            println!("{} ({})", &n.account.display_name, &n.account.acct.dimmed());
        })
        .await;

    Ok(())
}

pub async fn get_statuses(md: &Mastodon) -> Result<()> {
    let mut filters = StatusesRequest::new();
    filters.limit(5);

    let me = md.verify_credentials().await?;

    //md.statuses(&AccountId::new(me.id.as_ref()), filters)
    md.statuses(&me.id, filters)
        .await?
        .items_iter()
        .for_each(|status| async move {
            match remove_html_symbols(&status.content) {
                Ok(content) => {
                    let created = &status.created_at;
                    println!(
                        "{} {} {}",
                        "Status created:".bold().yellow(),
                        format!("{} {}, {}", created.year(), created.month(), created.day())
                            .cyan()
                            .dimmed(),
                        format!("{}:{}", created.hour(), {
                            let m = created.minute();
                            if m < 10 {
                                format!("0{m}")
                            } else {
                                m.to_string()
                            }
                        })
                        .cyan()
                        .bold(),
                    );
                    println!("{}\n", colored_str(&content));
                }
                Err(why) => eprintln!("[{}] Failed to get status: {why}", "E".bold().red()),
            }
        })
        .await;

    Ok(())
}

fn remove_html_symbols(s: &str) -> Result<String> {
    let s = s
        .replace("</p>", "\n\n")
        .replace("<br>", "\n")
        .replace("<br />", "\n")
        .replace("<br/>", "\n");
    let chars = s.chars().collect::<Vec<_>>();
    let mut buf = Vec::new();

    let mut is_add = true;

    for ch in chars {
        if ch == '<' {
            is_add = false;
        }

        if ch == '>' {
            is_add = true;
            continue;
        }

        if is_add {
            buf.push(ch);
        }
    }

    let mut s = String::new();
    for ch in buf {
        s.push(ch);
    }

    let s = s
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("...", "…")
        .trim()
        .to_string();

    Ok(s)
}

fn colored_str(s: &str) -> String {
    let chunks = s.split(' ').collect::<Vec<_>>();
    let mut buf = Vec::new();

    for chunk in chunks {
        if chunk.starts_with('#') {
            buf.push(format!("{}", chunk.dimmed()));
        } else if chunk.starts_with('@') {
            buf.push(format!("{}", chunk.underline().bold().magenta()));
        } else if chunk.contains("http") || chunk.contains("ftp") {
            buf.push(format!("{}", chunk.underline().blue()));
        } else {
            buf.push(chunk.to_string());
        }
        buf.push(" ".to_string());
    }

    let mut s = String::new();
    for i in buf {
        s.push_str(&i);
    }

    s
}
