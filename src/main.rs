use std::fmt::Debug;
mod add;
mod constant;
mod multiply;
mod subtract;

use crate::constant::Constant;

fn main() {}

impl<T: Expression> Printable for &T {
    #[inline]
    fn latex(&self) -> String {
        (*self).latex()
    }
    #[inline]
    fn math_print(&self) -> String {
        (*self).math_print()
    }
}

trait Expression: Debug + Printable {
    fn operation(&self) -> Operation;
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants() {
        let pi = Constant::new(r"\pi");
        insta::assert_debug_snapshot!(pi, @r###"
        Constant {
            name: "\\pi",
        }
        "###);
        let pi = Constant::new("π");
        insta::assert_debug_snapshot!(pi, @r###"
        Constant {
            name: "\\pi",
        }
        "###);
    }

    #[test]
    fn multiplication() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let pi = Constant::new(r"\pi");

        let exp = x.clone() * y.clone() * pi.clone();
        insta::assert_debug_snapshot!(exp, @r###"
        Multiply {
            lhs: Multiply {
                lhs: Constant {
                    name: "x",
                },
                rhs: Constant {
                    name: "y",
                },
            },
            rhs: Constant {
                name: "\\pi",
            },
        }
        "###);
        insta::assert_snapshot!(exp.latex(), @r###"xy\pi"###);
        insta::assert_snapshot!(exp.math_print(), @"x * y * π");

        let exp = x.clone() * (y.clone() * pi.clone());
        insta::assert_debug_snapshot!(exp, @r###"
        Multiply {
            lhs: Constant {
                name: "x",
            },
            rhs: Multiply {
                lhs: Constant {
                    name: "y",
                },
                rhs: Constant {
                    name: "\\pi",
                },
            },
        }
        "###);
        insta::assert_snapshot!(exp.latex(), @r###"xy\pi"###);
        insta::assert_snapshot!(exp.math_print(), @"x * y * π");
    }

    #[test]
    fn add() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let pi = Constant::new("\\pi");

        let exp = (x.clone() * y.clone())
            * ((y.clone() + pi.clone() + x.clone()) + (x.clone() - pi.clone()));
        insta::assert_debug_snapshot!(exp, @r###"
        Multiply {
            lhs: Multiply {
                lhs: Constant {
                    name: "x",
                },
                rhs: Constant {
                    name: "y",
                },
            },
            rhs: Add {
                lhs: Add {
                    lhs: Add {
                        lhs: Constant {
                            name: "y",
                        },
                        rhs: Constant {
                            name: "\\pi",
                        },
                    },
                    rhs: Constant {
                        name: "x",
                    },
                },
                rhs: Subtract {
                    lhs: Constant {
                        name: "x",
                    },
                    rhs: Constant {
                        name: "\\pi",
                    },
                },
            },
        }
        "###);
        insta::assert_snapshot!(exp.latex(), @r###"xy\left(y+\pi+x+\left(x-\pi\right)\right)"###);
        insta::assert_snapshot!(exp.math_print(), @"x * y * (y + π + x + (x - π))");
    }

    #[test]
    fn add_sub_mul() {
        let x = Constant::new("x");
        let y = Constant::new("y");
        let pi = Constant::new("\\pi");

        let exp = (x.clone() * y.clone()) * ((y.clone() - pi.clone()) - (x.clone() - pi.clone()));
        insta::assert_debug_snapshot!(exp, @r###"
        Multiply {
            lhs: Multiply {
                lhs: Constant {
                    name: "x",
                },
                rhs: Constant {
                    name: "y",
                },
            },
            rhs: Subtract {
                lhs: Subtract {
                    lhs: Constant {
                        name: "y",
                    },
                    rhs: Constant {
                        name: "\\pi",
                    },
                },
                rhs: Subtract {
                    lhs: Constant {
                        name: "x",
                    },
                    rhs: Constant {
                        name: "\\pi",
                    },
                },
            },
        }
        "###);
        insta::assert_snapshot!(exp.latex(), @r###"xy\left(\left(y-\pi\right)-\left(x-\pi\right)\right)"###);
        insta::assert_snapshot!(exp.math_print(), @"x * y * ((y - π) - (x - π))");
    }
}
