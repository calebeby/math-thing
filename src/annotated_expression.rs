use crate::{expression::Expression, token_stream::TokenStream, PrintOpts, Printable};

pub(crate) struct AnnotatedExpression {
    expression: Expression,
    annotations: Vec<Annotation>,
}

pub(crate) struct Annotation {
    pub(crate) target_id: usize,
}

impl Annotation {
    pub(crate) fn new(expr: &Expression) -> Self {
        Self {
            target_id: expr.id(),
        }
    }
}

impl AnnotatedExpression {
    fn print<'a>(&'a self, print_opts: &'a PrintOpts) -> TokenStream {
        self.expression.print(print_opts, &self.annotations)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;

    use crate::{constant::Constant, expression::AsExpression, math};

    use super::*;

    #[test]
    fn test_printing_annotated_expressions() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let inner = math![x + y].expr();
        let annotation = Annotation::new(&inner);
        let exp = math![{ inner } * x].expr();
        assert_display_snapshot!(exp, @"(x + y) * x");
        let annotated_exp = AnnotatedExpression {
            expression: exp,
            annotations: vec![annotation],
        };
    }
}
