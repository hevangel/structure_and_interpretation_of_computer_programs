# Structure and Interpretation of Computer Programs — Rust companion

An original, open-source learning companion to the classic **Wizard Book**, with 73 independently written and commented Rust programs, adjacent example explanations, introductions to all 22 major sections and 95 numbered subsections, automated checks, and a zero-build static site.

**Status:** scaffold and representative examples; this does **not** yet contain every example from all editions. See [COVERAGE.md](COVERAGE.md).

## Start here

- [Open the static code/Markdown browser](index.html) (enable GitHub Pages for hosted browsing).
- [Chapter introductions](chapters/) and [edition crosswalk](docs/editions.md).
- Browse `examples/chXX/section-YY/example-name/{main.rs,README.md}`.
- Run `cargo test --all-targets` to verify all Rust examples.
- Run one: `cargo run --bin ch01-newton-sqrt`.

## Editions and evolving adaptations

The 1984 first and 1996 second editions use Scheme. The 2022 JavaScript adaptation and the actively developed 2026 Python adaptation (SICPy) use different host languages. This companion tracks their shared ideas while preserving language-specific distinctions. Neither adaptation is a numbered successor to the Scheme editions.

The textbook is by Harold Abelson and Gerald Jay Sussman, with Julie Sussman; the JavaScript adaptation is by Martin Henz and Tobias Wrigstad. Official sources: [MIT Press second edition](https://mitpress.mit.edu/9780262510875/structure-and-interpretation-of-computer-programs/), [MIT Press JavaScript edition](https://mitpress.mit.edu/9780262543231/structure-and-interpretation-of-computer-programs/), [edition comparison](https://sicp.sourceacademy.org/), and [SICPy project](https://sicp.sourceacademy.org/split_py/chapters/making-of.html).

## Learning path

Read chapter introductions, then numbered subsection `INTRO.md` notes, then code and its sibling `README.md`. Each code example includes executable assertions, comments about invariants, and standard-library-only Rust.

## Static hosting

The root `index.html` fetches `manifest.json`, Markdown files and Rust source using relative paths. Host the repository root with GitHub Pages (deploy from `main` / `/ (root)`) or any static web server. `file://` browsing cannot normally fetch local files because of browser restrictions. No bundler or CDN is needed.

## Authorship and scope

The Rust implementations and explanations are independently authored educational adaptations, **not** verbatim reproductions of the published textbook. Exact examples, exercises, and solution coverage across editions remain a long-term inventory task. Contributions should follow [docs/porting-guide.md](docs/porting-guide.md) and [coverage methodology](docs/coverage-methodology.md). Use `python3 tools/verify_catalog.py` to validate the file index and internal links.
