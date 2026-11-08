enum Expr { Value(i64), Add(Box<Expr>,Box<Expr>), Mul(Box<Expr>,Box<Expr>) }
#[derive(Debug)]
enum Instr { Push(i64), Add, Mul }
fn compile(e:&Expr,out:&mut Vec<Instr>) {
    match e {
        Expr::Value(n)=>out.push(Instr::Push(*n)),
        Expr::Add(a,b)=>{compile(a,out);compile(b,out);out.push(Instr::Add);}
        Expr::Mul(a,b)=>{compile(a,out);compile(b,out);out.push(Instr::Mul);}
    }
}
fn run(program:&[Instr])->Result<i64,&'static str> {
    let mut stack=Vec::new();
    for ins in program {
        match ins {
            Instr::Push(n)=>stack.push(*n),
            Instr::Add|Instr::Mul=>{
                let rhs=stack.pop().ok_or("stack underflow")?;
                let lhs=stack.pop().ok_or("stack underflow")?;
                stack.push(match ins {
                    Instr::Add=>lhs+rhs,Instr::Mul=>lhs*rhs,_=>unreachable!(),
                });
            }
        }
    }
    if stack.len()!=1{return Err("program did not leave one result");}
    stack.pop().ok_or("missing result")
}
fn main() {
    let e=Expr::Add(Box::new(Expr::Value(2)),
        Box::new(Expr::Mul(Box::new(Expr::Value(3)),Box::new(Expr::Value(4)))));
    let mut program=Vec::new();compile(&e,&mut program);
    println!("{:?} -> {:?}",program,run(&program));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn generated_code() {
        let e=Expr::Add(Box::new(Expr::Value(2)),
            Box::new(Expr::Mul(Box::new(Expr::Value(3)),Box::new(Expr::Value(4)))));
        let mut p=Vec::new();compile(&e,&mut p);
        assert_eq!(run(&p).unwrap(),14);
        assert!(run(&[Instr::Add]).is_err());
    }
}
