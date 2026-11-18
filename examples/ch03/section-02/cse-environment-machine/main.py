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
class Assign:
    name: str
    rhs: object


@dataclass(frozen=True)
class Declare:
    name: str
    rhs: object


@dataclass(frozen=True)
class Lambda:
    params: tuple[str, ...]
    body: object


@dataclass(frozen=True)
class Call:
    callee: object
    args: tuple[object, ...]


@dataclass(frozen=True)
class Seq:
    stmts: tuple[object, ...]


@dataclass
class Frame:
    bindings: dict[str, object]
    parent: int | None


@dataclass(frozen=True)
class Closure:
    params: tuple[str, ...]
    body: object
    env: int


class CseMachine:
    def __init__(self) -> None:
        self.frames: list[Frame] = [Frame({}, None)]
        self.control: list[tuple[str, object]] = []
        self.stash: list[object] = []
        self.env: int = 0
        self.trace: list[tuple[int, int, int, str]] = []

    def extend_env(self, parent: int, bindings: dict[str, object]) -> int:
        self.frames.append(Frame(dict(bindings), parent))
        return len(self.frames) - 1

    def lookup(self, env: int, name: str) -> object:
        cur: int | None = env
        while cur is not None:
            if name in self.frames[cur].bindings:
                return self.frames[cur].bindings[name]
            cur = self.frames[cur].parent
        raise KeyError(f"unbound name: {name}")

    def assign(self, env: int, name: str, val: object) -> None:
        cur: int | None = env
        while cur is not None:
            if name in self.frames[cur].bindings:
                self.frames[cur].bindings[name] = val
                return
            cur = self.frames[cur].parent
        raise KeyError(f"unbound name: {name}")

    def step(self) -> bool:
        if not self.control:
            return False
        op, arg = self.control.pop()
        if op == "EVAL":
            if isinstance(arg, Num):
                self.stash.append(arg.value)
                action = f"push {arg.value}"
            elif isinstance(arg, Var):
                self.stash.append(self.lookup(self.env, arg.name))
                action = f"lookup {arg.name}"
            elif isinstance(arg, Add):
                self.control.extend([("ADD", None), ("EVAL", arg.right), ("EVAL", arg.left)])
                action = "expand add"
            elif isinstance(arg, Assign):
                self.control.extend([("ASSIGN", arg.name), ("EVAL", arg.rhs)])
                action = f"schedule assign {arg.name}"
            elif isinstance(arg, Declare):
                self.control.extend([("DECLARE", arg.name), ("EVAL", arg.rhs)])
                action = f"schedule declare {arg.name}"
            elif isinstance(arg, Lambda):
                self.stash.append(Closure(arg.params, arg.body, self.env))
                action = f"make closure in env#{self.env}"
            elif isinstance(arg, Call):
                self.control.append(("CALL", len(arg.args)))
                for a in reversed(arg.args):
                    self.control.append(("EVAL", a))
                self.control.append(("EVAL", arg.callee))
                action = f"schedule call ({len(arg.args)} args)"
            elif isinstance(arg, Seq):
                for idx in range(len(arg.stmts) - 1, -1, -1):
                    if idx + 1 < len(arg.stmts):
                        self.control.append(("POP", None))
                    self.control.append(("EVAL", arg.stmts[idx]))
                action = "expand sequence"
            else:
                raise TypeError(arg)
        elif op == "ADD":
            b = int(self.stash.pop())  # type: ignore[arg-type]
            a = int(self.stash.pop())  # type: ignore[arg-type]
            self.stash.append(a + b)
            action = "apply +"
        elif op == "ASSIGN":
            self.assign(self.env, str(arg), self.stash[-1])
            action = f"assign {arg}"
        elif op == "DECLARE":
            self.frames[self.env].bindings[str(arg)] = self.stash[-1]
            action = f"declare {arg} in env#{self.env}"
        elif op == "CALL":
            argc = int(arg)  # type: ignore[arg-type]
            args = [self.stash.pop() for _ in range(argc)][::-1]
            callee = self.stash.pop()
            assert isinstance(callee, Closure)
            saved = self.env
            call_env = self.extend_env(callee.env, dict(zip(callee.params, args)))
            self.control.append(("RESTORE_ENV", saved))
            self.control.append(("EVAL", callee.body))
            self.env = call_env
            action = f"enter closure env#{call_env}"
        elif op == "RESTORE_ENV":
            self.env = int(arg)  # type: ignore[arg-type]
            action = f"restore env#{self.env}"
        elif op == "POP":
            self.stash.pop()
            action = "pop discarded"
        else:
            raise ValueError(op)
        self.trace.append((len(self.control), len(self.stash), self.env, action))
        return True

    def eval(self, stmt: object) -> object:
        self.control.append(("EVAL", stmt))
        while self.step():
            pass
        return self.stash.pop()


def make_withdraw_program() -> Seq:
    return Seq(
        (
            Declare(
                "make_withdraw",
                Lambda(
                    ("balance",),
                    Lambda(
                        ("amount",),
                        Seq((Assign("balance", Add(Var("balance"), Var("amount"))), Var("balance"))),
                    ),
                ),
            ),
            Declare("w1", Call(Var("make_withdraw"), (Num(100),))),
            Declare("w2", Call(Var("make_withdraw"), (Num(100),))),
            Call(Var("w1"), (Num(-25),)),
            Call(Var("w1"), (Num(-10),)),
            Add(Call(Var("w2"), (Num(-5),)), Call(Var("w1"), (Num(0),))),
        )
    )


class CseMachineTest(unittest.TestCase):
    def test_independent_closures(self) -> None:
        m = CseMachine()
        self.assertEqual(m.eval(make_withdraw_program()), 160)
        self.assertGreaterEqual(len(m.frames), 5)

    def test_shared_outer_frame_mutation(self) -> None:
        m = CseMachine()
        prog = Seq(
            (
                Declare("shared", Num(10)),
                Declare("inc", Lambda((), Assign("shared", Add(Var("shared"), Num(1))))),
                Declare("read", Lambda((), Var("shared"))),
                Call(Var("inc"), ()),
                Call(Var("inc"), ()),
                Call(Var("read"), ()),
            )
        )
        self.assertEqual(m.eval(prog), 12)


if __name__ == "__main__":
    m = CseMachine()
    print(f"CSE result = {m.eval(make_withdraw_program())}, steps = {len(m.trace)}")
