from __future__ import annotations

from dataclasses import dataclass
from typing import Any
import unittest


def tokenize(src: str) -> list[str]:
    tokens: list[str] = []
    i = 0
    while i < len(src):
        c = src[i]
        if c.isspace():
            i += 1
        elif c == ";":
            while i < len(src) and src[i] != "\n":
                i += 1
        elif c in "()'":
            tokens.append(c)
            i += 1
        else:
            j = i
            while j < len(src) and not src[j].isspace() and src[j] not in "()":
                j += 1
            tokens.append(src[i:j])
            i = j
    return tokens


def parse_program(src: str) -> list[Any]:
    tokens = tokenize(src)
    pos = 0

    def parse_one() -> Any:
        nonlocal pos
        tok = tokens[pos]
        pos += 1
        if tok == "(":
            items = []
            while tokens[pos] != ")":
                items.append(parse_one())
            pos += 1
            return items
        if tok == "'":
            return ["quote", parse_one()]
        if tok in ("#t", "true"):
            return True
        if tok in ("#f", "false"):
            return False
        try:
            return int(tok)
        except ValueError:
            return tok

    out = []
    while pos < len(tokens):
        out.append(parse_one())
    return out


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
        raise KeyError(f"unbound variable: {name}")

    def set_var(self, name: str, val: Any) -> None:
        cur: Frame | None = self
        while cur is not None:
            if name in cur.bindings:
                cur.bindings[name] = val
                return
            cur = cur.parent
        raise KeyError(f"unbound variable for set!: {name}")


@dataclass
class Thunk:
    expr: Any
    env: Frame | None
    evaluated: bool = False
    value: Any = None


@dataclass
class Closure:
    params: list[str]
    body: list[Any]
    env: Frame


@dataclass(frozen=True)
class Primitive:
    name: str


class LazyMachine:
    def __init__(self) -> None:
        self.global_env = Frame(
            {op: Primitive(op) for op in ("+", "-", "*", "/", "=", "<", ">")}
        )
        self.thunks_created = 0
        self.thunks_forced = 0
        self.memoized_hits = 0

    def delay_it(self, expr: Any, env: Frame) -> Thunk:
        self.thunks_created += 1
        return Thunk(expr=expr, env=env)

    def force_it(self, val: Any) -> Any:
        if isinstance(val, Thunk):
            if val.evaluated:
                self.memoized_hits += 1
                return val.value
            assert val.env is not None
            self.thunks_forced += 1
            result = self.actual_value(val.expr, val.env)
            val.evaluated = True
            val.value = result
            val.env = None
            return result
        return val

    def actual_value(self, expr: Any, env: Frame) -> Any:
        return self.force_it(self.eval(expr, env))

    def eval_sequence(self, exprs: list[Any], env: Frame) -> Any:
        last: Any = None
        for expr in exprs:
            last = self.eval(expr, env)
        return last

    def eval(self, expr: Any, env: Frame) -> Any:
        if isinstance(expr, bool) or isinstance(expr, int):
            return expr
        if isinstance(expr, str):
            return env.lookup(expr)
        assert isinstance(expr, list)
        if not expr:
            return []
        head = expr[0]
        if head == "quote":
            return expr[1]
        if head == "if":
            if self.actual_value(expr[1], env) is not False:
                return self.eval(expr[2], env)
            return self.eval(expr[3], env) if len(expr) > 3 else False
        if head == "begin":
            return self.eval_sequence(expr[1:], env)
        if head == "lambda":
            return Closure(list(expr[1]), expr[2:], env)
        if head == "define":
            if isinstance(expr[1], list):
                name, *params = expr[1]
                env.bindings[name] = Closure(params, expr[2:], env)
            else:
                env.bindings[expr[1]] = self.eval(expr[2], env)
            return "ok"
        if head == "set!":
            env.set_var(expr[1], self.eval(expr[2], env))
            return "ok"
        proc = self.actual_value(head, env)
        return self.apply(proc, expr[1:], env)

    def apply(self, proc: Any, args: list[Any], call_env: Frame) -> Any:
        if isinstance(proc, Primitive):
            forced = [self.actual_value(a, call_env) for a in args]
            op = proc.name
            if op == "+":
                return sum(forced)
            if op == "*":
                acc = 1
                for x in forced:
                    acc *= x
                return acc
            if op == "-":
                return forced[0] - forced[1]
            if op == "/":
                if forced[1] == 0:
                    raise ZeroDivisionError("division by zero")
                return forced[0] // forced[1]
            if op == "=":
                return forced[0] == forced[1]
            if op == "<":
                return forced[0] < forced[1]
            if op == ">":
                return forced[0] > forced[1]
            raise ValueError(f"unknown primitive {op}")
        if isinstance(proc, Closure):
            if len(proc.params) != len(args):
                raise TypeError("arity mismatch")
            frame = Frame(
                {p: self.delay_it(a, call_env) for p, a in zip(proc.params, args)},
                parent=proc.env,
            )
            return self.eval_sequence(proc.body, frame)
        raise TypeError(f"not applicable: {proc!r}")

    def run_program(self, src: str) -> Any:
        last: Any = None
        for expr in parse_program(src):
            last = self.actual_value(expr, self.global_env)
        return last


class LazyEvaluatorTests(unittest.TestCase):
    def test_unless_and_memoization(self) -> None:
        vm = LazyMachine()
        count = vm.run_program(
            """
            (define count 0)
            (define (id x) (set! count (+ count 1)) x)
            (define (square x) (* x x))
            (define (unless cond usual exc) (if cond exc usual))
            (define a (unless #f (square (id 7)) (/ 1 0)))
            count
            """
        )
        self.assertEqual(count, 0)
        self.assertEqual(vm.run_program("a"), 49)
        self.assertEqual(vm.run_program("count"), 1)
        self.assertGreaterEqual(vm.memoized_hits, 1)

    def test_lazy_infinite_fibonacci_stream(self) -> None:
        vm = LazyMachine()
        res = vm.run_program(
            """
            (define (cons x y) (lambda (m) (m x y)))
            (define (car z) (z (lambda (p q) p)))
            (define (cdr z) (z (lambda (p q) q)))
            (define (list-ref items n)
              (if (= n 0) (car items) (list-ref (cdr items) (- n 1))))
            (define (add-lists a b)
              (cons (+ (car a) (car b)) (add-lists (cdr a) (cdr b))))
            (define fibs
              (cons 0 (cons 1 (add-lists (cdr fibs) fibs))))
            (list-ref fibs 10)
            """
        )
        self.assertEqual(res, 55)


if __name__ == "__main__":
    vm = LazyMachine()
    res = vm.run_program(
        """
        (define (unless condition usual exceptional)
          (if condition exceptional usual))
        (define (cons x y) (lambda (m) (m x y)))
        (define (car z) (z (lambda (p q) p)))
        (define (cdr z) (z (lambda (p q) q)))
        (define (list-ref items n)
          (if (= n 0) (car items) (list-ref (cdr items) (- n 1))))
        (define (add-lists l1 l2)
          (cons (+ (car l1) (car l2)) (add-lists (cdr l1) (cdr l2))))
        (define ones (cons 1 ones))
        (define integers (cons 1 (add-lists ones integers)))
        (+ (unless #f 10 (/ 1 0)) (list-ref integers 9))
        """
    )
    print(
        f"lazy result = {res} (created={vm.thunks_created}, forced={vm.thunks_forced}, memo_hits={vm.memoized_hits})"
    )
