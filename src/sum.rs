use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    annotated_expression::Annotation,
    expression::{Expression, ExpressionId, PRECEDENCE_SUM},
    token_stream::TokenStream,
    tokens,
    traversable::Traversable,
    PrintOpts, PrintTarget, Printable,
};

#[derive(Clone)]
pub(crate) struct Sum {
    terms: Vec<Expression>,
    id: ExpressionId,
}

impl Sum {
    #[inline]
    pub fn new(terms: Vec<Expression>) -> Self {
        let mut hasher = DefaultHasher::new();
        terms.hash(&mut hasher);
        Self {
            terms,
            id: hasher.finish(),
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

impl Hash for Sum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Printable for Sum {
    fn print<'a>(&'a self, print_opts: &'a PrintOpts, annotations: &[Annotation]) -> TokenStream {
        let is_latex = matches!(print_opts.target, PrintTarget::LaTex);
        TokenStream::from_iter(Box::new(self.terms.iter().enumerate().flat_map(
            |(i, term)| {
                if let Expression::Negation(neg) = term {
                    let inner = neg.inner();
                    let inner_printed = if inner.precedence() <= PRECEDENCE_SUM {
                        inner.print_with_parens(print_opts, annotations)
                    } else {
                        inner.print(print_opts, annotations)
                    };
                    if i == 0 || is_latex {
                        tokens!["-", inner_printed]
                    } else {
                        tokens![" - ", inner_printed]
                    }
                } else {
                    let inner_printed = if term.precedence() <= PRECEDENCE_SUM {
                        term.print_with_parens(print_opts, annotations)
                    } else {
                        term.print(print_opts, annotations)
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

impl Traversable for Sum {
    fn child_iter<'a>(&'a self) -> Box<(dyn Iterator<Item = &'a Expression> + 'a)> {
        Box::new(self.terms.iter())
    }

    fn from_children(_original: &Sum, children: Vec<Expression>) -> Sum {
        Sum::new(children)
    }
}

impl From<Sum> for Expression {
    #[inline]
    fn from(sum: Sum) -> Self {
        Expression::Sum(sum.into())
    }
}
