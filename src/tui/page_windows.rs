//! Windows for (shit)posting and reading statuses

use std::str::FromStr;

// use cursive::align::HAlign;
// use cursive::event::Key;
// use cursive::menu;
// use cursive::views::{Dialog, Panel, SelectView, TextView};
// use cursive::Cursive;
// use mastodon_async::{Mastodon, StatusesRequest, Result};

pub enum Page {
    NewStatus,
    StatusList,
}

impl Page {
    pub const ALL: [Self; 2] = [Self::NewStatus, Self::StatusList];

    pub fn title(&self) -> String {
        match self {
            Self::NewStatus => "New status",
            Self::StatusList => "My statuses",
        }
        .to_string()
    }

    /*pub async fn status_list(&self, scr: &mut Cursive, md: &Mastodon) -> Result<()> {
        let mut filters = StatusesRequest::new();
        filters.limit(5);

        let me = md.verify_credentials().await?;

        //md.statuses(&AccountId::new(me.id.as_ref()), filters)
        md.statuses(&me.id, filters)
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
    }*/
}

impl ToString for Page {
    fn to_string(&self) -> String {
        match self {
            Self::NewStatus => "new-status",
            Self::StatusList => "status-list",
        }
        .to_string()
    }
}

impl FromStr for Page {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "new-status" | "status" => Self::NewStatus,
            "status-list" => Self::StatusList,
            _ => Self::StatusList,
        })
    }
}
