# Plan: Implement one strcase converter without cross-subtask coupling

## Overview
This repository is intentionally structured for parallel landing of separate converter implementations, but the constraints mean a single issue should cover exactly one target function in `src/lib.rs`, not all three at once. The implementation must stay confined to the assigned function and that function's own unit tests. To preserve parallelism and avoid merge conflicts, do not introduce a shared word-splitting helper in common code; instead, implement the boundary-scanning logic locally inside the assigned converter. This accepts some duplication in exchange for independent PRs that can land safely.

## Steps
1. Identify the single assigned function for this issue: `to_snake_case`, `to_camel_case`, or `to_kebab_case`.
2. Implement local word-boundary scanning inside that function only.
3. Apply the shared boundary contract during scanning:
   - Treat runs of `_`, `-`, and ASCII whitespace as separators.
   - Split on `aA` transitions.
   - Split on `AAa` transitions by keeping the final uppercase with the following lowercase word.
   - Split on letter-to-digit transitions so digits start the next word.
   - Preserve non-ASCII bytes/chars inside the current word rather than trying to case-fold or split on them.
4. Re-join parsed words in the format required by the assigned function:
   - `snake`: lowercase each word, join with `_`
   - `camel`: lowercase the first word; capitalize the first ASCII letter of subsequent words and lowercase the rest
   - `kebab`: lowercase each word, join with `-`
5. Add unit tests for the assigned function only, covering:
   - `PascalCase`
   - `camelCase`
   - `snake_case`
   - `kebab-case`
   - `SCREAMING_SNAKE`
   - empty string
   - single character
   - acronym case like `HTTPServer`
6. Ensure the new tests exercise only the implemented function and do not require other converter stubs to be implemented.
7. Run `cargo test` and verify the crate is green under the current branch state.

## Failure modes
- Introducing a shared helper outside the assigned function creates cross-PR conflicts in `src/lib.rs`.
- Mis-handling `AAa` causes `HTTPServer` to split into per-letter fragments instead of `HTTP` + `Server`.
- Mis-handling `aD` causes digits to remain attached to the previous word instead of starting the next word.
- Applying ASCII lowercasing/capitalization naively to non-ASCII content may corrupt preserved UTF-8 text.
- Adding tests that call unimplemented sibling converters will make the subtask non-independent.

## Test plan
- Unit tests for the assigned converter cover the required input styles and edge cases.
- Include at least one assertion for separator normalization across mixed delimiters.
- Run `cargo test` after implementation and confirm no panic on UTF-8 input cases used by the new tests.

## Notes
- If the dispatcher wants all three converters planned, that should be tracked as three separate issues with the same implementation pattern, not as one decomposed plan with shared code.
- Duplication of the boundary scanner across the three functions is acceptable here because repository workflow optimizes for independent landing, not abstraction reuse.