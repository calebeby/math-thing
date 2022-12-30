use crate::{
    annotated_expression::{AnnotatedExpression, Annotation},
    expression::{AsExpression, Expression},
    product::Product,
    step::Step,
    sum::Sum,
    traverse::traverse,
};

pub(crate) fn simplify_excess_parens(expr: &Expression) -> Step {
    let mut steps = vec![];
    traverse(expr, |ctx| {
        let snapshot_before = ctx.snapshot();
        let mut annotations = vec![];
        match ctx.expression {
            Expression::Product(prod) => {
                if prod
                    .terms()
                    .iter()
                    .any(|term| matches!(term, Expression::Product(..)))
                {
                    let mut terms = vec![];
                    for t in prod.terms() {
                        match t {
                            Expression::Product(product) => {
                                annotations.push(Annotation {
                                    target_id: product.id(),
                                });
                                terms.extend(product.terms().iter().cloned());
                            }
                            _ => terms.push(t.clone()),
                        };
                    }
                    ctx.replace(Product::new(terms).expr());
                    steps.push(Step {
                        label: "Remove excess parentheses around product".to_owned().into(),
                        annotated_expression: Some(AnnotatedExpression {
                            expression: snapshot_before,
                            annotations,
                        }),
                        substeps: vec![],
                        result: ctx.snapshot(),
                    });
                }
            }
            Expression::Sum(prod) => {
                if prod
                    .terms()
                    .iter()
                    .any(|term| matches!(term, Expression::Sum(..)))
                {
                    let mut terms = vec![];
                    for t in prod.terms() {
                        match t {
                            Expression::Sum(sum) => {
                                annotations.push(Annotation {
                                    target_id: sum.id(),
                                });
                                terms.extend(sum.terms().iter().cloned());
                            }
                            _ => terms.push(t.clone()),
                        };
                    }
                    ctx.replace(Sum::new(terms).expr());
                    steps.push(Step {
                        label: "Remove excess parentheses around sum".to_owned().into(),
                        annotated_expression: Some(AnnotatedExpression {
                            expression: snapshot_before,
                            annotations,
                        }),
                        substeps: vec![],
                        result: ctx.snapshot(),
                    });
                }
            }
            _ => {}
        }
    });

    Step {
        label: "Simplify excess parentheses".to_owned().into(),
        annotated_expression: None,
        result: steps.last().unwrap().result.clone(),
        substeps: steps,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constant::Constant, expression::AsExpression, math};

    #[test]
    fn test_simplify_excess_parens() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let z = Constant::new("z");

        let exp = math![(x * y) * z].expr();
        insta::assert_display_snapshot!(exp, @"(x * y) * z");
        insta::assert_display_snapshot!(simplify_excess_parens(&exp), @r###"
        Simplify excess parentheses
          Remove excess parentheses around product
            (x * y) * z
             ^^^^^
            x * y * z
          x * y * z
        "###);

        let exp = math![x * (y * z)].expr();
        insta::assert_display_snapshot!(exp, @"x * (y * z)");
        insta::assert_display_snapshot!(simplify_excess_parens(&exp), @r###"
        Simplify excess parentheses
          Remove excess parentheses around product
            x * (y * z)
                 ^^^^^
            x * y * z
          x * y * z
        "###);

        let exp = math![((x * y) * (x * y)) * (z * (z * x * x))].expr();
        insta::assert_display_snapshot!(exp, @"((x * y) * (x * y)) * (z * (z * x * x))");
        insta::assert_display_snapshot!(simplify_excess_parens(&exp), @r###"
        Simplify excess parentheses
          Remove excess parentheses around product
            ((x * y) * (x * y)) * (z * (z * x * x))
              ^^^^^     ^^^^^
            (x * y * x * y) * (z * (z * x * x))
          Remove excess parentheses around product
            (x * y * x * y) * (z * (z * x * x))
                                    ^^^^^^^^^
            (x * y * x * y) * (z * z * x * x)
          Remove excess parentheses around product
            (x * y * x * y) * (z * z * x * x)
             ^^^^^^^^^^^^^     ^^^^^^^^^^^^^
            x * y * x * y * z * z * x * x
          x * y * x * y * z * z * x * x
        "###);

        let exp = math![((x + y) + y) * (x * y) * ((z * x) + y)].expr();
        insta::assert_display_snapshot!(exp, @"((x + y) + y) * (x * y) * (z * x + y)");
        insta::assert_display_snapshot!(simplify_excess_parens(&exp), @r###"
        Simplify excess parentheses
          Remove excess parentheses around sum
            ((x + y) + y) * (x * y) * (z * x + y)
              ^^^^^
            (x + y + y) * (x * y) * (z * x + y)
          Remove excess parentheses around product
            (x + y + y) * (x * y) * (z * x + y)
                           ^^^^^
            (x + y + y) * x * y * (z * x + y)
          (x + y + y) * x * y * (z * x + y)
        "###);
    }
}
