use std::{env::args, error::Error};

use crossterm::{
    event::{Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};

use tui_markup::generator::TuiTextGenerator;

mod common;

fn main() -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let text = common::compile_file::<TuiTextGenerator, _>(args().nth(1).unwrap());

    loop {
        terminal.draw(|frame| {
            frame.render_widget(Paragraph::new(text.clone()), frame.size());
        })?;

        if let Event::Key(key) = crossterm::event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        };
    }

    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use tui_markup::generator::TuiTextGenerator;

    use crate::common::compile_file;

    #[test]
    fn test_texts() {
        compile_file::<TuiTextGenerator, _>("examples/help.txt");
        compile_file::<TuiTextGenerator, _>("examples/indexed.txt");
    }
}
