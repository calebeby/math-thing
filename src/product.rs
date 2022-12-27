use crate::{
    expression::{Expression, PRECEDENCE_PRODUCT},
    token_stream::TokenStream,
    tokens, PrintOpts, PrintTarget, Printable,
};

#[derive(Clone)]
pub(crate) struct Product {
    pub(crate) terms: Vec<Expression>,
}

impl Printable for Product {
    fn print<'a>(&'a self, print_opts: &'a PrintOpts) -> TokenStream {
        TokenStream::from_iter(Box::new(self.terms.iter().enumerate().flat_map(
            |(i, term)| {
                let inner = if term.precedence() <= PRECEDENCE_PRODUCT {
                    term.print_with_parens(print_opts)
                } else {
                    term.print(print_opts)
                };
                if i != 0 {
                    if matches!(print_opts.target, PrintTarget::LaTex) {
                        tokens![" ", inner]
                    } else {
                        tokens![" * ", inner]
                    }
                } else {
                    inner
                }
            },
        )))
    }
}

impl From<Product> for Expression {
    #[inline]
    fn from(product: Product) -> Self {
        Expression::Product(product.into())
    }
}
