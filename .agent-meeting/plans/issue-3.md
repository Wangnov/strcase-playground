# Plan: Implement the assigned strcase converter

## Overview
This issue should target exactly one converter in `src/lib.rs`: `to_snake_case`, `to_camel_case`, or `to_kebab_case`, depending on the issue assignment. The work must stay issue-scoped because parallel workers are landing changes in the same file, and `AGENTS.md` forbids touching functions not owned by this task. The implementation should satisfy the shared word-boundary contract for the assigned converter only, add tests for that converter only, and leave the other `unimplemented!()` stubs unchanged.

## Steps
1. Confirm which single function this issue owns from the issue title/body before planning code changes any further.
2. In `src/lib.rs`, implement only that function's parsing and output formatting logic, keeping all other converter stubs untouched.
3. Apply the shared boundary rules inside the owned function's logic:
   - consume `_`, `-`, and whitespace as separators;
   - split on lowercase→uppercase transitions;
   - split uppercase runs before the last uppercase when followed by lowercase;
   - split letter→digit so digits begin the next word;
   - preserve non-ASCII UTF-8 bytes within words without panicking.
4. Format the resulting words according to the assigned converter:
   - snake: lowercase words joined by `_`;
   - camel: first word lowercase, later words capitalized with lowercase remainder, no separator;
   - kebab: lowercase words joined by `-`.
5. Add inline unit tests in the existing `tests` module for the owned function covering:
   - PascalCase input
   - camelCase input
   - snake_case input
   - kebab-case input
   - SCREAMING_SNAKE input
   - empty string
   - single character
   - `HTTPServer`
   - one non-ASCII input to verify no panic and stable preservation behavior
6. Run `cargo test` and require a green result before handing the task back to the dispatcher.

## Failure modes
- Planning all three functions at once violates issue ownership and creates avoidable merge conflicts in `src/lib.rs`.
- Introducing a shared helper for all converters is false parallelism in a single-file crate; separate PRs will collide on the helper and its call sites.
- The `AAa` boundary is easy to get wrong; `HTTPServer` must become `["HTTP", "Server"]`.
- The digit rule is asymmetric; `foo2bar` must split as `["foo", "2bar"]`, not `["foo2", "bar"]`.
- Empty separator runs must not generate empty words.
- Non-ASCII input must not panic even if ASCII-only case folding is used.

## Test plan
- Add unit tests only for the assigned function.
- Verify the required boundary matrix from `AGENTS.md`.
- Run `cargo test` and ensure the full crate stays green with parallel-task stubs untouched.