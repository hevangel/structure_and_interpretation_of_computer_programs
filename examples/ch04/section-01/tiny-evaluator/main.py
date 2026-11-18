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


Expr = IntLit | Var | Add | Let


def evaluate(e: Expr, env: dict[str, int]) -> int:
    if isinstance(e, IntLit):
        return e.value
    if isinstance(e, Var):
        if e.name not in env:
            raise KeyError(f"unbound: {e.name}")
        return env[e.name]
    if isinstance(e, Add):
        return evaluate(e.left, env) + evaluate(e.right, env)
    inner = dict(env)
    inner[e.name] = evaluate(e.value, env)
    return evaluate(e.body, inner)


class TinyEvaluatorTests(unittest.TestCase):
    def test_lexical_binding(self) -> None:
        e = Let("x", IntLit(2), Var("x"))
        self.assertEqual(evaluate(e, {}), 2)
        with self.assertRaises(KeyError):
            evaluate(Var("missing"), {})


if __name__ == "__main__":
    e = Let("x", IntLit(4), Add(Var("x"), IntLit(3)))
    print(f"evaluated: {evaluate(e, {})}")
