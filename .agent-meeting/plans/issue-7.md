# Plan: Implement String-Case Converters via Parallel Subtasks

## Overview
The Issue explicitly requires a three-way parallel decomposition across `to_snake_case`, `to_camel_case`, and `to_kebab_case`. Although all three subtasks edit `src/lib.rs`, they are constrained to different function bodies plus their own test coverage in the shared `tests` module. To preserve true PR independence, each subtask should implement the shared word-boundary logic locally inside its assigned function rather than introducing a shared helper. This intentionally trades DRY for isolation and conflict avoidance.

## Steps

### Subtask: snake
1. Implement `to_snake_case` in its existing function body only.
2. Encode the full word-boundary contract directly in that implementation:
   - consume runs of `_`, `-`, and whitespace as separators
   - split on lowercase→uppercase transitions (`aA`)
   - split uppercase runs before the last uppercase when followed by lowercase (`AAa`)
   - split letter→digit transitions (`aD`) so digits start the next word
   - preserve non-ASCII bytes as part of the current word without panicking
3. Lowercase each output word in an ASCII-correct way and join with `_`.
4. Add `to_snake_case` unit tests in the existing `tests` module without editing tests owned by other subtasks.
5. Cover at least: PascalCase, camelCase, snake_case, kebab-case, SCREAMING_SNAKE, empty string, single character, `HTTPServer`, and a digit-boundary case such as `foo2bar`.

### Subtask: camel
1. Implement `to_camel_case` in its existing function body only.
2. Encode the same word-boundary contract locally inside this function, independent of the other subtasks.
3. Render the first word lowercase; render subsequent words with an uppercase first ASCII letter and lowercase remainder.
4. Add `to_camel_case` unit tests in the shared `tests` module, limited to this function's assertions.
5. Cover at least: PascalCase, camelCase, snake_case, kebab-case, SCREAMING_SNAKE, empty string, single character, `HTTPServer`, and `foo2bar`.

### Subtask: kebab
1. Implement `to_kebab_case` in its existing function body only.
2. Encode the same word-boundary contract locally inside this function, independent of the other subtasks.
3. Lowercase each output word in an ASCII-correct way and join with `-`.
4. Add `to_kebab_case` unit tests in the shared `tests` module without changing other subtasks' tests.
5. Cover at least: PascalCase, camelCase, snake_case, kebab-case, SCREAMING_SNAKE, empty string, single character, `HTTPServer`, and `foo2bar`.

## Failure modes
- Introducing a shared `split_words` helper creates a hidden cross-PR dependency and undermines independent landing.
- Editing another converter's stub violates the issue contract and creates avoidable merge pressure.
- Inconsistent handling of `AAa` transitions causes acronym splits like `H_T_T_P_Server` instead of `HTTP_Server`.
- Incorrect digit handling can produce `foo_2_bar` instead of `foo_2bar`.
- UTF-8 bugs can occur if implementations slice at invalid byte boundaries instead of iterating safely.

## Test plan
- Each subtask adds tests only for its assigned converter.
- Each test set validates the required cases: PascalCase, camelCase, snake_case, kebab-case, SCREAMING_SNAKE, empty string, single character, and `HTTPServer`.
- Each subtask should also add an explicit `foo2bar` assertion to exercise the `aD` rule.
- Each worker runs `cargo test` before marking the subtask done, assuming the dispatcher's branch state includes whatever scaffolding is necessary for the crate to compile.