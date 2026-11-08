fn simpson<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, n: usize) -> f64 {
    assert!(n > 0 && n % 2 == 0, "number of subintervals must be positive and even");
    let h = (b - a) / n as f64;
    let mut total = f(a) + f(b);
    for k in 1..n {
        let weight = if k % 2 == 0 { 2.0 } else { 4.0 };
        total += weight * f(a + k as f64 * h);
    }
    total * h / 3.0
}
fn main() { println!("integral x^3 from 0 to 1 ≈ {}", simpson(|x| x*x*x, 0.0, 1.0, 100)); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn cubic_exactness() {
        assert!((simpson(|x| x.powi(3), 0.0, 1.0, 10) - 0.25).abs() < 1e-12);
    }
}
