from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Value:
    n: int


@dataclass(frozen=True)
class Add:
    lhs: Expr
    rhs: Expr


@dataclass(frozen=True)
class Mul:
    lhs: Expr
    rhs: Expr


Expr = Value | Add | Mul


@dataclass(frozen=True)
class Push:
    n: int


@dataclass(frozen=True)
class InstrAdd:
    pass


@dataclass(frozen=True)
class InstrMul:
    pass


Instr = Push | InstrAdd | InstrMul


def compile_expr(expr: Expr, out: list[Instr]) -> None:
    if isinstance(expr, Value):
        out.append(Push(expr.n))
    elif isinstance(expr, Add):
        compile_expr(expr.lhs, out)
        compile_expr(expr.rhs, out)
        out.append(InstrAdd())
    elif isinstance(expr, Mul):
        compile_expr(expr.lhs, out)
        compile_expr(expr.rhs, out)
        out.append(InstrMul())


def run(program: list[Instr]) -> int:
    stack: list[int] = []
    for ins in program:
        if isinstance(ins, Push):
            stack.append(ins.n)
        elif isinstance(ins, (InstrAdd, InstrMul)):
            if len(stack) < 2:
                raise ValueError("stack underflow")
            rhs = stack.pop()
            lhs = stack.pop()
            stack.append(lhs + rhs if isinstance(ins, InstrAdd) else lhs * rhs)
    if len(stack) != 1:
        raise ValueError("program did not leave one result")
    return stack.pop()


def main() -> None:
    expr = Add(Value(2), Mul(Value(3), Value(4)))
    program: list[Instr] = []
    compile_expr(expr, program)
    print(f"{program} -> {run(program)}")


class TestBytecodeCompiler(unittest.TestCase):
    def test_generated_code(self) -> None:
        expr = Add(Value(2), Mul(Value(3), Value(4)))
        program: list[Instr] = []
        compile_expr(expr, program)
        self.assertEqual(run(program), 14)
        with self.assertRaises(ValueError):
            run([InstrAdd()])


if __name__ == "__main__":
    main()
