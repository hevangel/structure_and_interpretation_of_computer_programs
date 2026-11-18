from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Literal:
    value: int


@dataclass(frozen=True)
class VarX:
    pass


@dataclass(frozen=True)
class Add:
    lhs: Expr
    rhs: Expr


@dataclass(frozen=True)
class Mul:
    lhs: Expr
    rhs: Expr


Expr = Literal | VarX | Add | Mul


@dataclass(frozen=True)
class Push:
    value: int


@dataclass(frozen=True)
class LoadX:
    pass


@dataclass(frozen=True)
class OpAdd:
    pass


@dataclass(frozen=True)
class OpMul:
    pass


Op = Push | LoadX | OpAdd | OpMul


def fold(expr: Expr) -> Expr:
    if isinstance(expr, Add):
        a = fold(expr.lhs)
        b = fold(expr.rhs)
        if isinstance(a, Literal) and isinstance(b, Literal):
            return Literal(a.value + b.value)
        return Add(a, b)
    if isinstance(expr, Mul):
        a = fold(expr.lhs)
        b = fold(expr.rhs)
        if isinstance(a, Literal) and isinstance(b, Literal):
            return Literal(a.value * b.value)
        return Mul(a, b)
    return expr


def compile_expr(expr: Expr, out: list[Op]) -> None:
    if isinstance(expr, Literal):
        out.append(Push(expr.value))
    elif isinstance(expr, VarX):
        out.append(LoadX())
    elif isinstance(expr, Add):
        compile_expr(expr.lhs, out)
        compile_expr(expr.rhs, out)
        out.append(OpAdd())
    elif isinstance(expr, Mul):
        compile_expr(expr.lhs, out)
        compile_expr(expr.rhs, out)
        out.append(OpMul())


def run(code: list[Op], x: int) -> int:
    stack: list[int] = []
    for op in code:
        if isinstance(op, Push):
            stack.append(op.value)
        elif isinstance(op, LoadX):
            stack.append(x)
        elif isinstance(op, (OpAdd, OpMul)):
            if len(stack) < 2:
                raise ValueError("missing operand")
            b = stack.pop()
            a = stack.pop()
            stack.append(a + b if isinstance(op, OpAdd) else a * b)
    if len(stack) != 1:
        raise ValueError("invalid stack depth")
    return stack.pop()


def sample_expression() -> Expr:
    # 2*3+x: its constant subtree can be evaluated before runtime.
    return Add(Mul(Literal(2), Literal(3)), VarX())


def main() -> None:
    original = sample_expression()
    before: list[Op] = []
    compile_expr(original, before)
    after: list[Op] = []
    compile_expr(fold(original), after)
    print(
        f"before={len(before)} opcodes ({run(before, 4)}), "
        f"after={len(after)} opcodes ({run(after, 4)})"
    )


class TestConstantFoldingCompiler(unittest.TestCase):
    def test_fewer_instructions_same_result(self) -> None:
        expr = sample_expression()
        raw: list[Op] = []
        compile_expr(expr, raw)
        optimized: list[Op] = []
        compile_expr(fold(expr), optimized)
        self.assertEqual(len(raw), 5)
        self.assertEqual(len(optimized), 3)
        for x in range(-5, 6):
            self.assertEqual(run(raw, x), run(optimized, x))


if __name__ == "__main__":
    main()
