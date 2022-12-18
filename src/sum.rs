use crate::{expression::Expression, Printable};

pub(crate) struct Sum {
    pub(crate) terms: Vec<Expression>,
}

impl Printable for Sum {
    fn latex(&self) -> String {
        self.terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                let (inner, is_neg) = if let Expression::Negation(inner_term) = term {
                    (
                        match inner_term.as_ref() {
                            &Expression::Constant(..) | &Expression::Product { .. } => {
                                inner_term.latex()
                            }
                            _ => inner_term.latex_with_parens(),
                        },
                        true,
                    )
                } else {
                    (
                        match term {
                            &Expression::Sum { .. }
                            | &Expression::Constant(..)
                            | &Expression::Product { .. } => term.latex(),
                            _ => term.latex_with_parens(),
                        },
                        false,
                    )
                };
                if is_neg {
                    format!("-{}", inner)
                } else if i != 0 {
                    format!("+{}", inner)
                } else {
                    inner
                }
            })
            .collect()
    }

    fn math_print(&self) -> String {
        self.terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                let (inner, is_neg) = if let Expression::Negation(inner_term) = term {
                    (
                        match inner_term.as_ref() {
                            &Expression::Constant(..) => inner_term.math_print(),
                            _ => inner_term.math_print_with_parens(),
                        },
                        true,
                    )
                } else {
                    (
                        match term {
                            &Expression::Sum { .. } | &Expression::Constant(..) => {
                                term.math_print()
                            }
                            _ => term.math_print_with_parens(),
                        },
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
            .collect()
    }
}

impl<T: Into<Expression>> std::ops::Add<T> for Expression {
    type Output = Expression;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Expression::Sum(Sum {
            terms: vec![self, rhs.into()],
        })
    }
}

impl<T: Into<Expression>> std::ops::Sub<T> for Expression {
    type Output = Expression;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Expression::Sum(Sum {
            terms: vec![self, -rhs.into()],
        })
    }
}
