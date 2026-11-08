fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = 3;
    // If n is composite, it has some divisor <= sqrt(n).
    while d <= n / d {
        if n % d == 0 { return false; }
        d += 2;
    }
    true
}
fn main() { println!("101 prime? {}", is_prime(101)); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn known_values() {
        assert!(!is_prime(1));
        assert!(is_prime(2) && is_prime(97));
        assert!(!is_prime(9) && !is_prime(221));
    }
}
