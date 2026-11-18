fn expmod(base: u64, exp: u64, m: u64) -> u64 {
    assert!(m >= 2);
    if exp == 0 {
        return 1 % m;
    }
    let mut result = 1u128;
    let mut b = u128::from(base % m);
    let mut e = exp;
    let mod_u = u128::from(m);
    while e > 0 {
        if e & 1 == 1 {
            result = (result * b) % mod_u;
        }
        e >>= 1;
        if e > 0 {
            b = (b * b) % mod_u;
        }
    }
    result as u64
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Deterministic Exercise 1.27 check: true iff every coprime 1 <= a < n satisfies a^n = a (mod n).
fn all_coprime_fermat_pass(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    (1..n)
        .filter(|&a| gcd(a, n) == 1)
        .all(|a| expmod(a, n, n) == a)
}

/// Miller-Rabin non-trivial square root of 1 check (Exercise 1.28).
fn expmod_checked(base: u64, exp: u64, m: u64) -> u64 {
    if exp == 0 {
        return 1;
    }
    if exp % 2 == 0 {
        let root = expmod_checked(base, exp / 2, m);
        if root == 0 {
            return 0;
        }
        let sq = ((u128::from(root) * u128::from(root)) % u128::from(m)) as u64;
        // A non-trivial square root of 1 mod m proves m is composite.
        if root != 1 && root != m - 1 && sq == 1 {
            return 0;
        }
        sq
    } else {
        let sub = expmod_checked(base, exp - 1, m);
        if sub == 0 {
            return 0;
        }
        ((u128::from(base) * u128::from(sub)) % u128::from(m)) as u64
    }
}

fn miller_rabin_witness(n: u64, base: u64) -> bool {
    if n < 2 {
        return false;
    }
    expmod_checked(base, n - 1, n) == 1
}

/// Deterministic Miller-Rabin for 64-bit integers using well-known witness bases.
fn is_probable_prime_mr(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
        if p < n && !miller_rabin_witness(n, p) {
            return false;
        }
    }
    true
}

fn is_prime_trial(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d <= n / d {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn main() {
    for &carmichael in &[561u64, 1105, 1729, 2465, 2821, 6601] {
        println!(
            "Carmichael {carmichael}: trial={}, fermat_all_coprime={}, mr_prime={}",
            is_prime_trial(carmichael),
            all_coprime_fermat_pass(carmichael),
            is_probable_prime_mr(carmichael)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carmichael_numbers_fool_fermat_but_fail_miller_rabin() {
        for &c in &[561u64, 1105, 1729, 2465, 2821, 6601] {
            assert!(!is_prime_trial(c));
            assert!(all_coprime_fermat_pass(c));
            assert!(!is_probable_prime_mr(c));
        }
    }

    #[test]
    fn miller_rabin_matches_trial_division_up_to_10000() {
        for n in 0..10_000 {
            assert_eq!(
                is_probable_prime_mr(n),
                is_prime_trial(n),
                "mismatch at {n}"
            );
        }
    }
}
