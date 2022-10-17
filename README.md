# tui markup

[![crates.io][badge-crate-version]][crate]
[![changelog][badge-changelog]][changelog]
[![docs.rs][badge-docs-rs]][doc]
![deps state][badge-deps-state]

This crate provides a markup language to quickly write colorful and styled terminal text in plain text.

## Usage

```rust
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
```

Result:

![result of example][usage-screenshot]

Notice the result type and how to show it is vary depends on what `Generator` you use.

The `ANSIStringsGenerator` is one of the [built-in generators][doc-builtin-gens] implementation, for directly print result
in any ASNI compliant terminal.

There is also a macro([`tui-markup-ansi-macro`] crate) to compile markup source into ANSI sequence at compile time, check it if you need.

You can add this markup support for other terminal/library/application easily by create you own generator.

## Examples

![help text][help-text-screenshot]

The example is shown in Windows Terminal, using the following command:

`cargo run --example tui --features tui -- examples/help.txt`

The source markup text of this article can be found in [examples/help.txt].

you can change the last argument to your file to render other article, for example `examples/indexed.txt` for a full xterm256 color chart:

![color chart][indexed-screenshot]

Those two screenshot are using built-in `tui` generator.

## Markup syntax

Only one syntax `<taglist content>` to add style to content.

`taglist` is a `tag` list sep by `,`.

`tag` has format of `mode:value`, available `mode` are:

- `fg:` for foreground color.
- `bg:` for background color.
- `mod:` for modifiers.

Mode and `:` is optional except for `bg:`, so `66ccff` = `fg:66ccf` , and `b` = `mod:b`.

Some examples:

- `<green text>` for a green color text, `<66ccff text>` for a #66ccff color text.
- `<bg:blue text>` for a blue background text, `<bg:66ccff text>` for a #66ccff background text.
- `<b text>` for a bold text, `<i text>` for a italic/slant text.
- `<bg:blue one<green two>>`, is a blue background one followed by a blue background and green foreground two.
- `<bg:blue,green,b,i text>` is a blue background, green foreground, bold, italic text.

And you can define your own tag, like example code above.

The formal syntax spec can be found in [docs/syntax.ebnf].

Color and modifier supports vary by generator you want to use, see their document for details.

## TODO

- [x] Generator for `tui` crate
- [x] `Generator` trait
- [x] Generator for ansi compliant terminal
- [ ] Generator for `corssterm`
- [ ] Generator for `termion`
- [ ] Generator for `ncurses`

## LICENSE

BSD-3-Clause-Clear, See [LICENSE].

[badge-crate-version]: https://img.shields.io/crates/v/tui-markup?style=for-the-badge
[badge-changelog]: https://img.shields.io/badge/-CHANGELOG-brightgreen?style=for-the-badge
[badge-docs-rs]: https://img.shields.io/docsrs/tui-markup?style=for-the-badge
[badge-deps-state]: https://img.shields.io/librariesio/release/cargo/tui-markup?style=for-the-badge

[crate]: https://crates.io/crates/tui-markup
[doc]: https://docs.rs/tui-markup/latest
[changelog]: https://github.com/7sDream/tui-markup/blob/master/CHANGELOG.md

[`tui-markup-ansi-macro`]: https://github.com/7sDream/tui-markup-ansi-macro

[usage-screenshot]: https://rikka.7sdre.am/files/79f88353-e689-49f6-a0fc-e8f9e373445f.png
[help-text-screenshot]: https://rikka.7sdre.am/files/ee68d36d-b1e7-4575-bb13-e37ba7ead044.png
[indexed-screenshot]: https://rikka.7sdre.am/files/788ef47c-2a8a-4667-b9b7-8f2b1b78e083.png
[doc-builtin-gens]: https://docs.rs/tui-markup/latest/tui_markup/index.html#builtin-generators
[examples/help.txt]: https://github.com/7sDream/tui-markup/blob/master/examples/help.txt
[docs/syntax.ebnf]: https://github.com/7sDream/tui-markup/blob/master/docs/syntax.ebnf
[LICENSE]: https://github.com/7sDream/tui-markup/blob/master/LICENSE
