use std::rc::Rc;

use crate::{constant::Constant, product::Product, sum::Sum, Printable};

pub(crate) const PRECEDENCE_SUM: usize = 1;
pub(crate) const PRECEDENCE_NEGATION: usize = 2;
pub(crate) const PRECEDENCE_PRODUCT: usize = 3;
pub(crate) const PRECEDENCE_CONSTANT: usize = 4;

#[derive(Clone)]
pub(crate) enum Expression {
    Constant(Constant),
    Product(Product),
    Sum(Sum),
    Negation(Rc<Expression>),
}

impl Expression {
    pub(crate) fn precedence(&self) -> usize {
        match self {
            Expression::Sum(..) => PRECEDENCE_SUM,
            Expression::Product(..) => PRECEDENCE_PRODUCT,
            Expression::Negation(..) => PRECEDENCE_NEGATION,
            Expression::Constant(..) => PRECEDENCE_CONSTANT,
        }
    }
    /// Removes unneeded parentheses,
    /// and simplifies/cancels multiple negatives in products,
    /// and distributes negatives.
    pub(crate) fn simplify_parens_and_negatives(&self) -> Expression {
        match self {
            Expression::Product(product) => {
                let mut num_negatives = 0;
                let terms_simplified = product
                    .terms
                    .iter()
                    .map(|t| {
                        let simplified = t.simplify_parens_and_negatives();
                        if let Expression::Negation(inner) = simplified {
                            num_negatives += 1;
                            inner.as_ref().clone()
                        } else {
                            simplified
                        }
                        .into()
                    })
                    .collect();

                let product = Expression::Product(Product {
                    terms: terms_simplified,
                });
                if num_negatives % 2 == 0 {
                    // Even negatives -> output is not negative
                    product
                } else {
                    // Odd negatives -> output is negative
                    Expression::Negation(Rc::new(product))
                }
            }
            _ => self.clone(),
        }
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.math_print())
    }
}

impl Printable for Expression {
    fn latex(&self) -> String {
        match self {
            Expression::Constant(constant) => constant.latex(),
            Expression::Product(product) => product.latex(),
            Expression::Sum(sum) => sum.latex(),
            Expression::Negation(exp) => format!(
                "-{}",
                if exp.precedence() <= PRECEDENCE_NEGATION {
                    exp.latex_with_parens()
                } else {
                    exp.latex()
                }
            ),
        }
    }

    fn math_print(&self) -> String {
        match self {
            Expression::Constant(constant) => constant.math_print(),
            Expression::Product(product) => product.math_print(),
            Expression::Sum(sum) => sum.math_print(),
            Expression::Negation(exp) => format!(
                "-{}",
                if exp.precedence() <= PRECEDENCE_NEGATION {
                    exp.math_print_with_parens()
                } else {
                    exp.math_print()
                }
            ),
        }
    }
}

impl std::ops::Neg for Expression {
    type Output = Expression;

    #[inline]
    fn neg(self) -> Self::Output {
        Expression::Negation(Rc::new(self))
    }
}

impl From<&Expression> for Expression {
    #[inline]
    fn from(expression: &Expression) -> Self {
        expression.clone()
    }
}
