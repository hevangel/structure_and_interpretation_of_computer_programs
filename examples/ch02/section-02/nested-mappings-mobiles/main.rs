#[derive(Clone, Debug)]
enum Structure {
    Weight(u64),
    SubMobile(Box<Mobile>),
}

#[derive(Clone, Debug)]
struct Branch {
    length: u64,
    structure: Structure,
}

#[derive(Clone, Debug)]
struct Mobile {
    left: Branch,
    right: Branch,
}

impl Mobile {
    fn total_weight(&self) -> u64 {
        branch_weight(&self.left) + branch_weight(&self.right)
    }

    fn is_balanced(&self) -> bool {
        let lw = branch_weight(&self.left);
        let rw = branch_weight(&self.right);
        self.left.length * lw == self.right.length * rw
            && branch_balanced(&self.left)
            && branch_balanced(&self.right)
    }
}

fn branch_weight(b: &Branch) -> u64 {
    match &b.structure {
        Structure::Weight(w) => *w,
        Structure::SubMobile(m) => m.total_weight(),
    }
}

fn branch_balanced(b: &Branch) -> bool {
    match &b.structure {
        Structure::Weight(_) => true,
        Structure::SubMobile(m) => m.is_balanced(),
    }
}

fn fold_right<T: Clone, U>(op: &impl Fn(&T, U) -> U, initial: U, seq: &[T]) -> U {
    match seq {
        [] => initial,
        [head, tail @ ..] => op(head, fold_right(op, initial, tail)),
    }
}

fn fold_left<T, U>(op: &impl Fn(U, &T) -> U, initial: U, seq: &[T]) -> U {
    let mut acc = initial;
    for item in seq {
        acc = op(acc, item);
    }
    acc
}

fn flatmap<T, U, F>(items: &[T], mut f: F) -> Vec<U>
where
    F: FnMut(&T) -> Vec<U>,
{
    items.iter().flat_map(&mut f).collect()
}

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn prime_sum_pairs(n: usize) -> Vec<(usize, usize, usize)> {
    let pairs = flatmap(&(1..=n).collect::<Vec<_>>(), |&i| {
        (1..i).map(|j| (i, j)).collect()
    });
    pairs
        .into_iter()
        .filter(|&(i, j)| is_prime(i + j))
        .map(|(i, j)| (i, j, i + j))
        .collect()
}

fn permutations<T: Clone + PartialEq>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() {
        return vec![vec![]];
    }
    flatmap(items, |x| {
        let rest: Vec<T> = items.iter().filter(|y| *y != x).cloned().collect();
        permutations(&rest)
            .into_iter()
            .map(|mut p| {
                p.insert(0, x.clone());
                p
            })
            .collect()
    })
}

fn main() {
    let inner = Mobile {
        left: Branch {
            length: 2,
            structure: Structure::Weight(3),
        },
        right: Branch {
            length: 3,
            structure: Structure::Weight(2),
        },
    };
    let root = Mobile {
        left: Branch {
            length: 4,
            structure: Structure::SubMobile(Box::new(inner)),
        },
        right: Branch {
            length: 5,
            structure: Structure::Weight(4),
        },
    };
    let nums = [2.0, 4.0, 8.0];
    let fr = fold_right(&|a, b| a / b, 1.0, &nums);
    let fl = fold_left(&|a, b| a / b, 1.0, &nums);
    println!(
        "mobile weight={}, balanced={}, fold_right={fr}, fold_left={fl}",
        root.total_weight(),
        root.is_balanced()
    );
    println!("prime_sum_pairs(4)={:?}", prime_sum_pairs(4));
    println!(
        "permutations([1,2,3]) count={}",
        permutations(&[1, 2, 3]).len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobile_recursive_balance_and_weight() {
        let balanced_sub = Mobile {
            left: Branch {
                length: 2,
                structure: Structure::Weight(3),
            },
            right: Branch {
                length: 3,
                structure: Structure::Weight(2),
            },
        };
        let outer = Mobile {
            left: Branch {
                length: 2,
                structure: Structure::SubMobile(Box::new(balanced_sub)),
            },
            right: Branch {
                length: 5,
                structure: Structure::Weight(2),
            },
        };
        assert_eq!(outer.total_weight(), 7);
        assert!(outer.is_balanced());
    }

    #[test]
    fn fold_commutativity_and_nested_mappings() {
        let seq = [1i64, 2, 3, 4];
        assert_eq!(fold_right(&|a, b| a - b, 0, &seq), 1 - (2 - (3 - 4)));
        assert_eq!(fold_left(&|a, b| a - b, 0, &seq), (((0 - 1) - 2) - 3) - 4);
        assert_eq!(
            fold_right(&|a, b| a + b, 0, &seq),
            fold_left(&|a, b| a + b, 0, &seq)
        );
        assert_eq!(
            prime_sum_pairs(4),
            vec![(2, 1, 3), (3, 2, 5), (4, 1, 5), (4, 3, 7)]
        );
        assert_eq!(permutations(&[1, 2, 3, 4]).len(), 24);
    }
}
