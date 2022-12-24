use std::rc::Rc;

use crate::expression::Expression;

pub(crate) struct Step {
    label: Option<String>,
    annotated_expression: Option<Expression>,
    substeps: Vec<Step>,
    result: Rc<Expression>,
}

#[cfg(test)]
mod tests {
    // use insta::assert_snapshot;

    // use crate::{constant::Constant, sum::Sum};

    // use super::*;

    #[test]
    fn printing_annotated_expressions() {}
}
