use crate::{expression::Expression, Printable};

pub(crate) struct Product {
    pub(crate) terms: Vec<Expression>,
}

impl Printable for Product {
    fn latex(&self) -> String {
        self.terms
            .iter()
            .map(|term| match term {
                &Expression::Product { .. } | &Expression::Constant(..) => term.latex(),
                _ => term.latex_with_parens(),
            })
            .collect()
    }

    fn math_print(&self) -> String {
        self.terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                let inner = match term {
                    &Expression::Product { .. } | &Expression::Constant(..) => term.math_print(),
                    _ => term.math_print_with_parens(),
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
            terms: vec![self, rhs.into()],
        })
    }
}
