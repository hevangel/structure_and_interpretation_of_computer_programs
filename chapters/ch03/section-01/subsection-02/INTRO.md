# 3.1.2 Benefits of Assignment

## What this subsection teaches

Why accept mutable state at all? Local state encapsulates changing details inside a module—such as a pseudorandom generator `rand()` that returns the next sample without forcing every Monte Carlo caller (`cesaro_test`, `estimate_integral`) to thread and return `seed` explicitly across every helper.

Notice the trade-off: `reset(seed)` gives convenience and modularity when testing, while pure seed-threading makes concurrency trivial.

## Connecting the concept to Rust and Python

- **In Rust:** Wraps state inside a `Rand { state: u64 }` struct with `next(&mut self)` and `reset(&mut self, seed)` methods (deterministic LCG).
- **In Python:** Wraps the LCG state inside a closure with `nonlocal seed` or a small `@dataclass` PRNG object (`generate()` / `reset(seed)`).
- **Why compare both:** Both implementations in [`monte-carlo-pi`](../../../../examples/ch03/section-01/monte-carlo-pi/README.md) show how encapsulating PRNG state simplifies high-level statistical experiments (`sqrt(6 / gcd_coprime_prob) ≈ pi`)—and why deterministic seeding makes tests repeatable.

## Exercise

Run [`monte-carlo-pi`](../../../../examples/ch03/section-01/monte-carlo-pi/README.md) twice from the same `reset(seed)` and confirm identical `pi` estimates.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
