//! Tree printing: canonical LaTeX (`to_latex`) and plain Unicode text (`to_text`).
//!
//! Canonical LaTeX: arguments always in braces (`x^{2}`, `\frac{1}{2}`), synonyms in the first form of
//! the table (`\leq` → `\le`), old constructs as new ones (`{a \over b}` → `{\frac{a}{b}}`,
//! `{\bf x}` → `{\mathbf{x}}`), spaces around relations and binary operations. The printout must
//! parse back into the same tree; the round-trip gate checks this (`gate.rs`).

use crate::ast::{Delim, EnvName, Node, Scripts};
use crate::table::{Accent, Class, EnvKind, Font, Stack, Sym, TextStyle, XArrow};

/// Deliberate corruption of printing, for the negative control of the round-trip gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// Swaps numerator and denominator.
    SwapFrac,
    /// Loses superscripts.
    DropSup,
    /// Loses the last atom of every row of three or more atoms.
    DropLast,
}

/// Tree → canonical LaTeX.
pub fn to_latex(n: &Node) -> String {
    let mut p = Latex { out: String::new(), word: false, fault: None, opt: false };
    p.inner(n);
    p.out
}

/// Tree → LaTeX with a deliberate defect (negative control).
pub fn to_latex_faulty(n: &Node, fault: Fault) -> String {
    let mut p = Latex { out: String::new(), word: false, fault: Some(fault), opt: false };
    p.inner(n);
    p.out
}

struct Latex {
    out: String,
    /// The last thing written was a control word (`\alpha`): a following letter needs a space.
    word: bool,
    fault: Option<Fault>,
    /// Inside `[…]` (root index, arrow label): `]` is printed as `\rbrack`.
    opt: bool,
}

impl Latex {
    /// Write a piece, inserting a space where tokens would otherwise merge.
    fn put(&mut self, s: &str) {
        let Some(first) = s.chars().next() else { return };
        let last = self.out.chars().next_back();
        let need = (self.word && first.is_ascii_alphanumeric())
            || (matches!(last, Some(c) if c.is_ascii_digit() || c == '.') && (first.is_ascii_digit() || first == '.'));
        if need {
            self.out.push(' ');
        }
        self.out.push_str(s);
        self.word = is_control_word_end(s);
    }

    fn space(&mut self) {
        if !self.out.is_empty() && !self.out.ends_with([' ', '{', '(', '[']) {
            self.out.push(' ');
        }
        self.word = false;
    }

    fn cmd(&mut self, c: &str) {
        self.put(c);
    }

    fn arg(&mut self, n: &Node) {
        self.put("{");
        self.inner(n);
        self.put("}");
    }

    fn opt_arg(&mut self, n: &Node) {
        self.put("[");
        let opt = std::mem::replace(&mut self.opt, true);
        self.inner(n);
        self.opt = opt;
        self.put("]");
    }

    /// Content without outer braces: a row as atoms, otherwise the node.
    fn inner(&mut self, n: &Node) {
        match n {
            Node::Row(v) => self.row(v),
            other => self.node(other),
        }
    }

    fn row(&mut self, items: &[Node]) {
        let items = if self.fault == Some(Fault::DropLast) && items.len() >= 3 { &items[..items.len() - 1] } else { items };
        for (i, it) in items.iter().enumerate() {
            let prev = if i > 0 { Some(&items[i - 1]) } else { None };
            match it {
                Node::Sym(s) if s.class() == Class::Rel => {
                    self.space();
                    self.node(it);
                    self.space();
                }
                Node::Sym(s) if s.class() == Class::Bin && prev.is_some_and(|p| !is_operator(p)) => {
                    self.space();
                    self.node(it);
                    self.space();
                }
                Node::Sym(s) if s.class() == Class::Punct => {
                    self.node(it);
                    self.space();
                }
                _ => self.node(it),
            }
            // `\foo` of unknown arity would swallow the next group, so separate them
            if matches!(it, Node::Unknown(_)) && items.get(i + 1).is_some_and(starts_with_brace) {
                self.cmd("\\relax");
            }
        }
    }

    fn delim(&mut self, d: Delim) {
        match d {
            None => self.put("."),
            Some(s) => self.put(s.latex()),
        }
    }

    fn node(&mut self, n: &Node) {
        match n {
            Node::Row(v) => self.row(v),
            Node::Num(s) => self.put(s),
            Node::Char(c) => {
                let mut b = [0u8; 4];
                self.put(c.encode_utf8(&mut b));
            }
            Node::Sym(Sym::RBrack) if self.opt => self.put("\\rbrack"),
            Node::Sym(s) => self.put(s.latex()),
            Node::Group(g) => {
                let opt = std::mem::replace(&mut self.opt, false);
                self.arg(g);
                self.opt = opt;
            }
            Node::Frac(k, a, b) => {
                let (a, b) = if self.fault == Some(Fault::SwapFrac) { (b, a) } else { (a, b) };
                match k.latex() {
                    Some(c) => {
                        self.cmd(c);
                        self.arg(a);
                        self.arg(b);
                    }
                    None => {
                        // infix form (\atop): it is always the only content of its group
                        self.inner(a);
                        self.space();
                        self.cmd(k.infix().unwrap_or("\\atop"));
                        self.space();
                        self.inner(b);
                    }
                }
            }
            Node::Sqrt(i, b) => {
                self.cmd("\\sqrt");
                if let Some(i) = i {
                    self.opt_arg(i);
                }
                self.arg(b);
            }
            Node::Scripts(s) => self.scripts(s),
            Node::OpName(b, lim) => {
                self.cmd(if *lim { "\\operatorname*" } else { "\\operatorname" });
                self.arg(b);
            }
            Node::LeftRight(l, b, r) => {
                self.cmd("\\left");
                self.delim(*l);
                self.inner(b);
                self.cmd("\\right");
                self.delim(*r);
            }
            Node::Middle(d) => {
                self.cmd("\\middle");
                self.delim(*d);
            }
            Node::Env(e) => {
                let (open, close): (String, String) = match &e.name {
                    EnvName::Known(k) => match k.name() {
                        Some(name) => (format!("\\begin{{{name}}}"), format!("\\end{{{name}}}")),
                        None => (format!("{}{{", k.plain().unwrap_or("\\matrix")), "}".to_string()),
                    },
                    EnvName::Unknown(name, _) => (format!("\\begin{{{name}}}"), format!("\\end{{{name}}}")),
                };
                self.put(&open);
                if let Some(a) = &e.arg {
                    self.put("{");
                    self.out.push_str(a);
                    self.word = false;
                    self.put("}");
                }
                // `\begin{aligned}[`: the parser would take `[…]` for an optional argument
                let first = e.rows.first().and_then(|r| r.first());
                if matches!(e.name, EnvName::Known(EnvKind::Aligned | EnvKind::Gathered)) && first.is_some_and(starts_with_bracket) {
                    self.cmd("\\relax");
                }
                let n = e.rows.len();
                for (ri, row) in e.rows.iter().enumerate() {
                    if ri > 0 {
                        self.space();
                        self.cmd("\\\\");
                        self.space();
                    }
                    for (ci, cell) in row.iter().enumerate() {
                        if ci > 0 {
                            self.space();
                            self.put("&");
                            self.space();
                        }
                        self.inner(cell);
                    }
                }
                // the parser drops an empty last row, so print one more `\\`
                if n > 1 && e.rows[n - 1].len() == 1 && e.rows[n - 1][0].is_empty_row() {
                    self.space();
                    self.cmd("\\\\");
                }
                self.put(&close);
            }
            Node::Font(f, b) => {
                self.cmd(f.latex());
                self.arg(b);
            }
            Node::Text(st, s) => {
                self.cmd(st.latex());
                self.put("{");
                self.out.push_str(s);
                self.word = false;
                self.put("}");
            }
            Node::Accent(a, b) => {
                self.cmd(a.latex());
                self.arg(b);
            }
            Node::Stack(k, a, b) => {
                self.cmd(k.latex());
                self.arg(a);
                self.arg(b);
            }
            Node::XArrow(k, below, above) => {
                self.cmd(k.latex());
                if let Some(b) = below {
                    self.opt_arg(b);
                }
                self.arg(above);
            }
            Node::Class(c, b) => {
                self.cmd(c.latex());
                self.arg(b);
            }
            Node::Not(b) => {
                self.cmd("\\not");
                self.arg(b);
            }
            Node::Unknown(u) => {
                self.put("\\");
                self.word = false;
                self.out.push_str(&u.name);
                self.word = u.name.bytes().all(|c| c.is_ascii_alphabetic());
                for a in &u.args {
                    self.arg(a);
                }
            }
        }
    }

    fn scripts(&mut self, s: &Scripts) {
        self.node(&s.base);
        for _ in 0..s.primes {
            self.put("'");
        }
        if let Some(sub) = &s.sub {
            self.put("_");
            self.arg(sub);
        }
        if let Some(sup) = &s.sup {
            if self.fault != Some(Fault::DropSup) {
                self.put("^");
                self.arg(sup);
            }
        }
    }
}

fn is_control_word_end(s: &str) -> bool {
    let b = s.as_bytes();
    let mut i = b.len();
    while i > 0 && b[i - 1].is_ascii_alphabetic() {
        i -= 1;
    }
    i < b.len() && i > 0 && b[i - 1] == b'\\'
}

/// The previous atom is an operator or an opening bracket: a minus after it is unary.
fn is_operator(n: &Node) -> bool {
    match n {
        Node::Sym(s) => matches!(s.class(), Class::Bin | Class::Rel | Class::Open | Class::Punct | Class::Op | Class::OpNoLim),
        Node::Scripts(s) => is_operator(&s.base),
        _ => false,
    }
}

fn starts_with_bracket(n: &Node) -> bool {
    match n {
        Node::Sym(Sym::LBrack) => true,
        Node::Scripts(s) => starts_with_bracket(&s.base),
        Node::Row(v) => v.first().is_some_and(starts_with_bracket),
        _ => false,
    }
}

fn starts_with_brace(n: &Node) -> bool {
    match n {
        Node::Group(_) => true,
        Node::Scripts(s) => starts_with_brace(&s.base),
        Node::Row(v) => v.first().is_some_and(starts_with_brace),
        _ => false,
    }
}

// ---------------------------------------------------------------- S-expression (debugging)

/// Tree → compact S-expression for humans: `(row (frac a b) \le (scripts x ^2))`.
pub fn to_sexpr(n: &Node) -> String {
    let mut s = String::new();
    sexpr(n, &mut s);
    s
}

fn sexpr(n: &Node, o: &mut String) {
    fn list(o: &mut String, head: &str, parts: &[&Node]) {
        o.push('(');
        o.push_str(head);
        for p in parts {
            o.push(' ');
            sexpr(p, o);
        }
        o.push(')');
    }
    fn delim(d: Delim) -> &'static str {
        d.map_or(".", |s| s.latex())
    }
    match n {
        Node::Row(v) => {
            o.push_str("(row");
            for x in v {
                o.push(' ');
                sexpr(x, o);
            }
            o.push(')');
        }
        Node::Num(s) => o.push_str(s),
        Node::Char(c) => o.push(*c),
        Node::Sym(s) => o.push_str(s.latex()),
        Node::Group(g) => list(o, "group", &[g]),
        Node::Frac(k, a, b) => list(o, &format!("{k:?}").to_lowercase(), &[a, b]),
        Node::Sqrt(i, b) => match i {
            Some(i) => list(o, "root", &[i, b]),
            None => list(o, "sqrt", &[b]),
        },
        Node::Scripts(s) => {
            o.push_str("(scripts ");
            sexpr(&s.base, o);
            if s.primes > 0 {
                o.push(' ');
                o.push_str(&"'".repeat(s.primes as usize));
            }
            if let Some(x) = &s.sub {
                o.push_str(" _");
                sexpr(x, o);
            }
            if let Some(x) = &s.sup {
                o.push_str(" ^");
                sexpr(x, o);
            }
            o.push(')');
        }
        Node::OpName(b, lim) => list(o, if *lim { "opname*" } else { "opname" }, &[b]),
        Node::LeftRight(l, b, r) => list(o, &format!("left{} right{}", delim(*l), delim(*r)), &[b]),
        Node::Middle(d) => {
            o.push_str("(middle ");
            o.push_str(delim(*d));
            o.push(')');
        }
        Node::Env(e) => {
            let name = match &e.name {
                EnvName::Known(k) => k.name().or(k.plain()).unwrap_or("?").trim_start_matches('\\').to_string(),
                EnvName::Unknown(n, sp) => format!("?{n}@{}", sp.start),
            };
            o.push_str("(env ");
            o.push_str(&name);
            for row in &e.rows {
                o.push_str(" [");
                for (i, c) in row.iter().enumerate() {
                    if i > 0 {
                        o.push_str(" & ");
                    }
                    sexpr(c, o);
                }
                o.push(']');
            }
            o.push(')');
        }
        Node::Font(f, b) => list(o, f.latex().trim_start_matches('\\'), &[b]),
        Node::Text(_, s) => {
            o.push_str("(text \"");
            o.push_str(s);
            o.push_str("\")");
        }
        Node::Accent(a, b) => list(o, a.latex().trim_start_matches('\\'), &[b]),
        Node::Stack(k, a, b) => list(o, k.latex().trim_start_matches('\\'), &[a, b]),
        Node::XArrow(k, below, above) => match below {
            Some(b) => list(o, k.latex().trim_start_matches('\\'), &[above, b]),
            None => list(o, k.latex().trim_start_matches('\\'), &[above]),
        },
        Node::Class(c, b) => list(o, c.latex().trim_start_matches('\\'), &[b]),
        Node::Not(b) => list(o, "not", &[b]),
        Node::Unknown(u) => {
            o.push_str(&format!("(unknown \\{}@{}", u.name, u.span.start));
            for a in &u.args {
                o.push(' ');
                sexpr(a, o);
            }
            o.push(')');
        }
    }
}

// ---------------------------------------------------------------- plain text

/// Tree → plain Unicode text: `\frac{a+b}{2}` → `(a + b)/2`, `x^{2}` → `x²`,
/// `\mathbb{R}` → `ℝ`, matrix → `[a b; c d]` (as in the computation language).
pub fn to_text(n: &Node) -> String {
    let mut out = String::new();
    text(n, &mut out);
    out.trim().to_string()
}

fn text(n: &Node, out: &mut String) {
    match n {
        Node::Row(v) => text_row(v, out),
        Node::Num(s) => out.push_str(s),
        Node::Char(c) => out.push(*c),
        Node::Sym(s) => out.push_str(s.text()),
        Node::Group(g) => text(g, out),
        Node::Frac(k, a, b) => {
            use crate::table::FracKind::*;
            match k {
                Binom | DBinom | TBinom => {
                    out.push_str("C(");
                    text(a, out);
                    out.push_str(", ");
                    text(b, out);
                    out.push(')');
                }
                Atop => {
                    text(a, out);
                    out.push_str(" / ");
                    text(b, out);
                }
                Brack | Brace => {
                    out.push_str(if *k == Brack { "[" } else { "{" });
                    text(a, out);
                    out.push(' ');
                    text(b, out);
                    out.push_str(if *k == Brack { "]" } else { "}" });
                }
                _ => {
                    wrapped(a, out);
                    out.push('/');
                    wrapped(b, out);
                }
            }
        }
        Node::Sqrt(i, b) => {
            match i.as_deref() {
                None => out.push('√'),
                Some(Node::Num(s)) if &**s == "3" => out.push('∛'),
                Some(Node::Num(s)) if &**s == "4" => out.push('∜'),
                Some(i) => {
                    let s = plain(i);
                    match superscript(&s) {
                        Some(sup) => out.push_str(&sup),
                        None => {
                            out.push_str("root[");
                            out.push_str(&s);
                            out.push(']');
                        }
                    }
                    out.push('√');
                }
            }
            wrapped(b, out);
        }
        Node::Scripts(s) => {
            text(&s.base, out);
            match s.primes {
                0 => {}
                1 => out.push('′'),
                2 => out.push('″'),
                3 => out.push('‴'),
                k => (0..k).for_each(|_| out.push('′')),
            }
            if let Some(sub) = &s.sub {
                let t = plain(sub);
                match subscript(&t.replace(' ', "")) {
                    Some(x) => out.push_str(&x),
                    None => {
                        out.push('_');
                        paren_if_long(&t, out);
                    }
                }
            }
            if let Some(sup) = &s.sup {
                let t = plain(sup);
                match superscript(&t.replace(' ', "")) {
                    Some(x) => out.push_str(&x),
                    None => {
                        out.push('^');
                        paren_if_long(&t, out);
                    }
                }
            }
        }
        Node::OpName(b, _) => {
            text(b, out);
            out.push(' ');
        }
        Node::LeftRight(l, b, r) => {
            if let Some(l) = l {
                out.push_str(l.text());
            }
            text(b, out);
            if let Some(r) = r {
                out.push_str(r.text());
            }
        }
        Node::Middle(d) => {
            if let Some(d) = d {
                out.push(' ');
                out.push_str(d.text());
                out.push(' ');
            }
        }
        Node::Env(e) => text_env(e, out),
        Node::Font(f, b) => {
            let t = plain(b);
            out.push_str(&font_text(*f, &t));
        }
        Node::Text(_, s) => out.push_str(&s.replace('$', "")),
        Node::Accent(a, b) => {
            let t = plain(b);
            match accent_mark(*a) {
                Some(mark) if t.chars().count() == 1 => {
                    out.push_str(&t);
                    out.push(mark);
                }
                _ => match a {
                    Accent::Pmod => {
                        out.push_str(" (mod ");
                        out.push_str(&t);
                        out.push(')');
                    }
                    Accent::Pod => {
                        out.push_str(" (");
                        out.push_str(&t);
                        out.push(')');
                    }
                    Accent::OverBrace | Accent::UnderBrace | Accent::Boxed | Accent::OverGroup | Accent::UnderGroup => {
                        out.push_str(&t)
                    }
                    _ => {
                        out.push_str(accent_name(*a));
                        out.push('(');
                        out.push_str(&t);
                        out.push(')');
                    }
                },
            }
        }
        Node::Stack(k, a, b) => {
            text(b, out);
            out.push_str(if *k == Stack::Under { "_(" } else { "^(" });
            text(a, out);
            out.push(')');
        }
        Node::XArrow(k, below, above) => {
            out.push_str(xarrow_text(*k));
            out.push('(');
            text(above, out);
            if let Some(b) = below {
                out.push_str("; ");
                text(b, out);
            }
            out.push(')');
        }
        Node::Class(_, b) => text(b, out),
        Node::Not(b) => {
            let t = plain(b);
            out.push_str(&t);
            out.push('\u{0338}');
        }
        Node::Unknown(u) => {
            out.push('\\');
            out.push_str(&u.name);
            for a in &u.args {
                out.push('{');
                text(a, out);
                out.push('}');
            }
        }
    }
}

fn plain(n: &Node) -> String {
    let mut s = String::new();
    text(n, &mut s);
    s.trim().to_string()
}

/// In a fraction, complex parts are bracketed: `(a + b)/2`, `1/(2π)`; `df/dx` stays as is.
fn wrapped(n: &Node, out: &mut String) {
    let t = plain(n);
    let num_then_more = matches!(n, Node::Row(v) if v.len() > 1 && matches!(v[0], Node::Num(_)));
    let simple = !matches!(n, Node::Frac(..)) && !t.contains(' ') && !num_then_more;
    if simple {
        out.push_str(&t);
    } else {
        out.push('(');
        out.push_str(&t);
        out.push(')');
    }
}

fn paren_if_long(t: &str, out: &mut String) {
    if t.chars().count() == 1 {
        out.push_str(t);
    } else {
        out.push('(');
        out.push_str(t);
        out.push(')');
    }
}

fn text_row(items: &[Node], out: &mut String) {
    for (i, it) in items.iter().enumerate() {
        let prev = if i > 0 { Some(&items[i - 1]) } else { None };
        let next = items.get(i + 1);
        match it {
            // `\frac{1}{12}n^3` → `(1/12)n³`, not `1/12n³`
            Node::Frac(k, ..)
                if !matches!(k, crate::table::FracKind::Binom | crate::table::FracKind::DBinom | crate::table::FracKind::TBinom)
                    && next.is_some_and(|n| !is_operator(n) && !matches!(n, Node::Sym(s) if s.class() == Class::Close)) =>
            {
                out.push('(');
                text(it, out);
                out.push(')');
            }
            Node::Sym(s) if s.class() == Class::Rel => {
                space(out);
                out.push_str(s.text());
                out.push(' ');
            }
            Node::Sym(s) if s.class() == Class::Bin && prev.is_some_and(|p| !is_operator(p)) => {
                space(out);
                out.push_str(s.text());
                out.push(' ');
            }
            Node::Sym(s) if s.class() == Class::Punct => {
                out.push_str(s.text());
                out.push(' ');
            }
            Node::Sym(s) if matches!(s.class(), Class::Fn | Class::FnLim) => {
                space(out);
                out.push_str(s.text());
                out.push(' ');
            }
            _ => text(it, out),
        }
    }
}

fn space(out: &mut String) {
    if !out.is_empty() && !out.ends_with([' ', '(', '[', '{']) {
        out.push(' ');
    }
}

fn text_env(e: &crate::ast::Env, out: &mut String) {
    let kind = match &e.name {
        EnvName::Known(k) => Some(*k),
        EnvName::Unknown(..) => None,
    };
    let rows: Vec<Vec<String>> = e.rows.iter().map(|r| r.iter().map(plain).collect()).collect();
    match kind {
        Some(EnvKind::Aligned) | Some(EnvKind::AlignedAt) | Some(EnvKind::Gathered) | Some(EnvKind::Split) => {
            let lines: Vec<String> = rows.iter().map(|r| r.join(" ").split_whitespace().collect::<Vec<_>>().join(" ")).collect();
            out.push_str(&lines.join("\n"));
        }
        Some(EnvKind::Cases) | Some(EnvKind::DCases) | Some(EnvKind::RCases) => {
            out.push_str("{ ");
            let lines: Vec<String> = rows.iter().map(|r| r.join(", ")).collect();
            out.push_str(&lines.join("; "));
            out.push_str(" }");
        }
        Some(EnvKind::Substack) => {
            let lines: Vec<String> = rows.iter().map(|r| r.join(" ")).collect();
            out.push_str(&lines.join(", "));
        }
        Some(EnvKind::Array) | Some(EnvKind::Subarray) | None => {
            let lines: Vec<String> = rows.iter().map(|r| r.join(" ")).collect();
            out.push_str(&lines.join("; "));
        }
        _ => {
            let (l, r) = match kind {
                Some(EnvKind::PMatrix) => ("(", ")"),
                Some(EnvKind::VMatrix) => ("|", "|"),
                Some(EnvKind::DVMatrix) => ("‖", "‖"),
                Some(EnvKind::BraceMatrix) => ("{", "}"),
                _ => ("[", "]"),
            };
            out.push_str(l);
            let lines: Vec<String> = rows.iter().map(|r| r.join(" ")).collect();
            out.push_str(&lines.join("; "));
            out.push_str(r);
        }
    }
}

fn map_chars(s: &str, f: fn(char) -> Option<char>) -> Option<String> {
    s.chars().map(f).collect()
}

fn superscript(s: &str) -> Option<String> {
    map_chars(s, |c| {
        Some(match c {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            '+' => '⁺',
            '−' | '-' => '⁻',
            '=' => '⁼',
            '(' => '⁽',
            ')' => '⁾',
            'n' => 'ⁿ',
            'i' => 'ⁱ',
            'T' => 'ᵀ',
            '*' | '∗' => '*',
            '′' => '′',
            _ => return None,
        })
    })
}

fn subscript(s: &str) -> Option<String> {
    map_chars(s, |c| {
        Some(match c {
            '0' => '₀',
            '1' => '₁',
            '2' => '₂',
            '3' => '₃',
            '4' => '₄',
            '5' => '₅',
            '6' => '₆',
            '7' => '₇',
            '8' => '₈',
            '9' => '₉',
            '+' => '₊',
            '−' | '-' => '₋',
            '=' => '₌',
            '(' => '₍',
            ')' => '₎',
            'a' => 'ₐ',
            'e' => 'ₑ',
            'h' => 'ₕ',
            'i' => 'ᵢ',
            'j' => 'ⱼ',
            'k' => 'ₖ',
            'l' => 'ₗ',
            'm' => 'ₘ',
            'n' => 'ₙ',
            'o' => 'ₒ',
            'p' => 'ₚ',
            'r' => 'ᵣ',
            's' => 'ₛ',
            't' => 'ₜ',
            'u' => 'ᵤ',
            'v' => 'ᵥ',
            'x' => 'ₓ',
            _ => return None,
        })
    })
}

/// Latin letter → Unicode math font (ℝ, 𝒜, 𝔤); anything else as is.
fn font_text(f: Font, s: &str) -> String {
    s.chars()
        .map(|c| {
            let (upper, lower): (u32, u32) = match f {
                Font::Bb => (0x1D538, 0x1D552),
                Font::Cal | Font::Scr => (0x1D49C, 0x1D4B6),
                Font::Frak => (0x1D504, 0x1D51E),
                _ => return c,
            };
            let exception = match (f, c) {
                (Font::Bb, 'C') => Some('ℂ'),
                (Font::Bb, 'H') => Some('ℍ'),
                (Font::Bb, 'N') => Some('ℕ'),
                (Font::Bb, 'P') => Some('ℙ'),
                (Font::Bb, 'Q') => Some('ℚ'),
                (Font::Bb, 'R') => Some('ℝ'),
                (Font::Bb, 'Z') => Some('ℤ'),
                (Font::Cal | Font::Scr, 'B') => Some('ℬ'),
                (Font::Cal | Font::Scr, 'E') => Some('ℰ'),
                (Font::Cal | Font::Scr, 'F') => Some('ℱ'),
                (Font::Cal | Font::Scr, 'H') => Some('ℋ'),
                (Font::Cal | Font::Scr, 'I') => Some('ℐ'),
                (Font::Cal | Font::Scr, 'L') => Some('ℒ'),
                (Font::Cal | Font::Scr, 'M') => Some('ℳ'),
                (Font::Cal | Font::Scr, 'R') => Some('ℛ'),
                (Font::Cal | Font::Scr, 'e') => Some('ℯ'),
                (Font::Cal | Font::Scr, 'g') => Some('ℊ'),
                (Font::Cal | Font::Scr, 'o') => Some('ℴ'),
                (Font::Frak, 'C') => Some('ℭ'),
                (Font::Frak, 'H') => Some('ℌ'),
                (Font::Frak, 'I') => Some('ℑ'),
                (Font::Frak, 'R') => Some('ℜ'),
                (Font::Frak, 'Z') => Some('ℨ'),
                _ => None,
            };
            if let Some(e) = exception {
                return e;
            }
            let code = if c.is_ascii_uppercase() {
                upper + (c as u32 - 'A' as u32)
            } else if c.is_ascii_lowercase() {
                lower + (c as u32 - 'a' as u32)
            } else {
                return c;
            };
            char::from_u32(code).unwrap_or(c)
        })
        .collect()
}

fn accent_mark(a: Accent) -> Option<char> {
    Some(match a {
        Accent::Hat | Accent::WideHat => '\u{0302}',
        Accent::Check | Accent::WideCheck => '\u{030C}',
        Accent::Tilde | Accent::WideTilde => '\u{0303}',
        Accent::Acute => '\u{0301}',
        Accent::Grave => '\u{0300}',
        Accent::Dot => '\u{0307}',
        Accent::DDot => '\u{0308}',
        Accent::DDDot => '\u{20DB}',
        Accent::DDDDot => '\u{20DC}',
        Accent::Breve => '\u{0306}',
        Accent::Bar | Accent::Overline => '\u{0304}',
        Accent::Vec | Accent::OverRightArrow => '\u{20D7}',
        Accent::OverLeftArrow => '\u{20D6}',
        Accent::Ring => '\u{030A}',
        Accent::Underline => '\u{0332}',
        Accent::UTilde => '\u{0330}',
        _ => return None,
    })
}

fn accent_name(a: Accent) -> &'static str {
    a.latex().trim_start_matches('\\')
}

fn xarrow_text(k: XArrow) -> &'static str {
    match k {
        XArrow::Right => "→",
        XArrow::Left => "←",
        XArrow::LeftRight => "↔",
        XArrow::DRight => "⇒",
        XArrow::DLeft => "⇐",
        XArrow::DLeftRight => "⇔",
        XArrow::MapsTo => "↦",
        XArrow::HookRight => "↪",
        XArrow::HookLeft => "↩",
        XArrow::TwoHeadRight => "↠",
        XArrow::TwoHeadLeft => "↞",
        XArrow::LongEqual => "=",
        XArrow::RightHarpoonUp => "⇀",
        XArrow::RightLeftHarpoons => "⇌",
    }
}

#[allow(dead_code)]
fn _uses(_: TextStyle, _: Sym) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;

    fn l(s: &str) -> String {
        to_latex(&parse(s).unwrap())
    }
    fn t(s: &str) -> String {
        to_text(&parse(s).unwrap())
    }

    #[test]
    fn canonical_latex() {
        assert_eq!(l("x^2+y_1"), "x^{2} + y_{1}");
        assert_eq!(l("\\frac12"), "\\frac{1}{2}");
        assert_eq!(l("a\\leq b"), "a \\le b");
        assert_eq!(l("{a\\over b}"), "{\\frac{a}{b}}");
        assert_eq!(l("\\alpha x"), "\\alpha x");
        assert_eq!(l("-x"), "-x");
        assert_eq!(l("\\sqrt[3]{x}"), "\\sqrt[3]{x}");
        assert_eq!(l("\\left(\\frac{a}{b}\\right)^2"), "\\left(\\frac{a}{b}\\right)^{2}");
        assert_eq!(l("\\begin{pmatrix}a&b\\\\c&d\\end{pmatrix}"), "\\begin{pmatrix}a & b \\\\ c & d\\end{pmatrix}");
        assert_eq!(l("f(x,y)"), "f(x, y)");
        assert_eq!(l("\\foo{x}"), "\\foo{x}");
        assert_eq!(l("1\\,000"), "1 000");
    }

    #[test]
    fn plain_text() {
        assert_eq!(t("x^2+y_1"), "x² + y₁");
        assert_eq!(t("\\frac{a+b}{2}"), "(a + b)/2");
        assert_eq!(t("\\alpha\\le\\beta"), "α ≤ β");
        assert_eq!(t("x\\in\\mathbb{R}^n"), "x ∈ ℝⁿ");
        assert_eq!(t("\\sqrt{x}"), "√x");
        assert_eq!(t("\\begin{pmatrix}a&b\\\\c&d\\end{pmatrix}"), "(a b; c d)");
        assert_eq!(t("\\sin x"), "sin x");
        assert_eq!(t("\\hat{x}"), "x\u{0302}");
    }
}
