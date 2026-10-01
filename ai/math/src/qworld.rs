//! The snake writes the quantitative world of a problem by itself (v1, in the spirit of ARIS — Hosseini et al. 2014), transparently and deterministically.
//!
//! Every number in the text is a world command: owner (subject; pronoun → previous subject), thing (unit),
//! action by the level-1 verb class (have → set, get/make → add, lose/give → subtract), rate
//! «X per Y». The question is a query: state of a thing (how many left / in total), difference of states («more X than Y»), sum of changes
//! of a given class («how many lost»), share of a rate («how many each»). The exact core computes (rational numbers).
//! Unknown means «don't know» (None): level 3 does not invent.

use std::collections::BTreeMap;

use crate::big::*;
use crate::steps::Prob;

#[derive(Clone, Debug)]
pub struct Cmd {
    pub leaf: usize,
    pub owner: String,
    pub thing: String,
    pub op: char, // '=', '+', '-', 'r' (rate)
    pub val: Q,
    pub class: String,
}

#[derive(Debug, Default)]
pub struct World {
    pub cmds: Vec<Cmd>,
    /// (owner, thing) → quantity
    pub state: BTreeMap<(String, String), Q>,
    pub log: Vec<String>,
}

fn norm(t: &str) -> String {
    let t = t.to_lowercase();
    if t.len() > 3 && t.ends_with("ies") { format!("{}y", &t[..t.len() - 3]) } else if t.len() > 3 && t.ends_with('s') && !t.ends_with("ss") { t[..t.len() - 1].to_string() } else { t }
}

/// Write the world: commands from the text's numbers and the state after them.
pub fn write(p: &Prob) -> World {
    let mut w = World::default();
    let mut last_owner = String::from("?");
    let mut last_thing = String::new();
    for i in 0..p.qs.len() {
        if p.qs[i].name.starts_with("const") { continue; }
        let d = &p.descs[i];
        let owner = if d.subj.is_empty() || matches!(d.subj.as_str(), "he" | "she" | "they" | "it" | "i" | "we") { last_owner.clone() } else { d.subj.clone() };
        let thing = if !d.of_noun.is_empty() { norm(&d.of_noun) } else if d.unit.is_empty() { last_thing.clone() } else { norm(&d.unit) };
        if !d.subj.is_empty() && !matches!(d.subj.as_str(), "he" | "she" | "they" | "it" | "i" | "we") { last_owner = owner.clone(); }
        if !thing.is_empty() { last_thing = thing.clone(); }
        let op = if !d.per.is_empty() { 'r' } else {
            match global::verb_class(&d.verb).map(|x| x.1) { Some('+') => '+', Some('-') => '-', Some('±') => '-', _ => '=' }
        };
        let val = p.qs[i].exact.clone();
        let key = (owner.clone(), thing.clone());
        match op {
            '=' => { w.state.insert(key, val.clone()); }
            '+' => { let v = w.state.get(&key).cloned().unwrap_or(q(0)); w.state.insert(key, v + val.clone()); }
            '-' => { let v = w.state.get(&key).cloned().unwrap_or(q(0)); w.state.insert(key, v - val.clone()); }
            _ => {}
        }
        w.log.push(format!("{op} {owner}.{thing} {} (verb {}{})", show(&val), d.verb, if d.per.is_empty() { String::new() } else { format!(", per {}", d.per) }));
        w.cmds.push(Cmd { leaf: i, owner, thing, op, val, class: global::verb_class(&d.verb).map(|x| x.0.to_string()).unwrap_or_default() });
    }
    w
}

/// Query the world by the question. Returns (answer, explanation, numbers used) or None — «don't know».
pub fn ask(p: &Prob, w: &World) -> Option<(Q, String, Vec<usize>)> {
    let ql: Vec<&str> = p.qwords.iter().map(|s| s.as_str()).collect();
    let has = |x: &str| ql.contains(&x);
    let targets: Vec<String> = p.qtargets.iter().map(|(h, _)| norm(h)).collect();
    let by_thing = |t: &str| -> Vec<&Cmd> { w.cmds.iter().filter(|c| c.thing == t).collect() };
    // «more X than Y»: difference of states of two things (or of two owners of one thing)
    if has("than") && targets.len() >= 2 {
        let st = |t: &str| -> Option<(Q, Vec<usize>)> {
            let cs = by_thing(t);
            if cs.is_empty() { return None; }
            let owners: Vec<&String> = w.state.keys().filter(|k| k.1 == t).map(|k| &k.0).collect();
            let v = owners.iter().filter_map(|o| w.state.get(&((*o).clone(), t.to_string()))).fold(q(0), |a, b| a + b.clone());
            Some((v, cs.iter().map(|c| c.leaf).collect()))
        };
        let (a, la) = st(&targets[0])?;
        let (b, lb) = st(&targets[1])?;
        let d = a.clone() - b.clone();
        let d = if d < q(0) { -d } else { d };
        return Some((d, format!("difference of states: {}={} and {}={}", targets[0], show(&a), targets[1], show(&b)), la.into_iter().chain(lb).collect()));
    }
    // «more did he lose than he found»: difference of sums of two action classes from the question
    if has("than") {
        let classes: Vec<&str> = ql.iter().filter_map(|w| global::verb_class(w).map(|x| x.0)).filter(|c| *c != "have" && *c != "move").collect();
        if classes.len() >= 2 && classes[0] != classes[1] {
            let sum = |c: &str| -> (Q, Vec<usize>) { let cs: Vec<&Cmd> = w.cmds.iter().filter(|x| x.class == c).collect(); (cs.iter().fold(q(0), |a, x| a + x.val.clone()), cs.iter().map(|x| x.leaf).collect()) };
            let (a, la) = sum(classes[0]);
            let (b, lb) = sum(classes[1]);
            if !la.is_empty() && !lb.is_empty() {
                let d = a.clone() - b.clone();
                let d = if d < q(0) { -d } else { d };
                return Some((d, format!("difference of actions: {}={} and {}={}", classes[0], show(&a), classes[1], show(&b)), la.into_iter().chain(lb).collect()));
            }
        }
    }
    // «how many lost / gave away / ate / received»: sum of changes of this class
    let qclass = p.qvclass.as_str();
    if matches!(qclass, "lose" | "give" | "get" | "make") && !has("left") && !has("now") {
        let cs: Vec<&Cmd> = w.cmds.iter().filter(|c| c.class == qclass && (targets.is_empty() || targets.contains(&c.thing))).collect();
        if !cs.is_empty() {
            let v = cs.iter().fold(q(0), |a, c| a + c.val.clone());
            return Some((v, format!("sum of changes of class {qclass}"), cs.iter().map(|c| c.leaf).collect()));
        }
    }
    // «how many each / per unit»: whole ÷ count
    if has("each") || has("per") || has("every") {
        let cs: Vec<&Cmd> = w.cmds.iter().filter(|c| c.op == '=').collect();
        if cs.len() == 2 {
            let (a, b) = (&cs[0].val, &cs[1].val);
            let (big, small) = if a > b { (a, b) } else { (b, a) };
            if *small != q(0) {
                return Some((big.clone() / small.clone(), "share: whole ÷ count".into(), cs.iter().map(|c| c.leaf).collect()));
            }
        }
    }
    // state of the target thing (left / now / in total): sum over owners, or of everything if the target is unknown
    let t = targets.first().cloned().or_else(|| w.cmds.last().map(|c| c.thing.clone()))?;
    let keys: Vec<&(String, String)> = w.state.keys().filter(|k| k.1 == t).collect();
    if keys.is_empty() { return None; }
    let owner_q = keys.iter().find(|k| k.0 == p.qsubj);
    let (v, why) = match owner_q {
        Some(k) if !(has("total") || has("altogether") || has("together") || has("all")) => (w.state[*k].clone(), format!("state {}.{}", k.0, k.1)),
        _ => (keys.iter().fold(q(0), |a, k| a + w.state[*k].clone()), format!("total {t} over owners")),
    };
    Some((v, why, by_thing(&t).iter().map(|c| c.leaf).collect()))
}

// ---------- training the world on verified LLM scripts ----------

/// Labels from a world script (world::quant): for each number in the text — action ('=', '+', '-', 'r') and cell (owner.thing).
pub fn script_labels(script: &str, p: &Prob) -> Vec<Option<(char, String)>> {
    let mut out: Vec<Option<(char, String)>> = vec![None; p.qs.len()];
    let mut used = vec![false; p.qs.len()];
    for l in script.lines() {
        let l = l.trim();
        let Some(rest) = l.strip_prefix('@') else { continue };
        let Some((_, body)) = rest.split_once(char::is_whitespace) else { continue };
        let body = body.split(" k=").next().unwrap_or(body).trim();
        let (verb, rest) = body.split_once(char::is_whitespace).unwrap_or((body, ""));
        if verb == "ask" { continue; }
        let Some((lhs, rhs)) = rest.split_once('=') else { continue };
        let target = lhs.split("->").last().unwrap_or(lhs).trim().to_lowercase();
        let from = lhs.split("->").next().unwrap_or(lhs).trim().to_lowercase();
        let rhs = rhs.trim();
        let single = parse_q(&rhs.replace(',', ""));
        // numbers on the right-hand side
        let nums: Vec<Q> = rhs.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '/')).filter_map(|t| parse_q(t)).collect();
        for v in nums {
            let Some(k) = (0..p.qs.len()).find(|&i| !used[i] && p.qs[i].exact == v && !p.qs[i].name.starts_with("const")) else { continue };
            used[k] = true;
            let op = match (verb, single.is_some()) {
                ("set", true) => '=',
                ("add", true) => '+',
                ("sub", true) => '-',
                ("move", true) => '-',
                _ => 'r',
            };
            let slot = if verb == "move" { from.clone() } else { target.clone() };
            out[k] = Some((op, slot));
        }
    }
    out
}

fn hh(s: &str) -> u64 {
    let mut x: u64 = 0xcbf29ce484222325;
    for b in s.bytes() { x ^= b as u64; x = x.wrapping_mul(0x100000001b3); }
    x
}

fn op_feats(p: &Prob, i: usize) -> Vec<String> {
    let d = &p.descs[i];
    let real: Vec<usize> = (0..p.qs.len()).filter(|&k| !p.qs[k].name.starts_with("const")).collect();
    let pos = real.iter().position(|&k| k == i).unwrap_or(0);
    let mut f = vec!["bias".to_string(), format!("vc:{}", d.vclass), format!("verb:{}", d.verb), format!("rate:{}", !d.per.is_empty()), format!("first:{}", pos == 0), format!("subjq:{}", !d.subj.is_empty() && d.subj == p.qsubj), format!("unit:{}", !d.unit.is_empty()), format!("of:{}", !d.of_noun.is_empty())];
    for c in &d.cues { f.push(format!("c:{c}")); }
    f
}

fn pair_feats(p: &Prob, i: usize, j: usize) -> Vec<String> {
    let (a, b) = (&p.descs[i], &p.descs[j]);
    let th = |d: &crate::steps::Desc| if !d.of_noun.is_empty() { norm(&d.of_noun) } else { norm(&d.unit) };
    let pron = |s: &str| matches!(s, "he" | "she" | "they" | "it" | "i" | "we" | "");
    vec![
        "bias".into(),
        format!("same_thing:{}", !th(a).is_empty() && th(a) == th(b)),
        format!("same_subj:{}", !a.subj.is_empty() && a.subj == b.subj),
        format!("pron_b:{}", pron(&b.subj)),
        format!("same_sent:{}", a.sent == b.sent),
        format!("vc:{}|{}", a.vclass, b.vclass),
        format!("rate:{}|{}", !a.per.is_empty(), !b.per.is_empty()),
        format!("mods_same:{}", !a.mods.is_empty() && a.mods == b.mods),
        format!("mods_diff:{}", !a.mods.is_empty() && !b.mods.is_empty() && a.mods != b.mods),
    ]
}

#[derive(Default)]
pub struct WorldModel {
    op_w: std::collections::HashMap<u64, f64>,
    pair_w: std::collections::HashMap<u64, f64>,
}

const OPS: [char; 4] = ['=', '+', '-', 'r'];

impl WorldModel {
    fn op_score(&self, f: &[String], o: char) -> f64 { f.iter().map(|x| self.op_w.get(&hh(&format!("{o}|{x}"))).copied().unwrap_or(0.0)).sum() }
    fn pair_score(&self, f: &[String]) -> f64 { f.iter().map(|x| self.pair_w.get(&hh(x)).copied().unwrap_or(0.0)).sum() }
    pub fn op(&self, p: &Prob, i: usize) -> char {
        let f = op_feats(p, i);
        *OPS.iter().max_by(|a, b| self.op_score(&f, **a).partial_cmp(&self.op_score(&f, **b)).unwrap()).unwrap()
    }
    pub fn same(&self, p: &Prob, i: usize, j: usize) -> f64 { self.pair_score(&pair_feats(p, i, j)) }

    /// Perceptrons (no averaging, 10 epochs): the number's action (multiclass) and «same cell» (pairwise).
    pub fn train(items: &[(&Prob, Vec<Option<(char, String)>>)]) -> WorldModel {
        let mut m = WorldModel::default();
        for _ in 0..10 {
            for (p, lab) in items {
                for i in 0..p.qs.len() {
                    let Some((o, _)) = &lab[i] else { continue };
                    let f = op_feats(p, i);
                    let pr = m.op(p, i);
                    if pr != *o {
                        for x in &f {
                            *m.op_w.entry(hh(&format!("{o}|{x}"))).or_default() += 1.0;
                            *m.op_w.entry(hh(&format!("{pr}|{x}"))).or_default() -= 1.0;
                        }
                    }
                }
                for i in 0..p.qs.len() {
                    for j in i + 1..p.qs.len() {
                        let (Some((_, si)), Some((_, sj))) = (&lab[i], &lab[j]) else { continue };
                        let y = si == sj;
                        let f = pair_feats(p, i, j);
                        if (m.pair_score(&f) > 0.0) != y {
                            let d = if y { 1.0 } else { -1.0 };
                            for x in &f { *m.pair_w.entry(hh(x)).or_default() += d; }
                        }
                    }
                }
            }
        }
        m
    }
}

/// World v2: action is a trained classifier; cells are greedy clustering with the trained pairwise model.
pub fn write_learned(p: &Prob, m: &WorldModel) -> World {
    let mut w = World::default();
    let real: Vec<usize> = (0..p.qs.len()).filter(|&k| !p.qs[k].name.starts_with("const")).collect();
    let mut slot_of: Vec<usize> = Vec::new();
    let mut names: Vec<(String, String)> = Vec::new();
    for (ri, &i) in real.iter().enumerate() {
        let best = (0..ri).map(|pj| (pj, m.same(p, real[pj], i))).filter(|x| x.1 > 0.0).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let s = match best { Some((pj, _)) => slot_of[pj], None => { let d = &p.descs[i]; names.push((if d.subj.is_empty() { "?".into() } else { d.subj.clone() }, if !d.of_noun.is_empty() { norm(&d.of_noun) } else { norm(&d.unit) })); names.len() - 1 } };
        slot_of.push(s);
        let (owner, thing) = names[s].clone();
        let op = m.op(p, i);
        let val = p.qs[i].exact.clone();
        let key = (owner.clone(), thing.clone());
        match op {
            '=' => { w.state.insert(key, val.clone()); }
            '+' => { let v = w.state.get(&key).cloned().unwrap_or(q(0)); w.state.insert(key, v + val.clone()); }
            '-' => { let v = w.state.get(&key).cloned().unwrap_or(q(0)); w.state.insert(key, v - val.clone()); }
            _ => {}
        }
        w.log.push(format!("{op} [{s}]{owner}.{thing} {}", show(&val)));
        w.cmds.push(Cmd { leaf: i, owner, thing, op, val, class: global::verb_class(&p.descs[i].verb).map(|x| x.0.to_string()).unwrap_or_default() });
    }
    w
}

// ---------- learned query ----------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tmpl { State, Diff, Sum, Ratio, Product }

pub const TMPLS: [Tmpl; 5] = [Tmpl::State, Tmpl::Diff, Tmpl::Sum, Tmpl::Ratio, Tmpl::Product];

/// Query template from a script: the main operation of the expression of the queried cell (by its last definition).
pub fn script_template(script: &str) -> Option<Tmpl> {
    let mut defs: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut ask = None;
    for l in script.lines() {
        let l = l.trim();
        let Some(rest) = l.strip_prefix('@') else { continue };
        let Some((_, body)) = rest.split_once(char::is_whitespace) else { continue };
        let body = body.split(" k=").next().unwrap_or(body).trim();
        let (verb, rest) = body.split_once(char::is_whitespace).unwrap_or((body, ""));
        if verb == "ask" { ask = Some(rest.trim().to_string()); continue; }
        if let Some((lhs, rhs)) = rest.split_once('=') {
            let t = lhs.split("->").last().unwrap_or(lhs).trim().to_lowercase();
            let e = match verb { "add" => format!("{t} + {}", rhs.trim()), "sub" => format!("{t} - {}", rhs.trim()), "mul" => format!("{t} * {}", rhs.trim()), "div" => format!("{t} / {}", rhs.trim()), _ => rhs.trim().to_string() };
            defs.insert(t, e);
        }
    }
    let mut e = ask?;
    for _ in 0..3 {
        let t = e.trim().to_lowercase();
        if t.contains(|c: char| "+-*/".contains(c)) { break; }
        match defs.get(&t) { Some(d) => e = d.clone(), None => break }
    }
    let top = |e: &str| -> Option<char> {
        let mut depth = 0;
        let mut found: Option<char> = None;
        for c in e.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '+' | '-' if depth == 0 => found = Some(c),
                '*' | '/' if depth == 0 && !matches!(found, Some('+') | Some('-')) => found = Some(c),
                _ => {}
            }
        }
        found
    };
    Some(match top(&e) { Some('-') => Tmpl::Diff, Some('+') => Tmpl::Sum, Some('/') => Tmpl::Ratio, Some('*') => Tmpl::Product, _ => Tmpl::State })
}

fn ask_feats(p: &Prob) -> Vec<String> {
    let mut f = vec!["bias".to_string(), format!("qvc:{}", p.qvclass), format!("targets:{}", p.qtargets.len().min(2))];
    for w in &p.qwords {
        if matches!(w.as_str(), "than" | "more" | "less" | "fewer" | "left" | "remain" | "now" | "total" | "altogether" | "together" | "all" | "each" | "per" | "every" | "times" | "much" | "many" | "cost" | "spend" | "earn" | "average" | "difference" | "long" | "if" | "need" | "still") {
            f.push(format!("q:{w}"));
        }
    }
    let n_rate = p.descs.iter().filter(|d| !d.per.is_empty()).count();
    f.push(format!("rates:{}", n_rate.min(2)));
    f
}

#[derive(Default)]
pub struct AskModel { w: std::collections::HashMap<u64, f64> }

impl AskModel {
    fn sc(&self, f: &[String], t: Tmpl) -> f64 { f.iter().map(|x| self.w.get(&hh(&format!("{t:?}|{x}"))).copied().unwrap_or(0.0)).sum() }
    pub fn predict(&self, p: &Prob) -> Tmpl {
        let f = ask_feats(p);
        *TMPLS.iter().max_by(|a, b| self.sc(&f, **a).partial_cmp(&self.sc(&f, **b)).unwrap()).unwrap()
    }
    pub fn train(items: &[(&Prob, Tmpl)]) -> AskModel {
        let mut m = AskModel::default();
        for _ in 0..15 {
            for (p, t) in items {
                let pr = m.predict(p);
                if pr != *t {
                    for x in ask_feats(p) {
                        *m.w.entry(hh(&format!("{t:?}|{x}"))).or_default() += 1.0;
                        *m.w.entry(hh(&format!("{pr:?}|{x}"))).or_default() -= 1.0;
                    }
                }
            }
        }
        m
    }
}

/// Query by a learned template.
pub fn ask_tmpl(p: &Prob, w: &World, t: Tmpl) -> Option<(Q, String, Vec<usize>)> {
    let targets: Vec<String> = p.qtargets.iter().map(|(h, _)| norm(h)).collect();
    let slots: Vec<(&(String, String), &Q)> = w.state.iter().collect();
    let pick_thing = |cs: &[&Cmd]| -> Vec<usize> { cs.iter().map(|c| c.leaf).collect() };
    match t {
        Tmpl::Diff => {
            // two cells: by the question's targets, otherwise the two largest numbers in the text
            if targets.len() >= 2 || w.cmds.len() >= 2 {
                if let Some(r) = ask(p, w).filter(|r| r.1.starts_with("difference")) { return Some(r); }
                // cells by the question's targets: two different targets — one for each; one target — the two largest with that thing
                let by = |t: &str| -> Vec<&Cmd> { let mut v: Vec<&Cmd> = w.cmds.iter().filter(|c| c.thing == t).collect(); v.sort_by(|a, b| b.val.partial_cmp(&a.val).unwrap()); v };
                if targets.len() >= 2 && targets[0] != targets[1] {
                    if let (Some(a), Some(b)) = (by(&targets[0]).first().copied(), by(&targets[1]).first().copied()) {
                        let d = a.val.clone() - b.val.clone();
                        return Some((if d < q(0) { -d } else { d }, format!("difference of targets {} and {}", targets[0], targets[1]), vec![a.leaf, b.leaf]));
                    }
                }
                if let Some(t0) = targets.first() {
                    let v = by(t0);
                    if v.len() >= 2 { return Some((v[0].val.clone() - v[1].val.clone(), format!("difference of two {t0}"), vec![v[0].leaf, v[1].leaf])); }
                }
                let mut vals: Vec<&Cmd> = w.cmds.iter().collect();
                vals.sort_by(|a, b| b.val.partial_cmp(&a.val).unwrap());
                let (a, b) = (vals[0], vals[1]);
                return Some((a.val.clone() - b.val.clone(), "difference of the two largest".into(), vec![a.leaf, b.leaf]));
            }
            None
        }
        Tmpl::Sum => {
            let cs: Vec<&Cmd> = w.cmds.iter().filter(|c| targets.is_empty() || targets.contains(&c.thing)).collect();
            let cs = if cs.len() >= 2 { cs } else { w.cmds.iter().collect() };
            if cs.len() < 2 { return None; }
            Some((cs.iter().fold(q(0), |a, c| a + c.val.clone()), "sum".into(), pick_thing(&cs)))
        }
        Tmpl::Ratio => {
            let mut vals: Vec<&Cmd> = w.cmds.iter().collect();
            if vals.len() < 2 { return None; }
            vals.sort_by(|a, b| b.val.partial_cmp(&a.val).unwrap());
            let (a, b) = (vals[0], vals[vals.len() - 1]);
            if b.val == q(0) { return None; }
            Some((a.val.clone() / b.val.clone(), "share: largest ÷ smallest".into(), vec![a.leaf, b.leaf]))
        }
        Tmpl::Product => {
            let r = w.cmds.iter().find(|c| c.op == 'r').or_else(|| w.cmds.first())?;
            let o = w.cmds.iter().filter(|c| c.leaf != r.leaf).max_by(|a, b| a.val.partial_cmp(&b.val).unwrap())?;
            Some((r.val.clone() * o.val.clone(), "product: rate × count".into(), vec![r.leaf, o.leaf]))
        }
        Tmpl::State => {
            let _ = slots;
            ask(p, w)
        }
    }
}
