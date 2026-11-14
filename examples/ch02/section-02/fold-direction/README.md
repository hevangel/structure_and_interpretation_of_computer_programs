# Left and right folds are not interchangeable

**Section:** 2.2 · **Edition scope:** shared

## Big idea

Two folds over the same sequence may produce different results when the combining operation is nonassociative.

## Rust design and invariants

Left fold combines an accumulator with each element in order; right fold builds from the rightmost element, passing the accumulated suffix as the second argument.

## Complexity

Both use O(n) time and constant extra space with slice iterators. They need not be equivalent if operations are not associative.

## Run

```sh
cargo run --bin ch02-fold-direction
cargo test --bin ch02-fold-direction
```

## Exercise

Use string concatenation to visualize exactly where parentheses would be placed.

[Undergraduate section guide](../../../../chapters/ch02/section-02/INTRO.md). This code and explanation are original pedagogical work, not a reproduction of the textbook's code.
