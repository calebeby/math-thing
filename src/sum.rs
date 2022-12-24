use crate::{
    expression::{Expression, PRECEDENCE_SUM},
    format_with_annotations,
    negation::Negation,
    step::{Annotatable, Annotation},
    MathPrintResult, Printable,
};

#[derive(Clone)]
pub(crate) struct Sum {
    pub(crate) terms: Vec<Expression>,
}

impl Annotatable for Sum {}

impl Printable for Sum {
    fn latex(&self) -> String {
        self.terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                if let Expression::Negation(neg) = term {
                    let Negation(inner) = neg.as_ref();
                    let inner_printed = if inner.precedence() <= PRECEDENCE_SUM {
                        inner.latex_with_parens()
                    } else {
                        inner.latex()
                    };
                    format!("-{}", inner_printed)
                } else {
                    let inner_printed = if term.precedence() <= PRECEDENCE_SUM {
                        term.latex_with_parens()
                    } else {
                        term.latex()
                    };

                    if i == 0 {
                        inner_printed
                    } else {
                        format!("+{}", inner_printed)
                    }
                }
            })
            .collect()
    }

    fn math_print_with_annotations(&self, annotations: &[Annotation]) -> MathPrintResult {
        self.terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                if let Expression::Negation(neg) = term {
                    let Negation(inner) = neg.as_ref();
                    let inner_printed = if inner.precedence() <= PRECEDENCE_SUM {
                        inner.math_print_with_parens_and_annotations(annotations)
                    } else {
                        inner.math_print_with_annotations(annotations)
                    };
                    if i == 0 {
                        format_with_annotations!("-{}", inner_printed)
                    } else {
                        format_with_annotations!(" - {}", inner_printed)
                    }
                } else {
                    let inner_printed = if term.precedence() <= PRECEDENCE_SUM {
                        term.math_print_with_parens_and_annotations(annotations)
                    } else {
                        term.math_print_with_annotations(annotations)
                    };
                    if i == 0 {
                        inner_printed
                    } else {
                        format_with_annotations!(" + {}", inner_printed)
                    }
                }
            })
            .collect()
    }
}
impl From<Sum> for Expression {
    fn from(sum: Sum) -> Self {
        Expression::Sum(sum.into())
    }
}
