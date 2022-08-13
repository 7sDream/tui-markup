# tui markup

This crate provided a markup language to quickly convert plain text into colorful terminal text of [tui] crate.

[Document][doc]

## Example

This is the output of `cargo run --example help`:

![help-text][help-text-screenshot]

The source markup text generated this can be found in [examples/help.txt].

## Syntax

Only one syntax `<tag content>` to add style to content.

`tag` is a `style` list sep by `,`.

`style` has format of `mode:value`, available `mode` are:

- `fg:` for foreground color.
- `bg:` for background color.
- `mod:` for modifiers.

Mode prefix is optional except `bg:`, so `fg:66ccf` = `66ccff`, and `mod:b` = `b`.

Some examples:

- `<green text>` for a green color text, `<66ccff text>` for a #66ccff color text.
- `<bg:blue text>` for a blue background text, `<bg:66ccff text>` for a #66ccff background text.
- `<b text>` for a bold text, `<i text>` for a italic/slant text.
- `<bg:blue one<green two>>`, is a blue background one followed by a blue background and green foreground two.
- `<bg:blue,green,b,i text>` is a blue background, green foreground, bold, italic text.

The forma syntax spec can be found in [crate document][doc-syntax], with complete list of available color and modifiers.

## LICENSE

BSD-3-Clause-Clear, See [LICENSE].

[tui]: https://docs.rs/tui/latest
[doc]: https://docs.rs/tui-markup/latest
[doc-syntax]: https://docs.rs/tui-markup/latest#Syntax
[help-text-screenshot]: empty
[examples/help.txt]: https://github.com/7sDream/tui-markup/blob/master/examples/help.txt
[LICENSE]: https://github.com/7sDream/tui-markup/blob/master/LICENSE
