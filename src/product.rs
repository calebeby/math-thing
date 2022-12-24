use crate::{
    expression::{Expression, PRECEDENCE_PRODUCT},
    format_with_annotations,
    step::{Annotatable, Annotation},
    MathPrintResult, Printable,
};

#[derive(Clone)]
pub(crate) struct Product {
    pub(crate) terms: Vec<Expression>,
}

impl Annotatable for Product {}

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

    fn math_print_with_annotations(&self, annotations: &[Annotation]) -> MathPrintResult {
        self.terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                let inner = if term.precedence() <= PRECEDENCE_PRODUCT {
                    term.math_print_with_parens_and_annotations(annotations)
                } else {
                    term.math_print_with_annotations(annotations)
                };
                if i != 0 {
                    format_with_annotations!(" * {}", inner)
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
