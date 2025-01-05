//pub mod command;
pub mod consts;
pub mod register;

//use std::time::Duration;

use mastodon_async::{prelude::*, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let md = register::get_mastodon_data("./md.toml", "https://mastodon.md", consts::CLIENT_SITE).await?;
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

    /*let mut i = 10;
    while i > 0 {
        println!("{i} secs...");
        std::thread::sleep(Duration::from_secs(i));
        i -= 1;
    }
    
    md.delete_status(status).await?;*/

    Ok(())
}

/*use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let app = App::default().run(&mut term);
    ratatui::restore();
    app
}

#[derive(Debug, Default)]
struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_cnt(),
            KeyCode::Right => self.increment_cnt(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_cnt(&mut self) {
        self.counter += 1;
    }

    fn decrement_cnt(&mut self) {
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Scrat Mastodon Client ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let cnt_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(cnt_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}*/
