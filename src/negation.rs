use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    annotated_expression::Annotation,
    expression::{Expression, ExpressionId, PRECEDENCE_NEGATION},
    token_stream::TokenStream,
    tokens,
    traversable::Traversable,
    PrintOpts, Printable,
};

#[derive(Clone)]
pub(crate) struct Negation {
    inner: Expression,
    id: ExpressionId,
}

impl Negation {
    #[inline]
    pub fn new(inner: Expression) -> Self {
        let mut hasher = DefaultHasher::new();
        inner.hash(&mut hasher);
        Self {
            inner,
            id: hasher.finish(),
        }
    }
    #[inline]
    pub fn inner(&self) -> &Expression {
        &self.inner
    }
    #[inline]
    pub(crate) fn id(&self) -> ExpressionId {
        self.id
    }
}

impl Hash for Negation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Printable for Negation {
    fn print<'a>(&'a self, print_opts: &'a PrintOpts, annotations: &[Annotation]) -> TokenStream {
        let inner = if self.inner.precedence() <= PRECEDENCE_NEGATION {
            self.inner.print_with_parens(print_opts, annotations)
        } else {
            self.inner.print(print_opts, annotations)
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
        Box::new(std::iter::once(&self.inner))
    }

    fn from_children(_original: &Negation, children: Vec<Expression>) -> Negation {
        if children.len() != 1 {
            unreachable!()
        }
        Negation::new(children.into_iter().next().unwrap())
    }
}
