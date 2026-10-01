//! Converters between UD dialects (`train/dialects-and-converters.md`, "Converter rule language"). Design note: "from the meta
//! back to the French tree, where it can be checked against the original".
//!
//! Rules are ```` ```convert ```` blocks in `convert.md` files under `treebanks/` and `dialects/`. The matching
//! language is the same as for seeds (`en::expert`, v1): `match`, `require`, `unless`; plus `from`, `to`
//! and `set` actions. Node and condition parsing and binding enumeration come from `en::expert`, no copies.
//!
//! Application is deterministic:
//! 1. rules of the pair `from → to` — in file order (paths alphabetically) and block order within a file;
//! 2. for a rule — all bindings on the current sentence state (all `require` clauses hold, no `unless`
//!    clause does); action values (`@form`, `@v`) come from that same state; then all actions at once;
//! 3. two actions that set different values of the same field of one word are an error (a FEATS feature
//!    and a MISC key are separate fields);
//! 4. after all rules — the tree gate: heads in range, exactly one root, `root` only on it, no cycles.
//!
//! Lines untouched by actions are carried over byte for byte (`conllu::Doc`): comments, multiword tokens,
//! empty nodes, DEPS, MISC, features outside `Feat`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::conllu::{Col, FullSentence};
use crate::expert::{self, Clauses, Node, ParseError};
use crate::gram::{Feat, Rel, Tag, UPos};

/// A `set` action on a node.
#[derive(Debug, Clone, PartialEq)]
enum Act {
    Rel(Rel),
    Upos(UPos),
    /// PTB tag or '_'
    Xpos(String),
    Lemma(String),
    /// `lemma=@form` — the form in lowercase
    LemmaForm,
    /// `feats+=X=V`
    FeatSet(Feat),
    /// `feats-=X`
    FeatDel(String),
    /// `feats+=X=@v` — feature X from node v (index in match); if v lacks it — nothing
    FeatCopy(String, usize),
    /// `head=v`
    Head(usize),
    /// `head=0`
    Root,
    /// `head=@v` — the head of node v
    HeadOf(usize),
    /// `misc+=K=V`
    MiscSet(String, String),
    /// `misc-=K`
    MiscDel(String),
}

/// A converter rule.
#[derive(Debug, Clone)]
pub struct ConvRule {
    pub id: String,
    pub what: String,
    pub from: String,
    pub to: String,
    pub source: String,
    pub file: String,
    matches: Vec<Node>,
    reqs: Clauses,
    unless: Clauses,
    /// (node index in match, action) — in written order
    sets: Vec<(usize, Act)>,
}

const FIELDS: &[&str] = &["rule", "what", "from", "to", "match", "require", "unless", "set", "source"];

/// Rules from md text (```` ```convert ```` blocks), in file order.
pub fn parse_file(text: &str, file: &str) -> (Vec<ConvRule>, Vec<ParseError>) {
    let (mut rules, mut errs) = (Vec::new(), Vec::new());
    for block in expert::blocks(text, "convert", true) {
        match parse_rule(&block, file) {
            Ok(r) => rules.push(r),
            Err(e) => errs.push(e),
        }
    }
    (rules, errs)
}

fn parse_rule(lines: &[String], file: &str) -> Result<ConvRule, ParseError> {
    let (f, bad) = expert::fields(lines, FIELDS);
    let id = f.get("rule").cloned().unwrap_or_default();
    let err = |msg: String| ParseError { file: file.to_string(), rule: id.clone(), msg };
    if id.is_empty() {
        return Err(err("missing field rule".into()));
    }
    if let Some(b) = bad.into_iter().next() {
        return Err(err(b));
    }
    let get = |k: &str| f.get(k).filter(|v| !v.is_empty()).ok_or_else(|| err(format!("missing field {k}")));
    let (from, to, source) = (get("from")?.clone(), get("to")?.clone(), get("source")?.clone());
    for d in [&from, &to] {
        if d.contains(char::is_whitespace) {
            return Err(err(format!("dialect name contains whitespace: '{d}'")));
        }
    }
    if from == to {
        return Err(err(format!("from and to are the same: '{from}'")));
    }
    let matches: Vec<Node> = expert::split_top(get("match")?, ";").into_iter().map(expert::node).collect::<Result<_, _>>().map_err(err)?;
    let opt = |k: &str| match f.get(k) {
        Some(v) => expert::clauses(v).map_err(err),
        None => Ok(Vec::new()),
    };
    let (reqs, unless) = (opt("require")?, opt("unless")?);
    expert::validate(&matches, &reqs, &unless).map_err(err)?;
    let names: Vec<&str> = matches.iter().map(|n| n.name.as_str()).collect();
    let sets = parse_set(get("set")?, &names).map_err(err)?;
    Ok(ConvRule { id: id.clone(), what: f.get("what").cloned().unwrap_or_default(), from, to, source, file: file.to_string(), matches, reqs, unless, sets })
}

/// Actions whose value may contain a comma (`feats+=PronType=Int,Rel`, `lemma=,`).
const COMMA_VALUES: &[&str] = &["feats+=", "lemma=", "misc+="];

/// `set: t[action, action]; u[action]` — nodes separated by `;`, actions in brackets separated by `,` or `;`
/// (`t[feats+=Case=Gen; feats+=Poss=Yes]`). A piece without '=' after a comma that follows an action from
/// `COMMA_VALUES` continues that action's value.
fn parse_set(line: &str, names: &[&str]) -> Result<Vec<(usize, Act)>, String> {
    let idx = |n: &str| names.iter().position(|x| *x == n.trim()).ok_or_else(|| format!("node '{}' not from match", n.trim()));
    let mut out = Vec::new();
    for part in expert::split_top(line, ";") {
        let part = part.trim();
        let (name, rest) = part.split_once('[').ok_or_else(|| format!("actions without '[': '{part}'"))?;
        let body = rest.strip_suffix(']').ok_or_else(|| format!("actions without ']': '{part}'"))?;
        let t = idx(name)?;
        let mut pieces: Vec<String> = Vec::new();
        for chunk in body.split(';') {
            for (k, p) in chunk.split(',').map(str::trim).enumerate() {
                let join = k > 0 && !p.contains('=') && pieces.last().is_some_and(|l| COMMA_VALUES.iter().any(|x| l.starts_with(x)));
                match pieces.last_mut() {
                    Some(last) if join => {
                        last.push(',');
                        last.push_str(p);
                    }
                    _ => pieces.push(p.to_string()),
                }
            }
        }
        for p in &pieces {
            let a = action(p, &idx)?;
            if a == Act::Head(t) {
                return Err(format!("'{p}': the word is its own head"));
            }
            out.push((t, a));
        }
    }
    Ok(out)
}

fn action(p: &str, idx: &impl Fn(&str) -> Result<usize, String>) -> Result<Act, String> {
    let item = |v: &str| v.is_empty() || v.contains(['|', '\t']);
    if let Some(v) = p.strip_prefix("feats+=") {
        let (k, val) = v.split_once('=').ok_or_else(|| format!("'{p}': expected feats+=Feature=Value"))?;
        if let Some(n) = val.strip_prefix('@') {
            expert::key_mask(k)?;
            return Ok(Act::FeatCopy(k.to_string(), idx(n)?));
        }
        return Feat::parse(v).map(Act::FeatSet).ok_or_else(|| format!("unknown feature '{v}'"));
    }
    if let Some(k) = p.strip_prefix("feats-=") {
        expert::key_mask(k)?;
        return Ok(Act::FeatDel(k.to_string()));
    }
    if let Some(v) = p.strip_prefix("misc+=") {
        return match v.split_once('=') {
            Some((k, val)) if !item(k) && !item(val) => Ok(Act::MiscSet(k.to_string(), val.to_string())),
            _ => Err(format!("'{p}': expected misc+=Key=Value")),
        };
    }
    if let Some(k) = p.strip_prefix("misc-=") {
        if item(k) || k.contains('=') {
            return Err(format!("'{p}': expected misc-=Key"));
        }
        return Ok(Act::MiscDel(k.to_string()));
    }
    let (k, v) = p.split_once('=').ok_or_else(|| format!("unknown action '{p}'"))?;
    let bad = || format!("unknown value '{v}' in '{p}'");
    Ok(match k {
        "rel" => Act::Rel(Rel::parse(v).ok_or_else(bad)?),
        "upos" => Act::Upos(UPos::parse(v).ok_or_else(bad)?),
        "xpos" if v == "_" || Tag::parse(v).is_some() => Act::Xpos(v.to_string()),
        "xpos" => return Err(bad()),
        "lemma" if v == "@form" => Act::LemmaForm,
        "lemma" if v.starts_with('@') => return Err(format!("'@' in lemma — only lemma=@form: '{p}'")),
        "lemma" if v.is_empty() || v.contains('\t') => return Err(bad()),
        "lemma" => Act::Lemma(v.to_string()),
        "head" if v == "0" => Act::Root,
        "head" => match v.strip_prefix('@') {
            Some(n) => Act::HeadOf(idx(n)?),
            None => Act::Head(idx(v)?),
        },
        _ => return Err(format!("unknown action '{k}'")),
    })
}

/// Rules from `convert.md` files under `dir` (or from the file `dir` itself): paths alphabetically, within a
/// file — block order. `target` and hidden directories are skipped. A repeated rule id is an error.
pub fn load(dir: &Path) -> (Vec<ConvRule>, Vec<ParseError>) {
    let (mut rules, mut errs) = (Vec::new(), Vec::new());
    let mut files: Vec<PathBuf> = Vec::new();
    if dir.is_file() {
        files.push(dir.to_path_buf());
    } else if dir.is_dir() {
        let mut stack = vec![dir.to_path_buf()];
        while let Some(d) = stack.pop() {
            let Ok(rd) = std::fs::read_dir(&d) else { continue };
            for e in rd.flatten() {
                let p = e.path();
                let name = e.file_name().to_string_lossy().into_owned();
                if p.is_dir() {
                    if !name.starts_with('.') && name != "target" {
                        stack.push(p);
                    }
                } else if name == "convert.md" {
                    files.push(p);
                }
            }
        }
        files.sort();
    } else {
        errs.push(ParseError { file: dir.display().to_string(), rule: String::new(), msg: "no such directory or file".into() });
    }
    for p in files {
        let file = p.display().to_string();
        match std::fs::read_to_string(&p) {
            Ok(text) => {
                let (r, e) = parse_file(&text, &file);
                rules.extend(r);
                errs.extend(e);
            }
            Err(e) => errs.push(ParseError { file, rule: String::new(), msg: e.to_string() }),
        }
    }
    errs.extend(dup_ids(&rules));
    (rules, errs)
}

/// Repeated rule ids (across all loaded files).
fn dup_ids(rules: &[ConvRule]) -> Vec<ParseError> {
    let mut seen: HashMap<&str, &str> = HashMap::new();
    rules
        .iter()
        .filter_map(|r| seen.insert(&r.id, &r.file).map(|f| ParseError { file: r.file.clone(), rule: r.id.clone(), msg: format!("rule id already exists in {f}") }))
        .collect()
}

/// Rules of the pair `from → to` in load order.
pub fn pair<'a>(rules: &'a [ConvRule], from: &str, to: &str) -> Vec<&'a ConvRule> {
    rules.iter().filter(|r| r.from == from && r.to == to).collect()
}

/// The word field an action sets: a column; a FEATS feature and a MISC key are separate fields.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Field {
    Col(Col),
    Feat(String),
    Misc(String),
}

impl Field {
    fn name(&self) -> String {
        match self {
            Field::Col(c) => c.name().to_string(),
            Field::Feat(k) => format!("FEATS.{k}"),
            Field::Misc(k) => format!("MISC.{k}"),
        }
    }
}

/// An edit: word (0-based index), field, value (`None` — remove the feature or MISC key).
type Edit = (usize, Field, Option<String>);

/// The key of a 'K=V' item (an item without '=' is its own key).
fn key_of(item: &str) -> &str {
    item.split_once('=').map_or(item, |x| x.0)
}

/// The value of a key in a 'K=V|K=V' column (FEATS, MISC).
fn value<'a>(col: &'a str, key: &str) -> Option<&'a str> {
    col.split('|').find(|it| key_of(it) == key).and_then(|it| it.split_once('=')).map(|x| x.1)
}

/// Set (`Some`) or remove (`None`) a key in a 'K=V|K=V' column; other items stay as they were. A new FEATS
/// key (`sorted`) goes in case-insensitive alphabetical order, as in CoNLL-U; a new MISC key goes at the end.
fn set_item(col: &str, key: &str, val: Option<&str>, sorted: bool) -> String {
    let mut items: Vec<String> = if col == "_" || col.is_empty() { Vec::new() } else { col.split('|').map(str::to_string).collect() };
    match val {
        None => items.retain(|it| key_of(it) != key),
        Some(v) => {
            let new = format!("{key}={v}");
            match items.iter().position(|it| key_of(it) == key) {
                Some(i) => {
                    items[i] = new;
                    let mut j = i + 1;
                    while j < items.len() {
                        if key_of(&items[j]) == key { items.remove(j); } else { j += 1; }
                    }
                }
                None if sorted => {
                    let lk = key.to_lowercase();
                    let pos = items.iter().position(|it| key_of(it).to_lowercase() > lk).unwrap_or(items.len());
                    items.insert(pos, new);
                }
                None => items.push(new),
            }
        }
    }
    if items.is_empty() { "_".to_string() } else { items.join("|") }
}

/// Edits of one rule: all bindings on the current sentence state; values come from that same state.
/// Two edits of the same field of one word with different values are an error.
fn edits(r: &ConvRule, s: &FullSentence) -> Result<Vec<Edit>, String> {
    let ws = expert::words(&s.sent);
    let mut binds: Vec<Vec<usize>> = Vec::new();
    expert::bindings(&r.matches, &ws, &mut |b: &[(String, usize)]| {
        if r.reqs.iter().all(|c| expert::clause_holds(c, &ws, b)) && !r.unless.iter().any(|c| expert::clause_holds(c, &ws, b)) {
            binds.push(b.iter().map(|x| x.1).collect());
        }
    });
    let mut out: Vec<Edit> = Vec::new();
    let mut seen: HashMap<(usize, Field), usize> = HashMap::new();
    for b in &binds {
        for (node, act) in &r.sets {
            let w = b[*node];
            let (field, val) = match act {
                Act::Rel(x) => (Field::Col(Col::Deprel), Some(x.name().to_string())),
                Act::Upos(x) => (Field::Col(Col::Upos), Some(x.name().to_string())),
                Act::Xpos(x) => (Field::Col(Col::Xpos), Some(x.clone())),
                Act::Lemma(x) => (Field::Col(Col::Lemma), Some(x.clone())),
                Act::LemmaForm => (Field::Col(Col::Lemma), Some(s.word(w).get(Col::Form).to_lowercase())),
                Act::FeatSet(f) => {
                    let (k, v) = f.name().split_once('=').expect("Feat is 'key=value'");
                    (Field::Feat(k.to_string()), Some(v.to_string()))
                }
                Act::FeatDel(k) => (Field::Feat(k.clone()), None),
                Act::FeatCopy(k, v) => match value(s.word(b[*v]).get(Col::Feats), k) {
                    Some(x) => (Field::Feat(k.clone()), Some(x.to_string())),
                    None => continue,
                },
                Act::Head(v) => (Field::Col(Col::Head), Some((b[*v] + 1).to_string())),
                Act::Root => (Field::Col(Col::Head), Some("0".to_string())),
                Act::HeadOf(v) => (Field::Col(Col::Head), Some(s.word(b[*v]).get(Col::Head).to_string())),
                Act::MiscSet(k, v) => (Field::Misc(k.clone()), Some(v.clone())),
                Act::MiscDel(k) => (Field::Misc(k.clone()), None),
            };
            match seen.get(&(w, field.clone())) {
                Some(&i) if out[i].2 != val => {
                    let show = |v: &Option<String>| v.clone().unwrap_or_else(|| "(remove)".into());
                    return Err(format!(
                        "conflict in rule {}: word {} '{}' field {} — '{}' vs '{}'",
                        r.id,
                        w + 1,
                        s.word(w).get(Col::Form),
                        field.name(),
                        show(&out[i].2),
                        show(&val)
                    ));
                }
                Some(_) => {}
                None => {
                    seen.insert((w, field.clone()), out.len());
                    out.push((w, field, val));
                }
            }
        }
    }
    Ok(out)
}

/// Apply rules (of one pair, in order) to a sentence, then the tree gate. An error carries the rule id.
/// For each rule — the words (0-based indices) where it actually changed at least one field.
pub fn apply(rules: &[&ConvRule], s: &mut FullSentence) -> Result<Vec<Vec<usize>>, String> {
    let mut changed = vec![Vec::new(); rules.len()];
    for (k, r) in rules.iter().enumerate() {
        let ed = edits(r, s)?;
        if ed.is_empty() {
            continue;
        }
        for (w, field, val) in ed {
            let row = s.word_mut(w);
            let before = row.clone();
            match field {
                Field::Col(c) => row.set(c, val.expect("a column is set, not removed")),
                Field::Feat(k) => {
                    let v = set_item(row.get(Col::Feats), &k, val.as_deref(), true);
                    row.set(Col::Feats, v);
                }
                Field::Misc(k) => {
                    let v = set_item(row.get(Col::Misc), &k, val.as_deref(), false);
                    row.set(Col::Misc, v);
                }
            }
            if *row != before && !changed[k].contains(&w) {
                changed[k].push(w);
            }
        }
        s.refresh().map_err(|e| format!("after rule {}: {e}", r.id))?;
    }
    tree_gate(s)?;
    Ok(changed)
}

/// Tree gate on words: the head is a number in 0..=n; exactly one root (HEAD 0), and `root` only on
/// it; no cycles.
pub fn tree_gate(s: &FullSentence) -> Result<(), String> {
    let n = s.words.len();
    let mut heads = Vec::with_capacity(n);
    for k in 0..n {
        let r = s.word(k);
        let h: usize = r.get(Col::Head).parse().map_err(|_| format!("tree gate: word {}: head '{}' is not a number", k + 1, r.get(Col::Head)))?;
        if h > n {
            return Err(format!("tree gate: word {}: head {h} outside the sentence ({n} words)", k + 1));
        }
        if (h == 0) != (r.get(Col::Deprel) == "root") {
            return Err(format!("tree gate: word {}: head {h}, relation '{}' — root only on the root", k + 1, r.get(Col::Deprel)));
        }
        heads.push(h);
    }
    let roots = heads.iter().filter(|&&h| h == 0).count();
    if n > 0 && roots != 1 {
        return Err(format!("tree gate: roots {roots}, expected one"));
    }
    for k in 0..n {
        let (mut x, mut steps) = (k + 1, 0);
        while x != 0 {
            x = heads[x - 1];
            steps += 1;
            if steps > n {
                return Err(format!("tree gate: cycle through word {}", k + 1));
            }
        }
    }
    Ok(())
}

/// Differences between two versions of a sentence: for each column — the word numbers (1-based) where it
/// differs; `other` — comments, multiword tokens, empty nodes or the row count differ.
#[derive(Default, Debug)]
pub struct Diff {
    pub cols: [Vec<usize>; 10],
    pub other: bool,
}

impl Diff {
    pub fn words(&self) -> usize {
        let mut v: Vec<usize> = self.cols.iter().flatten().copied().collect();
        v.sort_unstable();
        v.dedup();
        v.len()
    }
    pub fn is_empty(&self) -> bool {
        !self.other && self.cols.iter().all(Vec::is_empty)
    }
}

pub fn diff(a: &FullSentence, b: &FullSentence) -> Diff {
    let mut d = Diff { other: a.comments != b.comments || a.rows.len() != b.rows.len() || a.blank_after != b.blank_after, ..Default::default() };
    let mut word = 0usize;
    for (x, y) in a.rows.iter().zip(&b.rows) {
        if !x.is_word() || !y.is_word() {
            d.other |= x != y;
            continue;
        }
        word += 1;
        for c in Col::ALL {
            if x.get(c) != y.get(c) {
                d.cols[c as usize].push(word);
            }
        }
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conllu::Doc;

    /// A sentence from lines 'form lemma UPOS XPOS FEATS head relation [MISC]' (columns separated by spaces).
    fn doc(rows: &str) -> Doc {
        let mut t = String::from("# sent_id = t1\n");
        for (k, l) in rows.lines().map(str::trim).filter(|l| !l.is_empty()).enumerate() {
            let c: Vec<&str> = l.split_whitespace().collect();
            let misc = c.get(7).copied().unwrap_or("_");
            t.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t_\t{misc}\n", k + 1, c[0], c[1], c[2], c[3], c[4], c[5], c[6]));
        }
        t.push('\n');
        Doc::parse(&t).unwrap()
    }

    fn rules(body: &str) -> Vec<ConvRule> {
        let (r, e) = parse_file(body, "t.md");
        assert!(e.is_empty(), "{}", e.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n"));
        r
    }

    /// One rule from a body (from/to/source/rule are added).
    fn rule(body: &str) -> Vec<ConvRule> {
        rules(&format!("```convert\nrule: t\nfrom: a\nto: b\n{body}\nsource: test\n```\n"))
    }

    fn parse_err(body: &str) -> String {
        let (r, e) = parse_file(&format!("```convert\nrule: t\nfrom: a\nto: b\n{body}\nsource: test\n```\n"), "t.md");
        assert!(r.is_empty(), "should not have parsed: {body}");
        e[0].msg.clone()
    }

    /// Apply rules to the first sentence of a document; returns the sentence afterwards.
    fn run(rs: &[ConvRule], d: &Doc) -> Result<FullSentence, String> {
        let mut s = d.sents[0].clone();
        let refs: Vec<&ConvRule> = rs.iter().collect();
        apply(&refs, &mut s)?;
        Ok(s)
    }

    fn col(s: &FullSentence, w: usize, c: Col) -> String {
        s.word(w - 1).get(c).to_string()
    }

    /// "Last week I saw John ."
    fn week() -> Doc {
        doc("Last last ADJ JJ Degree=Pos 2 amod
             week week NOUN NN Number=Sing 4 obl:tmod
             I I PRON PRP Case=Nom|Number=Sing|Person=1|PronType=Prs 4 nsubj
             saw see VERB VBD Mood=Ind|Number=Sing|Person=1|Tense=Past|VerbForm=Fin 0 root
             John John PROPN NNP Number=Sing 4 obj SpaceAfter=No
             . . PUNCT . _ 4 punct")
    }

    #[test]
    fn set_rel_upos_xpos_lemma() {
        let d = week();
        let s = run(&rule("match: t[rel=obl:tmod]\nset: t[rel=obl:unmarked, upos=PROPN, xpos=NNP, lemma=Week]"), &d).unwrap();
        assert_eq!((col(&s, 2, Col::Deprel), col(&s, 2, Col::Upos), col(&s, 2, Col::Xpos), col(&s, 2, Col::Lemma)), ("obl:unmarked".into(), "PROPN".into(), "NNP".into(), "Week".into()));
        // the view is refreshed: the next rule sees the new state
        assert_eq!(s.sent.tokens[1].rel, Rel::OblUnmarked);
        // negative control: other words are unchanged, and a rule that does not match does nothing
        let dd = diff(&d.sents[0], &s);
        assert_eq!(dd.words(), 1);
        assert!(!dd.other);
        let s = run(&rule("match: t[rel=obl:npmod]\nset: t[rel=obl:unmarked]"), &d).unwrap();
        assert!(diff(&d.sents[0], &s).is_empty());
        // xpos=_ is allowed
        let s = run(&rule("match: t[form=john]\nset: t[xpos=_]"), &d).unwrap();
        assert_eq!(col(&s, 5, Col::Xpos), "_");
    }

    #[test]
    fn set_lemma_from_form() {
        let d = week();
        let s = run(&rule("match: t[upos=PROPN]\nset: t[lemma=@form]"), &d).unwrap();
        assert_eq!(col(&s, 5, Col::Lemma), "john");
        // negative control: form 'Last' — lemma 'last' is not changed to 'Last'
        let s = run(&rule("match: t[form=last]\nset: t[lemma=@form]"), &d).unwrap();
        assert!(diff(&d.sents[0], &s).is_empty());
    }

    #[test]
    fn set_feats_keep_unknown() {
        // Aspect and Zzz are outside `Feat`: they must pass through the rules
        let d = doc("saw see VERB VBD Aspect=Perf|Mood=Ind|Tense=Past|Zzz=Q 0 root");
        let s = run(&rule("match: t[upos=VERB]\nset: t[feats+=Person=3]"), &d).unwrap();
        assert_eq!(col(&s, 1, Col::Feats), "Aspect=Perf|Mood=Ind|Person=3|Tense=Past|Zzz=Q");
        assert!(s.sent.tokens[0].feats.has(Feat::Person3));
        // value replaced in place
        let s = run(&rule("match: t[upos=VERB]\nset: t[feats+=Tense=Pres]"), &d).unwrap();
        assert_eq!(col(&s, 1, Col::Feats), "Aspect=Perf|Mood=Ind|Tense=Pres|Zzz=Q");
        // remove — only the named one
        let s = run(&rule("match: t[upos=VERB]\nset: t[feats-=Mood, feats-=Tense]"), &d).unwrap();
        assert_eq!(col(&s, 1, Col::Feats), "Aspect=Perf|Zzz=Q");
        // remove the last one — '_'
        let d2 = doc("saw see VERB VBD Tense=Past 0 root");
        let s = run(&rule("match: t[upos=VERB]\nset: t[feats-=Tense]"), &d2).unwrap();
        assert_eq!(col(&s, 1, Col::Feats), "_");
        // add to empty
        let s = run(&rule("match: t[upos=VERB]\nset: t[feats+=Number=Sing]"), &doc("saw see VERB VBD _ 0 root")).unwrap();
        assert_eq!(col(&s, 1, Col::Feats), "Number=Sing");
        // comma in the value
        let s = run(&rule("match: t[upos=VERB]\nset: t[feats+=PronType=Int,Rel, upos=PRON]"), &d2).unwrap();
        assert_eq!(col(&s, 1, Col::Feats), "PronType=Int,Rel|Tense=Past");
        assert_eq!(col(&s, 1, Col::Upos), "PRON");
        // actions in brackets separated by ';', node twice — same as with commas
        for set in ["t[feats+=PronType=Int,Rel; upos=PRON]", "t[feats+=PronType=Int,Rel]; t[upos=PRON]"] {
            let s = run(&rule(&format!("match: t[upos=VERB]\nset: {set}")), &d2).unwrap();
            assert_eq!((col(&s, 1, Col::Feats), col(&s, 1, Col::Upos)), ("PronType=Int,Rel|Tense=Past".into(), "PRON".into()), "{set}");
        }
        // after ';' a piece without '=' is not a value continuation but an error
        assert!(parse_err("match: t[]\nset: t[feats+=PronType=Int; Rel]").contains("unknown action"));
    }

    #[test]
    fn set_feats_copy() {
        let d = week();
        // Person from subject I (Person=1) to the verb, which already has Person=1 — no change; to John — added
        let s = run(&rule("match: s[rel=nsubj]; o[rel=obj]\nset: o[feats+=Person=@s, feats+=Case=@s]"), &d).unwrap();
        assert_eq!(col(&s, 5, Col::Feats), "Case=Nom|Number=Sing|Person=1");
        // the source lacks the feature — nothing (the subject has no Degree)
        let s = run(&rule("match: s[rel=nsubj]; o[rel=amod]\nset: o[feats+=Person=@s, feats+=Degree=@s]"), &d).unwrap();
        assert_eq!(col(&s, 1, Col::Feats), "Degree=Pos|Person=1");
    }

    #[test]
    fn set_head() {
        let d = week();
        // head=v: John → onto week (the tree stays a tree)
        let s = run(&rule("match: t[form=john]; v[form=week]\nset: t[head=v]"), &d).unwrap();
        assert_eq!(col(&s, 5, Col::Head), "2");
        assert_eq!(s.sent.tokens[4].head, 2);
        // head=@v: Last → onto the head of week (saw)
        let s = run(&rule("match: t[form=last]; v[form=week]\nset: t[head=@v]"), &d).unwrap();
        assert_eq!(col(&s, 1, Col::Head), "4");
        // head=0 together with rel=root and demoting the old root
        let s = run(&rule("match: n[form=week]; v[rel=root]\nset: n[head=0, rel=root]; v[head=n, rel=parataxis]"), &d).unwrap();
        assert_eq!((col(&s, 2, Col::Head), col(&s, 4, Col::Head)), ("0".into(), "2".into()));
    }

    #[test]
    fn set_misc() {
        let d = week();
        let s = run(&rule("match: t[form=john]\nset: t[misc+=Entity=person]"), &d).unwrap();
        assert_eq!(col(&s, 5, Col::Misc), "SpaceAfter=No|Entity=person");
        let s = run(&rule("match: t[form=john]\nset: t[misc+=SpaceAfter=Yes]"), &d).unwrap();
        assert_eq!(col(&s, 5, Col::Misc), "SpaceAfter=Yes");
        let s = run(&rule("match: t[form=john]\nset: t[misc-=SpaceAfter]"), &d).unwrap();
        assert_eq!(col(&s, 5, Col::Misc), "_");
        assert!(s.sent.tokens[4].space_after, "the view sees that SpaceAfter=No was removed");
        // add, then remove — the same line
        let two = rules("```convert\nrule: a\nfrom: x\nto: y\nmatch: t[form=john]\nset: t[misc+=K=V]\nsource: t\n```\n```convert\nrule: b\nfrom: x\nto: y\nmatch: t[form=john]\nset: t[misc-=K]\nsource: t\n```\n");
        assert!(diff(&d.sents[0], &run(&two, &d).unwrap()).is_empty());
    }

    #[test]
    fn order_and_simultaneous_bindings() {
        // "the big dog": the word before a noun becomes a noun. Bindings are on the state before the rule, so only
        // big; step by step (after each binding) "the" would also become a noun
        let d = doc("the the DET DT _ 3 det
                     big big ADJ JJ _ 3 amod
                     dog dog NOUN NN _ 0 root");
        let r = rule("match: n[upos=NOUN]; t[prev=n]\nset: t[upos=NOUN]");
        let s = run(&r, &d).unwrap();
        assert_eq!((col(&s, 1, Col::Upos), col(&s, 2, Col::Upos)), ("DET".into(), "NOUN".into()));
        // the second rule sees the state after the first: now 'the' too
        let two = rules(&format!("{0}{0}", "```convert\nrule: X\nfrom: x\nto: y\nmatch: n[upos=NOUN]; t[prev=n]\nset: t[upos=NOUN]\nsource: t\n```\n").replacen("rule: X", "rule: one", 1).replacen("rule: X", "rule: two", 1));
        let s = run(&two, &d).unwrap();
        assert_eq!(col(&s, 1, Col::Upos), "NOUN");
        // what each rule changed: the second sets big=NOUN again — that is not a change
        let mut s = d.sents[0].clone();
        let refs: Vec<&ConvRule> = two.iter().collect();
        assert_eq!(apply(&refs, &mut s).unwrap(), vec![vec![1], vec![0]]);
    }

    #[test]
    fn require_and_unless_filter() {
        let d = week();
        let r = rule("match: t[upos=NOUN|PROPN]\nrequire: t[rel=obj] or t[rel~obl]\nunless: t[form=john]\nset: t[misc+=Hit=1]");
        let s = run(&r, &d).unwrap();
        assert_eq!(col(&s, 2, Col::Misc), "Hit=1");
        assert_eq!(col(&s, 5, Col::Misc), "SpaceAfter=No", "unless removes John");
    }

    #[test]
    fn conflict_is_error() {
        let d = week();
        // two nouns — one verb: do two bindings set different lemmas on saw? No — the same: not a conflict
        let same = rule("match: n[upos=NOUN|PROPN]; v[upos=VERB]\nset: v[lemma=x]");
        assert!(run(&same, &d).is_ok());
        // different values from different bindings — an error
        let e = run(&rule("match: n[upos=NOUN|PROPN]; v[upos=VERB]\nset: v[head=n]"), &d).unwrap_err();
        assert!(e.contains("conflict") && e.contains("HEAD"), "{e}");
        // feature: setting and removing the same one — conflict; different features — no
        let e = run(&rule("match: t[form=saw]\nset: t[feats+=Tense=Pres, feats-=Tense]"), &d).unwrap_err();
        assert!(e.contains("FEATS.Tense"), "{e}");
        assert!(run(&rule("match: t[form=saw]\nset: t[feats+=Tense=Pres, feats-=Mood]"), &d).is_ok());
        // copying a feature from two sources with different values — conflict
        let e = run(&rule("match: s[upos=PRON|PROPN]; o[form=last]\nset: o[feats+=Case=@s]"), &doc("Last last ADJ JJ _ 0 root
            I I PRON PRP Case=Nom 1 nsubj
            me I PRON PRP Case=Acc 1 obj")).unwrap_err();
        assert!(e.contains("FEATS.Case"), "{e}");
    }

    #[test]
    fn tree_gate_catches() {
        let d = week();
        // cycle with the root intact: Last → week, week → Last
        let e = run(&rule("match: t[form=week]; v[form=last]\nset: t[head=v]"), &d).unwrap_err();
        assert!(e.contains("cycle"), "{e}");
        // root hidden in a cycle: saw → John, John → saw — 0 roots
        let e = run(&rule("match: v[rel=root]; o[rel=obj]\nset: v[head=o, rel=parataxis]"), &d).unwrap_err();
        assert!(e.contains("roots 0"), "{e}");
        // two roots
        let e = run(&rule("match: t[form=john]\nset: t[head=0, rel=root]"), &d).unwrap_err();
        assert!(e.contains("roots 2"), "{e}");
        // HEAD 0 without root
        let e = run(&rule("match: t[form=john]\nset: t[head=0]"), &d).unwrap_err();
        assert!(e.contains("root only on the root"), "{e}");
        // root not on the root
        let e = run(&rule("match: t[form=john]\nset: t[rel=root]"), &d).unwrap_err();
        assert!(e.contains("root only on the root"), "{e}");
        // head outside the sentence — raw line
        let mut s = d.sents[0].clone();
        s.word_mut(0).set(Col::Head, "9");
        assert!(tree_gate(&s).unwrap_err().contains("outside the sentence"));
        // positive control: the original sentence passes
        assert!(tree_gate(&d.sents[0]).is_ok());
    }

    #[test]
    fn bad_rules_fail() {
        assert!(parse_err("match: t[]\nset: t[rell=obl]").contains("unknown action 'rell'"));
        assert!(parse_err("match: t[]\nset: t[rel=obl:tmodd]").contains("unknown value"));
        assert!(parse_err("match: t[]\nset: t[upos=NOPE]").contains("unknown value"));
        assert!(parse_err("match: t[]\nset: t[xpos=VBQ]").contains("unknown value"));
        assert!(parse_err("match: t[]\nset: t[feats+=Nubmer=Sing]").contains("unknown feature"));
        assert!(parse_err("match: t[]\nset: t[feats+=Number=Sng]").contains("unknown feature"));
        assert!(parse_err("match: t[]\nset: t[feats-=Nubmer]").contains("unknown feature"));
        assert!(parse_err("match: t[]\nset: t[feats+=Nubmer=@t]").contains("unknown feature"));
        assert!(parse_err("match: t[]\nset: q[rel=obl]").contains("node 'q' not from match"));
        assert!(parse_err("match: t[]\nset: t[head=q]").contains("node 'q' not from match"));
        assert!(parse_err("match: t[]\nset: t[head=@q]").contains("node 'q' not from match"));
        assert!(parse_err("match: t[]\nset: t[feats+=Number=@q]").contains("node 'q' not from match"));
        assert!(parse_err("match: t[]\nset: t[head=t]").contains("its own head"));
        assert!(parse_err("match: t[]\nset: t[lemma=@lemma]").contains("lemma=@form"));
        assert!(parse_err("match: t[]\nset: t[misc+=K]").contains("misc+="));
        assert!(parse_err("match: t[]\nset: t[misc+=K=a|b]").contains("misc+="));
        assert!(parse_err("match: t[]\nset: t[rel=obl,]").contains("unknown action"));
        assert!(parse_err("match: t[]\nset: t[rel=obl, uppos=NOUN]").contains("unknown action 'uppos'"));
        assert!(parse_err("match: t[]").contains("missing field set"));
        assert!(parse_err("set: t[rel=obl]").contains("missing field match"));
        assert!(parse_err("match: t[]\nset: t[rel=obl]\nrequre: t[]").contains("unknown field"));
        assert!(parse_err("match: t[]\nset: t[rel=obl]\nset: t[rel=nmod]").contains("twice"));
        assert!(parse_err("match: t[head=q]\nset: t[rel=obl]").contains("unknown node 'q'"));
        assert!(parse_err("match: t[]\nunless: q[]\nset: t[rel=obl]").contains("unknown node 'q'"));
        let (_, e) = parse_file("```convert\nrule: t\nfrom: a\nto: a\nmatch: t[]\nset: t[rel=obl]\nsource: t\n```\n", "t.md");
        assert!(e[0].msg.contains("the same"));
        let (_, e) = parse_file("```convert\nrule: t\nto: a\nmatch: t[]\nset: t[rel=obl]\nsource: t\n```\n", "t.md");
        assert!(e[0].msg.contains("missing field from"));
        let (_, e) = parse_file("```convert\nrule: t\nfrom: b\nto: a\nmatch: t[]\nset: t[rel=obl]\n```\n", "t.md");
        assert!(e[0].msg.contains("missing field source"));
        // repeated id in two files
        let (mut a, _) = parse_file("```convert\nrule: t\nfrom: b\nto: a\nmatch: t[]\nset: t[rel=obl]\nsource: t\n```\n", "a.md");
        let (b, _) = parse_file("```convert\nrule: t\nfrom: b\nto: a\nmatch: t[]\nset: t[rel=nmod]\nsource: t\n```\n", "b.md");
        a.extend(b);
        assert_eq!(dup_ids(&a).len(), 1);
        // the converter does not take a seed ```rule block, and vice versa; ```convert-todo is a disabled rule, not a block
        assert!(parse_file("```rule\nrule: x\nmatch: t[]\nrequire: t[]\n```\n", "t.md").0.is_empty());
        let (r, e) = parse_file("```convert-todo\nrule: t\nfrom: b\nto: a\nmatch: w[feats.PronType=Int,Rel]\nset: w[rel=obl]\nsource: t\n```\n", "t.md");
        assert!(r.is_empty() && e.is_empty());
        assert_eq!(parse_file("```convert  \nrule: t\nfrom: b\nto: a\nmatch: t[]\nset: t[rel=obl]\nsource: t\n```\n", "t.md").0.len(), 1);
        assert!(expert::parse_file("```convert\nrule: t\nfrom: b\nto: a\nmatch: t[]\nset: t[rel=obl]\nsource: t\n```\n", "t.md").0.is_empty());
    }

    #[test]
    fn pair_keeps_file_order() {
        let rs = rules("```convert\nrule: b1\nfrom: x\nto: mova\nmatch: t[]\nset: t[misc+=A=1]\nsource: t\n```\n```convert\nrule: a1\nfrom: mova\nto: x\nmatch: t[]\nset: t[misc-=A]\nsource: t\n```\n```convert\nrule: a2\nfrom: x\nto: mova\nmatch: t[]\nset: t[misc+=B=1]\nsource: t\n```\n");
        let ids: Vec<&str> = pair(&rs, "x", "mova").iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["b1", "a2"]);
        assert_eq!(pair(&rs, "mova", "x").len(), 1);
        assert!(pair(&rs, "y", "mova").is_empty());
    }

    /// ud-2.14 → mova → ud-2.14 on a sentence: DEPREL of word `w` after the forward and after the reverse pass.
    fn round_trip(rs: &[ConvRule], d: &Doc, w: usize) -> (String, String) {
        let mut s = d.sents[0].clone();
        apply(&pair(rs, "ud-2.14", "mova"), &mut s).unwrap();
        let mid = col(&s, w, Col::Deprel);
        apply(&pair(rs, "mova", "ud-2.14"), &mut s).unwrap();
        (mid, col(&s, w, Col::Deprel))
    }

    #[test]
    fn dialect_ud_2_14() {
        let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../dialects/ud-2.14/convert.md")).unwrap();
        let rs = rules(&text);
        // time → tmod (by lemma and, without lemmas, by form); measure before ADJ/ADV → npmod
        assert_eq!(round_trip(&rs, &week(), 2), ("obl:unmarked".into(), "obl:tmod".into()));
        let old = doc("He he PRON PRP _ 5 nsubj
                       is be AUX VBZ _ 5 cop
                       65 65 NUM CD _ 4 nummod
                       years year NOUN NNS _ 5 obl:npmod
                       old old ADJ JJ _ 0 root");
        assert_eq!(round_trip(&rs, &old, 4), ("obl:unmarked".into(), "obl:npmod".into()));
        let nolemma = doc("I _ PRON PRP _ 2 nsubj
                           left _ VERB VBD _ 0 root
                           yesterday _ NOUN NN _ 2 obl:tmod");
        assert_eq!(round_trip(&rs, &nolemma, 3), ("obl:unmarked".into(), "obl:tmod".into()));
        let bit = doc("a a DET DT _ 2 det
                       bit bit NOUN NN _ 3 obl:npmod
                       more more ADV RBR _ 0 root");
        assert_eq!(round_trip(&rs, &bit, 2), ("obl:unmarked".into(), "obl:npmod".into()));
        // negative control: a broken rule (time → npmod) gives a difference on the same sentence
        let broken = rules(&text.replacen("set: t[rel=obl:tmod]", "set: t[rel=obl:npmod]", 1));
        assert_eq!(round_trip(&broken, &week(), 2).1, "obl:npmod");
    }

    /// The repo's `convert.md` files (`dialects/`, `treebanks/`) parse without errors and without repeated ids.
    #[test]
    fn repo_rules_all_parse() {
        let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/.."));
        let mut n = 0;
        for dir in ["dialects", "treebanks"] {
            let (r, e) = load(&root.join(dir));
            assert!(e.is_empty(), "{}", e.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n"));
            n += r.len();
        }
        let (all, _) = load(root.join("dialects").as_path());
        assert!(!pair(&all, "ud-2.14", "mova").is_empty() && !pair(&all, "mova", "ud-2.14").is_empty(), "no ud-2.14 rules");
        assert!(n > 0);
    }
}
