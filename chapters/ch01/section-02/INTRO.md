# 1.2 Procedures and the Processes They Generate · [Chapter 1](../README.md)

## The big idea

A procedure's text does not uniquely determine the **shape of the process** it generates when executed. Linear recursion (`n * fact(n - 1)`) builds a deferred chain of pending multiplications requiring `O(n)` stack space; an **iterative process** tracks running state (`(product, counter)`) in fixed memory (`O(1)` space); and naive tree recursion (`fib(n - 1) + fib(n - 2)`) recomputes overlapping subproblems `O(phi^n)` times.

By noticing algebraic invariants—like halving an exponent (`b^n = (b^2)^(n/2)`), maintaining `gcd(a, b) == gcd(b, a % b)`, or squaring Fibonacci transformations—we shrink time and space from exponential or linear down to logarithmic (`O(log n)`).

## What you should focus on

Focus on ****process shape (linear recursive, linear iterative, tree recursive), Big-O time vs space growth, loop invariants, fast exponentiation, Euclid's GCD, and probabilistic primality****. The core discipline is separating *what* a computational method guarantees (its contract and invariant) from *how* any one helper is coded.

## Connecting the concept to Rust and Python

- **In Rust:** Rust does **not** guarantee tail-call optimization across platforms, so a tail-recursive function can still overflow the call stack on large `n`. Writing iterative processes directly with `loop`/`while` guarantees `O(1)` stack space, while integer widths (`u64`/`u128`) prevent silent overflow in modular arithmetic (`expmod`).
- **In Python:** Python likewise does **not** eliminate tail calls and caps recursion depth (~1,000 frames via `sys.getrecursionlimit()`), though Python's arbitrary-precision `int` avoids fixed-width overflow. Writing explicit loops or memoized dynamic programming gives both constant stack depth and massive speedup on tree recurrences.
- **Key contrast:** Neither Rust nor Python gives you Scheme's guaranteed tail-call stack reuse for free—which makes recognizing iterative state invariants *explicitly* even more practical in both languages.

## Subsections

- [1.2.1 Linear Recursion and Iteration](subsection-01/INTRO.md)
- [1.2.2 Tree Recursion](subsection-02/INTRO.md)
- [1.2.3 Orders of Growth](subsection-03/INTRO.md)
- [1.2.4 Exponentiation](subsection-04/INTRO.md)
- [1.2.5 Greatest Common Divisors](subsection-05/INTRO.md)
- [1.2.6 Testing for Primality](subsection-06/INTRO.md)

## Runnable examples in this section

- [1.2 Recursive and iterative factorial](../../../examples/ch01/section-02/factorial-processes/README.md)
- [1.2 Sine by recursive argument reduction](../../../examples/ch01/section-02/sine-triple-angle/README.md)
- [1.2 Fermat primality and Carmichael counterexample](../../../examples/ch01/section-02/fermat-test/README.md)
- [1.2 Multiplication by doubling and halving](../../../examples/ch01/section-02/fast-multiplication/README.md)
- [1.2 Counting coin-change combinations](../../../examples/ch01/section-02/counting-change/README.md)
- [1.2 Pascal triangle as a recursive process](../../../examples/ch01/section-02/pascal-triangle/README.md)
- [1.2 Exponentiation by repeated squaring](../../../examples/ch01/section-02/fast-exponentiation/README.md)
- [1.2 Euclid's greatest-common-divisor algorithm](../../../examples/ch01/section-02/euclidean-gcd/README.md)
- [1.2 Fibonacci recurrence and process shape](../../../examples/ch01/section-02/fibonacci-processes/README.md)
- [1.2 Trial-division primality test](../../../examples/ch01/section-02/prime-testing/README.md)
- [1.2 Fermat primality, Carmichael numbers, and Miller–Rabin](../../../examples/ch01/section-02/fermat-miller-rabin/README.md)

## Check your understanding

1. Why can two functions compute the exact same mathematical factorial while one uses `O(n)` stack frames and the other uses `O(1)`?
2. Why does Fermat's test fail on Carmichael numbers (`561`, `1105`, `1729`) while Miller–Rabin catches nontrivial square roots of `1 mod n`?

**Experiment:** Instrument `counting-change` and `fibonacci-processes` to count recursive calls vs DP/loop steps as `n` doubles.

[Chapter 1 overview](../README.md) · [Edition notes](../../../docs/editions.md)
