//! Bridge to `math::Expr` (feature `math`): examples, rejection reasons, relations, the round trip
//! `Expr → LaTeX → tree → Expr` and its negative control.
#![cfg(feature = "math")]

use latex::tomath::{Rel, fold_literals, from_expr, roundtrip, to_expr, to_relation};
use latex::{parse, to_latex};
use math::expr::{Const, Expr, Func, Var};
use math::rat::Rat;

fn v(name: &str) -> Expr {
    Expr::Var(Var::parse(name).unwrap())
}
fn n(k: i128) -> Expr {
    Expr::Num(Rat::int(k))
}
fn q(a: i128, b: i128) -> Expr {
    Expr::Num(Rat::new(a, b).unwrap())
}
fn b(e: Expr) -> Box<Expr> {
    Box::new(e)
}
fn call(f: Func, e: Expr) -> Expr {
    Expr::Call(f, vec![e])
}
fn e(src: &str) -> Expr {
    to_expr(&parse(src).unwrap()).unwrap_or_else(|why| panic!("{src}: {why}"))
}

#[test]
fn latex_to_expr() {
    let x = || v("x");
    let cases: Vec<(&str, Expr)> = vec![
        ("\\frac{x+1}{2}", Expr::Div(b(Expr::Add(vec![x(), n(1)])), b(n(2)))),
        ("2x^2-3x+1", Expr::Add(vec![Expr::Sub(b(Expr::Mul(vec![n(2), Expr::Pow(b(x()), b(n(2)))])), b(Expr::Mul(vec![n(3), x()]))), n(1)])),
        ("\\sqrt{x^2+y^2}", call(Func::Sqrt, Expr::Add(vec![Expr::Pow(b(x()), b(n(2))), Expr::Pow(b(v("y")), b(n(2)))]))),
        ("\\sqrt[3]{x}", Expr::Pow(b(x()), b(q(1, 3)))),
        ("\\sin^2 x+\\cos^2 x", Expr::Add(vec![Expr::Pow(b(call(Func::Sin, x())), b(n(2))), Expr::Pow(b(call(Func::Cos, x())), b(n(2)))])),
        ("\\sin 2x", call(Func::Sin, Expr::Mul(vec![n(2), x()]))),
        ("\\sin x \\cos x", Expr::Mul(vec![call(Func::Sin, x()), call(Func::Cos, x())])),
        ("(x+1)^2", Expr::Pow(b(Expr::Add(vec![x(), n(1)])), b(n(2)))),
        ("\\left(x+1\\right)^{2}", Expr::Pow(b(Expr::Add(vec![x(), n(1)])), b(n(2)))),
        ("|x-1|", call(Func::Abs, Expr::Sub(b(x()), b(n(1))))),
        ("|x|^2", Expr::Pow(b(call(Func::Abs, x())), b(n(2)))),
        ("e^{-x}", Expr::Pow(b(Expr::Const(Const::E)), b(Expr::Neg(b(x()))))),
        ("\\pi x^2", Expr::Mul(vec![Expr::Const(Const::Pi), Expr::Pow(b(x()), b(n(2)))])),
        ("-\\frac{1}{2}", q(-1, 2)),
        ("\\frac{2}{4}", Expr::Div(b(n(2)), b(n(4)))),
        ("0.25", q(1, 4)),
        ("3 \\cdot (-2)", Expr::Mul(vec![n(3), n(-2)])),
        ("x / y \\div z", Expr::Div(b(Expr::Div(b(x()), b(v("y")))), b(v("z")))),
        ("\\exp(x) + \\ln x", Expr::Add(vec![call(Func::Exp, x()), call(Func::Ln, x())])),
    ];
    for (src, want) in cases {
        assert_eq!(e(src), want, "{src}");
    }
}

#[test]
fn unsupported_has_a_reason() {
    let cases = [
        ("a+b", "variable"),
        ("x_1", "script"),
        ("\\log x", "function"),
        ("\\arcsin x", "function"),
        ("x=1", "relation"),
        ("\\sum_i x", "construct"),
        ("\\R", "unknown command"),
        ("\\begin{pmatrix}x\\end{pmatrix}", "construct"),
        ("\\binom{n}{2}", "construct"),
        ("(x+1", "syntax"),
        ("x+", "syntax"),
        ("\\sin", "syntax"),
        ("\\sin^{-1} x", "function"),
        ("1 2", "syntax"),
    ];
    for (src, kind) in cases {
        let why = to_expr(&parse(src).unwrap()).expect_err(src);
        assert_eq!(why.kind(), kind, "{src}: {why}");
    }
}

#[test]
fn relations_chain() {
    let (first, rest) = to_relation(&parse("x^2 = 2x+1").unwrap()).unwrap();
    assert_eq!(first, Expr::Pow(b(v("x")), b(n(2))));
    assert_eq!(rest, vec![(Rel::Eq, Expr::Add(vec![Expr::Mul(vec![n(2), v("x")]), n(1)]))]);
    let (_, rest) = to_relation(&parse("0 < x \\le 1").unwrap()).unwrap();
    assert_eq!(rest.iter().map(|r| r.0).collect::<Vec<_>>(), vec![Rel::Lt, Rel::Le]);
    assert!(to_relation(&parse("x \\in A").unwrap()).is_err());
}

#[test]
fn expr_prints_as_latex() {
    let x = || v("x");
    let ex = Expr::Sub(
        b(Expr::Div(b(Expr::Add(vec![x(), n(1)])), b(n(2)))),
        b(Expr::Mul(vec![n(3), call(Func::Sqrt, Expr::Add(vec![Expr::Pow(b(x()), b(n(2))), n(1)]))])),
    );
    assert_eq!(to_latex(&from_expr(&ex).unwrap()), "\\frac{x + 1}{2} - 3 \\cdot \\sqrt{x^{2} + 1}");
    assert_eq!(to_latex(&from_expr(&q(-3, 4)).unwrap()), "-\\frac{3}{4}");
    assert_eq!(to_latex(&from_expr(&Expr::Pow(b(call(Func::Sin, x())), b(n(2)))).unwrap()), "\\left(\\sin\\left(x\\right)\\right)^{2}");
}

fn corpus() -> Vec<Expr> {
    let (x, y, z) = (v("x"), v("y"), v("z"));
    vec![
        Expr::Add(vec![x.clone(), n(-3)]),
        Expr::Sub(b(x.clone()), b(Expr::Neg(b(y.clone())))),
        Expr::Mul(vec![n(-2), x.clone()]),
        Expr::Neg(b(Expr::Mul(vec![n(2), x.clone()]))),
        Expr::Neg(b(n(2))),
        Expr::Neg(b(Expr::Neg(b(x.clone())))),
        Expr::Pow(b(Expr::Add(vec![x.clone(), n(1)])), b(n(2))),
        Expr::Pow(b(n(-2)), b(x.clone())),
        Expr::Pow(b(call(Func::Sin, x.clone())), b(n(2))),
        Expr::Div(b(Expr::Add(vec![x.clone(), n(1)])), b(Expr::Sub(b(x.clone()), b(n(1))))),
        Expr::Div(b(n(1)), b(n(2))),
        Expr::Div(b(n(2)), b(n(4))),
        q(3, 4),
        q(-3, 4),
        call(Func::Sqrt, Expr::Add(vec![Expr::Pow(b(x.clone()), b(n(2))), n(1)])),
        call(Func::Abs, Expr::Sub(b(x.clone()), b(n(3)))),
        call(Func::Exp, Expr::Neg(b(x.clone()))),
        call(Func::Ln, x.clone()),
        call(Func::Tan, Expr::Div(b(Expr::Const(Const::Pi)), b(n(4)))),
        Expr::Add(vec![Expr::Add(vec![x.clone(), y.clone()]), z.clone()]),
        Expr::Add(vec![x.clone(), Expr::Add(vec![y.clone(), z.clone()])]),
        Expr::Mul(vec![Expr::Mul(vec![x.clone(), y.clone()]), z.clone()]),
        Expr::Sub(b(x.clone()), b(Expr::Sub(b(y.clone()), b(z.clone())))),
        Expr::Sub(b(Expr::Sub(b(x.clone()), b(y.clone()))), b(z.clone())),
        Expr::Add(vec![x.clone(), Expr::Sub(b(y.clone()), b(z.clone()))]),
        Expr::Add(vec![Expr::Sub(b(x.clone()), b(y.clone())), z.clone()]),
        Expr::Pow(b(x.clone()), b(Expr::Pow(b(y.clone()), b(n(2))))),
        Expr::Pow(b(Expr::Pow(b(x.clone()), b(n(2)))), b(y.clone())),
        Expr::Pow(b(Expr::Const(Const::E)), b(Expr::Div(b(x.clone()), b(n(2))))),
        Expr::Pow(b(q(1, 2)), b(x.clone())),
        Expr::Mul(vec![n(2), Expr::Div(b(x.clone()), b(y.clone()))]),
        Expr::Div(b(Expr::Mul(vec![x.clone(), y.clone()])), b(z.clone())),
        Expr::Pow(b(x.clone()), b(q(1, 3))),
        Expr::Mul(vec![Expr::Const(Const::Pi), Expr::Pow(b(v("t")), b(n(2)))]),
        Expr::Sub(b(call(Func::Cos, Expr::Mul(vec![n(2), x.clone()]))), b(Expr::Mul(vec![n(2), Expr::Pow(b(call(Func::Sin, x.clone())), b(n(2)))]))),
    ]
}

#[test]
fn expr_roundtrip_through_latex() {
    for ex in corpus() {
        let printed = to_latex(&from_expr(&ex).unwrap());
        assert!(roundtrip(&ex).unwrap(), "{ex:?}\n  printed: {printed}\n  back: {:?}", to_expr(&parse(&printed).unwrap()));
    }
}

/// Negative control of the bridge: a different expression gives a different tree; the comparison is not unconditional.
#[test]
fn bridge_gate_can_be_red() {
    let a = e("x-y-z");
    let b2 = e("x-(y-z)");
    assert_ne!(a, b2);
    // replace the "returned" value with a foreign expression — equality must fail
    let ex = Expr::Sub(b(v("x")), b(v("y")));
    let wrong = to_expr(&parse(&to_latex(&from_expr(&Expr::Sub(b(v("y")), b(v("x")))).unwrap())).unwrap()).unwrap();
    assert_ne!(fold_literals(&wrong), fold_literals(&ex));
}
