use crate::{
    expression::{Expression, PRECEDENCE_NEGATION},
    Printable,
};

pub(crate) struct Negation(pub(crate) Expression);

impl Printable for Negation {
    fn latex(&self) -> String {
        format!(
            "-{}",
            if self.0.precedence() <= PRECEDENCE_NEGATION {
                self.0.latex_with_parens()
            } else {
                self.0.latex()
            }
        )
    }

    fn math_print(&self) -> String {
        let inner = if self.0.precedence() <= PRECEDENCE_NEGATION {
            self.0.math_print_with_parens()
        } else {
            self.0.math_print()
        };
        format!("-{}", inner)
    }
}

impl From<Negation> for Expression {
    fn from(neg: Negation) -> Self {
        Expression::Negation(neg.into())
    }
}
