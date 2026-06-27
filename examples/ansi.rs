use std::env::args;

use tui_markup::generator::ANSIStringsGenerator;

mod common;

fn main() {
    let s = common::compile_file::<ANSIStringsGenerator, _>(args().nth(1).unwrap());

    println!("{}", s)
}

#[cfg(test)]
mod test {
    use tui_markup::generator::ANSIStringsGenerator;

    use super::common::compile_file;

    #[test]
    fn test_help_text() {
        compile_file::<ANSIStringsGenerator, _>("examples/help.txt");
        compile_file::<ANSIStringsGenerator, _>("examples/indexed.txt");
    }
}
