use crate::{Expression, Operation, Printable};

#[derive(Debug)]
pub(crate) struct Multiply {
    lhs: Box<dyn Expression>,
    rhs: Box<dyn Expression>,
}

impl Multiply {
    #[inline]
    pub(crate) fn new(lhs: Box<dyn Expression>, rhs: Box<dyn Expression>) -> Self {
        Multiply { lhs, rhs }
    }
    #[inline]
    fn needs_brackets(term: &dyn Expression) -> bool {
        !matches!(term.operation(), Operation::None | Operation::Multiply)
    }
}

impl<T: Expression + 'static> std::ops::Mul<T> for Multiply {
    type Output = Multiply;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Multiply::new(Box::new(self), Box::new(rhs))
    }
}

impl Printable for Multiply {
    fn latex(&self) -> String {
        let left = if Multiply::needs_brackets(self.lhs.as_ref()) {
            format!(r"\left({}\right)", self.lhs.latex())
        } else {
            self.lhs.latex()
        };
        let right = if Multiply::needs_brackets(self.rhs.as_ref()) {
            format!(r"\left({}\right)", self.rhs.latex())
        } else {
            self.rhs.latex()
        };
        left + &right
    }
    fn math_print(&self) -> String {
        let left_needs_brackets = Multiply::needs_brackets(self.lhs.as_ref());
        let right_needs_brackets = Multiply::needs_brackets(self.rhs.as_ref());

        match (left_needs_brackets, right_needs_brackets) {
            (true, true) => format!("({})({})", self.lhs.math_print(), self.rhs.math_print()),
            (false, true) => format!("{} * ({})", self.lhs.math_print(), self.rhs.math_print()),
            (true, false) => format!("({}) * {}", self.lhs.math_print(), self.rhs.math_print()),
            (false, false) => format!("{} * {}", self.lhs.math_print(), self.rhs.math_print()),
        }
    }
}

impl Expression for Multiply {
    fn operation(&self) -> Operation {
        Operation::Multiply
    }
}
