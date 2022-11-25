use crate::{Expression, Operation, Printable};

#[derive(Debug)]
pub(crate) struct Add {
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}

impl Add {
    #[inline]
    pub(crate) fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Self {
        Add { lhs, rhs }
    }
    #[inline]
    fn needs_brackets(term: &dyn Expression) -> bool {
        !matches!(term.operation(), Operation::None | Operation::Add)
    }
}

impl<T: Expression + 'static> std::ops::Add<T> for Add {
    type Output = Add;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Add::new(Box::new(self), Box::new(rhs))
    }
}

impl Printable for Add {
    fn latex(&self) -> String {
        let left = if Add::needs_brackets(self.lhs.as_ref()) {
            format!(r"\left({}\right)", self.lhs.latex())
        } else {
            self.lhs.latex()
        };
        let right = if Add::needs_brackets(self.rhs.as_ref()) {
            format!(r"\left({}\right)", self.rhs.latex())
        } else {
            self.rhs.latex()
        };
        format!("{left}+{right}")
    }
    fn math_print(&self) -> String {
        let left = if Add::needs_brackets(self.lhs.as_ref()) {
            format!("({})", self.lhs.math_print())
        } else {
            self.lhs.math_print()
        };
        let right = if Add::needs_brackets(self.rhs.as_ref()) {
            format!("({})", self.rhs.math_print())
        } else {
            self.rhs.math_print()
        };
        format!("{left} + {right}")
    }
}

impl Expression for Add {
    fn operation(&self) -> Operation {
        Operation::Add
    }
}
