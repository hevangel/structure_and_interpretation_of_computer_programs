fn continued_fraction<N: Fn(usize) -> f64, D: Fn(usize) -> f64>(
    numerator: N, denominator: D, depth: usize,
) -> f64 {
    let mut tail = 0.0;
    for i in (1..=depth).rev() {
        tail = numerator(i) / (denominator(i) + tail);
    }
    tail
}
fn main() {
    let inverse_phi = continued_fraction(|_| 1.0, |_| 1.0, 20);
    println!("1/phi ≈ {inverse_phi:.10}");
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn converges_to_inverse_phi() {
        let value = continued_fraction(|_| 1.0, |_| 1.0, 24);
        assert!((value - (5.0_f64.sqrt()-1.0)/2.0).abs() < 1e-9);
    }
}
