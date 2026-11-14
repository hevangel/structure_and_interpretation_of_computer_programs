fn sorted_unique(items:&[i32])->bool {
    items.windows(2).all(|w|w[0]<w[1])
}
fn combine(a:&[i32],b:&[i32])->(Vec<i32>,Vec<i32>) {
    assert!(sorted_unique(a)&&sorted_unique(b));
    let(mut i,mut j)=(0,0);
    let(mut union,mut intersection)=(Vec::new(),Vec::new());
    while i<a.len()&&j<b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less=>{union.push(a[i]);i+=1;}
            std::cmp::Ordering::Greater=>{union.push(b[j]);j+=1;}
            std::cmp::Ordering::Equal=>{
                union.push(a[i]);intersection.push(a[i]);i+=1;j+=1;
            }
        }
    }
    union.extend_from_slice(&a[i..]);
    union.extend_from_slice(&b[j..]);
    (union,intersection)
}
fn main() {println!("{:?}",combine(&[1,3,5],&[2,3,6]));}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_merge_laws() {
        assert_eq!(combine(&[1,3,5],&[2,3,6]),(vec![1,2,3,5,6],vec![3]));
        assert_eq!(combine(&[],&[2]),(vec![2],vec![]));
        assert_eq!(combine(&[1,2],&[1,2]),(vec![1,2],vec![1,2]));
    }
}
