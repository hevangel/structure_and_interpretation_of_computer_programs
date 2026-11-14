enum Tree { Empty,Node{key:i32,left:Box<Tree>,right:Box<Tree>} }
impl Tree {
    fn insert(self,key:i32)->Self {
        match self {
            Self::Empty=>Self::Node{key,left:Box::new(Self::Empty),right:Box::new(Self::Empty)},
            Self::Node{key:old,left,right} if key<old=>
                Self::Node{key:old,left:Box::new(left.insert(key)),right},
            Self::Node{key:old,left,right} if key>old=>
                Self::Node{key:old,left,right:Box::new(right.insert(key))},
            existing=>existing
        }
    }
    fn inorder(&self,out:&mut Vec<i32>) {
        if let Self::Node{key,left,right}=self {
            left.inorder(out);
            out.push(*key);
            right.inorder(out);
        }
    }
}
fn main() {
    let mut tree=Tree::Empty;
    for key in [5,2,8,1,3] {tree=tree.insert(key);}
    let mut ordered=Vec::new();tree.inorder(&mut ordered);
    println!("{ordered:?}");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sorted_without_duplicates() {
        let mut tree=Tree::Empty;
        for key in [5,2,8,1,3,2] {tree=tree.insert(key);}
        let mut values=Vec::new();tree.inorder(&mut values);
        assert_eq!(values,vec![1,2,3,5,8]);
    }
}
