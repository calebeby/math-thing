use crate::{expression::Expression, Printable};
use std::rc::{Rc, Weak};

pub(crate) trait Annotatable {}

pub(crate) struct Annotation {
    pub(crate) target: Weak<dyn Annotatable>,
}

pub(crate) struct AnnotatedExpression {
    annotations: Vec<Annotation>,
    expression: Rc<Expression>,
}

impl AnnotatedExpression {
    pub(crate) fn math_print(&self) -> String {
        self.expression
            .math_print_with_annotations(&self.annotations)
            .to_string_with_annotations()
    }
}

pub(crate) struct Step {
    label: Option<String>,
    annotated_expression: Option<AnnotatedExpression>,
    substeps: Vec<Step>,
    result: Rc<Expression>,
}

#[cfg(test)]
mod tests {
    // use insta::assert_snapshot;

    // use crate::{constant::Constant, sum::Sum};

    // use super::*;

    #[test]
    fn printing_annotated_expressions() {
        // let x = Constant::new("x");
        // let y = Constant::new("y");

        // let exp1 = &x * &y;
        // let x_plus_y = Rc::new(Sum {
        //     terms: vec![x.into(), y.into()],
        // });
        // let exp3 = Expression::Sum(Rc::new(Sum {
        //     terms: vec![exp1, Expression::Sum(Rc::clone(&x_plus_y))],
        // }));

        // let x_plus_y: Rc<dyn Annotatable> = x_plus_y;
        // assert_snapshot!(AnnotatedExpression {
        //     annotations: vec![Annotation { target: Rc::downgrade(&x_plus_y) }],
        //     expression: Rc::new(exp3),
        // }.math_print(), @"x * y + (x + y)");
    }
}
