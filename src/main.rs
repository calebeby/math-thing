use std::fs::File;
use std::io::Write;

use annotated_expression::Annotation;
use constant::Constant;
use expression::{AsExpression, DEFAULT_PRINT_OPTS};
use simplify::simplify_excess_parens;
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
mod traverse;

fn main() {
    let a = Constant::new("a");
    let b = Constant::new("b");
    let pi = Constant::new("\\pi");
    let x = Constant::new("x");
    let y = Constant::new("y");
    let exp = math![
        (a + b + (x + y) + ((x - y) + ((x + y) + ((a * pi) * ((pi + x) * y)) + ((pi + x) + y))))
    ]
    .expr();

    let steps = simplify_excess_parens(&exp);
    println!("{}", steps);

    let css = r###"
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.4/dist/katex.min.css" integrity="sha384-vKruj+a13U8yHIkAyGgK1J3ArTLzrFGBbBc0tDp4ad/EyewESeXE/Iv67Aj8gKZ0" crossorigin="anonymous">
        <style>
            body {
                font-size: 20px;
            }
            .katex .enclosing.hl {
              background: #ffcb0094;
              display: inline-block;
              --padding-x: 0.05em;
              --padding-y: 0em;
              padding: var(--padding-y) var(--padding-x);
              margin: calc(-1 * var(--padding-y)) calc(-1 * var(--padding-x));
              border-radius: 0.3em;
            }
            .substeps {
                margin-left: 2em;
                position: relative;
            }
            .substeps::before {
                display: block;
                content: '';
                width: 0.3em;
                height: 100%;
                position: absolute;
                background: #0000002e;
                left: -2em;
                border-radius: 1000px;
            }
        </style>
    "###;
    let steps_html = format!(
        "{css}\n{}",
        steps.html_print(&PrintOpts {
            target: PrintTarget::LaTex,
        })
    );

    let mut file = File::create("example.html").unwrap();
    writeln!(&mut file, "{}", steps_html).unwrap();
}

#[macro_export]
macro_rules! math {
    ($a:ident) => {&$a};
    ($a:literal) => {$a};
    ({$a:expr}) => {$a};
    ((-$a:ident)) => {
        $crate::negation::Negation::new(math!($a).into())
    };
    ((-$a:literal)) => {
        $crate::negation::Negation::new(math!($a).into())
    };
    ((-($($a:tt)*))) => {
        $crate::negation::Negation::new(math!($($a)*).into())
    };
    // Remove extra parentheses
    (($($a:tt)*)) => {
        math!($($a)*)
    };
    // Expand a+b+c+d...
    ($a:tt $(+ $b:tt)+) => {
        $crate::sum::Sum::new(vec![math!($a).into(), $(math!($b).into()),*])
    };
    // Expand a-b
    ($a:tt - $b:tt) => {
        $crate::sum::Sum::new(vec![
            math!($a).into(),
            $crate::negation::Negation::new(math!($b).into()).into(),
        ])
    };
    // Expand a*b*c*d...
    ($a:tt $(* $b:tt)+) => {
        $crate::product::Product::new(vec![math!($a).into(), $(math!($b).into()),*])
    };
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
        token_stream::math_print(&self.print(
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
