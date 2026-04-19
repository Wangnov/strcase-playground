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
pub fn to_camel_case(s: &str) -> String {
    let mut words: Vec<&str> = Vec::new();
    let mut word_start: Option<usize> = None;
    let mut prev_idx = 0usize;
    let mut prev_char: Option<char> = None;
    let mut prev_prev_char: Option<char> = None;

    for (idx, ch) in s.char_indices() {
        if ch == '_' || ch == '-' || ch.is_ascii_whitespace() {
            if let Some(start) = word_start.take() {
                words.push(&s[start..idx]);
            }
            prev_prev_char = None;
            prev_char = None;
            prev_idx = idx;
            continue;
        }

        if word_start.is_none() {
            word_start = Some(idx);
            prev_prev_char = prev_char;
            prev_char = Some(ch);
            prev_idx = idx;
            continue;
        }

        let split_idx = if let Some(prev) = prev_char {
            if prev.is_ascii_lowercase() && ch.is_ascii_uppercase() {
                Some(idx)
            } else if prev.is_ascii_alphabetic() && ch.is_ascii_digit() {
                Some(idx)
            } else if let Some(prev_prev) = prev_prev_char {
                if prev_prev.is_ascii_uppercase()
                    && prev.is_ascii_uppercase()
                    && ch.is_ascii_lowercase()
                {
                    Some(prev_idx)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(boundary) = split_idx {
            if let Some(start) = word_start {
                words.push(&s[start..boundary]);
            }
            word_start = Some(boundary);
        }

        prev_prev_char = prev_char;
        prev_char = Some(ch);
        prev_idx = idx;
    }

    if let Some(start) = word_start {
        words.push(&s[start..]);
    }

    if words.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    for (word_index, word) in words.into_iter().enumerate() {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            if word_index == 0 {
                out.push(first.to_ascii_lowercase());
            } else {
                out.push(first.to_ascii_uppercase());
            }
            for ch in chars {
                out.push(ch.to_ascii_lowercase());
            }
        }
    }

    out
}

/// Convert `s` to `kebab-case`. Stubbed — implement in issue-specific PR.
pub fn to_kebab_case(_s: &str) -> String {
    unimplemented!("to_kebab_case — implement in its own subtask")
}

#[cfg(test)]
mod tests {
    use super::to_camel_case;

    #[test]
    fn camel_case_from_pascal_case() {
        assert_eq!(to_camel_case("FooBar"), "fooBar");
    }

    #[test]
    fn camel_case_from_camel_case() {
        assert_eq!(to_camel_case("fooBar"), "fooBar");
    }

    #[test]
    fn camel_case_from_snake_case() {
        assert_eq!(to_camel_case("foo_bar"), "fooBar");
    }

    #[test]
    fn camel_case_from_kebab_case() {
        assert_eq!(to_camel_case("foo-bar"), "fooBar");
    }

    #[test]
    fn camel_case_from_screaming_snake() {
        assert_eq!(to_camel_case("FOO_BAR"), "fooBar");
    }

    #[test]
    fn camel_case_from_whitespace_separated_words() {
        assert_eq!(to_camel_case("foo \t bar\nbaz"), "fooBarBaz");
    }

    #[test]
    fn camel_case_preserves_acronym_boundaries() {
        assert_eq!(to_camel_case("HTTPServer"), "httpServer");
    }

    #[test]
    fn camel_case_splits_letter_digit_boundaries() {
        assert_eq!(to_camel_case("foo2bar"), "foo2bar");
    }

    #[test]
    fn camel_case_handles_empty_input() {
        assert_eq!(to_camel_case(""), "");
    }

    #[test]
    fn camel_case_handles_single_character() {
        assert_eq!(to_camel_case("A"), "a");
    }

    #[test]
    fn camel_case_preserves_non_ascii_without_panicking() {
        assert_eq!(to_camel_case("StraßeÖl"), "straßeÖl");
    }
}
