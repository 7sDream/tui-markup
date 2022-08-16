# tui markup

This crate provides a markup language to quickly write colorful and styled terminal text in plain text.

[Document][doc]|[Changelog][changelog]

## Example

![help-text][help-text-screenshot]

The source markup text of this article can be found in [examples/help.txt].

Besides the language syntax and parser, this crate defined a standard compilation process for you to
add this language support for your host application easily.

We provide some builtin implementation for popular crates, See [Builtin generators][doc-builtin-gens].

The example screenshot is using the `tui` generator, print in Windows Terminal, using the following command:

`cargo run --example tui --features tui -- examples/help.txt`:

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

The forma syntax spec can be found in [docs/syntax.ebnf].

Color and modifier supports vary by generator you want to use, see their document for details.

## LICENSE

BSD-3-Clause-Clear, See [LICENSE].

[doc]: https://docs.rs/tui-markup/latest
[changelog]: https://github.com/7sDream/tui-markup/blob/master/CHANGELOG.md
[help-text-screenshot]: https://rikka.7sdre.am/files/ee68d36d-b1e7-4575-bb13-e37ba7ead044.png
[doc-builtin-gens]: https://docs.rs/tui-markup/0.2.0-alpha/tui_markup/index.html#builtin-generators
[examples/help.txt]: <https://github.com/7sDream/tui-markup/blob/master/examples/help.txt>
[docs/syntax.ebnf]: <https://github.com/7sDream/tui-markup/blob/master/docs/syntax.ebnf>
[LICENSE]: <https://github.com/7sDream/tui-markup/blob/master/LICENSE>
