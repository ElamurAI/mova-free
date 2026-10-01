//! Understanding: English question → expression (`Expr`) via the UD tree from `en`.
//!
//! 1. Tokens — the `en` tokenizer; small boundary fixes («C.» → «C» «.»).
//! 2. Dictionary chunker (`lex`, enums): numbers (digits and words: «two dozen», «a third»), quantities
//!    with units («72 °F», «10 miles per hour»), bare units («in km», «to Celsius»).
//! 3. UD tree: each quantity becomes a placeholder of one or two tokens («72 degrees», «10 units») so
//!    the parser does not stumble on «°» and «F»; then the `en` parse and mapping of nodes onto items.
//! 4. Frames by lexical triggers; roles by the tree:
//!    - condition clause — `advcl` with `mark` if/when and a phrase with `case` with (plus preceding sentences);
//!    - case markers of quantities — children with `case` (of/from/to/in/between/by/than/for/on);
//!    - a function's argument list — the subtree of its «of» phrase.
//!    Linear order is only a fallback (also for arithmetic chains «12 plus 30», «3 + 4 * 2»,
//!    where UD for «times/plus» is unreliable); every use of the fallback is visible in the trace.
//! 5. If it does not fit — an honest «not understood» with a reason. An unknown content word — also «not understood».

use std::path::Path;

use anyhow::Result;
use en::annotate::Annotator;
use en::gram::{Rel, UPos};

use crate::err::{Error, R};
use crate::expr::{Expr, Func, ProbOp, Target, num, rat};
use crate::lex::{self, FnW, Kw, Lex, Op, UnitW};
use crate::rat::Rat;
use crate::units::{Dim, Mono, Scale, Unit};

pub const MODEL: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../en/models/ud-ewt-eslspok.bin");

/// Unit reference: multiplicative, a temperature scale, or «degrees» without a scale.
#[derive(Clone, Debug, PartialEq)]
pub enum URef {
    Mono(Mono),
    Scale(Scale),
    Degrees,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    /// Number (possibly with a unit); `frac` — a fraction («half», «a third») that requires «of».
    Qty { val: Rat, unit: Option<URef>, frac: bool },
    /// Bare unit — a conversion target or a period.
    Unit(URef),
    Word(Option<Lex>),
}

#[derive(Clone, Debug)]
pub struct Item {
    pub kind: Kind,
    pub text: String,
    /// Head in the tree (item index), None — root.
    pub head: Option<usize>,
    pub rel: Rel,
    pub upos: UPos,
    /// Case marker (preposition) and where it came from: the tree or linear order.
    pub case: Option<Kw>,
    pub case_tree: bool,
    /// Inside a condition clause.
    pub cond: bool,
    /// Temperature as a difference (a change of 10 °F, rose by 18 °F, 10 °F warmer).
    pub delta: bool,
}

impl Item {
    fn word(&self) -> Option<Lex> {
        match self.kind {
            Kind::Word(l) => l,
            _ => None,
        }
    }
    fn is_kw(&self, k: Kw) -> bool {
        self.word() == Some(Lex::Kw(k))
    }
    fn is_qty(&self) -> bool {
        matches!(self.kind, Kind::Qty { frac: false, .. })
    }
    fn unit(&self) -> Option<&URef> {
        match &self.kind {
            Kind::Qty { unit, .. } => unit.as_ref(),
            _ => None,
        }
    }
    fn is_pct_qty(&self) -> bool {
        matches!(self.unit(), Some(URef::Mono(m)) if m.is_pct())
    }
    fn is_temp_qty(&self) -> bool {
        matches!(self.unit(), Some(URef::Scale(_) | URef::Degrees))
    }
    fn time_qty(&self) -> bool {
        matches!(self.unit(), Some(URef::Mono(m)) if m.dim() == Dim::T)
    }
}

/// Parsed sentence: items and the tree over them.
#[derive(Clone, Debug)]
pub struct Sent {
    pub items: Vec<Item>,
}

/// Result of understanding: the expression and a trace — why exactly this.
#[derive(Debug)]
pub struct Parsed {
    pub expr: Expr,
    pub trace: Vec<String>,
}

pub struct Understander {
    ann: Annotator,
}

fn low(s: &str) -> String {
    s.to_lowercase()
}

fn lx(s: &str) -> Option<Lex> {
    lex::lookup(&low(s))
}

/// Units that are not units when bare (no number before them): «m», «s», «second» (ordinal), «min»…
const AMBIG: &[&str] = &["m", "s", "h", "g", "min", "sec", "second", "mb", "pa", "c", "f", "k"];

fn read_unit(t: &[String], i: usize, after_num: bool) -> Option<(URef, usize)> {
    let w = t.get(i)?;
    let lw = low(w);
    let prev = if i > 0 { low(&t[i - 1]) } else { String::new() };
    match lx(w)? {
        Lex::Degree => {
            if let Some(Lex::Scale(s)) = t.get(i + 1).and_then(|x| lx(x)) {
                return Some((URef::Scale(s), i + 2));
            }
            if w == "°" && !after_num {
                return None;
            }
            Some((URef::Degrees, i + 1))
        }
        Lex::Scale(s) => {
            if lw.len() == 1 && !after_num && !matches!(prev.as_str(), "to" | "into" | "in") {
                return None;
            }
            Some((URef::Scale(s), i + 1))
        }
        Lex::Pct if after_num => Some((URef::Mono(Mono::of(Unit::Pct)), i + 1)),
        Lex::Unit(u) => {
            if AMBIG.contains(&lw.as_str()) && !after_num && !matches!(prev.as_str(), "a" | "per" | "each" | "every") {
                return None;
            }
            // «inches of mercury»
            if u == UnitW::In && t.get(i + 1).map(|x| low(x)) == Some("of".into()) && t.get(i + 2).map(|x| low(x)) == Some("mercury".into()) {
                return Some((URef::Mono(Mono::of(Unit::InHg)), i + 3));
            }
            // «miles per hour»
            if t.get(i + 1).map(|x| low(x)) == Some("per".into()) {
                if let Some(Lex::Unit(u2)) = t.get(i + 2).and_then(|x| lx(x)) {
                    return Some((URef::Mono(u.mono().mul(&u2.mono().inv())), i + 3));
                }
            }
            Some((URef::Mono(u.mono()), i + 1))
        }
        _ => None,
    }
}

/// Number at position i: digits («1,000», «0.1», «1/3») or words («two thousand», «a dozen», «one third»).
fn read_number(t: &[String], i: usize) -> Option<(Rat, bool, usize)> {
    let w = t.get(i)?;
    if w.chars().next().is_some_and(|c| c.is_ascii_digit()) || (w.starts_with('.') && w.len() > 1) {
        let mut r = Rat::parse(w)?;
        let mut j = i + 1;
        if let Some(Lex::Mult(m)) = t.get(j).and_then(|x| lx(x)) {
            r = r.mul(Rat::int(m as i128)).ok()?;
            j += 1;
        }
        return Some((r, false, j));
    }
    let (mut total, mut cur) = (Rat::ZERO, Rat::ZERO);
    let mut j = i;
    let mut seen = false;
    let mut frac = false;
    // «a dozen», «a hundred», «a third», «a half»
    if matches!(lx(w), Some(Lex::Kw(Kw::A))) {
        match t.get(i + 1).and_then(|x| lx(x)) {
            Some(Lex::Mult(_) | Lex::Dozen | Lex::Frac(..)) => {
                cur = Rat::ONE;
                j = i + 1;
            }
            _ => return None,
        }
    }
    while j < t.len() {
        match lx(&t[j]) {
            Some(Lex::Num(n, d)) => {
                cur = cur.add(Rat::new(n as i128, d as i128).ok()?).ok()?;
                seen = true;
                j += 1;
                // «twenty-one»
                if t.get(j).map(String::as_str) == Some("-") && matches!(t.get(j + 1).and_then(|x| lx(x)), Some(Lex::Num(..))) {
                    j += 1;
                }
            }
            Some(Lex::Mult(m)) => {
                if cur.is_zero() {
                    cur = Rat::ONE;
                }
                cur = cur.mul(Rat::int(m as i128)).ok()?;
                if m >= 1000 {
                    total = total.add(cur).ok()?;
                    cur = Rat::ZERO;
                }
                seen = true;
                j += 1;
            }
            Some(Lex::Dozen) => {
                if cur.is_zero() {
                    cur = Rat::ONE;
                }
                cur = cur.mul(Rat::int(12)).ok()?;
                seen = true;
                j += 1;
            }
            Some(Lex::Frac(n, d)) => {
                if cur.is_zero() {
                    cur = Rat::ONE;
                }
                cur = cur.mul(Rat::new(n as i128, d as i128).ok()?).ok()?;
                seen = true;
                frac = true;
                j += 1;
                break;
            }
            _ => break,
        }
    }
    if !seen {
        return None;
    }
    Some((total.add(cur).ok()?, frac, j))
}

fn is_minus(s: &str) -> bool {
    s == "-" || s == "−"
}

fn chunk(t: &[String]) -> Vec<Item> {
    let mut out: Vec<Item> = Vec::new();
    let mut i = 0;
    let mk = |kind: Kind, text: String| Item { kind, text, head: None, rel: Rel::Dep, upos: UPos::X, case: None, case_tree: false, cond: false, delta: false };
    while i < t.len() {
        if low(&t[i]) == "at" && t.get(i + 1).map(|x| low(x)) == Some("least".into()) {
            out.push(mk(Kind::Word(Some(Lex::Kw(Kw::Least))), "at least".into()));
            i += 2;
            continue;
        }
        let operand_before = out.last().is_some_and(|x| matches!(x.kind, Kind::Qty { .. } | Kind::Unit(_)));
        let neg = is_minus(&t[i]) && !operand_before && t.get(i + 1).is_some_and(|x| x.chars().next().is_some_and(|c| c.is_ascii_digit()));
        let start = if neg { i + 1 } else { i };
        if let Some((val, frac, j)) = read_number(t, start) {
            let val = if neg { val.neg().unwrap_or(val) } else { val };
            let (unit, k) = if frac { (None, j) } else { read_unit(t, j, true).map(|(u, k)| (Some(u), k)).unwrap_or((None, j)) };
            out.push(mk(Kind::Qty { val, unit, frac }, t[i..k].join(" ")));
            i = k;
            continue;
        }
        // «a mile», «a day», «a week» — one unit
        if matches!(lx(&t[i]), Some(Lex::Kw(Kw::A))) {
            if let Some((u @ URef::Mono(_), k)) = read_unit(t, i + 1, false) {
                out.push(mk(Kind::Qty { val: Rat::ONE, unit: Some(u), frac: false }, t[i..k].join(" ")));
                i = k;
                continue;
            }
        }
        if let Some((u, k)) = read_unit(t, i, false) {
            out.push(mk(Kind::Unit(u), t[i..k].join(" ")));
            i = k;
            continue;
        }
        out.push(mk(Kind::Word(lx(&t[i])), t[i].clone()));
        i += 1;
    }
    out
}

/// Placeholders for the parser: a quantity — a number and a unit noun; a bare unit — a noun.
fn standins(items: &[Item]) -> (Vec<String>, Vec<usize>) {
    let mut forms = Vec::new();
    let mut owner = Vec::new();
    for (k, it) in items.iter().enumerate() {
        let mut push = |s: String| {
            forms.push(s);
            owner.push(k);
        };
        match &it.kind {
            Kind::Qty { val, unit, frac } => {
                if *frac {
                    push("half".into());
                    continue;
                }
                let n = val.abs().exact_decimal(6).unwrap_or_else(|| "7".into());
                push(n);
                match unit {
                    None => {}
                    Some(URef::Mono(m)) if m.is_pct() => push("%".into()),
                    Some(URef::Mono(_)) => push("units".into()),
                    Some(_) => push("degrees".into()),
                }
            }
            Kind::Unit(URef::Scale(_)) => push("Celsius".into()),
            Kind::Unit(_) => push("units".into()),
            Kind::Word(_) => push(it.text.clone()),
        }
    }
    (forms, owner)
}

const CASE_KWS: &[Kw] = &[Kw::Of, Kw::From, Kw::To, Kw::In, Kw::Between, Kw::By, Kw::Than, Kw::For, Kw::On, Kw::With, Kw::Per, Kw::At];

impl Understander {
    pub fn load(path: &Path) -> Result<Understander> {
        Ok(Understander { ann: Annotator::load(path)? })
    }

    /// Tokens with boundary fixes and sentence splitting.
    fn sentences(&self, text: &str) -> Vec<Vec<String>> {
        let mut toks: Vec<String> = Vec::new();
        for t in self.ann.tokenize(text) {
            let f = t.form.replace('−', "-");
            // the tokenizer glues «))» — split parentheses one by one
            if f.len() > 1 && (f.chars().all(|c| c == '(') || f.chars().all(|c| c == ')')) {
                toks.extend(f.chars().map(|c| c.to_string()));
                continue;
            }
            // «C.» «km.» at the end of a sentence — a unit and a period
            if f.len() > 1 && f.ends_with('.') && !f[..f.len() - 1].chars().all(|c| c.is_ascii_digit() || c == '.') {
                toks.push(f[..f.len() - 1].to_string());
                toks.push(".".into());
            } else {
                toks.push(f);
            }
        }
        let mut out = Vec::new();
        let mut cur = Vec::new();
        for t in toks {
            let end = matches!(t.as_str(), "." | "?" | "!");
            cur.push(t);
            if end {
                out.push(std::mem::take(&mut cur));
            }
        }
        if !cur.is_empty() {
            out.push(cur);
        }
        out
    }

    /// Sentence → items with a tree.
    pub fn parse_sentence(&self, toks: &[String]) -> Sent {
        let mut items = chunk(toks);
        let (forms, owner) = standins(&items);
        let words = self.ann.annotate(&forms);
        // head of an item: the token whose head lies outside the item
        for (k, it) in items.iter_mut().enumerate() {
            let toks_k: Vec<usize> = (0..forms.len()).filter(|&j| owner[j] == k).collect();
            let head_tok = toks_k.iter().copied().find(|&j| words[j].head == 0 || owner[words[j].head - 1] != k).unwrap_or(toks_k[0]);
            let w = &words[head_tok];
            it.head = if w.head == 0 { None } else { Some(owner[w.head - 1]) };
            it.rel = w.rel;
            it.upos = w.upos;
        }
        // case markers from the tree: a child with `case`
        for k in 0..items.len() {
            let found = (0..items.len()).find_map(|c| {
                if items[c].head == Some(k) && items[c].rel == Rel::Case {
                    match items[c].word() {
                        Some(Lex::Kw(kw)) if CASE_KWS.contains(&kw) => Some(kw),
                        _ => None,
                    }
                } else {
                    None
                }
            });
            if let Some(kw) = found {
                items[k].case = Some(kw);
                items[k].case_tree = true;
            }
        }
        // fallback: a preposition directly before (skipping an article)
        for k in 0..items.len() {
            if items[k].case.is_some() || !matches!(items[k].kind, Kind::Qty { .. } | Kind::Unit(_)) {
                continue;
            }
            let mut j = k;
            while j > 0 {
                j -= 1;
                match items[j].word() {
                    Some(Lex::Kw(Kw::Det | Kw::A)) => continue,
                    Some(Lex::Kw(kw)) if CASE_KWS.contains(&kw) => {
                        items[k].case = Some(kw);
                        break;
                    }
                    _ => break,
                }
            }
        }
        let mut s = Sent { items };
        s.mark_conditions();
        s.mark_deltas();
        s
    }

    /// Question → expression or an honest «not understood».
    pub fn understand(&self, text: &str) -> R<Parsed> {
        let sents: Vec<Sent> = self.sentences(text).iter().map(|t| self.parse_sentence(t)).collect();
        if sents.is_empty() {
            return Err(Error::NotUnderstood("empty question".into()));
        }
        Interp::new(sents).run()
    }
}

impl Sent {
    fn subtree(&self, root: usize) -> Vec<usize> {
        let mut out = vec![root];
        let mut i = 0;
        while i < out.len() {
            let r = out[i];
            for (c, it) in self.items.iter().enumerate() {
                if it.head == Some(r) && !out.contains(&c) {
                    out.push(c);
                }
            }
            i += 1;
        }
        out.sort();
        out
    }

    /// Condition clauses: advcl with mark if/when; a phrase with case with; linearly — «If/With …,» at the start.
    fn mark_conditions(&mut self) {
        let n = self.items.len();
        let mut cond = vec![false; n];
        for k in 0..n {
            let it = &self.items[k];
            let is_if_clause = matches!(it.rel, Rel::Advcl | Rel::Ccomp | Rel::Csubj) && self.subtree(k).iter().any(|&c| self.items[c].is_kw(Kw::If) && self.items[c].rel == Rel::Mark);
            let is_with = matches!(it.rel, Rel::Obl | Rel::Nmod | Rel::Advcl) && it.case == Some(Kw::With) && it.case_tree;
            if is_if_clause || is_with {
                for c in self.subtree(k) {
                    cond[c] = true;
                }
            }
        }
        // linearly: «If … ,» / «With … ,» at the start of the sentence
        if self.items.first().is_some_and(|x| x.is_kw(Kw::If) || x.is_kw(Kw::With)) {
            for k in 0..n {
                if self.items[k].text == "," {
                    break;
                }
                cond[k] = true;
            }
        }
        // «… if …» at the end: from if to the end
        if let Some(p) = self.items.iter().position(|x| x.is_kw(Kw::If)) {
            if p > 0 {
                for c in cond.iter_mut().skip(p) {
                    *c = true;
                }
            }
        }
        for (k, c) in cond.into_iter().enumerate() {
            self.items[k].cond = c;
        }
    }

    /// Temperature as a difference: «change/difference of X», «by X» with rise/fall, «X warmer/colder».
    fn mark_deltas(&mut self) {
        let n = self.items.len();
        let has_updown = self.items.iter().any(|x| x.is_kw(Kw::Up) || x.is_kw(Kw::Down));
        for k in 0..n {
            if !self.items[k].is_temp_qty() {
                continue;
            }
            let it = &self.items[k];
            let head_is = |f: &dyn Fn(&Item) -> bool| it.head.is_some_and(|h| f(&self.items[h]));
            let change_of = it.case == Some(Kw::Of) && (head_is(&|h: &Item| h.is_kw(Kw::Change) || h.word() == Some(Lex::Fn(FnW::Diff))) || (k >= 2 && (self.items[k - 2].is_kw(Kw::Change) || self.items[k - 2].word() == Some(Lex::Fn(FnW::Diff)))));
            let by_updown = it.case == Some(Kw::By) && has_updown;
            let comparative = self.items.get(k + 1).is_some_and(|x| x.is_kw(Kw::More) || x.is_kw(Kw::Less)) && !self.items.iter().any(|x| x.is_kw(Kw::Than));
            if change_of || by_updown || comparative {
                self.items[k].delta = true;
            }
        }
    }
}

/// Token stream for an arithmetic chain.
#[derive(Clone, Debug)]
enum Ct {
    /// Value; whether it is a percent; whether written as a fraction «a/b» (then «3/4 of 100» is a fraction of).
    Val(Expr, bool, bool),
    LParen,
    RParen,
    Op(Op),
    Of,
    Frac(Rat),
    Times(i64),
}

struct Interp {
    sents: Vec<Sent>,
    trace: Vec<String>,
    /// Scale for «degrees» without a scale — from another mention in the question.
    default_scale: Option<Scale>,
}

fn nu<T>(s: impl Into<String>) -> R<T> {
    Err(Error::NotUnderstood(s.into()))
}

impl Interp {
    fn new(sents: Vec<Sent>) -> Interp {
        let default_scale = sents.iter().flat_map(|s| s.items.iter()).find_map(|x| match &x.kind {
            // only from quantities: a conversion target («in Fahrenheit») is not the source scale, otherwise it is a guess
            Kind::Qty { unit: Some(URef::Scale(s)), .. } => Some(*s),
            _ => None,
        });
        Interp { sents, trace: Vec::new(), default_scale }
    }

    fn q(&self) -> &Sent {
        self.sents.last().unwrap()
    }

    /// Items of the question's main clause (indices).
    fn main(&self) -> Vec<usize> {
        (0..self.q().items.len()).filter(|&k| !self.q().items[k].cond).collect()
    }

    /// Conditions: preceding sentences and the question's condition clauses (references «sentence, index»).
    fn conds(&self) -> Vec<(usize, usize)> {
        let last = self.sents.len() - 1;
        let mut out = Vec::new();
        for (si, s) in self.sents.iter().enumerate() {
            for (k, it) in s.items.iter().enumerate() {
                if si < last || it.cond {
                    out.push((si, k));
                }
            }
        }
        out
    }

    fn item(&self, r: (usize, usize)) -> &Item {
        &self.sents[r.0].items[r.1]
    }

    fn qty_expr(&self, it: &Item) -> R<Expr> {
        let Kind::Qty { val, unit, frac } = &it.kind else { return nu(format!("«{}» — not a number", it.text)) };
        if *frac {
            return nu(format!("fraction «{}» without «of»", it.text));
        }
        let v = rat(*val);
        let scale = |s: Scale| if it.delta { Expr::Qty(v.clone().bx(), Mono::of(s.delta())) } else { Expr::Temp(v.clone().bx(), s) };
        Ok(match unit {
            None => v.clone(),
            Some(URef::Mono(m)) => Expr::Qty(v.clone().bx(), m.clone()),
            Some(URef::Scale(s)) => scale(*s),
            Some(URef::Degrees) => match self.default_scale {
                Some(s) => scale(s),
                None => return nu(format!("«{}» — degrees without a scale (Celsius or Fahrenheit?)", it.text)),
            },
        })
    }

    fn run(mut self) -> R<Parsed> {
        // gate for unknown words
        for s in &self.sents {
            for it in &s.items {
                if let Kind::Word(None) = it.kind {
                    if it.text.chars().any(|c| c.is_alphabetic()) {
                        return nu(format!("unknown word «{}» — not guessing", it.text));
                    }
                    if !matches!(it.text.as_str(), "(" | ")" | "\"" | "'" | ":" | ";") {
                        return nu(format!("unknown symbol «{}»", it.text));
                    }
                }
            }
        }
        let any_num = self.sents.iter().flat_map(|s| s.items.iter()).any(|x| matches!(x.kind, Kind::Qty { .. }));
        if !any_num {
            return nu("no numbers in the question");
        }
        let tree_cases = self.sents.iter().flat_map(|s| s.items.iter()).filter(|x| x.case.is_some() && x.case_tree).count();
        let lin_cases = self.sents.iter().flat_map(|s| s.items.iter()).filter(|x| x.case.is_some() && !x.case_tree).count();
        self.trace.push(format!("case markers: {tree_cases} from the tree (case), {lin_cases} linear"));
        if self.q().items.iter().any(|x| x.cond) || self.sents.len() > 1 {
            self.trace.push("condition: if/with clause (advcl+mark or case with from the tree) or a preceding sentence".into());
        }
        let e = self.frame()?;
        Ok(Parsed { expr: e, trace: self.trace })
    }

    fn frame(&mut self) -> R<Expr> {
        let main = self.main();
        let items = self.q().items.clone();
        let mi = |k: usize| &items[k];
        let has = |f: &dyn Fn(&Item) -> bool| main.iter().any(|&k| f(mi(k)));
        let has_kw = |kw: Kw| has(&|x: &Item| x.is_kw(kw));
        let bare_pct = has(&|x: &Item| x.word() == Some(Lex::Pct));

        // 1. probability
        if has_kw(Kw::Chance) {
            return self.prob_frame();
        }
        // 2. percent change: «percent change from A to B»
        if bare_pct && (has_kw(Kw::Change) || has_kw(Kw::Up) || has_kw(Kw::Down)) {
            let from = self.find_case(&main, Kw::From);
            let to = self.find_case(&main, Kw::To);
            if let (Some(a), Some(b)) = (from, to) {
                self.trace.push("frame: percent change (from/to — case markers)".into());
                let e = Expr::call(Func::PctChange, vec![self.qty_expr(mi(a))?, self.qty_expr(mi(b))?]);
                return Ok(if has_kw(Kw::Down) && !has_kw(Kw::Change) { Expr::Neg(e.bx()) } else { e });
            }
            return nu("percent change — no «from … to …» found");
        }
        // 3. percent share: «what percent of W is P»
        if bare_pct && has_kw(Kw::Wh) {
            let qs: Vec<usize> = main.iter().copied().filter(|&k| mi(k).is_qty()).collect();
            let whole = self.find_case(&main, Kw::Of);
            if let (Some(w), 2) = (whole, qs.len()) {
                let part = qs.iter().copied().find(|&k| k != w).unwrap();
                self.trace.push("frame: percent share («of» — the whole)".into());
                return Ok(Expr::call(Func::PctRatio, vec![self.qty_expr(mi(part))?, self.qty_expr(mi(w))?]));
            }
            return nu("«what percent» — no part and whole found");
        }
        // 4. increase/decrease by p%
        if (has_kw(Kw::Up) || has_kw(Kw::Down)) && has(&|x: &Item| x.is_pct_qty() && x.case == Some(Kw::By)) {
            let p = main.iter().copied().find(|&k| mi(k).is_pct_qty() && mi(k).case == Some(Kw::By)).unwrap();
            let x = main.iter().copied().find(|&k| mi(k).is_qty() && k != p);
            let Some(x) = x else { return nu("by how many percent — of what?") };
            self.trace.push("frame: change by a percent (by p%)".into());
            let pe = self.qty_expr(mi(p))?;
            let f = if has_kw(Kw::Up) { Expr::add(num(1), pe) } else { Expr::sub(num(1), pe) };
            return Ok(Expr::mul(self.qty_expr(mi(x))?, f));
        }
        // 5. comparison: «how much warmer is A than B»
        if (has_kw(Kw::More) || has_kw(Kw::Less)) && has_kw(Kw::Than) {
            let b = self.find_case(&main, Kw::Than);
            let a = main.iter().copied().find(|&k| mi(k).is_qty() && Some(k) != b);
            if let (Some(a), Some(b)) = (a, b) {
                self.trace.push("frame: comparison (than — case marker)".into());
                let (ea, eb) = (self.qty_expr(mi(a))?, self.qty_expr(mi(b))?);
                return Ok(if has_kw(Kw::More) { Expr::sub(ea, eb) } else { Expr::sub(eb, ea) });
            }
            return nu("comparison — two quantities not found");
        }
        // conversion target
        let target = self.find_target(&main);
        // 6. state: «it is 72 °F and it gets 10 °F warmer, what is the temperature?»
        if main.iter().all(|&k| !mi(k).is_qty()) && target.is_none() && has_kw(Kw::Topic) {
            let cs = self.conds();
            let point = cs.iter().copied().find(|&r| self.item(r).is_temp_qty() && !self.item(r).delta);
            let delta = cs.iter().copied().find(|&r| self.item(r).is_temp_qty() && self.item(r).delta);
            if let (Some(p), Some(d)) = (point, delta) {
                self.trace.push("frame: state change (temperature + difference from the condition)".into());
                let less = cs.iter().any(|&r| self.item(r).is_kw(Kw::Less) || self.item(r).is_kw(Kw::Down));
                let (ep, ed) = (self.qty_expr(self.item(p))?, self.qty_expr(self.item(d))?);
                return Ok(if less { Expr::sub(ep, ed) } else { Expr::add(ep, ed) });
            }
        }
        // 7. source: difference, aggregate, action or chain
        let src = self.source(&main, target.as_ref().map(|t| t.0))?;
        match target {
            Some((_, t)) => {
                // a source without a unit into a unit — not guessing the scale («Convert 72 to Celsius»)
                let to_pct = matches!(&t, Target::Mono(m) if m.is_pct());
                if !to_pct && matches!(src, Expr::Num(_) | Expr::Neg(_)) {
                    return nu(format!("«{src}» has no unit — convert from what into {t}?"));
                }
                self.trace.push(format!("frame: conversion to {t}"));
                Ok(Expr::Convert(src.bx(), t))
            }
            None => Ok(src),
        }
    }

    /// Quantity with case marker `kw` in the main clause.
    fn find_case(&self, main: &[usize], kw: Kw) -> Option<usize> {
        main.iter().copied().find(|&k| self.q().items[k].is_qty() && self.q().items[k].case == Some(kw))
    }

    /// Conversion target: a bare unit with in/to/into, after «convert», or «how many U».
    fn find_target(&self, main: &[usize]) -> Option<(usize, Target)> {
        let items = &self.q().items;
        for &k in main {
            let Kind::Unit(u) = &items[k].kind else { continue };
            let after_many = k > 0 && items[k - 1].is_kw(Kw::Many);
            let marked = matches!(items[k].case, Some(Kw::In | Kw::To));
            if after_many || marked {
                let t = match u {
                    URef::Mono(m) => Target::Mono(m.clone()),
                    URef::Scale(s) => Target::Scale(*s),
                    URef::Degrees => match self.default_scale {
                        Some(s) => Target::Scale(s),
                        None => continue,
                    },
                };
                return Some((k, t));
            }
        }
        None
    }

    /// Source: difference between, aggregate, Add/Subtract actions, root, chain; with no numbers in the main clause — from the condition.
    fn source(&mut self, main: &[usize], target: Option<usize>) -> R<Expr> {
        let items = self.q().items.clone();
        let main: Vec<usize> = main.iter().copied().filter(|&k| Some(k) != target).collect();
        let qs: Vec<usize> = main.iter().copied().filter(|&k| items[k].is_qty()).collect();
        let fnw = main.iter().copied().find_map(|k| match items[k].word() {
            Some(Lex::Fn(f)) => Some((k, f)),
            _ => None,
        });
        // difference between A and B
        if let Some((_, FnW::Diff)) = fnw {
            if qs.len() == 2 {
                self.trace.push("frame: difference between two quantities (|a − b|)".into());
                return Ok(Expr::call(Func::AbsDiff, vec![self.qty_expr(&items[qs[0]])?, self.qty_expr(&items[qs[1]])?]));
            }
        }
        if let Some((fk, f)) = fnw {
            if f != FnW::Diff {
                // arguments — quantities of the function's subtree (tree), otherwise all after it (linear)
                let sub = self.q().subtree(fk);
                let mut args: Vec<usize> = qs.iter().copied().filter(|k| sub.contains(k)).collect();
                let via_tree = !args.is_empty() && args.len() == qs.iter().filter(|&&k| k > fk).count();
                if !via_tree {
                    args = qs.iter().copied().filter(|&k| k > fk).collect();
                }
                self.trace.push(format!("frame: function {:?}; arguments {}", f, if via_tree { "from the subtree (tree)" } else { "linear after the word" }));
                let mut es: Vec<Expr> = args.iter().map(|&k| self.qty_expr(&items[k])).collect::<R<_>>()?;
                if es.is_empty() {
                    // «the average high if Monday was 70 °F, Tuesday 72 °F…» — data in the condition
                    let cq: Vec<Item> = self.conds().into_iter().map(|r| self.item(r).clone()).filter(|x| x.is_qty()).collect();
                    if cq.is_empty() {
                        return nu(format!("«{}» — of what exactly? no arguments", items[fk].text));
                    }
                    self.trace.push("function arguments — from the condition clause".into());
                    es = cq.iter().map(|x| self.qty_expr(x)).collect::<R<_>>()?;
                }
                return Ok(match f {
                    FnW::Mean => Expr::call(Func::Mean, es),
                    FnW::Median => Expr::call(Func::Median, es),
                    FnW::Max => Expr::call(Func::Max, es),
                    FnW::Min => Expr::call(Func::Min, es),
                    FnW::Sum => Expr::call(Func::Sum, es),
                    FnW::Range => Expr::call(Func::Range, es),
                    FnW::Product => Expr::Mul(es),
                    FnW::Root => {
                        if es.len() != 1 {
                            return nu("root — of a single number");
                        }
                        let cube = main.iter().any(|&k| items[k].is_kw(Kw::Cube));
                        if cube { Expr::pow(es[0].clone(), rat(Rat::new(1, 3).unwrap())) } else { Expr::call(Func::Sqrt, es) }
                    }
                    FnW::Diff => unreachable!(),
                });
            }
        }
        // Add A and B / Subtract A from B
        if main.iter().any(|&k| items[k].is_kw(Kw::Add)) && !qs.is_empty() {
            self.trace.push("frame: action «add» over the listed quantities".into());
            let es: Vec<Expr> = qs.iter().map(|&k| self.qty_expr(&items[k])).collect::<R<_>>()?;
            if es.len() < 2 {
                return nu("«add» — needs at least two addends");
            }
            return Ok(Expr::Add(es));
        }
        if main.iter().any(|&k| items[k].is_kw(Kw::Subtract)) {
            let from = qs.iter().copied().find(|&k| items[k].case == Some(Kw::From));
            let what = qs.iter().copied().find(|&k| items[k].case != Some(Kw::From));
            if let (Some(b), Some(a)) = (from, what) {
                self.trace.push("frame: action «subtract A from B» (from — case marker)".into());
                return Ok(Expr::sub(self.qty_expr(&items[b])?, self.qty_expr(&items[a])?));
            }
            return nu("«subtract» — no «A from B» found");
        }
        // no numbers in the main clause — a reference to the condition («what is that in Fahrenheit?»)
        let chain_items: Vec<usize> = if qs.is_empty() && main.iter().all(|&k| !matches!(items[k].kind, Kind::Qty { .. })) {
            let cs = self.conds();
            let cq: Vec<(usize, usize)> = cs.iter().copied().filter(|&r| matches!(self.item(r).kind, Kind::Qty { .. })).collect();
            if cq.len() == 1 {
                self.trace.push("source: quantity from the condition («that/it»)".into());
                return self.qty_expr(&self.item(cq[0]).clone());
            }
            if cq.is_empty() {
                return nu("no numbers found in the question");
            }
            // condition with a chain («if you add 17 and 25»)
            let last = self.sents.len() - 1;
            if cs.iter().all(|r| r.0 == last) {
                self.trace.push("source: condition clause (no numbers in the main clause)".into());
                let cond_main: Vec<usize> = cs.iter().map(|r| r.1).collect();
                return self.chain(&cond_main, None);
            }
            return nu("several numbers in the condition — unclear which one is asked");
        } else {
            main
        };
        self.chain(&chain_items, target)
    }

    /// Arithmetic chain in linear order: an operator grammar with precedences.
    fn chain(&mut self, ks: &[usize], _target: Option<usize>) -> R<Expr> {
        let items = self.q().items.clone();
        let mut ct: Vec<Ct> = Vec::new();
        let mut i = 0;
        let add_verb = ks.iter().any(|&k| items[k].is_kw(Kw::Add));
        while i < ks.len() {
            let it = &items[ks[i]];
            match (&it.kind, it.word()) {
                (Kind::Qty { frac: true, val, .. }, _) => ct.push(Ct::Frac(*val)),
                (Kind::Qty { .. }, _) => ct.push(Ct::Val(self.qty_expr(it)?, it.is_pct_qty(), it.unit().is_none() && it.text.contains('/'))),
                (Kind::Word(None), _) if it.text == "(" => ct.push(Ct::LParen),
                (Kind::Word(None), _) if it.text == ")" => ct.push(Ct::RParen),
                (_, Some(Lex::Op(o))) => ct.push(Ct::Op(o)),
                (_, Some(Lex::Times(n))) => ct.push(Ct::Times(n)),
                (_, Some(Lex::Kw(Kw::Of))) => ct.push(Ct::Of),
                (_, Some(Lex::Kw(Kw::To))) => {
                    // «to the power of»
                    let next = ks[i + 1..].iter().map(|&k| &items[k]).find(|x| !x.is_kw(Kw::Det));
                    if next.is_some_and(|x| x.word() == Some(Lex::Op(Op::Pow))) {
                        while i < ks.len() && items[ks[i]].word() != Some(Lex::Op(Op::Pow)) {
                            i += 1;
                        }
                        ct.push(Ct::Op(Op::Pow));
                        if ks.get(i + 1).is_some_and(|&k| items[k].is_kw(Kw::Of)) {
                            i += 1;
                        }
                    }
                }
                (_, Some(Lex::Kw(Kw::And))) if add_verb => ct.push(Ct::Op(Op::Add)),
                (_, Some(Lex::Kw(Kw::Change) | Lex::Fn(FnW::Diff))) => {
                    // «plus a change of 9 °F», «a difference of 9 °F» — the quantity is already marked as a difference
                    if ks.get(i + 1).is_some_and(|&k| items[k].is_kw(Kw::Of)) {
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        // «multiply 6 by 7» / «divide 84 by 4»: operator in front
        if let Some(Ct::Op(o @ (Op::Mul | Op::Div))) = ct.first().cloned() {
            let vals: Vec<Expr> = ct.iter().filter_map(|c| if let Ct::Val(e, _, _) = c { Some(e.clone()) } else { None }).collect();
            if vals.len() == 2 {
                self.trace.push("frame: action «multiply/divide A by B»".into());
                return Ok(if o == Op::Mul { Expr::mul(vals[0].clone(), vals[1].clone()) } else { Expr::div(vals[0].clone(), vals[1].clone()) });
            }
        }
        if ct.is_empty() {
            return nu("nothing to compute found");
        }
        self.trace.push("arithmetic: operator chain in linear order (precedences + − · / ^)".into());
        let mut p = Pratt { t: ct, i: 0 };
        let e = p.expr(0)?;
        if p.i < p.t.len() {
            return nu(format!("extra tokens after the expression: {:?}", p.t[p.i]));
        }
        Ok(e)
    }

    fn prob_frame(&mut self) -> R<Expr> {
        let main = self.main();
        let items = self.q().items.clone();
        let op = if main.iter().any(|&k| items[k].is_kw(Kw::Least) || items[k].is_kw(Kw::Once)) {
            ProbOp::AtLeastOnce
        } else if main.iter().any(|&k| items[k].is_kw(Kw::No)) {
            ProbOp::NoneOf
        } else if main.iter().any(|&k| items[k].is_kw(Kw::Every) || items[k].is_kw(Kw::All)) {
            ProbOp::AllOf
        } else {
            return nu("probability of what: at least once, never, or every time?");
        };
        // all items: conditions + main clause
        let mut all: Vec<(usize, usize)> = self.conds();
        let last = self.sents.len() - 1;
        all.extend(main.iter().map(|&k| (last, k)));
        all.sort();
        // «at least one» — not a quantity
        let sents = self.sents.clone();
        let skip = |r: (usize, usize)| r.1 > 0 && sents[r.0].items[r.1 - 1].is_kw(Kw::Least);
        let item = |r: (usize, usize)| &sents[r.0].items[r.1];
        let pcts: Vec<(usize, usize)> = all.iter().copied().filter(|&r| item(r).is_pct_qty() && !skip(r)).collect();
        if pcts.is_empty() {
            return nu("no daily probability (percent) found");
        }
        // probabilities for named days: «20% on Monday and 40% on Tuesday»
        let named = |r: (usize, usize)| {
            let s = &sents[r.0];
            s.items.get(r.1 + 1).is_some_and(|x| x.is_kw(Kw::On)) && s.items.get(r.1 + 2).is_some_and(|x| x.is_kw(Kw::DayName))
                || s.items.iter().any(|x| x.is_kw(Kw::DayName) && x.head == Some(r.1))
        };
        let mut groups = Vec::new();
        if pcts.iter().all(|&r| named(r)) && pcts.len() >= 2 {
            self.trace.push(format!("frame: probability {op:?}; separate days (on Monday…) — once each"));
            for &r in &pcts {
                groups.push((self.qty_expr(item(r))?, num(1)));
            }
            return Ok(Expr::Prob(op, groups));
        }
        if pcts.len() > 1 {
            return nu("several probabilities — unclear which one is daily");
        }
        // duration: time with in/for/over (case marker), not «at least one»
        let dur = all.iter().copied().find(|&r| item(r).time_qty() && matches!(item(r).case, Some(Kw::In | Kw::For)) && !skip(r));
        let Some(dr) = dur else { return nu("number of days not found") };
        let dq = self.qty_expr(self.item(dr))?;
        // number of days — exact conversion of the duration to days
        let n = match &dq {
            Expr::Qty(v, m) => {
                let Expr::Num(v) = **v else { return nu("duration") };
                v.mul(m.scale()?)?.div(Unit::Day.scale())?
            }
            _ => return nu("duration without a time unit"),
        };
        if !n.is_int() || n.is_neg() {
            return nu(format!("duration {} — not a whole number of days", self.item(dr).text));
        }
        self.trace.push(format!("frame: probability {op:?}; p — from the condition/phrase, duration «{}» (case marker {:?}{}) = {} days", self.item(dr).text, self.item(dr).case.unwrap(), if self.item(dr).case_tree { ", tree" } else { ", linear" }, n));
        groups.push((self.qty_expr(self.item(pcts[0]))?, rat(n)));
        Ok(Expr::Prob(op, groups))
    }
}

/// Operator grammar for chains.
struct Pratt {
    t: Vec<Ct>,
    i: usize,
}

impl Pratt {
    fn peek(&self) -> Option<&Ct> {
        self.t.get(self.i)
    }

    fn expr(&mut self, min: u8) -> R<Expr> {
        let mut lhs = self.unary()?;
        loop {
            let Some(Ct::Op(o)) = self.peek().cloned() else { break };
            let (prec, right) = match o {
                Op::Add | Op::Sub => (1, false),
                Op::Mul | Op::Div => (2, false),
                Op::Pow => (3, true),
                Op::Sq | Op::Cube => (4, false),
            };
            if prec < min {
                break;
            }
            self.i += 1;
            if matches!(o, Op::Sq | Op::Cube) {
                lhs = Expr::pow(lhs, num(if o == Op::Sq { 2 } else { 3 }));
                continue;
            }
            if self.peek().is_none() {
                return nu("operator without a second operand");
            }
            let rhs = self.expr(if right { prec } else { prec + 1 })?;
            // language convention: «100 minus 20 percent» = 100 · (1 − 20%) = 80, not 100 − 0.2
            let pct = |e: &Expr| matches!(e, Expr::Qty(_, m) if m.is_pct());
            if matches!(o, Op::Add | Op::Sub) && pct(&rhs) && !pct(&lhs) {
                let f = if o == Op::Add { Expr::add(num(1), rhs) } else { Expr::sub(num(1), rhs) };
                lhs = Expr::mul(lhs, f);
                continue;
            }
            lhs = match o {
                Op::Add => Expr::add(lhs, rhs),
                Op::Sub => Expr::sub(lhs, rhs),
                Op::Mul => Expr::mul(lhs, rhs),
                Op::Div => Expr::div(lhs, rhs),
                Op::Pow => Expr::pow(lhs, rhs),
                _ => unreachable!(),
            };
        }
        Ok(lhs)
    }

    fn unary(&mut self) -> R<Expr> {
        match self.peek().cloned() {
            Some(Ct::Op(Op::Sub)) => {
                self.i += 1;
                Ok(Expr::Neg(self.unary()?.bx()))
            }
            Some(Ct::Times(n)) => {
                self.i += 1;
                Ok(Expr::mul(num(n as i128), self.unary()?))
            }
            Some(Ct::Frac(r)) => {
                self.i += 1;
                if !matches!(self.peek(), Some(Ct::Of)) {
                    return nu("fraction without «of»");
                }
                self.i += 1;
                Ok(Expr::mul(rat(r), self.unary()?))
            }
            Some(Ct::LParen) => {
                self.i += 1;
                let e = self.expr(0)?;
                if !matches!(self.peek(), Some(Ct::RParen)) {
                    return nu("unclosed parenthesis");
                }
                self.i += 1;
                Ok(e)
            }
            Some(Ct::RParen) => nu("extra closing parenthesis"),
            Some(Ct::Val(e, pct, slash)) => {
                self.i += 1;
                if matches!(self.peek(), Some(Ct::Of)) && slash && !pct {
                    // «3/4 of 100» — a fraction of
                    self.i += 1;
                    let x = self.unary()?;
                    return Ok(Expr::mul(e, x));
                }
                if matches!(self.peek(), Some(Ct::Of)) {
                    if !pct {
                        return nu("«of» between numbers without a percent or fraction — unclear what it means");
                    }
                    self.i += 1;
                    if self.peek().is_none() {
                        return nu("percent — of what?");
                    }
                    let x = self.unary()?;
                    return Ok(Expr::call(Func::PctOf, vec![e, x]));
                }
                Ok(e)
            }
            Some(Ct::Of) => nu("«of» without a number before it"),
            Some(Ct::Op(_)) => nu("operator without a first operand"),
            None => nu("missing a number"),
        }
    }
}
