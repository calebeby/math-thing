use crate::{
    expression::{Expression, PRECEDENCE_SUM},
    Printable,
};

#[derive(Clone)]
pub(crate) struct Sum {
    pub(crate) terms: Vec<Expression>,
}

impl Printable for Sum {
    fn latex(&self) -> String {
        self.terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                if let Expression::Negation(inner) = term {
                    let inner_printed = if inner.precedence() <= PRECEDENCE_SUM {
                        inner.latex_with_parens()
                    } else {
                        inner.latex()
                    };
                    format!("-{}", inner_printed)
                } else {
                    let inner_printed = if term.precedence() <= PRECEDENCE_SUM {
                        term.latex_with_parens()
                    } else {
                        term.latex()
                    };

                    if i == 0 {
                        inner_printed
                    } else {
                        format!("+{}", inner_printed)
                    }
                }
            })
            .collect()
    }

    fn math_print(&self) -> String {
        self.terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                if let Expression::Negation(inner) = term {
                    let inner_printed = if inner.precedence() <= PRECEDENCE_SUM {
                        inner.math_print_with_parens()
                    } else {
                        inner.math_print()
                    };
                    if i == 0 {
                        format!("-{}", inner_printed)
                    } else {
                        format!(" - {}", inner_printed)
                    }
                } else {
                    let inner_printed = if term.precedence() <= PRECEDENCE_SUM {
                        term.math_print_with_parens()
                    } else {
                        term.math_print()
                    };
                    if i == 0 {
                        inner_printed
                    } else {
                        format!(" + {}", inner_printed)
                    }
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
            terms: match (self, rhs.into()) {
                (Expression::Sum(p1), Expression::Sum(p2)) => {
                    p1.terms.into_iter().chain(p2.terms.into_iter()).collect()
                }
                (Expression::Sum(p1), exp2) => {
                    p1.terms.into_iter().chain(std::iter::once(exp2)).collect()
                }
                (exp1, Expression::Sum(p2)) => {
                    std::iter::once(exp1).chain(p2.terms.into_iter()).collect()
                }
                (exp1, exp2) => vec![exp1, exp2],
            },
        })
    }
}

impl<T: Into<Expression>> std::ops::Sub<T> for Expression {
    type Output = Expression;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        let rhs_exp = -rhs.into();
        Expression::Sum(Sum {
            terms: match self {
                Expression::Sum(p1) => p1
                    .terms
                    .into_iter()
                    .chain(std::iter::once(rhs_exp))
                    .collect(),
                exp1 => vec![exp1, rhs_exp],
            },
        })
    }
}
