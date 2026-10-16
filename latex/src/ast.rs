//! Formula tree — own types, enums. The tree is "presentational": atoms, scripts, fractions, tables, as
//! TeX sees them, with no guesses about operator precedence (the bridge to `math` adds it). Spacing and
//! appearance commands are dropped; anything unknown is kept as an `Unknown` node with its position.

use crate::table::{Accent, Class, EnvKind, Font, FracKind, Stack, Sym, TextStyle, XArrow};

/// Byte span in the formula source. **Does not take part in `==`**: the tree from printing and the tree from
/// the source are equal if structurally equal — the round-trip gate relies on this.
#[derive(Clone, Copy, Debug, Default)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl PartialEq for Span {
    fn eq(&self, _: &Span) -> bool {
        true
    }
}
impl Eq for Span {}

/// Delimiter after `\left`, `\right`, `\middle`: a table symbol or empty (`.`).
pub type Delim = Option<Sym>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    /// Sequence of atoms. There is no single-atom row: it is the atom itself; the empty row is `{}`.
    Row(Vec<Node>),
    /// Number: digits, possibly with one decimal point (`3.14`).
    Num(Box<str>),
    /// A letter or character without a table entry.
    Char(char),
    /// Table symbol: Greek letter, operator, relation, delimiter, function.
    Sym(Sym),
    /// Explicit group `{…}` in a row.
    Group(Box<Node>),
    /// Fraction or binomial: kind, numerator, denominator.
    Frac(FracKind, Box<Node>, Box<Node>),
    /// Root: index (`\sqrt[n]`) and radicand.
    Sqrt(Option<Box<Node>>, Box<Node>),
    /// Base with scripts and primes.
    Scripts(Box<Scripts>),
    /// `\operatorname{…}`; `true` — starred form (limits below).
    OpName(Box<Node>, bool),
    /// `\left … \right`; the body may contain `Middle`.
    LeftRight(Delim, Box<Node>, Delim),
    /// `\middle|`.
    Middle(Delim),
    /// Table: matrix, `cases`, `aligned`… (and an unknown environment — with its name and position).
    Env(Box<Env>),
    Font(Font, Box<Node>),
    /// Text inside a formula — as written (canonicalized token stream).
    Text(TextStyle, Box<str>),
    Accent(Accent, Box<Node>),
    /// `\overset{script}{base}` etc.
    Stack(Stack, Box<Node>, Box<Node>),
    /// `\xrightarrow[below]{above}`.
    XArrow(XArrow, Option<Box<Node>>, Box<Node>),
    /// `\mathrel{…}` etc.: explicit atom class.
    Class(Class, Box<Node>),
    /// `\not X`, when the table has no negated symbol.
    Not(Box<Node>),
    /// Unknown command: name, group arguments, position. Never dropped silently.
    Unknown(Box<Unknown>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scripts {
    pub base: Node,
    pub sub: Option<Node>,
    pub sup: Option<Node>,
    /// Primes `f''` (come before the scripts).
    pub primes: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unknown {
    /// Name without `\`.
    pub name: Box<str>,
    /// Groups `{…}` that immediately followed the command.
    pub args: Vec<Node>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnvName {
    Known(EnvKind),
    /// Unknown environment: name and position of `\begin`.
    Unknown(Box<str>, Span),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Env {
    pub name: EnvName,
    /// Column spec of `array`/`subarray` or column count of `alignedat`.
    pub arg: Option<Box<str>>,
    /// Table rows; a cell is a node (an empty one is `Row([])`).
    pub rows: Vec<Vec<Node>>,
}

/// The empty row `{}`.
pub fn empty() -> Node {
    Node::Row(Vec::new())
}

/// Normalize an atom list: a single atom is itself, otherwise `Row`.
pub fn norm(mut items: Vec<Node>) -> Node {
    if items.len() == 1 { items.pop().unwrap() } else { Node::Row(items) }
}

impl Node {
    pub fn is_empty_row(&self) -> bool {
        matches!(self, Node::Row(v) if v.is_empty())
    }

    /// Atoms of the row (for a single node — the node itself).
    pub fn items(&self) -> &[Node] {
        match self {
            Node::Row(v) => v,
            other => std::slice::from_ref(other),
        }
    }

    /// Depth-first traversal of all nodes (the node itself first).
    pub fn walk<'n>(&'n self, f: &mut dyn FnMut(&'n Node)) {
        f(self);
        match self {
            Node::Row(v) => v.iter().for_each(|n| n.walk(f)),
            Node::Num(_) | Node::Char(_) | Node::Sym(_) | Node::Middle(_) | Node::Text(..) => {}
            Node::Group(n)
            | Node::OpName(n, _)
            | Node::Font(_, n)
            | Node::Accent(_, n)
            | Node::Class(_, n)
            | Node::Not(n)
            | Node::LeftRight(_, n, _) => n.walk(f),
            Node::Frac(_, a, b) | Node::Stack(_, a, b) => {
                a.walk(f);
                b.walk(f);
            }
            Node::Sqrt(i, b) | Node::XArrow(_, i, b) => {
                if let Some(i) = i {
                    i.walk(f);
                }
                b.walk(f);
            }
            Node::Scripts(s) => {
                s.base.walk(f);
                if let Some(n) = &s.sub {
                    n.walk(f);
                }
                if let Some(n) = &s.sup {
                    n.walk(f);
                }
            }
            Node::Env(e) => e.rows.iter().flatten().for_each(|n| n.walk(f)),
            Node::Unknown(u) => u.args.iter().for_each(|n| n.walk(f)),
        }
    }

    /// Unknown commands and environments: name (`\foo`, `\begin{foo}`) and position.
    pub fn unknowns(&self) -> Vec<(String, Span)> {
        let mut out = Vec::new();
        self.walk(&mut |n| match n {
            Node::Unknown(u) => out.push((format!("\\{}", u.name), u.span)),
            Node::Env(e) => {
                if let EnvName::Unknown(name, span) = &e.name {
                    out.push((format!("\\begin{{{name}}}"), *span));
                }
            }
            _ => {}
        });
        out
    }

    pub fn has_unknown(&self) -> bool {
        let mut found = false;
        self.walk(&mut |n| {
            if matches!(n, Node::Unknown(_)) || matches!(n, Node::Env(e) if matches!(e.name, EnvName::Unknown(..))) {
                found = true;
            }
        });
        found
    }

    /// Number of nodes (for statistics and tests).
    pub fn size(&self) -> usize {
        let mut k = 0;
        self.walk(&mut |_| k += 1);
        k
    }
}
