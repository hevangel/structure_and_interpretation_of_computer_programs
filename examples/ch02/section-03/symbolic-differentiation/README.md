# Symbolic differentiation

**Book concepts:** Section 2.3. **Edition applicability:** shared conceptual material across editions.

## What this demonstrates

Differentiate an expression tree without first evaluating it; this is computation over syntax.

## Implementation walkthrough

Recursive pattern matching directly implements constant, variable, sum, and product rules. Constructors simplify zero and one to keep the result readable.

## Complexity and correctness

Runtime and output size depend on the expression tree; naive product expansion can grow expressions rapidly.

## Run it

```sh
cargo run --bin ch02-symbolic-differentiation
cargo test --bin ch02-symbolic-differentiation
```

The implementation has self-contained assertions and uses only the Rust standard library. Compare the results and internal representation with the original edition; it is an independently written teaching example, not a line-for-line translation.

## Try this

Add a power node and compare numeric differentiation against your symbolic result.

[Section introduction](../../../..//chapters/ch02/section-03/INTRO.md) · [Coverage](../../../../COVERAGE.md)
