struct Euler<F> { t:f64,y:f64,dt:f64,derivative:F }
impl<F:Fn(f64,f64)->f64> Iterator for Euler<F> {
    type Item=(f64,f64);
    fn next(&mut self)->Option<Self::Item> {
        let current=(self.t,self.y);
        // Explicit Euler: the slope is evaluated at the old state.
        self.y+=self.dt*(self.derivative)(self.t,self.y);
        self.t+=self.dt;
        Some(current)
    }
}
fn main() {
    let solver=Euler{t:0.0,y:1.0,dt:0.001,derivative:|_,y|y};
    let (_,value)=solver.take(1001).last().expect("prefix not empty");
    println!("estimate e ≈ {:.6}",value);
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn exponential_growth() {
        let solver=Euler{t:0.0,y:1.0,dt:0.001,derivative:|_,y|y};
        let (time,value)=solver.take(1001).last().unwrap();
        assert!((time-1.0).abs()<1e-10);
        assert!((value-std::f64::consts::E).abs()<0.002);
    }
}
