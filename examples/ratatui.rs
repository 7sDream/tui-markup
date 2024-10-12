use std::{env::args, io};

use crossterm::event::{Event, KeyCode};
use ratatui::widgets::Paragraph;

use tui_markup::generator::RatatuiTextGenerator;

mod common;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::try_init()?;

    let text =
        common::compile_file::<RatatuiTextGenerator, _>(args().nth(1).expect("Expected a command line argument"));

    loop {
        terminal.draw(|frame| {
            frame.render_widget(Paragraph::new(text.clone()), frame.area());
        })?;

        if let Event::Key(key) = crossterm::event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        };
    }

    ratatui::try_restore()
}

#[cfg(test)]
mod test {
    use tui_markup::generator::RatatuiTextGenerator;

    use crate::common::compile_file;

    #[test]
    fn test_texts() {
        compile_file::<RatatuiTextGenerator, _>("examples/help.txt");
        compile_file::<RatatuiTextGenerator, _>("examples/indexed.txt");
    }
}
