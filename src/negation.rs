use crate::{
    expression::{Expression, PRECEDENCE_NEGATION},
    token_stream::TokenStream,
    tokens,
    traversable::Traversable,
    PrintOpts, Printable,
};

#[derive(Clone)]
pub(crate) struct Negation(pub(crate) Expression);

impl Printable for Negation {
    fn print<'a>(&'a self, print_opts: &'a PrintOpts) -> TokenStream {
        let inner = if self.0.precedence() <= PRECEDENCE_NEGATION {
            self.0.print_with_parens(print_opts)
        } else {
            self.0.print(print_opts)
        };
        tokens!("-", inner)
    }
}

impl From<Negation> for Expression {
    #[inline]
    fn from(neg: Negation) -> Self {
        Expression::Negation(neg.into())
    }
}

impl Traversable for Negation {
    fn child_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Expression> + 'a> {
        Box::new(std::iter::once(&self.0))
    }

    fn from_children(_original: &Negation, children: Vec<Expression>) -> Negation {
        if children.len() != 1 {
            unreachable!()
        }
        Negation(children.into_iter().next().unwrap())
    }
}
