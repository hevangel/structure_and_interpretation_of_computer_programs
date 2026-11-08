fn recursive(n: u64) -> u64 {
    if n == 0 { 1 } else { n * recursive(n - 1) }
}
fn iterative(n: u64) -> u64 {
    let mut result = 1;
    // Before each iteration: result = (i - 1)!.
    for i in 1..=n { result *= i; }
    result
}
fn main() { println!("10! = {}", iterative(10)); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn both_processes_agree() {
        for n in 0..=20 { assert_eq!(recursive(n), iterative(n)); }
        assert_eq!(iterative(5), 120);
    }
}
