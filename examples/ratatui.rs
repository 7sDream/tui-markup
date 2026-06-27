use crossterm::event::{Event, KeyCode};
use ratatui::{Terminal, backend::Backend, widgets::Paragraph};
use tui_markup::generator::RatatuiTextGenerator;

mod common;

fn display<B: Backend>(terminal: &mut Terminal<B>) -> std::io::Result<()>
where
    std::io::Error: From<B::Error>,
{
    let text = common::compile_file::<RatatuiTextGenerator, _>(
        std::env::args()
            .nth(1)
            .expect("Expected a command line argument"),
    );

    loop {
        terminal.draw(|frame| {
            frame.render_widget(Paragraph::new(text.clone()), frame.area());
        })?;

        if let Event::Key(key) = crossterm::event::read()?
            && let KeyCode::Char('q') = key.code
        {
            break;
        };
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    ratatui::run(display)
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
