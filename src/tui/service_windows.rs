//! Some help and service windows (`cursive`)

use cursive::views::{Dialog, LinearLayout, Panel, TextView};
use cursive::Cursive;

use crate::consts::{CLIENT_DOCS_PAGE, CLIENT_SITE, PROGRAM_NAME, PROGRAM_VER};

pub fn about_win(scr: &mut Cursive) {
    let about = LinearLayout::vertical()
        .child(Panel::new(TextView::new(format!(
            "{PROGRAM_NAME} ver. {PROGRAM_VER}. Simple console Mastodon client for Windows and Linux.\n\n\
              Copyright (C) 2025 Michail Krasnov <michail383krasnov@mail.ru>",
        ))))
        .child(
            Panel::new(TextView::new(format!(
                "Site: {CLIENT_SITE}\n\
                 Documentation: {CLIENT_DOCS_PAGE}\n\
                 GitHub repo: https://github.com/mskrasnov/scrat",
            )))
            .title("Resources"),
        )
        .child(
            Panel::new(TextView::new(format!(
                "2202 2062 5233 5406 (Sber) - for donates",
            )))
            .title("Support me!"),
        );

    let win = Dialog::around(about)
        .title("About Scrat")
        .button("OK", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}
