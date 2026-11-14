fn row(n: usize) -> Vec<u64> {
    let mut current = vec![1];
    for _ in 0..n {
        let mut next = vec![1; current.len() + 1];
        for i in 1..current.len() {
            // Entries on row k+1 combine adjacent entries from row k.
            next[i] = current[i - 1] + current[i];
        }
        current = next;
    }
    current
}
fn main() {
    for level in 0..6 { println!("{:?}", row(level)); }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn known_rows_and_symmetry() {
        assert_eq!(row(0), vec![1]);
        assert_eq!(row(4), vec![1,4,6,4,1]);
        for n in 0..=15 {
            let values = row(n);
            assert_eq!(values.iter().sum::<u64>(), 1u64 << n);
            assert!(values.iter().zip(values.iter().rev()).all(|(a,b)| a == b));
        }
    }
}
