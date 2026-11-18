from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Callable
import unittest


@dataclass
class Frame:
    bindings: dict[str, Any]
    parent: Frame | None = None

    def lookup(self, name: str) -> Any:
        cur: Frame | None = self
        while cur is not None:
            if name in cur.bindings:
                return cur.bindings[name]
            cur = cur.parent
        raise KeyError(f"unbound: {name}")

    def set_var(self, name: str, val: Any) -> None:
        cur: Frame | None = self
        while cur is not None:
            if name in cur.bindings:
                cur.bindings[name] = val
                return
            cur = cur.parent
        raise KeyError(f"unbound: {name}")


@dataclass
class CompiledProc:
    params: list[str]
    body_fn: Callable[[Frame], Any]
    env: Frame


def analyze(expr: Any, counters: dict[str, int]) -> Callable[[Frame], Any]:
    counters["analyze"] += 1
    if isinstance(expr, (int, bool)):
        return lambda _env: _bump(counters, expr)
    if isinstance(expr, str):
        return lambda env, n=expr: _bump(counters, env.lookup(n))
    head = expr[0]
    if head == "if":
        p_fn = analyze(expr[1], counters)
        c_fn = analyze(expr[2], counters)
        a_fn = analyze(expr[3], counters)
        return lambda env: (_bump(counters, None), c_fn(env) if p_fn(env) is not False else a_fn(env))[1]
    if head == "lambda":
        params = expr[1]
        b_fn = analyze_seq(expr[2:], counters)
        return lambda env: (_bump(counters, None), CompiledProc(params, b_fn, env))[1]
    if head == "define":
        name, v_fn = expr[1], analyze(expr[2], counters)

        def do_def(env: Frame) -> Any:
            counters["exec"] += 1
            val = v_fn(env)
            env.bindings[name] = val
            return val

        return do_def
    if head == "let":
        params = [b[0] for b in expr[1]]
        args = [b[1] for b in expr[1]]
        return analyze([["lambda", params, *expr[2:]], *args], counters)
    if head == "begin":
        return analyze_seq(expr[1:], counters)
    f_proc = analyze(head, counters)
    f_args = [analyze(a, counters) for a in expr[1:]]

    def do_call(env: Frame) -> Any:
        counters["exec"] += 1
        proc = f_proc(env)
        args = [af(env) for af in f_args]
        if isinstance(proc, CompiledProc):
            return proc.body_fn(Frame(dict(zip(proc.params, args)), proc.env))
        return proc(*args)

    return do_call


def analyze_seq(exprs: list[Any], counters: dict[str, int]) -> Callable[[Frame], Any]:
    fns = [analyze(e, counters) for e in exprs]

    def run(env: Frame) -> Any:
        res = None
        for f in fns:
            res = f(env)
        return res

    return run


def _bump(counters: dict[str, int], val: Any) -> Any:
    counters["exec"] += 1
    return val


def global_env() -> Frame:
    return Frame({"*": lambda a, b: a * b, "-": lambda a, b: a - b, "=": lambda a, b: a == b})


class AnalyzingEvaluatorTest(unittest.TestCase):
    def test_zero_reanalysis_during_execution(self) -> None:
        counters = {"analyze": 0, "exec": 0}
        prog = [
            "begin",
            ["define", "fact", ["lambda", ["n"], ["if", ["=", "n", 0], 1, ["*", "n", ["fact", ["-", "n", 1]]]]]],
            ["let", [["k", 10]], ["fact", "k"]],
        ]
        compiled = analyze(prog, counters)
        frozen_analyze = counters["analyze"]
        self.assertEqual(compiled(global_env()), 3_628_800)
        self.assertEqual(counters["analyze"], frozen_analyze)
        self.assertGreater(counters["exec"], frozen_analyze)


if __name__ == "__main__":
    c = {"analyze": 0, "exec": 0}
    fn = analyze(
        ["begin", ["define", "fact", ["lambda", ["n"], ["if", ["=", "n", 0], 1, ["*", "n", ["fact", ["-", "n", 1]]]]]], ["fact", 10]],
        c,
    )
    a_count = c["analyze"]
    print("fact(10) =", fn(global_env()), f"(analyze={a_count}, exec={c['exec']})")
