# Chapter 4: Metalinguistic Abstraction

The deepest way to solve a whole class of problems is to design a new language tailored to it—and write an evaluator that gives expressions their meaning.

Starting from a metacircular core (`eval`/`apply` and environment frames), Chapter 4 separates compile-time syntactic analysis from execution, swaps eager evaluation for **normal-order lazy thunks**, introduces continuation-passing **nondeterministic `amb` search**, and builds a **unification-based logic query engine**.

## Rust and Python perspectives

- **In Rust:** Because Rust lacks runtime `eval`, its interpreters clearly separate **host language** from **guest language**: guest ASTs, values, thunks, environments (`Rc<RefCell<Frame>>`), and continuations are ordinary Rust `enum`s and closures.
- **In Python:** Python's dynamic typing, dictionaries, generators, and higher-order closures let guest evaluators mirror classic Lisp/Scheme evaluator structure almost line for line while still isolating guest environments from Python's own namespace.
- **Comparing both:** In both Rust and Python, you never confuse *meta-level* host variables with *object-level* interpreted expressions—making environment lookup, thunk memoization, and backtracking undo crystal clear.

## Sections in this chapter

- [4.1 The Metacircular Evaluator](section-01/INTRO.md)
- [4.2 Variations on a Scheme: Lazy Evaluation](section-02/INTRO.md)
- [4.3 Variations on a Scheme: Nondeterministic Computing](section-03/INTRO.md)
- [4.4 Logic Programming](section-04/INTRO.md)

## How to study

1. Read each section and subsection note before inspecting the source files.
2. Predict the output, invariants, and time/space complexity on paper.
3. Compare the Rust (`main.rs`) and Python (`main.py`) implementations in the viewer (`index.html` Side-by-Side view) and run both test suites.
4. Change one representation detail or invariant and observe which tests break.

The commentary here is independently written educational material, not a reproduction of the textbook. · [Edition notes](../../docs/editions.md) · [Coverage ledger](../../COVERAGE.md)
