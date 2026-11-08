fn fixed_point<F: Fn(f64) -> f64>(f: F, mut guess: f64) -> Option<f64> {
    for _ in 0..1000 {
        let next = f(guess);
        if !next.is_finite() { return None; }
        if (next - guess).abs() < 1e-12 { return Some(next); }
        guess = next;
    }
    None
}
fn main() {
    println!("cos(x)=x at {:?}", fixed_point(f64::cos, 1.0));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn cosine_fixed_point() {
        let x = fixed_point(f64::cos, 1.0).expect("cosine converges here");
        assert!((x - 0.739085133215).abs() < 1e-10);
    }
}
