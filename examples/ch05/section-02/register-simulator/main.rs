use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Val {
    Int(i64),
    Label(usize),
    Text(String),
}

impl Val {
    fn as_int(&self) -> Result<i64, String> {
        match self {
            Val::Int(n) => Ok(*n),
            other => Err(format!("expected Int register value, got {other:?}")),
        }
    }
}

#[derive(Clone, Debug)]
enum Source {
    Const(i64),
    Reg(String),
    Label(String),
}

#[derive(Clone, Debug)]
enum Instr {
    AssignVal(String, Source),
    AssignOp(String, String, Vec<Source>),
    Test(String, Vec<Source>),
    Branch(String),
    GotoLabel(String),
    GotoReg(String),
    Save(String),
    Restore(String),
    Perform(String, Vec<Source>),
}

#[derive(Default, Debug)]
struct StackStats {
    total_pushes: usize,
    current_depth: usize,
    max_depth: usize,
}

type OpFn = Box<dyn Fn(&[Val]) -> Result<Val, String>>;

struct RegisterMachine {
    registers: BTreeMap<String, Val>,
    stack: Vec<Val>,
    stats: StackStats,
    flag: bool,
    pc: usize,
    labels: BTreeMap<String, usize>,
    label_at_pc: BTreeMap<usize, Vec<String>>,
    instructions: Vec<Instr>,
    ops: BTreeMap<String, OpFn>,
    trace: bool,
    trace_log: Vec<String>,
    insts_executed: usize,
}

impl RegisterMachine {
    fn new(reg_names: &[&str]) -> Self {
        let mut registers = BTreeMap::new();
        for name in reg_names {
            registers.insert((*name).to_string(), Val::Int(0));
        }
        let mut m = Self {
            registers,
            stack: Vec::new(),
            stats: StackStats::default(),
            flag: false,
            pc: 0,
            labels: BTreeMap::new(),
            label_at_pc: BTreeMap::new(),
            instructions: Vec::new(),
            ops: BTreeMap::new(),
            trace: false,
            trace_log: Vec::new(),
            insts_executed: 0,
        };
        m.register_op(
            "=",
            Box::new(|args| Ok(Val::Int(i64::from(args[0] == args[1])))),
        );
        m.register_op(
            "-",
            Box::new(|args| Ok(Val::Int(args[0].as_int()? - args[1].as_int()?))),
        );
        m.register_op(
            "+",
            Box::new(|args| Ok(Val::Int(args[0].as_int()? + args[1].as_int()?))),
        );
        m.register_op(
            "*",
            Box::new(|args| Ok(Val::Int(args[0].as_int()? * args[1].as_int()?))),
        );
        m.register_op(
            "<",
            Box::new(|args| Ok(Val::Int(i64::from(args[0].as_int()? < args[1].as_int()?)))),
        );
        m
    }

    fn register_op(&mut self, name: &str, f: OpFn) {
        self.ops.insert(name.to_string(), f);
    }

    fn set_register(&mut self, name: &str, val: Val) {
        self.registers.insert(name.to_string(), val);
    }

    fn get_register(&self, name: &str) -> Result<&Val, String> {
        self.registers
            .get(name)
            .ok_or_else(|| format!("unknown register: {name}"))
    }

    fn assemble(&mut self, controller_text: &str) -> Result<(), String> {
        self.instructions.clear();
        self.labels.clear();
        self.label_at_pc.clear();
        for raw_line in controller_text.lines() {
            let line = raw_line.split(';').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            if let Some(label) = line.strip_suffix(':') {
                let pc = self.instructions.len();
                self.labels.insert(label.trim().to_string(), pc);
                self.label_at_pc
                    .entry(pc)
                    .or_default()
                    .push(label.trim().to_string());
                continue;
            }
            let instr = parse_instruction(line)?;
            self.instructions.push(instr);
        }
        // Verify all labels referenced exist
        for instr in &self.instructions {
            match instr {
                Instr::Branch(lbl) | Instr::GotoLabel(lbl) => {
                    if !self.labels.contains_key(lbl) {
                        return Err(format!("undefined label: {lbl}"));
                    }
                }
                Instr::AssignVal(_, Source::Label(lbl)) => {
                    if !self.labels.contains_key(lbl) {
                        return Err(format!("undefined label: {lbl}"));
                    }
                }
                Instr::AssignOp(_, _, srcs) | Instr::Test(_, srcs) | Instr::Perform(_, srcs) => {
                    for s in srcs {
                        if let Source::Label(lbl) = s {
                            if !self.labels.contains_key(lbl) {
                                return Err(format!("undefined label: {lbl}"));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn eval_source(&self, src: &Source) -> Result<Val, String> {
        match src {
            Source::Const(n) => Ok(Val::Int(*n)),
            Source::Reg(r) => self.get_register(r).cloned(),
            Source::Label(lbl) => self
                .labels
                .get(lbl)
                .map(|&pc| Val::Label(pc))
                .ok_or_else(|| format!("undefined label: {lbl}")),
        }
    }

    fn eval_op(&self, op_name: &str, args: &[Source]) -> Result<Val, String> {
        let f = self
            .ops
            .get(op_name)
            .ok_or_else(|| format!("unknown op: {op_name}"))?;
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.eval_source(a)?);
        }
        f(&vals)
    }

    fn start(&mut self, max_steps: usize) -> Result<(), String> {
        self.pc = 0;
        self.stack.clear();
        self.stats = StackStats::default();
        self.insts_executed = 0;
        self.trace_log.clear();

        while self.pc < self.instructions.len() {
            if self.insts_executed >= max_steps {
                return Err("step budget exceeded".to_string());
            }
            if self.trace {
                if let Some(lbls) = self.label_at_pc.get(&self.pc) {
                    for lbl in lbls {
                        self.trace_log.push(format!("label {lbl}:"));
                    }
                }
                self.trace_log
                    .push(format!("pc={}: {:?}", self.pc, self.instructions[self.pc]));
            }
            let instr = self.instructions[self.pc].clone();
            self.insts_executed += 1;
            match instr {
                Instr::AssignVal(reg, src) => {
                    let v = self.eval_source(&src)?;
                    self.set_register(&reg, v);
                    self.pc += 1;
                }
                Instr::AssignOp(reg, op, args) => {
                    let v = self.eval_op(&op, &args)?;
                    self.set_register(&reg, v);
                    self.pc += 1;
                }
                Instr::Test(op, args) => {
                    let v = self.eval_op(&op, &args)?;
                    self.flag = v.as_int()? != 0;
                    self.pc += 1;
                }
                Instr::Branch(lbl) => {
                    if self.flag {
                        self.pc = self.labels[&lbl];
                    } else {
                        self.pc += 1;
                    }
                }
                Instr::GotoLabel(lbl) => {
                    self.pc = self.labels[&lbl];
                }
                Instr::GotoReg(reg) => {
                    let Val::Label(target) = self.get_register(&reg)?.clone() else {
                        return Err(format!("register {reg} does not hold a label"));
                    };
                    self.pc = target;
                }
                Instr::Save(reg) => {
                    let v = self.get_register(&reg)?.clone();
                    self.stack.push(v);
                    self.stats.total_pushes += 1;
                    self.stats.current_depth += 1;
                    if self.stats.current_depth > self.stats.max_depth {
                        self.stats.max_depth = self.stats.current_depth;
                    }
                    self.pc += 1;
                }
                Instr::Restore(reg) => {
                    let v = self
                        .stack
                        .pop()
                        .ok_or_else(|| "restore on empty stack".to_string())?;
                    self.stats.current_depth -= 1;
                    self.set_register(&reg, v);
                    self.pc += 1;
                }
                Instr::Perform(op, args) => {
                    let _ = self.eval_op(&op, &args)?;
                    self.pc += 1;
                }
            }
        }
        Ok(())
    }
}

fn parse_source(tok: &str) -> Result<Source, String> {
    if let Some(rest) = tok.strip_prefix("const(").and_then(|s| s.strip_suffix(')')) {
        let n = rest
            .trim()
            .parse::<i64>()
            .map_err(|_| format!("invalid const: {tok}"))?;
        Ok(Source::Const(n))
    } else if let Some(rest) = tok.strip_prefix("reg(").and_then(|s| s.strip_suffix(')')) {
        Ok(Source::Reg(rest.trim().to_string()))
    } else if let Some(rest) = tok.strip_prefix("label(").and_then(|s| s.strip_suffix(')')) {
        Ok(Source::Label(rest.trim().to_string()))
    } else {
        Err(format!("invalid operand syntax: {tok}"))
    }
}

fn parse_op_call(expr: &str) -> Result<(String, Vec<Source>), String> {
    let inner = expr
        .strip_prefix("op(")
        .and_then(|s| s.strip_suffix(')'))
        .ok_or_else(|| format!("expected op(...), got {expr}"))?;
    let mut parts = inner.split_whitespace();
    let op = parts
        .next()
        .ok_or_else(|| "missing op name".to_string())?
        .to_string();
    let args = parts.map(parse_source).collect::<Result<Vec<_>, _>>()?;
    Ok((op, args))
}

fn parse_instruction(line: &str) -> Result<Instr, String> {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    let head = parts[0];
    let rest = parts.get(1).copied().unwrap_or("").trim();
    match head {
        "assign" => {
            let split: Vec<&str> = rest.splitn(2, ' ').collect();
            let reg = split[0].trim().to_string();
            let rhs = split
                .get(1)
                .ok_or_else(|| format!("missing assign rhs in {line}"))?
                .trim();
            if rhs.starts_with("op(") {
                let (op, args) = parse_op_call(rhs)?;
                Ok(Instr::AssignOp(reg, op, args))
            } else {
                Ok(Instr::AssignVal(reg, parse_source(rhs)?))
            }
        }
        "test" => {
            let (op, args) = parse_op_call(rest)?;
            Ok(Instr::Test(op, args))
        }
        "branch" => {
            let Source::Label(lbl) = parse_source(rest)? else {
                return Err(format!("branch expects label(...), got {rest}"));
            };
            Ok(Instr::Branch(lbl))
        }
        "goto" => match parse_source(rest)? {
            Source::Label(lbl) => Ok(Instr::GotoLabel(lbl)),
            Source::Reg(r) => Ok(Instr::GotoReg(r)),
            other => Err(format!("goto expects label or reg, got {other:?}")),
        },
        "save" => Ok(Instr::Save(rest.to_string())),
        "restore" => Ok(Instr::Restore(rest.to_string())),
        "perform" => {
            let (op, args) = parse_op_call(rest)?;
            Ok(Instr::Perform(op, args))
        }
        _ => Err(format!("unknown instruction: {line}")),
    }
}

const FACT_CONTROLLER: &str = "
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
";

const FIB_CONTROLLER: &str = "
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
";

fn main() {
    let mut fact = RegisterMachine::new(&["n", "val", "continue"]);
    fact.assemble(FACT_CONTROLLER).unwrap();
    fact.set_register("n", Val::Int(6));
    fact.start(10_000).unwrap();
    println!(
        "fact(6) = {:?}, pushes = {}, max_depth = {}, insts = {}",
        fact.get_register("val").unwrap().as_int().unwrap(),
        fact.stats.total_pushes,
        fact.stats.max_depth,
        fact.insts_executed
    );

    let mut fib = RegisterMachine::new(&["n", "val", "continue"]);
    fib.assemble(FIB_CONTROLLER).unwrap();
    fib.set_register("n", Val::Int(8));
    fib.start(100_000).unwrap();
    println!(
        "fib(8) = {:?}, pushes = {}, max_depth = {}, insts = {}; tag={:?}",
        fib.get_register("val").unwrap().as_int().unwrap(),
        fib.stats.total_pushes,
        fib.stats.max_depth,
        fib.insts_executed,
        Val::Text("ok".into())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_matches_exact_stack_formulas() {
        let mut machine = RegisterMachine::new(&["n", "val", "continue"]);
        machine.assemble(FACT_CONTROLLER).unwrap();
        let mut expected_fact = 1_i64;
        for n in 1..=10 {
            expected_fact *= n;
            machine.set_register("n", Val::Int(n));
            machine.start(50_000).unwrap();
            assert_eq!(
                machine.get_register("val").unwrap().as_int().unwrap(),
                expected_fact
            );
            assert_eq!(machine.stats.total_pushes, 2 * (n as usize - 1));
            assert_eq!(machine.stats.max_depth, 2 * (n as usize - 1));
            assert_eq!(machine.stats.current_depth, 0);
        }
    }

    #[test]
    fn fibonacci_and_perform_trace_work() {
        let mut fib = RegisterMachine::new(&["n", "val", "continue"]);
        fib.trace = true;
        fib.assemble(FIB_CONTROLLER).unwrap();
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (n, &exp) in expected.iter().enumerate() {
            fib.set_register("n", Val::Int(n as i64));
            fib.start(100_000).unwrap();
            assert_eq!(fib.get_register("val").unwrap().as_int().unwrap(), exp);
        }
        assert!(!fib.trace_log.is_empty());

        let mut logged = RegisterMachine::new(&["x", "out"]);
        let count_log = std::rc::Rc::new(std::cell::Cell::new(0));
        let count_c = std::rc::Rc::clone(&count_log);
        logged.register_op(
            "emit",
            Box::new(move |args| {
                count_c.set(count_c.get() + 1);
                Ok(Val::Text(format!("emitted:{:?}", args[0])))
            }),
        );
        logged
            .assemble(
                "
                assign x const(42)
                perform op(emit reg(x))
                assign out op(+ reg(x) const(1))
                ",
            )
            .unwrap();
        logged.start(100).unwrap();
        assert_eq!(count_log.get(), 1);
        assert_eq!(logged.get_register("out").unwrap().as_int().unwrap(), 43);
    }
}
