use std::collections::BTreeMap;
#[derive(Clone,Debug,PartialEq,Eq)]
struct Polynomial { terms:BTreeMap<usize,i64> }
impl Polynomial {
    fn from_terms(items:&[(usize,i64)])->Self {
        let mut terms=BTreeMap::new();
        for &(degree,coefficient) in items {
            *terms.entry(degree).or_insert(0)+=coefficient;
        }
        terms.retain(|_,v|*v!=0);
        Self{terms}
    }
    fn add(&self,other:&Self)->Self {
        let mut terms=self.terms.clone();
        for (&degree,&coeff) in &other.terms {
            *terms.entry(degree).or_insert(0)+=coeff;
        }
        terms.retain(|_,v|*v!=0);
        Self{terms}
    }
    fn multiply(&self,other:&Self)->Self {
        let mut terms=BTreeMap::new();
        for (&a,&x) in &self.terms {
            for (&b,&y) in &other.terms {
                *terms.entry(a+b).or_insert(0)+=x*y;
            }
        }
        terms.retain(|_,v|*v!=0);
        Self{terms}
    }
    fn at(&self,x:i64)->i64 {
        self.terms.iter().map(|(&degree,&coeff)|coeff*x.pow(degree as u32)).sum()
    }
}
fn main() {
    let p=Polynomial::from_terms(&[(1,1),(0,1)]);
    println!("(x+1)^2 = {:?}",p.multiply(&p));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn polynomial_ring_operations() {
        let p=Polynomial::from_terms(&[(0,1),(1,1)]);
        let square=p.multiply(&p);
        assert_eq!(square,Polynomial::from_terms(&[(0,1),(1,2),(2,1)]));
        assert_eq!(square.at(3),16);
        assert_eq!(p.add(&Polynomial::from_terms(&[(1,-1)])),
                   Polynomial::from_terms(&[(0,1)]));
    }
}
