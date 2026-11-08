fn solve(n:usize)->Vec<Vec<usize>> {
    fn search(n:usize,columns:&mut Vec<usize>,out:&mut Vec<Vec<usize>>) {
        let row=columns.len();
        if row==n {out.push(columns.clone());return;}
        for col in 0..n {
            // No queen may share a column or either diagonal.
            if columns.iter().enumerate().all(|(r,&c)|c!=col && c.abs_diff(col)!=row-r) {
                columns.push(col);search(n,columns,out);columns.pop();
            }
        }
    }
    let mut out=Vec::new();
    search(n,&mut Vec::new(),&mut out);
    out
}
fn main() {println!("8-queen solutions: {}",solve(8).len());}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn solution_counts() {
        assert_eq!(solve(1).len(),1);
        assert_eq!(solve(4).len(),2);
        assert_eq!(solve(8).len(),92);
    }
}
