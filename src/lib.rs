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
    let mut words = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (idx, &ch) in chars.iter().enumerate() {
        if ch == '_' || ch == '-' || ch.is_ascii_whitespace() {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            continue;
        }

        let prev = current.chars().last();
        let next = chars.get(idx + 1).copied();
        let should_split = match prev {
            Some(prev_ch) if !current.is_empty() => {
                (prev_ch.is_ascii_lowercase() && ch.is_ascii_uppercase())
                    || (prev_ch.is_ascii_uppercase()
                        && ch.is_ascii_uppercase()
                        && next.is_some_and(|next_ch| next_ch.is_ascii_lowercase()))
                    || (prev_ch.is_ascii_alphabetic() && ch.is_ascii_digit())
            }
            _ => false,
        };

        if should_split {
            words.push(std::mem::take(&mut current));
        }

        current.push(ch);
    }

    if !current.is_empty() {
        words.push(current);
    }

    let mut out = String::new();
    for (idx, word) in words.iter().enumerate() {
        if idx > 0 {
            out.push('_');
        }
        for ch in word.chars() {
            out.push(ch.to_ascii_lowercase());
        }
    }

    out
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
    fn snake_case_from_pascal_case() {
        assert_eq!(to_snake_case("FooBar"), "foo_bar");
    }

    #[test]
    fn snake_case_from_camel_case() {
        assert_eq!(to_snake_case("fooBar"), "foo_bar");
    }

    #[test]
    fn snake_case_from_snake_case() {
        assert_eq!(to_snake_case("foo_bar"), "foo_bar");
    }

    #[test]
    fn snake_case_from_kebab_case() {
        assert_eq!(to_snake_case("foo-bar"), "foo_bar");
    }

    #[test]
    fn snake_case_from_screaming_snake() {
        assert_eq!(to_snake_case("FOO_BAR"), "foo_bar");
    }

    #[test]
    fn snake_case_from_empty_string() {
        assert_eq!(to_snake_case(""), "");
    }

    #[test]
    fn snake_case_from_single_character() {
        assert_eq!(to_snake_case("A"), "a");
    }

    #[test]
    fn snake_case_from_acronym_boundary() {
        assert_eq!(to_snake_case("HTTPServer"), "http_server");
    }

    #[test]
    fn snake_case_normalizes_mixed_separators_and_whitespace() {
        assert_eq!(to_snake_case("foo-\tbar__baz qux"), "foo_bar_baz_qux");
    }

    #[test]
    fn snake_case_splits_letter_to_digit_boundaries() {
        assert_eq!(to_snake_case("foo2bar"), "foo_2bar");
    }

    #[test]
    fn snake_case_preserves_non_ascii_without_panicking() {
        assert_eq!(to_snake_case("naïve Café"), "naïve_café");
    }
}
