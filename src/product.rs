use crate::{
    annotated_expression::Annotation,
    expression::{gen_id, Expression, ExpressionId, PRECEDENCE_PRODUCT},
    token_stream::TokenStream,
    tokens,
    traverse::Traversable,
    PrintOpts, PrintTarget, Printable,
};

#[derive(Clone)]
pub(crate) struct Product {
    terms: Vec<Expression>,
    id: ExpressionId,
}

impl Product {
    #[inline]
    pub fn new(terms: Vec<Expression>) -> Self {
        Self {
            terms,
            id: gen_id(),
        }
    }
    #[inline]
    pub fn terms(&self) -> &[Expression] {
        &self.terms
    }
    #[inline]
    pub(crate) fn id(&self) -> ExpressionId {
        self.id
    }
}

impl Printable for Product {
    fn print<'a>(&'a self, print_opts: &'a PrintOpts, annotations: &[Annotation]) -> TokenStream {
        TokenStream::from_iter(Box::new(self.terms.iter().enumerate().flat_map(
            |(i, term)| {
                let inner = if term.precedence() <= PRECEDENCE_PRODUCT {
                    term.print_with_parens(print_opts, annotations)
                } else {
                    term.print(print_opts, annotations)
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

impl Traversable for Product {
    fn child_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Expression> + 'a> {
        Box::new(self.terms.iter())
    }

    fn from_children(_original: &Product, children: Vec<Expression>) -> Product {
        Product::new(children)
    }
}

impl From<Product> for Expression {
    #[inline]
    fn from(product: Product) -> Self {
        Expression::Product(product.into())
    }
}
