from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Callable
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


class _Unassigned:
    pass


UNASSIGNED = _Unassigned()


@dataclass
class Frame:
    bindings: dict[str, Any]
    parent: Frame | None = None

    def lookup(self, name: str) -> Any:
        cur: Frame | None = self
        while cur is not None:
            if name in cur.bindings:
                val = cur.bindings[name]
                if val is UNASSIGNED:
                    raise RuntimeError(f"unassigned variable: {name}")
                return val
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
class Closure:
    params: list[str]
    body: list[Any]
    env: Frame


def scan_out_defines(body: list[Any]) -> list[Any]:
    defines: list[tuple[str, Any]] = []
    rest: list[Any] = []
    for expr in body:
        if isinstance(expr, list) and expr and expr[0] == "define":
            if isinstance(expr[1], list):
                name, *params = expr[1]
                defines.append((name, ["lambda", params, *expr[2:]]))
            else:
                defines.append((expr[1], expr[2]))
        else:
            rest.append(expr)
    if not defines:
        return body
    bindings = [[n, "*unassigned*"] for n, _ in defines]
    sets = [["set!", n, rhs] for n, rhs in defines]
    return [["let", bindings, *sets, *rest]]


def eval_seq(exprs: list[Any], env: Frame) -> Any:
    res = None
    for e in exprs:
        res = evaluate(e, env)
    return res


def evaluate(expr: Any, env: Frame) -> Any:
    if isinstance(expr, (int, bool)):
        return expr
    if expr == "*unassigned*":
        return UNASSIGNED
    if isinstance(expr, str):
        return env.lookup(expr)
    if not expr:
        return ()
    head = expr[0]
    if head == "quote":
        return expr[1]
    if head == "if":
        if evaluate(expr[1], env) is not False:
            return evaluate(expr[2], env)
        return evaluate(expr[3], env) if len(expr) > 3 else False
    if head == "cond":
        for clause in expr[1:]:
            if clause[0] == "else" or evaluate(clause[0], env) is not False:
                return eval_seq(clause[1:], env)
        return False
    if head == "begin":
        return eval_seq(expr[1:], env)
    if head == "set!":
        val = evaluate(expr[2], env)
        env.set_var(expr[1], val)
        return val
    if head == "define":
        if isinstance(expr[1], list):
            name, *params = expr[1]
            val: Any = Closure(params, scan_out_defines(expr[2:]), env)
        else:
            name = expr[1]
            val = evaluate(expr[2], env)
        env.bindings[name] = val
        return val
    if head == "lambda":
        return Closure(expr[1], scan_out_defines(expr[2:]), env)
    if head == "let":
        params = [b[0] for b in expr[1]]
        args = [b[1] for b in expr[1]]
        return evaluate([["lambda", params, *expr[2:]], *args], env)
    if head == "let*":
        nested: Any = ["begin", *expr[2:]]
        for pair in reversed(expr[1]):
            nested = ["let", [pair], nested]
        return evaluate(nested, env)
    proc = evaluate(head, env)
    args = [evaluate(a, env) for a in expr[1:]]
    if isinstance(proc, Closure):
        call_env = Frame(dict(zip(proc.params, args)), proc.env)
        return eval_seq(proc.body, call_env)
    assert callable(proc)
    return proc(*args)


def setup_global_env() -> Frame:
    return Frame(
        {
            "+": lambda *xs: sum(xs),
            "-": lambda a, b=None: -a if b is None else a - b,
            "*": lambda *xs: 1 if not xs else xs[0] * setup_global_env().lookup("*")( *xs[1:] ) if len(xs) > 1 else xs[0],
            "=": lambda a, b: a == b,
            "<": lambda a, b: a < b,
            "cons": lambda a, b: (a, b),
            "car": lambda p: p[0],
            "cdr": lambda p: p[1],
        }
    )


def run_program(src: str) -> Any:
    env = setup_global_env()
    return eval_seq(parse_program(src), env)


class MetacircularEvaluatorTest(unittest.TestCase):
    def test_mutual_internal_defines_cond_and_let_star(self) -> None:
        src = """
            (define (f x)
              (define (even? n) (if (= n 0) #t (odd? (- n 1))))
              (define (odd? n) (if (= n 0) #f (even? (- n 1))))
              (cond ((even? x) 'even) (else 'odd)))
            (let* ((a 2) (b (+ a 4)))
              (cons (f b) (f (- b 1))))
        """
        self.assertEqual(run_program(src), ("even", "odd"))

    def test_y_combinator_without_define(self) -> None:
        src = """
            ((lambda (n)
               ((lambda (fact) (fact fact n))
                (lambda (ft k)
                  (if (= k 1) 1 (* k (ft ft (- k 1)))))))
             6)
        """
        self.assertEqual(run_program(src), 720)


if __name__ == "__main__":
    print("Y-combinator 6! =", run_program("((lambda (n) ((lambda (f) (f f n)) (lambda (ft k) (if (= k 1) 1 (* k (ft ft (- k 1))))))) 6)"))
