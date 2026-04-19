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
    fn push_word(words: &mut Vec<String>, current: &mut String) {
        if !current.is_empty() {
            words.push(std::mem::take(current));
        }
    }

    fn split_words(s: &str) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        let mut words = Vec::new();
        let mut current = String::new();

        for (i, &ch) in chars.iter().enumerate() {
            if matches!(ch, '_' | '-') || ch.is_ascii_whitespace() {
                push_word(&mut words, &mut current);
                continue;
            }

            let prev = current.chars().last();
            let next = chars.get(i + 1).copied();
            let should_split =
                if let Some(prev) = prev {
                    (prev.is_ascii_lowercase() && ch.is_ascii_uppercase())
                        || (prev.is_ascii_alphabetic() && ch.is_ascii_digit())
                        || (prev.is_ascii_uppercase()
                            && ch.is_ascii_uppercase()
                            && next.is_some_and(|next| next.is_ascii_lowercase()))
                } else {
                    false
                };

            if should_split {
                push_word(&mut words, &mut current);
            }

            current.push(ch);
        }

        push_word(&mut words, &mut current);
        words
    }

    split_words(s)
        .into_iter()
        .map(|word| word.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::to_kebab_case;

    #[test]
    fn to_kebab_case_handles_required_inputs() {
        assert_eq!(to_kebab_case("FooBar"), "foo-bar");
        assert_eq!(to_kebab_case("fooBar"), "foo-bar");
        assert_eq!(to_kebab_case("foo_bar"), "foo-bar");
        assert_eq!(to_kebab_case("foo-bar"), "foo-bar");
        assert_eq!(to_kebab_case("SCREAMING_SNAKE"), "screaming-snake");
        assert_eq!(to_kebab_case(""), "");
        assert_eq!(to_kebab_case("X"), "x");
        assert_eq!(to_kebab_case("HTTPServer"), "http-server");
    }

    #[test]
    fn to_kebab_case_splits_letters_before_digits() {
        assert_eq!(to_kebab_case("foo2bar"), "foo-2bar");
    }

    #[test]
    fn to_kebab_case_preserves_non_ascii_without_panicking() {
        assert_eq!(to_kebab_case("GrüßGott"), "grüßgott");
    }
}
