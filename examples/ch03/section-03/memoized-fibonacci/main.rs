use std::collections::HashMap;
fn fibonacci(n:u32,cache:&mut HashMap<u32,u64>)->u64 {
    if n<2 {return u64::from(n);}
    if let Some(&value)=cache.get(&n) {return value;}
    // Each recursive dependency is evaluated at most once.
    let answer=fibonacci(n-1,cache)+fibonacci(n-2,cache);
    cache.insert(n,answer);
    answer
}
fn main() {
    let mut cache=HashMap::new();
    println!("fib(80)={} with {} cache entries",fibonacci(80,&mut cache),cache.len());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn cache_reuses_results() {
        let mut memo=HashMap::new();
        assert_eq!(fibonacci(10,&mut memo),55);
        let entries=memo.len();
        assert_eq!(fibonacci(10,&mut memo),55);
        assert_eq!(entries,memo.len());
        assert_eq!(fibonacci(80,&mut memo),23_416_728_348_467_685);
    }
}
