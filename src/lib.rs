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
pub fn to_snake_case(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut words = Vec::new();
    let mut current = String::new();

    for (i, ch) in chars.iter().copied().enumerate() {
        if ch == '_' || ch == '-' || ch.is_ascii_whitespace() {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            continue;
        }

        let starts_new_word = if let Some(prev) = current.chars().last() {
            let next = chars.get(i + 1).copied();

            (prev.is_ascii_lowercase() && ch.is_ascii_uppercase())
                || (prev.is_ascii_uppercase()
                    && ch.is_ascii_uppercase()
                    && next.is_some_and(|next_ch| next_ch.is_ascii_lowercase()))
                || (prev.is_ascii_alphabetic() && ch.is_ascii_digit())
        } else {
            false
        };

        if starts_new_word {
            words.push(std::mem::take(&mut current));
        }

        current.push(ch);
    }

    if !current.is_empty() {
        words.push(current);
    }

    words
        .into_iter()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

/// Convert `s` to `camelCase`. Stubbed — implement in issue-specific PR.
pub fn to_camel_case(_s: &str) -> String {
    unimplemented!("to_camel_case — implement in its own subtask")
}

/// Convert `s` to `kebab-case`. Stubbed — implement in issue-specific PR.
pub fn to_kebab_case(_s: &str) -> String {
    unimplemented!("to_kebab_case — implement in its own subtask")
}

#[cfg(test)]
mod tests {
    use super::to_snake_case;

    #[test]
    fn snake_case_converts_pascal_case() {
        assert_eq!(to_snake_case("FooBar"), "foo_bar");
    }

    #[test]
    fn snake_case_converts_camel_case() {
        assert_eq!(to_snake_case("fooBar"), "foo_bar");
    }

    #[test]
    fn snake_case_preserves_snake_case_shape() {
        assert_eq!(to_snake_case("foo_bar"), "foo_bar");
    }

    #[test]
    fn snake_case_converts_kebab_case() {
        assert_eq!(to_snake_case("foo-bar"), "foo_bar");
    }

    #[test]
    fn snake_case_converts_screaming_snake() {
        assert_eq!(to_snake_case("SCREAMING_SNAKE"), "screaming_snake");
    }

    #[test]
    fn snake_case_returns_empty_for_empty_input() {
        assert_eq!(to_snake_case(""), "");
    }

    #[test]
    fn snake_case_handles_single_character() {
        assert_eq!(to_snake_case("X"), "x");
    }

    #[test]
    fn snake_case_keeps_acronyms_together() {
        assert_eq!(to_snake_case("HTTPServer"), "http_server");
    }

    #[test]
    fn snake_case_splits_letter_digit_boundary() {
        assert_eq!(to_snake_case("foo2bar"), "foo_2bar");
    }
}
