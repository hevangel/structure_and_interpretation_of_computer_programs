fn multiply(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    assert!(!a.is_empty() && !b.is_empty() && !b[0].is_empty());
    let k = b.len();
    let n = b[0].len();
    assert!(a.iter().all(|row| row.len() == k));
    assert!(b.iter().all(|row| row.len() == n));
    (0..a.len()).map(|i| {
        (0..n).map(|j| (0..k).map(|t| a[i][t] * b[t][j]).sum()).collect()
    }).collect()
}
fn main() { println!("{:?}", multiply(&[vec![1,2]], &[vec![3],vec![4]])); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn dimension_and_values() {
        assert_eq!(multiply(&[vec![1,2],vec![3,4]], &[vec![5,6],vec![7,8]]),
                   vec![vec![19,22],vec![43,50]]);
    }
}
