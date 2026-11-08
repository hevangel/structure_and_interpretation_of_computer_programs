use std::collections::BTreeMap;
#[derive(Debug)]
struct Node {
    weight: usize, symbol: Option<char>,
    left: Option<Box<Node>>, right: Option<Box<Node>>,
}
fn make_tree(text: &str) -> Node {
    let mut counts=BTreeMap::new();
    for ch in text.chars() { *counts.entry(ch).or_insert(0usize)+=1; }
    assert!(!counts.is_empty());
    let mut forest: Vec<Node>=counts.into_iter().map(|(ch,weight)| Node {
        weight, symbol:Some(ch), left:None, right:None
    }).collect();
    while forest.len()>1 {
        forest.sort_by_key(|node| node.weight);
        let a=forest.remove(0);let b=forest.remove(0);
        forest.push(Node { weight:a.weight+b.weight, symbol:None,
            left:Some(Box::new(a)),right:Some(Box::new(b)) });
    }
    forest.pop().expect("at least one symbol")
}
fn find_code(node: &Node, ch: char, path: &mut Vec<bool>) -> bool {
    if node.symbol==Some(ch) { return true; }
    if let Some(left)=&node.left {
        path.push(false);
        if find_code(left,ch,path) { return true; }
        path.pop();
    }
    if let Some(right)=&node.right {
        path.push(true);
        if find_code(right,ch,path) { return true; }
        path.pop();
    }
    false
}
fn encode(root: &Node, text: &str) -> Vec<bool> {
    let mut bits=Vec::new();
    for ch in text.chars() {
        let mut code=Vec::new();
        assert!(find_code(root,ch,&mut code));
        // A single-symbol alphabet needs at least one bit per symbol.
        if code.is_empty() { code.push(false); }
        bits.extend(code);
    }
    bits
}
fn decode(root: &Node, bits: &[bool]) -> String {
    if let Some(ch)=root.symbol {
        assert!(bits.iter().all(|bit| !bit));
        return std::iter::repeat_n(ch,bits.len()).collect();
    }
    let mut out=String::new(); let mut current=root;
    for &bit in bits {
        current=if bit {current.right.as_deref().expect("right branch")}
                else {current.left.as_deref().expect("left branch")};
        if let Some(ch)=current.symbol {out.push(ch);current=root;}
    }
    assert!(std::ptr::eq(current,root),"incomplete symbol");
    out
}
fn main() {
    let message="banana bandana";let tree=make_tree(message);
    let code=encode(&tree,message);
    println!("{} bits decode to {:?}",code.len(),decode(&tree,&code));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn round_trip() {
        for text in ["banana bandana", "aaaaa", "abcabc", "rust"] {
            let tree=make_tree(text);
            assert_eq!(decode(&tree,&encode(&tree,text)),text);
        }
    }
}
