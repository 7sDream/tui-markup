# tui markup

This crate provides a markup language to quickly write colorful and styled terminal text in plain text.

[Document][doc]|[Changelog][changelog]

## Example

This is the output of `cargo run --example help`:

![help-text][help-text-screenshot]

The source markup text of this article can be found in [examples/help.txt].

## Syntax

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

The forma syntax spec can be found in [syntax.ebnf], with complete list of available color and modifiers.

## LICENSE

BSD-3-Clause-Clear, See [LICENSE].

[doc]: https://docs.rs/tui-markup/latest
[changelog]: https://github.com/7sDream/tui-markup/blob/master/CHANGELOG.md
[help-text-screenshot]: https://rikka.7sdre.am/files/37778eea-660b-47a6-bfd1-43979b5c703b.png
[examples/help.txt]: https://github.com/7sDream/tui-markup/blob/master/examples/help.txt
[syntax.ebnf]: https://github.com/7sDream/tui-markup/blob/master/syntax.ebnf
[LICENSE]: https://github.com/7sDream/tui-markup/blob/master/LICENSE
