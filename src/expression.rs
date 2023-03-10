use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::{
    annotated_expression::Annotation,
    constant::Constant,
    negation::Negation,
    product::Product,
    sum::Sum,
    token_stream::{MathPrintToken, TokenStream},
    traverse::Traversable,
    PrintOpts, Printable,
};

pub(crate) const PRECEDENCE_SUM: usize = 1;
pub(crate) const PRECEDENCE_NEGATION: usize = 2;
pub(crate) const PRECEDENCE_PRODUCT: usize = 3;
pub(crate) const PRECEDENCE_CONSTANT: usize = 4;
pub(crate) const DEFAULT_PRINT_OPTS: PrintOpts = PrintOpts {
    target: crate::PrintTarget::MathPrint,
};

pub(crate) type ExpressionId = u64;

pub(crate) fn gen_id() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub(crate) fn counterplus() {
    static COUNTERR: AtomicU64 = AtomicU64::new(0);
    println!("Counter: {}", COUNTERR.fetch_add(1, Ordering::Relaxed));
}

#[derive(Clone)]
pub(crate) enum Expression {
    Constant(Rc<Constant>),
    Product(Rc<Product>),
    Sum(Rc<Sum>),
    Negation(Rc<Negation>),
}

impl Expression {
    pub(crate) fn precedence(&self) -> usize {
        match self {
            Expression::Sum(..) => PRECEDENCE_SUM,
            Expression::Product(..) => PRECEDENCE_PRODUCT,
            Expression::Negation(..) => PRECEDENCE_NEGATION,
            Expression::Constant(..) => PRECEDENCE_CONSTANT,
        }
    }
    pub(crate) fn id(&self) -> ExpressionId {
        match self {
            Expression::Sum(inner) => inner.id(),
            Expression::Product(inner) => inner.id(),
            Expression::Negation(inner) => inner.id(),
            Expression::Constant(inner) => inner.id(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print(&DEFAULT_PRINT_OPTS, &[]))
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Constant(constant) => {
                write!(f, "Expression::Constant({})", constant.expr())
            }
            Expression::Product(product) => {
                write!(
                    f,
                    "Expression::Product {{\n  {}\n}}",
                    product
                        .terms()
                        .iter()
                        .enumerate()
                        .map(|(i, term)| {
                            if i == 0 {
                                format!("{}", term)
                            } else {
                                format!(",\n  {}", term)
                            }
                        })
                        .collect::<String>()
                )
            }
            Expression::Sum(sum) => {
                write!(
                    f,
                    "Expression::Sum {{\n  {}\n}}",
                    sum.terms()
                        .iter()
                        .enumerate()
                        .map(|(i, term)| {
                            if i == 0 {
                                format!("{}", term)
                            } else {
                                format!(",\n  {}", term)
                            }
                        })
                        .collect::<String>()
                )
            }
            Expression::Negation(negation) => {
                write!(f, "Expression::Negation {{\n  {}\n}}", negation.inner())
            }
        }
    }
}

impl Printable for Expression {
    fn print<'a>(&'a self, print_opts: &'a PrintOpts, annotations: &[Annotation]) -> TokenStream {
        let inner = match self {
            Expression::Constant(constant) => constant.print(print_opts, annotations),
            Expression::Product(product) => product.print(print_opts, annotations),
            Expression::Sum(sum) => sum.print(print_opts, annotations),
            Expression::Negation(neg) => neg.print(print_opts, annotations),
        };
        let id = self.id();
        if annotations
            .iter()
            .any(|annotation| annotation.target_id == id)
        {
            std::iter::once(MathPrintToken::AnnotationStart)
                .chain(inner)
                .chain(std::iter::once(MathPrintToken::AnnotationEnd))
                .collect()
        } else {
            inner
        }
    }
}

impl Traversable for Expression {
    fn child_iter<'a>(&'a self) -> Box<(dyn Iterator<Item = &'a Expression> + 'a)> {
        match self {
            Expression::Constant(constant) => constant.child_iter(),
            Expression::Product(product) => product.child_iter(),
            Expression::Sum(sum) => sum.child_iter(),
            Expression::Negation(neg) => neg.child_iter(),
        }
    }

    fn from_children(original: &Self, children: Vec<Expression>) -> Expression {
        counterplus();
        match original {
            Expression::Constant(original) => (&Constant::from_children(original, children)).into(),
            Expression::Product(original) => Product::from_children(original, children).into(),
            Expression::Sum(original) => Sum::from_children(original, children).into(),
            Expression::Negation(original) => Negation::from_children(original, children).into(),
        }
    }
}

pub(crate) trait AsExpression {
    fn expr(self) -> Expression;
}

impl<T> AsExpression for T
where
    T: Into<Expression>,
{
    #[inline]
    fn expr(self) -> Expression {
        self.into()
    }
}
