# Guide for adding a Rust example

1. Identify the book edition, chapter, section, and the mathematical idea—not merely an exercise number.
2. Create `examples/chXX/section-YY/short-name/main.rs` with a standalone `fn main` and deterministic unit tests.
3. Explain algorithms and non-obvious invariants with comments, not a line-by-line restatement.
4. Add an adjacent `README.md` containing the problem, approach, expected behavior, complexity, Rust-specific details, and an extension challenge.
5. Add an explicit `[[bin]]` target in `Cargo.toml` and an entry to `manifest.json` for the static catalog.
6. Run `cargo fmt --all -- --check`, `cargo test --all-targets`, `cargo clippy --all-targets -- -D warnings`. Keep dependencies minimal and reproducible.
7. Prefer original code, commentary, and tests. Link to the official text; do not copy its prose or any large copyrighted listing.

## Translation patterns

- Scheme pair/list => `Vec`, `Box`, or a recursive enum, depending on the invariant.
- Scheme tagged data => Rust enum and exhaustive `match`.
- Procedure as value => a generic `Fn` bound or trait object.
- Tail-recursive loop => explicit `loop` for portable constant-stack execution.
- Delayed stream => `Iterator`, with separate memoization if sharing is essential.
- Mutable shared cell => `Rc<RefCell<T>>` (single thread) or `Arc<Mutex<T>>` (multi-thread), if justified.
- Metacircular Scheme evaluator => a deliberately non-metacircular Rust interpreter; explain the change in assumptions.
