use crate::expression::Expression;

struct AnnotatedExpression {
    expression: Expression,
    annotations: Vec<()>,
}

#[cfg(test)]
mod tests {
    // use insta::assert_snapshot;

    #[test]
    fn printing_annotated_expressions() {}
}
