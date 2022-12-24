use crate::{
    expression::{Expression, PRECEDENCE_NEGATION},
    format_with_annotations,
    step::{Annotatable, Annotation},
    MathPrintResult, Printable,
};

pub(crate) struct Negation(pub(crate) Expression);

impl Annotatable for Negation {}

impl Printable for Negation {
    fn latex(&self) -> String {
        format!(
            "-{}",
            if self.0.precedence() <= PRECEDENCE_NEGATION {
                self.0.latex_with_parens()
            } else {
                self.0.latex()
            }
        )
    }

    fn math_print_with_annotations(&self, annotations: &[Annotation]) -> MathPrintResult {
        let inner = if self.0.precedence() <= PRECEDENCE_NEGATION {
            self.0.math_print_with_parens_and_annotations(annotations)
        } else {
            self.0.math_print_with_annotations(annotations)
        };
        format_with_annotations!("-{}", inner)
    }
}

impl From<Negation> for Expression {
    fn from(neg: Negation) -> Self {
        Expression::Negation(neg.into())
    }
}
