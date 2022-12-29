use crate::{
    expression::{Expression, ExpressionId, DEFAULT_PRINT_OPTS},
    token_stream::TokenStream,
    PrintOpts, Printable,
};

pub(crate) struct AnnotatedExpression {
    pub(crate) expression: Expression,
    pub(crate) annotations: Vec<Annotation>,
}

pub(crate) struct Annotation {
    pub(crate) target_id: ExpressionId,
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

impl std::fmt::Display for AnnotatedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print(&DEFAULT_PRINT_OPTS))
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
        let exp = math![({ inner } * x) + (x + y)].expr();
        assert_display_snapshot!(exp, @"(x + y) * x + (x + y)");
        let annotated_exp = AnnotatedExpression {
            expression: exp,
            annotations: vec![annotation],
        };
        assert_display_snapshot!(annotated_exp, @r###"
        (x + y) * x + (x + y)
         ^^^^^
        "###);
    }
}
