# 3.5.4 Streams and Delayed Evaluation

## What this subsection teaches

What if a stream depends on *itself* through an integral—such as solving the differential equation `dy/dt = f(y)` via `y = integral(dy, y0, dt)` where `dy = stream_map(f, y)`?

If `integral` evaluates its `integrand` argument eagerly on construction, constructing `y` tries to inspect `dy` before `y` even exists! Passing `delayed_integrand` as a **lazy thunk** breaks the circular initialization cleanly.

## Connecting the concept to Rust and Python

- **In Rust:** Passing `Rc<dyn Fn() -> MemoStream<f64>>` (or lazy thunk field) inside `integral_delayed` breaks the self-reference cycle in [`stream-integration`](../../../../examples/ch03/section-05/stream-integration/README.md) and [`stream-acceleration-pairs`](../../../../examples/ch03/section-05/stream-acceleration-pairs/README.md).
- **In Python:** Passing `integrand_thunk: Callable[[], MemoStream[float]]` defers computing `dy` until `y`'s first `dt` step is forced—breaking the initialization cycle identically.
- **Why compare both:** In both Rust and Python, `dy/dt = y` (`solve(lambda y: y, 1.0, 0.001)`) converges toward `e ≈ 2.71828` at `t = 1` once the feedback integrand is wrapped in a delay thunk.

## Exercise

Trace step 0 (`y0 = 1.0`) and step 1 (`y1 = 1.0 + dy0 * dt`) of delayed ODE `solve(y -> y, 1.0, 0.1)`.

[Parent section](../INTRO.md) · [Chapter 3 overview](../../README.md) · [Edition notes](../../../../docs/editions.md)
