fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        // Divisors common to (a, b) are exactly those common to (b, a%b).
        (a, b) = (b, a % b);
    }
    a
}
fn main() { println!("gcd(206, 40) = {}", gcd(206, 40)); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn common_divisors() {
        assert_eq!(gcd(206, 40), 2);
        assert_eq!(gcd(0, 9), 9);
        assert_eq!(gcd(13, 13), 13);
    }
}
