# Coverage ledger

This is an original Rust learning companion, not a reproduction of the book's text or source. **65 independently written example programs and 95 numbered subsection study guides are implemented, but this is not exhaustive coverage of any edition's exact source snippets or exercises.**

| Chapter | Major sections introduced | Rust example programs | Complete example inventory |
| --- | ---: | ---: | --- |
| 1. Building abstractions with procedures | 3 | 18 | No |
| 2. Building abstractions with data | 5 | 21 | No |
| 3. Modularity, objects, and state | 5 | 14 | No |
| 4. Metalinguistic abstraction | 4 | 6 | No |
| 5. Computing with register machines | 5 | 6 | No |
| **Total** | **22** | **65** | **Pending** |

The 1984 original, 1996 Scheme second edition, 2022 JavaScript adaptation, and the developing 2026 SICPy/Python adaptation all differ. Their exact example inventories are not interchangeable. See [coverage methodology](docs/coverage-methodology.md). Rust analogues are tagged `shared`, `1996-scheme`, or `2022-javascript` in [manifest.json](manifest.json).

## Next steps for exhaustive coverage

1. Make an edition-specific inventory of every published code listing and example, distinguishing subexpressions, sample interactions, full systems, and exercises. Provide source section labels and links.
2. Add all missing independently written Rust analogues to their chapter/section directories; never silently treat one representative program as all examples in a section.
3. Add expected behavior and regression tests for each port, including parsing, stateful evaluation, and interpreters.
4. Resolve the formatting and lint audits and ensure CI is passing.
5. Verify the subsection guides against each edition and expand edition-specific notes for historical additions, removals, and changed language semantics.

References: [Edition crosswalk](docs/editions.md) · [Porting guide](docs/porting-guide.md) · [MIT Press second edition](https://mitpress.mit.edu/9780262510875/structure-and-interpretation-of-computer-programs/) · [MIT Press JavaScript edition](https://mitpress.mit.edu/9780262543231/structure-and-interpretation-of-computer-programs/)
