pub mod github;
pub mod google;
pub mod twitter;

pub fn parse_command(query: &str) -> (&str, &str) {
    match query.find(' ') {
        Some(idx) => (&query[..idx], &query[idx + 1..]),
        None => (&query, ""),
    }
}

extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

fn encode_param(value: &str) -> String {
    utf8_percent_encode(value, FRAGMENT).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_no_options() {
        assert_eq!(
            parse_command("tw"),
            ("tw", "")
        );
    }

    #[test]
    fn test_parse_command_empty_options() {
        assert_eq!(
            parse_command("tw "),
            ("tw", "")
        );
    }

    #[test]
    fn test_parse_command_with_options() {
        assert_eq!(
            parse_command("tw hello world"),
            ("tw", "hello world")
        );
    }
}
