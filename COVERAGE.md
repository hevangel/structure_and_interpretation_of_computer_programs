# Coverage ledger

This is an original Rust and Python learning companion, not a reproduction of the book's text or source. **95 independently written example programs (in both Rust `main.rs` and Python `main.py`) and 95 subsection study guides (`1.1.1`–`5.5.7` plus Source Academy's `3.2.5`) are implemented, though this is not yet exhaustive coverage of every exercise across all editions.**

| Chapter | Major sections introduced | Rust & Python examples | Complete exercise inventory |
| --- | ---: | ---: | --- |
| 1. Building abstractions with procedures | 3 | 22 | No |
| 2. Building abstractions with data | 5 | 27 | No |
| 3. Modularity, objects, and state | 5 | 19 | No |
| 4. Metalinguistic abstraction | 4 | 15 | No |
| 5. Computing with register machines | 5 | 12 | No |
| **Total** | **22** | **95** | **Pending** |

The 1984 original, 1996 Scheme second edition, 2022 JavaScript adaptation, and the developing 2026 SICPy/Python adaptation all differ. Their exact example inventories are not interchangeable. See [coverage methodology](docs/coverage-methodology.md). Example entries are tagged `shared`, `1996-scheme`, or `2022-javascript` in [manifest.json](manifest.json).

## Next steps for exhaustive coverage

1. Make an edition-specific inventory of every published code listing and example, distinguishing subexpressions, sample interactions, full systems, and exercises. Provide source section labels and links.
2. Add all missing independently written Rust analogues to their chapter/section directories; never silently treat one representative program as all examples in a section.
3. Add expected behavior and regression tests for each port, including parsing, stateful evaluation, and interpreters.
4. Resolve the formatting and lint audits and ensure CI is passing.
5. Verify the subsection guides against each edition and expand edition-specific notes for historical additions, removals, and changed language semantics.

References: [Edition crosswalk](docs/editions.md) · [Porting guide](docs/porting-guide.md) · [MIT Press second edition](https://mitpress.mit.edu/9780262510875/structure-and-interpretation-of-computer-programs/) · [MIT Press JavaScript edition](https://mitpress.mit.edu/9780262543231/structure-and-interpretation-of-computer-programs/)
