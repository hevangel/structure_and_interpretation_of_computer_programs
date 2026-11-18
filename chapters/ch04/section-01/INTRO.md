# 4.1 The Metacircular Evaluator · [Chapter 4](../README.md)

## The big idea

An **evaluator** defines the meaning of every language construct via a mutual recursion between **`eval(exp, env)`** (dispatch on syntax: variables, conditionals, lambdas, definitions, derived forms like `cond`/`let`) and **`apply(proc, args)`** (extend a closure's saved environment frame with evaluated argument values and execute its body).

Separating **syntactic analysis (`analyze(exp)`)** from **execution (`exec_fn(env)`)** parses every loop body and derived form *once* up front rather than re-walking AST syntax on every iteration.

## What you should focus on

Focus on ****`eval`/`apply` mutual recursion, AST representation (`quote`, `cond`, `let*`), internal definition scanning (`scan_out_defines`), JavaScript blocks/`return` propagation, and separating analysis from execution****. Every variation keeps the same guest syntax while systematically shifting *when* and *how* expressions reduce.

## Connecting the concept to Rust and Python

- **In Rust:** AST nodes (`enum Expr`) and runtime values (`enum Value`) are distinct compile-time types checked exhaustively with `match`; environments use `Rc<RefCell<Frame>>` and `analyze` compiles ASTs into `Rc<dyn Fn(&Env) -> Result<Value, ...>>` closures.
- **In Python:** AST nodes and values are concise `@dataclass` unions evaluated against `Frame` objects (`dict[str, Value]` + parent pointer); `analyze` returns plain Python closures `Callable[[Frame], Value]`.
- **Key contrast:** Even in Python (which has dynamic typing), keeping guest `Value`/`Frame` objects isolated from Python's host `eval` clarifies the boundary between syntax, derived transformations, and runtime frames—explore `metacircular-evaluator`, `analyzing-evaluator`, and `javascript-parser`.

## Subsections

- [4.1.1 Core of the Evaluator](subsection-01/INTRO.md)
- [4.1.2 Representing Expressions](subsection-02/INTRO.md)
- [4.1.3 Evaluator Data Structures](subsection-03/INTRO.md)
- [4.1.4 Running the Evaluator](subsection-04/INTRO.md)
- [4.1.5 Data as Programs](subsection-05/INTRO.md)
- [4.1.6 Internal Declarations](subsection-06/INTRO.md)
- [4.1.7 Analysis versus Execution](subsection-07/INTRO.md)

## Runnable examples in this section

- [4.1 Expression evaluator with lexical bindings](../../../examples/ch04/section-01/tiny-evaluator/README.md)
- [4.1 Evaluator with lexical closures](../../../examples/ch04/section-01/lexical-closures/README.md)
- [4.1 Expression parsing with operator precedence](../../../examples/ch04/section-01/javascript-parser/README.md)
- [4.1 Separate syntactic analysis from execution](../../../examples/ch04/section-01/analysis-then-execution/README.md)
- [4.1 Analyzing evaluator (separating syntax analysis from execution)](../../../examples/ch04/section-01/analyzing-evaluator/README.md)
- [4.1 Full metacircular evaluator (quote, cond, define, set!, let*, scan_out_defines)](../../../examples/ch04/section-01/metacircular-evaluator/README.md)

## Check your understanding

1. Why must `apply` extend the **closure's captured environment** instead of the caller's environment?
2. Why does separating `analyze` from execution avoid re-evaluating macro/derived expansions (`cond -> nested if`, `let -> lambda call`) inside a tight loop?

**Experiment:** Compare analysis vs execution step counts on `fact(10)` in `analyzing-evaluator`, and test `return` short-circuiting out of nested blocks in `javascript-parser`.

[Chapter 4 overview](../README.md) · [Edition notes](../../../docs/editions.md)
