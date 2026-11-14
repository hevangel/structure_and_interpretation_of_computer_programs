# Editions and interpretation

This repository is an **independently written Rust companion**, not a translation or redistribution of the textbook's copyrighted prose or source listings. Section labels are navigational aids. Individual Rust programs reimplement familiar ideas with different syntax, tests, and engineering explanations.

| Edition | Source language | Relevant emphasis | Source |
| --- | --- | --- | --- |
| 1984 original (first edition) | Scheme / Lisp | Processes, abstractions, evaluators, machines | [Original and comparison overview](https://sicp.sourceacademy.org/) |
| 1996 second edition | Scheme | Revised interpreters and compilers; stronger emphasis on state and time, concurrency, streams, picture language | [MIT Press](https://mitpress.mit.edu/9780262510875/structure-and-interpretation-of-computer-programs/) |
| 2022 JavaScript adaptation | JavaScript | Functions, statements, parsing and explicit return-stack behavior | [MIT Press](https://mitpress.mit.edu/9780262543231/structure-and-interpretation-of-computer-programs/) |
| 2026 SICPy adaptation (in active development) | Python | Reframes language constructs in Python and Python § sublanguages, with Scheme/Python comparison | [Source Academy SICPy project](https://sicp.sourceacademy.org/split_py/chapters/making-of.html) |

The JavaScript and Python adaptations are **not** numbered third/fourth Scheme editions. Martin Henz started SICPy in 2026; its online text is evolving. The chapter structure is related, but parser, evaluator, and compiler examples need language-specific treatment. Existing example tags use `shared`, `1996-scheme`, and `2022-javascript`; new Python-specific work should use `2026-python`. A `shared` tag means an educational concept occurs in multiple versions, **not** that the exact snippet has been ported independently for every edition.

## Coverage policy

- An example is listed as **implemented** only if its Rust source and adjacent README are checked in.
- Coverage of a *section* does not imply coverage of all its exercises, diagrams, snippets, alternative solutions, or edition-specific changes.
- New examples should record an edition or `shared` applicability and any semantic difference from Scheme/JavaScript/Python.
- There is currently no claim that all published examples from all four editions/adaptations have been ported. See [coverage tracker](../COVERAGE.md).

## Editions crosswalk

The 1996 second edition and newer language adaptations organize their central ideas into five chapters. Terminology changes include “procedures” versus “functions,” and chapter 4/5 language processors are substantially adapted for JavaScript. The first edition also differs in ordering and exact example inventory; do not equate section number with identical source across editions. See the [comparison edition](https://sicp.sourceacademy.org/index.html) when validating a specific correspondence.

## Source-of-truth inventories

The [Source Academy SICP repository](https://github.com/source-academy/sicp) maintains separate original Scheme, JavaScript and Python source forms. Its `programs_scm`, `programs_js` and `programs_py` directories are **generated**, not tracked program copies in the Git tree. The repository's generator exports machine-readable per-section JSON and runnable snippet archives. For an exhaustive audit, **generate an inventory of snippet identifiers from each edition's structured source**, then cross-reference each with a separately authored Rust example. Do not assume that an empty directory means there are no examples.

The original 1984 edition requires its own historical reconciliation with 1996; the generated Scheme comparison represents the second edition's source, not a complete first-edition listing. See [Coverage methodology](coverage-methodology.md).
