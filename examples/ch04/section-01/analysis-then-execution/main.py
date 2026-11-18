from collections.abc import Callable
from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Number:
    value: int


@dataclass(frozen=True)
class Variable:
    name: str


@dataclass(frozen=True)
class Add:
    left: "Expr"
    right: "Expr"


Expr = Number | Variable | Add
Executor = Callable[[dict[str, int]], int]


def analyze(expr: Expr) -> Executor:
    if isinstance(expr, Number):
        val = expr.value
        return lambda _env: val
    if isinstance(expr, Variable):
        name = expr.name

        def lookup(env: dict[str, int]) -> int:
            if name not in env:
                raise KeyError(f"unbound: {name}")
            return env[name]

        return lookup
    left_exec = analyze(expr.left)
    right_exec = analyze(expr.right)
    # Reuse pre-analyzed child executors across environments.
    return lambda env: left_exec(env) + right_exec(env)


class AnalysisThenExecutionTests(unittest.TestCase):
    def test_analysis_is_reusable(self) -> None:
        expr = Add(Variable("x"), Number(3))
        program = analyze(expr)
        self.assertEqual(program({"x": 7}), 10)
        self.assertEqual(program({"x": 20}), 23)
        with self.assertRaises(KeyError):
            program({})


if __name__ == "__main__":
    expr = Add(Variable("x"), Number(3))
    program = analyze(expr)
    print(f"x=7 => {program({'x': 7})}")
