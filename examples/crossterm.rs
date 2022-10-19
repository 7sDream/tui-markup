mod common;

use std::io::Write;

use crossterm::QueueableCommand;

use tui_markup::generator::CrosstermCommandsGenerator;

fn main() {
    let s = common::compile_file::<CrosstermCommandsGenerator, _>(std::env::args().nth(1).unwrap());

    let mut stdout = std::io::stdout();
    s.iter().for_each(|span| {
        stdout.queue(span).unwrap();
    });

    stdout.flush().unwrap();
}
