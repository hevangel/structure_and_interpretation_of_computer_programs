fn cube_root(x: f64) -> f64 {
    assert!(x.is_finite());
    if x == 0.0 { return 0.0; }
    let magnitude = x.abs();
    let mut guess = magnitude.max(1.0);
    for _ in 0..250 {
        // Newton's method applied to f(g) = g^3 - |x|.
        let next = (2.0 * guess + magnitude / (guess * guess)) / 3.0;
        if (next - guess).abs() <= 1e-14 * next.max(1.0) {
            guess = next;
            break;
        }
        guess = next;
    }
    guess.copysign(x)
}
fn main() { println!("cube root of 27 is {:.9}", cube_root(27.0)); }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negative_and_positive_inputs() {
        for x in [0.0, 1.0, 8.0, -64.0, 125.0, 1e-9, 1e12] {
            let result = cube_root(x);
            let reference = x.cbrt();
            assert!((result-reference).abs() <= 1e-9 * reference.abs().max(1.0));
        }
    }
}
