type Picture = Vec<String>;
fn beside(left: &Picture, right: &Picture) -> Picture {
    assert_eq!(left.len(), right.len(), "equal height");
    left.iter().zip(right).map(|(a,b)| format!("{a}{b}")).collect()
}
fn above(top: &Picture, bottom: &Picture) -> Picture {
    assert!(top.first().map(String::len) == bottom.first().map(String::len));
    top.iter().chain(bottom).cloned().collect()
}
fn main() {
    let corner = vec![String::from("/\\"), String::from("\\/")];
    for row in above(&beside(&corner, &corner), &beside(&corner, &corner)) {
        println!("{row}");
    }
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn compositions_are_pictures() {
        let p=vec!["ab".to_string(), "cd".to_string()];
        assert_eq!(beside(&p,&p), vec!["abab","cdcd"]);
        assert_eq!(above(&p,&p).len(), 4);
    }
}
