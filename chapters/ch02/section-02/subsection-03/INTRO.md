# 2.2.3 Sequences as Conventional Interfaces

## What this subsection teaches

Instead of writing custom nested loops for matrices, permutations, prime-sum pairs, and Horner polynomial evaluation, decompose problems into four standard signal-processing blocks:
**enumerate → filter → map → accumulate (`fold`)**.

Notice also that `fold_left` and `fold_right` produce the *same* answer only when the combining operation is commutative/associative; for `-` or `/`, fold direction changes the result completely!

## Connecting the concept to Rust and Python

- **In Rust:** Rust iterator combinators (`.flat_map()`, `.filter()`, `.map()`, `.fold()`, `.rfold()`) fuse into tight loops at compile time without allocating intermediate vectors.
- **In Python:** Python uses list/generator comprehensions (`[f(x) for x in xs if p(x)]`), `itertools.chain`, and `functools.reduce` (while `reversed(xs)` expresses right folds cleanly).
- **Why compare both:** Run [`matrix-operations`](../../../../examples/ch02/section-02/matrix-operations/README.md), [`powerset`](../../../../examples/ch02/section-02/powerset/README.md), [`fold-direction`](../../../../examples/ch02/section-02/fold-direction/README.md), and [`nested-mappings-mobiles`](../../../../examples/ch02/section-02/nested-mappings-mobiles/README.md) to see identical pipeline stages in both languages.

## Exercise

Refactor `prime_sum_pairs(n)` into `flatmap` over `(i, j)` pairs followed by `filter(is_prime(i + j))`.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
