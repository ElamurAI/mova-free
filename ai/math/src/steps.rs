//! v4 (30.09): word problems as a transition system — the slow model walks through the numbers in text order and
//! at each step decides: take the number (SHIFT), skip it (SKIP) or combine the top two into a subproblem
//! (REDUCE with an operation and direction). The same principle as in our arc-standard parser (`en`), only instead of
//! arcs there are operations. The fast core computes every subproblem exactly.
//!
//! Features come from the UD tree (`en`): the number's unit (the noun it attaches to), verb, subject,
//! the sentence's cue words (per, each, more, left…), question words. This way the model sees "who has what and what
//! changes" (the ARIS idea, Hosseini et al. 2014: verb categories), not a bag of words.
//!
//! Training is a structured perceptron with beam search and early update (Collins & Roark 2004). The gold
//! action sequence comes from the gold tree (`arith`), if the tree can be built in text order.

use std::collections::HashMap;

use en::annotate::{Annotator, Word};
use en::gram::{Rel, UPos};

use crate::arith::{self, Op, Qty, Tree};
use crate::big::*;
use crate::memory::NUM_WORDS;

const CUES: &[&str] = &[
    "per", "each", "every", "more", "less", "fewer", "than", "times", "twice", "half", "left", "remain", "rest", "total", "altogether", "together", "combined", "all",
    "spend", "spent", "cost", "sell", "sold", "buy", "bought", "give", "gave", "lose", "lost", "eat", "ate", "use", "used", "earn", "save", "pay", "paid", "share",
    "split", "equally", "among", "between", "increase", "decrease", "discount", "off", "profit", "average", "remainder", "after", "before", "twice", "double", "triple",
    "percent", "%", "of", "a", "an", "if", "how", "much", "many", "difference", "longer", "older", "younger", "faster", "hour", "day", "week", "month", "year", "minute",
];

/// Verb class — from the static links of the global level (`global/seeds/shortcuts/possession.md`):
/// have, get, give, make, lose, move (ARIS; Roy & Roth 2018).
fn vclass_of(lemma: &str) -> &'static str {
    global::verb_class(lemma).map(|(c, _)| c).unwrap_or("")
}

/// Descriptions of a number from the UD tree.
#[derive(Clone, Debug, Default)]
pub struct Desc {
    /// the thing after "of" with a measure: "6 cups of flour" → flour
    pub of_noun: String,
    /// modifiers of the unit (amod, compound): "diet soda bottle" → [diet, soda]
    pub mods: Vec<String>,
    /// lemmas around the number (±4, excluding numbers and function words) — for linking to the question
    pub ctxw: Vec<String>,
    /// verb class (ARIS, Roy & Roth 2018): have, get, give, make, lose, compare, ""
    pub vclass: String,
    /// rate "X per Y" (UnitDep): the denominator is the unit after per/each/every/a
    pub per: String,
    pub unit: String,
    pub verb: String,
    pub subj: String,
    pub sent: usize,
    pub cues: Vec<String>,
}

fn num_val(form: &str) -> Option<f64> {
    let f = form.trim_start_matches('$').trim_end_matches('%').replace(',', "");
    if let Some(v) = f.parse::<f64>().ok() {
        return Some(if form.ends_with('%') { v / 100.0 } else { v });
    }
    let lw = form.to_lowercase();
    NUM_WORDS.iter().find(|(k, _)| *k == lw).map(|(_, v)| *v as f64)
}

fn up_to_verb(ws: &[Word], mut i: usize) -> Option<usize> {
    for _ in 0..8 {
        if ws[i].upos == UPos::VERB || ws[i].upos == UPos::AUX && ws[i].head == 0 {
            return Some(i);
        }
        if ws[i].head == 0 {
            return None;
        }
        i = ws[i].head - 1;
    }
    None
}

/// Numbers from the problem text with descriptions: numbers as in `arith::quantities` (same order and constants), descriptions
/// from the UD parse of each sentence; matched by value in order of appearance.
pub fn describe(ann: &Annotator, text: &str) -> (Vec<Qty>, Vec<Desc>, Vec<String>, Vec<Vec<String>>, (String, String, Vec<String>, Vec<(String, Vec<String>)>)) {
    let (qs, _) = arith::quantities(text);
    let toks: Vec<String> = ann.tokenize(text).into_iter().map(|t| t.form).collect();
    let mut sents: Vec<Vec<String>> = Vec::new();
    let mut cur = Vec::new();
    for t in toks {
        let end = matches!(t.as_str(), "." | "?" | "!");
        cur.push(t);
        if end {
            sents.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        sents.push(cur);
    }
    let mut found: Vec<(f64, Desc)> = Vec::new();
    let mut sent_feats: Vec<Vec<String>> = Vec::new();
    let (mut qsubj, mut qvclass) = (String::new(), String::new());
    let mut qcontent: Vec<String> = Vec::new();
    let mut qtargets: Vec<(String, Vec<String>)> = Vec::new();
    let mut qwords = Vec::new();
    let ns = sents.len();
    for (si, s) in sents.iter().enumerate() {
        let ws = ann.annotate(s);
        let lem = |i: usize| ws[i].lemma.to_lowercase();
        let cues: Vec<String> = (0..ws.len()).map(lem).filter(|l| CUES.contains(&l.as_str())).collect();
        let mut sf: Vec<String> = (0..ws.len()).filter(|&i| ws[i].upos != UPos::PUNCT && ws[i].upos != UPos::NUM).map(|i| format!("w:{}", lem(i))).collect();
        sf.extend((0..ws.len()).filter(|&i| ws[i].upos == UPos::VERB).map(|i| format!("v:{}", lem(i))));
        sf.extend((0..ws.len()).filter(|&i| ws[i].upos == UPos::NUM || num_val(&ws[i].form).is_some()).map(|i| format!("n>{}", ws.get(i + 1).map(|w| w.lemma.to_lowercase()).unwrap_or_default())));
        sf.push(format!("nnum:{}", (0..ws.len()).filter(|&i| num_val(&ws[i].form).is_some()).count().min(3)));
        if si + 1 == ns {
            sf.push("last".into());
        }
        sent_feats.push(sf);
        if si + 1 == ns {
            // targets: the noun after how many/much (the first noun after them) and after than
            let noun_mods = |u: usize| -> (String, Vec<String>) {
                let of = (0..ws.len()).find(|&k| ws[k].head == u + 1 && matches!(ws[k].rel, Rel::Nmod) && (0..ws.len()).any(|c| ws[c].head == k + 1 && ws[c].lemma.to_lowercase() == "of"));
                let h = of.map(lem).unwrap_or_else(|| lem(u));
                (h, (0..ws.len()).filter(|&k| ws[k].head == u + 1 && matches!(ws[k].rel, Rel::Amod | Rel::Compound)).map(lem).collect())
            };
            for (k, w) in ws.iter().enumerate() {
                let l = w.lemma.to_lowercase();
                if (l == "many" || l == "much") && k > 0 && ws[k - 1].lemma.to_lowercase() == "how" || l == "than" {
                    if let Some(u) = (k + 1..ws.len().min(k + 6)).find(|&j| matches!(ws[j].upos, UPos::NOUN)) {
                        qtargets.push(noun_mods(u));
                    }
                }
            }
            qcontent = (0..ws.len()).filter(|&i| matches!(ws[i].upos, UPos::NOUN | UPos::PROPN | UPos::VERB | UPos::ADJ) && !matches!(lem(i).as_str(), "many" | "much" | "do" | "be" | "have")).map(lem).collect();
            if let Some(r) = (0..ws.len()).find(|&i| ws[i].head == 0) {
                qvclass = vclass_of(&lem(r)).to_string();
                qsubj = (0..ws.len()).find(|&k| ws[k].head == r + 1 && matches!(ws[k].rel, Rel::Nsubj | Rel::NsubjPass)).map(lem).unwrap_or_default();
            }
            qwords = (0..ws.len()).filter(|&i| ws[i].upos != UPos::PUNCT && ws[i].upos != UPos::NUM).map(|i| format!("{}", lem(i))).collect();
        }
        for i in 0..ws.len() {
            let Some(v) = num_val(&ws[i].form) else { continue };
            // unit: the number's head if it is a noun; otherwise the nearest noun to the right
            let unit = if ws[i].head > 0 && matches!(ws[ws[i].head - 1].upos, UPos::NOUN | UPos::PROPN) {
                lem(ws[i].head - 1)
            } else if ws[i].form.starts_with('$') || (i > 0 && ws[i - 1].form == "$") {
                "$".into()
            } else {
                (i + 1..ws.len().min(i + 4)).find(|&k| ws[k].upos == UPos::NOUN).map(lem).unwrap_or_default()
            };
            let unit = if ws[i].form.ends_with('%') || ws.get(i + 1).is_some_and(|w| w.form == "%" || w.lemma == "percent") { "%".into() } else { unit };
            let verb_i = up_to_verb(&ws, i);
            let verb = verb_i.map(lem).unwrap_or_default();
            let subj = verb_i.and_then(|vi| (0..ws.len()).find(|&k| ws[k].head == vi + 1 && matches!(ws[k].rel, Rel::Nsubj | Rel::NsubjPass))).map(lem).unwrap_or_default();
            // rate: "per/each/every/a/an Y" to the right of the number (up to 5 words) or "$N for/an/a …"
            let per = (i + 1..ws.len().min(i + 6)).find_map(|k| {
                let l = lem(k);
                (matches!(l.as_str(), "per" | "each" | "every" | "a" | "an") && k + 1 < ws.len() && matches!(ws[k + 1].upos, UPos::NOUN | UPos::PROPN)).then(|| lem(k + 1))
            }).unwrap_or_default();
            let vclass = vclass_of(&verb).to_string();
            let unit_i: Option<usize> = if ws[i].head > 0 && matches!(ws[ws[i].head - 1].upos, UPos::NOUN | UPos::PROPN) { Some(ws[i].head - 1) } else { (i + 1..ws.len().min(i + 4)).find(|&k| ws[k].upos == UPos::NOUN) };
            let of_noun: String = unit_i.and_then(|u| (0..ws.len()).find(|&k| ws[k].head == u + 1 && matches!(ws[k].rel, Rel::Nmod) && (0..ws.len()).any(|c| ws[c].head == k + 1 && ws[c].lemma.to_lowercase() == "of"))).map(lem).unwrap_or_default();
            let mods: Vec<String> = unit_i.map(|u| (0..ws.len()).filter(|&k| ws[k].head == u + 1 && matches!(ws[k].rel, Rel::Amod | Rel::Compound)).map(lem).collect()).unwrap_or_default();
            let ctxw: Vec<String> = (i.saturating_sub(4)..(i + 5).min(ws.len())).filter(|&k| k != i && matches!(ws[k].upos, UPos::NOUN | UPos::PROPN | UPos::VERB | UPos::ADJ)).map(lem).collect();
            found.push((v, Desc { of_noun, mods, ctxw, vclass, per, unit, verb, subj, sent: si, cues: cues.clone() }));
        }
    }
    // matching: for each quantity, the next not-yet-taken number with the same value
    let mut descs = Vec::new();
    let mut from = 0;
    for x in &qs {
        if x.name.starts_with("const") {
            descs.push(Desc { unit: format!("{}", x.name), sent: ns, ..Default::default() });
            continue;
        }
        let k = (from..found.len()).find(|&k| (found[k].0 - x.val).abs() < 1e-9);
        match k {
            Some(k) => {
                descs.push(found[k].1.clone());
                from = k + 1;
            }
            None => descs.push(Desc::default()),
        }
    }
    (qs, descs, qwords, sent_feats, (qsubj, qvclass, qcontent, qtargets))
}

// ---------- transition system ----------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    Shift,
    Skip,
    /// s1 ∘ s0 (rev = false) or s0 ∘ s1 (rev = true)
    Reduce(Op, bool),
    /// s_k ∘ s0 or s0 ∘ s_k, k = 2, 3 (non-projective trees: numbers interleaved, constants from the end)
    Far(Op, bool, u8),
}

const BASE: [(Op, bool); 6] = [(Op::Add, false), (Op::Mul, false), (Op::Sub, false), (Op::Sub, true), (Op::Div, false), (Op::Div, true)];

fn acts_all() -> Vec<Act> {
    let mut v = vec![Act::Shift, Act::Skip];
    v.extend(BASE.iter().map(|&(o, r)| Act::Reduce(o, r)));
    for k in [2u8, 3] {
        v.extend(BASE.iter().map(|&(o, r)| Act::Far(o, r, k)));
    }
    v
}

fn act_name(a: Act) -> String {
    match a {
        Act::Far(o, r, k) => format!("{}@{k}", act_name(Act::Reduce(o, r))),
        a => act_name0(a).to_string(),
    }
}

fn act_name0(a: Act) -> &'static str {
    match a {
        Act::Far(..) => "far",
        Act::Shift => "SH",
        Act::Skip => "SK",
        Act::Reduce(Op::Add, _) => "+",
        Act::Reduce(Op::Mul, _) => "*",
        Act::Reduce(Op::Sub, false) => "-",
        Act::Reduce(Op::Sub, true) => "-r",
        Act::Reduce(Op::Div, false) => "/",
        Act::Reduce(Op::Div, true) => "/r",
    }
}

#[derive(Clone)]
struct Item {
    val: f64,
    tree: Tree,
    /// description of the last (in text order) leaf
    last: usize,
    op: Option<Op>,
    size: usize,
}

#[derive(Clone)]
struct State {
    stack: Vec<Item>,
    next: usize,
    score: f64,
    acts: Vec<Act>,
}

pub struct Prob {
    /// answer of another model — the v3 subproblem trees (arith): a different model for the ensemble
    pub arith_ans: Option<f64>,
    /// abstraction library (Stitch, LILO; chunking in SOAR): tree shape without numbers → share among gold train trees
    pub shapes: std::sync::Arc<std::collections::HashMap<String, f64>>,
    /// answer of the snake's world (math::qworld) and its template — a feature for the judge
    pub world_ans: Option<(f64, String)>,
    /// "ignore" trigger: numbers the search does not take (SHIFT forbidden, only SKIP)
    pub forbid: Vec<bool>,
    /// question targets: the thing after "how many/much" and after "than" — (head, modifiers)
    pub qtargets: Vec<(String, Vec<String>)>,
    /// content lemmas of the question in order (nouns, verbs, adjectives)
    pub qcontent: Vec<String>,
    /// subject and verb class of the question
    pub qsubj: String,
    pub qvclass: String,
    pub qs: Vec<Qty>,
    pub descs: Vec<Desc>,
    pub qwords: Vec<String>,
    /// sentence features (for the principle classifier) and guessed sentence principles
    pub sent_feats: Vec<Vec<String>>,
    pub sent_k: Vec<Vec<String>>,
}

pub fn prepare(ann: &Annotator, text: &str, max_q: usize) -> Prob {
    let (mut qs, mut descs, qwords, sent_feats, (qsubj, qvclass, qcontent, qtargets)) = describe(ann, text);
    qs.truncate(max_q);
    descs.truncate(max_q);
    let sent_k = vec![Vec::new(); sent_feats.len() + 1];
    Prob { arith_ans: None, shapes: Default::default(), world_ans: None, forbid: Vec::new(), qtargets, qcontent, qsubj, qvclass, qs, descs, qwords, sent_feats, sent_k }
}

/// Counter of search actions (expanded states) — for the "I am a turtle" self-measurement.
pub static ACTIONS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn h(s: &str) -> u64 {
    let mut x: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        x ^= b as u64;
        x = x.wrapping_mul(0x100000001b3);
    }
    x
}

fn legal(st: &State, n: usize, a: Act, forbid: &[bool], p: &Prob) -> bool {
    match a {
        Act::Shift => st.next < n && !forbid.get(st.next).copied().unwrap_or(false),
        Act::Skip => st.next < n,
        Act::Reduce(..) | Act::Far(..) => {
            let (op, rev, k) = match a { Act::Reduce(o, r) => (o, r, 1usize), Act::Far(o, r, k) => (o, r, k as usize), _ => unreachable!() };
            if st.stack.len() < k + 1 {
                return false;
            }
            let (x, y) = (&st.stack[st.stack.len() - 1 - k], &st.stack[st.stack.len() - 1]);
            let (l, r) = if rev { (y, x) } else { (x, y) };
            // UnitDep (Roy & Roth 2017), hard: ± only for equal units, × only for different ones — if neither is a rate
            if std::env::var("MATH_UNITHARD").as_deref() == Ok("1") {
                let (dl, dr) = (&p.descs[l.last], &p.descs[r.last]);
                let unit = |d: &Desc| if !d.of_noun.is_empty() { d.of_noun.clone() } else { d.unit.trim_end_matches('s').to_string() };
                let (ul, ur) = (unit(dl), unit(dr));
                let rate = !dl.per.is_empty() || !dr.per.is_empty();
                let leafs = l.size == 1 && r.size == 1;
                if leafs && !rate && !ul.is_empty() && !ur.is_empty() {
                    match op {
                        Op::Add | Op::Sub if ul != ur => return false,
                        Op::Mul if ul == ur => return false,
                        _ => {}
                    }
                }
            }
            match op {
                Op::Sub => l.val - r.val >= 0.0,
                Op::Div => r.val != 0.0,
                _ => true,
            }
        }
    }
}

fn apply(st: &State, a: Act) -> State {
    let mut s = st.clone();
    s.acts.push(a);
    match a {
        Act::Shift => {
            s.stack.push(Item { val: 0.0, tree: Tree::Leaf(s.next), last: s.next, op: None, size: 1 });
            s.next += 1;
        }
        Act::Skip => s.next += 1,
        Act::Reduce(..) | Act::Far(..) => {
            let (op, rev, k) = match a { Act::Reduce(o, r) => (op_of(o), r, 1usize), Act::Far(o, r, k) => (op_of(o), r, k as usize), _ => unreachable!() };
            let y = s.stack.pop().unwrap();
            let x = s.stack.remove(s.stack.len() - k);
            let (l, r) = if rev { (y.clone(), x.clone()) } else { (x.clone(), y.clone()) };
            let val = match op {
                Op::Add => l.val + r.val,
                Op::Sub => l.val - r.val,
                Op::Mul => l.val * r.val,
                Op::Div => l.val / r.val,
            };
            s.stack.push(Item { val, tree: Tree::Node(op, Box::new(l.tree), Box::new(r.tree)), last: x.last.max(y.last), op: Some(op), size: x.size + y.size });
        }
    }
    s
}

fn op_of(o: Op) -> Op {
    o
}

fn fix_leaf_vals(st: &mut State, p: &Prob) {
    if let Some(it) = st.stack.last_mut() {
        if let Tree::Leaf(i) = it.tree {
            it.val = p.qs[i].val;
        }
    }
}

/// State features (without the action); the action is added when hashing.
fn state_feats(st: &State, p: &Prob) -> Vec<String> {
    let mut f = vec!["bias".to_string()];
    let n = p.qs.len();
    let d = |i: usize| &p.descs[i];
    let nolex = matches!(std::env::var("MATH_NOLEX").as_deref(), Ok("1") | Ok("2"));
    let lex2 = std::env::var("MATH_NOLEX").as_deref() != Ok("2");
    let desc = |tag: &str, it: &Item, f: &mut Vec<String>| {
        let e = d(it.last);
        f.push(format!("{tag}vc:{}", e.vclass));
        f.push(format!("{tag}qbind:{}", p.qcontent.iter().any(|w| e.ctxw.contains(w) || *w == e.unit)));
        f.push(format!("{tag}rate:{}", !e.per.is_empty()));
        f.push(format!("{tag}subjq:{}", !e.subj.is_empty() && e.subj == p.qsubj));
        f.push(format!("{tag}vc_q:{}|{}", e.vclass, p.qvclass));
        if nolex {
            f.push(format!("{tag}op:{:?}", it.op));
            f.push(format!("{tag}sz:{}", it.size.min(3)));
            return;
        }
        f.push(format!("{tag}u:{}", e.unit));
        f.push(format!("{tag}v:{}", e.verb));
        f.push(format!("{tag}uv:{}|{}", e.unit, e.verb));
        f.push(format!("{tag}s:{}", e.subj));
        f.push(format!("{tag}op:{:?}", it.op));
        f.push(format!("{tag}sz:{}", it.size.min(3)));
        if it.val > 0.0 && it.val < 1.0 {
            f.push(format!("{tag}<1"));
        }
    };
    let k = st.stack.len();
    f.push(format!("k:{}", k.min(3)));
    f.push(format!("rest:{}", (n - st.next).min(3)));
    if k >= 1 {
        desc("s0", &st.stack[k - 1], &mut f);
    }
    if k >= 2 {
        desc("s1", &st.stack[k - 2], &mut f);
        let (a, b) = (d(st.stack[k - 1].last), d(st.stack[k - 2].last));
        // UnitDep: equal units → ±; a rate whose denominator is the other's unit → × / ÷
        f.push(format!("vcvc:{}|{}", b.vclass, a.vclass));
        f.push(format!("s0per_is_s1u:{}", !a.per.is_empty() && a.per == b.unit));
        f.push(format!("s1per_is_s0u:{}", !b.per.is_empty() && b.per == a.unit));
        f.push(format!("s0u_is_s1u_rate:{}|{}|{}", a.unit == b.unit, !a.per.is_empty(), !b.per.is_empty()));
        f.push(format!("s0vc_su_ss:{}|{}|{}", a.vclass, a.unit == b.unit, a.subj == b.subj));
        if !nolex {
            f.push(format!("uu:{}|{}", b.unit, a.unit));
        }
        f.push(format!("same_unit:{}", a.unit == b.unit));
        f.push(format!("same_sent:{}", a.sent == b.sent));
        if lex2 { f.push(format!("vv:{}|{}", b.verb, a.verb)); }
        for c in &a.cues {
            f.push(format!("s0c:{c}"));
        }
        if lex2 { f.push(format!("s1u_s0v:{}|{}", b.unit, a.verb)); }
        let same_u = a.unit == b.unit;
        let same_s = a.subj == b.subj;
        if lex2 { f.push(format!("s0v_su:{}|{same_u}", a.verb)); }
        if lex2 { f.push(format!("s0v_ss:{}|{same_s}", a.verb)); }
        f.push(format!("su_ss:{same_u}|{same_s}"));
        for c in &a.cues {
            f.push(format!("s0c_su:{c}|{same_u}"));
        }
        f.push(format!("s0u_s1u_same_sent:{}", a.sent == b.sent));
    }
    let ks = |i: usize| p.sent_k.get(p.descs[i].sent).cloned().unwrap_or_default();
    if k >= 1 {
        let s0 = &st.stack[k - 1];
        for kk in ks(s0.last) {
            f.push(format!("s0k:{kk}"));
            if k >= 2 {
                let s1 = d(st.stack[k - 2].last);
                f.push(format!("s0k_su:{kk}|{}", s1.unit == d(s0.last).unit));
                f.push(format!("s0k_ss:{kk}|{}", s1.sent == d(s0.last).sent));
            }
        }
    }
    if st.next < n {
        for kk in ks(st.next) {
            f.push(format!("b0k:{kk}"));
        }
        let e = d(st.next);
        f.push(format!("b0vc:{}", e.vclass));
        f.push(format!("b0qbind:{}", p.qcontent.iter().any(|w| e.ctxw.contains(w) || *w == e.unit)));
        f.push(format!("b0rate:{}", !e.per.is_empty()));
        f.push(format!("b0subjq:{}", !e.subj.is_empty() && e.subj == p.qsubj));
        if lex2 { f.push(format!("b0u:{}", e.unit)); }
        if lex2 { f.push(format!("b0v:{}", e.verb)); }
        if lex2 { f.push(format!("b0uv:{}|{}", e.unit, e.verb)); }
        if lex2 { f.push(format!("b0s:{}", e.subj)); }
        for c in &e.cues {
            f.push(format!("b0c:{c}"));
        }
        if k >= 1 {
            let s0 = d(st.stack[k - 1].last);
            if lex2 { f.push(format!("s0u_b0u:{}|{}", s0.unit, e.unit)); }
            f.push(format!("s0_b0_same_sent:{}", s0.sent == e.sent));
        }
        // whether the number's unit is mentioned in the question
        f.push(format!("b0u_in_q:{}", p.qwords.contains(&e.unit)));
    } else {
        for w in &p.qwords {
            f.push(format!("q:{w}"));
        }
        if k >= 1 {
            let s0 = d(st.stack[k - 1].last);
            f.push(format!("end_s0u_in_q:{}", p.qwords.contains(&s0.unit)));
        }
    }
    f
}

fn act_feats(sf: &[String], a: Act) -> Vec<u64> {
    let an = act_name(a);
    sf.iter().map(|s| h(&format!("{an}|{s}"))).collect()
}

#[derive(Default, Clone)]
pub struct Model {
    w: HashMap<u64, f64>,
    acc: HashMap<u64, f64>,
    stamp: HashMap<u64, u64>,
    t: u64,
    pub beam: usize,
}

pub struct Pred {
    pub tree: Tree,
    pub score: f64,
    pub margin: f64,
    pub acts: Vec<Act>,
}

impl Model {
    /// The same model with a different beam — for rethinking the problem in a second pass.
    pub fn with_beam(&self, beam: usize) -> Model {
        Model { beam, ..self.clone() }
    }

    pub fn new(beam: usize) -> Model {
        Model { beam, ..Default::default() }
    }
    fn wsum(&self, fs: &[u64]) -> f64 {
        fs.iter().map(|f| self.w.get(f).copied().unwrap_or(0.0)).sum()
    }
    fn update(&mut self, fs: &[u64], d: f64) {
        let t = self.t;
        for f in fs {
            let w = self.w.entry(*f).or_insert(0.0);
            let s = self.stamp.entry(*f).or_insert(0);
            *self.acc.entry(*f).or_insert(0.0) += *w * (t - *s) as f64;
            *s = t;
            *w += d;
        }
    }
    pub fn averaged(&self) -> Model {
        let mut m = Model::new(self.beam);
        for (f, w) in &self.w {
            let s = self.stamp.get(f).copied().unwrap_or(0);
            let a = self.acc.get(f).copied().unwrap_or(0.0) + w * (self.t - s) as f64;
            m.w.insert(*f, a / self.t.max(1) as f64);
        }
        m
    }

    fn seq_feats(&self, p: &Prob, acts: &[Act]) -> Vec<u64> {
        let mut st = State { stack: Vec::new(), next: 0, score: 0.0, acts: Vec::new() };
        let mut out = Vec::new();
        for &a in acts {
            out.extend(act_feats(&state_feats(&st, p), a));
            st = apply(&st, a);
            fix_leaf_vals(&mut st, p);
        }
        out
    }

    /// Beam; `gold` is for early update: returns (first step where gold fell out, the best state at that point).
    fn run(&self, p: &Prob, gold: Option<&[Act]>) -> (Vec<State>, Option<(usize, State)>) {
        let n = p.qs.len();
        let mut beam = vec![State { stack: Vec::new(), next: 0, score: 0.0, acts: Vec::new() }];
        let mut finals: Vec<State> = Vec::new();
        // max-violation (Huang, Fayong, Guo 2012): do not stop at the first fall-out of gold, but update where
        // the gap "best branch − gold prefix" is largest
        let maxviol = gold.is_some() && std::env::var("MATH_MAXVIOL").as_deref() == Ok("1");
        let mut gst = State { stack: Vec::new(), next: 0, score: 0.0, acts: Vec::new() };
        let mut worst: Option<(f64, usize, State)> = None;
        for step in 0..(3 * n + 2) {
            if maxviol {
                if let Some(g) = gold {
                    if let Some(&ga) = g.get(step) {
                        let sf = state_feats(&gst, p);
                        let sc = gst.score + self.wsum(&act_feats(&sf, ga));
                        gst = apply(&gst, ga);
                        fix_leaf_vals(&mut gst, p);
                        gst.score = sc;
                    }
                }
            }
            let mut next: Vec<State> = Vec::new();
            for st in &beam {
                if st.next >= n && st.stack.len() == 1 {
                    finals.push(st.clone());
                    continue;
                }
                let sf = state_feats(st, p);
                for a in acts_all() {
                    if !legal(st, n, a, &p.forbid, p) {
                        continue;
                    }
                    let mut s2 = apply(st, a);
                    ACTIONS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    fix_leaf_vals(&mut s2, p);
                    s2.score = st.score + self.wsum(&act_feats(&sf, a));
                    // dead ends: at the end the stack is empty
                    if s2.next >= n && s2.stack.is_empty() {
                        continue;
                    }
                    next.push(s2);
                }
            }
            if next.is_empty() {
                break;
            }
            next.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
            next.truncate(self.beam);
            if let Some(g) = gold {
                let k = step + 1;
                if k <= g.len() && !next.iter().any(|s| s.acts.len() >= k && s.acts[..k] == g[..k]) {
                    if !maxviol {
                        return (finals, Some((k, next[0].clone())));
                    }
                    let v = next[0].score - gst.score;
                    if worst.as_ref().is_none_or(|w| v > w.0) { worst = Some((v, k, next[0].clone())); }
                }
            }
            beam = next;
        }
        if let Some((_, k, st)) = worst { return (finals, Some((k, st))); }
        (finals, None)
    }

    pub fn predict(&self, p: &Prob) -> Option<Pred> {
        let (mut finals, _) = self.run(p, None);
        finals.retain(|s| {
            let v = s.stack[0].val;
            v.is_finite() && v >= 0.0 && (v - v.round()).abs() < 1e-6
        });
        finals.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        // top-K branches vote for a value (self-consistency): softmax of scores, summed per answer
        let temp: f64 = std::env::var("MATH_VOTE_T").ok().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        if temp > 0.0 && finals.len() > 1 {
            let top = finals[0].score;
            let mut by: Vec<(f64, f64, usize)> = Vec::new(); // (value, mass, index of the best branch)
            for (i, st) in finals.iter().enumerate() {
                let w = ((st.score - top) / temp).exp();
                let v = st.stack[0].val;
                match by.iter_mut().find(|x| (x.0 - v).abs() < 1e-6) {
                    Some(x) => x.1 += w,
                    None => by.push((v, w, i)),
                }
            }
            by.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            let total: f64 = by.iter().map(|x| x.1).sum();
            let b0 = &finals[by[0].2];
            let margin = (by[0].1 - by.get(1).map(|x| x.1).unwrap_or(0.0)) / total;
            return Some(Pred { tree: b0.stack[0].tree.clone(), score: by[0].1 / total, margin, acts: b0.acts.clone() });
        }
        let best = finals.first()?;
        let second = finals.iter().find(|s| (s.stack[0].val - best.stack[0].val).abs() > 1e-6).map(|s| s.score).unwrap_or(best.score - 10.0);
        Some(Pred { tree: best.stack[0].tree.clone(), score: best.score, margin: best.score - second, acts: best.acts.clone() })
    }

    /// Training step with early update. true — the prediction hit gold.
    pub fn learn(&mut self, p: &Prob, gold: &[Act]) -> bool {
        self.t += 1;
        let (finals, early) = self.run(p, Some(gold));
        if let Some((k, best)) = early {
            let gf = self.seq_feats(p, &gold[..k]);
            let bf = self.seq_feats(p, &best.acts);
            self.update(&gf, 1.0);
            self.update(&bf, -1.0);
            return false;
        }
        let best = finals.iter().max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal));
        match best {
            Some(b) if b.acts == gold => true,
            Some(b) => {
                let gf = self.seq_feats(p, gold);
                let bf = self.seq_feats(p, &b.acts);
                self.update(&gf, 1.0);
                self.update(&bf, -1.0);
                false
            }
            None => false,
        }
    }

    pub fn explain(&self, p: &Prob, acts: &[Act]) -> Vec<String> {
        let mut st = State { stack: Vec::new(), next: 0, score: 0.0, acts: Vec::new() };
        let mut out = Vec::new();
        for &a in acts {
            let sf = state_feats(&st, p);
            if let Act::Reduce(..) | Act::Far(..) = a {
                let an = act_name(a);
                let kk = match a { Act::Far(_, _, k) => k as usize, _ => 1 };
                let mut v: Vec<(String, f64)> = sf.iter().map(|s| (s.clone(), self.w.get(&h(&format!("{an}|{s}"))).copied().unwrap_or(0.0))).filter(|x| x.1 != 0.0).collect();
                v.sort_by(|x, y| y.1.abs().partial_cmp(&x.1.abs()).unwrap());
                let k = st.stack.len();
                out.push(format!("{} {an} {}: {}", st.stack[k - 1 - kk].tree.show(&p.qs), st.stack[k - 1].tree.show(&p.qs), v.iter().take(4).map(|(s, w)| format!("{s} {w:+.1}")).collect::<Vec<_>>().join(", ")));
            }
            st = apply(&st, a);
            fix_leaf_vals(&mut st, p);
        }
        out
    }
}

/// Gold action sequence for a tree: simulation — numbers in text order; as soon as the stack holds two
/// complete sibling subtrees (s0 and s1..s3), reduce them with the parent's operation. Repeated leaves (DAG) — not supported.
pub fn oracle(tree: &Tree, n: usize) -> Option<Vec<Act>> {
    // nodes: (leaf mask of the left, mask of the right, operation) for each internal node
    fn walk(t: &Tree, nodes: &mut Vec<(u64, u64, Op)>) -> Option<u64> {
        match t {
            Tree::Leaf(i) => (*i < 64).then(|| 1u64 << i),
            Tree::Node(op, l, r) => {
                let a = walk(l, nodes)?;
                let b = walk(r, nodes)?;
                if a & b != 0 {
                    return None;
                }
                nodes.push((a, b, *op));
                Some(a | b)
            }
        }
    }
    let mut nodes = Vec::new();
    let all = walk(tree, &mut nodes)?;
    let mut stack: Vec<u64> = Vec::new();
    let mut acts = Vec::new();
    let mut next = 0;
    loop {
        // reduce while possible
        let mut reduced = true;
        while reduced {
            reduced = false;
            let top = stack.len();
            for k in 1..=3usize {
                if top < k + 1 {
                    break;
                }
                let (x, y) = (stack[top - 1 - k], stack[top - 1]);
                if let Some(&(l, _, op)) = nodes.iter().find(|(l, r, _)| (*l == x && *r == y) || (*l == y && *r == x)) {
                    let rev = l == y; // the left operand is the stack top
                    acts.push(if k == 1 { Act::Reduce(op, rev) } else { Act::Far(op, rev, k as u8) });
                    stack.remove(top - 1 - k);
                    stack.pop();
                    stack.push(x | y);
                    reduced = true;
                    break;
                }
            }
        }
        if next >= n {
            break;
        }
        if all >> next & 1 == 1 {
            acts.push(Act::Shift);
            stack.push(1u64 << next);
        } else {
            acts.push(Act::Skip);
        }
        next += 1;
    }
    (stack.len() == 1 && stack[0] == all).then_some(acts)
}

pub fn exact_answer(t: &Tree, qs: &[Qty]) -> Option<Q> {
    t.exact(qs)
}

// ---------- exact gold from dataset annotations ----------

fn match_leaf(v: &Q, qs: &[Qty], used: &mut Vec<bool>) -> Option<usize> {
    let k = (0..qs.len()).find(|&i| !used[i] && qs[i].exact == *v).or_else(|| (0..qs.len()).find(|&i| qs[i].exact == *v))?;
    used[k] = true;
    Some(k)
}

fn parse_eq(toks: &[String], i: &mut usize, qs: &[Qty], used: &mut Vec<bool>, prev: &[(Q, Tree)]) -> Option<Tree> {
    fn atom(toks: &[String], i: &mut usize, qs: &[Qty], used: &mut Vec<bool>, prev: &[(Q, Tree)]) -> Option<Tree> {
        let t = toks.get(*i)?.clone();
        *i += 1;
        if t == "(" {
            let e = parse_eq(toks, i, qs, used, prev)?;
            if toks.get(*i).map(|s| s.as_str()) != Some(")") {
                return None;
            }
            *i += 1;
            return Some(e);
        }
        let v = parse_q(&t)?;
        // first the result of a previous step (GSM8K calculator steps), then a number from the problem text
        if let Some((_, tr)) = prev.iter().rev().find(|(x, _)| *x == v) {
            return Some(tr.clone());
        }
        match_leaf(&v, qs, used).map(Tree::Leaf)
    }
    fn term(toks: &[String], i: &mut usize, qs: &[Qty], used: &mut Vec<bool>, prev: &[(Q, Tree)]) -> Option<Tree> {
        let mut l = atom(toks, i, qs, used, prev)?;
        while let Some(op) = toks.get(*i).and_then(|s| match s.as_str() { "*" | "x" | "×" => Some(Op::Mul), "/" => Some(Op::Div), _ => None }) {
            *i += 1;
            l = Tree::Node(op, Box::new(l), Box::new(atom(toks, i, qs, used, prev)?));
        }
        Some(l)
    }
    let mut l = term(toks, i, qs, used, prev)?;
    while let Some(op) = toks.get(*i).and_then(|s| match s.as_str() { "+" => Some(Op::Add), "-" => Some(Op::Sub), _ => None }) {
        *i += 1;
        l = Tree::Node(op, Box::new(l), Box::new(term(toks, i, qs, used, prev)?));
    }
    Some(l)
}

fn eq_toks(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in s.chars() {
        if "+-*/()×".contains(c) {
            if !cur.trim().is_empty() {
                out.push(cur.trim().replace(',', "").trim_start_matches('$').to_string());
            }
            cur.clear();
            out.push(c.to_string());
        } else if c.is_whitespace() {
            if !cur.trim().is_empty() {
                out.push(cur.trim().replace(',', "").trim_start_matches('$').to_string());
            }
            cur.clear();
        } else {
            cur.push(c);
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().replace(',', "").trim_start_matches('$').to_string());
    }
    out
}

/// SVAMP equation ("( 290.0 / 2.0 )") → tree over the problem's numbers.
pub fn tree_from_equation(eq: &str, qs: &[Qty]) -> Option<Tree> {
    let t = eq_toks(eq);
    let mut i = 0;
    let mut used = vec![false; qs.len()];
    let tr = parse_eq(&t, &mut i, qs, &mut used, &[])?;
    (i == t.len()).then_some(tr)
}

/// GSM8K calculator steps ("<<48/2=24>>" …) → answer tree: results of previous steps become subtrees.
pub fn tree_from_calcs(answer: &str, qs: &[Qty]) -> Option<Tree> {
    let mut prev: Vec<(Q, Tree)> = Vec::new();
    let mut used = vec![false; qs.len()];
    for part in answer.split("<<").skip(1) {
        let inner = part.split(">>").next()?;
        let (e, r) = inner.rsplit_once('=')?;
        let t = eq_toks(e);
        let mut i = 0;
        let tr = parse_eq(&t, &mut i, qs, &mut used, &prev)?;
        if i != t.len() {
            return None;
        }
        let v = parse_q(r.trim().replace(',', "").trim_start_matches('$'))?;
        if tr.exact(qs)? != v {
            return None;
        }
        prev.push((v, tr));
    }
    prev.pop().map(|x| x.1)
}


// ---------- sentence principle classifier ----------

/// Principles (k= from world scripts) for a sentence: one perceptron per principle, features are words and UD verbs.
#[derive(Default, Clone)]
pub struct Princ {
    w: HashMap<u64, f64>,
    pub kinds: Vec<String>,
}

impl Princ {
    fn score(&self, k: &str, fs: &[String]) -> f64 {
        fs.iter().map(|f| self.w.get(&h(&format!("{k}|{f}"))).copied().unwrap_or(0.0)).sum::<f64>() + self.w.get(&h(&format!("{k}|bias"))).copied().unwrap_or(0.0)
    }
    pub fn train(data: &[(&Vec<String>, &Vec<String>)], kinds: &[String], epochs: usize) -> Princ {
        let mut m = Princ { w: HashMap::new(), kinds: kinds.to_vec() };
        let mut acc: HashMap<u64, f64> = HashMap::new();
        let mut t = 0.0;
        for _ in 0..epochs {
            for (fs, ys) in data {
                t += 1.0;
                for k in kinds {
                    let y = ys.contains(k);
                    let pr = m.score(k, fs) > 0.0;
                    if y != pr {
                        let d = if y { 1.0 } else { -1.0 };
                        for f in fs.iter().map(|f| format!("{k}|{f}")).chain(std::iter::once(format!("{k}|bias"))) {
                            let key = h(&f);
                            *m.w.entry(key).or_insert(0.0) += d;
                            *acc.entry(key).or_insert(0.0) += d * t;
                        }
                    }
                }
            }
        }
        // averaging (Daumé's trick): w − acc/t
        for (key, a) in acc {
            if let Some(w) = m.w.get_mut(&key) {
                *w -= a / t;
            }
        }
        m
    }
    pub fn predict(&self, fs: &[String]) -> Vec<String> {
        self.kinds.iter().filter(|k| self.score(k, fs) > 0.0).cloned().collect()
    }
}

/// Sentence principles from a world script: for each sentence @N, the set of k= (without given).
pub fn script_kinds(script: &str, n_sents: usize) -> Vec<Vec<String>> {
    let mut out = vec![Vec::new(); n_sents];
    for l in script.lines() {
        let l = l.trim();
        let Some(rest) = l.strip_prefix('@') else { continue };
        let Some((n, _)) = rest.split_once(char::is_whitespace) else { continue };
        let Ok(n) = n.parse::<usize>() else { continue };
        if let Some((_, k)) = l.rsplit_once(" k=") {
            let k = k.trim().to_string();
            if n >= 1 && n <= n_sents && k != "given" && !out[n - 1].contains(&k) {
                out[n - 1].push(k);
            }
        }
    }
    out
}


// ---------- top-K reranking (Collins 2000) ----------

/// Candidate: a finished tree from the beam, its score and answer.
pub struct Cand {
    pub tree: Tree,
    pub score: f64,
    pub val: f64,
    pub acts: Vec<Act>,
    /// how many ensemble models proposed this answer (1 — no ensemble)
    pub votes: usize,
}

impl Model {
    /// Top-K final branches with distinct values (non-negative integers).
    pub fn topk(&self, p: &Prob, k: usize) -> Vec<Cand> {
        let (mut finals, _) = self.run(p, None);
        finals.retain(|s| {
            let v = s.stack[0].val;
            v.is_finite() && v >= 0.0 && (v - v.round()).abs() < 1e-6
        });
        finals.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        let mut out: Vec<Cand> = Vec::new();
        for f in finals {
            let v = f.stack[0].val;
            if out.iter().any(|c| (c.val - v).abs() < 1e-6) {
                continue;
            }
            out.push(Cand { tree: f.stack[0].tree.clone(), score: f.score, val: v, acts: f.acts.clone(), votes: 1 });
            if out.len() >= k {
                break;
            }
        }
        out
    }
}

fn tree_ops(t: &Tree, out: &mut Vec<Op>) {
    if let Tree::Node(op, l, r) = t {
        out.push(*op);
        tree_ops(l, out);
        tree_ops(r, out);
    }
}

fn tree_leaves(t: &Tree, out: &mut Vec<usize>) {
    match t {
        Tree::Leaf(i) => out.push(*i),
        Tree::Node(_, l, r) => {
            tree_leaves(l, out);
            tree_leaves(r, out);
        }
    }
}

/// Global candidate features: used/unused numbers and whether they relate to the question, root operation × question, answer size.
pub fn cand_feats(p: &Prob, c: &Cand, rank: usize, top_score: f64) -> Vec<String> {
    let mut f = vec!["bias".to_string(), format!("rank:{}", rank.min(5))];
    if c.votes > 1 || std::env::var("MATH_VOTES").as_deref() == Ok("1") {
        f.push(format!("votes:{}", c.votes.min(3)));
        f.push(format!("votes_rank:{}|{}", c.votes.min(3), rank.min(3)));
    }
    let mut ls = Vec::new();
    tree_leaves(&c.tree, &mut ls);
    let real: Vec<usize> = (0..p.qs.len()).filter(|&i| !p.qs[i].name.starts_with("const")).collect();
    let unused: Vec<usize> = real.iter().copied().filter(|i| !ls.contains(i)).collect();
    f.push(format!("unused:{}", unused.len().min(3)));
    f.push(format!("used:{}", ls.len().min(5)));
    for &i in &unused {
        let d = &p.descs[i];
        f.push(format!("unused_uq:{}", p.qwords.contains(&d.unit)));
        f.push(format!("unused_vc:{}", d.vclass));
        f.push(format!("unused_rate:{}", !d.per.is_empty()));
    }
    let root = match &c.tree { Tree::Node(op, ..) => format!("{op:?}"), _ => "leaf".into() };
    f.push(format!("root:{root}"));
    f.push(format!("root_qvc:{root}|{}", p.qvclass));
    for w in &p.qwords {
        if CUES.contains(&w.as_str()) {
            f.push(format!("root_q:{root}|{w}"));
        }
    }
    let mut ops = Vec::new();
    tree_ops(&c.tree, &mut ops);
    let mut sig: Vec<String> = ops.iter().map(|o| format!("{o:?}")).collect();
    sig.sort();
    f.push(format!("ops:{}", sig.join("")));
    let maxin = real.iter().map(|&i| p.qs[i].val).fold(0.0f64, f64::max).max(1e-9);
    let ratio = c.val / maxin;
    let bucket = if ratio < 0.1 { "<0.1" } else if ratio < 1.0 { "<1" } else if ratio < 10.0 { "<10" } else if ratio < 100.0 { "<100" } else { ">=100" };
    f.push(format!("ratio:{bucket}"));
    f.push(format!("ratio_root:{bucket}|{root}"));
    f.push(format!("zero:{}", c.val == 0.0));
    let gap = top_score - c.score;
    f.push(format!("gap:{}", if gap < 0.5 { "<0.5" } else if gap < 2.0 { "<2" } else if gap < 5.0 { "<5" } else { ">=5" }));
    // another model (arith): answer agreement
    if let Some(av) = p.arith_ans {
        let ag = (av - c.val).abs() < 1e-6;
        f.push(format!("arith_agree:{ag}"));
        f.push(format!("arith_agree_rank:{ag}|{}", rank.min(3)));
    }
    // shape library: whether the branch shape is a typical subproblem from solved trees
    if !p.shapes.is_empty() {
        let sh = shape(&c.tree);
        let fr = p.shapes.get(&sh).copied().unwrap_or(0.0);
        f.push(format!("shape_known:{}", fr > 0.0));
        f.push(format!("shape_freq:{}", if fr == 0.0 { "0" } else if fr < 0.01 { "<1%" } else if fr < 0.05 { "<5%" } else if fr < 0.2 { "<20%" } else { ">=20%" }));
        f.push(format!("shape:{sh}"));
    }
    // the snake's world: whether the branch matches the world's answer (and which query template)
    if let Some((wv, tm)) = &p.world_ans {
        let ag = (wv - c.val).abs() < 1e-6;
        f.push(format!("world_agree:{ag}"));
        f.push(format!("world_agree_tmpl:{ag}|{tm}"));
        f.push(format!("world_agree_rank:{ag}|{}", rank.min(3)));
    }
    // linking to the question (Patel 2021, ARIS): whether content words of the question occur near the number; first match position
    let qpos = |i: usize| -> Option<usize> { p.descs.get(i).and_then(|d| p.qcontent.iter().position(|w| d.ctxw.contains(w) || *w == d.unit)) };
    let used_q = ls.iter().filter(|&&i| qpos(i).is_some()).count();
    let used_nq = ls.len() - used_q;
    let unused_q = unused.iter().filter(|&&i| qpos(i).is_some()).count();
    f.push(format!("used_q:{}", used_q.min(3)));
    f.push(format!("used_nq:{}", used_nq.min(3)));
    f.push(format!("unused_qn:{}", unused_q.min(3)));
    f.push(format!("bind:{}|{}", used_nq.min(2), unused_q.min(2)));
    // order in a subtraction/division root versus the order in the question ("more X than Y" → X − Y)
    if let Tree::Node(op @ (Op::Sub | Op::Div), l, r) = &c.tree {
        let (mut a, mut b) = (Vec::new(), Vec::new());
        tree_leaves(l, &mut a);
        tree_leaves(r, &mut b);
        let pa = a.iter().filter_map(|&i| qpos(i)).min();
        let pb = b.iter().filter_map(|&i| qpos(i)).min();
        let ord = match (pa, pb) { (Some(x), Some(y)) if x < y => "agree", (Some(x), Some(y)) if x > y => "reverse", (Some(_), None) => "left_only", (None, Some(_)) => "right_only", _ => "none" };
        f.push(format!("root_{op:?}_qorder:{ord}"));
        let than = p.qwords.iter().any(|w| w == "than");
        f.push(format!("root_{op:?}_qorder_than:{ord}|{than}"));
    }
    // UnitDep over the whole tree: ± between different units, × / ÷ without a rate; operation × verb class of the right leaf
    fn walk_units(t: &Tree, p: &Prob, f: &mut Vec<String>) -> Option<usize> {
        match t {
            Tree::Leaf(i) => Some(*i),
            Tree::Node(op, l, r) => {
                let a = walk_units(l, p, f);
                let b = walk_units(r, p, f);
                if let (Some(a), Some(b)) = (a, b) {
                    let (da, db) = (&p.descs[a], &p.descs[b]);
                    let same = !da.unit.is_empty() && da.unit == db.unit;
                    let rate = !da.per.is_empty() || !db.per.is_empty();
                    f.push(format!("n_{op:?}_same:{same}"));
                    f.push(format!("n_{op:?}_rate:{rate}"));
                    f.push(format!("n_{op:?}_vc:{}|{}", da.vclass, db.vclass));
                    f.push(format!("n_{op:?}_ss:{}", !da.subj.is_empty() && da.subj == db.subj));
                }
                // the subtree's representative is the right leaf (like the "representative number" in Roy & Roth 2018)
                b.or(a)
            }
        }
    }
    walk_units(&c.tree, p, &mut f);
    // unit of the root (last leaf) in the question
    if let Some(&last) = ls.iter().max() {
        f.push(format!("lastu_q:{}", p.qwords.contains(&p.descs[last].unit)));
    }
    f
}

#[derive(Default)]
pub struct Rerank {
    w: HashMap<u64, f64>,
}

impl Rerank {
    fn score(&self, fs: &[String]) -> f64 {
        fs.iter().map(|x| self.w.get(&h(x)).copied().unwrap_or(0.0)).sum()
    }
    /// Training: for each problem, candidates labelled "correct"; averaged perceptron.
    pub fn train(items: &[Vec<(Vec<String>, bool)>], epochs: usize) -> Rerank {
        let mut m = Rerank::default();
        let mut acc: HashMap<u64, f64> = HashMap::new();
        let mut t = 0.0;
        for _ in 0..epochs {
            for cs in items {
                if !cs.iter().any(|c| c.1) {
                    continue;
                }
                t += 1.0;
                let best = cs.iter().enumerate().max_by(|a, b| m.score(&a.1.0).partial_cmp(&m.score(&b.1.0)).unwrap()).unwrap().0;
                if cs[best].1 {
                    continue;
                }
                let gold = cs.iter().enumerate().filter(|c| c.1.1).max_by(|a, b| m.score(&a.1.0).partial_cmp(&m.score(&b.1.0)).unwrap()).unwrap().0;
                for (fs, d) in [(&cs[gold].0, 1.0), (&cs[best].0, -1.0)] {
                    for x in fs {
                        let k = h(x);
                        *m.w.entry(k).or_insert(0.0) += d;
                        *acc.entry(k).or_insert(0.0) += d * t;
                    }
                }
            }
        }
        for (k, a) in acc {
            if let Some(w) = m.w.get_mut(&k) {
                *w -= a / t.max(1.0);
            }
        }
        m
    }
    /// Judge scores for candidates (softmax for confidence).
    pub fn scores(&self, cands: &[Vec<String>]) -> Vec<f64> {
        cands.iter().map(|c| self.score(c)).collect()
    }
    pub fn pick(&self, cands: &[Vec<String>]) -> usize {
        (0..cands.len()).max_by(|&a, &b| self.score(&cands[a]).partial_cmp(&self.score(&cands[b])).unwrap()).unwrap_or(0)
    }
}

/// Value of the gold action sequence (for labelling candidates).
pub fn oracle_value(acts: &[Act], p: &Prob) -> Option<f64> {
    let mut st = State { stack: Vec::new(), next: 0, score: 0.0, acts: Vec::new() };
    for &a in acts {
        st = apply(&st, a);
        fix_leaf_vals(&mut st, p);
    }
    (st.stack.len() == 1).then(|| st.stack[0].val)
}

/// A simple TinyGSM Python function (assignments, + − * / //, parentheses, `result = …`) → tree over the problem's numbers.
/// A number not present in the text (the model computed it in its head) means no tree: this is also a quality filter.
pub fn tree_from_python(code: &str, qs: &[Qty]) -> Option<Tree> {
    let mut vars: HashMap<String, Tree> = HashMap::new();
    let mut used = vec![false; qs.len()];
    let mut in_doc = false;
    let mut result: Option<Tree> = None;
    for raw in code.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.starts_with("\"\"\"") {
            if !(line.len() > 3 && line.ends_with("\"\"\"")) {
                in_doc = !in_doc;
            }
            continue;
        }
        if in_doc || line.is_empty() || line.starts_with("def ") {
            continue;
        }
        if let Some(r) = line.strip_prefix("return ") {
            let r = r.trim();
            result = vars.get(r).cloned();
            break;
        }
        let (name, expr) = line.split_once('=')?;
        let name = name.trim();
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') || expr.contains('=') {
            return None;
        }
        // tokens: variables are substituted by trees, numbers by leaves
        let mut toks: Vec<String> = Vec::new();
        let mut cur = String::new();
        let e = expr.replace("//", "/");
        for c in e.chars() {
            if "+-*/()".contains(c) || c.is_whitespace() {
                if !cur.is_empty() {
                    toks.push(std::mem::take(&mut cur));
                }
                if !c.is_whitespace() {
                    toks.push(c.to_string());
                }
            } else {
                cur.push(c);
            }
        }
        if !cur.is_empty() {
            toks.push(cur);
        }
        // variables → markers of previous results
        let mut prev: Vec<(Q, Tree)> = Vec::new();
        let mut t2: Vec<String> = Vec::new();
        for t in toks {
            if t.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_') {
                let tr = vars.get(&t)?.clone();
                let v = tr.exact(qs)?;
                // unique marker: the value as a string; parse_eq takes the previous result by value
                prev.push((v.clone(), tr));
                t2.push(show(&v));
            } else {
                t2.push(t);
            }
        }
        let mut i = 0;
        let tr = parse_eq(&t2, &mut i, qs, &mut used, &prev)?;
        if i != t2.len() {
            return None;
        }
        vars.insert(name.to_string(), tr);
    }
    result
}

#[cfg(test)]
mod py_tests {
    use super::*;

    #[test]
    fn python_tree() {
        let q = "Mark has 10 crayons. He gives 2 crayons to his younger sister and loses another 4 while he was playing. How many crayons does Mark have left?";
        let code = "def simple_math_problem() -> int:\n    \"\"\"\n    Mark has 10 crayons.\n    \"\"\"\n    crayonsTotal = 10\n    crayonsGiven = 2\n    crayonsLost = 4\n    crayonsLeft = crayonsTotal - crayonsGiven - crayonsLost\n    result = crayonsLeft\n\n    return result\n";
        let (qs, _) = crate::arith::quantities(q);
        let t = tree_from_python(code, &qs).expect("tree");
        assert_eq!(t.exact(&qs), Some(q_(4)));
        // negative control: a number from the head (5.5) — no tree
        let bad = "def f():\n    t = 5.5\n    result = 10 / t\n    return result\n";
        assert!(tree_from_python(bad, &qs).is_none());
    }

    fn q_(i: i64) -> Q {
        crate::big::q(i)
    }
}


/// The snake's report when it cannot solve ("it can submit a report: I can't do it, but I did this and that,
/// here are my judgments, teach me this verb and what that noun is"). Whatever level 1 doesn't know is a gap.
pub fn unknowns(p: &Prob) -> (Vec<String>, Vec<String>) {
    let mut verbs = Vec::new();
    let mut nouns = Vec::new();
    for d in &p.descs {
        if !d.verb.is_empty() && d.vclass.is_empty() && global::concepts_in(&d.verb).is_empty() && !verbs.contains(&d.verb) {
            verbs.push(d.verb.clone());
        }
        if !d.unit.is_empty() && d.unit != "$" && d.unit != "%" && global::concepts_in(&d.unit).is_empty() && !nouns.contains(&d.unit) {
            nouns.push(d.unit.clone());
        }
    }
    (verbs, nouns)
}

/// Step-by-step explanation of the solution as the snake sees it ("a step-by-step explanation mode, how it
/// sees it"): each step is the numbers with their descriptions (unit, verb, class, rate) and why this operation — a chain of
/// level 1 links (verb class → change → operation; rate → multiplication; total → addition), not weights.
pub fn explain_steps(p: &Prob, t: &Tree) -> Vec<String> {
    fn rep(t: &Tree) -> usize {
        match t {
            Tree::Leaf(i) => *i,
            Tree::Node(_, _, r) => rep(r),
        }
    }
    fn go(p: &Prob, t: &Tree, out: &mut Vec<String>) -> String {
        match t {
            Tree::Leaf(i) => {
                let q = &p.qs[*i];
                let d = &p.descs[*i];
                let mut bits = Vec::new();
                if !d.unit.is_empty() { bits.push(d.unit.clone()); }
                if !d.verb.is_empty() { bits.push(format!("verb {}{}", d.verb, if d.vclass.is_empty() { String::new() } else { format!(" (class {})", d.vclass) })); }
                if !d.per.is_empty() { bits.push(format!("per {}", d.per)); }
                if bits.is_empty() { q.name.clone() } else { format!("{} [{}]", q.name, bits.join(", ")) }
            }
            Tree::Node(op, l, r) => {
                let a = go(p, l, out);
                let b = go(p, r, out);
                let v = t.exact(&p.qs).map(|x| crate::big::show(&x)).unwrap_or_default();
                let target = match op { Op::Add => "add", Op::Sub => "subtract", Op::Mul => "multiply", Op::Div => "divide" };
                // why: a path in the level 1 graph from the verb class or rate of the right number to the operation
                let d = &p.descs[rep(r)];
                let mut why = String::from("found no link in level 1 — report: teach me why this operation applies here");
                let mut starts: Vec<&str> = Vec::new();
                if !d.vclass.is_empty() { starts.push(d.vclass.as_str()); }
                // level 1 concepts for a verb without a class (from "teach me" reports)
                let vcon: Vec<&'static str> = if d.vclass.is_empty() { global::concepts_in(&d.verb).into_iter().filter(|&c| !global::CONCEPTS[c].category).map(|c| global::CONCEPTS[c].name).collect() } else { Vec::new() };
                starts.extend(vcon);
                if !d.per.is_empty() { starts.push("rate"); }
                if p.qwords.iter().any(|w| matches!(w.as_str(), "total" | "altogether" | "together" | "all")) { starts.push("total"); }
                if matches!(op, Op::Sub) && p.qwords.iter().any(|w| w == "than") { starts.insert(0, "compare"); }
                // level 1 noun categories (unit of the right number): part and whole → rate, measure, countable
                let ld = &p.descs[rep(l)];
                for c in global::concepts_in(&d.unit).into_iter().chain(global::concepts_in(&ld.unit)) {
                    if global::CONCEPTS[c].category {
                        starts.push(global::CONCEPTS[c].name);
                    }
                }
                if matches!(op, Op::Mul | Op::Div) && (!d.per.is_empty() || !ld.per.is_empty()) { starts.insert(0, "rate"); }
                if matches!(op, Op::Div) && p.qwords.iter().chain(d.cues.iter()).any(|w| matches!(w.as_str(), "each" | "equally" | "among" | "split" | "share" | "per")) { starts.insert(0, "share"); }
                for s0 in starts {
                    if let (Some(f), Some(to)) = (global::concept(s0), global::concept(target)) {
                        if let Some(path) = global::path(f, to) {
                            let names: Vec<&str> = std::iter::once(global::CONCEPTS[f].name).chain(path.iter().map(|l| global::CONCEPTS[l.to].name)).collect();
                            why = format!("{} ({})", names.join(" → "), path.iter().map(|l| l.why).collect::<Vec<_>>().join("; "));
                            break;
                        }
                    }
                }
                out.push(format!("step {}: {a} {} {b} = {v} — why: {why}", out.len() + 1, match op { Op::Add => "+", Op::Sub => "−", Op::Mul => "×", Op::Div => "÷" }));
                v
            }
        }
    }
    let mut out = Vec::new();
    let v = go(p, t, &mut out);
    out.push(format!("answer: {v}"));
    out
}


/// "Ignore this and that" trigger (a lesson from LLM hints — "extra number"): numbers near which there is
/// no content word of the question, when the question names a specific thing and ≥ 2 linked numbers remain.
pub fn ignore_set(p: &Prob) -> Vec<usize> {
    let real: Vec<usize> = (0..p.qs.len()).filter(|&i| !p.qs[i].name.starts_with("const")).collect();
    if std::env::var("MATH_IGNORE_V").as_deref() == Ok("1") {
        // v1: by overlap of nearby words with the question
        if p.qcontent.is_empty() { return Vec::new(); }
        let bound = |i: usize| p.descs.get(i).is_some_and(|d| p.qcontent.iter().any(|w| d.ctxw.contains(w) || *w == d.unit));
        let unb: Vec<usize> = real.iter().copied().filter(|&i| !bound(i)).collect();
        return if real.len() - unb.len() >= 2 { unb } else { Vec::new() };
    }
    // v2: by the number's event — its thing (head + modifiers) versus the question targets (how many X, than Y)
    if p.qtargets.is_empty() { return Vec::new(); }
    let matches = |d: &Desc| p.qtargets.iter().any(|(h, m)| {
        let head = d.unit == *h || d.unit.trim_end_matches('s') == h.trim_end_matches('s');
        // if the target has modifiers and the number has different ones, it is not the same thing ("diet soda" ≠ "regular soda")
        let mods_ok = m.is_empty() || d.mods.is_empty() || m.iter().any(|x| d.mods.contains(x));
        head && mods_ok
    });
    let with_unit: Vec<usize> = real.iter().copied().filter(|&i| p.descs.get(i).is_some_and(|d| !d.unit.is_empty())).collect();
    let bound: Vec<usize> = with_unit.iter().copied().filter(|&i| matches(&p.descs[i])).collect();
    let ign: Vec<usize> = with_unit.iter().copied().filter(|i| !bound.contains(i)).collect();
    if bound.len() >= 1 && !ign.is_empty() && real.len() - ign.len() >= 2 { ign } else { Vec::new() }
}

pub fn uses_any(t: &Tree, set: &[usize]) -> bool {
    let mut ls = Vec::new();
    tree_leaves(t, &mut ls);
    ls.iter().any(|i| set.contains(i))
}

// ---------- learned linking of a number to the question (step 4 of the "ignore" trigger plan) ----------

/// Features linking number i to the question — non-lexical (matches, classes, flags).
pub fn bind_feats(p: &Prob, i: usize) -> Vec<String> {
    let d = &p.descs[i];
    let mut f = vec!["bias".to_string()];
    let head = p.qtargets.iter().any(|(h, _)| d.unit == *h || d.unit.trim_end_matches('s') == h.trim_end_matches('s'));
    let mods = p.qtargets.iter().any(|(_, m)| m.iter().any(|x| d.mods.contains(x)));
    let mods_clash = p.qtargets.iter().any(|(h, m)| (d.unit == *h) && !m.is_empty() && !d.mods.is_empty() && !m.iter().any(|x| d.mods.contains(x)));
    f.push(format!("head:{head}"));
    f.push(format!("mods:{mods}"));
    f.push(format!("mods_clash:{mods_clash}"));
    f.push(format!("targets:{}", p.qtargets.len().min(2)));
    f.push(format!("ctx_q:{}", p.qcontent.iter().filter(|w| d.ctxw.contains(w) || **w == d.unit).count().min(3)));
    f.push(format!("unit_in_q:{}", p.qwords.contains(&d.unit)));
    f.push(format!("vc:{}", d.vclass));
    f.push(format!("subjq:{}", !d.subj.is_empty() && d.subj == p.qsubj));
    f.push(format!("rate:{}", !d.per.is_empty()));
    f.push(format!("const:{}", p.qs[i].name.starts_with("const")));
    f.push(format!("no_unit:{}", d.unit.is_empty()));
    f.push(format!("head_vc:{head}|{}", d.vclass));
    f
}

/// Which numbers are used in the gold action sequence.
pub fn used_leaves(acts: &[Act]) -> Vec<usize> {
    let mut next = 0;
    let mut out = Vec::new();
    for a in acts {
        match a {
            Act::Shift => { out.push(next); next += 1; }
            Act::Skip => next += 1,
            _ => {}
        }
    }
    out
}

#[derive(Default, Clone)]
pub struct BindModel {
    w: HashMap<u64, f64>,
}

impl BindModel {
    pub fn score(&self, f: &[String]) -> f64 {
        f.iter().map(|x| self.w.get(&h(x)).copied().unwrap_or(0.0)).sum()
    }
    /// Averaged perceptron: label = "the number is used in the gold tree".
    pub fn train(items: &[(Vec<String>, bool)], epochs: usize) -> BindModel {
        let mut m = BindModel::default();
        let mut acc: HashMap<u64, f64> = HashMap::new();
        let mut t = 0.0;
        for _ in 0..epochs {
            for (f, y) in items {
                t += 1.0;
                let pr = m.score(f) > 0.0;
                if pr != *y {
                    let d = if *y { 1.0 } else { -1.0 };
                    for x in f {
                        let k = h(x);
                        *m.w.entry(k).or_insert(0.0) += d;
                        *acc.entry(k).or_insert(0.0) += d * t;
                    }
                }
            }
        }
        for (k, a) in acc {
            if let Some(w) = m.w.get_mut(&k) { *w -= a / t.max(1.0); }
        }
        m
    }
}


/// Tree shape without numbers: "((·-·)*·)"; ± and × with sorted children.
pub fn shape(t: &Tree) -> String {
    match t {
        Tree::Leaf(_) => "·".into(),
        Tree::Node(op, l, r) => {
            let (a, b) = (shape(l), shape(r));
            let (a, b) = if matches!(op, Op::Add | Op::Mul) && a > b { (b, a) } else { (a, b) };
            format!("({a}{}{b})", match op { Op::Add => "+", Op::Sub => "-", Op::Mul => "*", Op::Div => "/" })
        }
    }
}

/// Tree from an action sequence (for the shape library).
pub fn tree_of(p: &Prob, acts: &[Act]) -> Option<Tree> {
    let mut st = State { stack: Vec::new(), next: 0, score: 0.0, acts: Vec::new() };
    for &a in acts {
        st = apply(&st, a);
        fix_leaf_vals(&mut st, p);
    }
    (st.stack.len() == 1).then(|| st.stack[0].tree.clone())
}


/// Merge the top-K of several models: identical answers merge, votes add up; order follows the first model.
pub fn merge_votes(lists: Vec<Vec<Cand>>) -> Vec<Cand> {
    let mut out: Vec<Cand> = Vec::new();
    for l in lists {
        for c in l {
            match out.iter_mut().find(|x| (x.val - c.val).abs() < 1e-6) {
                Some(x) => x.votes += 1,
                None => out.push(c),
            }
        }
    }
    out
}
