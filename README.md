# Structure and Interpretation of Computer Programs — Rust & Python Companion

An original, open-source learning companion to the classic **Wizard Book**, with **95 independently written and tested programs in both Rust and Python**, adjacent example explanations, introductions to all 22 major sections and 95 subsection study guides (`1.1.1`–`5.5.7` plus Source Academy's `3.2.5` web supplement), automated checks, and a zero-build static site with embedded syntax-highlighted source code.

**Status:** broad concept and multi-part system coverage across Chapters 1–5; this does **not** yet contain every exercise or snippet from all editions. See [COVERAGE.md](COVERAGE.md).

## Start here

- [Open the static code/Markdown browser](index.html) (embeds Rust + Python source tabs with syntax highlighting; enable GitHub Pages for hosted browsing).
- [Chapter introductions](chapters/) and [edition crosswalk](docs/editions.md).
- Browse `examples/chXX/section-YY/example-name/{main.rs,main.py,README.md}`.
- Verify all Rust examples: `cargo test --all-targets` (single target: `cargo run --bin ch01-newton-sqrt`).
- Verify a Python example: `python3 -m unittest examples/ch01/section-01/newton-sqrt/main.py` (or run `python3 examples/ch01/section-01/newton-sqrt/main.py`).

## Editions and evolving adaptations

The 1984 first and 1996 second editions use Scheme. The 2022 JavaScript adaptation and the actively developed 2026 Python adaptation (SICPy) use different host languages. This companion tracks their shared ideas while preserving language-specific distinctions. Neither adaptation is a numbered successor to the Scheme editions.

The textbook is by Harold Abelson and Gerald Jay Sussman, with Julie Sussman; the JavaScript adaptation is by Martin Henz and Tobias Wrigstad. Official sources: [MIT Press second edition](https://mitpress.mit.edu/9780262510875/structure-and-interpretation-of-computer-programs/), [MIT Press JavaScript edition](https://mitpress.mit.edu/9780262543231/structure-and-interpretation-of-computer-programs/), [edition comparison](https://sicp.sourceacademy.org/), and [SICPy project](https://sicp.sourceacademy.org/split_py/chapters/making-of.html).

## Learning path

Read chapter introductions, then numbered subsection `INTRO.md` notes, then each example's `README.md` alongside its `main.rs` and `main.py` implementations. Every code example includes executable assertions and uses only the Rust and Python standard libraries.

## Static hosting

The root `index.html` fetches `manifest.json`, Markdown files, and both Rust and Python source files using relative paths, embedding syntax-highlighted Rust (`main.rs`), Python (`main.py`), and side-by-side code views directly on each example page. Host the repository root with GitHub Pages (deploy from `main` / `/ (root)`) or any static web server. `file://` browsing cannot normally fetch local files because of browser restrictions. No bundler or CDN is needed.

## Authorship and scope

The Rust and Python implementations and explanations are independently authored educational adaptations, **not** verbatim reproductions of the published textbook. Exact examples, exercises, and solution coverage across editions remain a long-term inventory task. Contributions should follow [docs/porting-guide.md](docs/porting-guide.md) and [coverage methodology](docs/coverage-methodology.md). Use `python3 tools/verify_catalog.py` to validate the file index, Rust/Python entrypoints, and internal links.
