use std::env::args;

use ansi_term::ANSIStrings;
use tui_markup::generator::ANSIStringsGenerator;

mod common;

fn main() {
    let s = common::compile_file::<ANSIStringsGenerator, _>(args().nth(1).unwrap());

    println!("{}", ANSIStrings(&s))
}

#[cfg(test)]
mod test {
    use super::common::compile_file;
    use tui_markup::generator::ANSIStringsGenerator;

    #[test]
    fn test_help_text() {
        compile_file::<ANSIStringsGenerator, _>("examples/help.txt");
        compile_file::<ANSIStringsGenerator, _>("examples/indexed.txt");
    }
}
