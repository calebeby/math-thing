mod constant;
mod expression;
mod product;
mod sum;

fn main() {}

trait Printable {
    fn latex(&self) -> String;
    fn math_print(&self) -> String;
    fn math_print_with_parens(&self) -> String {
        format!("({})", self.math_print())
    }
    fn latex_with_parens(&self) -> String {
        format!("\\left({}\\right)", self.latex())
    }
}

#[cfg(test)]
mod tests {
    use crate::{constant::Constant, expression::Expression, product::Product};

    use super::*;

    #[test]
    fn printing() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let pi = Constant::new(r"\pi");

        let exp = &x * &y * &pi;

        insta::assert_snapshot!(exp.math_print(), @"x * y * π");
        insta::assert_snapshot!(exp.latex(), @r###"x y \pi"###);

        let exp = &x * (&y * &pi);
        println!("{exp:?}");
        insta::assert_snapshot!(exp.math_print(), @"x * y * π");
        insta::assert_snapshot!(exp.latex(), @r###"x y \pi"###);

        let exp = &x + &y + &pi;

        insta::assert_snapshot!(exp.math_print(), @"x + y + π");
        insta::assert_snapshot!(exp.latex(), @r###"x+y+\pi"###);

        let exp = &x + &y + -&pi;

        insta::assert_snapshot!(exp.math_print(), @"x + y - π");
        insta::assert_snapshot!(exp.latex(), @r###"x+y-\pi"###);

        let exp = &x + &y - &pi;

        insta::assert_snapshot!(exp.math_print(), @"x + y - π");
        insta::assert_snapshot!(exp.latex(), @r###"x+y-\pi"###);

        let exp = -(&x + &y) + &pi;

        insta::assert_snapshot!(exp.math_print(), @"-(x + y) + π");
        insta::assert_snapshot!(exp.latex(), @r###"-\left(x+y\right)+\pi"###);

        let exp = (&x * &y) * (&y - &pi) - (&x - &pi);
        insta::assert_snapshot!(exp.math_print(), @"x * y * (y - π) - (x - π)");
        insta::assert_snapshot!(exp.latex(), @r###"x y \left(y-\pi\right)-\left(x-\pi\right)"###);

        let exp = -&pi * &x + &y * (-&y) * &y - &x * (&pi - &x);
        // Yes, the parens around the negative signs should be there.
        // It makes it more clear when substitutions have happened.
        // And the parens will be removed
        // when the negative sign is moved outwards during simplification steps.
        insta::assert_snapshot!(exp.math_print(), @"(-π) * x + y * (-y) * y - x * (π - x)");
        insta::assert_snapshot!(exp.latex(), @r###"\left(-\pi\right) x+y \left(-y\right) y-x \left(\pi-x\right)"###);

        let exp = -(&pi * &x * &y);
        // Yes, the parens around the negative signs should be there.
        // It makes it more clear when substitutions have happened.
        // And the parens will be removed
        // when the negative sign is moved outwards during simplification steps.
        insta::assert_snapshot!(exp.math_print(), @"-π * x * y");
        insta::assert_snapshot!(exp.latex(), @r###"-\pi x y"###);

        // Check that non-reference constants can be used
        let exp = y * pi;
        insta::assert_snapshot!(exp.math_print(), @"y * π");
        insta::assert_snapshot!(exp.latex(), @r###"y \pi"###);
    }

    #[test]
    fn simplify_parens_and_negatives() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let z = Constant::new("z");

        let exp = -&x * -&y * -&z;
        insta::assert_snapshot!(exp.math_print(), @"(-x) * (-y) * (-z)");
        insta::assert_snapshot!(exp.simplify_parens_and_negatives().math_print(), @"-x * y * z");

        // The mul operator implementation automatically collapses nested Products
        // We can manually construct nested products to test collapsing them.
        // TODO: these should all simplify correctly
        let exp: Expression = Product::new(Product::new(-&x, -&y).into(), -&z).into();
        insta::assert_snapshot!(exp.math_print(), @"((-x) * (-y)) * (-z)");
        insta::assert_snapshot!(exp.simplify_parens_and_negatives().math_print(), @"-(x * y) * z");

        let exp: Expression =
            Product::new(Product::new(-&x, -&y).into(), Product::new(-&z, -&x).into()).into();
        insta::assert_snapshot!(exp.math_print(), @"((-x) * (-y)) * ((-z) * (-x))");
        insta::assert_snapshot!(exp.simplify_parens_and_negatives().math_print(), @"(x * y) * (z * x)");

        let exp: Expression = Product::new(-&x, Product::new(-&z, -&x).into()).into();
        insta::assert_snapshot!(exp.math_print(), @"(-x) * ((-z) * (-x))");
        insta::assert_snapshot!(exp.simplify_parens_and_negatives().math_print(), @"-x * (z * x)");
    }
}
