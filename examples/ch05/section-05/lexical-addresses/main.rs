#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct Address { depth:usize,slot:usize }
fn resolve(layouts:&[Vec<&str>],name:&str)->Option<Address> {
    for (depth,frame) in layouts.iter().rev().enumerate() {
        if let Some(slot)=frame.iter().position(|binding|*binding==name) {
            return Some(Address{depth,slot});
        }
    }
    None
}
fn access(values:&[Vec<i64>],address:Address)->Option<i64> {
    let index=values.len().checked_sub(address.depth+1)?;
    values.get(index)?.get(address.slot).copied()
}
fn main() {
    let layouts=vec![vec!["x","y"],vec!["x"]];
    let addr=resolve(&layouts,"y").unwrap();
    println!("lexical address for y: {:?}",addr);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn shadowing_and_ancestor_frames() {
        let layout=vec![vec!["x","y"],vec!["x","z"]];
        let data=vec![vec![1,2],vec![99,8]];
        assert_eq!(resolve(&layout,"x"),Some(Address{depth:0,slot:0}));
        assert_eq!(resolve(&layout,"y"),Some(Address{depth:1,slot:1}));
        assert_eq!(access(&data,resolve(&layout,"y").unwrap()),Some(2));
        assert_eq!(access(&data,resolve(&layout,"x").unwrap()),Some(99));
        assert!(resolve(&layout,"missing").is_none());
    }
}
