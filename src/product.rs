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

impl From<Product> for Expression {
    fn from(product: Product) -> Self {
        Expression::Product(product.into())
    }
}
