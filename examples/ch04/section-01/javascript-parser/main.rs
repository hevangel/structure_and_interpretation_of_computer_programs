#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Token { Num(i64), Plus, Times, Left, Right }
#[derive(Debug)]
enum Expr { Num(i64), Add(Box<Expr>,Box<Expr>), Mul(Box<Expr>,Box<Expr>) }
fn lex(input:&str)->Vec<Token> {
    let chars:Vec<char>=input.chars().collect();
    let mut i=0;let mut out=Vec::new();
    while i<chars.len() {
        if chars[i].is_whitespace() {i+=1;continue;}
        if chars[i].is_ascii_digit() {
            let mut value=0i64;
            while i<chars.len()&&chars[i].is_ascii_digit() {
                value=value*10 + i64::from(chars[i].to_digit(10).expect("digit"));
                i+=1;
            }
            out.push(Token::Num(value));continue;
        }
        out.push(match chars[i] {
            '+'=>Token::Plus,'*'=>Token::Times,'('=>Token::Left,')'=>Token::Right,
            _=>panic!("unsupported character"),
        });i+=1;
    }
    out
}
struct Parser { tokens:Vec<Token>,pos:usize }
impl Parser {
    fn next(&mut self)->Token {
        let t=*self.tokens.get(self.pos).expect("unexpected end");
        self.pos+=1;t
    }
    fn expr(&mut self,min_bp:u8)->Expr {
        let mut lhs=match self.next() {
            Token::Num(n)=>Expr::Num(n),
            Token::Left=>{
                let e=self.expr(0);
                assert_eq!(self.next(),Token::Right,"closing parenthesis");e
            }
            _=>panic!("expected number or opening parenthesis"),
        };
        loop {
            let (left_bp,right_bp,op)=match self.tokens.get(self.pos) {
                Some(Token::Plus)=>(1,2,Token::Plus),
                Some(Token::Times)=>(3,4,Token::Times),
                _=>break,
            };
            if left_bp<min_bp {break;}
            self.pos+=1;
            let rhs=self.expr(right_bp);
            lhs=match op {
                Token::Plus=>Expr::Add(Box::new(lhs),Box::new(rhs)),
                Token::Times=>Expr::Mul(Box::new(lhs),Box::new(rhs)),
                _=>unreachable!(),
            };
        }
        lhs
    }
}
fn parse(s:&str)->Expr {
    let mut p=Parser{tokens:lex(s),pos:0};
    let e=p.expr(0);
    assert_eq!(p.pos,p.tokens.len(),"unconsumed input");e
}
fn eval(e:&Expr)->i64 {
    match e {
        Expr::Num(n)=>*n,
        Expr::Add(a,b)=>eval(a)+eval(b),
        Expr::Mul(a,b)=>eval(a)*eval(b),
    }
}
fn main() {println!("2+3*4 = {}",eval(&parse("2+3*4")));}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn precedence_and_parentheses() {
        assert_eq!(eval(&parse("2+3*4")),14);
        assert_eq!(eval(&parse("(2+3)*4")),20);
        assert_eq!(eval(&parse(" 7 ")),7);
    }
}
