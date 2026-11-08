# Editions and interpretation

This repository is an **independently written Rust companion**, not a translation or redistribution of the textbook's copyrighted prose or source listings. Section labels are navigational aids. Individual Rust programs reimplement familiar ideas with different syntax, tests, and engineering explanations.

| Edition | Source language | Relevant emphasis | Source |
| --- | --- | --- | --- |
| 1984 original (first edition) | Scheme / Lisp | Processes, abstractions, evaluators, machines | [Original and comparison overview](https://sicp.sourceacademy.org/) |
| 1996 second edition | Scheme | Revised interpreters and compilers; stronger emphasis on state and time, concurrency, streams, picture language | [MIT Press](https://mitpress.mit.edu/9780262510875/structure-and-interpretation-of-computer-programs/) |
| 2022 JavaScript edition | JavaScript | Functions and statement-oriented execution; parsing and return-stack discipline in language processors | [MIT Press](https://mitpress.mit.edu/9780262543231/structure-and-interpretation-of-computer-programs/) |

The JavaScript edition is an *adaptation*, not a numbered third Scheme edition. Its chapters share much conceptual structure, but language-specific examples need distinct treatment. The 2022 chapter 4 parsing and chapter 5 compilation examples are therefore marked `javascript-2022` in the example metadata.

## Coverage policy

- An example is listed as **implemented** only if its Rust source and adjacent README are checked in.
- Coverage of a *section* does not imply coverage of all its exercises, diagrams, snippets, alternative solutions, or edition-specific changes.
- New examples should record an edition or `shared` applicability and any semantic difference from Scheme/JavaScript.
- There is currently no claim that all published examples from all three editions have been ported. See [coverage tracker](../COVERAGE.md).

## Editions crosswalk

The 1996 and 2022 editions organize their central ideas into five chapters. Terminology changes include “procedures” versus “functions,” and chapter 4/5 language processors are substantially adapted for JavaScript. The first edition also differs in ordering and exact example inventory; do not equate section number with identical source across editions. See the [comparison edition](https://sicp.sourceacademy.org/index.html) when validating a specific correspondence.
