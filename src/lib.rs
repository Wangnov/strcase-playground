//! `strcase` — minimal string case conversions.
//!
//! This crate is the validation target for agent-meeting M4.1. It exposes
//! three converter functions that are independent of each other and can be
//! implemented as genuinely parallel subtasks:
//!
//! - [`to_snake_case`]   — "FooBar" → "foo_bar"
//! - [`to_camel_case`]   — "foo_bar" → "fooBar"
//! - [`to_kebab_case`]   — "FooBar" → "foo-bar"
//!
//! All three functions must satisfy the shared word-boundary contract
//! described in `CLAUDE.md`. Each function ships its own unit tests in the
//! `tests` module at the bottom of this file; the CI gate is `cargo test`.

/// Convert `s` to `snake_case`. Stubbed — implement in issue-specific PR.
pub fn to_snake_case(_s: &str) -> String {
    unimplemented!("to_snake_case — implement in its own subtask")
}

/// Convert `s` to `camelCase`. Stubbed — implement in issue-specific PR.
pub fn to_camel_case(_s: &str) -> String {
    unimplemented!("to_camel_case — implement in its own subtask")
}

/// Convert `s` to `kebab-case`. Stubbed — implement in issue-specific PR.
pub fn to_kebab_case(s: &str) -> String {
    let mut words: Vec<&str> = Vec::new();
    let mut word_start: Option<usize> = None;
    let mut prev: Option<(usize, char)> = None;

    for (idx, ch) in s.char_indices() {
        let is_separator = ch == '_' || ch == '-' || ch.is_whitespace();
        if is_separator {
            if let Some(start) = word_start.take() {
                words.push(&s[start..idx]);
            }
            prev = None;
            continue;
        }

        if let Some((prev_idx, prev_ch)) = prev {
            let boundary = (prev_ch.is_ascii_lowercase() && ch.is_ascii_uppercase())
                || (prev_ch.is_ascii_alphabetic() && ch.is_ascii_digit())
                || (prev_ch.is_ascii_uppercase() && ch.is_ascii_lowercase() && {
                    let mut lookback = s[..prev_idx].chars().rev();
                    let before_prev = lookback.next();
                    before_prev.is_some_and(|c| c.is_ascii_uppercase())
                });

            if boundary {
                let split_at = if prev_ch.is_ascii_uppercase() && ch.is_ascii_lowercase() {
                    prev_idx
                } else {
                    idx
                };

                if let Some(start) = word_start {
                    words.push(&s[start..split_at]);
                }
                word_start = Some(split_at);
            }
        } else {
            word_start = Some(idx);
        }

        prev = Some((idx, ch));
    }

    if let Some(start) = word_start {
        words.push(&s[start..]);
    }

    words
        .into_iter()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::to_kebab_case;

    #[test]
    fn kebab_case_from_pascal_case() {
        assert_eq!(to_kebab_case("FooBar"), "foo-bar");
    }

    #[test]
    fn kebab_case_from_camel_case() {
        assert_eq!(to_kebab_case("fooBar"), "foo-bar");
    }

    #[test]
    fn kebab_case_from_snake_case() {
        assert_eq!(to_kebab_case("foo_bar"), "foo-bar");
    }

    #[test]
    fn kebab_case_from_existing_kebab_case() {
        assert_eq!(to_kebab_case("foo-bar"), "foo-bar");
    }

    #[test]
    fn kebab_case_from_screaming_snake() {
        assert_eq!(to_kebab_case("FOO_BAR"), "foo-bar");
    }

    #[test]
    fn kebab_case_handles_empty_string() {
        assert_eq!(to_kebab_case(""), "");
    }

    #[test]
    fn kebab_case_handles_single_character() {
        assert_eq!(to_kebab_case("X"), "x");
    }

    #[test]
    fn kebab_case_keeps_acronym_groups() {
        assert_eq!(to_kebab_case("HTTPServer"), "http-server");
    }

    #[test]
    fn kebab_case_splits_letter_digit_boundaries() {
        assert_eq!(to_kebab_case("foo2bar"), "foo-2bar");
    }

    #[test]
    fn kebab_case_does_not_split_non_ascii_before_digits() {
        assert_eq!(to_kebab_case("é2bar"), "é2bar");
    }

    #[test]
    fn kebab_case_splits_on_unicode_whitespace() {
        assert_eq!(to_kebab_case("foo\u{2003}bar"), "foo-bar");
    }
}
