use ansi_term::ANSIStrings;

use tui_markup::generator::ansi::ANSIStringsGenerator;

static HELP_TEXTS: &str = include_str!("help.txt");

fn main() {
    let s = tui_markup::compile::<ANSIStringsGenerator>(HELP_TEXTS).unwrap();

    println!("{}", ANSIStrings(&s))
}
