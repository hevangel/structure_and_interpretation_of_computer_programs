from __future__ import annotations

from dataclasses import dataclass, field
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
        elif c in "()":
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


@dataclass
class Compound:
    params: list[str]
    body: list[Any]
    env: Frame


@dataclass(frozen=True)
class Primitive:
    name: str


@dataclass
class StackStats:
    total_pushes: int = 0
    current_depth: int = 0
    max_depth: int = 0


class ExplicitControlEvaluator:
    def __init__(self) -> None:
        self.global_env = Frame({op: Primitive(op) for op in ("+", "-", "*", "=", "<", ">")})
        self.exp: Any = None
        self.env: Frame = self.global_env
        self.val: Any = None
        self.cont: str = "done"
        self.proc: Any = None
        self.argl: list[Any] = []
        self.unev: Any = None
        self.stack: list[Any] = []
        self.stats = StackStats()

    def save(self, value: Any) -> None:
        self.stack.append(value)
        self.stats.total_pushes += 1
        self.stats.current_depth += 1
        if self.stats.current_depth > self.stats.max_depth:
            self.stats.max_depth = self.stats.current_depth

    def restore(self) -> Any:
        self.stats.current_depth -= 1
        return self.stack.pop()

    def reset_stats(self) -> None:
        self.stack.clear()
        self.stats = StackStats()

    def eval_expr(self, expr: Any) -> Any:
        self.exp = expr
        self.env = self.global_env
        self.cont = "done"
        state = "eval_dispatch"

        while True:
            if state == "done":
                return self.val
            if state == "eval_dispatch":
                e = self.exp
                if isinstance(e, (bool, int)):
                    self.val = e
                    state = self.cont
                elif isinstance(e, str):
                    cur: Frame | None = self.env
                    found = False
                    while cur is not None:
                        if e in cur.bindings:
                            self.val = cur.bindings[e]
                            found = True
                            break
                        cur = cur.parent
                    if not found:
                        raise KeyError(f"unbound: {e}")
                    state = self.cont
                else:
                    head = e[0]
                    if head == "if":
                        self.save(self.exp)
                        self.save(self.env)
                        self.save(self.cont)
                        self.cont = "ev_if_decide"
                        self.exp = e[1]
                        state = "eval_dispatch"
                    elif head == "begin":
                        self.unev = list(e[1:])
                        self.save(self.cont)
                        state = "ev_sequence"
                    elif head == "lambda":
                        self.val = Compound(list(e[1]), list(e[2:]), self.env)
                        state = self.cont
                    elif head == "define":
                        if isinstance(e[1], list):
                            name, *params = e[1]
                            val_expr = ["lambda", params, *e[2:]]
                        else:
                            name, val_expr = e[1], e[2]
                        self.unev = name
                        self.save(self.unev)
                        self.exp = val_expr
                        self.save(self.env)
                        self.save(self.cont)
                        self.cont = "ev_definition_1"
                        state = "eval_dispatch"
                    elif head == "set!":
                        self.unev = e[1]
                        self.save(self.unev)
                        self.exp = e[2]
                        self.save(self.env)
                        self.save(self.cont)
                        self.cont = "ev_assignment_1"
                        state = "eval_dispatch"
                    else:
                        self.save(self.cont)
                        self.save(self.env)
                        self.unev = list(e[1:])
                        self.save(self.unev)
                        self.exp = head
                        self.cont = "ev_appl_did_operator"
                        state = "eval_dispatch"
            elif state == "ev_appl_did_operator":
                self.unev = self.restore()
                self.env = self.restore()
                self.argl = []
                self.proc = self.val
                if not self.unev:
                    state = "apply_dispatch"
                else:
                    self.save(self.proc)
                    state = "ev_appl_operand_loop"
            elif state == "ev_appl_operand_loop":
                self.save(list(self.argl))
                self.exp = self.unev[0]
                if len(self.unev) == 1:
                    state = "ev_appl_last_arg"
                else:
                    self.save(self.env)
                    self.save(self.unev)
                    self.cont = "ev_appl_accumulate_arg"
                    state = "eval_dispatch"
            elif state == "ev_appl_accumulate_arg":
                self.unev = self.restore()
                self.env = self.restore()
                self.argl = self.restore()
                self.argl.append(self.val)
                self.unev = self.unev[1:]
                state = "ev_appl_operand_loop"
            elif state == "ev_appl_last_arg":
                self.cont = "ev_appl_accum_last_arg"
                state = "eval_dispatch"
            elif state == "ev_appl_accum_last_arg":
                self.argl = self.restore()
                self.argl.append(self.val)
                self.proc = self.restore()
                state = "apply_dispatch"
            elif state == "apply_dispatch":
                if isinstance(self.proc, Primitive):
                    op = self.proc.name
                    a = self.argl
                    if op == "+":
                        self.val = sum(a)
                    elif op == "*":
                        p = 1
                        for x in a:
                            p *= x
                        self.val = p
                    elif op == "-":
                        self.val = a[0] - a[1]
                    elif op == "=":
                        self.val = a[0] == a[1]
                    elif op == "<":
                        self.val = a[0] < a[1]
                    elif op == ">":
                        self.val = a[0] > a[1]
                    self.cont = self.restore()
                    state = self.cont
                else:
                    assert isinstance(self.proc, Compound)
                    self.env = Frame(
                        dict(zip(self.proc.params, self.argl)), parent=self.proc.env
                    )
                    self.unev = list(self.proc.body)
                    state = "ev_sequence"
            elif state == "ev_sequence":
                self.exp = self.unev[0]
                if len(self.unev) == 1:
                    state = "ev_sequence_last_exp"
                else:
                    self.save(self.unev)
                    self.save(self.env)
                    self.cont = "ev_sequence_continue"
                    state = "eval_dispatch"
            elif state == "ev_sequence_continue":
                self.env = self.restore()
                self.unev = self.restore()
                self.unev = self.unev[1:]
                state = "ev_sequence"
            elif state == "ev_sequence_last_exp":
                self.cont = self.restore()
                state = "eval_dispatch"
            elif state == "ev_if_decide":
                self.cont = self.restore()
                self.env = self.restore()
                self.exp = self.restore()
                if self.val is not False:
                    self.exp = self.exp[2]
                    state = "eval_dispatch"
                elif len(self.exp) > 3:
                    self.exp = self.exp[3]
                    state = "eval_dispatch"
                else:
                    self.val = False
                    state = self.cont
            elif state == "ev_definition_1":
                self.cont = self.restore()
                self.env = self.restore()
                self.unev = self.restore()
                self.env.bindings[self.unev] = self.val
                self.val = "ok"
                state = self.cont
            elif state == "ev_assignment_1":
                self.cont = self.restore()
                self.env = self.restore()
                self.unev = self.restore()
                cur = self.env
                while cur is not None:
                    if self.unev in cur.bindings:
                        cur.bindings[self.unev] = self.val
                        break
                    cur = cur.parent
                self.val = "ok"
                state = self.cont

    def run_program(self, src: str) -> Any:
        last: Any = None
        for expr in parse_program(src):
            last = self.eval_expr(expr)
        return last


class ExplicitControlTests(unittest.TestCase):
    def test_factorial_stack_formulas(self) -> None:
        vm = ExplicitControlEvaluator()
        vm.run_program(
            """
            (define (factorial n)
              (if (= n 1)
                  1
                  (* (factorial (- n 1)) n)))
            """
        )
        expected = 1
        for n in range(1, 9):
            expected *= n
            vm.reset_stats()
            self.assertEqual(vm.run_program(f"(factorial {n})"), expected)
            self.assertEqual(vm.stats.total_pushes, 32 * n - 16)
            self.assertEqual(vm.stats.max_depth, 5 * n + 3)

    def test_tail_recursion_constant_stack_depth(self) -> None:
        vm = ExplicitControlEvaluator()
        vm.run_program(
            """
            (define (sum-iter n acc)
              (if (= n 0)
                  acc
                  (sum-iter (- n 1) (+ acc n))))
            """
        )
        vm.reset_stats()
        r10 = vm.run_program("(sum-iter 10 0)")
        d10 = vm.stats.max_depth
        vm.reset_stats()
        r500 = vm.run_program("(sum-iter 500 0)")
        d500 = vm.stats.max_depth
        self.assertEqual(r10, 55)
        self.assertEqual(r500, 125_250)
        self.assertEqual(d10, d500)


if __name__ == "__main__":
    vm = ExplicitControlEvaluator()
    vm.run_program(
        """
        (define (factorial n)
          (if (= n 1)
              1
              (* (factorial (- n 1)) n)))
        (define (sum-iter n acc)
          (if (= n 0)
              acc
              (sum-iter (- n 1) (+ acc n))))
        """
    )
    vm.reset_stats()
    f6 = vm.run_program("(factorial 6)")
    print(f"factorial(6) = {f6}, pushes = {vm.stats.total_pushes}, max_depth = {vm.stats.max_depth}")
    vm.reset_stats()
    vm.run_program("(sum-iter 10 0)")
    d10 = vm.stats.max_depth
    vm.reset_stats()
    s500 = vm.run_program("(sum-iter 500 0)")
    d500 = vm.stats.max_depth
    print(f"sum(500) = {s500}, depth(10) = {d10}, depth(500) = {d500}")
