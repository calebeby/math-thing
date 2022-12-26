use std::rc::Rc;

use crate::{
    constant::Constant, negation::Negation, product::Product, sum::Sum, token_stream::TokenStream,
    PrintOpts, Printable,
};

pub(crate) const PRECEDENCE_SUM: usize = 1;
pub(crate) const PRECEDENCE_NEGATION: usize = 2;
pub(crate) const PRECEDENCE_PRODUCT: usize = 3;
pub(crate) const PRECEDENCE_CONSTANT: usize = 4;
const DEFAULT_PRINT_OPTS: PrintOpts = PrintOpts {
    target: crate::PrintTarget::MathPrint,
};

#[derive(Clone)]
pub(crate) enum Expression {
    Constant(Rc<Constant>),
    Product(Rc<Product>),
    Sum(Rc<Sum>),
    Negation(Rc<Negation>),
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
                            inner.0.clone()
                        } else {
                            simplified
                        }
                    })
                    .collect();

                let product = Expression::Product(
                    Product {
                        terms: terms_simplified,
                    }
                    .into(),
                );
                if num_negatives % 2 == 0 {
                    // Even negatives -> output is not negative
                    product
                } else {
                    // Odd negatives -> output is negative
                    Expression::Negation(Negation(product).into())
                }
            }
            _ => self.clone(),
        }
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print(&DEFAULT_PRINT_OPTS))
    }
}

impl Printable for Expression {
    fn print<'a>(&'a self, print_opts: &'a PrintOpts) -> TokenStream {
        match self {
            Expression::Constant(constant) => constant.print(print_opts),
            Expression::Product(product) => product.print(print_opts),
            Expression::Sum(sum) => sum.print(print_opts),
            Expression::Negation(neg) => neg.print(print_opts),
        }
    }
}

pub(crate) trait AsExpression {
    fn expr(self) -> Expression;
}

impl<T> AsExpression for T
where
    T: Into<Expression>,
{
    fn expr(self) -> Expression {
        self.into()
    }
}
