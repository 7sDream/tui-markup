fn main() {
    use ansi_term::{ANSIStrings, Color, Style};
    use tui_markup::{compile, compile_with, generator::ANSIStringsGenerator};

    // Parse markup into some final result for showing
    let result = compile::<ANSIStringsGenerator>("You got a <yellow Coin>").unwrap();
    // Show it
    println!("{}", ANSIStrings(&result));

    // With custom tag
    let gen = ANSIStringsGenerator::new(|tag: &str| match tag {
        "keyboard" => Some(Style::default().fg(Color::Blue).on(Color::Black).bold()),
        _ => None,
    });
    let result = compile_with("Press <keyboard Space> to jump", gen).unwrap();
    println!("{}", ANSIStrings(&result));
}
