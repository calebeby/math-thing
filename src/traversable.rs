use crate::expression::Expression;

pub(crate) trait Traversable: Clone {
    fn child_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Expression> + 'a>;
    fn from_children(original: &Self, children: Vec<Expression>) -> Self;
}
