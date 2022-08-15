//! Helper functions for create generator.

use std::str::CharIndices;

/// Convert a escaped string in [PlainText][crate::parser::Item::PlainText] into a iterator of unescaped strings.
///
/// ## Example
///
/// ```
/// # use tui_markup::generator::helper::unescape;
///
/// assert_eq!(unescape("\\>").collect::<Vec<_>>(), vec![">"]);
/// assert_eq!(unescape("1\\<2").collect::<Vec<_>>(), vec!["1", "<2"]);
/// ```
pub fn unescape(escaped: &str) -> Unescape {
    Unescape {
        escaped,
        chars: escaped.char_indices(),
        cursor: 0,
        last_is_escape: false,
    }
}

/// Iterator type for [unescape].
#[derive(Debug)]
pub struct Unescape<'a> {
    escaped: &'a str,
    chars: CharIndices<'a>,
    cursor: usize,
    last_is_escape: bool,
}

impl<'a> Unescape<'a> {
    fn try_next(&mut self, idx: usize) -> Option<&'a str> {
        let result = if idx > self.cursor {
            Some(&self.escaped[self.cursor..idx])
        } else {
            None
        };
        self.cursor = idx + 1;
        result
    }
}

impl<'a> Iterator for Unescape<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, c)) = self.chars.next() {
            if !self.last_is_escape && c == '\\' {
                self.last_is_escape = true;
                let next = self.try_next(idx);
                if next.is_some() {
                    return next;
                }
            } else {
                self.last_is_escape = false;
            }
        }
        self.try_next(self.escaped.len())
    }
}

#[cfg(test)]
mod test {
    macro_rules! test_unescape {
        ($escaped:literal => $($result:expr),* $(,)?) => {
           assert_eq!(crate::generator::helper::unescape($escaped).collect::<Vec<_>>(), vec![$($result),+])
        };
    }

    #[test]
    fn test_escaped_string_at_begin() {
        test_unescape!("\\<b" => "<b");
        test_unescape!("\\>b" => ">b");
        test_unescape!("\\\\b" => "\\b");
    }

    #[test]
    fn test_escaped_string_at_middle() {
        test_unescape!("a\\<b" => "a", "<b");
        test_unescape!("a\\>b" => "a", ">b");
        test_unescape!("a\\\\b" => "a", "\\b");
    }

    #[test]
    fn test_escaped_string_at_end() {
        test_unescape!("a\\<" => "a", "<");
        test_unescape!("a\\>" => "a", ">");
        test_unescape!("a\\\\" => "a", "\\");
    }

    #[test]
    fn test_escaped_string_multi() {
        test_unescape!("1\\<2\\<3 \\\\ 3\\>2\\>1" => "1", "<2", "<3 ", "\\ 3", ">2", ">1");
    }
}
