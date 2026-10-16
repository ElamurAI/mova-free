//! End-to-end tests: a set of constructs (parsing without `Unknown`, round trip, canonical printing
//! is idempotent), golden outputs, negative controls (unbalanced → error with a position; corrupted
//! printing → the gate turns red), macros, formula extraction from a document.

use latex::gate::{self, RoundTrip};
use latex::print::{Fault, to_latex_faulty};
use latex::{ErrorKind, Macros, Node, extract, parse, parse_with, to_latex, to_text};

/// Constructs from the brief and from arXiv abstracts. Each one: no `Unknown`, the round trip is green.
const SUITE: &[&str] = &[
    // numbers, symbols, Greek
    "3.14", "x", "\\alpha+\\beta-\\Gamma", "\\varepsilon\\epsilon\\vartheta\\varphi\\ell\\hbar", "12a_{ij}",
    "α≤β×γ", "ℝ^n", "\\infty \\partial \\nabla \\forall \\exists \\emptyset",
    // fractions, roots, scripts, groups
    "\\frac{a}{b}", "\\frac12", "\\dfrac{1}{x}+\\tfrac{1}{2}+\\cfrac{1}{1+\\cfrac{1}{x}}", "\\sqrt{x}",
    "\\sqrt[n]{x^n}", "x_1^2", "x^{2}_{1}", "{x^2}^3", "a^{b^{c}}", "f'(x)+f''(x)", "f'^2", "x_1'",
    "^{14}C", "{}_3F_2", "{a+b}", "{}", "\\binom{n}{k}+\\dbinom nk", "{n \\choose k}", "{a \\over b+c}",
    "{n \\atop k}", "{n \\brack k}_q", "\\genfrac{(}{)}{0pt}{}{n}{k}", "\\root 3 \\of x",
    // operators and relations
    "a+b-c\\cdot d\\times e\\div f", "a=b\\neq c\\le d\\ge e<f>g", "a\\leq b\\geq c", "x\\in A\\subseteq B\\cup C",
    "a\\equiv b\\pmod{n}", "a \\bmod n", "\\pm 1 \\mp 2", "p \\Rightarrow q \\iff r", "f\\colon A\\to B",
    "x\\mapsto x^2", "A\\setminus B", "\\not\\in", "\\not=", "\\not\\approx", "a \\ll b \\sim c \\approx d",
    // functions
    "\\sin x+\\cos(x)+\\tan^2 x", "\\log_2 n + \\ln x + \\exp(x)", "\\operatorname{rank} A", "\\operatorname*{arg\\,max}_x f(x)",
    "\\sinh x \\cosh x \\arcsin x \\det A \\gcd(a,b)",
    // large operators with limits
    "\\sum_{i=1}^{n} i", "\\prod_{k} a_k", "\\int_0^1 f(x)\\,dx", "\\iint_D f \\, dA", "\\oint_C \\omega",
    "\\lim_{x\\to 0}\\frac{\\sin x}{x}=1", "\\sum\\limits_{i}\\nolimits a_i", "\\bigcup_{i\\in I} A_i", "\\max_{x} f", "\\limsup_{n\\to\\infty} a_n",
    // \left … \right
    "\\left(\\frac{a}{b}\\right)^2", "\\left.\\frac{df}{dx}\\right|_{x=0}", "\\left\\{ x \\middle| x>0 \\right\\}",
    "\\left\\langle u, v \\right\\rangle", "\\left\\lfloor x \\right\\rfloor", "\\left[ 0, 1 \\right)", "\\bigl( x \\bigr)",
    // matrices and tables
    "\\begin{pmatrix} a & b \\\\ c & d \\end{pmatrix}", "\\begin{bmatrix} 1 & 0 \\\\ 0 & 1 \\end{bmatrix}",
    "\\begin{vmatrix} a & b \\\\ c & d \\end{vmatrix}", "\\begin{Vmatrix} x \\end{Vmatrix}", "\\begin{Bmatrix} x \\end{Bmatrix}",
    "\\begin{array}{c|cc} 1 & 2 & 3 \\\\ \\hline 4 & 5 & 6 \\end{array}", "\\begin{matrix} a \\\\ \\\\ \\end{matrix}",
    "\\begin{cases} x, & x \\ge 0 \\\\ -x, & \\text{otherwise} \\end{cases}", "\\begin{smallmatrix} a&b \\end{smallmatrix}",
    "\\begin{aligned} a &= b \\\\ &= c \\end{aligned}", "a &= b \\\\ c &= d", "\\begin{gathered} a \\\\ b \\end{gathered}",
    "\\begin{alignedat}{2} a &= b &\\quad& c \\end{alignedat}", "\\sum_{\\substack{i<n \\\\ j<m}} a_{ij}", "\\pmatrix{a & b \\cr c & d}",
    "\\begin{pmatrix}\\end{pmatrix}", "\\begin{aligned}[t] [a] &= b\\end{aligned}",
    // fonts, text
    "\\mathbb{R}^n", "\\mathcal{O}(n)", "\\mathrm{d}x", "\\mathbf{v}\\cdot\\boldsymbol{\\omega}", "\\mathfrak{g}", "\\mathscr{L}",
    "{\\bf x}+{\\rm d}y+{\\cal A}", "\\text{if } x>0", "\\textbf{bold}\\mbox{ and }\\textit{it}", "\\text{for all $x \\in A$}",
    // accents, over/under-sets
    "\\hat{x}+\\bar{y}+\\vec{v}+\\tilde{z}+\\dot{x}+\\ddot{x}", "\\overline{AB}", "\\widehat{abc}", "\\underbrace{a+b}_{2}",
    "\\overset{def}{=}", "\\stackrel{?}{=}", "\\underset{x}{\\min}", "\\xrightarrow[below]{above}", "\\boxed{E=mc^2}",
    "\\mathrel{\\sim}", "\\mathop{\\rm Tr} A",
    // spaces are ignored, number splitting
    "a\\,b\\;c\\quad d\\qquad e\\!f", "1\\,000", "x \\hspace{1cm} y", "x\\kern3pt y\\mkern-2mu z", "a~b",
    "\\displaystyle\\sum_i x_i", "x \\label{eq} \\tag{1} \\nonumber", "\\color{red} x + \\textcolor{blue}{y}",
    "\\phantom{x}y\\smash{z}", "\\ensuremath{x}",
    // miscellaneous from arXiv
    "\\#P", "O(n\\log n)", "|x|+\\|y\\|", "\\langle x, y\\rangle", "[0,1)", "\\{x : x > 0\\}", "2^{\\aleph_0}",
    "\\ldots \\cdots \\dots \\vdots \\ddots", "\\lVert x \\rVert", "\\unicode{x2208}", "x\\slash y",
    "\\left\\{\\aligned &a=b \\\\ &c=d \\endaligned\\right.", "x=1 \\eqno(1)", "\\sqrt[\\rbrack]{x}",
];

fn ok(src: &str) -> Node {
    parse(src).unwrap_or_else(|e| panic!("{src}: {e} — {}", e.context(src)))
}

#[test]
fn suite_parses_clean_and_roundtrips() {
    let mut fails = Vec::new();
    for &src in SUITE {
        let t = ok(src);
        if !t.unknowns().is_empty() {
            fails.push(format!("{src}: unknown {:?}", t.unknowns()));
        }
        match gate::check(&t) {
            RoundTrip::Ok => {}
            other => fails.push(format!("{src}: {other:?}")),
        }
        // canonical printing is a fixed point
        let once = to_latex(&t);
        let twice = to_latex(&ok(&once));
        if once != twice {
            fails.push(format!("{src}: printing is not idempotent: {once} → {twice}"));
        }
    }
    assert!(fails.is_empty(), "{}", fails.join("\n"));
}

#[test]
fn golden_canonical_latex() {
    let cases = [
        ("x^2+y_1", "x^{2} + y_{1}"),
        ("\\frac12", "\\frac{1}{2}"),
        ("a\\leq b\\neq c", "a \\le b \\ne c"),
        ("{a\\over b}", "{\\frac{a}{b}}"),
        ("{n\\choose k}", "{\\binom{n}{k}}"),
        ("{\\bf x}", "{\\mathbf{x}}"),
        ("\\not\\in", "\\notin"),
        ("\\root 3 \\of x", "\\sqrt[3]{x}"),
        ("\\sqrt[3]{x}", "\\sqrt[3]{x}"),
        ("\\left(\\frac{a}{b}\\right)^2", "\\left(\\frac{a}{b}\\right)^{2}"),
        ("\\sum_{i=1}^n i", "\\sum_{i = 1}^{n}i"),
        ("\\begin{pmatrix}a&b\\\\c&d\\end{pmatrix}", "\\begin{pmatrix}a & b \\\\ c & d\\end{pmatrix}"),
        ("a &= b \\\\ &= c", "\\begin{aligned}a & = b \\\\ & = c\\end{aligned}"),
        ("\\pmatrix{a&b\\cr c&d}", "\\begin{pmatrix}a & b \\\\ c & d\\end{pmatrix}"),
        ("-x+(-y)", "-x + (-y)"),
        ("f(x,y)", "f(x, y)"),
        ("\\alpha x\\cdot 3", "\\alpha x \\cdot 3"),
        ("1\\,000", "1 000"),
        ("\\foo{x}", "\\foo{x}"),
        ("\\text{a\\ \\ b}", "\\text{a b}"),
        ("#P", "\\#P"),
        ("\\genfrac{}{}{0pt}{}{a}{b}", "{a \\atop b}"),
    ];
    for (src, want) in cases {
        assert_eq!(to_latex(&ok(src)), want, "{src}");
    }
}

#[test]
fn golden_plain_text() {
    let cases = [
        ("x^2+y_1", "x² + y₁"),
        ("\\frac{a+b}{2}", "(a + b)/2"),
        ("\\frac{df}{dx}", "df/dx"),
        ("\\alpha\\le\\beta", "α ≤ β"),
        ("x\\in\\mathbb{R}^n", "x ∈ ℝⁿ"),
        ("\\sqrt{x}+\\sqrt[3]{y}", "√x + ∛y"),
        ("\\begin{pmatrix}a&b\\\\c&d\\end{pmatrix}", "(a b; c d)"),
        ("\\sin x", "sin x"),
        ("x^{n+1}", "xⁿ⁺¹"),
        ("\\mathcal{L}", "ℒ"),
        ("f'(x)", "f′(x)"),
        ("\\binom{n}{k}", "C(n, k)"),
    ];
    for (src, want) in cases {
        assert_eq!(to_text(&ok(src)), want, "{src}");
    }
}

#[test]
fn unknown_commands_are_kept_with_name_position_args() {
    let src = "a + \\norm{x}_2 + \\R^n + \\begin{CD} A @>>> B \\end{CD}";
    let t = ok(src);
    let u = t.unknowns();
    let names: Vec<&str> = u.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, vec!["\\norm", "\\R", "\\begin{CD}"]);
    assert_eq!(&src[u[0].1.start as usize..u[0].1.end as usize], "\\norm");
    assert_eq!(&src[u[1].1.start as usize..u[1].1.end as usize], "\\R");
    assert!(src[u[2].1.start as usize..].starts_with("\\begin"));
    assert!(gate::check(&t).is_ok());
    // arguments are kept and printed back
    assert_eq!(to_latex(&t), "a + \\norm{x}_{2} + \\R^{n} + \\begin{CD}A@ > > > B\\end{CD}");
}

/// Negative control of parsing: unbalanced input is an error with the exact position.
#[test]
fn unbalanced_is_an_error_with_position() {
    let cases: &[(&str, ErrorKind, u32)] = &[
        ("\\frac{a}{b", ErrorKind::UnclosedGroup, 8),
        ("{x", ErrorKind::UnclosedGroup, 0),
        ("a+{b+{c}", ErrorKind::UnclosedGroup, 2),
        ("x}", ErrorKind::ExtraClose, 1),
        ("\\sqrt{x}}+1", ErrorKind::ExtraClose, 8),
        ("\\left( x", ErrorKind::MissingRight, 0),
        ("a \\right)", ErrorKind::ExtraRight, 2),
        ("\\left x \\right)", ErrorKind::BadDelim, 6),
        ("\\begin{pmatrix} a", ErrorKind::UnclosedEnv("pmatrix".into()), 0),
        ("\\end{pmatrix}", ErrorKind::ExtraEnd("pmatrix".into()), 0),
        ("\\sqrt[3{x}", ErrorKind::UnclosedOpt, 5),
        ("x^", ErrorKind::MissingArg("^"), 1),
        ("x^2^3", ErrorKind::DoubleSup, 3),
        ("x_1_2", ErrorKind::DoubleSub, 3),
        ("{a & b}", ErrorKind::MisplacedAmp, 3),
        ("{a \\\\ b}", ErrorKind::MisplacedRowSep, 3),
        ("a $ b", ErrorKind::Dollar, 2),
    ];
    for (src, kind, pos) in cases {
        let e = parse(src).expect_err(src);
        assert_eq!((&e.kind, e.pos), (kind, *pos), "{src}: {e}");
    }
    // position in characters for humans, and a location mark
    let e = parse("α+\\frac{β}{γ").unwrap_err();
    assert_eq!(e.column("α+\\frac{β}{γ"), 10);
    assert_eq!(e.context("α+\\frac{β}{γ"), "α+\\frac{β}⟦{γ");
}

/// Negative control of the gate: corrupted printing turns the gate red; correct printing keeps it green.
#[test]
fn corrupted_printer_turns_gate_red() {
    let affected = ["\\frac{a}{b}", "x^2", "\\sum_{i=1}^n x_i^2", "a+b+c", "\\frac{1}{x}+y^{n}"];
    for src in affected {
        let t = ok(src);
        assert!(gate::check(&t).is_ok(), "{src}");
        let red = [Fault::SwapFrac, Fault::DropSup, Fault::DropLast].iter().any(|f| !gate::check_faulty(&t, *f).is_ok());
        assert!(red, "{src}: no defect turned the gate red");
    }
    // a defect that does not affect the formula leaves the gate green — red is not unconditional
    let t = ok("x+y");
    assert!(gate::check_faulty(&t, Fault::SwapFrac).is_ok());
    assert_ne!(to_latex_faulty(&ok("\\frac{a}{b}"), Fault::SwapFrac), to_latex(&ok("\\frac{a}{b}")));
}

#[test]
fn macros_newcommand_def_let_declareop() {
    let doc = r"\newcommand{\R}{\mathbb{R}} \newcommand{\ip}[2][\cdot]{\langle #1, #2 \rangle}
        \def\half{\frac{1}{2}} \def\pair(#1,#2){(#1; #2)} \DeclareMathOperator{\tr}{tr}
        \let\oldsqrt\sqrt \renewcommand{\sqrt}[1]{\oldsqrt{#1}}";
    let mut m = Macros::new();
    let st = m.scan(doc);
    assert_eq!((st.defined, st.skipped), (7, 0));
    let same = |a: &str, b: &str| assert_eq!(parse_with(a, &m).unwrap(), parse(b).unwrap(), "{a}");
    same("x \\in \\R^n", "x \\in \\mathbb{R}^n");
    same("\\ip{u}", "\\langle \\cdot, u \\rangle");
    same("\\ip[w]{u}", "\\langle w, u \\rangle");
    same("\\half x", "\\frac{1}{2} x");
    same("\\pair(a,b)", "(a; b)");
    same("\\tr A", "\\operatorname{tr} A");
    // \let kept the old value of \sqrt — no loop
    same("\\sqrt{2}", "\\sqrt{2}");
    // definition inside a formula
    assert_eq!(parse("\\newcommand{\\f}[1]{#1^2} \\f{x}+\\f y").unwrap(), parse("x^2+y^2").unwrap());
    // a loop is an error, not a hang
    assert_eq!(parse("\\def\\a{\\a x}\\a").unwrap_err().kind, ErrorKind::MacroLoop);
    assert_eq!(parse("\\def\\b{\\b\\b}\\b").unwrap_err().kind, ErrorKind::MacroLoop);
}

#[test]
fn document_extraction_with_preamble_macros() {
    let tex = include_str!("data/sample.tex");
    let mut m = Macros::new();
    let st = m.scan(tex);
    assert_eq!(st.skipped, 0);
    let (formulas, bad) = extract(tex);
    assert!(bad.is_empty(), "{bad:?}");
    let envs: Vec<Option<&str>> = formulas.iter().map(|f| f.env).collect();
    assert_eq!(
        envs,
        vec![None, None, None, None, Some("equation"), Some("align*"), Some("gather"), None, Some("alignat"), Some("multline"), None]
    );
    for f in &formulas {
        assert!(!f.body.contains("notaformula") && !f.body.contains("this is not math"));
        let t = parse_with(f.body, &m).unwrap_or_else(|e| panic!("{}: {e}", f.body));
        assert!(t.unknowns().is_empty(), "{}: {:?}", f.body, t.unknowns());
        assert!(gate::check(&t).is_ok(), "{}", f.body);
    }
}

/// Property on random inputs (deterministic generator): the parser does not panic, and everything that
/// parsed passes the round trip and has idempotent printing.
#[test]
fn random_token_soup_never_panics_and_roundtrips() {
    const PIECES: &[&str] = &[
        "x", "y", "1", "2", "0", ".", ",", "+", "-", "=", "<", "^", "_", "{", "}", "{", "}", "(", ")", "[", "]", "|",
        "'", " ", "\\frac", "\\sqrt", "\\left(", "\\right)", "\\left.", "\\right|", "\\middle|", "\\alpha", "\\over",
        "\\atop", "&", "\\\\", "\\begin{pmatrix}", "\\end{pmatrix}", "\\begin{cases}", "\\end{cases}", "\\,", "\\quad",
        "\\text{a b}", "\\text{$z$}", "\\mathbb", "\\bf", "\\rm", "\\foo", "\\not", "\\|", "\\sum", "\\limits", "\\int",
        "\\sin", "\\operatorname", "\\hat", "\\overset", "\\xrightarrow", "\\substack", "\\pmatrix", "\\genfrac",
        "\\newcommand{\\q}[1]{#1^2}", "\\q", "\\def\\p#1{#1#1}", "\\p", "#", "#1", "$", "%", "\n", "~", "\\unicode{x3b1}",
        "\\root", "\\of", "\\aligned", "\\endaligned", "\\eqno", "\\brack", "\\textcolor{red}", "\\phantom", "\\mbox",
    ];
    let mut seed: u64 = 0x9E3779B97F4A7C15;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    let (mut parsed, mut total) = (0, 0);
    for _ in 0..30_000 {
        let len = 1 + (next() % 14) as usize;
        let src: String = (0..len).map(|_| PIECES[(next() % PIECES.len() as u64) as usize]).collect();
        total += 1;
        let Ok(t) = parse(&src) else { continue };
        parsed += 1;
        match gate::check(&t) {
            RoundTrip::Ok => {}
            other => panic!("{src:?}: {other:?}"),
        }
        let once = to_latex(&t);
        assert_eq!(to_latex(&ok(&once)), once, "{src:?}");
        let _ = to_text(&t);
    }
    // the generator must produce both parsed inputs and errors — otherwise the check is empty
    assert!(parsed > total / 10 && parsed < total, "parsed {parsed} of {total}");
}
