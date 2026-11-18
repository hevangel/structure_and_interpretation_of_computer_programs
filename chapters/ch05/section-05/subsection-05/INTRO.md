# 5.5.5 Example of Compiled Code

## What this subsection teaches

Inspecting the actual generated instructions for recursive `(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))` proves the compiler's payoff quantitatively:
- Compiling `(- n 1)` needs zero `env` saves.
- Compiling `n` inside `(* ... n)` only saves `env` + `continue` around the inner `(factorial (- n 1))` call.
- Result on `n = 6`: **`32` total pushes and `17` max depth** (`6n - 4` pushes, `3n - 1` depth) vs `eceval`'s **`176` pushes and `33` depth**—over **5× fewer stack operations!**

## Connecting the concept to Rust and Python

- **In Rust:** Both `constant-folding-compiler` (AST algebraic folding before bytecode emission) and `register-compiler` (exact push/depth verification on `factorial(6)`) test generated opcodes end-to-end.
- **In Python:** Python `register-compiler/main.py` and `constant-folding-compiler/main.py` reproduce the **exact same instruction sequences, folded lengths (`5 -> 3` opcodes), and `(32, 17)` push/depth counts**.
- **Why compare both:** Seeing identical `(32, 17)` counts in Rust and Python confirms the savings come entirely from static `needs`/`modifies` optimization on compiled register code.

## Exercise

Run [`register-compiler`](../../../../examples/ch05/section-05/register-compiler/README.md) and [`constant-folding-compiler`](../../../../examples/ch05/section-05/constant-folding-compiler/README.md) to compare unoptimized vs optimized code metrics.

[Parent section](../INTRO.md) · [Chapter 5 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
