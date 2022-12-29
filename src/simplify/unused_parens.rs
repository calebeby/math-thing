use std::{borrow::Cow, cell::RefCell};

use crate::{
    annotated_expression::{AnnotatedExpression, Annotation},
    expression::{AsExpression, Expression},
    product::Product,
    step::Step,
    sum::Sum,
    traversable::Traversable,
};

pub(crate) enum TraverseResult {
    Replace(Expression),
    LeaveAlone,
}

pub(crate) fn traverse<Visitor>(expr: &Expression, visitor: Visitor) -> Expression
where
    Visitor: Fn(&Expression, &mut dyn FnMut(Expression), &dyn Fn() -> Expression),
{
    struct QueueItem<'a> {
        expr: Cow<'a, Expression>,
        child_indices: Vec<usize>,
        invalidated_children: bool,
        parent_index: usize,
    }

    struct StackItem<'a> {
        expr: &'a Expression,
        parent_index: usize,
    }

    let root_parent_index = usize::MAX;
    let mut queue = vec![];
    let mut stack = vec![StackItem {
        expr,
        parent_index: root_parent_index,
    }];

    // Creates a topologically-sorted queue based on the expression tree

    while let Some(StackItem { expr, parent_index }) = stack.pop() {
        let new_parent_index = queue.len();
        let children: Vec<_> = expr
            .child_iter()
            .map(|expr| StackItem {
                expr,
                parent_index: new_parent_index,
            })
            .collect();
        queue.push(QueueItem {
            expr: Cow::Borrowed(expr),
            child_indices: vec![],
            invalidated_children: false,
            parent_index,
        });
        if parent_index != root_parent_index {
            queue[parent_index].child_indices.push(new_parent_index);
        }
        stack.extend(children);
    }

    // Travel through the queue, end to beginning
    // For each element, call the visitor.
    // If the visitor produces a changed node, propagate the change upwards.
    // Whenever a change is produced, create a snapshot of the root expression for Steps

    let queue_len = queue.len();
    let queue_refcell = RefCell::new(queue);
    for i in (0..queue_len).rev() {
        let queue = queue_refcell.borrow();
        let item = &queue[i];
        let up_to_date_expr: Expression = if item.invalidated_children {
            // Create an updated expression based on the changed children
            let cow = Cow::Owned(Expression::from_children(
                &item.expr,
                item.child_indices
                    .iter()
                    .rev()
                    .map(|&child_index| {
                        let m = &queue[child_index];
                        assert!(!m.invalidated_children);
                        m.expr.as_ref().clone()
                    })
                    .collect(),
            ));
            drop(queue);
            let item = &mut queue_refcell.borrow_mut()[i];
            item.expr = cow;
            item.invalidated_children = false;
            item.expr.as_ref().clone()
        } else {
            let expr = item.expr.as_ref().clone();
            drop(queue);
            expr
        };
        fn snapshot_recursive(root: &QueueItem, queue: &[QueueItem]) -> Expression {
            if !root.invalidated_children {
                root.expr.as_ref().clone()
            } else {
                // TODO: Save this onto the object to save updating-work later
                Expression::from_children(
                    &root.expr,
                    root.child_indices
                        .iter()
                        .rev()
                        .map(|&child_index| {
                            let m = &queue[child_index];
                            snapshot_recursive(m, queue)
                        })
                        .collect(),
                )
            }
        }

        let snapshot = || -> Expression {
            snapshot_recursive(&queue_refcell.borrow()[0], &queue_refcell.borrow())
        };

        let mut replace = |replacement: Expression| {
            let mut queue = queue_refcell.borrow_mut();
            let item = &mut queue[i];
            item.expr = Cow::Owned(replacement);
            item.invalidated_children = false;

            let mut i = item.parent_index;
            loop {
                if i == root_parent_index {
                    break;
                }
                let parent = &mut queue[i];
                // If the parent already knows children are modified, no need to do anything.
                // This is fine, because when the parent gets modified, it will already get updated.
                // If the parent does not know that its children are modified,
                // now we need to mark it as modified and continue traversing upwards.
                if !parent.invalidated_children {
                    parent.invalidated_children = true;
                    i = parent.parent_index;
                } else {
                    break;
                }
            }
        };

        visitor(&up_to_date_expr, &mut replace, &snapshot);
    }

    let item = &queue_refcell.borrow()[0];
    // It should have already been updated (it was the last to update, from end to start)
    assert!(!item.invalidated_children);
    item.expr.clone().into_owned()
}

pub(crate) fn simplify_unused_parens(expr: &Expression) -> Expression {
    traverse(expr, |node, replace, snapshot| {
        let snapshot_before = snapshot();
        let mut annotations = vec![];
        match node {
            Expression::Product(prod) => {
                if prod
                    .terms()
                    .iter()
                    .any(|term| matches!(term, Expression::Product(..)))
                {
                    let mut terms = vec![];
                    for t in prod.terms() {
                        match t {
                            Expression::Product(product) => {
                                annotations.push(Annotation {
                                    target_id: product.id(),
                                });
                                terms.extend(product.terms().iter().cloned());
                            }
                            _ => terms.push(t.clone()),
                        };
                    }
                    replace(Product::new(terms).expr());
                    println!(
                        "Step: {}",
                        Step {
                            label: "Remove extra parens around product".to_owned().into(),
                            annotated_expression: Some(AnnotatedExpression {
                                expression: snapshot_before,
                                annotations,
                            }),
                            substeps: vec![],
                            result: snapshot(),
                        }
                    );
                }
            }
            Expression::Sum(prod) => {
                if prod
                    .terms()
                    .iter()
                    .any(|term| matches!(term, Expression::Sum(..)))
                {
                    let mut terms = vec![];
                    for t in prod.terms() {
                        match t {
                            Expression::Sum(sum) => {
                                annotations.push(Annotation {
                                    target_id: sum.id(),
                                });
                                terms.extend(sum.terms().iter().cloned());
                            }
                            _ => terms.push(t.clone()),
                        };
                    }
                    replace(Sum::new(terms).expr());
                    println!(
                        "Step: {}",
                        Step {
                            label: "Remove extra parens around sum".to_owned().into(),
                            annotated_expression: Some(AnnotatedExpression {
                                expression: snapshot_before,
                                annotations,
                            }),
                            substeps: vec![],
                            result: snapshot(),
                        }
                    );
                }
            }
            _ => {}
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constant::Constant, expression::AsExpression, math};

    #[test]
    fn test_simplify_unused_parens() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let z = Constant::new("z");

        let exp = math![(x * y) * z].expr();
        insta::assert_display_snapshot!(exp, @"(x * y) * z");
        insta::assert_display_snapshot!(simplify_unused_parens(&exp), @"x * y * z");

        let exp = math![x * (y * z)].expr();
        insta::assert_display_snapshot!(exp, @"x * (y * z)");
        insta::assert_display_snapshot!(simplify_unused_parens(&exp), @"x * y * z");

        let exp = math![((x * y) * (x * y)) * (z * (z * x * x))].expr();
        insta::assert_display_snapshot!(exp, @"((x * y) * (x * y)) * (z * (z * x * x))");
        insta::assert_display_snapshot!(simplify_unused_parens(&exp), @"x * y * x * y * z * z * x * x");

        let exp = math![((x + y) + y) * (x * y) * ((z * x) + y)].expr();
        insta::assert_display_snapshot!(exp, @"((x + y) + y) * (x * y) * (z * x + y)");
        insta::assert_display_snapshot!(simplify_unused_parens(&exp), @"(x + y + y) * x * y * (z * x + y)");
    }
}
