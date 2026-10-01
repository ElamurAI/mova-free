//! English grammar expert system: the rule engine ("an expert system
//! that will test and help the annotator… learn by itself from materials, i.e. research papers and
//! dictionaries, not from examples").
//!
//! Rules are written by the small agents (Opus) in seeds `en/seeds/**.md`, in ```` ```rule ```` blocks; the language is
//! `ai/en/src/expert.rs` (v1: negation of any condition, `unless:`, `or`, `@node`, `lemma.suffix`,
//! `next`/`prev`). The engine is deterministic:
//! 1. for each sentence it enumerates bindings of the `match` nodes to distinct words;
//! 2. for each binding it checks `require` (all clauses; within a clause at least one `or` alternative);
//! 3. an unmet requirement is a violation unless some `unless` clause holds.
//!
//! Parsing of nodes and conditions, reference checking and binding enumeration (`bindings`) are reused by
//! the dialect converter `en::convert`: the same language plus `from`/`to`/`set`.

use std::fmt;
use std::path::Path;

use crate::conllu::{Sentence, Token};
use crate::gram::{Feat, Rel, Tag, UPos};

/// A condition on a node.
#[derive(Debug, Clone)]
pub(crate) enum Cond {
    Upos(Vec<UPos>),
    Xpos(Vec<Tag>),
    Lemma(Vec<String>),
    Form(Vec<String>),
    /// the feature equals one of the values (values are full "Key=Value" pairs)
    FeatEq(Vec<Feat>),
    /// the feature is present (bits of all its values)
    FeatHas(u128),
    /// `feats.X=@s`: node s has feature X, and this word has the same value (bits of X's values, node)
    FeatSame(u128, String),
    Rel(Vec<Rel>),
    RelBase(Vec<Rel>),
    Head(Ref),
    Before(String),
    After(String),
    /// `next=v`: the word right after v
    Next(String),
    /// `prev=v`: the word right before v
    Prev(String),
    Suffix(Vec<String>),
    Prefix(Vec<String>),
    LemmaSuffix(Vec<String>),
    LemmaPrefix(Vec<String>),
    /// `lemma=@form`: the lemma equals the form (case-insensitive)
    LemmaIsForm,
    /// negation: `k!=v`, `rel!~v`, `!feats.X`
    Not(Box<Cond>),
}

impl Cond {
    /// The `match` node the condition refers to.
    pub(crate) fn target(&self) -> Option<&str> {
        match self {
            Cond::Head(Ref::Var(n)) | Cond::Before(n) | Cond::After(n) | Cond::Next(n) | Cond::Prev(n) | Cond::FeatSame(_, n) => Some(n.as_str()),
            Cond::Not(c) => c.target(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Ref {
    Root,
    Var(String),
}

#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub(crate) name: String,
    pub(crate) conds: Vec<Cond>,
}

#[derive(Debug, Clone)]
pub(crate) enum Req {
    Holds(Node),
    Not(Node),
    Exists(Node),
    None(Node),
}

impl Req {
    fn node(&self) -> &Node {
        match self {
            Req::Holds(n) | Req::Not(n) | Req::Exists(n) | Req::None(n) => n,
        }
    }
}

/// `require` or `unless` clauses: a clause is a list of alternatives joined by `or`.
pub(crate) type Clauses = Vec<Vec<Req>>;

/// A rule from a seed.
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub what: String,
    pub severity: String,
    pub source: String,
    pub file: String,
    matches: Vec<Node>,
    reqs: Clauses,
    unless: Clauses,
}

impl Rule {
    /// Names of the `match` nodes, in the order of the numbers `check` returns.
    pub fn names(&self) -> Vec<&str> {
        self.matches.iter().map(|n| n.name.as_str()).collect()
    }
}

/// A rule parse error, so the small agent knows what to fix.
#[derive(Debug)]
pub struct ParseError {
    pub file: String,
    pub rule: String,
    pub msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}]: {}", self.file, self.rule, self.msg)
    }
}

fn list<T>(v: &str, parse: impl Fn(&str) -> Option<T>) -> Result<Vec<T>, String> {
    v.split('|').map(|x| parse(x.trim()).ok_or_else(|| format!("unknown value '{x}'"))).collect()
}

/// Bits of all values of a feature (`Number` → Sing|Plur|Ptan). Features outside `Feat` (Aspect) are dropped
/// by `conllu::read`, so for `feats.X` / `!feats.X` such a feature is simply always absent (as in v0): 0.
fn key_bits(key: &str) -> u128 {
    Feat::ALL.iter().filter(|f| f.key() == key).fold(0u128, |a, f| a | 1 << f.idx())
}

/// Same for `feats.X=@s`: here an unknown feature is an error, not silence.
pub(crate) fn key_mask(key: &str) -> Result<u128, String> {
    let m = key_bits(key);
    if m == 0 { Err(format!("unknown feature '{key}'")) } else { Ok(m) }
}

/// Splits by a separator outside `[…]` brackets.
pub(crate) fn split_top<'a>(s: &'a str, sep: &str) -> Vec<&'a str> {
    let (b, sb) = (s.as_bytes(), sep.as_bytes());
    let (mut out, mut depth, mut start, mut i) = (Vec::new(), 0i32, 0usize, 0usize);
    while i < b.len() {
        match b[i] {
            b'[' => depth += 1,
            b']' => depth -= 1,
            _ if depth == 0 && b[i..].starts_with(sb) => {
                out.push(&s[start..i]);
                i += sb.len();
                start = i;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    out.push(&s[start..]);
    out
}

/// A `key=value` condition without negation.
fn eq(k: &str, v: &str) -> Result<Cond, String> {
    if let Some(n) = v.strip_prefix('@') {
        return match k.strip_prefix("feats.") {
            Some(key) if !n.is_empty() => Ok(Cond::FeatSame(key_mask(key)?, n.to_string())),
            None if k == "lemma" && n == "form" => Ok(Cond::LemmaIsForm),
            _ => Err(format!("'@' only in feats.X=@node and lemma=@form: '{k}={v}'")),
        };
    }
    let low = |x: &str| Some(x.to_lowercase());
    Ok(match k {
        "upos" => Cond::Upos(list(v, UPos::parse)?),
        "xpos" => Cond::Xpos(list(v, Tag::parse)?),
        "lemma" => Cond::Lemma(list(v, low)?),
        "form" => Cond::Form(list(v, low)?),
        "rel" => Cond::Rel(list(v, Rel::parse)?),
        "head" => Cond::Head(if v == "0" { Ref::Root } else { Ref::Var(v.to_string()) }),
        "before" => Cond::Before(v.to_string()),
        "after" => Cond::After(v.to_string()),
        "next" => Cond::Next(v.to_string()),
        "prev" => Cond::Prev(v.to_string()),
        "suffix" => Cond::Suffix(list(v, low)?),
        "prefix" => Cond::Prefix(list(v, low)?),
        "lemma.suffix" => Cond::LemmaSuffix(list(v, low)?),
        "lemma.prefix" => Cond::LemmaPrefix(list(v, low)?),
        _ if k.starts_with("feats.") => {
            let key = &k["feats.".len()..];
            Cond::FeatEq(list(v, |x| Feat::parse(&format!("{key}={x}")))?)
        }
        _ => return Err(format!("unknown condition '{k}'")),
    })
}

fn cond(c: &str) -> Result<Cond, String> {
    let c = c.trim();
    if let Some(k) = c.strip_prefix('!') {
        return Ok(Cond::Not(Box::new(Cond::FeatHas(key_bits(k.trim().trim_start_matches("feats."))))));
    }
    // key: letters, digits, '.', '_'; then operator `=`, `!=`, `~`, `!~`; the value may be anything
    let end = c.find(|x: char| !(x.is_alphanumeric() || x == '.' || x == '_')).unwrap_or(c.len());
    let (k, rest) = (&c[..end], c[end..].trim_start());
    if rest.is_empty() {
        return match k.strip_prefix("feats.") {
            Some(key) => Ok(Cond::FeatHas(key_bits(key))),
            None => Err(format!("unclear condition '{c}'")),
        };
    }
    let (neg, tilde, v) = if let Some(v) = rest.strip_prefix("!=") {
        (true, false, v)
    } else if let Some(v) = rest.strip_prefix("!~") {
        (true, true, v)
    } else if let Some(v) = rest.strip_prefix('=') {
        (false, false, v)
    } else if let Some(v) = rest.strip_prefix('~') {
        (false, true, v)
    } else {
        return Err(format!("unclear condition '{c}'"));
    };
    let v = v.trim();
    let pos = if tilde {
        if k != "rel" {
            return Err(format!("'~' only for rel: '{c}'"));
        }
        Cond::RelBase(list(v, Rel::parse)?)
    } else {
        eq(k, v)?
    };
    Ok(if neg { Cond::Not(Box::new(pos)) } else { pos })
}

/// `name[condition, condition]`
pub(crate) fn node(s: &str) -> Result<Node, String> {
    let s = s.trim();
    let (name, rest) = s.split_once('[').ok_or_else(|| format!("node without '[': '{s}'"))?;
    let body = rest.strip_suffix(']').ok_or_else(|| format!("node without ']': '{s}'"))?;
    let conds = if body.trim().is_empty() { Vec::new() } else { body.split(',').map(cond).collect::<Result<_, _>>()? };
    Ok(Node { name: name.trim().to_string(), conds })
}

/// `v[…]`, `not v[…]`, `exists c[…]`, `none c[…]`
fn req(p: &str) -> Result<Req, String> {
    let p = p.trim();
    Ok(if let Some(x) = p.strip_prefix("not ") {
        Req::Not(node(x)?)
    } else if let Some(x) = p.strip_prefix("exists ") {
        Req::Exists(node(x)?)
    } else if let Some(x) = p.strip_prefix("none ") {
        Req::None(node(x)?)
    } else {
        Req::Holds(node(p)?)
    })
}

/// A `require`/`unless` line: clauses separated by `;`, alternatives within a clause by ` or `.
pub(crate) fn clauses(line: &str) -> Result<Clauses, String> {
    split_top(line, ";").into_iter().map(|part| split_top(part, " or ").into_iter().map(req).collect::<Result<Vec<Req>, String>>()).collect()
}

/// Lines of ```` ```<tag> ```` blocks in md text, in file order (an unclosed block at the end is not taken).
/// `exact`: the tag ends with a space or end of line (```` ```convert-todo ```` is not a `convert` block);
/// otherwise any line starting with ```` ```<tag> ```` (this is how seeds have been read since v0).
pub(crate) fn blocks(text: &str, tag: &str, exact: bool) -> Vec<Vec<String>> {
    let open = format!("```{tag}");
    let (mut out, mut block, mut inside) = (Vec::new(), Vec::new(), false);
    for line in text.lines() {
        let t = line.trim();
        if !inside && t.starts_with(&open) && (!exact || t[open.len()..].chars().next().is_none_or(char::is_whitespace)) {
            inside = true;
            block.clear();
            continue;
        }
        if inside && t.starts_with("```") {
            inside = false;
            out.push(std::mem::take(&mut block));
            continue;
        }
        if inside {
            block.push(line.to_string());
        }
    }
    out
}

/// Rules from a seed text (```` ```rule ```` blocks).
pub fn parse_file(text: &str, file: &str) -> (Vec<Rule>, Vec<ParseError>) {
    let (mut rules, mut errs) = (Vec::new(), Vec::new());
    for block in blocks(text, "rule", false) {
        match parse_rule(&block, file) {
            Ok(r) => rules.push(r),
            Err(e) => errs.push(e),
        }
    }
    (rules, errs)
}

const FIELDS: &[&str] = &["rule", "what", "match", "require", "unless", "severity", "source"];

/// Block fields "key: value". One field per line; an unknown field (not in `allowed`) or a repeat goes to the
/// second list (a typo in `unless` is not silent).
pub(crate) fn fields(lines: &[String], allowed: &[&str]) -> (std::collections::HashMap<String, String>, Vec<String>) {
    let mut f = std::collections::HashMap::new();
    let mut bad = Vec::new();
    for l in lines {
        if let Some((k, v)) = l.split_once(':') {
            let k = k.trim().to_string();
            if !allowed.contains(&k.as_str()) {
                bad.push(format!("unknown field '{k}'"));
            } else if f.insert(k.clone(), v.trim().to_string()).is_some() {
                bad.push(format!("field '{k}' twice"));
            }
        }
    }
    (f, bad)
}

/// `match` nodes are distinct; requirements `v[…]`/`not v[…]` and references (head=, before=, after=, next=, prev=, @)
/// point only to `match` nodes: otherwise the condition would be silently false.
pub(crate) fn validate(matches: &[Node], reqs: &Clauses, unless: &Clauses) -> Result<(), String> {
    let names: Vec<&str> = matches.iter().map(|n| n.name.as_str()).collect();
    for (k, n) in names.iter().enumerate() {
        if names[..k].contains(n) {
            return Err(format!("node '{n}' twice in match"));
        }
    }
    for r in reqs.iter().chain(unless).flatten() {
        if let Req::Holds(n) | Req::Not(n) = r {
            if !names.contains(&n.name.as_str()) {
                return Err(format!("requirement on unknown node '{}'", n.name));
            }
        }
    }
    for n in matches.iter().chain(reqs.iter().chain(unless).flatten().map(Req::node)) {
        for c in &n.conds {
            if let Some(t) = c.target() {
                if !names.contains(&t) {
                    return Err(format!("reference to unknown node '{t}' in '{}'", n.name));
                }
            }
        }
    }
    Ok(())
}

fn parse_rule(lines: &[String], file: &str) -> Result<Rule, ParseError> {
    let (f, bad) = fields(lines, FIELDS);
    let id = f.get("rule").cloned().unwrap_or_default();
    let err = |msg: String| ParseError { file: file.to_string(), rule: id.clone(), msg };
    if id.is_empty() {
        return Err(err("missing field rule".into()));
    }
    if let Some(b) = bad.into_iter().next() {
        return Err(err(b));
    }
    let matches: Vec<Node> = split_top(f.get("match").ok_or_else(|| err("missing field match".into()))?, ";").into_iter().map(node).collect::<Result<_, _>>().map_err(err)?;
    let reqs = clauses(f.get("require").ok_or_else(|| err("missing field require".into()))?).map_err(err)?;
    let unless = match f.get("unless") {
        Some(u) => clauses(u).map_err(err)?,
        None => Vec::new(),
    };
    validate(&matches, &reqs, &unless).map_err(err)?;
    Ok(Rule {
        id: id.clone(),
        what: f.get("what").cloned().unwrap_or_default(),
        severity: f.get("severity").cloned().unwrap_or_else(|| "warn".into()),
        source: f.get("source").cloned().unwrap_or_default(),
        file: file.to_string(),
        matches,
        reqs,
        unless,
    })
}

/// All rules from seeds under `dir`.
pub fn load(dir: &Path) -> (Vec<Rule>, Vec<ParseError>) {
    let (mut rules, mut errs) = (Vec::new(), Vec::new());
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "md") {
                let text = std::fs::read_to_string(&p).unwrap_or_default();
                let (r, er) = parse_file(&text, &p.display().to_string());
                rules.extend(r);
                errs.extend(er);
            }
        }
    }
    rules.sort_by(|a, b| a.id.cmp(&b.id));
    (rules, errs)
}

/// A word for the engine.
pub struct W<'a> {
    t: &'a Token,
}

/// Sentence words for the engine (word number in CoNLL-U = index + 1).
pub(crate) fn words(s: &Sentence) -> Vec<W<'_>> {
    s.tokens.iter().map(|t| W { t }).collect()
}

fn holds(c: &Cond, i: usize, ws: &[W], bind: &[(String, usize)]) -> bool {
    let t = ws[i].t;
    let var = |name: &str| bind.iter().find(|(n, _)| n == name).map(|x| x.1);
    match c {
        Cond::Upos(v) => t.upos.is_some_and(|u| v.contains(&u)),
        Cond::Xpos(v) => t.tag.is_some_and(|x| v.contains(&x)),
        Cond::Lemma(v) => v.contains(&t.lemma.to_lowercase()),
        Cond::Form(v) => v.contains(&t.form.to_lowercase()),
        Cond::FeatEq(v) => v.iter().any(|f| t.feats.has(*f)),
        Cond::FeatHas(m) => t.feats.0 & m != 0,
        Cond::FeatSame(m, n) => var(n).is_some_and(|j| {
            let s = ws[j].t.feats.0 & m;
            s != 0 && s == t.feats.0 & m
        }),
        Cond::Rel(v) => v.contains(&t.rel),
        Cond::RelBase(v) => v.iter().any(|r| r.base() == t.rel.base()),
        Cond::Head(Ref::Root) => t.head == 0,
        Cond::Head(Ref::Var(n)) => var(n).is_some_and(|j| t.head == j + 1),
        Cond::Before(n) => var(n).is_some_and(|j| i < j),
        Cond::After(n) => var(n).is_some_and(|j| i > j),
        Cond::Next(n) => var(n).is_some_and(|j| i == j + 1),
        Cond::Prev(n) => var(n).is_some_and(|j| i + 1 == j),
        Cond::Suffix(v) => v.iter().any(|s| t.form.to_lowercase().ends_with(s.as_str())),
        Cond::Prefix(v) => v.iter().any(|s| t.form.to_lowercase().starts_with(s.as_str())),
        Cond::LemmaSuffix(v) => v.iter().any(|s| t.lemma.to_lowercase().ends_with(s.as_str())),
        Cond::LemmaPrefix(v) => v.iter().any(|s| t.lemma.to_lowercase().starts_with(s.as_str())),
        Cond::LemmaIsForm => t.lemma.to_lowercase() == t.form.to_lowercase(),
        Cond::Not(c) => !holds(c, i, ws, bind),
    }
}

fn node_holds(n: &Node, i: usize, ws: &[W], bind: &[(String, usize)]) -> bool {
    n.conds.iter().all(|c| holds(c, i, ws, bind))
}

fn req_holds(r: &Req, ws: &[W], bind: &[(String, usize)]) -> bool {
    let at = |n: &Node| bind.iter().find(|(x, _)| *x == n.name).map(|x| x.1);
    let free = |n: &Node| (0..ws.len()).any(|i| !bind.iter().any(|(_, j)| *j == i) && node_holds(n, i, ws, bind));
    match r {
        Req::Holds(n) => at(n).is_some_and(|i| node_holds(n, i, ws, bind)),
        Req::Not(n) => at(n).is_some_and(|i| !node_holds(n, i, ws, bind)),
        Req::Exists(n) => free(n),
        Req::None(n) => !free(n),
    }
}

/// At least one alternative holds (a clause holds if any of its alternatives does; `require` needs all clauses, `unless` any).
pub(crate) fn clause_holds(c: &[Req], ws: &[W], bind: &[(String, usize)]) -> bool {
    c.iter().any(|r| req_holds(r, ws, bind))
}

/// Enumerates bindings of the `match` nodes to distinct words of the sentence (in word order). For each one where
/// the conditions of all nodes hold, calls `f` with pairs (node name, 0-based word index) in `match` order.
pub(crate) fn bindings<F: FnMut(&[(String, usize)])>(matches: &[Node], ws: &[W], f: &mut F) {
    fn go<F: FnMut(&[(String, usize)])>(matches: &[Node], k: usize, ws: &[W], bind: &mut Vec<(String, usize)>, f: &mut F) {
        if k == matches.len() {
            // match conditions referring to later nodes are checked now, when all are bound
            if matches.iter().enumerate().all(|(m, n)| node_holds(n, bind[m].1, ws, bind)) {
                f(bind);
            }
            return;
        }
        let n = &matches[k];
        for i in 0..ws.len() {
            if bind.iter().any(|(_, j)| *j == i) {
                continue;
            }
            // quick filter: conditions without node references
            let local = n.conds.iter().all(|c| c.target().is_some() || holds(c, i, ws, bind));
            if !local {
                continue;
            }
            bind.push((n.name.clone(), i));
            go(matches, k + 1, ws, bind, f);
            bind.pop();
        }
    }
    let mut bind: Vec<(String, usize)> = Vec::new();
    go(matches, 0, ws, &mut bind, f);
}

/// Rule violations in a sentence: for each, the word numbers of the binding (1-based).
/// The first number is how many `match` bindings were found (`unless` does not reduce it).
pub fn check(rule: &Rule, s: &Sentence) -> (usize, Vec<Vec<usize>>) {
    let ws = words(s);
    let mut fired = 0usize;
    let mut out = Vec::new();
    bindings(&rule.matches, &ws, &mut |bind: &[(String, usize)]| {
        fired += 1;
        let ok = rule.reqs.iter().all(|c| clause_holds(c, &ws, bind));
        if !ok && !rule.unless.iter().any(|c| clause_holds(c, &ws, bind)) {
            out.push(bind.iter().map(|(_, i)| i + 1).collect());
        }
    });
    (fired, out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gram::Feats;

    /// A sentence from lines "form lemma UPOS XPOS FEATS head relation".
    fn sent(rows: &str) -> Sentence {
        let tokens = rows
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(|l| {
                let c: Vec<&str> = l.split_whitespace().collect();
                Token { form: c[0].into(), lemma: c[1].into(), upos: UPos::parse(c[2]), tag: Tag::parse(c[3]), feats: Feats::parse(c[4]).0, head: c[5].parse().unwrap(), rel: Rel::parse(c[6]).unwrap(), space_after: true }
            })
            .collect();
        Sentence { tokens, ..Default::default() }
    }

    fn rule(body: &str) -> Rule {
        let (mut r, e) = parse_file(&format!("```rule\nrule: t\n{body}\nsource: test\n```\n"), "t.md");
        assert!(e.is_empty(), "{e:?}");
        r.remove(0)
    }

    fn parse_err(body: &str) -> String {
        let (r, e) = parse_file(&format!("```rule\nrule: t\n{body}\nsource: test\n```\n"), "t.md");
        assert!(r.is_empty(), "should have failed to parse: {body}");
        e[0].msg.clone()
    }

    /// Numbers of words satisfying the conditions (rule "any such word is a violation").
    fn sel(conds: &str, s: &Sentence) -> Vec<usize> {
        check(&rule(&format!("match: x[{conds}]\nrequire: not x[]")), s).1.into_iter().map(|b| b[0]).collect()
    }

    /// Pairs (s, x): x satisfies the conditions referring to s.
    fn pairs(s_conds: &str, x_conds: &str, s: &Sentence) -> Vec<Vec<usize>> {
        check(&rule(&format!("match: s[{s_conds}]; x[{x_conds}]\nrequire: not x[]")), s).1
    }

    /// "A lot of people say it ."
    fn lot() -> Sentence {
        sent("A a DET DT Definite=Ind|PronType=Art 2 det
              lot lot NOUN NN Number=Sing 5 nsubj
              of of ADP IN _ 4 case
              people people NOUN NNS Number=Plur 2 nmod
              say say VERB VBP Mood=Ind|Tense=Pres|VerbForm=Fin 0 root
              it it PRON PRP Case=Acc|Gender=Neut|Number=Sing|Person=3|PronType=Prs 5 obj
              . . PUNCT . _ 5 punct")
    }

    #[test]
    fn rules_parse_and_fire() {
        let md = "# VBZ\n```rule\nrule: t.vbz\nwhat: VBZ\nmatch: v[xpos=VBZ]; s[rel~nsubj, head=v]\nrequire: not s[feats.Number=Plur]\nseverity: error\nsource: test\n```\n";
        let (rules, errs) = parse_file(md, "t.md");
        assert!(errs.is_empty(), "{errs:?}");
        let tok = |form: &str, upos: UPos, tag: Tag, feats: &str, head: usize, rel: Rel| Token { form: form.into(), lemma: form.to_lowercase(), tag: Some(tag), upos: Some(upos), feats: Feats::parse(feats).0, head, rel, space_after: true };
        let good = Sentence { tokens: vec![tok("Dog", UPos::NOUN, Tag::NN, "Number=Sing", 2, Rel::Nsubj), tok("runs", UPos::VERB, Tag::VBZ, "", 0, Rel::Root)], ..Default::default() };
        let bad = Sentence { tokens: vec![tok("Dogs", UPos::NOUN, Tag::NNS, "Number=Plur", 2, Rel::Nsubj), tok("runs", UPos::VERB, Tag::VBZ, "", 0, Rel::Root)], ..Default::default() };
        assert_eq!(check(&rules[0], &good), (1, vec![]));
        // negative control: plural with VBZ is a violation
        assert_eq!(check(&rules[0], &bad), (1, vec![vec![2, 1]]));
        // a broken rule fails to parse rather than staying silent
        let (_, errs) = parse_file("```rule\nrule: t.bad\nmatch: v[xpos=VBQ]\nrequire: v[]\n```\n", "t.md");
        assert_eq!(errs.len(), 1);
    }

    #[test]
    fn v0_negations_unchanged() {
        let s = lot();
        assert_eq!(sel("feats.Number", &s), vec![2, 4, 6]);
        assert_eq!(sel("!feats.Number", &s), vec![1, 3, 5, 7]);
        // `feats.X!=V` is also true when the feature is absent
        assert_eq!(sel("feats.Number!=Sing", &s), vec![1, 3, 4, 5, 7]);
        assert_eq!(sel("feats.Number=Sing", &s), vec![2, 6]);
    }

    #[test]
    fn v1_negation_of_any_condition() {
        let s = lot();
        // a positive condition and its negation split the sentence in two: the negation is neither empty nor everything
        for (pos, neg) in [
            ("upos=NOUN|PUNCT", "upos!=NOUN|PUNCT"),
            ("xpos=NN|NNS", "xpos!=NN|NNS"),
            ("lemma=lot|number|couple", "lemma!=lot|number|couple"),
            ("form=a|it", "form!=a|it"),
            ("rel=nsubj|obj", "rel!=nsubj|obj"),
            ("rel~det|nmod", "rel!~det|nmod"),
            ("head=0", "head!=0"),
            ("suffix=t", "suffix!=t"),
        ] {
            let (a, b) = (sel(pos, &s), sel(neg, &s));
            assert!(!a.is_empty() && !b.is_empty(), "{pos} / {neg}");
            let mut all: Vec<usize> = a.iter().chain(&b).copied().collect();
            all.sort();
            assert_eq!(all, (1..=7).collect::<Vec<_>>(), "{pos} / {neg}");
        }
        assert_eq!(sel("upos!=NOUN|PUNCT", &s), vec![1, 3, 5, 6]);
        // the quantity noun is excluded from the list, an ordinary one is not
        assert_eq!(sel("upos=NOUN, lemma!=lot|number|couple", &s), vec![4]);
        assert_eq!(sel("upos=NOUN, lemma!=lot|people", &s), Vec::<usize>::new());
        // `rel!~` works by base: nmod:poss is also nmod
        let p = sent("John John PROPN NNP Number=Sing 2 nmod:poss
                      dog dog NOUN NN Number=Sing 0 root");
        assert_eq!(sel("rel!~nmod", &p), vec![2]);
        assert_eq!(sel("rel!=nmod", &p), vec![1, 2]);
    }

    /// "John and Mary say" / "John say" / "They say"
    fn subj(kind: &str) -> Sentence {
        match kind {
            "conj" => sent("John John PROPN NNP Number=Sing 4 nsubj
                            and and CCONJ CC _ 3 cc
                            Mary Mary PROPN NNP Number=Sing 1 conj
                            say say VERB VBP Mood=Ind|Tense=Pres|VerbForm=Fin 0 root"),
            "sing" => sent("John John PROPN NNP Number=Sing 2 nsubj
                            say say VERB VBP Mood=Ind|Tense=Pres|VerbForm=Fin 0 root"),
            _ => sent("They they PRON PRP Case=Nom|Number=Plur|Person=3|PronType=Prs 2 nsubj
                       say say VERB VBP Mood=Ind|Tense=Pres|VerbForm=Fin 0 root"),
        }
    }

    #[test]
    fn v1_or_in_require() {
        let r = rule("match: v[xpos=VBP]; s[rel~nsubj, head=v]\nrequire: s[feats.Number=Plur] or exists c[rel=conj, head=s]");
        assert!(check(&r, &subj("conj")).1.is_empty());
        assert!(check(&r, &subj("plur")).1.is_empty());
        // negative control: no alternative holds, so it is a violation
        assert_eq!(check(&r, &subj("sing")), (1, vec![vec![2, 1]]));
        // clauses separated by ';' still all must hold: the second requirement breaks the coordinated subject too
        let r = rule("match: v[xpos=VBP]; s[rel~nsubj, head=v]\nrequire: s[feats.Number=Plur] or exists c[rel=conj, head=s]; none d[rel=cc]");
        assert_eq!(check(&r, &subj("conj")).1.len(), 1);
        assert!(check(&r, &subj("plur")).1.is_empty());
    }

    #[test]
    fn v1_unless() {
        let r = rule("match: v[xpos=VBP]; s[rel~nsubj, head=v, feats.Number=Sing]\nrequire: not v[]\nunless: s[lemma=lot|number|couple]; exists c[rel=conj, head=s]");
        assert_eq!(check(&r, &lot()), (1, vec![]));
        assert_eq!(check(&r, &subj("conj")), (1, vec![]));
        // negative control: no unless clause holds, so the violation remains
        assert_eq!(check(&r, &subj("sing")), (1, vec![vec![2, 1]]));
        // unless does not reduce "fired" and does not act when require is satisfied
        let r = rule("match: v[xpos=VBP]; s[rel~nsubj, head=v]\nrequire: v[]\nunless: s[]");
        assert_eq!(check(&r, &subj("sing")), (1, vec![]));
        // `not` and `none` in unless
        let r = rule("match: v[xpos=VBP]; s[rel~nsubj, head=v]\nrequire: s[feats.Number=Plur]\nunless: not s[upos=PROPN]");
        assert_eq!(check(&r, &subj("sing")).1.len(), 1);
        let r = rule("match: v[xpos=VBP]; s[rel~nsubj, head=v]\nrequire: s[feats.Number=Plur]\nunless: none c[rel=cc]");
        assert!(check(&r, &subj("sing")).1.is_empty());
        assert_eq!(check(&r, &subj("conj")).1.len(), 1);
    }

    /// "I hurt myself" / "I hurt yourself" / "I hurt it"
    fn refl(obj: &str) -> Sentence {
        let o = match obj {
            "myself" => "myself myself PRON PRP Case=Acc|Number=Sing|Person=1|PronType=Prs|Reflex=Yes",
            "yourself" => "yourself yourself PRON PRP Case=Acc|Number=Sing|Person=2|PronType=Prs|Reflex=Yes",
            _ => "it it PRON PRP Case=Acc|Gender=Neut|Number=Sing|Person=3|PronType=Prs",
        };
        sent(&format!(
            "I I PRON PRP Case=Nom|Number=Sing|Person=1|PronType=Prs 2 nsubj
             hurt hurt VERB VBD Mood=Ind|Tense=Past|VerbForm=Fin 0 root
             {o} 2 obj"
        ))
    }

    #[test]
    fn v1_feature_of_other_node() {
        let r = rule("match: v[]; s[rel~nsubj, head=v]; o[feats.Reflex=Yes, head=v]\nrequire: o[feats.Person=@s, feats.Number=@s]");
        assert!(check(&r, &refl("myself")).1.is_empty());
        // negative control: a different person is a violation
        assert_eq!(check(&r, &refl("yourself")).1, vec![vec![2, 1, 3]]);
        // `!=@s` is the negation
        let r = rule("match: v[]; s[rel~nsubj, head=v]; o[rel=obj, head=v]\nrequire: o[feats.Person!=@s]");
        assert!(check(&r, &refl("it")).1.is_empty());
        assert_eq!(check(&r, &refl("myself")).1.len(), 1);
        // s lacks the feature: `=@s` is false even when the word lacks it too; `!=@s` is true
        let s = lot();
        assert_eq!(pairs("upos=VERB", "feats.Number=@s", &s), Vec::<Vec<usize>>::new());
        assert_eq!(pairs("upos=VERB", "upos=ADP, feats.Number=@s", &s), Vec::<Vec<usize>>::new());
        assert_eq!(pairs("upos=VERB", "upos=ADP, feats.Number!=@s", &s), vec![vec![5, 3]]);
        // singular to singular: lot ↔ it; plural people is not
        assert_eq!(pairs("lemma=lot", "feats.Number=@s", &s), vec![vec![2, 6]]);
    }

    #[test]
    fn v1_lemma_is_form_and_lemma_affixes() {
        let s = sent("The the DET DT Definite=Def|PronType=Art 2 det
                      studies study NOUN NNS Number=Plur 5 nsubj
                      went go VERB VBD Mood=Ind|Tense=Past|VerbForm=Fin 0 root
                      unhappy unhappy ADJ JJ Degree=Pos 3 xcomp
                      happiness happiness NOUN NN Number=Sing 3 obj");
        // case-insensitive: The/the are equal
        assert_eq!(sel("lemma=@form", &s), vec![1, 4, 5]);
        assert_eq!(sel("lemma!=@form", &s), vec![2, 3]);
        assert_eq!(sel("lemma.suffix=y", &s), vec![2, 4]);
        assert_eq!(sel("suffix=y", &s), vec![4]);
        assert_eq!(sel("lemma.prefix=g|un", &s), vec![3, 4]);
        assert_eq!(sel("prefix=g|un", &s), vec![4]);
        assert_eq!(sel("lemma.suffix!=y|ness", &s), vec![1, 3]);
    }

    #[test]
    fn v1_adjacency() {
        let s = lot();
        // right after "A" is lot; before "people" is of
        assert_eq!(pairs("form=a", "next=s", &s), vec![vec![1, 2]]);
        assert_eq!(pairs("form=people", "prev=s", &s), vec![vec![4, 3]]);
        // negative control: before/after are wider, next/prev only the neighbour
        assert_eq!(pairs("form=a", "after=s", &s).len(), 6);
        assert_eq!(pairs("form=a", "next=s, upos=ADP", &s), Vec::<Vec<usize>>::new());
        assert_eq!(pairs("form=a", "next!=s, upos=NOUN", &s), vec![vec![1, 4]]);
        // a + vowel right after: the a/an rule
        let r = rule("match: a[lemma=a]; w[next=a]\nrequire: not w[prefix=a|e|i|o|u]");
        let bad = sent("a a DET DT _ 2 det
                        apple apple NOUN NN Number=Sing 0 root");
        assert_eq!(check(&r, &bad).1, vec![vec![1, 2]]);
        assert!(check(&r, &lot()).1.is_empty());
    }

    #[test]
    fn v1_bad_rules_fail() {
        assert!(parse_err("match: v[head=zz]\nrequire: v[]").contains("unknown node 'zz'"));
        assert!(parse_err("match: v[]\nrequire: exists c[next=q]").contains("unknown node 'q'"));
        assert!(parse_err("match: v[]\nrequire: v[feats.Number=@q]").contains("unknown node 'q'"));
        assert!(parse_err("match: v[]\nrequire: v[]\nunless: q[]").contains("unknown node 'q'"));
        assert!(parse_err("match: v[]\nrequire: v[lemma=@v]").contains("'@'"));
        assert!(parse_err("match: v[]\nrequire: v[upos=@v]").contains("'@'"));
        assert!(parse_err("match: v[]\nrequire: v[feats.Numbr=@v]").contains("unknown feature"));
        // a feature outside `Feat` (Aspect), as in v0: the reader drops it, so it is always absent
        assert_eq!(sel("!feats.Aspect, upos=VERB", &lot()), vec![5]);
        assert_eq!(sel("feats.Aspect", &lot()), Vec::<usize>::new());
        assert!(parse_err("match: v[xpos!~NN]\nrequire: v[]").contains("'~' only for rel"));
        assert!(parse_err("match: v[upos!=NOPE]\nrequire: v[]").contains("unknown value"));
        assert!(parse_err("match: v[]\nrequire: v[]\nunles: v[]").contains("unknown field"));
        assert!(parse_err("match: v[]\nrequire: v[]\nrequire: v[]").contains("twice"));
        assert!(parse_err("match: v[]; v[]\nrequire: v[]").contains("twice"));
    }

    #[test]
    fn seeds_all_parse() {
        let (rules, errs) = load(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/seeds")));
        assert!(errs.is_empty(), "{}", errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n"));
        assert!(rules.len() > 400);
        let mut ids: Vec<&str> = rules.iter().map(|r| r.id.as_str()).collect();
        ids.dedup();
        assert_eq!(ids.len(), rules.len(), "duplicate rule id");
    }
}
