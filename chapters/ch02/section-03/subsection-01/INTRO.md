# 2.3.1 Quotation and Strings

## What this subsection teaches

Normally a name `x` evaluates to whatever value it holds. **Quotation** steps back one meta-level: `'x` (or string `"x"`) refers to the *symbol/name itself* so programs can compare variable names (`variable == var`), rewrite formulas, or parse source code.

## Connecting the concept to Rust and Python

- **In Rust:** Rust distinguishes identifiers bound in code from runtime symbol strings (`&str` / `String` / interned IDs)—`expr == Var("x")` checks symbolic identity explicitly via `PartialEq`.
- **In Python:** Python distinguishes bound variables from `"x"` string tokens or `Sym("x")` dataclass wrappers; unlike Scheme, Python has no built-in `'quote` syntax so symbolic ASTs represent names explicitly.
- **Why compare both:** Seeing how both Rust (`Expr::Var(&str)`) and Python (`Var(name)`) avoid accidental variable evaluation clarifies why metacircular evaluators distinguish symbols from variables.

## Exercise

Explain the difference between `x`, `"x"`, and `Var("x")` in an expression AST.

[Parent section](../INTRO.md) · [Chapter 2 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
