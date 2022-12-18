use crate::{
    expression::{Expression, PRECEDENCE_PRODUCT},
    Printable,
};

#[derive(Clone)]
pub(crate) struct Product {
    pub(crate) terms: Vec<Expression>,
}

impl Printable for Product {
    fn latex(&self) -> String {
        self.terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                let inner = if term.precedence() <= PRECEDENCE_PRODUCT {
                    term.latex_with_parens()
                } else {
                    term.latex()
                };
                if i != 0 {
                    format!(" {}", inner)
                } else {
                    inner
                }
            })
            .collect()
    }

    fn math_print(&self) -> String {
        self.terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                let inner = if term.precedence() <= PRECEDENCE_PRODUCT {
                    term.math_print_with_parens()
                } else {
                    term.math_print()
                };
                if i != 0 {
                    format!(" * {}", inner)
                } else {
                    inner
                }
            })
            .collect()
    }
}

impl<T: Into<Expression>> std::ops::Mul<T> for Expression {
    type Output = Expression;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Expression::Product(Product {
            terms: match (self, rhs.into()) {
                (Expression::Product(p1), Expression::Product(p2)) => {
                    p1.terms.into_iter().chain(p2.terms.into_iter()).collect()
                }
                (Expression::Product(p1), exp2) => {
                    p1.terms.into_iter().chain(std::iter::once(exp2)).collect()
                }
                (exp1, Expression::Product(p2)) => {
                    std::iter::once(exp1).chain(p2.terms.into_iter()).collect()
                }
                (exp1, exp2) => vec![exp1, exp2],
            },
        })
    }
}
