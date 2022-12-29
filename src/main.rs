use annotated_expression::Annotation;
use token_stream::TokenStream;

mod annotated_expression;
mod constant;
mod expression;
mod negation;
mod product;
mod simplify;
mod step;
mod sum;
mod token_stream;
mod traversable;

fn main() {}

#[macro_export]
macro_rules! math {
    ($a:ident) => {&$a};
    ($a:literal) => {$a};
    ({$a:expr}) => {$a};
    ((-$a:ident)) => {{
        use $crate::negation::Negation;
        Negation::new(math!($a).into())
    }};
    ((-$a:literal)) => {{
        use $crate::negation::Negation;
        Negation::new(math!($a).into())
    }};
    ((-($($a:tt)*))) => {{
        use $crate::negation::Negation;
        Negation::new(math!($($a)*).into())
    }};
    // Remove extra parentheses
    (($($a:tt)*)) => {
        math!($($a)*)
    };
    // Expand a+b+c+d...
    ($a:tt $(+ $b:tt)+) => {{
        use $crate::sum::Sum;
        Sum::new(vec![math!($a).into(), $(math!($b).into()),*])
    }};
    // Expand a-b
    ($a:tt - $b:tt) => {{
        use $crate::sum::Sum;
        use $crate::negation::Negation;
        Sum::new(vec![math!($a).into(), Negation::new(math!($b).into()).into()])
    }};
    // Expand a*b*c*d...
    ($a:tt $(* $b:tt)+) => {{
        use $crate::product::Product;
        Product::new(vec![math!($a).into(), $(math!($b).into()),*])
    }};
    // // Expand a/b
    // ($a:tt / $b:tt) => {
    //     Divide(math!($a), math!($b))
    // };
    // // Expand a^b
    // ($a:tt ^ $b:tt) => {
    //     Exponent(math!($a), math!($b))
    // };
}

pub(crate) enum PrintTarget {
    LaTex,
    MathPrint,
}

pub(crate) struct PrintOpts {
    target: PrintTarget,
}

trait Printable {
    fn latex_with_annotations(&self, annotations: &[Annotation]) -> String {
        token_stream::print(&self.print(
            &PrintOpts {
                target: PrintTarget::LaTex,
            },
            annotations,
        ))
    }
    fn latex(&self) -> String {
        self.latex_with_annotations(&[])
    }
    fn print<'a>(&'a self, print_opts: &'a PrintOpts, annotations: &[Annotation]) -> TokenStream;
    fn print_with_parens<'a>(
        &'a self,
        print_opts: &'a PrintOpts,
        annotations: &[Annotation],
    ) -> TokenStream {
        let inner = self.print(print_opts, annotations);
        if matches!(print_opts.target, PrintTarget::LaTex) {
            tokens!["\\left(", inner, "\\right)"]
        } else {
            tokens!["(", inner, ")"]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{constant::Constant, expression::AsExpression};

    use super::*;

    #[test]
    fn printing() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let pi = Constant::new(r"\pi");

        let exp = math![x * y * pi].expr();

        insta::assert_display_snapshot!(exp, @"x * y * π");
        insta::assert_snapshot!(exp.latex(), @r###"x y \pi"###);

        let exp = math![x * (y * pi)].expr();
        insta::assert_display_snapshot!(exp, @"x * (y * π)");
        insta::assert_snapshot!(exp.latex(), @r###"x \left(y \pi\right)"###);

        let exp = math![x + y + pi].expr();

        insta::assert_display_snapshot!(exp, @"x + y + π");
        insta::assert_snapshot!(exp.latex(), @r###"x+y+\pi"###);

        let exp = math![x + y + (-pi)].expr();

        insta::assert_display_snapshot!(exp, @"x + y - π");
        insta::assert_snapshot!(exp.latex(), @r###"x+y-\pi"###);

        let exp = math![(-(x + y)) + pi].expr();

        insta::assert_display_snapshot!(exp, @"-(x + y) + π");
        insta::assert_snapshot!(exp.latex(), @r###"-\left(x+y\right)+\pi"###);

        let exp = math![(x * y * (y - pi)) + (-(x - pi))].expr();
        insta::assert_display_snapshot!(exp, @"x * y * (y - π) - (x - π)");
        insta::assert_snapshot!(exp.latex(), @r###"x y \left(y-\pi\right)-\left(x-\pi\right)"###);

        let exp = math![((-pi) * x) + (y * (-y) * y) + (-(x * (pi - x)))].expr();
        // Yes, the parens around the negative signs should be there.
        // It makes it more clear when substitutions have happened.
        // And the parens will be removed
        // when the negative sign is moved outwards during simplification steps.
        insta::assert_display_snapshot!(exp, @"(-π) * x + y * (-y) * y - x * (π - x)");
        insta::assert_snapshot!(exp.latex(), @r###"\left(-\pi\right) x+y \left(-y\right) y-x \left(\pi-x\right)"###);

        let exp = math![(-(pi * x * y))].expr();
        insta::assert_display_snapshot!(exp, @"-π * x * y");
        insta::assert_snapshot!(exp.latex(), @r###"-\pi x y"###);
    }

    #[test]
    fn simplify_parens_and_negatives() {
        // let x = Constant::new("x");
        // let y = Constant::new("y");
        // let z = Constant::new("z");

        // let exp = math![(-x) * (-y) * (-z)].expr();
        // insta::assert_display_snapshot!(exp, @"(-x) * (-y) * (-z)");
        // insta::assert_display_snapshot!(exp.simplify_parens_and_negatives(), @"-x * y * z");

        // // TODO: these should all simplify fully and correctly
        // let exp = math![((-x) * (-y)) * (-z)].expr();
        // insta::assert_display_snapshot!(exp, @"((-x) * (-y)) * (-z)");
        // insta::assert_display_snapshot!(exp.simplify_parens_and_negatives(), @"-(x * y) * z");

        // let exp = math![((-x) * (-y)) * ((-z) * (-x))].expr();
        // insta::assert_display_snapshot!(exp, @"((-x) * (-y)) * ((-z) * (-x))");
        // insta::assert_display_snapshot!(exp.simplify_parens_and_negatives(), @"(x * y) * (z * x)");

        // let exp = math![(-x) * ((-z) * (-x))].expr();
        // insta::assert_display_snapshot!(exp, @"(-x) * ((-z) * (-x))");
        // insta::assert_display_snapshot!(exp.simplify_parens_and_negatives(), @"-x * (z * x)");
    }
}
