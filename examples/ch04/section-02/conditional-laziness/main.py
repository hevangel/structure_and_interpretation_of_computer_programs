from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Number:
    value: int


@dataclass(frozen=True)
class Divide:
    left: "Expr"
    right: "Expr"


@dataclass(frozen=True)
class IfZero:
    condition: "Expr"
    then_branch: "Expr"
    else_branch: "Expr"


Expr = Number | Divide | IfZero


def evaluate(expr: Expr) -> int:
    if isinstance(expr, Number):
        return expr.value
    if isinstance(expr, Divide):
        lhs = evaluate(expr.left)
        rhs = evaluate(expr.right)
        if rhs == 0:
            raise ZeroDivisionError("division by zero")
        return int(lhs / rhs)
    # Never evaluate the unselected branch.
    if evaluate(expr.condition) == 0:
        return evaluate(expr.then_branch)
    return evaluate(expr.else_branch)


class ConditionalLazinessTests(unittest.TestCase):
    def test_only_selected_branch_is_evaluated(self) -> None:
        bad = Divide(Number(1), Number(0))
        safe = IfZero(Number(0), Number(42), bad)
        self.assertEqual(evaluate(safe), 42)
        with self.assertRaises(ZeroDivisionError):
            evaluate(Divide(Number(7), Number(0)))


if __name__ == "__main__":
    e = IfZero(Number(0), Number(42), Divide(Number(1), Number(0)))
    print(f"lazy branch result: {evaluate(e)}")
