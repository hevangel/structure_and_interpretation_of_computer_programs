# 4.1.2 Representing Expressions

## What this subsection teaches

An evaluator should not care whether syntax came from S-expressions `(+ 2 (* 3 4))` or JavaScript infix statements `return 2 + 3 * 4;`! Wrapping syntax checks and accessors (`is_if`, `if_predicate`, `lambda_body`, derived `cond -> if`) isolates parsing from evaluation.

In the 2022 JavaScript edition, a dedicated **`parse()`** front-end turns precedence operators, `const`/`let`/`function` declarations, and `{ ... }` statement blocks into explicit tagged AST nodes.

## Connecting the concept to Rust and Python

- **In Rust:** Tokenizes input into an explicit `Expr` / `Stmt` AST enum with Pratt/recursive-descent precedence parsing (`javascript-parser`) and explicit S-expression parsing (`metacircular-evaluator`).
- **In Python:** Mirrors the same recursive-descent tokenizer/parser with `@dataclass` AST nodes (`BinOp`, `ConstDecl`, `Block`, `Return`) and raises syntax errors on mismatched tokens.
- **Why compare both:** Both languages enforce explicit AST schemas so later evaluator stages never re-parse raw source strings (`javascript-parser`, `metacircular-evaluator`).

## Exercise

Trace the AST built for `const x = 2 + 3 * 4; return x;` and compare it against `(let ((x (+ 2 (* 3 4)))) x)`.

[Parent section](../INTRO.md) · [Chapter 4 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
