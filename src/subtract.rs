use crate::{Expression, Operation, Printable};

#[derive(Debug)]
pub(crate) struct Subtract {
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}

impl Subtract {
    #[inline]
    pub(crate) fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Self {
        Subtract { lhs, rhs }
    }
    #[inline]
    fn needs_brackets(term: &dyn Expression) -> bool {
        !matches!(term.operation(), Operation::None)
    }
}

impl<T: Expression + 'static> std::ops::Sub<T> for Subtract {
    type Output = Subtract;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Subtract::new(Box::new(self), Box::new(rhs))
    }
}

impl Printable for Subtract {
    fn latex(&self) -> String {
        let left = if Subtract::needs_brackets(self.lhs.as_ref()) {
            format!(r"\left({}\right)", self.lhs.latex())
        } else {
            self.lhs.latex()
        };
        let right = if Subtract::needs_brackets(self.rhs.as_ref()) {
            format!(r"\left({}\right)", self.rhs.latex())
        } else {
            self.rhs.latex()
        };
        format!("{left}-{right}")
    }
    fn math_print(&self) -> String {
        let left = if Subtract::needs_brackets(self.lhs.as_ref()) {
            format!("({})", self.lhs.math_print())
        } else {
            self.lhs.math_print()
        };
        let right = if Subtract::needs_brackets(self.rhs.as_ref()) {
            format!("({})", self.rhs.math_print())
        } else {
            self.rhs.math_print()
        };
        format!("{left} - {right}")
    }
}

impl Expression for Subtract {
    fn operation(&self) -> Operation {
        Operation::Subtract
    }
}
