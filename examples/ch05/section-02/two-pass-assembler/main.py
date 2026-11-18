from __future__ import annotations

from dataclasses import dataclass
import unittest


@dataclass(frozen=True)
class Label:
    name: str


@dataclass(frozen=True)
class SrcLoad:
    reg: int
    val: int


@dataclass(frozen=True)
class SrcSub:
    a: int
    b: int


@dataclass(frozen=True)
class SrcJump:
    label: str


@dataclass(frozen=True)
class SrcJumpIfZero:
    reg: int
    label: str


@dataclass(frozen=True)
class SrcHalt:
    pass


Source = Label | SrcLoad | SrcSub | SrcJump | SrcJumpIfZero | SrcHalt


@dataclass(frozen=True)
class OpLoad:
    reg: int
    val: int


@dataclass(frozen=True)
class OpSub:
    a: int
    b: int


@dataclass(frozen=True)
class OpJump:
    addr: int


@dataclass(frozen=True)
class OpJumpIfZero:
    reg: int
    addr: int


@dataclass(frozen=True)
class OpHalt:
    pass


Op = OpLoad | OpSub | OpJump | OpJumpIfZero | OpHalt


def assemble(source: list[Source]) -> list[Op]:
    labels: dict[str, int] = {}
    address = 0
    for item in source:
        if isinstance(item, Label):
            if item.name in labels:
                raise ValueError(f"duplicate label {item.name}")
            labels[item.name] = address
        else:
            address += 1

    def lookup(name: str) -> int:
        if name not in labels:
            raise ValueError(f"unknown label {name}")
        return labels[name]

    program: list[Op] = []
    for item in source:
        if isinstance(item, Label):
            continue
        if isinstance(item, SrcLoad):
            program.append(OpLoad(item.reg, item.val))
        elif isinstance(item, SrcSub):
            program.append(OpSub(item.a, item.b))
        elif isinstance(item, SrcJump):
            program.append(OpJump(lookup(item.label)))
        elif isinstance(item, SrcJumpIfZero):
            program.append(OpJumpIfZero(item.reg, lookup(item.label)))
        elif isinstance(item, SrcHalt):
            program.append(OpHalt())
    return program


def execute(code: list[Op], limit: int) -> list[int]:
    regs = [0, 0]
    pc = 0
    for _ in range(limit):
        if not (0 <= pc < len(code)):
            raise ValueError("invalid pc")
        op = code[pc]
        if isinstance(op, OpLoad):
            if not (0 <= op.reg < len(regs)):
                raise ValueError("invalid register")
            regs[op.reg] = op.val
        elif isinstance(op, OpSub):
            if not (0 <= op.b < len(regs)):
                raise ValueError("invalid source register")
            if not (0 <= op.a < len(regs)):
                raise ValueError("invalid destination register")
            regs[op.a] -= regs[op.b]
        elif isinstance(op, OpJump):
            pc = op.addr
            continue
        elif isinstance(op, OpJumpIfZero):
            if not (0 <= op.reg < len(regs)):
                raise ValueError("invalid register")
            if regs[op.reg] == 0:
                pc = op.addr
                continue
        elif isinstance(op, OpHalt):
            return regs
        pc += 1
    raise RuntimeError("step limit exceeded")


def sample_program() -> list[Source]:
    return [
        SrcLoad(0, 5),
        SrcLoad(1, 1),
        Label("loop"),
        SrcJumpIfZero(0, "done"),
        SrcSub(0, 1),
        SrcJump("loop"),
        Label("done"),
        SrcHalt(),
    ]


def main() -> None:
    print(f"registers: {execute(assemble(sample_program()), 100)}")


class TestTwoPassAssembler(unittest.TestCase):
    def test_symbols_resolve_and_program_runs(self) -> None:
        code = assemble(sample_program())
        self.assertEqual(len(code), 6)
        self.assertEqual(execute(code, 100), [0, 1])
        with self.assertRaises(RuntimeError):
            execute(code, 1)
        with self.assertRaises(ValueError):
            assemble([SrcJump("missing")])
        with self.assertRaises(ValueError):
            assemble([Label("a"), Label("a")])


if __name__ == "__main__":
    main()
