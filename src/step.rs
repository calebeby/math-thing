use crate::{
    annotated_expression::AnnotatedExpression,
    expression::{Expression, DEFAULT_PRINT_OPTS},
    PrintOpts, PrintTarget,
};

pub(crate) struct Step {
    pub(crate) label: Option<String>,
    pub(crate) annotated_expression: Option<AnnotatedExpression>,
    pub(crate) substeps: Vec<Step>,
    pub(crate) result: Expression,
}

impl Step {
    fn print<'a>(&'a self, print_opts: &'a PrintOpts) -> String {
        match print_opts.target {
            PrintTarget::LaTex => todo!(),
            PrintTarget::MathPrint => {
                let mut inner = String::new();
                if let Some(annotated_expression) = &self.annotated_expression {
                    inner.push_str(&format!("\n{}", annotated_expression));
                }
                for step in &self.substeps {
                    inner.push_str(&format!("\n{}", step.print(print_opts)));
                }
                inner.push_str(&format!("\n{}", self.result));
                if let Some(label) = &self.label {
                    format!(
                        "{label}\n{}",
                        inner
                            .trim_start()
                            .lines()
                            .enumerate()
                            .map(|(i, line)| if i == 0 {
                                format!("  {line}")
                            } else {
                                format!("\n  {line}")
                            })
                            .collect::<String>()
                    )
                } else {
                    inner
                }
            }
        }
    }
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print(&DEFAULT_PRINT_OPTS))
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_display_snapshot;

    use crate::{
        annotated_expression::Annotation, constant::Constant, expression::AsExpression, math,
        sum::Sum,
    };

    use super::*;

    fn label(string: &str) -> Option<String> {
        Some(string.to_string())
    }

    #[test]
    fn test_printing_steps() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let a = Constant::new("a");
        let b = Constant::new("b");

        let inner_1 = math![x + y].expr();
        let inner_2 = math![a + b].expr();
        let inner_1_id = inner_1.id();
        let inner_2_id = inner_2.id();
        let input_exp = math![{ inner_1.clone() } - { inner_2 }].expr();
        let final_exp = Sum::new(vec![
            math!(x).expr(),
            math!(y).expr(),
            math![(-a)].expr(),
            math![(-b)].expr(),
        ])
        .expr();

        let steps = Step {
            label: label("Simplify parentheses and negative signs:"),
            annotated_expression: None,
            substeps: vec![
                Step {
                    label: label("Distribute negative signs into parentheses"),
                    annotated_expression: Some(AnnotatedExpression {
                        expression: input_exp.clone().expr(),
                        annotations: vec![Annotation {
                            target_id: inner_2_id,
                        }],
                    }),
                    substeps: vec![],
                    result: Sum::new(vec![inner_1, math![(-a)].expr(), math![(-b)].expr()]).expr(),
                },
                Step {
                    label: label("Remove unneeded parentheses"),
                    annotated_expression: Some(AnnotatedExpression {
                        expression: input_exp.expr(),
                        annotations: vec![Annotation {
                            target_id: inner_1_id,
                        }],
                    }),
                    substeps: vec![],
                    result: final_exp.clone(),
                },
            ],
            result: final_exp,
        };
        assert_display_snapshot!(steps, @r###"
        Simplify parentheses and negative signs:
          Distribute negative signs into parentheses
            (x + y) - (a + b)
                       ^^^^^
            (x + y) - a - b
          Remove unneeded parentheses
            (x + y) - (a + b)
             ^^^^^
            x + y - a - b
          x + y - a - b
        "###);
    }
}
