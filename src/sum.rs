use crate::{
    expression::{Expression, PRECEDENCE_SUM},
    negation::Negation,
    token_stream::TokenStream,
    tokens, PrintOpts, PrintTarget, Printable,
};

#[derive(Clone)]
pub(crate) struct Sum {
    pub(crate) terms: Vec<Expression>,
}

impl Printable for Sum {
    fn print<'a>(&'a self, print_opts: &'a PrintOpts) -> TokenStream {
        let is_latex = matches!(print_opts.target, PrintTarget::LaTex);
        TokenStream::from_iter(Box::new(self.terms.iter().enumerate().flat_map(
            |(i, term)| {
                if let Expression::Negation(neg) = term {
                    let Negation(inner) = neg.as_ref();
                    let inner_printed = if inner.precedence() <= PRECEDENCE_SUM {
                        inner.print_with_parens(print_opts)
                    } else {
                        inner.print(print_opts)
                    };
                    if i == 0 || is_latex {
                        tokens!["-", inner_printed]
                    } else {
                        tokens![" - ", inner_printed]
                    }
                } else {
                    let inner_printed = if term.precedence() <= PRECEDENCE_SUM {
                        term.print_with_parens(print_opts)
                    } else {
                        term.print(print_opts)
                    };
                    if i == 0 {
                        inner_printed
                    } else if is_latex {
                        tokens!["+", inner_printed]
                    } else {
                        tokens![" + ", inner_printed]
                    }
                }
            },
        )))
    }
}
impl From<Sum> for Expression {
    fn from(sum: Sum) -> Self {
        Expression::Sum(sum.into())
    }
}
