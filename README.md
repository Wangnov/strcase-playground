# strcase

A minimal Rust string case conversion library — plus the agent-meeting
**M4.1 end-to-end validation target**.

## API (once implemented)

```rust
use strcase::{to_snake_case, to_camel_case, to_kebab_case};

assert_eq!(to_snake_case("FooBar"), "foo_bar");
assert_eq!(to_camel_case("foo_bar"), "fooBar");
assert_eq!(to_kebab_case("FooBar"), "foo-bar");
```

See `CLAUDE.md` for the full word-boundary contract all three converters
must respect.

## Why this repo exists

It's a deliberately tiny library so we can stress-test the multi-agent
dispatcher (`agent-meeting`) in two ways:

1. **Multi-issue parallel** — three issues, three pipelines running at once.
2. **Single-issue, multi-subtask parallel** — one issue decomposes into
   three parallel worker subtasks via the orchestrator's multi-`<<execute>>`
   dispatch.

Each converter is genuinely independent, so parallelism is real (not fake).
