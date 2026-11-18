# Logic query engine with rules, variable-renaming unification, and frame streams

**Book concepts:** Section 4.4 (Subsections 4.4.1–4.4.4.8). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

A complete logic programming engine featuring nested term patterns, two-sided unification with an explicit `depends_on` cyclic occurs check, monotonic rule variable alpha-renaming (`rule_counter`), and stream query combinators (`and`, `or`, `not`, `unique`, and predicate filters). Recursive rules compute transitive `outranked-by` chains and bidirectional `append-to-form` list splits.

## Implementation walkthrough

Pattern matching extends frames unilaterally against ground assertions, whereas `unify_match` constrains variables symmetrically across rule conclusions and pattern queries. Whenever a rule applies, all pattern variables inside the conclusion and body are stamped with a fresh `rule_id` so recursive rule applications never collide.

## Complexity and correctness

Because `depends_on` inspects transitive variable bindings before extending a frame, circular equations such as `?x = (f ?x)` fail cleanly without diverging.

## Run it

```sh
cargo run --bin ch04-query-unification-engine
cargo test --bin ch04-query-unification-engine
python3 examples/ch04/section-04/query-unification-engine/main.py
python3 -m unittest examples/ch04/section-04/query-unification-engine/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add an index keyed by assertion functor (`list` head symbol) to `Database` so queries skip unrelated facts.

[Section introduction](../../../../chapters/ch04/section-04/INTRO.md) · [Coverage](../../../../COVERAGE.md)
