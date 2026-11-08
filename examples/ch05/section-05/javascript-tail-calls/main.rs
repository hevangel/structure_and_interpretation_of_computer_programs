enum Bounce<T> { Done(T), Call(Box<dyn FnOnce()->Bounce<T>>) }
fn trampoline<T>(mut step:Bounce<T>)->T {
    loop {
        step=match step {
            Bounce::Done(value)=>return value,
            Bounce::Call(next)=>next(),
        };
    }
}
fn tail_sum(n:u64,acc:u64)->Bounce<u64> {
    if n==0 {Bounce::Done(acc)}
    else {Bounce::Call(Box::new(move || tail_sum(n-1,acc+n)))}
}
fn main() {println!("sum(1..100000) = {}",trampoline(tail_sum(100_000,0)));}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn deep_tail_recursion_without_stack_growth() {
        assert_eq!(trampoline(tail_sum(100_000,0)),5_000_050_000);
        assert_eq!(trampoline(tail_sum(0,17)),17);
    }
}
