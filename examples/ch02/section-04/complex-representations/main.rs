#[derive(Debug, Clone, Copy)]
enum Complex { Cartesian{re:f64,im:f64}, Polar{r:f64,angle:f64} }
impl Complex {
    fn re(self)->f64 {match self {Self::Cartesian{re,..}=>re,Self::Polar{r,angle}=>r*angle.cos()}}
    fn im(self)->f64 {match self {Self::Cartesian{im,..}=>im,Self::Polar{r,angle}=>r*angle.sin()}}
    fn radius(self)->f64 {self.re().hypot(self.im())}
    fn angle(self)->f64 {self.im().atan2(self.re())}
    fn plus(self,b:Self)->Self {Self::Cartesian{re:self.re()+b.re(),im:self.im()+b.im()}}
    fn times(self,b:Self)->Self {Self::Polar{r:self.radius()*b.radius(),angle:self.angle()+b.angle()}}
}
fn main() {
    let a=Complex::Cartesian{re:1.0,im:1.0};
    let b=Complex::Cartesian{re:1.0,im:-1.0};
    println!("(1+i)(1-i) = {} + {}i",a.times(b).re(),a.times(b).im());
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn two_operations() {
        let a=Complex::Cartesian{re:1.0,im:1.0};
        let b=Complex::Cartesian{re:1.0,im:-1.0};
        assert!((a.times(b).re()-2.0).abs()<1e-12);
        assert!(a.plus(b).im().abs()<1e-12);
    }
}
