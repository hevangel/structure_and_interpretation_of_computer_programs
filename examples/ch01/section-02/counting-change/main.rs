/// Tree-recursive coin change (1.2.2) using an explicit slice of coin denominations.
fn count_change_tree(amount: i64, coins: &[i64]) -> u64 {
    if amount == 0 {
        return 1;
    }
    if amount < 0 || coins.is_empty() {
        return 0;
    }
    count_change_tree(amount, &coins[1..]) + count_change_tree(amount - coins[0], coins)
}

/// Bottom-up dynamic programming count for coin combinations.
fn count_change_dp(target: usize, coins: &[usize]) -> u64 {
    assert!(coins.iter().all(|&coin| coin > 0));
    let mut ways = vec![0u64; target + 1];
    ways[0] = 1;
    for &coin in coins {
        for amount in coin..=target {
            ways[amount] += ways[amount - coin];
        }
    }
    ways[target]
}

/// Pascal's triangle entry (row, col) with 0-based indices (Exercise 1.12).
fn pascal(row: u64, col: u64) -> u64 {
    if col == 0 || col == row {
        1
    } else if col > row {
        0
    } else {
        pascal(row - 1, col - 1) + pascal(row - 1, col)
    }
}

/// O(lg n) Fibonacci via transformation squaring (Exercise 1.19):
/// T_{p,q}(a, b) = (bq + aq + ap, bp + aq), and T_{p,q}^2 = T_{p^2+q^2, 2pq+q^2}.
fn fib_log(n: u64) -> u128 {
    let (mut a, mut b) = (1u128, 0u128);
    let (mut p, mut q) = (0u128, 1u128);
    let mut count = n;
    while count > 0 {
        if count % 2 == 0 {
            let next_p = p * p + q * q;
            let next_q = 2 * p * q + q * q;
            p = next_p;
            q = next_q;
            count /= 2;
        } else {
            let next_a = b * q + a * q + a * p;
            let next_b = b * p + a * q;
            a = next_a;
            b = next_b;
            count -= 1;
        }
    }
    b
}

fn fib_iter(n: u64) -> u128 {
    let (mut a, mut b) = (0u128, 1u128);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

fn main() {
    let tree = count_change_tree(100, &[50, 25, 10, 5, 1]);
    let dp = count_change_dp(100, &[1, 5, 10, 25, 50]);
    println!("change(100): tree={tree}, dp={dp}");
    println!("pascal(5, 2) = {}", pascal(5, 2));
    println!("fib_log(50) = {} (iter={})", fib_log(50), fib_iter(50));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_and_dp_agree_on_coin_change() {
        assert_eq!(count_change_tree(100, &[50, 25, 10, 5, 1]), 292);
        assert_eq!(count_change_dp(100, &[50, 25, 10, 5, 1]), 292);
        for amount in 0..=40 {
            assert_eq!(
                count_change_tree(amount, &[25, 10, 5, 1]),
                count_change_dp(amount as usize, &[1, 5, 10, 25])
            );
        }
    }

    #[test]
    fn pascal_row_sums_are_powers_of_two() {
        for row in 0..=8 {
            let sum: u64 = (0..=row).map(|col| pascal(row, col)).sum();
            assert_eq!(sum, 1u64 << row);
        }
        assert_eq!(pascal(4, 2), 6);
    }

    #[test]
    fn logarithmic_fibonacci_matches_iterative() {
        for n in 0..=90 {
            assert_eq!(fib_log(n), fib_iter(n));
        }
    }
}
