use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::{
    annotated_expression::Annotation,
    expression::{Expression, ExpressionId},
    token_stream::TokenStream,
    tokens,
    traversable::Traversable,
    PrintOpts, PrintTarget, Printable,
};

#[derive(Hash)]
pub(crate) struct ConstantInfo {
    pub(crate) name: String,
}

#[derive(Clone)]
pub(crate) struct Constant {
    info: Rc<ConstantInfo>,
    id: ExpressionId,
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
    pub fn new(name: &str) -> Self {
        if name.starts_with('\\') {
            // Assume latex character, look up
            if latex_to_unicode(name).is_none() {
                panic!("Unrecognized LaTeX code {name}");
            }
        }
        let constant_info = ConstantInfo {
            name: unicode_to_latex(name).to_owned(),
        };
        let mut hasher = DefaultHasher::new();
        constant_info.hash(&mut hasher);
        Constant {
            info: Rc::new(constant_info),
            id: hasher.finish(),
        }
    }
    #[inline]
    pub(crate) fn id(&self) -> ExpressionId {
        self.id
    }
}

impl Hash for Constant {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Printable for Constant {
    #[inline]
    fn print(&self, print_opts: &PrintOpts, _annotations: &[Annotation]) -> TokenStream {
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

impl Traversable for Constant {
    fn child_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Expression> + 'a> {
        Box::new(std::iter::empty())
    }

    fn from_children(original: &Constant, children: Vec<Expression>) -> Constant {
        if children.len() != 0 {
            unreachable!()
        }
        original.clone()
    }
}

impl From<&Constant> for Expression {
    #[inline]
    fn from(constant: &Constant) -> Self {
        Expression::Constant(constant.clone().into())
    }
}
