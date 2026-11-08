struct Thunk<F:FnOnce()->i32> {
    pending:Option<F>,
    cached:Option<i32>,
}
impl<F:FnOnce()->i32> Thunk<F> {
    fn new(f:F)->Self {Self{pending:Some(f),cached:None}}
    fn force(&mut self)->i32 {
        if let Some(v)=self.cached{return v;}
        let f=self.pending.take().expect("pending before first force");
        let value=f();
        self.cached=Some(value);
        value
    }
}
fn main() {
    let mut delayed=Thunk::new(||21*2);
    println!("first {}, second {}",delayed.force(),delayed.force());
}
#[cfg(test)] mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;
    #[test] fn evaluate_exactly_once() {
        let calls=Rc::new(Cell::new(0));
        let seen=Rc::clone(&calls);
        let mut t=Thunk::new(move || {seen.set(seen.get()+1);42});
        assert_eq!(t.force(),42);
        assert_eq!(t.force(),42);
        assert_eq!(calls.get(),1);
    }
}
