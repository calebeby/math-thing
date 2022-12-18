use crate::{constant::Constant, product::Product, sum::Sum, Printable};

pub(crate) const PRECEDENCE_SUM: usize = 1;
pub(crate) const PRECEDENCE_PRODUCT: usize = 2;
pub(crate) const PRECEDENCE_NEGATION: usize = 2;
pub(crate) const PRECEDENCE_CONSTANT: usize = 3;

#[derive(Clone)]
pub(crate) enum Expression {
    Constant(Constant),
    Product(Product),
    Sum(Sum),
    Negation(Box<Expression>),
}

impl Expression {
    pub(crate) const fn precedence(&self) -> usize {
        match self {
            Expression::Sum(..) => PRECEDENCE_SUM,
            Expression::Product(..) => PRECEDENCE_PRODUCT,
            Expression::Negation(..) => PRECEDENCE_NEGATION,
            Expression::Constant(..) => PRECEDENCE_CONSTANT,
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
                match exp.as_ref() {
                    &Expression::Constant(..) => exp.latex(),
                    _ => exp.latex_with_parens(),
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
                match exp.as_ref() {
                    &Expression::Constant(..) => exp.math_print(),
                    _ => exp.math_print_with_parens(),
                }
            ),
        }
    }
}

impl std::ops::Neg for Expression {
    type Output = Expression;

    #[inline]
    fn neg(self) -> Self::Output {
        Expression::Negation(Box::new(self))
    }
}
