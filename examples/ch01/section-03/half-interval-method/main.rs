fn root<F: Fn(f64)->f64>(f: F, mut low: f64, mut high: f64, tolerance: f64) -> Option<f64> {
    assert!(tolerance > 0.0 && low < high);
    let mut left = f(low);
    let right = f(high);
    if !left.is_finite() || !right.is_finite() { return None; }
    if left == 0.0 { return Some(low); }
    if right == 0.0 { return Some(high); }
    if left.is_sign_positive() == right.is_sign_positive() { return None; }
    for _ in 0..200 {
        let mid = low + (high-low)/2.0;
        let value = f(mid);
        if value == 0.0 || (high-low) <= tolerance { return Some(mid); }
        if !value.is_finite() { return None; }
        if value.is_sign_positive() == left.is_sign_positive() {
            low = mid; left = value;
        } else {
            high = mid;
        }
    }
    Some(low + (high-low)/2.0)
}
fn main() { println!("cuberoot(2)≈{:?}",root(|x|x*x*x-2.0,1.0,2.0,1e-12)); }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bracketing_is_preserved() {
        let value=root(|x|x*x*x-2.0,1.0,2.0,1e-12).unwrap();
        assert!((value-2.0_f64.cbrt()).abs() < 1e-10);
        assert_eq!(root(|x|x*x+1.0,0.0,2.0,1e-12),None);
        assert_eq!(root(|x|x-3.0,3.0,4.0,1e-12),Some(3.0));
    }
}
