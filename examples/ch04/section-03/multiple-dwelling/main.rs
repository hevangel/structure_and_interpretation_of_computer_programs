#[derive(Debug,PartialEq,Eq)]
struct Solution { baker:u8,cooper:u8,fletcher:u8,miller:u8,smith:u8 }
fn search()->Vec<Solution> {
    fn explore(floors:&mut Vec<u8>,out:&mut Vec<Solution>) {
        let person=floors.len();
        if person==5 {
            let (b,c,f,m,s)=(floors[0],floors[1],floors[2],floors[3],floors[4]);
            if m>c && s.abs_diff(f)!=1 && f.abs_diff(c)!=1 {
                out.push(Solution{baker:b,cooper:c,fletcher:f,miller:m,smith:s});
            }
            return;
        }
        for floor in 1..=5 {
            if floors.contains(&floor) {continue;}
            // Baker not top; Cooper not ground; Fletcher neither.
            if (person==0 && floor==5)
                || (person==1 && floor==1)
                || (person==2 && (floor==1 || floor==5)) {continue;}
            floors.push(floor);
            explore(floors,out);
            floors.pop(); // Undo this choice before trying the next.
        }
    }
    let mut solutions=Vec::new();
    explore(&mut Vec::new(),&mut solutions);
    solutions
}
fn main(){println!("solutions: {:?}",search());}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn one_solution_satisfies_every_clue() {
        assert_eq!(search(),vec![Solution{baker:3,cooper:2,fletcher:4,miller:5,smith:1}]);
    }
}
