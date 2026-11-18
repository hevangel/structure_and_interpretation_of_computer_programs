from __future__ import annotations

from dataclasses import dataclass, field
from typing import Callable
import unittest


@dataclass
class StackStats:
    total_pushes: int = 0
    current_depth: int = 0
    max_depth: int = 0


def parse_source(tok: str) -> tuple[str, int | str]:
    if tok.startswith("const(") and tok.endswith(")"):
        return ("const", int(tok[6:-1].strip()))
    if tok.startswith("reg(") and tok.endswith(")"):
        return ("reg", tok[4:-1].strip())
    if tok.startswith("label(") and tok.endswith(")"):
        return ("label", tok[6:-1].strip())
    raise ValueError(f"invalid operand: {tok}")


def parse_op_call(expr: str) -> tuple[str, list[tuple[str, int | str]]]:
    assert expr.startswith("op(") and expr.endswith(")")
    parts = expr[3:-1].split()
    return parts[0], [parse_source(p) for p in parts[1:]]


class RegisterMachine:
    def __init__(self, reg_names: list[str]) -> None:
        self.registers: dict[str, int | tuple[str, int] | str] = {r: 0 for r in reg_names}
        self.stack: list[int | tuple[str, int] | str] = []
        self.stats = StackStats()
        self.flag = False
        self.pc = 0
        self.labels: dict[str, int] = {}
        self.instructions: list[tuple[str, ...]] = []
        self.ops: dict[str, Callable[..., int | str]] = {
            "=": lambda a, b: int(a == b),
            "-": lambda a, b: int(a) - int(b),
            "+": lambda a, b: int(a) + int(b),
            "*": lambda a, b: int(a) * int(b),
            "<": lambda a, b: int(int(a) < int(b)),
        }
        self.insts_executed = 0

    def assemble(self, controller_text: str) -> None:
        self.instructions.clear()
        self.labels.clear()
        for raw in controller_text.splitlines():
            line = raw.split(";", 1)[0].strip()
            if not line:
                continue
            if line.endswith(":"):
                self.labels[line[:-1].strip()] = len(self.instructions)
                continue
            head, _, rest = line.partition(" ")
            rest = rest.strip()
            if head == "assign":
                reg, _, rhs = rest.partition(" ")
                rhs = rhs.strip()
                if rhs.startswith("op("):
                    op, args = parse_op_call(rhs)
                    self.instructions.append(("assign_op", reg.strip(), op, args))
                else:
                    self.instructions.append(("assign_val", reg.strip(), parse_source(rhs)))
            elif head == "test":
                op, args = parse_op_call(rest)
                self.instructions.append(("test", op, args))
            elif head == "branch":
                kind, lbl = parse_source(rest)
                assert kind == "label"
                self.instructions.append(("branch", str(lbl)))
            elif head == "goto":
                self.instructions.append(("goto", parse_source(rest)))
            elif head in ("save", "restore"):
                self.instructions.append((head, rest))
            elif head == "perform":
                op, args = parse_op_call(rest)
                self.instructions.append(("perform", op, args))
            else:
                raise ValueError(f"unknown instruction: {line}")

    def eval_source(self, src: tuple[str, int | str]) -> int | tuple[str, int] | str:
        kind, val = src
        if kind == "const":
            return int(val)
        if kind == "reg":
            return self.registers[str(val)]
        return ("label", self.labels[str(val)])

    def start(self, max_steps: int = 100_000) -> None:
        self.pc = 0
        self.stack.clear()
        self.stats = StackStats()
        self.insts_executed = 0
        while self.pc < len(self.instructions):
            if self.insts_executed >= max_steps:
                raise RuntimeError("step budget exceeded")
            instr = self.instructions[self.pc]
            self.insts_executed += 1
            tag = instr[0]
            if tag == "assign_val":
                self.registers[instr[1]] = self.eval_source(instr[2])
                self.pc += 1
            elif tag == "assign_op":
                args = [self.eval_source(a) for a in instr[3]]
                self.registers[instr[1]] = self.ops[instr[2]](*args)
                self.pc += 1
            elif tag == "test":
                args = [self.eval_source(a) for a in instr[2]]
                self.flag = bool(self.ops[instr[1]](*args))
                self.pc += 1
            elif tag == "branch":
                self.pc = self.labels[instr[1]] if self.flag else self.pc + 1
            elif tag == "goto":
                kind, val = instr[1]
                if kind == "label":
                    self.pc = self.labels[str(val)]
                else:
                    target = self.registers[str(val)]
                    assert isinstance(target, tuple) and target[0] == "label"
                    self.pc = target[1]
            elif tag == "save":
                self.stack.append(self.registers[instr[1]])
                self.stats.total_pushes += 1
                self.stats.current_depth += 1
                if self.stats.current_depth > self.stats.max_depth:
                    self.stats.max_depth = self.stats.current_depth
                self.pc += 1
            elif tag == "restore":
                self.registers[instr[1]] = self.stack.pop()
                self.stats.current_depth -= 1
                self.pc += 1
            elif tag == "perform":
                args = [self.eval_source(a) for a in instr[2]]
                self.ops[instr[1]](*args)
                self.pc += 1


FACT_CONTROLLER = """
  assign continue label(fact-done)
fact-loop:
  test op(= reg(n) const(1))
  branch label(base-case)
  save continue
  save n
  assign n op(- reg(n) const(1))
  assign continue label(after-fact)
  goto label(fact-loop)
after-fact:
  restore n
  restore continue
  assign val op(* reg(n) reg(val))
  goto reg(continue)
base-case:
  assign val const(1)
  goto reg(continue)
fact-done:
"""

FIB_CONTROLLER = """
  assign continue label(fib-done)
fib-loop:
  test op(< reg(n) const(2))
  branch label(immediate-answer)
  save continue
  assign continue label(afterfib-n-1)
  save n
  assign n op(- reg(n) const(1))
  goto label(fib-loop)
afterfib-n-1:
  restore n
  restore continue
  assign n op(- reg(n) const(2))
  save continue
  assign continue label(afterfib-n-2)
  save val
  goto label(fib-loop)
afterfib-n-2:
  assign n reg(val)
  restore val
  restore continue
  assign val op(+ reg(val) reg(n))
  goto reg(continue)
immediate-answer:
  assign val reg(n)
  goto reg(continue)
fib-done:
"""


class RegisterSimulatorTests(unittest.TestCase):
    def test_factorial_stack_formulas(self) -> None:
        m = RegisterMachine(["n", "val", "continue"])
        m.assemble(FACT_CONTROLLER)
        fact = 1
        for n in range(1, 11):
            fact *= n
            m.registers["n"] = n
            m.start()
            self.assertEqual(m.registers["val"], fact)
            self.assertEqual(m.stats.total_pushes, 2 * (n - 1))
            self.assertEqual(m.stats.max_depth, 2 * (n - 1))

    def test_fibonacci_controller(self) -> None:
        m = RegisterMachine(["n", "val", "continue"])
        m.assemble(FIB_CONTROLLER)
        expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
        for n, exp in enumerate(expected):
            m.registers["n"] = n
            m.start()
            self.assertEqual(m.registers["val"], exp)


if __name__ == "__main__":
    m = RegisterMachine(["n", "val", "continue"])
    m.assemble(FACT_CONTROLLER)
    m.registers["n"] = 6
    m.start()
    print(
        f"fact(6) = {m.registers['val']}, pushes = {m.stats.total_pushes}, "
        f"max_depth = {m.stats.max_depth}, insts = {m.insts_executed}"
    )
