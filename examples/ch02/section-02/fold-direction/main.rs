fn fold_left<F:Fn(i64,i64)->i64>(items:&[i64],initial:i64,f:F)->i64 {
    items.iter().copied().fold(initial,f)
}
fn fold_right<F:Fn(i64,i64)->i64>(items:&[i64],initial:i64,f:F)->i64 {
    items.iter().rev().fold(initial,|suffix,&item|f(item,suffix))
}
fn main() {
    let values=[1,2,3];
    println!("left {}, right {}",fold_left(&values,0,|a,b|a-b),
        fold_right(&values,0,|a,b|a-b));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subtraction_is_not_associative() {
        let values=[1,2,3];
        assert_eq!(fold_left(&values,0,|a,b|a-b),-6);
        assert_eq!(fold_right(&values,0,|a,b|a-b),2);
        assert_eq!(fold_left(&values,0,|a,b|a+b),fold_right(&values,0,|a,b|a+b));
    }
}
