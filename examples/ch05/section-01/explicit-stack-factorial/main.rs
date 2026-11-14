fn factorial(n:u64)->(u64,usize) {
    let mut stack=Vec::new();
    let mut current=n;
    // Descend into the recurrence, preserving pending multiplications.
    while current>1 {
        stack.push(current);
        current-=1;
    }
    let max_depth=stack.len();
    let mut value=1;
    while let Some(multiplier)=stack.pop() {
        value*=multiplier;
    }
    (value,max_depth)
}
fn main() {println!("10! and stack depth: {:?}",factorial(10));}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn deferred_multiplications() {
        assert_eq!(factorial(0),(1,0));
        assert_eq!(factorial(1),(1,0));
        assert_eq!(factorial(5),(120,4));
        assert_eq!(factorial(10).0,3_628_800);
    }
}
