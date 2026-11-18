# Symbolic polynomial algebra, pseudo-remainder GCD, and rational functions

**Book concepts:** Section 2.5 (Subsection 2.5.3). **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Symbolic algebra builds a complete polynomial ring (`add`, `sub`, `mul`, `div_rem`) and uses integer-preserving pseudo-division (`pseudoremainder`) with primitive-part normalization (`content_gcd`) to compute polynomial GCDs and reduce rational functions `(num, den)` to lowest terms.

## Implementation walkthrough

`Poly` stores sparse non-zero `(degree -> coefficient)` maps over a named variable `var`. `div_rem` performs long division returning `(quotient, remainder)`. Because exact division over integers can encounter non-divisible leading coefficients, `pseudoremainder` scales the dividend by `leading_coeff(divisor)^(1 + deg1 - deg2)` prior to division, ensuring Euclid's algorithm stays inside integer arithmetic, and `reduce_poly_fraction` divides numerator and denominator by their GCD.

## Complexity and correctness

Sparse maps omit zero terms so addition takes O(|P| + |Q|) and multiplication takes O(|P| * |Q|). Content division prevents exponential coefficient growth across successive pseudo-remainder steps.

## Run it

```sh
cargo run --bin ch02-symbolic-polynomials
cargo test --bin ch02-symbolic-polynomials
python3 examples/ch02/section-05/symbolic-polynomials/main.py
python3 -m unittest examples/ch02/section-05/symbolic-polynomials/main.py
```

The implementation has self-contained assertions and uses only the Rust and Python standard libraries. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Support nested polynomials whose coefficients are themselves polynomials in another indeterminate (ordered lexicographically by variable name).

[Section introduction](../../../../chapters/ch02/section-05/INTRO.md) · [Coverage](../../../../COVERAGE.md)
