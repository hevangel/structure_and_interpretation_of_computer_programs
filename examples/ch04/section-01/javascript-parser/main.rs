use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Num(i64),
    Ident(String),
    Const,
    Let,
    Function,
    Return,
    If,
    Else,
    Plus,
    Minus,
    Star,
    EqEq,
    Eq,
    Semi,
    Comma,
    LBrace,
    RBrace,
    LParen,
    RParen,
}

fn lex(src: &str) -> Vec<Token> {
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            let num: i64 = chars[start..i].iter().collect::<String>().parse().unwrap();
            out.push(Token::Num(num));
        } else if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            out.push(match word.as_str() {
                "const" => Token::Const,
                "let" => Token::Let,
                "function" => Token::Function,
                "return" => Token::Return,
                "if" => Token::If,
                "else" => Token::Else,
                _ => Token::Ident(word),
            });
        } else if i + 1 < chars.len() && chars[i] == '=' && chars[i + 1] == '=' {
            out.push(Token::EqEq);
            i += 2;
        } else {
            out.push(match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '=' => Token::Eq,
                ';' => Token::Semi,
                ',' => Token::Comma,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '(' => Token::LParen,
                ')' => Token::RParen,
                _ => panic!("unexpected character: {c}"),
            });
            i += 1;
        }
    }
    out
}

#[derive(Clone, Debug)]
enum Expr {
    Num(i64),
    Var(String),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Assign(String, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(Clone, Copy, Debug)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Eq,
}

#[derive(Clone, Debug)]
enum Stmt {
    ConstDecl(String, Expr),
    LetDecl(String, Expr),
    FunDecl(String, Vec<String>, Vec<Stmt>),
    Return(Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    Block(Vec<Stmt>),
    Expr(Expr),
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    fn expect(&mut self, expected: Token) {
        let got = self.next();
        assert_eq!(got, expected, "expected {expected:?}, got {got:?}");
    }

    fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_stmt());
        }
        stmts
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.expect(Token::LBrace);
        let mut stmts = Vec::new();
        while self.peek() != Some(&Token::RBrace) {
            stmts.push(self.parse_stmt());
        }
        self.expect(Token::RBrace);
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Some(Token::Const) => {
                self.next();
                let Token::Ident(name) = self.next() else {
                    panic!("expected identifier");
                };
                self.expect(Token::Eq);
                let rhs = self.parse_expr();
                self.expect(Token::Semi);
                Stmt::ConstDecl(name, rhs)
            }
            Some(Token::Let) => {
                self.next();
                let Token::Ident(name) = self.next() else {
                    panic!("expected identifier");
                };
                self.expect(Token::Eq);
                let rhs = self.parse_expr();
                self.expect(Token::Semi);
                Stmt::LetDecl(name, rhs)
            }
            Some(Token::Function) => {
                self.next();
                let Token::Ident(name) = self.next() else {
                    panic!("expected function name");
                };
                self.expect(Token::LParen);
                let mut params = Vec::new();
                while self.peek() != Some(&Token::RParen) {
                    let Token::Ident(p) = self.next() else {
                        panic!("expected param");
                    };
                    params.push(p);
                    if self.peek() == Some(&Token::Comma) {
                        self.next();
                    }
                }
                self.expect(Token::RParen);
                let body = self.parse_block();
                Stmt::FunDecl(name, params, body)
            }
            Some(Token::Return) => {
                self.next();
                let e = self.parse_expr();
                self.expect(Token::Semi);
                Stmt::Return(e)
            }
            Some(Token::If) => {
                self.next();
                self.expect(Token::LParen);
                let cond = self.parse_expr();
                self.expect(Token::RParen);
                let then_blk = self.parse_block();
                let else_blk = if self.peek() == Some(&Token::Else) {
                    self.next();
                    Some(self.parse_block())
                } else {
                    None
                };
                Stmt::If(cond, then_blk, else_blk)
            }
            Some(Token::LBrace) => Stmt::Block(self.parse_block()),
            _ => {
                let e = self.parse_expr();
                self.expect(Token::Semi);
                Stmt::Expr(e)
            }
        }
    }

    fn parse_expr(&mut self) -> Expr {
        let lhs = self.parse_eq();
        if self.peek() == Some(&Token::Eq) {
            if let Expr::Var(name) = lhs {
                self.next();
                return Expr::Assign(name, Box::new(self.parse_expr()));
            }
        }
        lhs
    }

    fn parse_eq(&mut self) -> Expr {
        let mut lhs = self.parse_add();
        while self.peek() == Some(&Token::EqEq) {
            self.next();
            lhs = Expr::BinOp(BinOp::Eq, Box::new(lhs), Box::new(self.parse_add()));
        }
        lhs
    }

    fn parse_add(&mut self) -> Expr {
        let mut lhs = self.parse_mul();
        while matches!(self.peek(), Some(Token::Plus | Token::Minus)) {
            let op = match self.next() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            lhs = Expr::BinOp(op, Box::new(lhs), Box::new(self.parse_mul()));
        }
        lhs
    }

    fn parse_mul(&mut self) -> Expr {
        let mut lhs = self.parse_call();
        while self.peek() == Some(&Token::Star) {
            self.next();
            lhs = Expr::BinOp(BinOp::Mul, Box::new(lhs), Box::new(self.parse_call()));
        }
        lhs
    }

    fn parse_call(&mut self) -> Expr {
        let mut expr = self.parse_atom();
        while self.peek() == Some(&Token::LParen) {
            self.next();
            let mut args = Vec::new();
            while self.peek() != Some(&Token::RParen) {
                args.push(self.parse_expr());
                if self.peek() == Some(&Token::Comma) {
                    self.next();
                }
            }
            self.expect(Token::RParen);
            expr = Expr::Call(Box::new(expr), args);
        }
        expr
    }

    fn parse_atom(&mut self) -> Expr {
        match self.next() {
            Token::Num(n) => Expr::Num(n),
            Token::Ident(s) => Expr::Var(s),
            Token::LParen => {
                let inner = self.parse_expr();
                self.expect(Token::RParen);
                inner
            }
            other => panic!("unexpected token in expr: {other:?}"),
        }
    }
}

#[derive(Clone, Debug)]
enum Value {
    Int(i64),
    Bool(bool),
    Undefined,
    Closure {
        params: Vec<String>,
        body: Vec<Stmt>,
        env: Env,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Undefined, Value::Undefined) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
enum Signal {
    Normal(Value),
    ReturnValue(Value),
}

#[derive(Clone, Debug)]
struct Binding {
    val: Value,
    is_const: bool,
    initialized: bool,
}

type Env = Rc<RefCell<Frame>>;

#[derive(Debug)]
struct Frame {
    bindings: BTreeMap<String, Binding>,
    parent: Option<Env>,
}

fn scan_declarations(stmts: &[Stmt]) -> Vec<(String, bool)> {
    let mut decls = Vec::new();
    for s in stmts {
        match s {
            Stmt::ConstDecl(n, _) => decls.push((n.clone(), true)),
            Stmt::LetDecl(n, _) | Stmt::FunDecl(n, _, _) => decls.push((n.clone(), false)),
            _ => {}
        }
    }
    decls
}

fn extend_block_env(parent: &Env, stmts: &[Stmt]) -> Env {
    let mut bindings = BTreeMap::new();
    for (name, is_const) in scan_declarations(stmts) {
        bindings.insert(
            name,
            Binding {
                val: Value::Undefined,
                is_const,
                initialized: false,
            },
        );
    }
    Rc::new(RefCell::new(Frame {
        bindings,
        parent: Some(Rc::clone(parent)),
    }))
}

fn lookup_js(env: &Env, name: &str) -> Result<Value, String> {
    let mut cur = Some(Rc::clone(env));
    while let Some(f) = cur {
        let b = f.borrow();
        if let Some(slot) = b.bindings.get(name) {
            if !slot.initialized {
                return Err(format!(
                    "temporal dead zone: {name} read before initialization"
                ));
            }
            return Ok(slot.val.clone());
        }
        cur = b.parent.clone();
    }
    Err(format!("unbound identifier: {name}"))
}

fn assign_js(env: &Env, name: &str, val: Value) -> Result<(), String> {
    let mut cur = Some(Rc::clone(env));
    while let Some(f) = cur {
        let parent = {
            let mut b = f.borrow_mut();
            if let Some(slot) = b.bindings.get_mut(name) {
                if slot.is_const && slot.initialized {
                    return Err(format!("cannot reassign const {name}"));
                }
                slot.val = val;
                slot.initialized = true;
                return Ok(());
            }
            b.parent.clone()
        };
        cur = parent;
    }
    Err(format!("cannot assign unbound: {name}"))
}

fn eval_expr(expr: &Expr, env: &Env) -> Result<Value, String> {
    match expr {
        Expr::Num(n) => Ok(Value::Int(*n)),
        Expr::Var(s) => lookup_js(env, s),
        Expr::Assign(name, rhs) => {
            let v = eval_expr(rhs, env)?;
            assign_js(env, name, v.clone())?;
            Ok(v)
        }
        Expr::BinOp(op, l, r) => {
            let Value::Int(a) = eval_expr(l, env)? else {
                return Err("expected int".into());
            };
            let Value::Int(b) = eval_expr(r, env)? else {
                return Err("expected int".into());
            };
            Ok(match op {
                BinOp::Add => Value::Int(a + b),
                BinOp::Sub => Value::Int(a - b),
                BinOp::Mul => Value::Int(a * b),
                BinOp::Eq => Value::Bool(a == b),
            })
        }
        Expr::Call(callee, args) => {
            let func = eval_expr(callee, env)?;
            let arg_vals: Vec<Value> = args
                .iter()
                .map(|a| eval_expr(a, env))
                .collect::<Result<_, _>>()?;
            match func {
                Value::Closure {
                    params,
                    body,
                    env: def_env,
                } => {
                    let mut bindings = BTreeMap::new();
                    for (p, a) in params.iter().zip(arg_vals) {
                        bindings.insert(
                            p.clone(),
                            Binding {
                                val: a,
                                is_const: false,
                                initialized: true,
                            },
                        );
                    }
                    let call_env = Rc::new(RefCell::new(Frame {
                        bindings,
                        parent: Some(def_env),
                    }));
                    let blk_env = extend_block_env(&call_env, &body);
                    match eval_sequence(&body, &blk_env)? {
                        Signal::ReturnValue(v) => Ok(v),
                        Signal::Normal(_) => Ok(Value::Undefined),
                    }
                }
                _ => Err("not a function".into()),
            }
        }
    }
}

fn eval_sequence(stmts: &[Stmt], env: &Env) -> Result<Signal, String> {
    let mut last = Value::Undefined;
    for stmt in stmts {
        match eval_stmt(stmt, env)? {
            ret @ Signal::ReturnValue(_) => return Ok(ret),
            Signal::Normal(v) => last = v,
        }
    }
    Ok(Signal::Normal(last))
}

fn eval_stmt(stmt: &Stmt, env: &Env) -> Result<Signal, String> {
    match stmt {
        Stmt::ConstDecl(name, rhs) | Stmt::LetDecl(name, rhs) => {
            let val = eval_expr(rhs, env)?;
            if let Some(slot) = env.borrow_mut().bindings.get_mut(name) {
                slot.val = val.clone();
                slot.initialized = true;
            }
            Ok(Signal::Normal(val))
        }
        Stmt::FunDecl(name, params, body) => {
            let c = Value::Closure {
                params: params.clone(),
                body: body.clone(),
                env: Rc::clone(env),
            };
            if let Some(slot) = env.borrow_mut().bindings.get_mut(name) {
                slot.val = c.clone();
                slot.initialized = true;
            }
            Ok(Signal::Normal(c))
        }
        Stmt::Return(expr) => Ok(Signal::ReturnValue(eval_expr(expr, env)?)),
        Stmt::If(cond, then_blk, else_blk) => {
            let truth = match eval_expr(cond, env)? {
                Value::Bool(b) => b,
                Value::Int(n) => n != 0,
                _ => true,
            };
            if truth {
                let sub = extend_block_env(env, then_blk);
                eval_sequence(then_blk, &sub)
            } else if let Some(eb) = else_blk {
                let sub = extend_block_env(env, eb);
                eval_sequence(eb, &sub)
            } else {
                Ok(Signal::Normal(Value::Undefined))
            }
        }
        Stmt::Block(stmts) => {
            let sub = extend_block_env(env, stmts);
            eval_sequence(stmts, &sub)
        }
        Stmt::Expr(expr) => Ok(Signal::Normal(eval_expr(expr, env)?)),
    }
}

fn eval_js(src: &str) -> Result<Value, String> {
    let mut p = Parser {
        tokens: lex(src),
        pos: 0,
    };
    let stmts = p.parse_program();
    let global = extend_block_env(
        &Rc::new(RefCell::new(Frame {
            bindings: BTreeMap::new(),
            parent: None,
        })),
        &stmts,
    );
    match eval_sequence(&stmts, &global)? {
        Signal::Normal(v) | Signal::ReturnValue(v) => Ok(v),
    }
}

fn main() {
    let src = r#"
        const base = 10;
        function fact(n) {
            if (n == 0) { return 1; } else { return n * fact(n - 1); }
        }
        {
            const base = 5;
            fact(base);
        }
    "#;
    println!("JS block/return eval = {:?}", eval_js(src));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_unwinds_nested_blocks_and_preserves_outer_const() {
        let src = r#"
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
        "#;
        // pick(1) returns 8 early; pick(0) finishes block and returns 100 + 21 = 121.
        assert_eq!(eval_js(src), Ok(Value::Int(8121)));
    }

    #[test]
    fn const_reassignment_and_tdz_fail() {
        assert!(eval_js("const x = 1; x = 2;").is_err());
        assert!(eval_js("const a = b; const b = 3;").is_err());
    }
}
