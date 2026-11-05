//! Rules out of text: every sentence goes through the parser (or its agreed tree from the parse database) into prag
//! frames, and a frame with a deontic marker becomes a rule. The frames under it give the act, the conditions
//! («if», «when») and the exceptions («unless», «except», «but if» in the next sentence); a «Rebbi X said»
//! frame above it gives the authority.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use prag::frames::{Ctx, Ent, Frame, Kind, Modal, Read};

use crate::rule::{Clause, Force, Rule};

/// A sentence with its address.
#[derive(Clone, Debug)]
pub struct Sent {
    pub id: String,
    pub addr: String,
    pub text: String,
    /// the default authority of its text («Torah», «Mishnah (anonymous)», a code's article)
    pub authority: String,
}

const LIABLE: [&str; 6] = ["liable", "guilty", "culpable", "punishable", "responsible", "accountable"];
const EXEMPT: [&str; 5] = ["exempt", "innocent", "blameless", "quit", "clear"];
const FORBIDDEN: [&str; 5] = ["forbidden", "prohibited", "unlawful", "abomination", "illegal"];
const PERMITTED: [&str; 5] = ["permitted", "allowed", "permissible", "lawful", "free"];
const OBLIGATED: [&str; 6] = ["obligated", "obliged", "required", "bound", "subject", "due"];
/// verbs of a sanction: a rule with one of them as its main verb is a liability with that penalty
const SANCTION: [&str; 23] = ["pay", "restore", "repay", "compensate", "replace", "put", "stone", "burn", "cut off", "cut", "bear", "punish", "fine", "imprison", "flog", "scourge", "throw", "drown", "hang", "beat", "kill", "die", "knock"];
/// nouns of guilt: «there shall be no bloodguiltiness for him» exempts, «there shall be bloodguiltiness» makes liable
const GUILT: [&str; 7] = ["bloodguiltiness", "bloodguilt", "guilt", "claim", "liability", "sin", "punishment"];

fn has(list: &[&str], w: &str) -> bool {
    list.contains(&w)
}

/// The deontic force of a frame and its penalty, when it has one.
fn force_of(f: &Frame) -> Option<Force> {
    let v = f.verb.as_str();
    let copula = matches!(v, "be" | "become" | "hold" | "remain" | "deem" | "consider");
    if copula {
        for a in &f.attrs {
            let a = a.as_str();
            if has(&LIABLE, a) {
                return Some(if f.neg { Force::Exemption } else { Force::Liability });
            }
            if has(&EXEMPT, a) || (a == "free" && f.verb == "be" && f.tam.modal.is_none() && f.agent.is_some()) {
                return Some(if f.neg { Force::Liability } else { Force::Exemption });
            }
            if has(&FORBIDDEN, a) {
                return Some(if f.neg { Force::Permission } else { Force::Prohibition });
            }
            if has(&PERMITTED, a) {
                return Some(if f.neg { Force::Prohibition } else { Force::Permission });
            }
            if has(&OBLIGATED, a) {
                return Some(if f.neg { Force::Exemption } else { Force::Obligation });
            }
        }
        // «there shall be no bloodguiltiness», «no claim can be made»
        if let Some(e) = f.agent.as_ref().or(f.patient.as_ref()) {
            if has(&GUILT, e.id.as_str()) {
                let no = e.det.as_ref().is_some_and(|d| d.0 == "no");
                return Some(if no || f.neg { Force::Exemption } else { Force::Liability });
            }
        }
    }
    if f.tam.passive {
        match v {
            "obligate" | "oblige" | "require" | "bind" => return Some(if f.neg { Force::Exemption } else { Force::Obligation }),
            "forbid" | "prohibit" => return Some(if f.neg { Force::Permission } else { Force::Prohibition }),
            "permit" | "allow" => return Some(if f.neg { Force::Prohibition } else { Force::Permission }),
            "exempt" | "free" | "acquit" => return Some(if f.neg { Force::Liability } else { Force::Exemption }),
            _ => {}
        }
    }
    let modal = f.tam.modal?;
    match modal {
        Modal::Shall | Modal::Must | Modal::Ought => {
            let v1 = v.split(' ').next().unwrap_or(v);
            let sanction_passive = f.tam.passive && (has(&SANCTION, v) || has(&SANCTION, v1));
            let sanction_active = !f.tam.passive && (matches!(v, "pay" | "restore" | "repay" | "compensate" | "replace" | "bear" | "die" | "make good") || (v == "put" && f.obls.iter().any(|o| o.1.id == "death")) || (v == "make" && f.patient.as_ref().is_some_and(|p| matches!(p.id.as_str(), "restitution" | "good" | "amends" | "compensation"))));
            if !f.neg && (sanction_passive || sanction_active) {
                Some(Force::Liability)
            } else if f.neg {
                Some(Force::Prohibition)
            } else {
                Some(Force::Obligation)
            }
        }
        Modal::May => Some(if f.neg { Force::Prohibition } else { Force::Permission }),
        Modal::Can if f.patient.as_ref().or(f.agent.as_ref()).is_some_and(|e| has(&GUILT, e.id.as_str()) && (f.neg || e.det.as_ref().is_some_and(|d| d.0 == "no"))) => Some(Force::Exemption),
        _ => None,
    }
}

/// A frame as a clause of a rule.
fn clause(f: &Frame) -> Clause {
    let mut args: Vec<String> = Vec::new();
    for a in &f.attrs {
        if a.starts_with("when:") {
            args.push(a.clone());
        }
    }
    if let Some(p) = &f.place {
        args.push(format!("place:{}", p.id));
    }
    for (prep, e) in &f.obls {
        if !matches!(prep.as_str(), "by" | "unto" | "to" | "for" | "with") {
            args.push(format!("{prep}:{}", e.id));
        }
    }
    args.sort();
    args.dedup();
    // a passive's patient is its subject: «if the theft be found» → find theft
    // a patient read from a pronoun keeps the pronoun: «kill it or sell it» is about whatever was stolen
    // a patient read from a pronoun keeps its antecedent with the heads coordinated with it («kill it or sell it»
    // after «an ox, or a sheep»: ox~sheep); an unresolved pronoun stays a pronoun and covers anything
    let patient = f.patient.as_ref().map(|e| {
        let mut alts = vec![e.id.clone()];
        alts.extend(e.and.iter().cloned());
        alts.join("~")
    });
    if f.verb == "be" {
        for a in &f.attrs {
            if !a.starts_with("when:") {
                args.push(format!("attr:{a}"));
            }
        }
    }
    Clause { verb: f.verb.clone(), neg: f.neg, patient: patient.filter(|p| p != "_"), args }
}

fn ent_text(e: &Ent) -> String {
    let mut s = if e.text.is_empty() { e.id.clone() } else { e.text.clone() };
    for c in &e.conj {
        s += &format!(" {} {}", c.cc.as_deref().unwrap_or("and"), if c.text.is_empty() { &c.id } else { &c.text });
    }
    s
}

/// Parties that are not the sanction's object: «he shall be put to death» says no more than «put to death».
const PERSONS: [&str; 16] = ["he", "she", "they", "it", "person", "man", "woman", "thief", "owner", "builder", "accuser", "shepherd", "finder", "one", "him", "who"];

/// The sanction as said: the verb as written with its objects and obliques («pay five oxen for an ox», «be put to
/// death»).
fn penalty_text(r: &Read, i: usize, f: &Frame) -> String {
    let form = r.pred.get(i).and_then(|&(s, t)| r.trees.get(s).and_then(|tr| tr.w.get(t))).map(|w| w.form.to_lowercase()).unwrap_or_else(|| f.verb.clone());
    let mut s = String::new();
    if f.tam.passive {
        if let Some(p) = f.patient.as_ref().filter(|p| !PERSONS.contains(&p.id.as_str()) && !p.pron.is_some()) {
            s += &ent_text(p);
            s += " ";
        }
        s += "be ";
    }
    s += &form;
    if f.verb.contains(' ') && !form.contains(' ') {
        s += " ";
        s += f.verb.split_once(' ').map(|x| x.1).unwrap_or("");
    }
    if let Some(p) = &f.patient {
        if !f.tam.passive {
            s += " ";
            s += &ent_text(p);
        }
    }
    if f.tam.passive {
        if let Some(a) = &f.agent {
            s += &format!(" by {}", ent_text(a));
        }
    }
    for (prep, e) in &f.obls {
        s += &format!(" {prep} {}", ent_text(e));
    }
    let head = r.pred.get(i).and_then(|&(si, t)| r.trees.get(si).and_then(|tr| tr.w.get(t))).map(|w| w.lemma.to_lowercase()).unwrap_or_default();
    for a in &f.attrs {
        if !f.manner.contains(a) && !a.starts_with("when:") && a != "surely" && *a != head && !LIABLE.contains(&a.as_str()) {
            s += &format!(" {a}");
        }
    }
    s.trim().to_lowercase()
}

/// The authority a Say frame gives its words.
fn speaker(f: &Frame) -> Option<String> {
    if f.verb == "write" {
        return Some("Torah (quoted)".into());
    }
    let a = f.agent.as_ref()?;
    if matches!(a.id.as_str(), "sage" | "sages" | "rabbi" | "rabbis") && a.plural {
        return Some("the Sages".into());
    }
    if a.named || matches!(a.id.as_str(), "rebbi" | "rav" | "rabban" | "rabbi" | "samuel") {
        // the name with its flat and compound parts: «Rebbi Yose», «Rebbi Ḥiyya bar Ada»
        let mut parts: Vec<String> = vec![a.id.clone()];
        for (rel, w, _) in &a.words {
            if matches!(rel.name(), "flat" | "compound" | "flat:name") {
                parts.push(w.clone());
            }
        }
        let t: Vec<String> = parts.iter().map(|w| {
            let mut c = w.chars();
            c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
        }).collect();
        return Some(t.join(" ").chars().take(60).collect());
    }
    None
}

/// The rules of one sentence. `prev` is the last rule of the same address, for «but if».
pub fn rules_of(jur: &str, s: &Sent, r: &Read, tree: &str) -> Vec<Rule> {
    let fr = &r.frames;
    let n = fr.len();
    if s.text.trim_end().ends_with('?') {
        return Vec::new();
    }
    let parent = |i: usize| fr[i].link.as_ref().map(|l| l.0).filter(|&p| p < n && p != i);
    let force: Vec<Option<Force>> = fr.iter().map(force_of).collect();
    // the owner of each frame: the nearest deontic frame above it
    let owner = |i: usize| -> Option<usize> {
        let mut cur = parent(i);
        let mut steps = 0;
        while let Some(p) = cur {
            if force[p].is_some() {
                return Some(p);
            }
            cur = parent(p);
            steps += 1;
            if steps > n {
                break;
            }
        }
        None
    };
    // authorities: the frames a Say frame carries (and those under them)
    let mut said_by: BTreeMap<usize, String> = BTreeMap::new();
    for (i, f) in fr.iter().enumerate() {
        if let Kind::Say(_) = f.kind {
            if let Some(who) = speaker(f) {
                for &q in &f.quote {
                    said_by.entry(q).or_insert(who.clone());
                }
                for k in 0..n {
                    if parent(k) == Some(i) && k != i {
                        said_by.entry(k).or_insert(who.clone());
                    }
                }
            }
        }
    }
    let auth_of = |i: usize| -> String {
        let mut cur = Some(i);
        let mut steps = 0;
        while let Some(c) = cur {
            if let Some(w) = said_by.get(&c) {
                return w.clone();
            }
            cur = parent(c);
            steps += 1;
            if steps > n {
                break;
            }
        }
        s.authority.clone()
    };
    // a stated reason or purpose: «for ye were strangers», «that thine ox may have rest», «because …», a cause link
    // («that»/«so that» only with may, might or shall: a purpose, not a result or a complement)
    let purpose = |i: usize| -> bool {
        let marks: Vec<&str> = fr[i].mark.iter().map(|m| m.0.as_str()).collect();
        marks.iter().any(|m| matches!(*m, "lest" | "because" | "since" | "for"))
            || fr[i].cc.as_deref() == Some("for")
            || (marks.iter().any(|m| matches!(*m, "that" | "so")) && matches!(fr[i].tam.modal, Some(Modal::May | Modal::Might | Modal::Shall)))
    };
    let deontic: Vec<usize> = (0..n).filter(|&i| force[i].is_some() && !fr[i].cond && !fr[i].irrealis && !(purpose(i) && parent(i).is_some())).collect();
    // conditions that hang on nothing deontic go to the first deontic frame after them (else the last)
    let mut loose: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let is_cond = |i: usize| fr[i].cond || fr[i].mark.iter().any(|m| matches!(m.0.as_str(), "if" | "when" | "whenever"));
    let cond_root = |i: usize| -> bool {
        let mut cur = Some(i);
        let mut steps = 0;
        while let Some(c) = cur {
            if is_cond(c) {
                return true;
            }
            if force[c].is_some() {
                return false;
            }
            cur = parent(c);
            steps += 1;
            if steps > n {
                break;
            }
        }
        false
    };
    for i in 0..n {
        if force[i].is_none() && !matches!(fr[i].kind, Kind::Say(_)) && owner(i).is_none() && cond_root(i) {
            if let Some(&d) = deontic.iter().find(|&&d| d > i).or(deontic.last()) {
                loose.entry(d).or_default().push(i);
            }
        }
    }
    let mut out = Vec::new();
    for (k, &d) in deontic.iter().enumerate() {
        let f = &fr[d];
        let fc = force[d].unwrap();
        let mut acts: Vec<Clause> = Vec::new();
        let mut conds: Vec<Vec<Clause>> = Vec::new();
        let mut excs: Vec<Vec<Clause>> = Vec::new();
        let mut cond_idx: BTreeSet<usize> = BTreeSet::new();
        let mut reasons: Vec<String> = Vec::new();
        let mut subs: Vec<usize> = (0..n).filter(|&i| i != d && (force[i].is_none() || (purpose(i) && !deontic.contains(&i))) && owner(i) == Some(d)).collect();
        subs.extend(loose.get(&d).cloned().unwrap_or_default());
        subs.sort();
        subs.dedup();
        for &i in &subs {
            let g = &fr[i];
            if matches!(g.kind, Kind::Say(_)) {
                continue;
            }
            let marks: Vec<&str> = g.mark.iter().map(|m| m.0.as_str()).collect();
            let rel = g.link.as_ref().map(|l| l.1.name()).unwrap_or("");
            let parent_cond = parent(i).is_some_and(|p| cond_idx.contains(&p));
            if purpose(i) && !g.cond && !marks.contains(&"if") {
                let mark = marks.iter().find(|m| matches!(**m, "that" | "lest" | "because" | "since" | "for" | "so")).copied().or(g.cc.as_deref()).unwrap_or("that");
                let who = g.agent.as_ref().map(|e| format!("{} ", e.id)).unwrap_or_default();
                reasons.push(format!("{mark}: {who}{}", clause(g).show()));
                continue;
            }
            if marks.iter().any(|m| matches!(*m, "unless" | "except")) {
                excs.push(vec![clause(g)]);
            } else if g.cond || marks.iter().any(|m| matches!(*m, "if" | "when" | "whenever" | "once" | "where" | "after")) || (parent_cond && rel == "conj") || (owner(i).is_none() && cond_root(i)) {
                cond_idx.insert(i);
                if parent_cond && rel == "conj" && g.cc.as_deref() == Some("or") {
                    if let Some(last) = conds.last_mut() {
                        last.push(clause(g));
                        continue;
                    }
                }
                conds.push(vec![clause(g)]);
            } else if matches!(rel, "conj" | "parataxis") {
                // a coordinated main clause: not part of this rule
            } else {
                acts.push(clause(g));
            }
        }
        let mut penalty = String::new();
        match fc {
            Force::Liability => penalty = penalty_text(r, d, f),
            Force::Prohibition | Force::Obligation | Force::Permission => {
                // «thou shalt not steal»: the frame itself is the act, unless it is a copula over its complement
                if f.verb != "be" && !f.tam.passive {
                    // the act itself is positive: «thou shalt not steal» forbids stealing
                    let mut c = clause(f);
                    c.neg = false;
                    acts.insert(0, c);
                }
            }
            _ => {}
        }
        let empty = |c: &Clause| c.verb == "be" && c.patient.is_none() && c.args.is_empty();
        acts.retain(|c| !empty(c));
        for g in conds.iter_mut() {
            g.retain(|c| !empty(c));
        }
        conds.retain(|g| !g.is_empty());
        if acts.is_empty() && conds.is_empty() {
            continue;
        }
        let mut parties = Vec::new();
        let agent = acts.first().map(|_| ()).and(subs.iter().chain(std::iter::once(&d)).find_map(|&i| fr[i].agent.as_ref()));
        if let Some(a) = agent.or(f.agent.as_ref()) {
            parties.push(format!("agent={}", a.id));
        }
        if let Some(rc) = subs.iter().chain(std::iter::once(&d)).find_map(|&i| fr[i].recipient.as_ref()) {
            parties.push(format!("recipient={}", rc.id));
        }
        out.push(Rule {
            id: format!("{jur}:{}:{}", s.id.replace(':', "."), k + 1),
            jur: jur.to_string(),
            source: s.addr.clone(),
            sent: s.id.clone(),
            authority: auth_of(d),
            force: fc,
            penalty,
            acts,
            conds,
            excs,
            parties,
            dispute: String::new(),
            tree: tree.to_string(),
            reason: reasons.join("; "),
            text: s.text.clone(),
        });
    }
    out
}

/// «But if …»: the rules of a sentence that begins so are exceptions to the last rule before them in the address.
pub fn attach_but_if(rules: &mut [Rule]) {
    for k in 1..rules.len() {
        let t = rules[k].text.trim_start().to_lowercase();
        if (t.starts_with("but if") || t.starts_with("but when")) && rules[k].source == rules[k - 1].source && rules[k].sent != rules[k - 1].sent {
            let exc: Vec<Clause> = rules[k].conds.iter().filter_map(|g| g.first().cloned()).collect();
            if !exc.is_empty() {
                rules[k - 1].excs.push(exc);
            }
        }
    }
}

/// Disputes: rules on the same halakhah (the address without its segment) and the same act, by different
/// authorities, with different force or penalty.
pub fn mark_disputes(rules: &mut [Rule]) {
    let mut groups: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    for (i, r) in rules.iter().enumerate() {
        let parts: Vec<&str> = r.source.split('.').collect();
        if parts.len() < 4 {
            continue;
        }
        let Some(v) = r.act_verb() else { continue };
        groups.entry((parts[..3].join("."), v.to_string())).or_default().push(i);
    }
    for ((h, v), ix) in groups {
        let auths: BTreeSet<&str> = ix.iter().map(|&i| rules[i].authority.as_str()).collect();
        let outcomes: BTreeSet<(Force, &str)> = ix.iter().map(|&i| (rules[i].force, rules[i].penalty.as_str())).collect();
        if auths.len() >= 2 && outcomes.len() >= 2 {
            let id = format!("dispute:{h}:{}", v.replace(' ', "_"));
            for i in ix {
                rules[i].dispute = id.clone();
            }
        }
    }
}

/// `$VAR` in a path from the data files: the environment (MOVA_BIG, MOVA_DATA).
pub fn expand(p: &str) -> Result<PathBuf> {
    let mut out = String::new();
    for (k, part) in p.split('/').enumerate() {
        if k > 0 {
            out.push('/');
        }
        if let Some(var) = part.strip_prefix('$') {
            let v = std::env::var(var).with_context(|| format!("{var} is not set (needed for {p})"))?;
            out += &v;
        } else {
            out += part;
        }
    }
    Ok(PathBuf::from(out))
}

/// Does an address fall in a filter like `EXO.21-23,LEV,NUM.30` or `Berakhot,Peah`?
pub fn in_filter(addr: &str, filter: &str) -> bool {
    let p: Vec<&str> = addr.split('.').collect();
    filter.split(',').any(|item| {
        let q: Vec<&str> = item.trim().split('.').collect();
        if q.is_empty() || p.is_empty() || q[0] != p[0] {
            return false;
        }
        match (q.get(1), p.get(1)) {
            (None, _) => true,
            (Some(range), Some(ch)) => {
                let ch: u32 = ch.parse().unwrap_or(0);
                match range.split_once('-') {
                    Some((a, b)) => a.parse::<u32>().is_ok_and(|a| a <= ch) && b.parse::<u32>().is_ok_and(|b| ch <= b),
                    None => range.parse::<u32>().is_ok_and(|a| a == ch),
                }
            }
            _ => false,
        }
    })
}

/// A sentence pool (`id<TAB>text`) and its address map (`<pool>.verses`: `id<TAB>address`), filtered.
pub fn read_pool(pool: &Path, filter: &str, default_auth: &dyn Fn(&str) -> String) -> Result<Vec<Sent>> {
    let verses_path = PathBuf::from(format!("{}.verses", pool.display()));
    let verses: BTreeMap<String, String> = std::fs::read_to_string(&verses_path).with_context(|| format!("{}", verses_path.display()))?.lines().filter_map(|l| l.split_once('\t')).map(|(a, b)| (a.to_string(), b.to_string())).collect();
    let mut out = Vec::new();
    for l in std::fs::read_to_string(pool).with_context(|| format!("{}", pool.display()))?.lines() {
        let Some((id, text)) = l.split_once('\t') else { continue };
        let Some(addr) = verses.get(id) else { continue };
        if in_filter(addr, filter) {
            out.push(Sent { id: id.to_string(), addr: addr.clone(), text: text.to_string(), authority: default_auth(addr) });
        }
    }
    Ok(out)
}

/// A jurisdiction's own text: `address<TAB>authority<TAB>text`.
pub fn read_text(path: &Path) -> Result<Vec<Sent>> {
    let mut out = Vec::new();
    for l in std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?.lines() {
        if l.trim().is_empty() || l.starts_with('#') {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 3 {
            bail!("{}: a text line needs address, authority, text: {l}", path.display());
        }
        out.push(Sent { id: c[0].to_string(), addr: c[0].to_string(), text: c[2].to_string(), authority: c[1].to_string() });
    }
    Ok(out)
}

/// Agreed trees from a parse database (`current.conllu`), by sentence id, lower-cased as the reader parses.
pub fn read_trees(path: &Path, ids: &BTreeSet<String>) -> Result<BTreeMap<String, (String, Vec<en::annotate::Word>)>> {
    let mut out = BTreeMap::new();
    for s in en::conllu::read(path)? {
        if !ids.contains(&s.id) || !s.tokens.iter().all(|t| t.tag.is_some() && t.upos.is_some()) {
            continue;
        }
        let w: Vec<en::annotate::Word> = s.tokens.iter().map(|t| en::annotate::Word { form: t.form.to_lowercase(), lemma: t.lemma.to_lowercase(), upos: t.upos.unwrap(), tag: t.tag.unwrap(), feats: t.feats, head: t.head, rel: t.rel }).collect();
        out.insert(s.id.clone(), (s.text.clone(), w));
    }
    Ok(out)
}

fn letters(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase()).collect()
}

/// Read sentences into rules. One discourse per address (a verse, a segment): pronouns resolve inside it.
pub fn extract(jur: &str, sents: &[Sent], trees: &BTreeMap<String, (String, Vec<en::annotate::Word>)>) -> Result<(Vec<Rule>, usize, usize)> {
    world::tree::annotator()?;
    let mut rules: Vec<Rule> = Vec::new();
    let (mut from_db, mut from_slm) = (0usize, 0usize);
    let mut ctx = Ctx::default();
    let mut last_addr = String::new();
    for s in sents {
        if s.addr != last_addr {
            ctx = Ctx::default();
            last_addr = s.addr.clone();
        }
        let db = trees.get(&s.id);
        let used_slm = Cell::new(false);
        let parse = |piece: &str| -> Vec<en::annotate::Word> {
            if let Some((t, w)) = db {
                if letters(t) == letters(piece) {
                    return w.clone();
                }
            }
            used_slm.set(true);
            world::frames::parse_words(piece)
        };
        let r = prag::frames::read_with(&s.text, &mut ctx, false, &parse);
        let tree = if used_slm.get() { "slm" } else { "db" };
        if used_slm.get() {
            from_slm += 1;
        } else {
            from_db += 1;
        }
        let mut rs = rules_of(jur, s, &r, tree);
        rules.append(&mut rs);
    }
    attach_but_if(&mut rules);
    mark_disputes(&mut rules);
    Ok((rules, from_db, from_slm))
}


#[cfg(test)]
mod wave31_tests {
    use super::*;

    #[test]
    fn force_priority_and_question_gate() {
        let mut f = Frame {
            kind: Kind::Bare,
            agent: None,
            verb: "carry".into(),
            neg: false,
            patient: None,
            recipient: None,
            place: None,
            attrs: Vec::new(),
            manner: Vec::new(),
            obls: Vec::new(),
            quote: Vec::new(),
            quoted: false,
            cause: None,
            next: None,
            cond: false,
            irrealis: false,
            src: (0, 0),
            how: Default::default(),
            unsaid: Vec::new(),
            words: Vec::new(),
            nps: Vec::new(),
            mark: Vec::new(),
            cc: None,
            link: None,
            tam: Default::default(),
        };
        assert_eq!(force_of(&f), None);
        f.tam.modal = Some(Modal::Must);
        assert_eq!(force_of(&f), Some(Force::Obligation));
        f.neg = true;
        assert_eq!(force_of(&f), Some(Force::Prohibition));
        f.verb = "be".into();
        f.attrs = vec!["liable".into(), "forbidden".into()];
        assert_eq!(force_of(&f), Some(Force::Exemption));
        f.verb = "carry".into();
        f.attrs.clear();
        let r = Read {
            frames: vec![f],
            ..Default::default()
        };
        let mut s = Sent {
            id: "s".into(),
            addr: "a".into(),
            text: "Must not carry?".into(),
            authority: "Synthetic".into(),
        };
        assert!(rules_of("toy", &s, &r, "hand").is_empty());
        s.text = "Must not carry.".into();
        let out = rules_of("toy", &s, &r, "hand");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].force, Force::Prohibition);
        assert_eq!(out[0].acts[0].verb, "carry");
        assert!(!out[0].acts[0].neg);
    }
}
