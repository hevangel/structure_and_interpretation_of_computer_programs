#[derive(Clone, Copy, Debug, PartialEq)]
enum Number { Int(i64), Rat(i64,i64), Real(f64) }
fn gcd(mut a:i64,mut b:i64)->i64 {
    while b!=0 {(a,b)=(b,a%b);}
    a.abs()
}
fn rat(n:i64,d:i64)->Number {
    assert!(d!=0);
    let sign=if d<0 {-1} else {1};
    let g=gcd(n,d);
    Number::Rat(sign*n/g,sign*d/g)
}
impl Number {
    fn as_real(self)->f64 {
        match self {Self::Int(n)=>n as f64,Self::Rat(n,d)=>n as f64/d as f64,Self::Real(x)=>x}
    }
    fn as_rat(self)->(i64,i64) {
        match self {Self::Int(n)=>(n,1),Self::Rat(n,d)=>(n,d),Self::Real(_)=>panic!("lossy conversion")}
    }
    fn plus(self,other:Self)->Self {
        match (self,other) {
            (Self::Real(_),_)|(_,Self::Real(_))=>Self::Real(self.as_real()+other.as_real()),
            (Self::Int(a),Self::Int(b))=>Self::Int(a+b),
            _=>{
                let (a,b)=self.as_rat();let (c,d)=other.as_rat();
                rat(a*d+b*c,b*d)
            }
        }
    }
}
fn main() { println!("{:?}",Number::Int(2).plus(rat(1,3))); }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn mixed_arithmetic() {
        assert_eq!(Number::Int(2).plus(rat(1,3)),rat(7,3));
        assert_eq!(Number::Int(2).plus(Number::Real(0.5)),Number::Real(2.5));
    }
}
