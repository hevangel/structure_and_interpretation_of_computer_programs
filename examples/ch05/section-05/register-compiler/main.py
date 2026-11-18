from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any
import unittest


def tokenize(src: str) -> list[str]:
    tokens: list[str] = []
    i = 0
    while i < len(src):
        c = src[i]
        if c.isspace():
            i += 1
        elif c == ";":
            while i < len(src) and src[i] != "\n":
                i += 1
        elif c in "()":
            tokens.append(c)
            i += 1
        else:
            j = i
            while j < len(src) and not src[j].isspace() and src[j] not in "()":
                j += 1
            tokens.append(src[i:j])
            i = j
    return tokens


def parse_program(src: str) -> list[Any]:
    tokens = tokenize(src)
    pos = 0

    def parse_one() -> Any:
        nonlocal pos
        tok = tokens[pos]
        pos += 1
        if tok == "(":
            items = []
            while tokens[pos] != ")":
                items.append(parse_one())
            pos += 1
            return items
        if tok in ("#t", "true"):
            return True
        if tok in ("#f", "false"):
            return False
        try:
            return int(tok)
        except ValueError:
            return tok

    out = []
    while pos < len(tokens):
        out.append(parse_one())
    return out


ALL_REGS = ("env", "proc", "argl", "continue", "val")


@dataclass
class InstructionSequence:
    needs: set[str] = field(default_factory=set)
    modifies: set[str] = field(default_factory=set)
    statements: list[tuple[Any, ...]] = field(default_factory=list)


def append_seqs(*seqs: InstructionSequence) -> InstructionSequence:
    needs: set[str] = set()
    modifies: set[str] = set()
    statements: list[tuple[Any, ...]] = []
    for s in seqs:
        for r in s.needs:
            if r not in modifies:
                needs.add(r)
        modifies.update(s.modifies)
        statements.extend(s.statements)
    return InstructionSequence(needs, modifies, statements)


def preserving(
    regs: list[str], seq1: InstructionSequence, seq2: InstructionSequence
) -> InstructionSequence:
    s1 = InstructionSequence(set(seq1.needs), set(seq1.modifies), list(seq1.statements))
    for reg in reversed(regs):
        if reg in seq2.needs and reg in s1.modifies:
            s1.statements = [("save", reg), *s1.statements, ("restore", reg)]
            s1.needs.add(reg)
            s1.modifies.discard(reg)
    return append_seqs(s1, seq2)


def parallel_seqs(s1: InstructionSequence, s2: InstructionSequence) -> InstructionSequence:
    return InstructionSequence(
        s1.needs | s2.needs,
        s1.modifies | s2.modifies,
        [*s1.statements, *s2.statements],
    )


def tack_on(seq: InstructionSequence, body: InstructionSequence) -> InstructionSequence:
    return InstructionSequence(
        set(seq.needs), set(seq.modifies), [*seq.statements, *body.statements]
    )


class Compiler:
    def __init__(self) -> None:
        self.label_counter = 0

    def make_label(self, prefix: str) -> str:
        self.label_counter += 1
        return f"{prefix}{self.label_counter}"

    def find_variable(self, name: str, ctenv: list[list[str]]) -> tuple[int, int] | None:
        for f_idx, frame in enumerate(ctenv):
            if name in frame:
                return (f_idx, frame.index(name))
        return None

    def compile_linkage(self, linkage: str) -> InstructionSequence:
        if linkage == "next":
            return InstructionSequence()
        if linkage == "return":
            return InstructionSequence({"continue"}, set(), [("goto_reg", "continue")])
        return InstructionSequence(set(), set(), [("goto_label", linkage)])

    def end_with_linkage(self, linkage: str, seq: InstructionSequence) -> InstructionSequence:
        return preserving(["continue"], seq, self.compile_linkage(linkage))

    def compile(
        self, expr: Any, target: str, linkage: str, ctenv: list[list[str]]
    ) -> InstructionSequence:
        if isinstance(expr, (bool, int)):
            return self.end_with_linkage(
                linkage,
                InstructionSequence(set(), {target}, [("assign_const", target, expr)]),
            )
        if isinstance(expr, str):
            addr = self.find_variable(expr, ctenv)
            stmt = (
                ("assign_lexical", target, addr)
                if addr is not None
                else ("assign_lookup", target, expr)
            )
            return self.end_with_linkage(
                linkage, InstructionSequence({"env"}, {target}, [stmt])
            )
        head = expr[0]
        if head == "if":
            return self.compile_if(expr, target, linkage, ctenv)
        if head == "begin":
            return self.compile_sequence(expr[1:], target, linkage, ctenv)
        if head == "lambda":
            return self.compile_lambda(expr, target, linkage, ctenv)
        if head == "define":
            return self.compile_definition(expr, target, linkage, ctenv)
        return self.compile_application(expr, target, linkage, ctenv)

    def compile_if(
        self, expr: list[Any], target: str, linkage: str, ctenv: list[list[str]]
    ) -> InstructionSequence:
        t_lbl = self.make_label("true-branch")
        f_lbl = self.make_label("false-branch")
        after_lbl = self.make_label("after-if")
        c_linkage = after_lbl if linkage == "next" else linkage
        p_code = self.compile(expr[1], "val", "next", ctenv)
        c_code = self.compile(expr[2], target, c_linkage, ctenv)
        a_code = self.compile(expr[3] if len(expr) > 3 else False, target, linkage, ctenv)
        branches = parallel_seqs(
            append_seqs(InstructionSequence(statements=[("label", t_lbl)]), c_code),
            append_seqs(InstructionSequence(statements=[("label", f_lbl)]), a_code),
        )
        return preserving(
            ["env", "continue"],
            p_code,
            append_seqs(
                InstructionSequence(
                    {"val"},
                    set(),
                    [("test_false", "val"), ("branch", f_lbl), ("goto_label", t_lbl)],
                ),
                branches,
                InstructionSequence(statements=[("label", after_lbl)]),
            ),
        )

    def compile_sequence(
        self, exprs: list[Any], target: str, linkage: str, ctenv: list[list[str]]
    ) -> InstructionSequence:
        if len(exprs) == 1:
            return self.compile(exprs[0], target, linkage, ctenv)
        return preserving(
            ["env", "continue"],
            self.compile(exprs[0], target, "next", ctenv),
            self.compile_sequence(exprs[1:], target, linkage, ctenv),
        )

    def compile_definition(
        self, expr: list[Any], target: str, linkage: str, ctenv: list[list[str]]
    ) -> InstructionSequence:
        if isinstance(expr[1], list):
            name, *params = expr[1]
            val_expr: Any = ["lambda", params, *expr[2:]]
        else:
            name, val_expr = expr[1], expr[2]
        get_val = self.compile(val_expr, "val", "next", ctenv)
        return self.end_with_linkage(
            linkage,
            preserving(
                ["env"],
                get_val,
                InstructionSequence(
                    {"env", "val"},
                    {target},
                    [("define_var", name, "val"), ("assign_const", target, True)],
                ),
            ),
        )

    def compile_lambda(
        self, expr: list[Any], target: str, linkage: str, ctenv: list[list[str]]
    ) -> InstructionSequence:
        proc_entry = self.make_label("entry")
        after_lbl = self.make_label("after-lambda")
        lam_link = after_lbl if linkage == "next" else linkage
        params = list(expr[1])
        body = self.compile_sequence(expr[2:], "val", "return", [params, *ctenv])
        header = self.end_with_linkage(
            lam_link,
            InstructionSequence(
                {"env"}, {target}, [("assign_make_proc", target, proc_entry)]
            ),
        )
        body_seq = append_seqs(
            InstructionSequence(
                {"proc", "argl"},
                {"env"},
                [
                    ("label", proc_entry),
                    ("assign_proc_env", "env", "proc"),
                    ("extend_env", "env", params, "argl", "env"),
                ],
            ),
            body,
        )
        return append_seqs(
            tack_on(header, body_seq),
            InstructionSequence(statements=[("label", after_lbl)]),
        )

    def construct_arglist(
        self, operands: list[Any], ctenv: list[list[str]]
    ) -> InstructionSequence:
        if not operands:
            return InstructionSequence(set(), {"argl"}, [("assign_empty_argl",)])
        rev_codes = [self.compile(op, "val", "next", ctenv) for op in reversed(operands)]
        seq = append_seqs(
            rev_codes[0],
            InstructionSequence({"val"}, {"argl"}, [("assign_list_arg", "argl", "val")]),
        )
        for next_code in rev_codes[1:]:
            step = preserving(
                ["argl"],
                next_code,
                InstructionSequence(
                    {"val", "argl"}, {"argl"}, [("assign_cons_arg", "argl", "val", "argl")]
                ),
            )
            seq = preserving(["env"], seq, step)
        return seq

    def compile_application(
        self, expr: list[Any], target: str, linkage: str, ctenv: list[list[str]]
    ) -> InstructionSequence:
        proc_code = self.compile(expr[0], "proc", "next", ctenv)
        arg_code = self.construct_arglist(expr[1:], ctenv)
        call_code = self.compile_call(target, linkage)
        return preserving(
            ["env", "continue"],
            proc_code,
            preserving(["proc", "continue"], arg_code, call_code),
        )

    def compile_call(self, target: str, linkage: str) -> InstructionSequence:
        prim_lbl = self.make_label("primitive-branch")
        comp_lbl = self.make_label("compiled-branch")
        after_lbl = self.make_label("after-call")
        comp_link = after_lbl if linkage == "next" else linkage
        prim_seq = append_seqs(
            InstructionSequence(statements=[("label", prim_lbl)]),
            self.end_with_linkage(
                linkage,
                InstructionSequence(
                    {"proc", "argl"}, {target}, [("apply_prim", target, "proc", "argl")]
                ),
            ),
        )
        if comp_link == "return":
            appl_seq = InstructionSequence(
                {"proc", "continue"},
                set(ALL_REGS),
                [("assign_proc_entry", "val", "proc"), ("goto_reg", "val")],
            )
        else:
            appl_seq = InstructionSequence(
                {"proc"},
                set(ALL_REGS),
                [
                    ("assign_label", "continue", comp_link),
                    ("assign_proc_entry", "val", "proc"),
                    ("goto_reg", "val"),
                ],
            )
        comp_seq = append_seqs(
            InstructionSequence(statements=[("label", comp_lbl)]), appl_seq
        )
        return append_seqs(
            InstructionSequence(
                {"proc"},
                set(),
                [("test_prim", "proc"), ("branch", prim_lbl), ("goto_label", comp_lbl)],
            ),
            parallel_seqs(comp_seq, prim_seq),
            InstructionSequence(statements=[("label", after_lbl)]),
        )


@dataclass
class Frame:
    lexical_slots: list[Any]
    named: dict[str, Any]
    parent: Frame | None = None


@dataclass
class CompiledProc:
    entry: int
    env: Frame


@dataclass(frozen=True)
class Primitive:
    name: str


class CompiledVM:
    def __init__(self) -> None:
        self.global_env = Frame(
            [], {op: Primitive(op) for op in ("+", "-", "*", "=", "<", ">")}, None
        )
        self.code: list[tuple[Any, ...]] = []
        self.labels: dict[str, int] = {}
        self.total_pushes = 0
        self.current_depth = 0
        self.max_depth = 0

    def reset_stats(self) -> None:
        self.total_pushes = 0
        self.current_depth = 0
        self.max_depth = 0

    def run(self, seq: InstructionSequence) -> Any:
        start_pc = len(self.code)
        for stmt in seq.statements:
            if stmt[0] == "label":
                self.labels[stmt[1]] = len(self.code)
            else:
                self.code.append(stmt)
        end_pc = len(self.code)
        regs: dict[str, Any] = {
            "env": self.global_env,
            "val": False,
            "proc": False,
            "argl": [],
            "continue": end_pc,
        }
        stack: list[Any] = []
        flag = False
        pc = start_pc
        while pc != end_pc:
            ins = self.code[pc]
            op = ins[0]
            if op == "assign_const":
                regs[ins[1]] = ins[2]
                pc += 1
            elif op == "assign_label":
                regs[ins[1]] = self.labels[ins[2]]
                pc += 1
            elif op == "assign_lookup":
                cur: Frame | None = regs["env"]
                while cur is not None:
                    if ins[2] in cur.named:
                        regs[ins[1]] = cur.named[ins[2]]
                        break
                    cur = cur.parent
                pc += 1
            elif op == "assign_lexical":
                f_idx, d_idx = ins[2]
                cur = regs["env"]
                for _ in range(f_idx):
                    assert cur.parent is not None
                    cur = cur.parent
                regs[ins[1]] = cur.lexical_slots[d_idx]
                pc += 1
            elif op == "assign_make_proc":
                regs[ins[1]] = CompiledProc(self.labels[ins[2]], regs["env"])
                pc += 1
            elif op == "assign_empty_argl":
                regs["argl"] = []
                pc += 1
            elif op == "assign_list_arg":
                regs[ins[1]] = [regs[ins[2]]]
                pc += 1
            elif op == "assign_cons_arg":
                regs[ins[1]] = [regs[ins[2]], *regs[ins[3]]]
                pc += 1
            elif op == "assign_proc_env":
                regs[ins[1]] = regs[ins[2]].env
                pc += 1
            elif op == "extend_env":
                args = regs[ins[3]]
                params = ins[2]
                regs[ins[1]] = Frame(list(args), dict(zip(params, args)), regs[ins[4]])
                pc += 1
            elif op == "assign_proc_entry":
                regs[ins[1]] = regs[ins[2]].entry
                pc += 1
            elif op == "define_var":
                regs["env"].named[ins[1]] = regs[ins[2]]
                pc += 1
            elif op == "test_prim":
                flag = isinstance(regs[ins[1]], Primitive)
                pc += 1
            elif op == "test_false":
                flag = regs[ins[1]] is False
                pc += 1
            elif op == "apply_prim":
                p = regs[ins[2]].name
                a = regs[ins[3]]
                if p == "+":
                    regs[ins[1]] = sum(a)
                elif p == "*":
                    r = 1
                    for x in a:
                        r *= x
                    regs[ins[1]] = r
                elif p == "-":
                    regs[ins[1]] = a[0] - a[1]
                elif p == "=":
                    regs[ins[1]] = a[0] == a[1]
                elif p == "<":
                    regs[ins[1]] = a[0] < a[1]
                elif p == ">":
                    regs[ins[1]] = a[0] > a[1]
                pc += 1
            elif op == "branch":
                pc = self.labels[ins[1]] if flag else pc + 1
            elif op == "goto_label":
                pc = self.labels[ins[1]]
            elif op == "goto_reg":
                pc = regs[ins[1]]
            elif op == "save":
                self.total_pushes += 1
                self.current_depth += 1
                if self.current_depth > self.max_depth:
                    self.max_depth = self.current_depth
                stack.append(regs[ins[1]])
                pc += 1
            elif op == "restore":
                self.current_depth -= 1
                regs[ins[1]] = stack.pop()
                pc += 1
        return regs["val"]


class RegisterCompilerTests(unittest.TestCase):
    def test_preserving_elides_unneeded_saves_and_lexical_addressing_works(self) -> None:
        c = Compiler()
        seq = c.compile(parse_program("(+ 10 20)")[0], "val", "next", [])
        self.assertEqual(sum(1 for s in seq.statements if s[0] == "save"), 0)
        vm = CompiledVM()
        self.assertEqual(vm.run(seq), 30)

        lam_seq = c.compile(
            parse_program("(lambda (x y) ((lambda (z) (+ x z)) y))")[0], "val", "next", []
        )
        self.assertTrue(
            any(s == ("assign_lexical", "val", (1, 0)) for s in lam_seq.statements)
        )

    def test_compiled_factorial_outperforms_eceval(self) -> None:
        c = Compiler()
        vm = CompiledVM()
        vm.run(
            c.compile_sequence(
                parse_program(
                    "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))"
                ),
                "val",
                "next",
                [],
            )
        )
        vm.reset_stats()
        f6 = vm.run(c.compile(parse_program("(factorial 6)")[0], "val", "next", []))
        self.assertEqual(f6, 720)
        self.assertEqual(vm.total_pushes, 32)
        self.assertEqual(vm.max_depth, 17)


if __name__ == "__main__":
    c = Compiler()
    vm = CompiledVM()
    vm.run(
        c.compile_sequence(
            parse_program(
                "(define (factorial n) (if (= n 1) 1 (* (factorial (- n 1)) n)))"
            ),
            "val",
            "next",
            [],
        )
    )
    vm.reset_stats()
    res = vm.run(c.compile(parse_program("(factorial 6)")[0], "val", "next", []))
    print(
        f"compiled factorial(6) = {res}, pushes = {vm.total_pushes}, max_depth = {vm.max_depth}"
    )
