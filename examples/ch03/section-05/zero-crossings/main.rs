fn crossings(samples:&[f64])->Vec<(usize,&'static str)> {
    let mut events=Vec::new();
    for index in 1..samples.len() {
        let before=samples[index-1];
        let after=samples[index];
        // Report an event at the index of the new sample.
        if before<=0.0 && after>0.0 {
            events.push((index,"up"));
        } else if before>=0.0 && after<0.0 {
            events.push((index,"down"));
        }
    }
    events
}
fn main() {println!("{:?}",crossings(&[-1.0,1.0,3.0,-2.0]));}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn reports_crossings_in_order() {
        assert_eq!(crossings(&[-1.0,1.0,3.0,-2.0,0.0,5.0]),
                   vec![(1,"up"),(3,"down"),(5,"up")]);
        assert!(crossings(&[]).is_empty());
        assert!(crossings(&[3.0]).is_empty());
    }
}
