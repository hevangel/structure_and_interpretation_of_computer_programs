# 4.4 Logic Programming · [Chapter 4](../README.md)

## The big idea

A **logic query language** pushes declarative programming even further: relations like `(append_to_form ?x ?y (a b c))` work **bidirectionally**—finding the concatenated list, solving for missing prefixes/suffixes, or enumerating every `(x, y)` split!

Its engine builds streams of variable-binding **frames** via two-sided **unification** (pairing pattern variables on both sides plus an **occurs check** rejecting cyclic self-bindings like `?x = (f ?x)`), **alpha-renaming** rule variables per invocation (`rule_counter`) so recursive rules never collide, and `and`/`or`/`not`/`unique` combinators.

## What you should focus on

Focus on ****pattern matching vs two-sided unification, occurs check, rule variable renaming, `and`/`or`/`not` frame streams, and closed-world negation pitfalls****. Every variation keeps the same guest syntax while systematically shifting *when* and *how* expressions reduce.

## Connecting the concept to Rust and Python

- **In Rust:** Pattern terms (`enum Term`), binding frames (`BTreeMap<Var, Term>`), unification with `depends_on` cycle checks, and stream combinators combine with `RuleCounter` renaming (`query-unification-engine`).
- **In Python:** Immutable `@dataclass` terms (`Atom`, `Var`, `Pair`), persistent dict frame copies `{**frame, var: val}`, and generator/list frame pipelines express unification and rule chaining cleanly.
- **Key contrast:** Both implementations in `query-unification-engine` show why logical `(and A B)` behaves differently under `(not A)` (closed-world filter!) or infinite rule recursion (`right-to-left` vs `left-to-right` conjunct ordering).

## Subsections

- [4.4.1 Deductive Information Retrieval](subsection-01/INTRO.md)
- [4.4.2 How Queries Work](subsection-02/INTRO.md)
- [4.4.3 Logic and Operational Search](subsection-03/INTRO.md)
- [4.4.4 Implementing Queries](subsection-04/INTRO.md)

## Runnable examples in this section

- [4.4 Relational queries with variable bindings](../../../examples/ch04/section-04/logic-relations/README.md)
- [4.4 Recursive ancestor relation by graph traversal](../../../examples/ch04/section-04/ancestor-relation/README.md)
- [4.4 Logic query engine with rules, variable-renaming unification, and frame streams](../../../examples/ch04/section-04/query-unification-engine/README.md)

## Check your understanding

1. Why can unification succeed when matching `(?x 2)` against `(1 ?y)`, and why must `?x = (cons 1 ?x)` fail the occurs check?
2. Why is `(and (job ?p (computer programmer)) (not (supervisor ?p ?s)))` safe, whereas putting `(not ...)` *before* binding `?p` gives an empty stream?

**Experiment:** Trace bidirectional `(append_to_form ?x ?y (1 2 3))` in `query-unification-engine` to inspect all four returned substitutions.

[Chapter 4 overview](../README.md) · [Edition notes](../../../docs/editions.md)
