fn sine(angle: f64) -> (f64, usize) {
    fn approximate(x: f64, depth: usize) -> (f64,usize) {
        if x.abs() < 0.05 { return (x,depth); }
        let (small, levels) = approximate(x/3.0,depth+1);
        (3.0*small-4.0*small*small*small,levels)
    }
    approximate(angle,0)
}
fn main() {
    let (value, levels) = sine(12.15);
    println!("sin(12.15)≈{value:.6}, reductions={levels}");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reduced_angles_are_accurate() {
        for x in [-12.15,-2.0,-0.5,0.0,0.5,2.0,12.15] {
            let (estimate,_) = sine(x);
            assert!((estimate-x.sin()).abs() < 0.002);
        }
        assert!(sine(12.15).1 > 1);
    }
}
