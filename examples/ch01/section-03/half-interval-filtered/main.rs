fn filtered_accumulate<T, Comb, Term, Next, Pred>(
    combiner: Comb,
    null_value: T,
    term: Term,
    mut a: i64,
    next: Next,
    b: i64,
    filter: Pred,
) -> T
where
    Comb: Fn(T, T) -> T,
    Term: Fn(i64) -> T,
    Next: Fn(i64) -> i64,
    Pred: Fn(i64) -> bool,
{
    let mut acc = null_value;
    while a <= b {
        if filter(a) {
            acc = combiner(acc, term(a));
        }
        a = next(a);
    }
    acc
}

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.abs()
}

fn half_interval_method<F: Fn(f64) -> f64>(
    f: F,
    a: f64,
    b: f64,
    tol: f64,
) -> Result<f64, &'static str> {
    let fa = f(a);
    let fb = f(b);
    if fa * fb > 0.0 {
        return Err("endpoints must have opposite signs");
    }
    let (mut neg, mut pos) = if fa < 0.0 { (a, b) } else { (b, a) };
    for _ in 0..200 {
        let mid = 0.5 * (neg + pos);
        if (pos - neg).abs() < tol {
            return Ok(mid);
        }
        let test = f(mid);
        if test.abs() < tol * 0.1 {
            return Ok(mid);
        } else if test > 0.0 {
            pos = mid;
        } else {
            neg = mid;
        }
    }
    Ok(0.5 * (neg + pos))
}

fn fixed_point_damped<F: Fn(f64) -> f64>(f: F, mut guess: f64, tol: f64) -> f64 {
    for _ in 0..500 {
        let next = 0.5 * (guess + f(guess));
        if (next - guess).abs() < tol {
            return next;
        }
        guess = next;
    }
    guess
}

fn cont_frac<N, D>(n: N, d: D, k: usize) -> f64
where
    N: Fn(usize) -> f64,
    D: Fn(usize) -> f64,
{
    let mut acc = 0.0;
    for i in (1..=k).rev() {
        acc = n(i) / (d(i) + acc);
    }
    acc
}

fn euler_e(k: usize) -> f64 {
    2.0 + cont_frac(
        |_| 1.0,
        |i| {
            if i % 3 == 2 {
                2.0 * ((i + 1) / 3) as f64
            } else {
                1.0
            }
        },
        k,
    )
}

fn tan_cf(x: f64, k: usize) -> f64 {
    cont_frac(
        |i| if i == 1 { x } else { -x * x },
        |i| (2 * i - 1) as f64,
        k,
    )
}

fn main() {
    let sum_sq_primes = filtered_accumulate(|a, b| a + b, 0, |x| x * x, 1, |x| x + 1, 10, is_prime);
    let coprime_prod =
        filtered_accumulate(|a, b| a * b, 1, |x| x, 1, |x| x + 1, 9, |i| gcd(i, 10) == 1);
    let pi = half_interval_method(f64::sin, 2.0, 4.0, 1e-12).unwrap();
    let sqrt2 = fixed_point_damped(|y| 2.0 / y, 1.0, 1e-12);
    println!("sum_sq_primes(1..=10)={sum_sq_primes}, coprime_prod(<10)={coprime_prod}, pi≈{pi:.12}, sqrt(2)≈{sqrt2:.12}");
    println!(
        "euler_e(20)≈{:.12}, tan_cf(0.7, 20)≈{:.12}",
        euler_e(20),
        tan_cf(0.7, 20)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filtered_accumulate_primes_and_coprimes() {
        // Primes <= 10: 2, 3, 5, 7 -> squares: 4+9+25+49 = 87
        assert_eq!(
            filtered_accumulate(|a, b| a + b, 0, |x| x * x, 1, |x| x + 1, 10, is_prime),
            87
        );
        // Product of integers < 10 coprime to 10: 1*3*7*9 = 189
        assert_eq!(
            filtered_accumulate(|a, b| a * b, 1, |x| x, 1, |x| x + 1, 9, |i| gcd(i, 10) == 1),
            189
        );
    }

    #[test]
    fn roots_and_continued_fractions_converge() {
        let pi = half_interval_method(f64::sin, 2.0, 4.0, 1e-12).unwrap();
        assert!((pi - std::f64::consts::PI).abs() < 1e-10);
        assert!(half_interval_method(|x| x * x + 1.0, 0.0, 2.0, 1e-9).is_err());
        assert!((fixed_point_damped(|y| 2.0 / y, 1.0, 1e-12) - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!((euler_e(20) - std::f64::consts::E).abs() < 1e-12);
        assert!((tan_cf(0.7, 20) - 0.7_f64.tan()).abs() < 1e-12);
    }
}
