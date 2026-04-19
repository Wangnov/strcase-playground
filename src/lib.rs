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
    fn is_sep(ch: char) -> bool {
        ch == '_' || ch == '-' || ch.is_whitespace()
    }

    let chars: Vec<char> = s.chars().collect();
    let mut words = Vec::new();
    let mut current = String::new();

    for (idx, &ch) in chars.iter().enumerate() {
        if is_sep(ch) {
            if !current.is_empty() {
                words.push(current);
                current = String::new();
            }
            continue;
        }

        let prev = idx.checked_sub(1).and_then(|i| chars.get(i)).copied();
        let next = chars.get(idx + 1).copied();

        let starts_new_word = if current.is_empty() {
            false
        } else if let Some(prev_ch) = prev {
            (prev_ch.is_ascii_lowercase() && ch.is_ascii_uppercase())
                || (prev_ch.is_ascii_alphabetic() && ch.is_ascii_digit())
                || (prev_ch.is_ascii_uppercase()
                    && ch.is_ascii_uppercase()
                    && next.is_some_and(|next_ch| next_ch.is_ascii_lowercase()))
        } else {
            false
        };

        if starts_new_word {
            words.push(current);
            current = String::new();
        }

        current.push(ch);
    }

    if !current.is_empty() {
        words.push(current);
    }

    let mut result = String::new();

    for (idx, word) in words.into_iter().enumerate() {
        let mut word_chars = word.chars();
        if let Some(first) = word_chars.next() {
            if idx == 0 {
                result.push(first.to_ascii_lowercase());
            } else {
                result.push(first.to_ascii_uppercase());
            }

            for ch in word_chars {
                result.push(ch.to_ascii_lowercase());
            }
        }
    }

    result
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
        assert_eq!(to_camel_case("SCREAMING_SNAKE"), "screamingSnake");
    }

    #[test]
    fn camel_case_empty_input() {
        assert_eq!(to_camel_case(""), "");
    }

    #[test]
    fn camel_case_single_character() {
        assert_eq!(to_camel_case("X"), "x");
    }

    #[test]
    fn camel_case_preserves_acronym_boundary() {
        assert_eq!(to_camel_case("HTTPServer"), "httpServer");
    }

    #[test]
    fn camel_case_splits_letter_digit_boundary() {
        assert_eq!(to_camel_case("foo2bar"), "foo2bar");
    }

    #[test]
    fn camel_case_splits_unicode_whitespace() {
        assert_eq!(to_camel_case("foo\u{2003}bar"), "fooBar");
    }
}
