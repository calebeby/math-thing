use step::Annotation;

mod constant;
mod expression;
mod negation;
mod product;
mod step;
mod sum;

fn main() {}

#[macro_export]
macro_rules! math {
    ($a:ident) => {&$a};
    ($a:literal) => {$a};
    ((-$a:ident)) => {{
        use $crate::negation::Negation;
        Negation(math!($a).into())
    }};
    ((-$a:literal)) => {{
        use $crate::negation::Negation;
        Negation(math!($a).into())
    }};
    ((-($($a:tt)*))) => {{
        use $crate::negation::Negation;
        Negation(math!($($a)*).into())
    }};
    // Remove extra parentheses
    (($($a:tt)*)) => {
        math!($($a)*)
    };
    // Expand a+b+c+d...
    ($a:tt $(+ $b:tt)+) => {{
        use $crate::sum::Sum;
        Sum {
            terms: vec![math!($a).into(), $(math!($b).into()),*]
        }
    }};
    // Expand a-b
    ($a:tt - $b:tt) => {{
        use $crate::sum::Sum;
        use $crate::negation::Negation;
        Sum {
            terms: vec![math!($a).into(), Negation(math!($b).into()).into()]
        }
    }};
    // Expand a*b*c*d...
    ($a:tt $(* $b:tt)+) => {{
        use $crate::product::Product;
        Product {
            terms: vec![math!($a).into(), $(math!($b).into()),*]
        }
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

pub(crate) struct MathPrintResult {
    printed: String,
    annotation_indexes: Vec<(usize, usize)>,
}

impl MathPrintResult {
    pub(crate) fn to_string_with_annotations(&self) -> String {
        let mut annotations = vec![' '; self.printed.len()];
        for &(start, end) in &self.annotation_indexes {
            for i in start..end {
                annotations[i] = '^';
            }
        }
        format!(
            "{}\n{}",
            self.printed,
            annotations.iter().collect::<String>()
        )
    }
}

impl FromIterator<MathPrintResult> for MathPrintResult {
    fn from_iter<T: IntoIterator<Item = MathPrintResult>>(t: T) -> Self {
        t.into_iter().fold(
            MathPrintResult {
                printed: String::new(),
                annotation_indexes: vec![],
            },
            |mut acc, val| {
                let offset = acc.printed.len();
                acc.printed += &val.printed;
                acc.annotation_indexes.extend(
                    val.annotation_indexes
                        .iter()
                        .map(|&(start, end)| (start + offset, end + offset)),
                );
                acc
            },
        )
    }
}

#[macro_export]
macro_rules! format_with_annotations {
    ($format_str:tt, $($arg:ident),*) => {{
        use $crate::MathPrintResult;
        let printed = format!($format_str, $($arg.printed),*);
        let sub_annotations = [$(&$arg),*];
        let mut s = $format_str;
        let mut annotation_indexes = vec![];
        let mut i = 0;
        let mut last_annotation_index = 0;
        while let Some(pattern_index) = s.find("{}") {
            let offset = last_annotation_index + pattern_index;
            annotation_indexes.extend(
                sub_annotations[i]
                    .annotation_indexes
                    .iter()
                    .map(|(start, end)| (start + offset, end + offset)),
            );
            last_annotation_index += sub_annotations[i].printed.len() + offset;
            s = &s[(pattern_index + 2)..];
            i += 1;
        }
        MathPrintResult {
            printed,
            annotation_indexes,
        }
    }}
}

trait Printable {
    fn latex(&self) -> String;
    fn math_print_with_annotations(&self, annotations: &[Annotation]) -> MathPrintResult;
    fn math_print_with_parens_and_annotations(
        &self,
        annotations: &[Annotation],
    ) -> MathPrintResult {
        let inner = self.math_print_with_annotations(annotations);
        format_with_annotations!("({})", inner)
    }
    fn math_print(&self) -> String {
        self.math_print_with_annotations(&[]).printed
    }
    fn latex_with_parens(&self) -> String {
        format!("\\left({}\\right)", self.latex())
    }
}

#[cfg(test)]
mod tests {
    use crate::{constant::Constant, expression::Expression};

    use super::*;

    #[test]
    fn printing() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let pi = Constant::new(r"\pi");

        let exp = math![x * y * pi];

        insta::assert_snapshot!(exp.math_print(), @"x * y * π");
        insta::assert_snapshot!(exp.latex(), @r###"x y \pi"###);

        let exp = math![x * (y * pi)];
        insta::assert_snapshot!(exp.math_print(), @"x * (y * π)");
        insta::assert_snapshot!(exp.latex(), @r###"x \left(y \pi\right)"###);

        let exp = math![x + y + pi];

        insta::assert_snapshot!(exp.math_print(), @"x + y + π");
        insta::assert_snapshot!(exp.latex(), @r###"x+y+\pi"###);

        let exp = math![x + y + (-pi)];

        insta::assert_snapshot!(exp.math_print(), @"x + y - π");
        insta::assert_snapshot!(exp.latex(), @r###"x+y-\pi"###);

        let exp = math![(-(x + y)) + pi];

        insta::assert_snapshot!(exp.math_print(), @"-(x + y) + π");
        insta::assert_snapshot!(exp.latex(), @r###"-\left(x+y\right)+\pi"###);

        let exp = math![(x * y * (y - pi)) + (-(x - pi))];
        insta::assert_snapshot!(exp.math_print(), @"x * y * (y - π) - (x - π)");
        insta::assert_snapshot!(exp.latex(), @r###"x y \left(y-\pi\right)-\left(x-\pi\right)"###);

        let exp = math![((-pi) * x) + (y * (-y) * y) + (-(x * (pi - x)))];
        // Yes, the parens around the negative signs should be there.
        // It makes it more clear when substitutions have happened.
        // And the parens will be removed
        // when the negative sign is moved outwards during simplification steps.
        insta::assert_snapshot!(exp.math_print(), @"(-π) * x + y * (-y) * y - x * (π - x)");
        insta::assert_snapshot!(exp.latex(), @r###"\left(-\pi\right) x+y \left(-y\right) y-x \left(\pi-x\right)"###);

        let exp = math![(-(pi * x * y))];
        insta::assert_snapshot!(exp.math_print(), @"-π * x * y");
        insta::assert_snapshot!(exp.latex(), @r###"-\pi x y"###);
    }

    #[test]
    fn simplify_parens_and_negatives() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let z = Constant::new("z");

        let exp: Expression = math![(-x) * (-y) * (-z)].into();
        insta::assert_snapshot!(exp.math_print(), @"(-x) * (-y) * (-z)");
        insta::assert_snapshot!(exp.simplify_parens_and_negatives().math_print(), @"-x * y * z");

        // TODO: these should all simplify fully and correctly
        let exp: Expression = math![((-x) * (-y)) * (-z)].into();
        insta::assert_snapshot!(exp.math_print(), @"((-x) * (-y)) * (-z)");
        insta::assert_snapshot!(exp.simplify_parens_and_negatives().math_print(), @"-(x * y) * z");

        let exp: Expression = math![((-x) * (-y)) * ((-z) * (-x))].into();
        insta::assert_snapshot!(exp.math_print(), @"((-x) * (-y)) * ((-z) * (-x))");
        insta::assert_snapshot!(exp.simplify_parens_and_negatives().math_print(), @"(x * y) * (z * x)");

        let exp: Expression = math![(-x) * ((-z) * (-x))].into();
        insta::assert_snapshot!(exp.math_print(), @"(-x) * ((-z) * (-x))");
        insta::assert_snapshot!(exp.simplify_parens_and_negatives().math_print(), @"-x * (z * x)");
    }
}
