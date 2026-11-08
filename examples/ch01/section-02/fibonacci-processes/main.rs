fn recursive(n: u32) -> u64 {
    match n { 0 => 0, 1 => 1, _ => recursive(n - 1) + recursive(n - 2) }
}
fn iterative(n: u32) -> u64 {
    let (mut prev, mut curr) = (0, 1);
    for _ in 0..n { (prev, curr) = (curr, prev + curr); }
    prev
}
fn main() { println!("fib(20) = {}", iterative(20)); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn same_sequence() {
        for n in 0..=25 { assert_eq!(recursive(n), iterative(n)); }
        assert_eq!(iterative(10), 55);
    }
}
