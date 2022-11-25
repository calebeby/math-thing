use crate::add::Add;
use crate::multiply::Multiply;
use crate::subtract::Subtract;
use crate::{Expression, Operation, Printable};

#[derive(Clone, Debug)]
pub(crate) struct Constant {
    name: String,
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
            name: unicode_to_latex(name).to_owned(),
        }
    }
}

impl Expression for Constant {
    fn operation(&self) -> Operation {
        Operation::None
    }
}

impl Printable for Constant {
    #[inline]
    fn latex(&self) -> String {
        self.name.to_owned()
    }
    #[inline]
    fn math_print(&self) -> String {
        latex_to_unicode(&self.name)
            .unwrap_or(&self.name)
            .to_owned()
    }
}

impl<T: Expression + 'static> std::ops::Mul<T> for Constant {
    type Output = Multiply;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Multiply::new(Box::new(self), Box::new(rhs))
    }
}

impl<T: Expression + 'static> std::ops::Sub<T> for Constant {
    type Output = Subtract;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Subtract::new(Box::new(self), Box::new(rhs))
    }
}

impl<T: Expression + 'static> std::ops::Add<T> for Constant {
    type Output = Add;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Add::new(Box::new(self), Box::new(rhs))
    }
}
