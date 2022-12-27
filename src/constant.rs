use std::rc::Rc;

use crate::{
    expression::Expression, token_stream::TokenStream, tokens, PrintOpts, PrintTarget, Printable,
};

struct ConstantInfo {
    name: String,
}

#[derive(Clone)]
pub(crate) struct Constant {
    info: Rc<ConstantInfo>,
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
    fn print(&self, print_opts: &PrintOpts) -> TokenStream {
        tokens!(std::iter::once(
            if matches!(print_opts.target, PrintTarget::LaTex) {
                self.info.name.to_owned()
            } else {
                latex_to_unicode(&self.info.name)
                    .unwrap_or(&self.info.name)
                    .to_owned()
            }
            .into()
        ))
    }
}

impl From<&Constant> for Expression {
    #[inline]
    fn from(constant: &Constant) -> Self {
        Expression::Constant(constant.clone().into())
    }
}
