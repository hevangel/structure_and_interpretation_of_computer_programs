#[derive(Debug)]
enum Tree { Leaf(i64), Branch(Vec<Tree>) }
fn sum(tree: &Tree) -> i64 {
    match tree {
        Tree::Leaf(value) => *value,
        Tree::Branch(children) => children.iter().map(sum).sum(),
    }
}
fn count_leaves(tree: &Tree) -> usize {
    match tree {
        Tree::Leaf(_) => 1,
        Tree::Branch(children) => children.iter().map(count_leaves).sum(),
    }
}
fn main() {
    let t = Tree::Branch(vec![Tree::Leaf(2), Tree::Branch(vec![Tree::Leaf(3),Tree::Leaf(5)])]);
    println!("{} leaves, sum {}", count_leaves(&t), sum(&t));
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn folds_nesting() {
        let t=Tree::Branch(vec![Tree::Leaf(2), Tree::Branch(vec![Tree::Leaf(3)])]);
        assert_eq!(sum(&t), 5);
        assert_eq!(count_leaves(&t), 2);
    }
}
