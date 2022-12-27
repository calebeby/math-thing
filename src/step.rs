use std::rc::Rc;

use crate::expression::Expression;

pub(crate) struct Step {
    label: Option<String>,
    annotated_expression: Option<Expression>,
    substeps: Vec<Step>,
    result: Rc<Expression>,
}
