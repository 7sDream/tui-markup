use std::error::Error;

use crossterm::{
    event::{Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};

use tui_markup::{compile, generator::TuiTextGenerator};

static HELP_TEXTS: &str = include_str!("help.txt");

fn main() -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    loop {
        terminal.draw(|frame| {
            frame.render_widget(
                Paragraph::new(compile::<TuiTextGenerator>(HELP_TEXTS).unwrap()),
                frame.size(),
            );
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
    use tui::style::{Color, Style};
    use tui_markup::{compile_with, generator::TuiTextGenerator};

    #[test]
    fn test_help_text() {
        assert!(tui_markup::compile::<TuiTextGenerator>(super::HELP_TEXTS).is_ok());
    }

    #[test]
    fn test_help_text2() {
        let gen = TuiTextGenerator::new(|s: &str| match s {
            "keyboard" => Some(Style::default().fg(Color::Green)),
            _ => None,
        });

        assert!(compile_with(super::HELP_TEXTS, gen).is_ok());
    }
}