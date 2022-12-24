use std::rc::Rc;

use crate::{
    expression::Expression,
    step::{Annotatable, Annotation},
    MathPrintResult, Printable,
};

struct ConstantInfo {
    name: String,
}

#[derive(Clone)]
pub(crate) struct Constant {
    info: Rc<ConstantInfo>,
}

impl Annotatable for Constant {}

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

impl Printable for Constant {
    #[inline]
    fn latex(&self) -> String {
        self.info.name.to_owned()
    }
    #[inline]
    fn math_print_with_annotations(&self, _annotations: &[Annotation]) -> MathPrintResult {
        MathPrintResult {
            printed: latex_to_unicode(&self.info.name)
                .unwrap_or(&self.info.name)
                .to_owned(),
            annotation_indexes: vec![],
        }
    }
}

impl From<&Constant> for Expression {
    fn from(constant: &Constant) -> Self {
        Expression::Constant(constant.clone().into())
    }
}
