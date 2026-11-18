# 5.5.2 Compiling Expressions

## What this subsection teaches

Compiling simple expressions shows target/linkage composition in action:
- **Constants / Variables:** emit `assign(target, const/lookup(exp, env))` plus `end_with_linkage(linkage)` (`needs: {env}`, `modifies: {target}`).
- **Conditionals `(if p c a)`:** evaluate `p -> val` (linkage `Next`), allocate `true_branch`, `false_branch`, `after_if`; compile `c` with `after_if` linkage (or `Return` if the whole `if` has `Return`!) and `a` with `after_if`, wrapping `p` + branches with `preserving([env, continue], ...)`!
- **Lambdas:** emit body instructions out-of-line under a unique entry label (`entry_k`), and inline emit `make_compiled_procedure(entry_k, env) -> target`!

## Connecting the concept to Rust and Python

- **In Rust:** Emits typed `Instr` items into a persistent VM code buffer with unique generated labels (`t_branch_0`, `after_if_1`) (`register-compiler`).
- **In Python:** Generates the exact same branch/lambda label structure and appends compiled procedure blocks into the shared persistent VM instruction list.
- **Why compare both:** Because `define (factorial n) ...` compiles into persistent `code` memory, calling `eval_compiled("(factorial 6)")` jumps straight into `entry_k` cleanly across separate compilations in both Rust and Python.

## Exercise

Trace the compiled instructions for `(if (= n 0) 1 (* n 2))` with `target = Val`, `linkage = Return`.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
