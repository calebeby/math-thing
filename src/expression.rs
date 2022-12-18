use crate::{constant::Constant, product::Product, sum::Sum, Printable};

pub(crate) enum Expression {
    Constant(Constant),
    Product(Product),
    Sum(Sum),
    Negation(Box<Expression>),
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

impl<T: Into<Expression>> std::ops::Mul<T> for Expression {
    type Output = Expression;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Expression::Product(Product {
            terms: vec![self, rhs.into()],
        })
    }
}

impl<T: Into<Expression>> std::ops::Add<T> for Expression {
    type Output = Expression;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Expression::Sum(Sum {
            terms: vec![self, rhs.into()],
        })
    }
}

impl<T: Into<Expression>> std::ops::Sub<T> for Expression {
    type Output = Expression;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Expression::Sum(Sum {
            terms: vec![self, -rhs.into()],
        })
    }
}

impl std::ops::Neg for Expression {
    type Output = Expression;

    #[inline]
    fn neg(self) -> Self::Output {
        Expression::Negation(Box::new(self))
    }
}
