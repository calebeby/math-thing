mod constant;
mod expression;

fn main() {}

// impl<T: Expression> Printable for T {
//     #[inline]
//     fn latex(&self) -> String {
//         (*self).latex()
//     }
//     #[inline]
//     fn math_print(&self) -> String {
//         (*self).math_print()
//     }
// }

#[derive(PartialEq)]
enum Operation {
    Multiply,
    Subtract,
    Add,
    None,
}

trait Printable {
    fn latex(&self) -> String;
    fn math_print(&self) -> String;
    fn wrap_print_parens(&self, wrap: bool) -> String {
        if wrap {
            format!("({})", self.math_print())
        } else {
            self.math_print()
        }
    }
    fn wrap_latex_parens(&self, wrap: bool) -> String {
        if wrap {
            format!("\\left({}\\right)", self.latex())
        } else {
            self.latex()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constant::Constant;

    use super::*;

    #[test]
    fn constants() {
        let pi = Constant::new(r"\pi");
        insta::assert_debug_snapshot!(pi, @"Constant π");
        let pi = Constant::new("π");
        insta::assert_debug_snapshot!(pi, @"Constant π");
    }

    #[test]
    fn multiplication() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let pi = Constant::new(r"\pi");

        let exp = &x * &y * &pi;

        insta::assert_snapshot!(exp.latex(), @r###"xy\pi"###);
        insta::assert_snapshot!(exp.math_print(), @"x * y * π");

        let exp = &x * (&y * &pi);
        insta::assert_snapshot!(exp.latex(), @r###"xy\pi"###);
        insta::assert_snapshot!(exp.math_print(), @"x * y * π");

        // Check that non-reference constants can be used
        let _exp3 = y * pi;
    }

    #[test]
    fn add_subtract_neg() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let pi = Constant::new("\\pi");

        let exp = &x + &y + &pi;

        insta::assert_snapshot!(exp.latex(), @r###"x+y+\pi"###);
        insta::assert_snapshot!(exp.math_print(), @"x + y + π");

        // TODO: should this print differently than the one below?
        let exp = &x + &y + -&pi;

        insta::assert_snapshot!(exp.latex(), @r###"x+y-\pi"###);
        insta::assert_snapshot!(exp.math_print(), @"x + y - π");

        let exp = &x + &y - &pi;

        insta::assert_snapshot!(exp.latex(), @r###"x+y-\pi"###);
        insta::assert_snapshot!(exp.math_print(), @"x + y - π");

        let exp = -(&x + &y) + &pi;

        insta::assert_snapshot!(exp.latex(), @r###"-\left(x+y\right)+\pi"###);
        insta::assert_snapshot!(exp.math_print(), @"-(x + y) + π");
    }

    #[test]
    fn add_sub_mul() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let pi = Constant::new("\\pi");

        let exp = (&x * &y) * (&y - &pi) - (&x - &pi);
        insta::assert_snapshot!(exp.latex(), @r###"xy\left(y-\pi\right)-\left(x-\pi\right)"###);
        insta::assert_snapshot!(exp.math_print(), @"(x * y * (y - π)) - (x - π)");

        let exp = -&pi * &x + &y * (-&y) * &y - &x * (&pi - &x);
        // Yes, the parens around the negative signs should be there.
        // It makes it more clear when substitutions have happened.
        // And the parens will be removed
        // when the negative sign is moved outwards during simplification steps.
        insta::assert_snapshot!(exp.latex(), @r###"\left(-\pi\right)x+y\left(-y\right)y-x\left(\pi-x\right)"###);
        insta::assert_snapshot!(exp.math_print(), @"((-π) * x) + (y * (-y) * y) - (x * (π - x))");
    }
}
