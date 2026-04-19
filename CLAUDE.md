# strcase — project rules for agent-meeting workers

This crate exposes three independent string-case conversion functions. It is
the agent-meeting M4.1 end-to-end validation target.

## What you may touch

- `src/lib.rs` — the one and only source file for now. Implement the function
  the Issue assigns you. **Do not touch other functions' stubs** — they are
  owned by other subtask PRs landing in parallel.
- Inline unit tests inside `src/lib.rs`'s `tests` module. Each subtask adds
  its own `#[test]` fns; avoid renaming or editing tests that aren't yours.
- `Cargo.toml` — only if your task legitimately needs a dependency (discuss
  in the plan first). Keep the crate dependency-free if possible.

## What you MUST NOT touch

- `src/lib.rs` functions that aren't yours (leave other `unimplemented!()`
  stubs alone — parallel subtask PRs are landing them).
- `CLAUDE.md`, `README.md`, `.agent-meeting/` — infrastructure, not yours.
- `.gitignore` unless adding something directly caused by your task.

## Contract: word boundaries (shared by all three converters)

All three converters split the input into **words** using these boundaries,
then re-join with the format the target case dictates:

1. ASCII letters/digits form words; runs of `_`, `-`, or whitespace are
   word separators and are consumed.
2. `aA` — lowercase followed by uppercase starts a new word
   (`"fooBar"` → `["foo", "Bar"]`).
3. `AAa` — an uppercase run followed by lowercase splits between the last
   two uppercase letters (`"HTTPServer"` → `["HTTP", "Server"]`, not
   `["H", "T", "T", "P", "Server"]`).
4. `aD` — a letter followed by a digit starts a new word
   (`"foo2bar"` → `["foo", "2bar"]` — digits attach to the word they start).
5. Empty input returns empty output. No panics on any UTF-8 input; non-ASCII
   characters may be preserved verbatim inside a word without splitting
   (we're not building a full Unicode case system — ASCII-correctness is
   enough).

## Output formats

- `to_snake_case("FooBar")` → `"foo_bar"` (all lowercase, `_` between words).
- `to_camel_case("foo_bar")` → `"fooBar"` (first word lowercase, subsequent
  words' first letter uppercase, rest lowercase, no separator).
- `to_kebab_case("FooBar")` → `"foo-bar"` (all lowercase, `-` between words).

## Testing discipline

- Every converter function must land with unit tests covering at least:
  `PascalCase` input, `camelCase` input, `snake_case` input, `kebab-case`
  input, `SCREAMING_SNAKE` input, empty string, single character, and an
  all-uppercase acronym case (`HTTPServer`).
- `cargo test` must be green before you emit your terminal `WorkerResult`
  with `status: "done"`.

## You are not allowed to run git or gh

The agent-meeting dispatcher owns all git/gh operations. Don't run
`git add`, `git commit`, `git push`, `gh pr create`, or anything like that.
Just write code; the dispatcher takes care of the rest.
