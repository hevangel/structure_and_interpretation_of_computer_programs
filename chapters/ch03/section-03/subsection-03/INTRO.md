# 3.3.3 Representing Tables

## What this subsection teaches

A **table** maps keys (1D, 2D `(row, col)`, or arbitrary-depth key paths `&[K]`) to stored values and allows incremental `insert`/`lookup`.

Tables turn exponential recursive algorithms (like naive `fib(40)` or longest common subsequence) into linear/polynomial **dynamic programming** via **`memoize`**: wrap the function with a private local lookup table that caches computed `(args -> result)` pairs automatically.

## Connecting the concept to Rust and Python

- **In Rust:** Nested `BTreeMap`s or an explicit generic trie (`Table<K, V>`) store hierarchical keys with deterministic iteration order, while `memoize` wraps closures over `Rc<RefCell<HashMap<...>>>` or explicit caches.
- **In Python:** Nested Python `dict`s (`table.setdefault(k1, {})[k2] = val`), tuple keys (`(k1, k2)`), and `functools.lru_cache` express multi-key tables and memoization in a few lines.
- **Why compare both:** Explore [`memoized-fibonacci`](../../../../examples/ch03/section-03/memoized-fibonacci/README.md) and [`local-tables-memoize`](../../../../examples/ch03/section-03/local-tables-memoize/README.md) side by side—same `O(n)` call reduction invariant in both languages.

## Exercise

Build a 2D table with custom approximate key equality (for `float` tolerance) and wrap recursive `fib` to prove call count drops from exponential to linear.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
