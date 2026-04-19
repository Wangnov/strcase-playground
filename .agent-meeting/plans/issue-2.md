# Plan: Implement `to_camel_case`

## Overview
Implement `to_camel_case` entirely inside `src/lib.rs` without touching the other converter stubs or introducing shared helpers that parallel subtasks are likely to conflict on. The function should do a single left-to-right pass that identifies word boundaries from the shared contract, then emit camelCase using ASCII-only case folding so non-ASCII bytes are preserved and never cause panics.

## Steps
1. Implement boundary detection locally inside `to_camel_case`.
   - Treat `_`, `-`, and ASCII whitespace as consumed separators.
   - Start a new word on `lowercase -> uppercase`.
   - Split `AAa` before the last uppercase so `HTTPServer` becomes `HTTP` + `Server`.
   - Split on `letter -> digit` so `foo2bar` becomes `foo` + `2bar`.
   - Keep non-ASCII characters inside the current word rather than trying to classify them with Unicode case rules.

2. Build output directly from detected word slices.
   - Return `""` for empty input.
   - Lowercase the first word with ASCII-only lowering.
   - For each later word, uppercase its first ASCII letter and ASCII-lowercase the remainder.
   - Leave digits and non-ASCII characters unchanged except where ASCII case folding applies to ASCII letters around them.

3. Add isolated unit tests in the existing `tests` module for this function only.
   - Required coverage: `PascalCase`, `camelCase`, `snake_case`, `kebab-case`, `SCREAMING_SNAKE`, empty string, single character, `HTTPServer`.
   - Include whitespace-separated input explicitly, since separators include whitespace.
   - Include `foo2bar` to lock the `letter -> digit` rule.
   - Include at least one non-ASCII smoke test to verify “no panic, preserve verbatim inside a word” behavior.

4. Verify with `cargo test`.
   - The success condition is green tests without modifying the other unimplemented converters or their tests.

## Failure modes
- Splitting `AAa` at the wrong position and producing `HttPserver`-style corruption.
- Using full Unicode `to_lowercase` / `to_uppercase` and accidentally changing non-ASCII characters contrary to the crate’s ASCII-scoped contract.
- Treating digits as separators instead of the start of a new word.
- Adding shared helpers or broad test edits that create avoidable merge conflicts with the parallel subtasks.

## Test plan
- `cargo test` passes for the new `to_camel_case` cases and existing unaffected tests.
- Spot-check expected outputs:
  - `FooBar -> fooBar`
  - `fooBar -> fooBar`
  - `foo_bar -> fooBar`
  - `foo-bar -> fooBar`
  - `FOO_BAR -> fooBar`
  - `HTTPServer -> httpServer`
  - `foo2bar -> foo2bar`
  - `"" -> ""`
  - `"A" -> "a"`