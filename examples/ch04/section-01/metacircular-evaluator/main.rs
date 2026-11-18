use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum SExpr {
    Num(i64),
    Bool(bool),
    Sym(String),
    List(Vec<SExpr>),
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
            let start = i;
            while i < chars.len() && !chars[i].is_whitespace() && chars[i] != '(' && chars[i] != ')'
            {
                i += 1;
            }
            tokens.push(chars[start..i].iter().collect());
        }
    }
    tokens
}

fn parse_one(tokens: &[String], pos: &mut usize) -> Result<SExpr, String> {
    let tok = tokens.get(*pos).ok_or("unexpected EOF")?;
    *pos += 1;
    match tok.as_str() {
        "(" => {
            let mut items = Vec::new();
            while tokens.get(*pos).map(String::as_str) != Some(")") {
                items.push(parse_one(tokens, pos)?);
            }
            *pos += 1;
            Ok(SExpr::List(items))
        }
        ")" => Err("unexpected ')'".into()),
        "'" => Ok(SExpr::List(vec![
            SExpr::Sym("quote".into()),
            parse_one(tokens, pos)?,
        ])),
        "#t" | "true" => Ok(SExpr::Bool(true)),
        "#f" | "false" => Ok(SExpr::Bool(false)),
        other => {
            if let Ok(n) = other.parse::<i64>() {
                Ok(SExpr::Num(n))
            } else {
                Ok(SExpr::Sym(other.to_string()))
            }
        }
    }
}

fn parse_program(src: &str) -> Result<Vec<SExpr>, String> {
    let tokens = tokenize(src);
    let mut pos = 0;
    let mut out = Vec::new();
    while pos < tokens.len() {
        out.push(parse_one(&tokens, &mut pos)?);
    }
    Ok(out)
}

#[derive(Clone, Debug)]
enum Value {
    Num(i64),
    Bool(bool),
    Sym(String),
    Pair(Box<Value>, Box<Value>),
    Nil,
    Unassigned,
    Primitive(String, fn(&[Value]) -> Result<Value, String>),
    Closure {
        params: Vec<String>,
        body: Vec<SExpr>,
        env: Env,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Num(a), Value::Num(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Sym(a), Value::Sym(b)) => a == b,
            (Value::Pair(a1, a2), Value::Pair(b1, b2)) => a1 == b1 && a2 == b2,
            (Value::Nil, Value::Nil) | (Value::Unassigned, Value::Unassigned) => true,
            (Value::Primitive(a, _), Value::Primitive(b, _)) => a == b,
            _ => false,
        }
    }
}

type Env = Rc<RefCell<Frame>>;

#[derive(Debug)]
struct Frame {
    bindings: BTreeMap<String, Value>,
    parent: Option<Env>,
}

fn extend_env(parent: &Env, params: &[String], args: &[Value]) -> Result<Env, String> {
    if params.len() != args.len() {
        return Err(format!(
            "expected {} args, got {}",
            params.len(),
            args.len()
        ));
    }
    let mut bindings = BTreeMap::new();
    for (p, a) in params.iter().zip(args) {
        bindings.insert(p.clone(), a.clone());
    }
    Ok(Rc::new(RefCell::new(Frame {
        bindings,
        parent: Some(Rc::clone(parent)),
    })))
}

fn lookup_var(env: &Env, name: &str) -> Result<Value, String> {
    let mut cur = Some(Rc::clone(env));
    while let Some(frame_rc) = cur {
        let frame = frame_rc.borrow();
        if let Some(val) = frame.bindings.get(name) {
            if matches!(val, Value::Unassigned) {
                return Err(format!("unassigned internal define: {name}"));
            }
            return Ok(val.clone());
        }
        cur = frame.parent.clone();
    }
    Err(format!("unbound variable: {name}"))
}

fn set_var(env: &Env, name: &str, val: Value) -> Result<(), String> {
    let mut cur = Some(Rc::clone(env));
    while let Some(frame_rc) = cur {
        let parent = {
            let mut frame = frame_rc.borrow_mut();
            if frame.bindings.contains_key(name) {
                frame.bindings.insert(name.to_string(), val);
                return Ok(());
            }
            frame.parent.clone()
        };
        cur = parent;
    }
    Err(format!("cannot set! unbound variable: {name}"))
}

fn define_var(env: &Env, name: &str, val: Value) {
    env.borrow_mut().bindings.insert(name.to_string(), val);
}

fn quote_sexpr(e: &SExpr) -> Value {
    match e {
        SExpr::Num(n) => Value::Num(*n),
        SExpr::Bool(b) => Value::Bool(*b),
        SExpr::Sym(s) => Value::Sym(s.clone()),
        SExpr::List(items) => items.iter().rev().fold(Value::Nil, |acc, item| {
            Value::Pair(Box::new(quote_sexpr(item)), Box::new(acc))
        }),
    }
}

/// Section 4.1.6: transform internal `define`s into `*unassigned*` `let` bindings + `set!`.
fn scan_out_defines(body: &[SExpr]) -> Vec<SExpr> {
    let mut defines = Vec::new();
    let mut rest = Vec::new();
    for expr in body {
        if let SExpr::List(items) = expr {
            if items.first() == Some(&SExpr::Sym("define".into())) {
                match items.get(1) {
                    Some(SExpr::Sym(name)) if items.len() == 3 => {
                        defines.push((name.clone(), items[2].clone()));
                        continue;
                    }
                    Some(SExpr::List(sig)) if !sig.is_empty() => {
                        if let SExpr::Sym(name) = &sig[0] {
                            let params = sig[1..].to_vec();
                            let mut lam = vec![SExpr::Sym("lambda".into()), SExpr::List(params)];
                            lam.extend_from_slice(&items[2..]);
                            defines.push((name.clone(), SExpr::List(lam)));
                            continue;
                        }
                    }
                    _ => {}
                }
            }
        }
        rest.push(expr.clone());
    }
    if defines.is_empty() {
        return body.to_vec();
    }
    let bindings = defines
        .iter()
        .map(|(n, _)| {
            SExpr::List(vec![
                SExpr::Sym(n.clone()),
                SExpr::Sym("*unassigned*".into()),
            ])
        })
        .collect();
    let mut inner = Vec::new();
    for (n, rhs) in defines {
        inner.push(SExpr::List(vec![
            SExpr::Sym("set!".into()),
            SExpr::Sym(n),
            rhs,
        ]));
    }
    inner.extend(rest);
    let mut let_form = vec![SExpr::Sym("let".into()), SExpr::List(bindings)];
    let_form.extend(inner);
    vec![SExpr::List(let_form)]
}

fn is_truthy(v: &Value) -> bool {
    !matches!(v, Value::Bool(false))
}

fn eval_seq(exprs: &[SExpr], env: &Env) -> Result<Value, String> {
    let mut last = Value::Nil;
    for e in exprs {
        last = eval(e, env)?;
    }
    Ok(last)
}

fn eval(expr: &SExpr, env: &Env) -> Result<Value, String> {
    match expr {
        SExpr::Num(n) => Ok(Value::Num(*n)),
        SExpr::Bool(b) => Ok(Value::Bool(*b)),
        SExpr::Sym(s) if s == "*unassigned*" => Ok(Value::Unassigned),
        SExpr::Sym(s) => lookup_var(env, s),
        SExpr::List(items) if items.is_empty() => Ok(Value::Nil),
        SExpr::List(items) => {
            if let SExpr::Sym(head) = &items[0] {
                match head.as_str() {
                    "quote" => return Ok(quote_sexpr(&items[1])),
                    "if" => {
                        let cond = eval(&items[1], env)?;
                        return if is_truthy(&cond) {
                            eval(&items[2], env)
                        } else if let Some(alt) = items.get(3) {
                            eval(alt, env)
                        } else {
                            Ok(Value::Bool(false))
                        };
                    }
                    "cond" => {
                        for clause in &items[1..] {
                            let SExpr::List(parts) = clause else {
                                return Err("invalid cond clause".into());
                            };
                            if parts[0] == SExpr::Sym("else".into())
                                || is_truthy(&eval(&parts[0], env)?)
                            {
                                return eval_seq(&parts[1..], env);
                            }
                        }
                        return Ok(Value::Bool(false));
                    }
                    "and" => {
                        let mut res = Value::Bool(true);
                        for sub in &items[1..] {
                            res = eval(sub, env)?;
                            if !is_truthy(&res) {
                                return Ok(Value::Bool(false));
                            }
                        }
                        return Ok(res);
                    }
                    "or" => {
                        for sub in &items[1..] {
                            let res = eval(sub, env)?;
                            if is_truthy(&res) {
                                return Ok(res);
                            }
                        }
                        return Ok(Value::Bool(false));
                    }
                    "begin" => return eval_seq(&items[1..], env),
                    "set!" => {
                        let SExpr::Sym(name) = &items[1] else {
                            return Err("set! target must be symbol".into());
                        };
                        let val = eval(&items[2], env)?;
                        set_var(env, name, val.clone())?;
                        return Ok(val);
                    }
                    "define" => match &items[1] {
                        SExpr::Sym(name) => {
                            let val = eval(&items[2], env)?;
                            define_var(env, name, val.clone());
                            return Ok(val);
                        }
                        SExpr::List(sig) => {
                            let SExpr::Sym(name) = &sig[0] else {
                                return Err("invalid define signature".into());
                            };
                            let params: Vec<String> = sig[1..]
                                .iter()
                                .map(|s| match s {
                                    SExpr::Sym(n) => Ok(n.clone()),
                                    _ => Err("param must be symbol".to_string()),
                                })
                                .collect::<Result<_, String>>()?;
                            let closure = Value::Closure {
                                params,
                                body: scan_out_defines(&items[2..]),
                                env: Rc::clone(env),
                            };
                            define_var(env, name, closure.clone());
                            return Ok(closure);
                        }
                        _ => return Err("invalid define".into()),
                    },
                    "lambda" => {
                        let SExpr::List(param_exprs) = &items[1] else {
                            return Err("lambda params must be list".into());
                        };
                        let params: Vec<String> = param_exprs
                            .iter()
                            .map(|s| match s {
                                SExpr::Sym(n) => Ok(n.clone()),
                                _ => Err("param must be symbol".to_string()),
                            })
                            .collect::<Result<_, String>>()?;
                        return Ok(Value::Closure {
                            params,
                            body: scan_out_defines(&items[2..]),
                            env: Rc::clone(env),
                        });
                    }
                    "let" => {
                        let SExpr::List(bindings) = &items[1] else {
                            return Err("let bindings must be list".into());
                        };
                        let mut params = Vec::new();
                        let mut args = Vec::new();
                        for b in bindings {
                            let SExpr::List(pair) = b else {
                                return Err("invalid let binding".into());
                            };
                            params.push(pair[0].clone());
                            args.push(pair[1].clone());
                        }
                        let mut lam = vec![SExpr::Sym("lambda".into()), SExpr::List(params)];
                        lam.extend_from_slice(&items[2..]);
                        let mut app = vec![SExpr::List(lam)];
                        app.extend(args);
                        return eval(&SExpr::List(app), env);
                    }
                    "let*" => {
                        let SExpr::List(bindings) = &items[1] else {
                            return Err("let* bindings must be list".into());
                        };
                        let mut nested = SExpr::List({
                            let mut b = vec![SExpr::Sym("begin".into())];
                            b.extend_from_slice(&items[2..]);
                            b
                        });
                        for pair in bindings.iter().rev() {
                            nested = SExpr::List(vec![
                                SExpr::Sym("let".into()),
                                SExpr::List(vec![pair.clone()]),
                                nested,
                            ]);
                        }
                        return eval(&nested, env);
                    }
                    _ => {}
                }
            }
            let proc = eval(&items[0], env)?;
            let args: Vec<Value> = items[1..]
                .iter()
                .map(|a| eval(a, env))
                .collect::<Result<_, _>>()?;
            apply(&proc, &args)
        }
    }
}

fn apply(proc: &Value, args: &[Value]) -> Result<Value, String> {
    match proc {
        Value::Primitive(_, f) => f(args),
        Value::Closure { params, body, env } => {
            let call_env = extend_env(env, params, args)?;
            eval_seq(body, &call_env)
        }
        other => Err(format!("cannot apply non-procedure: {other:?}")),
    }
}

type PrimEntry = (&'static str, fn(&[Value]) -> Result<Value, String>);

fn setup_global_env() -> Env {
    let mut bindings = BTreeMap::new();
    let prims: [PrimEntry; 9] = [
        ("+", |args| {
            let mut sum = 0;
            for a in args {
                let Value::Num(n) = a else {
                    return Err("expected num".into());
                };
                sum += *n;
            }
            Ok(Value::Num(sum))
        }),
        ("-", |args| match args {
            [Value::Num(a)] => Ok(Value::Num(-a)),
            [Value::Num(a), Value::Num(b)] => Ok(Value::Num(a - b)),
            _ => Err("bad - args".into()),
        }),
        ("*", |args| {
            let mut prod = 1;
            for a in args {
                let Value::Num(n) = a else {
                    return Err("expected num".into());
                };
                prod *= *n;
            }
            Ok(Value::Num(prod))
        }),
        ("=", |args| match args {
            [Value::Num(a), Value::Num(b)] => Ok(Value::Bool(a == b)),
            _ => Err("bad = args".into()),
        }),
        ("<", |args| match args {
            [Value::Num(a), Value::Num(b)] => Ok(Value::Bool(a < b)),
            _ => Err("bad < args".into()),
        }),
        ("cons", |args| {
            Ok(Value::Pair(
                Box::new(args[0].clone()),
                Box::new(args[1].clone()),
            ))
        }),
        ("car", |args| match &args[0] {
            Value::Pair(a, _) => Ok((**a).clone()),
            _ => Err("car on non-pair".into()),
        }),
        ("cdr", |args| match &args[0] {
            Value::Pair(_, d) => Ok((**d).clone()),
            _ => Err("cdr on non-pair".into()),
        }),
        ("null?", |args| {
            Ok(Value::Bool(matches!(args[0], Value::Nil)))
        }),
    ];
    for (name, f) in prims {
        bindings.insert(name.to_string(), Value::Primitive(name.to_string(), f));
    }
    Rc::new(RefCell::new(Frame {
        bindings,
        parent: None,
    }))
}

fn run_program(src: &str) -> Result<Value, String> {
    let env = setup_global_env();
    let exprs = parse_program(src)?;
    eval_seq(&exprs, &env)
}

fn main() {
    let src = r#"
        (define (even-odd n)
          (define (even? k) (if (= k 0) #t (odd? (- k 1))))
          (define (odd? k) (if (= k 0) #f (even? (- k 1))))
          (even? n))
        (let* ((a 3) (b (* a 4)))
          (if (even-odd b) 'even (+ a b)))
    "#;
    let tag = match run_program(src) {
        Ok(Value::Sym(s)) => s,
        other => format!("{other:?}"),
    };
    println!("metacircular eval result = {tag}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutual_internal_defines_quote_cond_and_let_star() {
        let src = r#"
            (define (f x)
              (define (even? n) (if (= n 0) #t (odd? (- n 1))))
              (define (odd? n) (if (= n 0) #f (even? (- n 1))))
              (cond ((even? x) 'even) (else 'odd)))
            (let* ((a 2) (b (+ a 4)))
              (cons (f b) (f (- b 1))))
        "#;
        let res = run_program(src).unwrap();
        assert_eq!(
            res,
            Value::Pair(
                Box::new(Value::Sym("even".into())),
                Box::new(Value::Sym("odd".into()))
            )
        );
    }

    #[test]
    fn y_combinator_factorial_without_define() {
        // Exercise 4.21: recursion solely via lambda application.
        let src = r#"
            ((lambda (n)
               ((lambda (fact) (fact fact n))
                (lambda (ft k)
                  (if (= k 1) 1 (* k (ft ft (- k 1)))))))
             6)
        "#;
        assert_eq!(run_program(src), Ok(Value::Num(720)));
    }
}
