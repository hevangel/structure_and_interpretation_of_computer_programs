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
        } else if c == '(' || c == ')' {
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
    let mut out = Vec::new();
    while pos < tokens.len() {
        out.push(parse_tokens(&tokens, &mut pos)?);
    }
    Ok(out)
}

#[derive(Clone, Debug)]
struct Frame {
    bindings: BTreeMap<String, RegVal>,
    parent: Option<Env>,
}

type Env = Rc<RefCell<Frame>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ContLabel {
    Done,
    EvApplDidOperator,
    EvApplAccumulateArg,
    EvApplAccumLastArg,
    EvIfDecide,
    EvSequenceContinue,
    EvAssignment1,
    EvDefinition1,
}

#[derive(Clone, Debug)]
enum RegVal {
    Empty,
    Num(i64),
    Bool(bool),
    Sym(String),
    Expr(Expr),
    ExprSlice(Vec<Expr>),
    ArgList(Vec<RegVal>),
    Env(Env),
    Cont(ContLabel),
    Primitive(String),
    Compound {
        params: Vec<String>,
        body: Vec<Expr>,
        env: Env,
    },
}

impl RegVal {
    fn as_num(&self) -> Result<i64, String> {
        match self {
            RegVal::Num(n) => Ok(*n),
            other => Err(format!("expected number, got {other:?}")),
        }
    }

    fn as_expr(&self) -> Result<&Expr, String> {
        match self {
            RegVal::Expr(e) => Ok(e),
            other => Err(format!("expected Expr register, got {other:?}")),
        }
    }

    fn as_env(&self) -> Result<Env, String> {
        match self {
            RegVal::Env(e) => Ok(Rc::clone(e)),
            other => Err(format!("expected Env register, got {other:?}")),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct StackStats {
    total_pushes: usize,
    current_depth: usize,
    max_depth: usize,
}

#[derive(Clone, Copy, Debug)]
enum State {
    EvalDispatch,
    EvApplDidOperator,
    EvApplOperandLoop,
    EvApplAccumulateArg,
    EvApplLastArg,
    EvApplAccumLastArg,
    ApplyDispatch,
    EvSequence,
    EvSequenceContinue,
    EvSequenceLastExp,
    EvIfDecide,
    EvAssignment1,
    EvDefinition1,
    Done,
}

struct ExplicitControlEvaluator {
    // Seven canonical SICP §5.4 machine registers
    exp: RegVal,
    env: RegVal,
    val: RegVal,
    cont: RegVal, // `continue`
    proc_reg: RegVal,
    argl: RegVal,
    unev: RegVal,

    stack: Vec<RegVal>,
    stats: StackStats,
    global_env: Env,
}

impl ExplicitControlEvaluator {
    fn new() -> Self {
        let mut bindings = BTreeMap::new();
        for p in ["+", "-", "*", "=", "<", ">"] {
            bindings.insert(p.to_string(), RegVal::Primitive(p.to_string()));
        }
        let global_env = Rc::new(RefCell::new(Frame {
            bindings,
            parent: None,
        }));
        Self {
            exp: RegVal::Empty,
            env: RegVal::Env(Rc::clone(&global_env)),
            val: RegVal::Empty,
            cont: RegVal::Cont(ContLabel::Done),
            proc_reg: RegVal::Empty,
            argl: RegVal::Empty,
            unev: RegVal::Empty,
            stack: Vec::new(),
            stats: StackStats::default(),
            global_env,
        }
    }

    fn save(&mut self, reg: RegVal) {
        self.stack.push(reg);
        self.stats.total_pushes += 1;
        self.stats.current_depth += 1;
        if self.stats.current_depth > self.stats.max_depth {
            self.stats.max_depth = self.stats.current_depth;
        }
    }

    fn restore(&mut self) -> Result<RegVal, String> {
        let v = self
            .stack
            .pop()
            .ok_or_else(|| "stack underflow on restore".to_string())?;
        self.stats.current_depth -= 1;
        Ok(v)
    }

    fn reset_stats(&mut self) {
        self.stack.clear();
        self.stats = StackStats::default();
    }

    fn lookup_var(&self, name: &str, env: &Env) -> Result<RegVal, String> {
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

    fn set_var(&self, name: &str, val: RegVal, env: &Env) -> Result<(), String> {
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

    fn apply_primitive(&self, op: &str, args: &[RegVal]) -> Result<RegVal, String> {
        match op {
            "+" => Ok(RegVal::Num(args.iter().map(|a| a.as_num().unwrap()).sum())),
            "*" => Ok(RegVal::Num(
                args.iter().map(|a| a.as_num().unwrap()).product(),
            )),
            "-" => Ok(RegVal::Num(args[0].as_num()? - args[1].as_num()?)),
            "=" => Ok(RegVal::Bool(args[0].as_num()? == args[1].as_num()?)),
            "<" => Ok(RegVal::Bool(args[0].as_num()? < args[1].as_num()?)),
            ">" => Ok(RegVal::Bool(args[0].as_num()? > args[1].as_num()?)),
            _ => Err(format!("unknown primitive {op}")),
        }
    }

    fn continue_to(&self) -> Result<State, String> {
        match &self.cont {
            RegVal::Cont(lbl) => Ok(match lbl {
                ContLabel::Done => State::Done,
                ContLabel::EvApplDidOperator => State::EvApplDidOperator,
                ContLabel::EvApplAccumulateArg => State::EvApplAccumulateArg,
                ContLabel::EvApplAccumLastArg => State::EvApplAccumLastArg,
                ContLabel::EvIfDecide => State::EvIfDecide,
                ContLabel::EvSequenceContinue => State::EvSequenceContinue,
                ContLabel::EvAssignment1 => State::EvAssignment1,
                ContLabel::EvDefinition1 => State::EvDefinition1,
            }),
            other => Err(format!("invalid continue register: {other:?}")),
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Result<RegVal, String> {
        self.exp = RegVal::Expr(expr);
        self.env = RegVal::Env(Rc::clone(&self.global_env));
        self.cont = RegVal::Cont(ContLabel::Done);
        let mut state = State::EvalDispatch;

        loop {
            state = match state {
                State::Done => return Ok(self.val.clone()),
                State::EvalDispatch => {
                    let expr = self.exp.as_expr()?.clone();
                    match expr {
                        Expr::Num(n) => {
                            self.val = RegVal::Num(n);
                            self.continue_to()?
                        }
                        Expr::Bool(b) => {
                            self.val = RegVal::Bool(b);
                            self.continue_to()?
                        }
                        Expr::Sym(s) => {
                            let env = self.env.as_env()?;
                            self.val = self.lookup_var(&s, &env)?;
                            self.continue_to()?
                        }
                        Expr::List(items) => {
                            if items.is_empty() {
                                return Err("cannot evaluate empty combination".to_string());
                            }
                            if let Expr::Sym(head) = &items[0] {
                                match head.as_str() {
                                    "if" => {
                                        self.save(self.exp.clone());
                                        self.save(self.env.clone());
                                        self.save(self.cont.clone());
                                        self.cont = RegVal::Cont(ContLabel::EvIfDecide);
                                        self.exp = RegVal::Expr(items[1].clone());
                                        State::EvalDispatch
                                    }
                                    "begin" => {
                                        self.unev = RegVal::ExprSlice(items[1..].to_vec());
                                        self.save(self.cont.clone());
                                        State::EvSequence
                                    }
                                    "lambda" => {
                                        let params = match &items[1] {
                                            Expr::List(ps) => ps
                                                .iter()
                                                .map(|p| match p {
                                                    Expr::Sym(s) => Ok(s.clone()),
                                                    _ => Err("param must be symbol".to_string()),
                                                })
                                                .collect::<Result<Vec<_>, String>>()?,
                                            _ => return Err("malformed lambda".into()),
                                        };
                                        self.val = RegVal::Compound {
                                            params,
                                            body: items[2..].to_vec(),
                                            env: self.env.as_env()?,
                                        };
                                        self.continue_to()?
                                    }
                                    "define" => {
                                        let (name, val_expr) = match &items[1] {
                                            Expr::Sym(s) => (s.clone(), items[2].clone()),
                                            Expr::List(sig) => {
                                                let n = match &sig[0] {
                                                    Expr::Sym(s) => s.clone(),
                                                    _ => return Err("bad proc name".into()),
                                                };
                                                let lam = Expr::List({
                                                    let mut v = vec![
                                                        Expr::Sym("lambda".into()),
                                                        Expr::List(sig[1..].to_vec()),
                                                    ];
                                                    v.extend(items[2..].iter().cloned());
                                                    v
                                                });
                                                (n, lam)
                                            }
                                            _ => return Err("bad define".into()),
                                        };
                                        self.unev = RegVal::Sym(name);
                                        self.save(self.unev.clone());
                                        self.exp = RegVal::Expr(val_expr);
                                        self.save(self.env.clone());
                                        self.save(self.cont.clone());
                                        self.cont = RegVal::Cont(ContLabel::EvDefinition1);
                                        State::EvalDispatch
                                    }
                                    "set!" => {
                                        let Expr::Sym(name) = &items[1] else {
                                            return Err("set! target must be symbol".into());
                                        };
                                        self.unev = RegVal::Sym(name.clone());
                                        self.save(self.unev.clone());
                                        self.exp = RegVal::Expr(items[2].clone());
                                        self.save(self.env.clone());
                                        self.save(self.cont.clone());
                                        self.cont = RegVal::Cont(ContLabel::EvAssignment1);
                                        State::EvalDispatch
                                    }
                                    _ => {
                                        self.save(self.cont.clone());
                                        self.save(self.env.clone());
                                        self.unev = RegVal::ExprSlice(items[1..].to_vec());
                                        self.save(self.unev.clone());
                                        self.exp = RegVal::Expr(items[0].clone());
                                        self.cont = RegVal::Cont(ContLabel::EvApplDidOperator);
                                        State::EvalDispatch
                                    }
                                }
                            } else {
                                self.save(self.cont.clone());
                                self.save(self.env.clone());
                                self.unev = RegVal::ExprSlice(items[1..].to_vec());
                                self.save(self.unev.clone());
                                self.exp = RegVal::Expr(items[0].clone());
                                self.cont = RegVal::Cont(ContLabel::EvApplDidOperator);
                                State::EvalDispatch
                            }
                        }
                    }
                }
                State::EvApplDidOperator => {
                    self.unev = self.restore()?;
                    self.env = self.restore()?;
                    self.argl = RegVal::ArgList(Vec::new());
                    self.proc_reg = self.val.clone();
                    let RegVal::ExprSlice(ops) = &self.unev else {
                        unreachable!()
                    };
                    if ops.is_empty() {
                        State::ApplyDispatch
                    } else {
                        self.save(self.proc_reg.clone());
                        State::EvApplOperandLoop
                    }
                }
                State::EvApplOperandLoop => {
                    self.save(self.argl.clone());
                    let RegVal::ExprSlice(ops) = &self.unev else {
                        unreachable!()
                    };
                    self.exp = RegVal::Expr(ops[0].clone());
                    if ops.len() == 1 {
                        State::EvApplLastArg
                    } else {
                        self.save(self.env.clone());
                        self.save(self.unev.clone());
                        self.cont = RegVal::Cont(ContLabel::EvApplAccumulateArg);
                        State::EvalDispatch
                    }
                }
                State::EvApplAccumulateArg => {
                    self.unev = self.restore()?;
                    self.env = self.restore()?;
                    self.argl = self.restore()?;
                    let RegVal::ArgList(mut args) = self.argl.clone() else {
                        unreachable!()
                    };
                    args.push(self.val.clone());
                    self.argl = RegVal::ArgList(args);
                    let RegVal::ExprSlice(ops) = &self.unev else {
                        unreachable!()
                    };
                    self.unev = RegVal::ExprSlice(ops[1..].to_vec());
                    State::EvApplOperandLoop
                }
                State::EvApplLastArg => {
                    self.cont = RegVal::Cont(ContLabel::EvApplAccumLastArg);
                    State::EvalDispatch
                }
                State::EvApplAccumLastArg => {
                    self.argl = self.restore()?;
                    let RegVal::ArgList(mut args) = self.argl.clone() else {
                        unreachable!()
                    };
                    args.push(self.val.clone());
                    self.argl = RegVal::ArgList(args);
                    self.proc_reg = self.restore()?;
                    State::ApplyDispatch
                }
                State::ApplyDispatch => match self.proc_reg.clone() {
                    RegVal::Primitive(op) => {
                        let RegVal::ArgList(args) = &self.argl else {
                            unreachable!()
                        };
                        self.val = self.apply_primitive(&op, args)?;
                        self.cont = self.restore()?;
                        self.continue_to()?
                    }
                    RegVal::Compound { params, body, env } => {
                        let RegVal::ArgList(args) = self.argl.clone() else {
                            unreachable!()
                        };
                        if params.len() != args.len() {
                            return Err("arity mismatch".to_string());
                        }
                        let mut bindings = BTreeMap::new();
                        for (p, a) in params.into_iter().zip(args) {
                            bindings.insert(p, a);
                        }
                        self.env = RegVal::Env(Rc::new(RefCell::new(Frame {
                            bindings,
                            parent: Some(env),
                        })));
                        self.unev = RegVal::ExprSlice(body);
                        State::EvSequence
                    }
                    other => return Err(format!("unknown procedure type: {other:?}")),
                },
                State::EvSequence => {
                    let RegVal::ExprSlice(exprs) = &self.unev else {
                        unreachable!()
                    };
                    self.exp = RegVal::Expr(exprs[0].clone());
                    if exprs.len() == 1 {
                        State::EvSequenceLastExp
                    } else {
                        self.save(self.unev.clone());
                        self.save(self.env.clone());
                        self.cont = RegVal::Cont(ContLabel::EvSequenceContinue);
                        State::EvalDispatch
                    }
                }
                State::EvSequenceContinue => {
                    self.env = self.restore()?;
                    self.unev = self.restore()?;
                    let RegVal::ExprSlice(exprs) = &self.unev else {
                        unreachable!()
                    };
                    self.unev = RegVal::ExprSlice(exprs[1..].to_vec());
                    State::EvSequence
                }
                State::EvSequenceLastExp => {
                    self.cont = self.restore()?;
                    State::EvalDispatch
                }
                State::EvIfDecide => {
                    self.cont = self.restore()?;
                    self.env = self.restore()?;
                    self.exp = self.restore()?;
                    let Expr::List(items) = self.exp.as_expr()?.clone() else {
                        unreachable!()
                    };
                    let truthy = !matches!(self.val, RegVal::Bool(false));
                    if truthy {
                        self.exp = RegVal::Expr(items[2].clone());
                    } else if items.len() > 3 {
                        self.exp = RegVal::Expr(items[3].clone());
                    } else {
                        self.val = RegVal::Bool(false);
                        return self.eval_next_after_empty_if();
                    }
                    State::EvalDispatch
                }
                State::EvAssignment1 => {
                    self.cont = self.restore()?;
                    self.env = self.restore()?;
                    self.unev = self.restore()?;
                    let RegVal::Sym(name) = &self.unev else {
                        unreachable!()
                    };
                    let env = self.env.as_env()?;
                    self.set_var(name, self.val.clone(), &env)?;
                    self.val = RegVal::Sym("ok".into());
                    self.continue_to()?
                }
                State::EvDefinition1 => {
                    self.cont = self.restore()?;
                    self.env = self.restore()?;
                    self.unev = self.restore()?;
                    let RegVal::Sym(name) = &self.unev else {
                        unreachable!()
                    };
                    let env = self.env.as_env()?;
                    env.borrow_mut()
                        .bindings
                        .insert(name.clone(), self.val.clone());
                    self.val = RegVal::Sym("ok".into());
                    self.continue_to()?
                }
            };
        }
    }

    fn eval_next_after_empty_if(&self) -> Result<RegVal, String> {
        Ok(self.val.clone())
    }

    fn run_program(&mut self, src: &str) -> Result<RegVal, String> {
        let exprs = parse_program(src)?;
        let mut last = RegVal::Empty;
        for e in exprs {
            last = self.eval_expr(e)?;
        }
        Ok(last)
    }
}

fn main() {
    let mut vm = ExplicitControlEvaluator::new();
    vm.run_program(
        "
        (define (factorial n)
          (if (= n 1)
              1
              (* (factorial (- n 1)) n)))
        (define (sum-iter n acc)
          (if (= n 0)
              acc
              (sum-iter (- n 1) (+ acc n))))
        ",
    )
    .unwrap();

    vm.reset_stats();
    let f6 = vm.run_program("(factorial 6)").unwrap().as_num().unwrap();
    println!(
        "factorial(6) = {f6}, pushes = {}, max_depth = {}",
        vm.stats.total_pushes, vm.stats.max_depth
    );

    vm.reset_stats();
    let s10 = vm.run_program("(sum-iter 10 0)").unwrap().as_num().unwrap();
    let d10 = vm.stats.max_depth;
    vm.reset_stats();
    let s500 = vm
        .run_program("(sum-iter 500 0)")
        .unwrap()
        .as_num()
        .unwrap();
    let d500 = vm.stats.max_depth;
    println!("sum(10)={s10} depth={d10}, sum(500)={s500} depth={d500} (tail-recursive constant)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_factorial_matches_exact_eceval_stack_formulas() {
        let mut vm = ExplicitControlEvaluator::new();
        vm.run_program(
            "
            (define (factorial n)
              (if (= n 1)
                  1
                  (* (factorial (- n 1)) n)))
            ",
        )
        .unwrap();
        let mut expected = 1_i64;
        for n in 1..=8 {
            expected *= n;
            vm.reset_stats();
            let call = format!("(factorial {n})");
            let out = vm.run_program(&call).unwrap().as_num().unwrap();
            assert_eq!(out, expected);
            assert_eq!(vm.stats.total_pushes, 32 * n as usize - 16);
            assert_eq!(vm.stats.max_depth, 5 * n as usize + 3);
            assert_eq!(vm.stats.current_depth, 0);
        }
    }

    #[test]
    fn tail_recursive_sequence_keeps_constant_max_stack_depth() {
        let mut vm = ExplicitControlEvaluator::new();
        vm.run_program(
            "
            (define (sum-iter n acc)
              (if (= n 0)
                  acc
                  (sum-iter (- n 1) (+ acc n))))
            (define count 10)
            (begin
              (set! count (+ count 5))
              count)
            ",
        )
        .unwrap();

        vm.reset_stats();
        let r10 = vm.run_program("(sum-iter 10 0)").unwrap().as_num().unwrap();
        let depth10 = vm.stats.max_depth;

        vm.reset_stats();
        let r500 = vm
            .run_program("(sum-iter 500 0)")
            .unwrap()
            .as_num()
            .unwrap();
        let depth500 = vm.stats.max_depth;

        assert_eq!(r10, 55);
        assert_eq!(r500, 125_250);
        assert_eq!(depth10, depth500);
        assert_eq!(vm.stats.current_depth, 0);
    }
}
