use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum Expr {
    Num(i64),
    Bool(bool),
    Sym(String),
    List(Vec<Expr>),
}

fn tokenize(src: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == ';' {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
        } else if c == '(' || c == ')' || c == '\'' {
            tokens.push(c.to_string());
            i += 1;
        } else {
            let mut j = i;
            while j < chars.len() && !chars[j].is_whitespace() && chars[j] != '(' && chars[j] != ')'
            {
                j += 1;
            }
            tokens.push(chars[i..j].iter().collect());
            i = j;
        }
    }
    tokens
}

fn parse_tokens(tokens: &[String], pos: &mut usize) -> Result<Expr, String> {
    let tok = tokens
        .get(*pos)
        .ok_or_else(|| "unexpected EOF".to_string())?
        .clone();
    *pos += 1;
    match tok.as_str() {
        "(" => {
            let mut items = Vec::new();
            while tokens.get(*pos).map(String::as_str) != Some(")") {
                if *pos >= tokens.len() {
                    return Err("unclosed parenthesis".to_string());
                }
                items.push(parse_tokens(tokens, pos)?);
            }
            *pos += 1;
            Ok(Expr::List(items))
        }
        "'" => Ok(Expr::List(vec![
            Expr::Sym("quote".into()),
            parse_tokens(tokens, pos)?,
        ])),
        "#t" | "true" => Ok(Expr::Bool(true)),
        "#f" | "false" => Ok(Expr::Bool(false)),
        other => {
            if let Ok(n) = other.parse::<i64>() {
                Ok(Expr::Num(n))
            } else {
                Ok(Expr::Sym(other.to_string()))
            }
        }
    }
}

fn parse_program(src: &str) -> Result<Vec<Expr>, String> {
    let tokens = tokenize(src);
    let mut pos = 0;
    let mut exprs = Vec::new();
    while pos < tokens.len() {
        exprs.push(parse_tokens(&tokens, &mut pos)?);
    }
    Ok(exprs)
}

#[derive(Clone, Debug)]
struct Frame {
    bindings: BTreeMap<String, Value>,
    parent: Option<Rc<RefCell<Frame>>>,
}

type Env = Rc<RefCell<Frame>>;

#[derive(Clone, Debug)]
enum ThunkState {
    Evaluated(Value),
    Unevaluated(Expr, Env),
}

#[derive(Clone, Debug)]
enum Value {
    Num(i64),
    Bool(bool),
    Sym(String),
    Nil,
    Pair(Box<Value>, Box<Value>),
    Primitive(String),
    Closure {
        params: Vec<String>,
        body: Vec<Expr>,
        env: Env,
    },
    Thunk(Rc<RefCell<ThunkState>>),
}

impl Value {
    fn as_num(&self) -> Result<i64, String> {
        match self {
            Value::Num(n) => Ok(*n),
            other => Err(format!("expected number, got {other:?}")),
        }
    }

    fn is_truthy(&self) -> bool {
        !matches!(self, Value::Bool(false))
    }
}

struct LazyMachine {
    global: Env,
    thunks_created: usize,
    thunks_forced: usize,
    memoized_hits: usize,
}

impl LazyMachine {
    fn new() -> Self {
        let mut bindings = BTreeMap::new();
        for name in ["+", "-", "*", "/", "=", "<", ">"] {
            bindings.insert(name.to_string(), Value::Primitive(name.to_string()));
        }
        Self {
            global: Rc::new(RefCell::new(Frame {
                bindings,
                parent: None,
            })),
            thunks_created: 0,
            thunks_forced: 0,
            memoized_hits: 0,
        }
    }

    fn lookup(&self, env: &Env, name: &str) -> Result<Value, String> {
        let mut cur = Some(Rc::clone(env));
        while let Some(frame_rc) = cur {
            let frame = frame_rc.borrow();
            if let Some(val) = frame.bindings.get(name) {
                return Ok(val.clone());
            }
            cur = frame.parent.clone();
        }
        Err(format!("unbound variable: {name}"))
    }

    fn set_var(&self, env: &Env, name: &str, val: Value) -> Result<(), String> {
        let mut cur = Some(Rc::clone(env));
        while let Some(frame_rc) = cur {
            let mut frame = frame_rc.borrow_mut();
            if frame.bindings.contains_key(name) {
                frame.bindings.insert(name.to_string(), val);
                return Ok(());
            }
            cur = frame.parent.clone();
        }
        Err(format!("unbound variable for set!: {name}"))
    }

    fn delay_it(&mut self, expr: Expr, env: &Env) -> Value {
        self.thunks_created += 1;
        Value::Thunk(Rc::new(RefCell::new(ThunkState::Unevaluated(
            expr,
            Rc::clone(env),
        ))))
    }

    fn force_it(&mut self, val: Value) -> Result<Value, String> {
        match val {
            Value::Thunk(cell) => {
                let state = cell.borrow().clone();
                match state {
                    ThunkState::Evaluated(cached) => {
                        self.memoized_hits += 1;
                        Ok(cached)
                    }
                    ThunkState::Unevaluated(expr, env) => {
                        self.thunks_forced += 1;
                        let evaluated = self.actual_value(&expr, &env)?;
                        *cell.borrow_mut() = ThunkState::Evaluated(evaluated.clone());
                        Ok(evaluated)
                    }
                }
            }
            other => Ok(other),
        }
    }

    fn actual_value(&mut self, expr: &Expr, env: &Env) -> Result<Value, String> {
        let v = self.eval(expr, env)?;
        self.force_it(v)
    }

    fn text_of_quotation(expr: &Expr) -> Value {
        match expr {
            Expr::Num(n) => Value::Num(*n),
            Expr::Bool(b) => Value::Bool(*b),
            Expr::Sym(s) => Value::Sym(s.clone()),
            Expr::List(items) => {
                let mut acc = Value::Nil;
                for item in items.iter().rev() {
                    acc = Value::Pair(Box::new(Self::text_of_quotation(item)), Box::new(acc));
                }
                acc
            }
        }
    }

    fn eval_sequence(&mut self, exprs: &[Expr], env: &Env) -> Result<Value, String> {
        let mut last = Value::Nil;
        for expr in exprs {
            last = self.eval(expr, env)?;
        }
        Ok(last)
    }

    fn eval(&mut self, expr: &Expr, env: &Env) -> Result<Value, String> {
        match expr {
            Expr::Num(n) => Ok(Value::Num(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Sym(s) => self.lookup(env, s),
            Expr::List(items) if items.is_empty() => Ok(Value::Nil),
            Expr::List(items) => {
                if let Expr::Sym(head) = &items[0] {
                    match head.as_str() {
                        "quote" => return Ok(Self::text_of_quotation(&items[1])),
                        "if" => {
                            let pred = self.actual_value(&items[1], env)?;
                            if pred.is_truthy() {
                                return self.eval(&items[2], env);
                            } else if items.len() > 3 {
                                return self.eval(&items[3], env);
                            } else {
                                return Ok(Value::Bool(false));
                            }
                        }
                        "begin" => return self.eval_sequence(&items[1..], env),
                        "lambda" => {
                            let params = match &items[1] {
                                Expr::List(ps) => ps
                                    .iter()
                                    .map(|p| match p {
                                        Expr::Sym(s) => Ok(s.clone()),
                                        _ => Err("lambda param must be symbol".to_string()),
                                    })
                                    .collect::<Result<Vec<_>, _>>()?,
                                _ => return Err("malformed lambda params".to_string()),
                            };
                            return Ok(Value::Closure {
                                params,
                                body: items[2..].to_vec(),
                                env: Rc::clone(env),
                            });
                        }
                        "define" => {
                            match &items[1] {
                                Expr::Sym(name) => {
                                    let val = self.eval(&items[2], env)?;
                                    env.borrow_mut().bindings.insert(name.clone(), val);
                                }
                                Expr::List(sig) => {
                                    let name = match &sig[0] {
                                        Expr::Sym(s) => s.clone(),
                                        _ => return Err("procedure name must be symbol".into()),
                                    };
                                    let params = sig[1..]
                                        .iter()
                                        .map(|p| match p {
                                            Expr::Sym(s) => Ok(s.clone()),
                                            _ => Err("param must be symbol".to_string()),
                                        })
                                        .collect::<Result<Vec<_>, String>>()?;
                                    let closure = Value::Closure {
                                        params,
                                        body: items[2..].to_vec(),
                                        env: Rc::clone(env),
                                    };
                                    env.borrow_mut().bindings.insert(name, closure);
                                }
                                _ => return Err("malformed define".into()),
                            }
                            return Ok(Value::Sym("ok".into()));
                        }
                        "set!" => {
                            let name = match &items[1] {
                                Expr::Sym(s) => s,
                                _ => return Err("set! target must be symbol".into()),
                            };
                            let val = self.eval(&items[2], env)?;
                            self.set_var(env, name, val)?;
                            return Ok(Value::Sym("ok".into()));
                        }
                        _ => {}
                    }
                }
                let proc_val = self.actual_value(&items[0], env)?;
                self.apply(proc_val, &items[1..], env)
            }
        }
    }

    fn apply(&mut self, proc_val: Value, args: &[Expr], call_env: &Env) -> Result<Value, String> {
        match proc_val {
            Value::Primitive(op) => {
                let mut forced = Vec::with_capacity(args.len());
                for arg in args {
                    forced.push(self.actual_value(arg, call_env)?);
                }
                match op.as_str() {
                    "+" => Ok(Value::Num(forced.iter().map(|v| v.as_num().unwrap()).sum())),
                    "*" => Ok(Value::Num(
                        forced.iter().map(|v| v.as_num().unwrap()).product(),
                    )),
                    "-" => {
                        let a = forced[0].as_num()?;
                        let b = forced[1].as_num()?;
                        Ok(Value::Num(a - b))
                    }
                    "/" => {
                        let a = forced[0].as_num()?;
                        let b = forced[1].as_num()?;
                        if b == 0 {
                            Err("division by zero".to_string())
                        } else {
                            Ok(Value::Num(a / b))
                        }
                    }
                    "=" => Ok(Value::Bool(forced[0].as_num()? == forced[1].as_num()?)),
                    "<" => Ok(Value::Bool(forced[0].as_num()? < forced[1].as_num()?)),
                    ">" => Ok(Value::Bool(forced[0].as_num()? > forced[1].as_num()?)),
                    _ => Err(format!("unknown primitive {op}")),
                }
            }
            Value::Closure { params, body, env } => {
                if params.len() != args.len() {
                    return Err(format!(
                        "arity mismatch: expected {}, got {}",
                        params.len(),
                        args.len()
                    ));
                }
                let mut bindings = BTreeMap::new();
                for (p, arg_expr) in params.into_iter().zip(args.iter()) {
                    bindings.insert(p, self.delay_it(arg_expr.clone(), call_env));
                }
                let frame = Rc::new(RefCell::new(Frame {
                    bindings,
                    parent: Some(env),
                }));
                self.eval_sequence(&body, &frame)
            }
            other => Err(format!("not applicable: {other:?}")),
        }
    }

    fn run_program(&mut self, src: &str) -> Result<Value, String> {
        let exprs = parse_program(src)?;
        let env = Rc::clone(&self.global);
        let mut last = Value::Nil;
        for expr in &exprs {
            last = self.actual_value(expr, &env)?;
        }
        Ok(last)
    }
}

fn main() {
    let mut vm = LazyMachine::new();
    let res = vm
        .run_program(
            "
            (define (unless condition usual exceptional)
              (if condition exceptional usual))
            (define (cons x y) (lambda (m) (m x y)))
            (define (car z) (z (lambda (p q) p)))
            (define (cdr z) (z (lambda (p q) q)))
            (define (list-ref items n)
              (if (= n 0) (car items) (list-ref (cdr items) (- n 1))))
            (define (add-lists l1 l2)
              (cons (+ (car l1) (car l2)) (add-lists (cdr l1) (cdr l2))))
            (define ones (cons 1 ones))
            (define integers (cons 1 (add-lists ones integers)))
            (+ (unless #f 10 (/ 1 0)) (list-ref integers 9))
            ",
        )
        .unwrap();
    let quoted = vm.run_program("'(alpha beta)").unwrap();
    let head_sym = match &quoted {
        Value::Pair(head, tail) => match (&**head, &**tail) {
            (Value::Sym(s), _) => s.as_str(),
            _ => "",
        },
        _ => "",
    };
    println!(
        "lazy result = {:?} (head={}, created={}, forced={}, memo_hits={})",
        res.as_num().unwrap(),
        head_sym,
        vm.thunks_created,
        vm.thunks_forced,
        vm.memoized_hits
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_order_avoids_div_by_zero_and_memoizes() {
        let mut vm = LazyMachine::new();
        let val = vm
            .run_program(
                "
                (define count 0)
                (define (id x) (set! count (+ count 1)) x)
                (define (square x) (* x x))
                (define (unless cond usual exc) (if cond exc usual))
                (define a (unless #f (square (id 7)) (/ 1 0)))
                count
                ",
            )
            .unwrap();
        // `a` still holds an unforced thunk returned from `unless`, so `count == 0`!
        assert_eq!(val.as_num().unwrap(), 0);
        // Forcing `a` returns 49, runs `(id 7)` once (`count == 1`), and hits memoized `x` in `(* x x)`.
        assert_eq!(vm.run_program("a").unwrap().as_num().unwrap(), 49);
        assert_eq!(vm.run_program("count").unwrap().as_num().unwrap(), 1);
        assert!(vm.memoized_hits >= 1);
    }

    #[test]
    fn lazy_procedural_infinite_streams_work() {
        let mut vm = LazyMachine::new();
        let eighth = vm
            .run_program(
                "
                (define (cons x y) (lambda (m) (m x y)))
                (define (car z) (z (lambda (p q) p)))
                (define (cdr z) (z (lambda (p q) q)))
                (define (list-ref items n)
                  (if (= n 0) (car items) (list-ref (cdr items) (- n 1))))
                (define (add-lists a b)
                  (cons (+ (car a) (car b)) (add-lists (cdr a) (cdr b))))
                (define fibs
                  (cons 0 (cons 1 (add-lists (cdr fibs) fibs))))
                (list-ref fibs 10)
                ",
            )
            .unwrap()
            .as_num()
            .unwrap();
        assert_eq!(eighth, 55);
        let q = vm.run_program("'(alpha beta)").unwrap();
        let formatted = match &q {
            Value::Pair(head, _) => match &**head {
                Value::Sym(s) => s.clone(),
                _ => String::new(),
            },
            _ => String::new(),
        };
        assert_eq!(formatted, "alpha");
    }
}
