use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    Num(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, i64),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{n}"),
            Expr::Var(s) => write!(f, "{s}"),
            Expr::Add(a, b) => write!(f, "({a} + {b})"),
            Expr::Mul(a, b) => write!(f, "({a} * {b})"),
            Expr::Pow(b, e) => write!(f, "({b} ** {e})"),
        }
    }
}

fn make_sum(a: Expr, b: Expr) -> Expr {
    match (a, b) {
        (Expr::Num(0), x) | (x, Expr::Num(0)) => x,
        (Expr::Num(x), Expr::Num(y)) => Expr::Num(x + y),
        (l, r) => Expr::Add(Box::new(l), Box::new(r)),
    }
}

fn make_prod(a: Expr, b: Expr) -> Expr {
    match (a, b) {
        (Expr::Num(0), _) | (_, Expr::Num(0)) => Expr::Num(0),
        (Expr::Num(1), x) | (x, Expr::Num(1)) => x,
        (Expr::Num(x), Expr::Num(y)) => Expr::Num(x * y),
        (l, r) => Expr::Mul(Box::new(l), Box::new(r)),
    }
}

fn make_pow(base: Expr, exp: i64) -> Expr {
    match (base, exp) {
        (_, 0) => Expr::Num(1),
        (b, 1) => b,
        (Expr::Num(b), e) if e >= 0 => Expr::Num(b.pow(e as u32)),
        (b, e) => Expr::Pow(Box::new(b), e),
    }
}

fn deriv(expr: &Expr, var: &str) -> Expr {
    match expr {
        Expr::Num(_) => Expr::Num(0),
        Expr::Var(v) => Expr::Num(i64::from(v == var)),
        Expr::Add(a, b) => make_sum(deriv(a, var), deriv(b, var)),
        Expr::Mul(a, b) => make_sum(
            make_prod((**a).clone(), deriv(b, var)),
            make_prod(deriv(a, var), (**b).clone()),
        ),
        Expr::Pow(base, exp) => make_prod(
            make_prod(Expr::Num(*exp), make_pow((**base).clone(), exp - 1)),
            deriv(base, var),
        ),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Num(i64),
    Ident(String),
    Plus,
    Star,
    Pow,
    LParen,
    RParen,
}

fn lex_infix(input: &str) -> Vec<Token> {
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
        } else if chars[i].is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            out.push(Token::Num(s.parse().expect("valid int")));
        } else if chars[i].is_ascii_alphabetic() {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            out.push(Token::Ident(chars[start..i].iter().collect()));
        } else if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            out.push(Token::Pow);
            i += 2;
        } else {
            out.push(match chars[i] {
                '+' => Token::Plus,
                '*' => Token::Star,
                '(' => Token::LParen,
                ')' => Token::RParen,
                c => panic!("unexpected char: {c}"),
            });
            i += 1;
        }
    }
    out
}

struct InfixParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl InfixParser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    fn parse_sum(&mut self) -> Expr {
        let mut lhs = self.parse_prod();
        while matches!(self.peek(), Some(Token::Plus)) {
            self.pos += 1;
            lhs = make_sum(lhs, self.parse_prod());
        }
        lhs
    }

    fn parse_prod(&mut self) -> Expr {
        let mut lhs = self.parse_pow();
        while matches!(self.peek(), Some(Token::Star)) {
            self.pos += 1;
            lhs = make_prod(lhs, self.parse_pow());
        }
        lhs
    }

    fn parse_pow(&mut self) -> Expr {
        let base = self.parse_atom();
        if matches!(self.peek(), Some(Token::Pow)) {
            self.pos += 1;
            if let Token::Num(exp) = self.next() {
                return make_pow(base, exp);
            }
            panic!("expected integer exponent");
        }
        base
    }

    fn parse_atom(&mut self) -> Expr {
        match self.next() {
            Token::Num(n) => Expr::Num(n),
            Token::Ident(s) => Expr::Var(s),
            Token::LParen => {
                let inner = self.parse_sum();
                assert_eq!(self.next(), Token::RParen);
                inner
            }
            other => panic!("unexpected token: {other:?}"),
        }
    }
}

fn parse_infix(src: &str) -> Expr {
    let mut p = InfixParser {
        tokens: lex_infix(src),
        pos: 0,
    };
    let e = p.parse_sum();
    assert_eq!(p.pos, p.tokens.len());
    e
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tree {
    Empty,
    Node {
        entry: i32,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

impl Tree {
    fn to_list(&self) -> Vec<i32> {
        fn copy_to_list(t: &Tree, acc: &mut Vec<i32>) {
            if let Tree::Node { entry, left, right } = t {
                copy_to_list(left, acc);
                acc.push(*entry);
                copy_to_list(right, acc);
            }
        }
        let mut out = Vec::new();
        copy_to_list(self, &mut out);
        out
    }

    fn height(&self) -> usize {
        match self {
            Tree::Empty => 0,
            Tree::Node { left, right, .. } => 1 + left.height().max(right.height()),
        }
    }
}

/// Exercise 2.64 O(n) balanced BST builder from a sorted slice.
fn partial_tree(elts: &[i32], n: usize) -> (Tree, &[i32]) {
    if n == 0 {
        return (Tree::Empty, elts);
    }
    let left_size = (n - 1) / 2;
    let (left_tree, non_left) = partial_tree(elts, left_size);
    let this_entry = non_left[0];
    let right_size = n - (left_size + 1);
    let (right_tree, remaining) = partial_tree(&non_left[1..], right_size);
    (
        Tree::Node {
            entry: this_entry,
            left: Box::new(left_tree),
            right: Box::new(right_tree),
        },
        remaining,
    )
}

fn list_to_balanced_tree(elts: &[i32]) -> Tree {
    let (tree, rest) = partial_tree(elts, elts.len());
    assert!(rest.is_empty());
    tree
}

fn union_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::with_capacity(a.len() + b.len());
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            Ordering::Less => {
                out.push(a[i]);
                i += 1;
            }
            Ordering::Greater => {
                out.push(b[j]);
                j += 1;
            }
            Ordering::Equal => {
                out.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

fn intersection_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::new();
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
            Ordering::Equal => {
                out.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    out
}

/// Exercise 2.65 O(n) BST union via tree->list, linear merge, and list->balanced_tree.
fn union_set(a: &Tree, b: &Tree) -> Tree {
    list_to_balanced_tree(&union_sorted(&a.to_list(), &b.to_list()))
}

fn intersection_set(a: &Tree, b: &Tree) -> Tree {
    list_to_balanced_tree(&intersection_sorted(&a.to_list(), &b.to_list()))
}

fn main() {
    let expr = parse_infix("x ** 3 + 2 * x + 5");
    println!("d/dx({expr}) = {}", deriv(&expr, "x"));

    let t1 = list_to_balanced_tree(&[1, 3, 5, 7, 9]);
    let t2 = list_to_balanced_tree(&[2, 3, 6, 7, 10]);
    println!(
        "union={:?}, height={}",
        union_set(&t1, &t2).to_list(),
        union_set(&t1, &t2).height()
    );
    println!("intersection={:?}", intersection_set(&t1, &t2).to_list());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infix_power_rule_differentiation() {
        let expr = parse_infix("x ** 4 + 3 * x ** 2 + 7");
        let d = deriv(&expr, "x");
        // d/dx(x^4 + 3x^2 + 7) = 4*x^3 + 3*(2*x)
        assert_eq!(format!("{d}"), "((4 * (x ** 3)) + (3 * (2 * x)))");
        assert_eq!(
            deriv(&parse_infix("(x + 1) ** 2"), "x"),
            parse_infix("2 * (x + 1)")
        );
    }

    #[test]
    fn balanced_tree_height_is_logarithmic_up_to_1023() {
        for n in [1usize, 7, 63, 255, 1023] {
            let items: Vec<i32> = (1..=n as i32).collect();
            let tree = list_to_balanced_tree(&items);
            assert_eq!(tree.to_list(), items);
            let max_h = (usize::BITS - n.leading_zeros()) as usize;
            assert!(tree.height() <= max_h, "n={n}, h={}", tree.height());
        }
    }

    #[test]
    fn linear_bst_union_and_intersection() {
        let a = list_to_balanced_tree(&[1, 2, 4, 8, 16]);
        let b = list_to_balanced_tree(&[2, 3, 4, 5, 16]);
        assert_eq!(union_set(&a, &b).to_list(), vec![1, 2, 3, 4, 5, 8, 16]);
        assert_eq!(intersection_set(&a, &b).to_list(), vec![2, 4, 16]);
    }
}
