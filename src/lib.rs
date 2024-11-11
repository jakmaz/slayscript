pub mod utils;

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, op_str) = utils::extract_op(s);

        let op = match op_str {
            "drip" => Self::Add,
            "lack" => Self::Sub,
            "combo" => Self::Mul,
            "ratio" => Self::Div,
            _ => unreachable!(),
        };

        (s, op)
    }
}

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, number) = utils::extract_digits(s);
        (s, Self(number.parse().unwrap()))
    }
}

#[derive(Debug, PartialEq)]
struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, op) = Op::new(s);
        let (s, rhs) = Number::new(s);

        (s, Self { lhs, rhs, op })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), ("", Number(123)));
    }
    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("drip"), ("", Op::Add));
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("lack"), ("", Op::Sub));
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("combo"), ("", Op::Mul));
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("ratio"), ("", Op::Div));
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1drip2"),
            (
                "",
                Expr {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add,
                },
            ),
        );
    }
}
