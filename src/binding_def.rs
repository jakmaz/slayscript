use crate::{
    env::Env,
    expr::Expr,
    utils::{self, extract_whitespace},
};

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> (&str, Self) {
        let s = utils::tag("fresh", s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, name) = utils::extract_ident(s);
        let (s, _) = utils::extract_whitespace(s);

        let s = utils::tag("be", s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s);

        (
            s,
            Self {
                name: name.to_string(),
                val,
            },
        )
    }

    pub(crate) fn eval(&self, env: &mut Env) {
        env.store_binding(self.name.clone(), self.val.eval());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("fresh a be 10 ratio 2"),
            (
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div,
                    },
                },
            ),
        );
    }
}
