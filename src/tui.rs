//! Terminal user interface based on `cursive`.

mod service_windows;
mod page_windows;

use page_windows::Page;
use std::str::FromStr;

use cursive::align::HAlign;
use cursive::event::Key;
use cursive::menu;
use cursive::views::{Dialog, Panel, SelectView, TextView};
use cursive::Cursive;

use crate::consts::PROGRAM_NAME;

pub fn main_win() {
    let mut scr = cursive::default();
    menu(&mut scr);

    let mut views = SelectView::new().h_align(HAlign::Left).autojump();

    for i in Page::ALL {
        views.add_item(i.title(), i.to_string());
    }

    views.set_on_submit(page_win);

    scr.add_layer(
        Dialog::around(Panel::new(views))
            .title(PROGRAM_NAME)
            .button("Exit", |s| s.quit()),
    );

    scr.run();
}

fn menu(scr: &mut Cursive) {
    scr.menubar()
        .add_subtree("File", menu::Tree::new().leaf("Quit [F10]", |s| s.quit()))
        .add_subtree(
            "Help",
            menu::Tree::new().leaf("About", service_windows::about_win),
        );

    scr.set_autohide_menu(false);

    scr.add_global_callback(Key::F1, |s| s.select_menubar());
    scr.add_global_callback(Key::F10, |s| s.quit());
}

fn page_win(scr: &mut Cursive, page: &String) {
    let win = Dialog::around(Panel::new(TextView::new(page)))
        .button("OK", |s| {
            s.pop_layer();
        })
        .title(Page::from_str(page).unwrap().title());

    scr.add_layer(win);
}
