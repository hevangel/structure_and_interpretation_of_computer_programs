from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class IntLit:
    value: int


@dataclass(frozen=True)
class Var:
    name: str


@dataclass(frozen=True)
class Add:
    left: "Expr"
    right: "Expr"


@dataclass(frozen=True)
class Let:
    name: str
    value: "Expr"
    body: "Expr"


@dataclass(frozen=True)
class Lambda:
    param: str
    body: "Expr"


@dataclass(frozen=True)
class Call:
    function: "Expr"
    argument: "Expr"


Expr = IntLit | Var | Add | Let | Lambda | Call


@dataclass(frozen=True)
class Closure:
    param: str
    body: Expr
    env: dict[str, "Value"]


Value = int | Closure


def expect_number(v: Value) -> int:
    if not isinstance(v, int):
        raise TypeError("expected number")
    return v


def evaluate(e: Expr, env: dict[str, Value]) -> Value:
    if isinstance(e, IntLit):
        return e.value
    if isinstance(e, Var):
        if e.name not in env:
            raise KeyError(f"unbound: {e.name}")
        return env[e.name]
    if isinstance(e, Add):
        return expect_number(evaluate(e.left, env)) + expect_number(evaluate(e.right, env))
    if isinstance(e, Let):
        inner = dict(env)
        inner[e.name] = evaluate(e.value, env)
        return evaluate(e.body, inner)
    if isinstance(e, Lambda):
        return Closure(e.param, e.body, dict(env))
    func = evaluate(e.function, env)
    arg = evaluate(e.argument, env)
    if not isinstance(func, Closure):
        raise TypeError("cannot call a number")
    # Extend the definition environment, not the call-site environment.
    call_env = dict(func.env)
    call_env[func.param] = arg
    return evaluate(func.body, call_env)


def example() -> Expr:
    return Let(
        "x",
        IntLit(10),
        Let(
            "f",
            Lambda("y", Add(Var("x"), Var("y"))),
            Let("x", IntLit(100), Call(Var("f"), IntLit(3))),
        ),
    )


class LexicalClosuresTests(unittest.TestCase):
    def test_uses_definition_environment(self) -> None:
        result = evaluate(example(), {})
        self.assertEqual(expect_number(result), 13)


if __name__ == "__main__":
    print(f"captured lexical x gives {expect_number(evaluate(example(), {}))}")
