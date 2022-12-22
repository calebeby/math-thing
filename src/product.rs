use std::rc::Rc;

use crate::{
    expression::{Expression, PRECEDENCE_PRODUCT},
    Printable,
};

#[derive(Clone)]
pub(crate) struct Product {
    pub(crate) terms: Vec<Rc<Expression>>,
}

impl Product {
    pub(crate) fn new(a: Expression, b: Expression) -> Self {
        Self {
            terms: vec![Rc::new(a), Rc::new(b)],
        }
    }
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
                (Expression::Product(p1), exp2) => p1
                    .terms
                    .into_iter()
                    .chain(std::iter::once(Rc::new(exp2)))
                    .collect(),
                (exp1, Expression::Product(p2)) => std::iter::once(Rc::new(exp1))
                    .chain(p2.terms.into_iter())
                    .collect(),
                (exp1, exp2) => vec![Rc::new(exp1), Rc::new(exp2)],
            },
        })
    }
}

impl<T: Into<Expression>> std::ops::Mul<T> for &Expression {
    type Output = Expression;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.clone() * rhs
    }
}

impl From<&Product> for Expression {
    #[inline]
    fn from(product: &Product) -> Self {
        Expression::Product(product.clone())
    }
}
impl From<Product> for Expression {
    #[inline]
    fn from(product: Product) -> Self {
        Expression::Product(product)
    }
}
