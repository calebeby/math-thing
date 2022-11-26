use crate::{constant::Constant, Printable};

pub(crate) enum Expression {
    Constant(Constant),
    Product { terms: Vec<Expression> },
    Sum { terms: Vec<Expression> },
    Negate(Box<Expression>),
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.math_print())
    }
}

impl Printable for Expression {
    fn latex(&self) -> String {
        match self {
            Expression::Constant(constant) => constant.latex(),
            Expression::Product { terms } => terms
                .iter()
                .map(|term| {
                    term.wrap_latex_parens(!matches!(
                        term,
                        &Expression::Product { .. } | &Expression::Constant(..)
                    ))
                })
                .collect(),
            Expression::Sum { terms } => terms
                .iter()
                .enumerate()
                .map(|(i, term)| {
                    let (inner, is_neg) = if let Expression::Negate(inner_term) = term {
                        (
                            inner_term.wrap_latex_parens(!matches!(
                                inner_term.as_ref(),
                                &Expression::Constant(..) | &Expression::Product { .. }
                            )),
                            true,
                        )
                    } else {
                        (
                            term.wrap_latex_parens(!matches!(
                                term,
                                &Expression::Sum { .. }
                                    | &Expression::Constant(..)
                                    | &Expression::Product { .. }
                            )),
                            false,
                        )
                    };
                    if i != 0 {
                        if is_neg {
                            format!("-{}", inner)
                        } else {
                            format!("+{}", inner)
                        }
                    } else if is_neg {
                        format!("-{}", inner)
                    } else {
                        inner
                    }
                })
                .collect(),
            Expression::Negate(exp) => format!(
                "-{}",
                exp.wrap_latex_parens(!matches!(exp.as_ref(), &Expression::Constant(..)))
            ),
        }
    }

    fn math_print(&self) -> String {
        match self {
            Expression::Constant(constant) => constant.math_print(),
            Expression::Product { terms } => terms
                .iter()
                .enumerate()
                .map(|(i, term)| {
                    let inner = term.wrap_print_parens(!matches!(
                        term,
                        &Expression::Product { .. } | &Expression::Constant(..)
                    ));
                    if i != 0 {
                        format!(" * {}", inner)
                    } else {
                        inner
                    }
                })
                .collect(),
            Expression::Sum { terms } => terms
                .iter()
                .enumerate()
                .map(|(i, term)| {
                    let (inner, is_neg) = if let Expression::Negate(inner_term) = term {
                        (
                            inner_term.wrap_print_parens(!matches!(
                                inner_term.as_ref(),
                                &Expression::Constant(..)
                            )),
                            true,
                        )
                    } else {
                        (
                            term.wrap_print_parens(!matches!(
                                term,
                                &Expression::Sum { .. } | &Expression::Constant(..)
                            )),
                            false,
                        )
                    };
                    if i != 0 {
                        if is_neg {
                            format!(" - {}", inner)
                        } else {
                            format!(" + {}", inner)
                        }
                    } else if is_neg {
                        format!("-{}", inner)
                    } else {
                        inner
                    }
                })
                .collect(),
            Expression::Negate(exp) => format!(
                "-{}",
                exp.wrap_print_parens(!matches!(exp.as_ref(), &Expression::Constant(..)))
            ),
        }
    }
}

impl<T: Into<Expression>> std::ops::Mul<T> for Expression {
    type Output = Expression;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Expression::Product {
            terms: vec![self, rhs.into()],
        }
    }
}

impl<T: Into<Expression>> std::ops::Add<T> for Expression {
    type Output = Expression;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Expression::Sum {
            terms: vec![self, rhs.into()],
        }
    }
}

impl<T: Into<Expression>> std::ops::Sub<T> for Expression {
    type Output = Expression;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Expression::Sum {
            terms: vec![self, -rhs.into()],
        }
    }
}

impl std::ops::Neg for Expression {
    type Output = Expression;

    #[inline]
    fn neg(self) -> Self::Output {
        Expression::Negate(Box::new(self))
    }
}
