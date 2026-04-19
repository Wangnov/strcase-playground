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
pub fn to_kebab_case(_s: &str) -> String {
    unimplemented!("to_kebab_case — implement in its own subtask")
}

#[cfg(test)]
mod tests {
    // Intentionally empty. Each subtask PR must add its own tests covering
    // the shared word-boundary contract (see CLAUDE.md) for the function it
    // implements. Until that happens, `cargo test` has nothing to run and
    // trivially passes.
}
