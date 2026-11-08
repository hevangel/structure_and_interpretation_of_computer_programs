use std::collections::VecDeque;
struct Queue<T> { items: VecDeque<T> }
impl<T> Queue<T> {
    fn new() -> Self { Self { items: VecDeque::new() } }
    fn push(&mut self, item: T) { self.items.push_back(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop_front() }
    fn len(&self) -> usize { self.items.len() }
}
fn main() {
    let mut q=Queue::new();
    q.push("first");q.push("second");
    println!("head = {:?}",q.pop());
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn fifo_and_empty() {
        let mut q=Queue::new();
        assert_eq!(q.pop(),None);
        q.push(1);q.push(2);
        assert_eq!(q.len(),2);
        assert_eq!(q.pop(),Some(1));
        assert_eq!(q.pop(),Some(2));
        assert_eq!(q.pop(),None);
    }
}
