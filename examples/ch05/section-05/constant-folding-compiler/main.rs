#[derive(Clone)]
enum Expr { Literal(i64),X,Add(Box<Expr>,Box<Expr>),Mul(Box<Expr>,Box<Expr>) }
#[derive(Debug)]
enum Op { Push(i64),LoadX,Add,Mul }
fn fold(e:Expr)->Expr {
    match e {
        Expr::Add(a,b)=>{
            let a=fold(*a);let b=fold(*b);
            match (&a,&b) {
                (Expr::Literal(x),Expr::Literal(y))=>Expr::Literal(x+y),
                _=>Expr::Add(Box::new(a),Box::new(b)),
            }
        }
        Expr::Mul(a,b)=>{
            let a=fold(*a);let b=fold(*b);
            match (&a,&b) {
                (Expr::Literal(x),Expr::Literal(y))=>Expr::Literal(x*y),
                _=>Expr::Mul(Box::new(a),Box::new(b)),
            }
        }
        other=>other,
    }
}
fn compile(e:&Expr,out:&mut Vec<Op>) {
    match e {
        Expr::Literal(n)=>out.push(Op::Push(*n)),
        Expr::X=>out.push(Op::LoadX),
        Expr::Add(a,b)=>{compile(a,out);compile(b,out);out.push(Op::Add);}
        Expr::Mul(a,b)=>{compile(a,out);compile(b,out);out.push(Op::Mul);}
    }
}
fn run(code:&[Op],x:i64)->Result<i64,&'static str> {
    let mut stack=Vec::new();
    for op in code {
        match op {
            Op::Push(v)=>stack.push(*v),
            Op::LoadX=>stack.push(x),
            Op::Add|Op::Mul=>{
                let b=stack.pop().ok_or("missing operand")?;
                let a=stack.pop().ok_or("missing operand")?;
                stack.push(match op {Op::Add=>a+b,Op::Mul=>a*b,_=>unreachable!()});
            }
        }
    }
    if stack.len()!=1 {return Err("invalid stack depth");}
    stack.pop().ok_or("no result")
}
fn expression()->Expr {
    // 2*3+x: its constant subtree can be evaluated before runtime.
    Expr::Add(
        Box::new(Expr::Mul(Box::new(Expr::Literal(2)),Box::new(Expr::Literal(3)))),
        Box::new(Expr::X))
}
fn main() {
    let original=expression();
    let mut before=Vec::new();compile(&original,&mut before);
    let mut after=Vec::new();compile(&fold(original),&mut after);
    println!("before={} opcodes, after={} opcodes",before.len(),after.len());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fewer_instructions_same_result() {
        let expr=expression();let mut raw=Vec::new();compile(&expr,&mut raw);
        let mut optimized=Vec::new();compile(&fold(expr),&mut optimized);
        assert_eq!(raw.len(),5);
        assert_eq!(optimized.len(),3);
        for x in -5..=5 {assert_eq!(run(&raw,x),run(&optimized,x));}
    }
}
