fn count_change(target: usize, coins: &[usize]) -> u64 {
    assert!(coins.iter().all(|&coin| coin > 0));
    let mut ways = vec![0u64; target + 1];
    ways[0] = 1; // Exactly one way to make zero: select no coins.
    for &coin in coins {
        for amount in coin..=target {
            ways[amount] += ways[amount - coin];
        }
    }
    ways[target]
}
fn main() {
    println!("ways to make 100 cents: {}", count_change(100, &[1,5,10,25,50]));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distinct_combinations_not_permutations() {
        assert_eq!(count_change(100,&[1,5,10,25,50]), 292);
        assert_eq!(count_change(10,&[1,5,10]), 4);
        assert_eq!(count_change(0,&[1,5,10]), 1);
        assert_eq!(count_change(3,&[2,4]), 0);
    }
}
