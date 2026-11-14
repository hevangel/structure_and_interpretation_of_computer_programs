fn subsets<T:Clone>(items:&[T])->Vec<Vec<T>> {
    let mut result=vec![Vec::new()];
    for item in items {
        let existing=result.len();
        // Each old subset gains a companion containing the current item.
        for index in 0..existing {
            let mut extended=result[index].clone();
            extended.push(item.clone());
            result.push(extended);
        }
    }
    result
}
fn main() {println!("{:?}",subsets(&[1,2,3]));}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn includes_empty_and_complete_subsets() {
        let all=subsets(&[1,2,3]);
        assert_eq!(all.len(),8);
        assert!(all.contains(&vec![]));
        assert!(all.contains(&vec![1,2,3]));
        assert_eq!(subsets::<i32>(&[]),vec![vec![]]);
    }
}
