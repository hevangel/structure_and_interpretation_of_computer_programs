# Coverage ledger

This is an original Rust learning companion, not a reproduction of the book's text or source. **37 independently written example programs are implemented, but this is not exhaustive coverage of any edition's exact source snippets or exercises.**

| Chapter | Major sections introduced | Rust example programs | Complete example inventory |
| --- | ---: | ---: | --- |
| 1. Building abstractions with procedures | 3 | 10 | No |
| 2. Building abstractions with data | 5 | 11 | No |
| 3. Modularity, objects, and state | 5 | 5 | No |
| 4. Metalinguistic abstraction | 4 | 5 | No |
| 5. Computing with register machines | 5 | 6 | No |
| **Total** | **22** | **37** | **Pending** |

The 1984 original, the 1996 Scheme edition, and the 2022 JavaScript adaptation all have differences in their example sets, including new/revised second-edition systems and the JavaScript edition's parsing and return-stack concepts. Rust analogues are tagged `shared`, `1996-scheme`, or `2022-javascript` in [manifest.json](manifest.json).

## Next steps for exhaustive coverage

1. Make an edition-specific inventory of every published code listing and example, distinguishing subexpressions, sample interactions, full systems, and exercises. Provide source section labels and links.
2. Add all missing independently written Rust analogues to their chapter/section directories; never silently treat one representative program as all examples in a section.
3. Add expected behavior and regression tests for each port, including parsing, stateful evaluation, and interpreters.
4. Resolve the formatting and lint audits and ensure CI is passing.
5. Map section introductions down to individual numbered subsections when the complete inventory has been established.

References: [Edition crosswalk](docs/editions.md) · [Porting guide](docs/porting-guide.md) · [MIT Press second edition](https://mitpress.mit.edu/9780262510875/structure-and-interpretation-of-computer-programs/) · [MIT Press JavaScript edition](https://mitpress.mit.edu/9780262543231/structure-and-interpretation-of-computer-programs/)
