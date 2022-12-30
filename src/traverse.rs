use std::borrow::Cow;

use crate::expression::Expression;

pub(crate) trait Traversable: Clone {
    fn child_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Expression> + 'a>;
    fn from_children(original: &Self, children: Vec<Expression>) -> Self;
}

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

static ROOT_PARENT_INDEX: usize = usize::MAX;

pub(crate) struct TraverserContext<'a, 'b> {
    i: usize,
    queue: &'a mut Vec<QueueItem<'b>>,
    pub(crate) expression: &'a Expression,
}
impl TraverserContext<'_, '_> {
    pub(crate) fn snapshot(&mut self) -> Expression {
        fn snapshot_recursive(root_index: usize, queue: &mut [QueueItem]) -> Expression {
            let root = &queue[root_index];
            if !root.invalidated_children {
                root.expr.as_ref().clone()
            } else {
                let child_indices = root.child_indices.clone();
                let children: Vec<_> = child_indices
                    .iter()
                    .rev()
                    .map(|&child_index| snapshot_recursive(child_index, queue))
                    .collect();
                let root = &queue[root_index];
                let updated_exp = Expression::from_children(&root.expr, children);

                queue[root_index].expr = Cow::Owned(updated_exp.clone());
                queue[root_index].invalidated_children = false;

                updated_exp
            }
        }

        snapshot_recursive(0, self.queue)
    }

    pub(crate) fn replace(&mut self, replacement: Expression) {
        let item = &mut self.queue[self.i];
        item.expr = Cow::Owned(replacement);
        item.invalidated_children = false;

        let mut i = item.parent_index;
        loop {
            if i == ROOT_PARENT_INDEX {
                break;
            }
            let parent = &mut self.queue[i];
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
    }
}

pub(crate) fn traverse<Visitor>(expr: &Expression, mut visitor: Visitor) -> Expression
where
    Visitor: FnMut(&mut TraverserContext),
{
    let mut queue = vec![];
    let mut stack = vec![StackItem {
        expr,
        parent_index: ROOT_PARENT_INDEX,
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
        if parent_index != ROOT_PARENT_INDEX {
            queue[parent_index].child_indices.push(new_parent_index);
        }
        stack.extend(children);
    }

    // Travel through the queue, end to beginning
    // For each element, call the visitor.
    // If the visitor produces a changed node, propagate the change upwards.
    // Whenever a change is produced, create a snapshot of the root expression for Steps

    let queue_len = queue.len();
    for i in (0..queue_len).rev() {
        let q = &queue;
        let item = &q[i];
        let up_to_date_expr: Expression = if item.invalidated_children {
            // Create an updated expression based on the changed children
            let cow = Cow::Owned(Expression::from_children(
                &item.expr,
                item.child_indices
                    .iter()
                    .rev()
                    .map(|&child_index| {
                        let m = &q[child_index];
                        assert!(!m.invalidated_children);
                        m.expr.as_ref().clone()
                    })
                    .collect(),
            ));
            let item = &mut queue[i];
            item.expr = cow;
            item.invalidated_children = false;
            item.expr.as_ref().clone()
        } else {
            item.expr.as_ref().clone()
        };

        let mut context = TraverserContext {
            i,
            queue: &mut queue,
            expression: &up_to_date_expr,
        };

        visitor(&mut context);
    }

    let item = &mut queue[0];
    // It should have already been updated (it was the last to update, from end to start)
    assert!(!item.invalidated_children);
    item.expr.clone().into_owned()
}
