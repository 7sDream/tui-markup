# tui markup

This crate provides a markup language to quickly write colorful and styled terminal text in plain text.

[Document][doc]|[Changelog][changelog]

## Examples

![help-text][help-text-screenshot]

The example is shown in Windows Terminal, using the following command:

`cargo run --example tui --features tui -- examples/help.txt`

The source markup text of this article can be found in [examples/help.txt].

you can change the last argument to your file to render other article, for example `examples/indexed.txt` for a full xterm256 color chart:

![color-chart][indexed-screenshot]

## Generators

Besides the markup syntax and parser, this crate defined a standard compilation process for you to
add this language support for your host application easily.

We provide some builtin implementation for popular crates, See [Builtin generators][doc-builtin-gens].

All examples above uses `tui` generator, but others like `ansi` will work fine too, just change all `tui` in that command to `ansi` too see it.

## Markup syntax

Only one syntax `<tag content>` to add style to content.

`tag` is a `style` list sep by `,`.

`style` has format of `mode:value`, available `mode` are:

- `fg:` for foreground color.
- `bg:` for background color.
- `mod:` for modifiers.

Mode and `:` is optional except `bg`, so `fg:66ccf` = `66ccff`, and `mod:b` = `b`.

Some examples:

- `<green text>` for a green color text, `<66ccff text>` for a #66ccff color text.
- `<bg:blue text>` for a blue background text, `<bg:66ccff text>` for a #66ccff background text.
- `<b text>` for a bold text, `<i text>` for a italic/slant text.
- `<bg:blue one<green two>>`, is a blue background one followed by a blue background and green foreground two.
- `<bg:blue,green,b,i text>` is a blue background, green foreground, bold, italic text.

The forma syntax spec can be found in [docs/syntax.ebnf].

Color and modifier supports vary by generator you want to use, see their document for details.

## TODO

- [x] Generator for `tui` crate
- [x] `Generator` abstract trait
- [x] Generator for ansi terminal
- [ ] Generator for Corssterm
- [ ] Generator for Termion

## LICENSE

BSD-3-Clause-Clear, See [LICENSE].

[doc]: https://docs.rs/tui-markup/latest
[changelog]: https://github.com/7sDream/tui-markup/blob/master/CHANGELOG.md
[help-text-screenshot]: https://rikka.7sdre.am/files/ee68d36d-b1e7-4575-bb13-e37ba7ead044.png
[indexed-screenshot]: https://rikka.7sdre.am/files/788ef47c-2a8a-4667-b9b7-8f2b1b78e083.png
[doc-builtin-gens]: https://docs.rs/tui-markup/latest/tui_markup/index.html#builtin-generators
[examples/help.txt]: <https://github.com/7sDream/tui-markup/blob/master/examples/help.txt>
[docs/syntax.ebnf]: <https://github.com/7sDream/tui-markup/blob/master/docs/syntax.ebnf>
[LICENSE]: <https://github.com/7sDream/tui-markup/blob/master/LICENSE>
