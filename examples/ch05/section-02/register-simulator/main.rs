#[derive(Clone,Copy)]
enum Instr {
    Load(usize,i64), Add(usize,usize), Sub(usize,usize),
    JumpIfZero(usize,usize), Jump(usize), Halt,
}
fn execute(program:&[Instr],limit:usize)->Result<[i64;3],&'static str> {
    let mut pc=0usize;let mut r=[0i64;3];
    for _ in 0..limit {
        let ins=*program.get(pc).ok_or("invalid program counter")?;
        match ins {
            Instr::Load(dst,value)=>*r.get_mut(dst).ok_or("bad register")?=value,
            Instr::Add(dst,src)=>{
                let rhs=*r.get(src).ok_or("bad register")?;
                *r.get_mut(dst).ok_or("bad register")?+=rhs;
            }
            Instr::Sub(dst,src)=>{
                let rhs=*r.get(src).ok_or("bad register")?;
                *r.get_mut(dst).ok_or("bad register")?-=rhs;
            }
            Instr::JumpIfZero(reg,target)=>{
                if *r.get(reg).ok_or("bad register")?==0 {pc=target;continue;}
            }
            Instr::Jump(target)=>{pc=target;continue;}
            Instr::Halt=>return Ok(r),
        }
        pc+=1;
    }
    Err("step budget exceeded")
}
fn sum_program()->Vec<Instr> {
    // r0 counts down, r1 accumulates, r2 is the constant one.
    vec![
        Instr::Load(0,5),Instr::Load(1,0),Instr::Load(2,1),
        Instr::JumpIfZero(0,7),Instr::Add(1,0),Instr::Sub(0,2),
        Instr::Jump(3),Instr::Halt,
    ]
}
fn main() {println!("registers: {:?}",execute(&sum_program(),100));}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn calculate_sum() {
        assert_eq!(execute(&sum_program(),100).unwrap()[1],15);
        assert!(execute(&sum_program(),1).is_err());
    }
}
