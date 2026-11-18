from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Num:
    value: int


@dataclass(frozen=True)
class Var:
    name: str


@dataclass(frozen=True)
class Add:
    left: object
    right: object


@dataclass(frozen=True)
class EqZero:
    operand: object


@dataclass(frozen=True)
class IfExpr:
    pred: object
    consequent: object
    alternative: object


@dataclass(frozen=True)
class Call:
    name: str
    args: tuple[object, ...]


@dataclass(frozen=True)
class Block:
    defs: tuple[tuple[str, object], ...]
    body: object


@dataclass(frozen=True)
class Proc:
    params: tuple[str, ...]
    body: object


@dataclass(frozen=True)
class Deferred:
    expr: object
    env: dict[str, object]


class StepLimitExceeded(RuntimeError):
    pass


class Evaluator:
    def __init__(self, procs: dict[str, Proc], step_limit: int = 50) -> None:
        self.procs = procs
        self.step_limit = step_limit
        self.steps = 0

    def _tick(self) -> None:
        self.steps += 1
        if self.steps > self.step_limit:
            raise StepLimitExceeded("evaluation exceeded step limit")

    def eval_applicative(self, expr: object, env: dict[str, int]) -> int:
        self._tick()
        if isinstance(expr, Num):
            return expr.value
        if isinstance(expr, Var):
            if expr.name not in env:
                raise KeyError(f"unbound variable: {expr.name}")
            return env[expr.name]
        if isinstance(expr, Add):
            return self.eval_applicative(expr.left, env) + self.eval_applicative(expr.right, env)
        if isinstance(expr, EqZero):
            return int(self.eval_applicative(expr.operand, env) == 0)
        if isinstance(expr, IfExpr):
            if self.eval_applicative(expr.pred, env) != 0:
                return self.eval_applicative(expr.consequent, env)
            return self.eval_applicative(expr.alternative, env)
        if isinstance(expr, Call):
            proc = self.procs[expr.name]
            if len(proc.params) != len(expr.args):
                raise ValueError("arity mismatch")
            # Applicative order evaluates all arguments before entering the procedure body.
            local = {
                param: self.eval_applicative(arg, env)
                for param, arg in zip(proc.params, expr.args)
            }
            return self.eval_applicative(proc.body, local)
        if isinstance(expr, Block):
            inner = dict(env)
            for name, rhs in expr.defs:
                inner[name] = self.eval_applicative(rhs, inner)
            return self.eval_applicative(expr.body, inner)
        raise TypeError(f"unknown expression: {expr!r}")

    def eval_normal(self, expr: object, env: dict[str, object]) -> int:
        self._tick()
        if isinstance(expr, Num):
            return expr.value
        if isinstance(expr, Var):
            entry = env[expr.name]
            if isinstance(entry, Deferred):
                return self.eval_normal(entry.expr, entry.env)
            return int(entry)
        if isinstance(expr, Add):
            return self.eval_normal(expr.left, env) + self.eval_normal(expr.right, env)
        if isinstance(expr, EqZero):
            return int(self.eval_normal(expr.operand, env) == 0)
        if isinstance(expr, IfExpr):
            if self.eval_normal(expr.pred, env) != 0:
                return self.eval_normal(expr.consequent, env)
            return self.eval_normal(expr.alternative, env)
        if isinstance(expr, Call):
            proc = self.procs[expr.name]
            if len(proc.params) != len(expr.args):
                raise ValueError("arity mismatch")
            # Normal order defers argument evaluation until forced by a primitive or conditional.
            local: dict[str, object] = {
                param: Deferred(arg, env)
                for param, arg in zip(proc.params, expr.args)
            }
            return self.eval_normal(proc.body, local)
        if isinstance(expr, Block):
            inner = dict(env)
            for name, rhs in expr.defs:
                inner[name] = self.eval_normal(rhs, inner)
            return self.eval_normal(expr.body, inner)
        raise TypeError(f"unknown expression: {expr!r}")


def ben_bitdiddle_setup() -> tuple[dict[str, Proc], Call]:
    procs = {
        "p": Proc((), Call("p", ())),
        "test": Proc(
            ("x", "y"),
            IfExpr(EqZero(Var("x")), Num(0), Var("y")),
        ),
    }
    call = Call("test", (Num(0), Call("p", ())))
    return procs, call


class EvaluationOrdersTest(unittest.TestCase):
    def test_applicative_diverges_while_normal_terminates(self) -> None:
        procs, call = ben_bitdiddle_setup()
        app = Evaluator(procs, step_limit=30)
        with self.assertRaises(StepLimitExceeded):
            app.eval_applicative(call, {})

        norm = Evaluator(procs, step_limit=30)
        self.assertEqual(norm.eval_normal(call, {}), 0)
        self.assertLess(norm.steps, 10)

    def test_block_scoping_preserves_outer_bindings(self) -> None:
        ev = Evaluator({}, step_limit=50)
        expr = Block(
            defs=(("x", Num(10)),),
            body=Add(
                Block(defs=(("x", Num(3)),), body=Var("x")),
                Var("x"),
            ),
        )
        self.assertEqual(ev.eval_applicative(expr, {}), 13)
        self.assertEqual(ev.eval_normal(expr, {}), 13)


if __name__ == "__main__":
    procs, call = ben_bitdiddle_setup()
    norm = Evaluator(procs, step_limit=25)
    print(f"normal order test(0, p()) -> {norm.eval_normal(call, {})} in {norm.steps} steps")
    app = Evaluator(procs, step_limit=25)
    try:
        app.eval_applicative(call, {})
    except StepLimitExceeded as exc:
        print(f"applicative order test(0, p()) -> {exc}")
