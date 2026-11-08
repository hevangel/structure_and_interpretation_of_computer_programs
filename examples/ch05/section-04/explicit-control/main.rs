enum Expr { Value(i64), Add(Box<Expr>,Box<Expr>) }
enum Frame { NeedRight(Expr), Combine(i64) }
enum Control { Evaluate(Expr), Return(i64) }
fn eval(root:Expr)->i64 {
    let mut stack=Vec::new();
    let mut control=Control::Evaluate(root);
    loop {
        control=match control {
            Control::Evaluate(Expr::Value(v))=>Control::Return(v),
            Control::Evaluate(Expr::Add(a,b))=>{
                // Save the second operand while the first is evaluated.
                stack.push(Frame::NeedRight(*b));Control::Evaluate(*a)
            }
            Control::Return(value)=>match stack.pop() {
                Some(Frame::NeedRight(right))=>{
                    stack.push(Frame::Combine(value));Control::Evaluate(right)
                }
                Some(Frame::Combine(left))=>Control::Return(left+value),
                None=>return value,
            }
        };
    }
}
fn main() {
    let expression=Expr::Add(Box::new(Expr::Value(10)),Box::new(Expr::Value(20)));
    println!("evaluated = {}",eval(expression));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn explicit_continuation_stack() {
        let expression=Expr::Add(
            Box::new(Expr::Value(2)),
            Box::new(Expr::Add(Box::new(Expr::Value(3)),Box::new(Expr::Value(4)))));
        assert_eq!(eval(expression),9);
    }
}
