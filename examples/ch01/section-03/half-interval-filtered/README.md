# Filtered accumulation, half-interval roots, and continued fractions

**Book concepts:** Section 1.3 (Subsections 1.3.1 and 1.3.3). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Higher-order abstractions parameterize both combination rules (`filtered_accumulate`) and numerical approximation strategies (`half_interval_method`, damped `fixed_point_damped`, and finite continued fractions for Euler's `e` and Lambert's `tan x`).

## Implementation walkthrough

`filtered_accumulate` folds over `[a, b]` while guarding terms with a predicate. `half_interval_method` enforces opposite endpoint signs and bisects until interval width falls below tolerance. `cont_frac` folds bottom-up from term `k` down to `1`, computing Euler's `e - 2` expansion and Lambert's continued fraction for `tan(x)`.

## Complexity and correctness

Bisection halves the uncertainty bracket each step (O(log((b - a) / tol))). Bottom-up continued-fraction evaluation runs in O(k) time and O(1) auxiliary space without recursion stack growth.

## Run it

```sh
cargo run --bin ch01-half-interval-filtered
cargo test --bin ch01-half-interval-filtered
python3 examples/ch01/section-03/half-interval-filtered/main.py
python3 -m unittest examples/ch01/section-03/half-interval-filtered/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Compare the convergence speed of `cont_frac` for the golden ratio `1/phi` (`n_i = d_i = 1`) as `k` grows from 5 to 20.

[Section introduction](../../../../chapters/ch01/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
