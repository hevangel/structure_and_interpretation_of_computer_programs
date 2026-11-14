fn prime(n:u64)->bool {
    if n<2 {return false;}
    let mut divisor=2;
    while divisor<=n/divisor {
        if n%divisor==0 {return false;}
        divisor+=1;
    }
    true
}
fn pairs(limit:u64)->Vec<(u64,u64)> {
    let mut result=Vec::new();
    for larger in 1..=limit {
        for smaller in 1..larger {
            if prime(smaller+larger) { result.push((smaller,larger)); }
        }
    }
    result
}
fn main() {println!("prime-sum pairs up to 6: {:?}",pairs(6));}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filters_without_duplicates() {
        let result=pairs(6);
        assert_eq!(result.len(),7);
        assert!(result.contains(&(1,2)));
        assert!(result.contains(&(5,6)));
        assert!(result.iter().all(|&(a,b)|a<b&&prime(a+b)));
    }
}
