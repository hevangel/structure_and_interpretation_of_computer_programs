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
        raise KeyError(name)

    def set_var(self, name: str, val: Any) -> Any:
        cur: Frame | None = self
        while cur is not None:
            if name in cur.bindings:
                old = cur.bindings[name]
                cur.bindings[name] = val
                return old
            cur = cur.parent
        raise KeyError(name)


@dataclass
class Closure:
    params: list[str]
    body: Callable[..., Any]
    env: Frame


@dataclass(frozen=True)
class Primitive:
    name: str


def apply_primitive(op: str, args: list[Any]) -> Any:
    if op == "+":
        return sum(args)
    if op == "*":
        acc = 1
        for x in args:
            acc *= x
        return acc
    if op == "-":
        return args[0] - args[1]
    if op == "=":
        return args[0] == args[1]
    if op == "<":
        return args[0] < args[1]
    if op == ">":
        return args[0] > args[1]
    if op == "not":
        return args[0] is False
    if op == "null?":
        return args[0] == ()
    if op == "pair?":
        return isinstance(args[0], tuple) and len(args[0]) == 2
    if op == "eq?":
        return args[0] == args[1]
    if op == "cons":
        return (args[0], args[1])
    if op == "car":
        return args[0][0]
    if op == "cdr":
        return args[0][1]
    if op == "list":
        acc: Any = ()
        for item in reversed(args):
            acc = (item, acc)
        return acc
    if op == "abs":
        return abs(args[0])
    raise ValueError(f"unknown primitive {op}")


def format_sexp(val: Any) -> str:
    if isinstance(val, tuple) and len(val) == 2:
        parts = []
        cur: Any = val
        while isinstance(cur, tuple) and len(cur) == 2:
            parts.append(format_sexp(cur[0]))
            cur = cur[1]
        if cur == ():
            return f"({' '.join(parts)})"
        return f"({' '.join(parts)} . {format_sexp(cur)})"
    if val == ():
        return "()"
    if isinstance(val, bool):
        return "#t" if val else "#f"
    return str(val)


def to_scheme_quote(expr: Any) -> Any:
    if isinstance(expr, list):
        acc: Any = ()
        for item in reversed(expr):
            acc = (to_scheme_quote(item), acc)
        return acc
    return expr


def analyze_sequence(exprs: list[Any]) -> Callable[..., Any]:
    procs = [analyze(e) for e in exprs]

    def sequentially(p1: Callable[..., Any], p2: Callable[..., Any]) -> Callable[..., Any]:
        def run(env: Frame, succeed: Callable[..., Any], fail: Callable[[], Any]) -> Any:
            return p1(
                env,
                lambda _val, fail2: (lambda: p2(env, succeed, fail2)),
                fail,
            )

        return run

    acc = procs[0]
    for p in procs[1:]:
        acc = sequentially(acc, p)
    return acc


def analyze(expr: Any) -> Callable[..., Any]:
    if isinstance(expr, (bool, int)):
        return lambda _env, succeed, fail: succeed(expr, fail)
    if isinstance(expr, str):
        def lookup_exec(env: Frame, succeed: Callable[..., Any], fail: Callable[[], Any]) -> Any:
            try:
                val = env.lookup(expr)
            except KeyError:
                return fail
            return succeed(val, fail)

        return lookup_exec
    assert isinstance(expr, list)
    if not expr:
        return lambda _env, succeed, fail: succeed((), fail)
    head = expr[0]
    if head == "quote":
        quoted = to_scheme_quote(expr[1])
        return lambda _env, succeed, fail: succeed(quoted, fail)
    if head == "amb":
        choices = [analyze(c) for c in expr[1:]]

        def amb_exec(env: Frame, succeed: Callable[..., Any], fail: Callable[[], Any]) -> Any:
            def try_next(rest: list[Callable[..., Any]]) -> Any:
                if not rest:
                    return fail
                return rest[0](env, succeed, lambda: (lambda: try_next(rest[1:])))

            return try_next(choices)

        return amb_exec
    if head == "require":
        pred = analyze(expr[1])

        def req_exec(env: Frame, succeed: Callable[..., Any], fail: Callable[[], Any]) -> Any:
            return pred(
                env,
                lambda v, fail2: succeed("ok", fail2) if v is not False else fail2,
                fail,
            )

        return req_exec
    if head == "if":
        pred = analyze(expr[1])
        cseq = analyze(expr[2])
        alt = analyze(expr[3]) if len(expr) > 3 else (lambda _e, s, f: s(False, f))

        def if_exec(env: Frame, succeed: Callable[..., Any], fail: Callable[[], Any]) -> Any:
            return pred(
                env,
                lambda pv, fail2: cseq(env, succeed, fail2)
                if pv is not False
                else alt(env, succeed, fail2),
                fail,
            )

        return if_exec
    if head == "if-fail":
        first = analyze(expr[1])
        second = analyze(expr[2])
        return lambda env, succeed, fail: first(
            env, succeed, lambda: second(env, succeed, fail)
        )
    if head == "begin":
        return analyze_sequence(expr[1:])
    if head == "lambda":
        params = list(expr[1])
        body = analyze_sequence(expr[2:])
        return lambda env, succeed, fail: succeed(Closure(params, body, env), fail)
    if head == "define":
        if isinstance(expr[1], list):
            name, *params = expr[1]
            body = analyze_sequence(expr[2:])
            rhs: Callable[..., Any] = lambda env, s, f: s(Closure(params, body, env), f)
        else:
            name = expr[1]
            rhs = analyze(expr[2])

        def def_exec(env: Frame, succeed: Callable[..., Any], fail: Callable[[], Any]) -> Any:
            def after(val: Any, fail2: Callable[[], Any]) -> Any:
                env.bindings[name] = val
                return succeed("ok", fail2)

            return rhs(env, after, fail)

        return def_exec
    if head in ("set!", "permanent-set!"):
        name = expr[1]
        val_exec = analyze(expr[2])
        permanent = head == "permanent-set!"

        def set_exec(env: Frame, succeed: Callable[..., Any], fail: Callable[[], Any]) -> Any:
            def after(new_val: Any, fail2: Callable[[], Any]) -> Any:
                try:
                    old_val = env.set_var(name, new_val)
                except KeyError:
                    return fail2
                if permanent:
                    return succeed("ok", fail2)

                def undo() -> Any:
                    env.set_var(name, old_val)
                    return fail2

                return succeed("ok", undo)

            return val_exec(env, after, fail)

        return set_exec

    fproc = analyze(head)
    aprocs = [analyze(a) for a in expr[1:]]

    def get_args(
        remaining: list[Callable[..., Any]],
        env: Frame,
        succeed: Callable[[list[Any], Callable[[], Any]], Any],
        fail: Callable[[], Any],
    ) -> Any:
        if not remaining:
            return succeed([], fail)
        return remaining[0](
            env,
            lambda arg, fail2: get_args(
                remaining[1:],
                env,
                lambda rest_args, fail3: succeed([arg, *rest_args], fail3),
                fail2,
            ),
            fail,
        )

    def app_exec(env: Frame, succeed: Callable[..., Any], fail: Callable[[], Any]) -> Any:
        def on_proc(proc: Any, fail2: Callable[[], Any]) -> Any:
            def on_args(args: list[Any], fail3: Callable[[], Any]) -> Any:
                if isinstance(proc, Primitive):
                    try:
                        val = apply_primitive(proc.name, args)
                    except Exception:
                        return fail3
                    return succeed(val, fail3)
                if isinstance(proc, Closure) and len(proc.params) == len(args):
                    frame = Frame(dict(zip(proc.params, args)), parent=proc.env)
                    return lambda: proc.body(frame, succeed, fail3)
                return fail3

            return get_args(aprocs, env, on_args, fail2)

        return fproc(env, on_proc, fail)

    return app_exec


def collect_all(src: str) -> list[Any]:
    exprs = parse_program(src)
    prog = analyze_sequence(exprs)
    env = Frame(
        {
            p: Primitive(p)
            for p in (
                "+",
                "-",
                "*",
                "=",
                "<",
                ">",
                "not",
                "null?",
                "pair?",
                "eq?",
                "cons",
                "car",
                "cdr",
                "list",
                "abs",
            )
        }
    )
    results: list[Any] = []

    def on_success(val: Any, fail: Callable[[], Any]) -> Any:
        results.append(val)
        return fail

    bounce = prog(env, on_success, lambda: None)
    while bounce is not None:
        bounce = bounce()
    return results


class AmbEvaluatorTests(unittest.TestCase):
    def test_pythagorean_triples_and_multiple_dwelling(self) -> None:
        triples = collect_all(
            """
            (define (an-integer-between low high)
              (require (not (> low high)))
              (amb low (an-integer-between (+ low 1) high)))
            (define a (an-integer-between 1 13))
            (define b (an-integer-between a 13))
            (define c (an-integer-between b 13))
            (require (= (+ (* a a) (* b b)) (* c c)))
            (list a b c)
            """
        )
        self.assertEqual(
            [format_sexp(t) for t in triples], ["(3 4 5)", "(5 12 13)", "(6 8 10)"]
        )

        dwelling = collect_all(
            """
            (define (distinct? items)
              (if (null? items) #t
                  (if (null? (cdr items)) #t
                      (if (member? (car items) (cdr items)) #f
                          (distinct? (cdr items))))))
            (define (member? x xs)
              (if (null? xs) #f
                  (if (= x (car xs)) #t (member? x (cdr xs)))))
            (define baker (amb 1 2 3 4 5))
            (define cooper (amb 1 2 3 4 5))
            (define fletcher (amb 1 2 3 4 5))
            (define miller (amb 1 2 3 4 5))
            (define smith (amb 1 2 3 4 5))
            (require (distinct? (list baker cooper fletcher miller smith)))
            (require (not (= baker 5)))
            (require (not (= cooper 1)))
            (require (not (= fletcher 5)))
            (require (not (= fletcher 1)))
            (require (> miller cooper))
            (require (not (= (abs (- smith fletcher)) 1)))
            (require (not (= (abs (- fletcher cooper)) 1)))
            (list baker cooper fletcher miller smith)
            """
        )
        self.assertEqual([format_sexp(x) for x in dwelling], ["(3 2 4 5 1)"])

    def test_reversible_vs_permanent_set(self) -> None:
        out = collect_all(
            """
            (define count 0)
            (define temp 0)
            (if-fail
              (begin
                (define x (amb 1 3 5))
                (permanent-set! count (+ count 1))
                (set! temp (+ temp 10))
                (require (= x 9))
                x)
              (list count temp))
            """
        )
        self.assertEqual([format_sexp(x) for x in out], ["(3 0)"])


if __name__ == "__main__":
    triples = collect_all(
        """
        (define (an-integer-between low high)
          (require (not (> low high)))
          (amb low (an-integer-between (+ low 1) high)))
        (define a (an-integer-between 1 13))
        (define b (an-integer-between a 13))
        (define c (an-integer-between b 13))
        (require (= (+ (* a a) (* b b)) (* c c)))
        (list a b c)
        """
    )
    for t in triples:
        print(f"triple: {format_sexp(t)}")
