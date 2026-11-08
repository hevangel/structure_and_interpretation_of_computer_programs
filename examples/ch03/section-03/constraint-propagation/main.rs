#[derive(Clone,Copy)]
enum Constraint { Add(usize,usize,usize), Mul(usize,usize,usize) }
struct Network { values:Vec<Option<f64>>, rules:Vec<Constraint> }
impl Network {
    fn set(&mut self,id:usize,value:f64)->bool {
        match self.values[id] {
            Some(old)=>{assert!((old-value).abs()<1e-9,"conflicting value");false}
            None=>{self.values[id]=Some(value);true}
        }
    }
    fn propagate(&mut self) {
        loop {
            let mut changed=false;
            // Each rule can compute a missing input or output.
            for rule in self.rules.clone() {
                match rule {
                    Constraint::Add(a,b,total)=>{
                        match (self.values[a],self.values[b],self.values[total]) {
                            (Some(x),Some(y),_)=>changed|=self.set(total,x+y),
                            (Some(x),_,Some(z))=>changed|=self.set(b,z-x),
                            (_,Some(y),Some(z))=>changed|=self.set(a,z-y),
                            _=>{}
                        }
                    }
                    Constraint::Mul(a,b,product)=>{
                        match (self.values[a],self.values[b],self.values[product]) {
                            (Some(x),Some(y),_)=>changed|=self.set(product,x*y),
                            (Some(x),_,Some(z)) if x!=0.0=>changed|=self.set(b,z/x),
                            (_,Some(y),Some(z)) if y!=0.0=>changed|=self.set(a,z/y),
                            _=>{}
                        }
                    }
                }
            }
            if !changed {break;}
        }
    }
}
fn converter()->Network {
    // C*9 = (F-32)*5; indices: C, 9, F-32, 5, shared product, F, 32.
    Network {
        values:vec![None,Some(9.0),None,Some(5.0),None,None,Some(32.0)],
        rules:vec![Constraint::Mul(0,1,4),Constraint::Mul(2,3,4),
                   Constraint::Add(2,6,5)],
    }
}
fn main() {
    let mut n=converter();n.set(0,100.0);n.propagate();
    println!("100 Celsius = {:?} Fahrenheit",n.values[5]);
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn forward_and_backward_inference() {
        let mut c=converter();c.set(0,100.0);c.propagate();
        assert_eq!(c.values[5],Some(212.0));
        let mut f=converter();f.set(5,32.0);f.propagate();
        assert_eq!(f.values[0],Some(0.0));
    }
}
