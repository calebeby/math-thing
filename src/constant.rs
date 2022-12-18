use std::rc::Rc;

use crate::{expression::Expression, product::Product, sum::Sum, Printable};

struct ConstantInfo {
    name: String,
}

#[derive(Clone)]
pub(crate) struct Constant {
    info: Rc<ConstantInfo>,
}

impl std::fmt::Debug for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Constant {}", self.math_print())
    }
}

fn latex_to_unicode(latex: &str) -> Option<&'static str> {
    match latex {
        r"\pi" => Some("π"),
        r"\rho" => Some("ρ"),
        _ => None,
    }
}

fn unicode_to_latex(unicode: &str) -> &str {
    match unicode {
        "π" => r"\pi",
        "ρ" => r"\rho",
        _ => unicode,
    }
}

impl Constant {
    pub(crate) fn new(name: &str) -> Self {
        if name.starts_with('\\') {
            // Assume latex character, look up
            if latex_to_unicode(name).is_none() {
                panic!("Unrecognized LaTeX code {name}");
            }
        }
        Constant {
            info: Rc::new(ConstantInfo {
                name: unicode_to_latex(name).to_owned(),
            }),
        }
    }
}

// impl Expression for &Constant {
//     fn operation(&self) -> Operation {
//         Operation::None
//     }
// }

impl Printable for Constant {
    #[inline]
    fn latex(&self) -> String {
        self.info.name.to_owned()
    }
    #[inline]
    fn math_print(&self) -> String {
        latex_to_unicode(&self.info.name)
            .unwrap_or(&self.info.name)
            .to_owned()
    }
}

// TODO: Borrow<Constant> type?
// impl Printable for &Constant {
//     #[inline]
//     fn latex(&self) -> String {
//         (*self).latex()
//     }

//     #[inline]
//     fn math_print(&self) -> String {
//         (*self).math_print()
//     }
// }

impl From<&Constant> for Expression {
    #[inline]
    fn from(constant: &Constant) -> Self {
        Expression::Constant(constant.clone())
    }
}
impl From<Constant> for Expression {
    #[inline]
    fn from(constant: Constant) -> Self {
        Expression::Constant(constant)
    }
}

impl<T: Into<Expression>> std::ops::Mul<T> for &Constant {
    type Output = Expression;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Expression::Product(Product {
            terms: vec![self.into(), rhs.into()],
        })
    }
}

impl<T: Into<Expression>> std::ops::Mul<T> for Constant {
    type Output = Expression;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Expression::Product(Product {
            terms: vec![self.into(), rhs.into()],
        })
    }
}

impl<T: Into<Expression>> std::ops::Add<T> for &Constant {
    type Output = Expression;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Expression::Sum(Sum {
            terms: vec![self.into(), rhs.into()],
        })
    }
}

impl<T: Into<Expression>> std::ops::Add<T> for Constant {
    type Output = Expression;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Expression::Sum(Sum {
            terms: vec![self.into(), rhs.into()],
        })
    }
}

impl<T: Into<Expression>> std::ops::Sub<T> for &Constant {
    type Output = Expression;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Expression::Sum(Sum {
            terms: vec![self.into(), -rhs.into()],
        })
    }
}
impl<T: Into<Expression>> std::ops::Sub<T> for Constant {
    type Output = Expression;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Expression::Sum(Sum {
            terms: vec![self.into(), -rhs.into()],
        })
    }
}

impl std::ops::Neg for Constant {
    type Output = Expression;

    #[inline]
    fn neg(self) -> Self::Output {
        Expression::Negation(Box::new(self.into()))
    }
}

impl std::ops::Neg for &Constant {
    type Output = Expression;

    #[inline]
    fn neg(self) -> Self::Output {
        Expression::Negation(Box::new(self.into()))
    }
}
