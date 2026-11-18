use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Reg {
    Env,
    Val,
    Proc,
    Argl,
    Continue,
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
struct LexicalAddr {
    frame: usize,
    disp: usize,
}

#[derive(Clone, Debug)]
enum Instr {
    Label(String),
    AssignConstNum(Reg, i64),
    AssignConstBool(Reg, bool),
    AssignLabel(Reg, String),
    AssignLookupVar(Reg, String),
    AssignLexicalLookup(Reg, LexicalAddr),
    AssignMakeCompiledProc(Reg, String),
    AssignEmptyArglist(Reg),
    AssignConsArg(Reg, Reg, Reg),
    AssignListArg(Reg, Reg),
    AssignCompiledProcEnv(Reg, Reg),
    AssignExtendEnv(Reg, Vec<String>, Reg, Reg),
    AssignCompiledProcEntry(Reg, Reg),
    DefineVar(String, Reg),
    TestPrimitiveApp(Reg),
    TestFalse(Reg),
    ApplyPrimitive(Reg, Reg, Reg),
    Branch(String),
    GotoLabel(String),
    GotoReg(Reg),
    Save(Reg),
    Restore(Reg),
}

#[derive(Clone, Debug)]
struct InstructionSequence {
    needs: BTreeSet<Reg>,
    modifies: BTreeSet<Reg>,
    statements: Vec<Instr>,
}

impl InstructionSequence {
    fn new(needs: &[Reg], modifies: &[Reg], statements: Vec<Instr>) -> Self {
        Self {
            needs: needs.iter().copied().collect(),
            modifies: modifies.iter().copied().collect(),
            statements,
        }
    }

    fn label(name: String) -> Self {
        Self::new(&[], &[], vec![Instr::Label(name)])
    }
}

fn append_instruction_sequences(seqs: Vec<InstructionSequence>) -> InstructionSequence {
    let mut acc = InstructionSequence::new(&[], &[], Vec::new());
    for seq in seqs {
        for r in &seq.needs {
            if !acc.modifies.contains(r) {
                acc.needs.insert(*r);
            }
        }
        for r in &seq.modifies {
            acc.modifies.insert(*r);
        }
        acc.statements.extend(seq.statements);
    }
    acc
}

fn preserving(
    regs: &[Reg],
    mut seq1: InstructionSequence,
    seq2: InstructionSequence,
) -> InstructionSequence {
    for &reg in regs.iter().rev() {
        if seq2.needs.contains(&reg) && seq1.modifies.contains(&reg) {
            let mut wrapped = Vec::with_capacity(seq1.statements.len() + 2);
            wrapped.push(Instr::Save(reg));
            wrapped.extend(seq1.statements);
            wrapped.push(Instr::Restore(reg));
            seq1.needs.insert(reg);
            seq1.modifies.remove(&reg);
            seq1.statements = wrapped;
        }
    }
    append_instruction_sequences(vec![seq1, seq2])
}

fn tack_on_instruction_sequence(
    mut seq: InstructionSequence,
    body_seq: InstructionSequence,
) -> InstructionSequence {
    seq.statements.extend(body_seq.statements);
    seq
}

fn parallel_instruction_sequences(
    seq1: InstructionSequence,
    seq2: InstructionSequence,
) -> InstructionSequence {
    let mut needs = seq1.needs;
    needs.extend(seq2.needs);
    let mut modifies = seq1.modifies;
    modifies.extend(seq2.modifies);
    let mut statements = seq1.statements;
    statements.extend(seq2.statements);
    InstructionSequence {
        needs,
        modifies,
        statements,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Linkage {
    Next,
    Return,
    Label(String),
}

type CompileEnv = Vec<Vec<String>>;

struct Compiler {
    label_counter: usize,
    all_regs: Vec<Reg>,
}

impl Compiler {
    fn new() -> Self {
        Self {
            label_counter: 0,
            all_regs: vec![Reg::Env, Reg::Proc, Reg::Argl, Reg::Continue, Reg::Val],
        }
    }

    fn make_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        format!("{prefix}{}", self.label_counter)
    }

    fn find_variable(&self, name: &str, ctenv: &CompileEnv) -> Option<LexicalAddr> {
        for (frame_idx, frame_vars) in ctenv.iter().enumerate() {
            if let Some(disp_idx) = frame_vars.iter().position(|v| v == name) {
                return Some(LexicalAddr {
                    frame: frame_idx,
                    disp: disp_idx,
                });
            }
        }
        None
    }

    fn compile_linkage(&self, linkage: &Linkage) -> InstructionSequence {
        match linkage {
            Linkage::Next => InstructionSequence::new(&[], &[], Vec::new()),
            Linkage::Return => {
                InstructionSequence::new(&[Reg::Continue], &[], vec![Instr::GotoReg(Reg::Continue)])
            }
            Linkage::Label(lbl) => {
                InstructionSequence::new(&[], &[], vec![Instr::GotoLabel(lbl.clone())])
            }
        }
    }

    fn end_with_linkage(&self, linkage: &Linkage, seq: InstructionSequence) -> InstructionSequence {
        preserving(&[Reg::Continue], seq, self.compile_linkage(linkage))
    }

    fn compile(
        &mut self,
        expr: &Expr,
        target: Reg,
        linkage: Linkage,
        ctenv: &CompileEnv,
    ) -> Result<InstructionSequence, String> {
        match expr {
            Expr::Num(n) => Ok(self.end_with_linkage(
                &linkage,
                InstructionSequence::new(&[], &[target], vec![Instr::AssignConstNum(target, *n)]),
            )),
            Expr::Bool(b) => Ok(self.end_with_linkage(
                &linkage,
                InstructionSequence::new(&[], &[target], vec![Instr::AssignConstBool(target, *b)]),
            )),
            Expr::Sym(s) => {
                let instr = if let Some(addr) = self.find_variable(s, ctenv) {
                    Instr::AssignLexicalLookup(target, addr)
                } else {
                    Instr::AssignLookupVar(target, s.clone())
                };
                Ok(self.end_with_linkage(
                    &linkage,
                    InstructionSequence::new(&[Reg::Env], &[target], vec![instr]),
                ))
            }
            Expr::List(items) => {
                if items.is_empty() {
                    return Err("empty application".into());
                }
                if let Expr::Sym(head) = &items[0] {
                    match head.as_str() {
                        "if" => return self.compile_if(items, target, linkage, ctenv),
                        "begin" => {
                            return self.compile_sequence(&items[1..], target, linkage, ctenv)
                        }
                        "lambda" => return self.compile_lambda(items, target, linkage, ctenv),
                        "define" => return self.compile_definition(items, target, linkage, ctenv),
                        _ => {}
                    }
                }
                self.compile_application(items, target, linkage, ctenv)
            }
        }
    }

    fn compile_if(
        &mut self,
        items: &[Expr],
        target: Reg,
        linkage: Linkage,
        ctenv: &CompileEnv,
    ) -> Result<InstructionSequence, String> {
        let t_branch = self.make_label("true-branch");
        let f_branch = self.make_label("false-branch");
        let after_if = self.make_label("after-if");
        let consequent_linkage = match &linkage {
            Linkage::Next => Linkage::Label(after_if.clone()),
            other => other.clone(),
        };
        let p_code = self.compile(&items[1], Reg::Val, Linkage::Next, ctenv)?;
        let c_code = self.compile(&items[2], target, consequent_linkage, ctenv)?;
        let a_code = if items.len() > 3 {
            self.compile(&items[3], target, linkage.clone(), ctenv)?
        } else {
            self.compile(&Expr::Bool(false), target, linkage.clone(), ctenv)?
        };
        let branch_block = parallel_instruction_sequences(
            append_instruction_sequences(vec![
                InstructionSequence::label(t_branch.clone()),
                c_code,
            ]),
            append_instruction_sequences(vec![
                InstructionSequence::label(f_branch.clone()),
                a_code,
            ]),
        );
        Ok(preserving(
            &[Reg::Env, Reg::Continue],
            p_code,
            append_instruction_sequences(vec![
                InstructionSequence::new(
                    &[Reg::Val],
                    &[],
                    vec![
                        Instr::TestFalse(Reg::Val),
                        Instr::Branch(f_branch),
                        Instr::GotoLabel(t_branch),
                    ],
                ),
                branch_block,
                InstructionSequence::label(after_if),
            ]),
        ))
    }

    fn compile_sequence(
        &mut self,
        exprs: &[Expr],
        target: Reg,
        linkage: Linkage,
        ctenv: &CompileEnv,
    ) -> Result<InstructionSequence, String> {
        if exprs.len() == 1 {
            return self.compile(&exprs[0], target, linkage, ctenv);
        }
        let first = self.compile(&exprs[0], target, Linkage::Next, ctenv)?;
        let rest = self.compile_sequence(&exprs[1..], target, linkage, ctenv)?;
        Ok(preserving(&[Reg::Env, Reg::Continue], first, rest))
    }

    fn compile_definition(
        &mut self,
        items: &[Expr],
        target: Reg,
        linkage: Linkage,
        ctenv: &CompileEnv,
    ) -> Result<InstructionSequence, String> {
        let (name, val_expr) = match &items[1] {
            Expr::Sym(s) => (s.clone(), items[2].clone()),
            Expr::List(sig) => {
                let n = match &sig[0] {
                    Expr::Sym(s) => s.clone(),
                    _ => return Err("proc name must be symbol".into()),
                };
                let mut lam = vec![Expr::Sym("lambda".into()), Expr::List(sig[1..].to_vec())];
                lam.extend(items[2..].iter().cloned());
                (n, Expr::List(lam))
            }
            _ => return Err("malformed define".into()),
        };
        let get_val = self.compile(&val_expr, Reg::Val, Linkage::Next, ctenv)?;
        Ok(self.end_with_linkage(
            &linkage,
            preserving(
                &[Reg::Env],
                get_val,
                InstructionSequence::new(
                    &[Reg::Env, Reg::Val],
                    &[target],
                    vec![
                        Instr::DefineVar(name, Reg::Val),
                        Instr::AssignConstBool(target, true),
                    ],
                ),
            ),
        ))
    }

    fn compile_lambda(
        &mut self,
        items: &[Expr],
        target: Reg,
        linkage: Linkage,
        ctenv: &CompileEnv,
    ) -> Result<InstructionSequence, String> {
        let proc_entry = self.make_label("entry");
        let after_lambda = self.make_label("after-lambda");
        let lambda_linkage = match &linkage {
            Linkage::Next => Linkage::Label(after_lambda.clone()),
            other => other.clone(),
        };
        let params = match &items[1] {
            Expr::List(ps) => ps
                .iter()
                .map(|p| match p {
                    Expr::Sym(s) => Ok(s.clone()),
                    _ => Err("param must be symbol".to_string()),
                })
                .collect::<Result<Vec<_>, String>>()?,
            _ => return Err("malformed lambda params".into()),
        };
        let mut child_env = vec![params.clone()];
        child_env.extend(ctenv.iter().cloned());
        let body = self.compile_sequence(&items[2..], Reg::Val, Linkage::Return, &child_env)?;
        let header = InstructionSequence::new(
            &[Reg::Env],
            &[target],
            vec![Instr::AssignMakeCompiledProc(target, proc_entry.clone())],
        );
        let body_seq = append_instruction_sequences(vec![
            InstructionSequence::new(
                &[Reg::Proc, Reg::Argl],
                &[Reg::Env],
                vec![
                    Instr::Label(proc_entry),
                    Instr::AssignCompiledProcEnv(Reg::Env, Reg::Proc),
                    Instr::AssignExtendEnv(Reg::Env, params, Reg::Argl, Reg::Env),
                ],
            ),
            body,
        ]);
        let combined =
            tack_on_instruction_sequence(self.end_with_linkage(&lambda_linkage, header), body_seq);
        Ok(append_instruction_sequences(vec![
            combined,
            InstructionSequence::label(after_lambda),
        ]))
    }

    fn construct_arglist(
        &mut self,
        operands: &[Expr],
        ctenv: &CompileEnv,
    ) -> Result<InstructionSequence, String> {
        if operands.is_empty() {
            return Ok(InstructionSequence::new(
                &[],
                &[Reg::Argl],
                vec![Instr::AssignEmptyArglist(Reg::Argl)],
            ));
        }
        // Reverse order (last to first) as in SICP §5.5.3 so first arg ends at front
        let mut rev_codes: Vec<InstructionSequence> = Vec::with_capacity(operands.len());
        for op in operands.iter().rev() {
            rev_codes.push(self.compile(op, Reg::Val, Linkage::Next, ctenv)?);
        }
        let last_code = rev_codes.remove(0);
        let mut seq = append_instruction_sequences(vec![
            last_code,
            InstructionSequence::new(
                &[Reg::Val],
                &[Reg::Argl],
                vec![Instr::AssignListArg(Reg::Argl, Reg::Val)],
            ),
        ]);
        for next_code in rev_codes {
            let step = preserving(
                &[Reg::Argl],
                next_code,
                InstructionSequence::new(
                    &[Reg::Val, Reg::Argl],
                    &[Reg::Argl],
                    vec![Instr::AssignConsArg(Reg::Argl, Reg::Val, Reg::Argl)],
                ),
            );
            seq = preserving(&[Reg::Env], seq, step);
        }
        Ok(seq)
    }

    fn compile_application(
        &mut self,
        items: &[Expr],
        target: Reg,
        linkage: Linkage,
        ctenv: &CompileEnv,
    ) -> Result<InstructionSequence, String> {
        let proc_code = self.compile(&items[0], Reg::Proc, Linkage::Next, ctenv)?;
        let arg_code = self.construct_arglist(&items[1..], ctenv)?;
        let call_code = self.compile_procedure_call(target, linkage);
        Ok(preserving(
            &[Reg::Env, Reg::Continue],
            proc_code,
            preserving(&[Reg::Proc, Reg::Continue], arg_code, call_code),
        ))
    }

    fn compile_procedure_call(&mut self, target: Reg, linkage: Linkage) -> InstructionSequence {
        let prim_branch = self.make_label("primitive-branch");
        let comp_branch = self.make_label("compiled-branch");
        let after_call = self.make_label("after-call");
        let compiled_linkage = match &linkage {
            Linkage::Next => Linkage::Label(after_call.clone()),
            other => other.clone(),
        };

        let prim_seq = append_instruction_sequences(vec![
            InstructionSequence::label(prim_branch.clone()),
            self.end_with_linkage(
                &linkage,
                InstructionSequence::new(
                    &[Reg::Proc, Reg::Argl],
                    &[target],
                    vec![Instr::ApplyPrimitive(target, Reg::Proc, Reg::Argl)],
                ),
            ),
        ]);

        let comp_seq = append_instruction_sequences(vec![
            InstructionSequence::label(comp_branch.clone()),
            self.compile_proc_appl(target, compiled_linkage),
        ]);

        append_instruction_sequences(vec![
            InstructionSequence::new(
                &[Reg::Proc],
                &[],
                vec![
                    Instr::TestPrimitiveApp(Reg::Proc),
                    Instr::Branch(prim_branch),
                    Instr::GotoLabel(comp_branch),
                ],
            ),
            parallel_instruction_sequences(comp_seq, prim_seq),
            InstructionSequence::label(after_call),
        ])
    }

    fn compile_proc_appl(&mut self, target: Reg, linkage: Linkage) -> InstructionSequence {
        let all_mod = self.all_regs.clone();
        match (&target, &linkage) {
            (Reg::Val, Linkage::Label(ret_lbl)) => InstructionSequence::new(
                &[Reg::Proc],
                &all_mod,
                vec![
                    Instr::AssignLabel(Reg::Continue, ret_lbl.clone()),
                    Instr::AssignCompiledProcEntry(Reg::Val, Reg::Proc),
                    Instr::GotoReg(Reg::Val),
                ],
            ),
            (Reg::Val, Linkage::Return) => InstructionSequence::new(
                &[Reg::Proc, Reg::Continue],
                &all_mod,
                vec![
                    Instr::AssignCompiledProcEntry(Reg::Val, Reg::Proc),
                    Instr::GotoReg(Reg::Val),
                ],
            ),
            (Reg::Val, Linkage::Next) => unreachable!("handled by after-call label"),
            (_, _) => unreachable!("application target is Val in this model"),
        }
    }
}

#[derive(Clone, Debug)]
struct Frame {
    lexical_slots: Vec<RuntimeVal>,
    named: BTreeMap<String, RuntimeVal>,
    parent: Option<Env>,
}

type Env = Rc<RefCell<Frame>>;

#[derive(Clone, Debug)]
enum RuntimeVal {
    Num(i64),
    Bool(bool),
    Label(usize),
    ArgList(Vec<RuntimeVal>),
    Primitive(String),
    CompiledProc { entry: usize, env: Env },
}

impl RuntimeVal {
    fn as_num(&self) -> Result<i64, String> {
        match self {
            RuntimeVal::Num(n) => Ok(*n),
            other => Err(format!("expected number, got {other:?}")),
        }
    }
}

struct CompiledVM {
    global_env: Env,
    code: Vec<Instr>,
    labels: BTreeMap<String, usize>,
    total_pushes: usize,
    current_depth: usize,
    max_depth: usize,
}

impl CompiledVM {
    fn new() -> Self {
        let mut named = BTreeMap::new();
        for p in ["+", "-", "*", "=", "<", ">"] {
            named.insert(p.to_string(), RuntimeVal::Primitive(p.to_string()));
        }
        Self {
            global_env: Rc::new(RefCell::new(Frame {
                lexical_slots: Vec::new(),
                named,
                parent: None,
            })),
            code: Vec::new(),
            labels: BTreeMap::new(),
            total_pushes: 0,
            current_depth: 0,
            max_depth: 0,
        }
    }

    fn reset_stats(&mut self) {
        self.total_pushes = 0;
        self.current_depth = 0;
        self.max_depth = 0;
    }

    fn run_compiled(&mut self, seq: &InstructionSequence) -> Result<RuntimeVal, String> {
        let start_pc = self.code.len();
        for stmt in &seq.statements {
            if let Instr::Label(lbl) = stmt {
                self.labels.insert(lbl.clone(), self.code.len());
            } else {
                self.code.push(stmt.clone());
            }
        }
        let end_pc = self.code.len();
        let mut env_reg = Rc::clone(&self.global_env);
        let mut val_reg = RuntimeVal::Bool(false);
        let mut proc_reg = RuntimeVal::Bool(false);
        let mut argl_reg = RuntimeVal::ArgList(Vec::new());
        let mut cont_reg = RuntimeVal::Label(end_pc);
        let mut stack: Vec<(Reg, Box<dyn std::any::Any>)> = Vec::new();
        let mut flag = false;
        let mut pc = start_pc;

        let reg_get = |r: Reg,
                       val_r: &RuntimeVal,
                       proc_r: &RuntimeVal,
                       argl_r: &RuntimeVal,
                       cont_r: &RuntimeVal|
         -> RuntimeVal {
            match r {
                Reg::Val => val_r.clone(),
                Reg::Proc => proc_r.clone(),
                Reg::Argl => argl_r.clone(),
                Reg::Continue => cont_r.clone(),
                Reg::Env => unreachable!(),
            }
        };

        while pc != end_pc {
            let ins = self.code[pc].clone();
            match &ins {
                Instr::Label(_) => unreachable!(),
                Instr::AssignConstNum(r, n) => {
                    let v = RuntimeVal::Num(*n);
                    match r {
                        Reg::Val => val_reg = v,
                        Reg::Proc => proc_reg = v,
                        _ => unreachable!(),
                    }
                    pc += 1;
                }
                Instr::AssignConstBool(r, b) => {
                    let v = RuntimeVal::Bool(*b);
                    match r {
                        Reg::Val => val_reg = v,
                        Reg::Proc => proc_reg = v,
                        _ => unreachable!(),
                    }
                    pc += 1;
                }
                Instr::AssignLabel(r, lbl) => {
                    let v = RuntimeVal::Label(self.labels[lbl]);
                    match r {
                        Reg::Continue => cont_reg = v,
                        Reg::Val => val_reg = v,
                        _ => unreachable!(),
                    }
                    pc += 1;
                }
                Instr::AssignLookupVar(r, name) => {
                    let mut cur = Some(Rc::clone(&env_reg));
                    let mut found = None;
                    while let Some(frame) = cur {
                        if let Some(v) = frame.borrow().named.get(name) {
                            found = Some(v.clone());
                            break;
                        }
                        cur = frame.borrow().parent.clone();
                    }
                    let v = found.ok_or_else(|| format!("unbound variable: {name}"))?;
                    match r {
                        Reg::Val => val_reg = v,
                        Reg::Proc => proc_reg = v,
                        _ => unreachable!(),
                    }
                    pc += 1;
                }
                Instr::AssignLexicalLookup(r, addr) => {
                    let mut cur = Rc::clone(&env_reg);
                    for _ in 0..addr.frame {
                        let next = cur.borrow().parent.clone().expect("valid lexical depth");
                        cur = next;
                    }
                    let v = cur.borrow().lexical_slots[addr.disp].clone();
                    match r {
                        Reg::Val => val_reg = v,
                        Reg::Proc => proc_reg = v,
                        _ => unreachable!(),
                    }
                    pc += 1;
                }
                Instr::AssignMakeCompiledProc(r, entry_lbl) => {
                    let v = RuntimeVal::CompiledProc {
                        entry: self.labels[entry_lbl],
                        env: Rc::clone(&env_reg),
                    };
                    match r {
                        Reg::Val => val_reg = v,
                        Reg::Proc => proc_reg = v,
                        _ => unreachable!(),
                    }
                    pc += 1;
                }
                Instr::AssignEmptyArglist(Reg::Argl) => {
                    argl_reg = RuntimeVal::ArgList(Vec::new());
                    pc += 1;
                }
                Instr::AssignListArg(Reg::Argl, src) => {
                    let v = reg_get(*src, &val_reg, &proc_reg, &argl_reg, &cont_reg);
                    argl_reg = RuntimeVal::ArgList(vec![v]);
                    pc += 1;
                }
                Instr::AssignConsArg(Reg::Argl, head, tail) => {
                    let h = reg_get(*head, &val_reg, &proc_reg, &argl_reg, &cont_reg);
                    let RuntimeVal::ArgList(mut list) =
                        reg_get(*tail, &val_reg, &proc_reg, &argl_reg, &cont_reg)
                    else {
                        unreachable!()
                    };
                    let mut next = vec![h];
                    next.append(&mut list);
                    argl_reg = RuntimeVal::ArgList(next);
                    pc += 1;
                }
                Instr::AssignCompiledProcEnv(Reg::Env, Reg::Proc) => {
                    let RuntimeVal::CompiledProc { env, .. } = &proc_reg else {
                        return Err("not a compiled procedure".into());
                    };
                    env_reg = Rc::clone(env);
                    pc += 1;
                }
                Instr::AssignExtendEnv(Reg::Env, params, Reg::Argl, Reg::Env) => {
                    let RuntimeVal::ArgList(args) = &argl_reg else {
                        unreachable!()
                    };
                    if params.len() != args.len() {
                        return Err("arity mismatch".into());
                    }
                    let mut named = BTreeMap::new();
                    for (p, a) in params.iter().zip(args.iter()) {
                        named.insert(p.clone(), a.clone());
                    }
                    env_reg = Rc::new(RefCell::new(Frame {
                        lexical_slots: args.clone(),
                        named,
                        parent: Some(Rc::clone(&env_reg)),
                    }));
                    pc += 1;
                }
                Instr::AssignCompiledProcEntry(Reg::Val, Reg::Proc) => {
                    let RuntimeVal::CompiledProc { entry, .. } = &proc_reg else {
                        return Err("not a compiled procedure".into());
                    };
                    val_reg = RuntimeVal::Label(*entry);
                    pc += 1;
                }
                Instr::DefineVar(name, src) => {
                    let v = reg_get(*src, &val_reg, &proc_reg, &argl_reg, &cont_reg);
                    env_reg.borrow_mut().named.insert(name.clone(), v);
                    pc += 1;
                }
                Instr::TestPrimitiveApp(Reg::Proc) => {
                    flag = matches!(proc_reg, RuntimeVal::Primitive(_));
                    pc += 1;
                }
                Instr::TestFalse(Reg::Val) => {
                    flag = matches!(val_reg, RuntimeVal::Bool(false));
                    pc += 1;
                }
                Instr::ApplyPrimitive(Reg::Val, Reg::Proc, Reg::Argl) => {
                    let RuntimeVal::Primitive(op) = &proc_reg else {
                        unreachable!()
                    };
                    let RuntimeVal::ArgList(args) = &argl_reg else {
                        unreachable!()
                    };
                    val_reg = match op.as_str() {
                        "+" => RuntimeVal::Num(args.iter().map(|a| a.as_num().unwrap()).sum()),
                        "*" => RuntimeVal::Num(args.iter().map(|a| a.as_num().unwrap()).product()),
                        "-" => RuntimeVal::Num(args[0].as_num()? - args[1].as_num()?),
                        "=" => RuntimeVal::Bool(args[0].as_num()? == args[1].as_num()?),
                        "<" => RuntimeVal::Bool(args[0].as_num()? < args[1].as_num()?),
                        ">" => RuntimeVal::Bool(args[0].as_num()? > args[1].as_num()?),
                        _ => return Err(format!("unknown primitive {op}")),
                    };
                    pc += 1;
                }
                Instr::Branch(lbl) => {
                    pc = if flag { self.labels[lbl] } else { pc + 1 };
                }
                Instr::GotoLabel(lbl) => pc = self.labels[lbl],
                Instr::GotoReg(r) => {
                    let RuntimeVal::Label(target) =
                        reg_get(*r, &val_reg, &proc_reg, &argl_reg, &cont_reg)
                    else {
                        return Err("goto non-label".into());
                    };
                    pc = target;
                }
                Instr::Save(r) => {
                    self.total_pushes += 1;
                    self.current_depth += 1;
                    if self.current_depth > self.max_depth {
                        self.max_depth = self.current_depth;
                    }
                    match r {
                        Reg::Env => stack.push((Reg::Env, Box::new(Rc::clone(&env_reg)))),
                        _ => {
                            let v = reg_get(*r, &val_reg, &proc_reg, &argl_reg, &cont_reg);
                            stack.push((*r, Box::new(v)));
                        }
                    }
                    pc += 1;
                }
                Instr::Restore(r) => {
                    let (saved_r, boxed) = stack.pop().ok_or_else(|| "empty stack".to_string())?;
                    assert_eq!(saved_r, *r);
                    self.current_depth -= 1;
                    match r {
                        Reg::Env => {
                            env_reg = *boxed.downcast::<Env>().unwrap();
                        }
                        Reg::Val => val_reg = *boxed.downcast::<RuntimeVal>().unwrap(),
                        Reg::Proc => proc_reg = *boxed.downcast::<RuntimeVal>().unwrap(),
                        Reg::Argl => argl_reg = *boxed.downcast::<RuntimeVal>().unwrap(),
                        Reg::Continue => cont_reg = *boxed.downcast::<RuntimeVal>().unwrap(),
                    }
                    pc += 1;
                }
                _ => return Err("unhandled compiled instruction variant".into()),
            }
        }
        Ok(val_reg)
    }

    fn eval_compiled(&mut self, compiler: &mut Compiler, src: &str) -> Result<RuntimeVal, String> {
        let exprs = parse_program(src)?;
        let seq = compiler.compile_sequence(&exprs, Reg::Val, Linkage::Next, &Vec::new())?;
        self.run_compiled(&seq)
    }
}

fn main() {
    let mut compiler = Compiler::new();
    let mut vm = CompiledVM::new();
    vm.eval_compiled(
        &mut compiler,
        "
        (define (factorial n)
          (if (= n 1)
              1
              (* (factorial (- n 1)) n)))
        ",
    )
    .unwrap();
    vm.reset_stats();
    let res = vm
        .eval_compiled(&mut compiler, "(factorial 6)")
        .unwrap()
        .as_num()
        .unwrap();
    println!(
        "compiled factorial(6) = {res}, pushes = {}, max_depth = {} (vs 176 pushes, depth 33 interpreted)",
        vm.total_pushes, vm.max_depth
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserving_elides_saves_on_non_interfering_operands() {
        let mut compiler = Compiler::new();
        let expr = &parse_program("(+ 10 20)").unwrap()[0];
        let seq = compiler
            .compile(expr, Reg::Val, Linkage::Next, &Vec::new())
            .unwrap();
        let save_count = seq
            .statements
            .iter()
            .filter(|s| matches!(s, Instr::Save(_)))
            .count();
        // Neither operand modifies Env or Argl, so `preserving` emits zero saves!
        assert_eq!(save_count, 0);

        let mut vm = CompiledVM::new();
        assert_eq!(vm.run_compiled(&seq).unwrap().as_num().unwrap(), 30);
    }

    #[test]
    fn lexical_addressing_and_compiled_factorial_beat_interpreter() {
        let mut compiler = Compiler::new();
        // Verify inner lambda variable resolves via lexical addressing `(frame, disp)`!
        let closure_expr = &parse_program("(lambda (x y) ((lambda (z) (+ x z)) y))").unwrap()[0];
        let closure_seq = compiler
            .compile(closure_expr, Reg::Val, Linkage::Next, &Vec::new())
            .unwrap();
        let has_lexical_x = closure_seq.statements.iter().any(|s| {
            matches!(
                s,
                Instr::AssignLexicalLookup(Reg::Val, LexicalAddr { frame: 1, disp: 0 })
            )
        });
        assert!(has_lexical_x);

        let mut vm = CompiledVM::new();
        let val = vm
            .eval_compiled(
                &mut compiler,
                "((lambda (x y) ((lambda (z) (+ x z)) y)) 11 31)",
            )
            .unwrap()
            .as_num()
            .unwrap();
        assert_eq!(val, 42);

        vm.eval_compiled(
            &mut compiler,
            "
            (define (factorial n)
              (if (= n 1)
                  1
                  (* (factorial (- n 1)) n)))
            ",
        )
        .unwrap();
        vm.reset_stats();
        let f6 = vm
            .eval_compiled(&mut compiler, "(factorial 6)")
            .unwrap()
            .as_num()
            .unwrap();
        assert_eq!(f6, 720);
        // Compiled factorial(6) pushes 6n - 4 = 32 (vs 32n - 16 = 176 in eceval)
        // and reaches max_depth 3n - 1 = 17 (vs 5n + 3 = 33 in eceval).
        assert_eq!(vm.total_pushes, 32);
        assert_eq!(vm.max_depth, 17);
    }
}
