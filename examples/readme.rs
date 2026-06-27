fn main() {
    use anstyle::{AnsiColor, Style};
    use tui_markup::{compile, compile_with, generator::ANSIStringsGenerator};

    // Parse markup into some final result for showing
    let result = compile::<ANSIStringsGenerator>("You got a <yellow Coin>").unwrap();
    // Show it
    println!("{}", result);

    // With custom tag
    let generator = ANSIStringsGenerator::new(|tag: &str| match tag {
        "keyboard" => Some(
            Style::new()
                .fg_color(Some(AnsiColor::Blue.into()))
                .bg_color(Some(AnsiColor::Black.into()))
                .bold(),
        ),
        _ => None,
    });
    let result = compile_with("Press <keyboard Space> to jump", generator).unwrap();
    println!("{}", result);
}
