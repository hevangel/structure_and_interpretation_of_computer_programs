use std::collections::HashMap;
#[derive(Clone,Copy)]
enum Source {
    Label(&'static str),Load(usize,i64),Sub(usize,usize),
    Jump(&'static str),JumpIfZero(usize,&'static str),Halt,
}
#[derive(Clone,Copy)]
enum Op { Load(usize,i64),Sub(usize,usize),Jump(usize),JumpIfZero(usize,usize),Halt }
fn assemble(source:&[Source])->Result<Vec<Op>,String> {
    let mut labels=HashMap::new();
    let mut address=0usize;
    for item in source {
        if let Source::Label(name)=item {
            if labels.insert(*name,address).is_some() {
                return Err(format!("duplicate label {name}"));
            }
        } else {address+=1;}
    }
    let mut program=Vec::new();
    for item in source {
        let lookup=|name:&str| labels.get(name).copied()
                .ok_or_else(||format!("unknown label {name}"));
        let op=match *item {
            Source::Label(_)=>continue,
            Source::Load(reg,val)=>Op::Load(reg,val),
            Source::Sub(a,b)=>Op::Sub(a,b),
            Source::Jump(label)=>Op::Jump(lookup(label)?),
            Source::JumpIfZero(reg,label)=>Op::JumpIfZero(reg,lookup(label)?),
            Source::Halt=>Op::Halt,
        };
        program.push(op);
    }
    Ok(program)
}
fn execute(code:&[Op],limit:usize)->Result<[i64;2],String> {
    let mut regs=[0;2];let mut pc=0usize;
    for _ in 0..limit {
        let op=*code.get(pc).ok_or("invalid pc")?;
        match op {
            Op::Load(r,v)=>*regs.get_mut(r).ok_or("invalid register")?=v,
            Op::Sub(a,b)=>{
                let value=*regs.get(b).ok_or("invalid source register")?;
                *regs.get_mut(a).ok_or("invalid destination register")?-=value;
            }
            Op::Jump(next)=>{pc=next;continue;}
            Op::JumpIfZero(r,next)=>{
                if *regs.get(r).ok_or("invalid register")?==0 {pc=next;continue;}
            }
            Op::Halt=>return Ok(regs),
        }
        pc+=1;
    }
    Err("step limit exceeded".into())
}
fn program()->Vec<Source> {
    vec![Source::Load(0,5),Source::Load(1,1),
        Source::Label("loop"),Source::JumpIfZero(0,"done"),
        Source::Sub(0,1),Source::Jump("loop"),
        Source::Label("done"),Source::Halt]
}
fn main(){println!("registers: {:?}",execute(&assemble(&program()).unwrap(),100));}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn symbols_resolve_and_program_runs() {
        let code=assemble(&program()).unwrap();
        assert_eq!(code.len(),6);
        assert_eq!(execute(&code,100).unwrap(),[0,1]);
        assert!(execute(&code,1).is_err());
        assert!(assemble(&[Source::Jump("missing")]).is_err());
        assert!(assemble(&[Source::Label("a"),Source::Label("a")]).is_err());
    }
}
