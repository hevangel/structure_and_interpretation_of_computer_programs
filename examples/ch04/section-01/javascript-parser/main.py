from __future__ import annotations

from dataclasses import dataclass
from typing import Any
import unittest


def lex(src: str) -> list[tuple[str, Any]]:
    tokens: list[tuple[str, Any]] = []
    i = 0
    while i < len(src):
        c = src[i]
        if c.isspace():
            i += 1
        elif c.isdigit():
            j = i
            while j < len(src) and src[j].isdigit():
                j += 1
            tokens.append(("NUM", int(src[i:j])))
            i = j
        elif c.isalpha() or c == "_":
            j = i
            while j < len(src) and (src[j].isalnum() or src[j] == "_"):
                j += 1
            word = src[i:j]
            kw = word if word in {"const", "let", "function", "return", "if", "else"} else "IDENT"
            tokens.append((kw, word))
            i = j
        elif src.startswith("==", i):
            tokens.append(("==", "=="))
            i += 2
        elif c in "+-*=;{}(),":
            tokens.append((c, c))
            i += 1
        else:
            raise ValueError(f"unexpected char: {c}")
    return tokens


class Parser:
    def __init__(self, tokens: list[tuple[str, Any]]) -> None:
        self.tokens = tokens
        self.pos = 0

    def peek(self) -> str | None:
        return self.tokens[self.pos][0] if self.pos < len(self.tokens) else None

    def next(self) -> tuple[str, Any]:
        t = self.tokens[self.pos]
        self.pos += 1
        return t

    def expect(self, kind: str) -> Any:
        k, v = self.next()
        assert k == kind, f"expected {kind}, got {k}"
        return v

    def parse_program(self) -> list[Any]:
        stmts = []
        while self.peek() is not None:
            stmts.append(self.parse_stmt())
        return stmts

    def parse_block(self) -> list[Any]:
        self.expect("{")
        stmts = []
        while self.peek() != "}":
            stmts.append(self.parse_stmt())
        self.expect("}")
        return stmts

    def parse_stmt(self) -> Any:
        pk = self.peek()
        if pk in ("const", "let"):
            kind = self.next()[0]
            name = self.expect("IDENT")
            self.expect("=")
            rhs = self.parse_expr()
            self.expect(";")
            return (kind, name, rhs)
        if pk == "function":
            self.next()
            name = self.expect("IDENT")
            self.expect("(")
            params = []
            while self.peek() != ")":
                params.append(self.expect("IDENT"))
                if self.peek() == ",":
                    self.next()
            self.expect(")")
            return ("function", name, params, self.parse_block())
        if pk == "return":
            self.next()
            e = self.parse_expr()
            self.expect(";")
            return ("return", e)
        if pk == "if":
            self.next()
            self.expect("(")
            cond = self.parse_expr()
            self.expect(")")
            then_blk = self.parse_block()
            else_blk = None
            if self.peek() == "else":
                self.next()
                else_blk = self.parse_block()
            return ("if", cond, then_blk, else_blk)
        if pk == "{":
            return ("block", self.parse_block())
        e = self.parse_expr()
        self.expect(";")
        return ("expr", e)

    def parse_expr(self) -> Any:
        lhs = self.parse_eq()
        if self.peek() == "=" and isinstance(lhs, tuple) and lhs[0] == "var":
            self.next()
            return ("assign", lhs[1], self.parse_expr())
        return lhs

    def parse_eq(self) -> Any:
        lhs = self.parse_add()
        while self.peek() == "==":
            self.next()
            lhs = ("binop", "==", lhs, self.parse_add())
        return lhs

    def parse_add(self) -> Any:
        lhs = self.parse_mul()
        while self.peek() in ("+", "-"):
            op = self.next()[0]
            lhs = ("binop", op, lhs, self.parse_mul())
        return lhs

    def parse_mul(self) -> Any:
        lhs = self.parse_call()
        while self.peek() == "*":
            self.next()
            lhs = ("binop", "*", lhs, self.parse_call())
        return lhs

    def parse_call(self) -> Any:
        expr = self.parse_atom()
        while self.peek() == "(":
            self.next()
            args = []
            while self.peek() != ")":
                args.append(self.parse_expr())
                if self.peek() == ",":
                    self.next()
            self.expect(")")
            expr = ("call", expr, args)
        return expr

    def parse_atom(self) -> Any:
        kind, val = self.next()
        if kind == "NUM":
            return ("num", val)
        if kind == "IDENT":
            return ("var", val)
        if kind == "(":
            inner = self.parse_expr()
            self.expect(")")
            return inner
        raise ValueError(f"unexpected token {kind}")


@dataclass
class Binding:
    val: Any
    is_const: bool
    initialized: bool


@dataclass
class Frame:
    bindings: dict[str, Binding]
    parent: Frame | None = None


@dataclass(frozen=True)
class ReturnValue:
    value: Any


@dataclass(frozen=True)
class JsClosure:
    params: list[str]
    body: list[Any]
    env: Frame


def extend_block(parent: Frame | None, stmts: list[Any]) -> Frame:
    bindings: dict[str, Binding] = {}
    for s in stmts:
        if s[0] == "const":
            bindings[s[1]] = Binding(None, True, False)
        elif s[0] in ("let", "function"):
            bindings[s[1]] = Binding(None, False, False)
    return Frame(bindings, parent)


def lookup(env: Frame, name: str) -> Any:
    cur: Frame | None = env
    while cur is not None:
        if name in cur.bindings:
            slot = cur.bindings[name]
            if not slot.initialized:
                raise RuntimeError(f"temporal dead zone: {name}")
            return slot.val
        cur = cur.parent
    raise KeyError(name)


def assign(env: Frame, name: str, val: Any) -> None:
    cur: Frame | None = env
    while cur is not None:
        if name in cur.bindings:
            slot = cur.bindings[name]
            if slot.is_const and slot.initialized:
                raise RuntimeError(f"cannot reassign const {name}")
            slot.val = val
            slot.initialized = True
            return
        cur = cur.parent
    raise KeyError(name)


def eval_expr(expr: Any, env: Frame) -> Any:
    tag = expr[0]
    if tag == "num":
        return expr[1]
    if tag == "var":
        return lookup(env, expr[1])
    if tag == "assign":
        v = eval_expr(expr[2], env)
        assign(env, expr[1], v)
        return v
    if tag == "binop":
        a, b = eval_expr(expr[2], env), eval_expr(expr[3], env)
        if expr[1] == "+":
            return a + b
        if expr[1] == "-":
            return a - b
        if expr[1] == "*":
            return a * b
        return a == b
    if tag == "call":
        fn = eval_expr(expr[1], env)
        args = [eval_expr(a, env) for a in expr[2]]
        assert isinstance(fn, JsClosure)
        call_env = Frame({p: Binding(a, False, True) for p, a in zip(fn.params, args)}, fn.env)
        blk_env = extend_block(call_env, fn.body)
        res = eval_seq(fn.body, blk_env)
        return res.value if isinstance(res, ReturnValue) else None
    raise ValueError(tag)


def eval_seq(stmts: list[Any], env: Frame) -> Any:
    last = None
    for s in stmts:
        res = eval_stmt(s, env)
        if isinstance(res, ReturnValue):
            return res
        last = res
    return last


def eval_stmt(stmt: Any, env: Frame) -> Any:
    tag = stmt[0]
    if tag in ("const", "let"):
        val = eval_expr(stmt[2], env)
        slot = env.bindings[stmt[1]]
        slot.val = val
        slot.initialized = True
        return val
    if tag == "function":
        fn = JsClosure(stmt[2], stmt[3], env)
        slot = env.bindings[stmt[1]]
        slot.val = fn
        slot.initialized = True
        return fn
    if tag == "return":
        return ReturnValue(eval_expr(stmt[1], env))
    if tag == "if":
        if eval_expr(stmt[1], env):
            return eval_seq(stmt[2], extend_block(env, stmt[2]))
        if stmt[3] is not None:
            return eval_seq(stmt[3], extend_block(env, stmt[3]))
        return None
    if tag == "block":
        return eval_seq(stmt[1], extend_block(env, stmt[1]))
    if tag == "expr":
        return eval_expr(stmt[1], env)
    raise ValueError(tag)


def eval_js(src: str) -> Any:
    stmts = Parser(lex(src)).parse_program()
    res = eval_seq(stmts, extend_block(None, stmts))
    return res.value if isinstance(res, ReturnValue) else res


class JavaScriptStatementEvaluatorTest(unittest.TestCase):
    def test_return_unwinding_and_block_scoping(self) -> None:
        src = """
            const x = 100;
            function pick(flag) {
                let acc = 1;
                {
                    const x = 7;
                    if (flag == 1) { return x + acc; }
                    acc = acc + 20;
                }
                return x + acc;
            }
            pick(1) * 1000 + pick(0);
        """
        self.assertEqual(eval_js(src), 8121)

    def test_const_and_tdz_errors(self) -> None:
        with self.assertRaises(RuntimeError):
            eval_js("const x = 1; x = 2;")
        with self.assertRaises(RuntimeError):
            eval_js("const a = b; const b = 3;")


if __name__ == "__main__":
    print(
        "JS fact(5) =",
        eval_js("function fact(n) { if (n == 0) { return 1; } else { return n * fact(n - 1); } } fact(5);"),
    )
