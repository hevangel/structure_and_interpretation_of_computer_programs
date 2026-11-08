fn square_root(x: f64) -> f64 {
    assert!(x.is_finite() && x >= 0.0, "requires finite nonnegative input");
    if x == 0.0 { return 0.0; }
    // Start away from zero, which would make the Newton quotient undefined.
    let mut guess = if x >= 1.0 { x } else { 1.0 };
    for _ in 0..200 {
        let next = (guess + x / guess) / 2.0;
        // Relative change is more robust than a fixed absolute delta.
        if (next - guess).abs() <= 1e-13 * next.abs().max(1.0) {
            return next;
        }
        guess = next;
    }
    guess
}
fn main() { println!("sqrt(2) ≈ {:.12}", square_root(2.0)); }
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn approximate_roots() {
        for x in [0.0, 0.01, 2.0, 9.0, 10_000.0] {
            let r = square_root(x);
            assert!((r*r - x).abs() <= 1e-9 * x.max(1.0));
        }
    }
}
