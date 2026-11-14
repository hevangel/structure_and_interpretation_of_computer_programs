fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    assert!(modulus >= 2);
    let mut result = 1;
    base %= modulus;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        exponent >>= 1;
        if exponent > 0 {
            base = ((base as u128 * base as u128) % modulus as u128) as u64;
        }
    }
    result
}
fn passes_fermat(n: u64, base: u64) -> bool {
    n >= 2 && mod_pow(base, n, n) == base % n
}
fn trial_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let mut divisor = 2;
    while divisor <= n / divisor {
        if n % divisor == 0 { return false; }
        divisor += 1;
    }
    true
}
fn main() {
    println!("561 passes base-2 Fermat? {}", passes_fermat(561,2));
    println!("561 is actually prime? {}", trial_prime(561));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prime_test_is_not_proof() {
        assert_eq!(mod_pow(2,10,1000),24);
        assert!(passes_fermat(13,2));
        assert!(passes_fermat(561,2));
        assert!(!trial_prime(561));
        assert!(trial_prime(563));
    }
}
