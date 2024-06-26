/// Convert a escaped string into a iterator of unescaped strings.
///
/// In implementation, the returned iterator will skip any `\` character
/// except itself is after a skipped `\`.
///
/// ## Example
///
/// ```
/// # use tui_markup::generator::helper::unescape;
/// assert_eq!(unescape("\\>").collect::<Vec<_>>(), vec![">"]);
/// assert_eq!(unescape("1\\<2").collect::<Vec<_>>(), vec!["1", "<2"]);
///
/// // notice the final `\` is not returned
/// assert_eq!(unescape("A\\\\B\\").collect::<Vec<_>>(), vec!["A", "\\B"]);
/// ```
#[must_use]
pub fn unescape(escaped: &str) -> Unescape {
    let cursor = escaped.starts_with('\\').into();
    Unescape { escaped, cursor }
}

/// Iterator type for [unescape].
#[derive(Debug)]
pub struct Unescape<'a> {
    escaped: &'a str,
    cursor: usize,
}

impl<'a> Iterator for Unescape<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.escaped.len() {
            return None;
        }

        let start = self.cursor + 1;
        let end = if start >= self.escaped.len() {
            self.escaped.len()
        } else {
            self.escaped[start..]
                .find('\\')
                .map_or(self.escaped.len(), |i| start + i)
        };

        let result = Some(&self.escaped[self.cursor..end]);
        self.cursor = end + 1;

        result
    }
}

#[cfg(test)]
mod test {
    macro_rules! test_unescape {
        ($escaped:expr => $($result:expr),* $(,)?) => {
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
