//! Multi-pass coreference sieve (Raghunathan et al. 2010 — papers/r/ra/raghunathan-2010-multi-pass-sieve-coreference-resolution;
//! Lee et al. 2013 — papers/l/le/lee-2013-deterministic-coreference-resolution-based-entity): sieves from the most precise
//! to the loosest, each sees the clusters of the previous ones (cluster features are the union of mention features).
//! Only a mention that is first in its cluster gets linked; candidates are visited in Hobbs order
//! (breadth-first tree traversal left to right: first the mention's clause, then the rest of the sentence, then previous sentences).
//! Every link is a `Link`: sieve id, from whom to whom, and evidence.

use std::collections::{HashMap, HashSet};

use en::gram::{Feat, Rel, UPos};

use crate::doc::{Document, Sent};
use crate::mention::{self, ANIM, FEM, INAN, Kind, MASC, Mention, NEUT, P1, P2, P3, PL, Pron, SG};

/// Sieve (rule) — id in output, explanations and tables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Sieve {
    Speaker,
    Exact,
    Relaxed,
    Appos,
    Pred,
    Relpron,
    Acronym,
    Reflexive,
    StrictA,
    StrictB,
    StrictC,
    ProperHead,
    RelaxedHead,
    Pronoun,
    /// Baseline: pronoun → nearest agreeing mention.
    Nearest,
}

impl Sieve {
    /// Default sieve: all except strict_c and relaxed_head — on GUM dev they have precision 31% and 24%
    /// and do not add CoNLL F1 (contribution table in).
    pub const DEFAULT: [Sieve; 12] = [
        Sieve::Speaker,
        Sieve::Exact,
        Sieve::Relaxed,
        Sieve::Appos,
        Sieve::Pred,
        Sieve::Relpron,
        Sieve::Acronym,
        Sieve::Reflexive,
        Sieve::StrictA,
        Sieve::StrictB,
        Sieve::ProperHead,
        Sieve::Pronoun,
    ];
    /// Full sieve (all 14) in application order — for the contribution table.
    pub const FULL: [Sieve; 14] = [
        Sieve::Speaker,
        Sieve::Exact,
        Sieve::Relaxed,
        Sieve::Appos,
        Sieve::Pred,
        Sieve::Relpron,
        Sieve::Acronym,
        Sieve::Reflexive,
        Sieve::StrictA,
        Sieve::StrictB,
        Sieve::StrictC,
        Sieve::ProperHead,
        Sieve::RelaxedHead,
        Sieve::Pronoun,
    ];
    pub fn id(self) -> &'static str {
        match self {
            Sieve::Speaker => "speaker",
            Sieve::Exact => "exact",
            Sieve::Relaxed => "relaxed",
            Sieve::Appos => "appos",
            Sieve::Pred => "pred",
            Sieve::Relpron => "relpron",
            Sieve::Acronym => "acronym",
            Sieve::Reflexive => "reflexive",
            Sieve::StrictA => "strict_a",
            Sieve::StrictB => "strict_b",
            Sieve::StrictC => "strict_c",
            Sieve::ProperHead => "proper_head",
            Sieve::RelaxedHead => "relaxed_head",
            Sieve::Pronoun => "pronoun",
            Sieve::Nearest => "nearest",
        }
    }
    pub fn about(self) -> &'static str {
        match self {
            Sieve::Speaker => "speaker: I/we/you of the same speaker (# speaker, quotes, quote author)",
            Sieve::Exact => "exact match of mention text (nominals)",
            Sieve::Relaxed => "match of text up to and including the head (without postmodifiers)",
            Sieve::Appos => "apposition (appos in the tree)",
            Sieve::Pred => "predicative noun (subject — cop — noun)",
            Sieve::Relpron => "relative pronoun → the noun the relative clause is attached to",
            Sieve::Acronym => "acronym: capital letters of the name",
            Sieve::Reflexive => "reflexive pronoun → subject of its predicate",
            Sieve::StrictA => "head in cluster + words ⊆ + modifiers ⊆",
            Sieve::StrictB => "head in cluster + words ⊆",
            Sieve::StrictC => "head in cluster + modifiers ⊆",
            Sieve::ProperHead => "same proper-name head, no conflicting proper names or numbers in modifiers",
            Sieve::RelaxedHead => "proper-name head is among the cluster's words + words ⊆",
            Sieve::Pronoun => "3rd-person pronoun: agreement, binding, Hobbs order, ≤ 3 sentences",
            Sieve::Nearest => "baseline: nearest preceding agreeing mention",
        }
    }
    pub fn parse(s: &str) -> Option<Sieve> {
        Sieve::FULL.iter().copied().chain([Sieve::Nearest]).find(|x| x.id() == s)
    }
}

/// Link: mention `from` is attached to the cluster of mention `to` by sieve `sieve`; `why` is the evidence.
#[derive(Clone, Debug)]
pub struct Link {
    pub sieve: Sieve,
    pub from: usize,
    pub to: usize,
    pub why: String,
}

/// Settings.
#[derive(Clone, Debug)]
pub struct Config {
    pub sieves: Vec<Sieve>,
    /// A 3rd-person pronoun looks for an antecedent no further than this many sentences back (Lee et al. 2013: 3).
    pub pron_window: usize,
    /// he/she first look for a cluster with animacy evidence; if none — then any agreeing one.
    pub prefer_animate: bool,
    /// Also link demonstratives this/that/these/those (often to events, not to nominals).
    pub demonstratives: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config { sieves: Sieve::DEFAULT.to_vec(), pron_window: 3, prefer_animate: true, demonstratives: false }
    }
}

/// Who speaks: speaker from `# speaker`, quote, narrator.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Voice {
    Narrator,
    Speaker(String),
    Quote(u32),
}

/// Cluster: mentions and merged features.
#[derive(Clone, Debug, Default)]
struct Cluster {
    members: Vec<usize>,
    number: u8,
    gender: u8,
    animacy: u8,
    person: u8,
    /// Voices from which the cluster has an "I".
    voices_i: HashSet<Voice>,
}

pub struct Resolver<'a> {
    pub doc: &'a Document,
    pub ms: Vec<Mention>,
    pub links: Vec<Link>,
    cfg: Config,
    /// Cluster of each mention (index in `clusters`).
    of: Vec<usize>,
    clusters: Vec<Cluster>,
    /// Quote author: quote number → speaker mention.
    quote_author: HashMap<u32, usize>,
    /// Mention by head (sentence, word) — the proper one (without coordination).
    by_head: HashMap<(usize, usize), usize>,
}

/// Experiment switches on dev (`COREF_X=name,…`).
fn opt_x(name: &str) -> bool {
    std::env::var("COREF_X").is_ok_and(|v| v.split(',').any(|x| x == name))
}

fn agree(a: u8, b: u8) -> bool {
    a == 0 || b == 0 || a & b != 0
}

fn feat_name(kind: &str, v: u8) -> String {
    let names: &[(u8, &str)] = match kind {
        "n" => &[(SG, "Sing"), (PL, "Plur")],
        "g" => &[(MASC, "Masc"), (FEM, "Fem"), (NEUT, "Neut")],
        "a" => &[(ANIM, "Anim"), (INAN, "Inan")],
        _ => &[(P1, "1"), (P2, "2"), (P3, "3")],
    };
    if v == 0 {
        return "?".into();
    }
    names.iter().filter(|(b, _)| v & b != 0).map(|(_, n)| *n).collect::<Vec<_>>().join(",")
}

impl<'a> Resolver<'a> {
    pub fn new(doc: &'a Document, cfg: Config) -> Resolver<'a> {
        let ms = mention::detect(doc);
        let mut r = Resolver { doc, ms, links: Vec::new(), cfg, of: Vec::new(), clusters: Vec::new(), quote_author: HashMap::new(), by_head: HashMap::new() };
        for i in 0..r.ms.len() {
            let v = r.voice(i);
            let m = &r.ms[i];
            let mut c = Cluster { members: vec![i], number: m.number, gender: m.gender, animacy: m.animacy, person: m.person, voices_i: HashSet::new() };
            if m.is_ppr() && m.person == P1 && m.number == SG {
                c.voices_i.insert(v);
            }
            r.of.push(i);
            r.clusters.push(c);
            if !m.coord {
                r.by_head.insert((m.span.sent, m.head), i);
            }
        }
        r.find_quote_authors();
        // names of speakers and addressees from # speaker / # addressee: a proper-name mention with such text is animate
        let names: HashSet<String> = doc.sents.iter().flat_map(|s| s.speaker.iter().chain(s.addressee.iter())).map(|n| n.to_lowercase()).collect();
        if !names.is_empty() {
            for i in 0..r.ms.len() {
                if r.ms[i].kind == Kind::Proper && names.contains(&r.ms[i].text) {
                    r.ms[i].animacy = ANIM;
                    r.clusters[i].animacy = ANIM;
                }
            }
        }
        r
    }

    fn sent(&self, m: usize) -> &Sent {
        &self.doc.sents[self.ms[m].span.sent]
    }

    fn voice(&self, m: usize) -> Voice {
        let x = &self.ms[m];
        if x.quote > 0 {
            Voice::Quote(x.quote)
        } else if let Some(s) = &x.speaker {
            Voice::Speaker(s.clone())
        } else {
            Voice::Narrator
        }
    }

    /// Quote author: in a sentence with a quote — the subject of the speech verb outside the quotes (Lee et al. 2013, sec. 3.3.1;
    /// Baldwin 1995). Speech verbs are a closed hand-made list.
    fn find_quote_authors(&mut self) {
        const SAY: &[&str] = &["say", "ask", "reply", "tell", "answer", "shout", "whisper", "cry", "exclaim", "add", "respond", "yell", "call", "continue", "explain", "insist", "murmur", "mutter", "declare", "scream", "remark", "note", "write", "think", "wonder", "sigh", "laugh", "suggest", "announce"];
        for (si, s) in self.doc.sents.iter().enumerate() {
            let quotes: HashSet<u32> = s.quote.iter().copied().filter(|&q| q > 0).collect();
            if quotes.is_empty() {
                continue;
            }
            for v in 0..s.len() {
                if s.quote[v] != 0 || s.upos(v) != Some(UPos::VERB) || !SAY.contains(&s.lem(v).as_str()) {
                    continue;
                }
                let Some(subj) = s.kids_rel(v, Rel::Nsubj).next() else { continue };
                let Some(&sm) = self.by_head.get(&(si, subj)) else { continue };
                if self.ms[sm].is_pron() && self.ms[sm].person != P3 {
                    continue;
                }
                for &q in &quotes {
                    self.quote_author.entry(q).or_insert(sm);
                }
            }
        }
        // a quote continuing into the next sentence without a new author has the same author
        let mut last: Option<(u32, usize)> = None;
        for s in &self.doc.sents {
            for &q in s.quote.iter().filter(|&&q| q > 0) {
                if let Some(&a) = self.quote_author.get(&q) {
                    last = Some((q, a));
                } else if let Some((lq, a)) = last
                    && lq == q
                {
                    self.quote_author.insert(q, a);
                }
            }
        }
    }

    fn root(&self, m: usize) -> usize {
        self.of[m]
    }

    fn first_in_cluster(&self, m: usize) -> bool {
        self.clusters[self.of[m]].members.iter().all(|&x| x >= m)
    }

    /// Whether clusters can be merged (features and "I" voices).
    fn compatible(&self, a: usize, b: usize) -> Result<(), String> {
        let (ca, cb) = (&self.clusters[self.of[a]], &self.clusters[self.of[b]]);
        if self.of[a] == self.of[b] {
            return Err("already together".into());
        }
        if !agree(ca.number, cb.number) {
            return Err(format!("Number {}≠{}", feat_name("n", ca.number), feat_name("n", cb.number)));
        }
        if !agree(ca.gender, cb.gender) {
            return Err(format!("Gender {}≠{}", feat_name("g", ca.gender), feat_name("g", cb.gender)));
        }
        if !agree(ca.animacy, cb.animacy) {
            return Err(format!("Animacy {}≠{}", feat_name("a", ca.animacy), feat_name("a", cb.animacy)));
        }
        if !agree(ca.person, cb.person) {
            return Err(format!("Person {}≠{}", feat_name("p", ca.person), feat_name("p", cb.person)));
        }
        if !ca.voices_i.is_empty() && !cb.voices_i.is_empty() && ca.voices_i.is_disjoint(&cb.voices_i) {
            return Err("«I» of different speakers".into());
        }
        Ok(())
    }

    /// i-within-i: one mention inside another.
    fn nested(&self, a: usize, b: usize) -> bool {
        let (x, y) = (self.ms[a].span, self.ms[b].span);
        x.sent == y.sent && ((x.start <= y.start && y.end <= x.end) || (y.start <= x.start && x.end <= y.end))
    }

    fn merge(&mut self, sieve: Sieve, from: usize, to: usize, why: String) {
        let (ra, rb) = (self.of[from], self.of[to]);
        if ra == rb {
            return;
        }
        // is the antecedent the inner mention (relative pronoun)? For the check take the first visible mention of its cluster
        let mut to_vis = to;
        if self.ms[to].internal
            && let Some(&v) = self.clusters[rb].members.iter().find(|&&x| !self.ms[x].internal && x < from)
        {
            to_vis = v;
        }
        let moved = std::mem::take(&mut self.clusters[ra]);
        for &x in &moved.members {
            self.of[x] = rb;
        }
        let c = &mut self.clusters[rb];
        c.members.extend(moved.members);
        c.members.sort_unstable();
        c.number |= moved.number;
        c.gender |= moved.gender;
        c.animacy |= moved.animacy;
        c.person |= moved.person;
        c.voices_i.extend(moved.voices_i);
        self.links.push(Link { sieve, from, to: to_vis, why });
    }

    /// Head of the clause the word belongs to: the nearest predicate ancestor (verb, predicate with cop, or root).
    fn clause_head(s: &Sent, mut i: usize) -> usize {
        let mut guard = 0;
        loop {
            let pred = matches!(s.upos(i), Some(UPos::VERB)) || s.kids[i].iter().any(|&k| s.rel(k) == Rel::Cop) || s.parent(i).is_none();
            if pred && matches!(s.rel(i).base(), Rel::Root | Rel::Ccomp | Rel::Xcomp | Rel::Advcl | Rel::Acl | Rel::Csubj | Rel::Parataxis | Rel::Conj) {
                return i;
            }
            match s.parent(i) {
                Some(p) => i = p,
                None => return i,
            }
            guard += 1;
            if guard > s.len() {
                return i;
            }
        }
    }

    /// Antecedent candidates of mention `m` in Hobbs order. `window` — how many sentences back (None — all).
    /// In previous sentences: for pronouns — left to right (subject first), for nominals — right to left.
    fn candidates(&self, m: usize, window: Option<usize>) -> Vec<usize> {
        let mm = &self.ms[m];
        let si = mm.span.sent;
        let lo = window.map_or(0, |w| si.saturating_sub(w));
        let mut by_sent: Vec<Vec<usize>> = vec![Vec::new(); si - lo + 1];
        for c in (0..m).rev() {
            let x = &self.ms[c];
            if x.span.sent < lo {
                break;
            }
            if x.coord && mm.is_pron() && mm.number == SG {
                continue;
            }
            if self.nested(c, m) || x.span.start >= mm.span.start && x.span.sent == si {
                continue;
            }
            by_sent[si - x.span.sent].push(c);
        }
        let s = &self.doc.sents[si];
        let own = Self::clause_head(s, s.parent(mm.head).unwrap_or(mm.head));
        let mut out = Vec::new();
        for (k, mut v) in by_sent.into_iter().enumerate() {
            if k == 0 {
                v.sort_by_key(|&c| {
                    let x = &self.ms[c];
                    (!s.dominates(own, x.head), x.depth, x.span.start, std::cmp::Reverse(x.span.end))
                });
            } else if mm.is_pron() {
                v.sort_by_key(|&c| (self.ms[c].depth, self.ms[c].span.start, std::cmp::Reverse(self.ms[c].span.end)));
            } else {
                v.sort_by_key(|&c| (self.ms[c].depth, std::cmp::Reverse(self.ms[c].span.start), std::cmp::Reverse(self.ms[c].span.end)));
            }
            out.extend(v);
        }
        out
    }

    fn describe(&self, m: usize) -> String {
        let x = &self.ms[m];
        format!("«{}» ({}:{})", self.sent(m).text(x.span.start, x.span.end), self.sent(m).id, x.head + 1)
    }

    pub fn run(&mut self) {
        let sieves = self.cfg.sieves.clone();
        for s in sieves {
            match s {
                Sieve::Speaker => self.speaker(),
                Sieve::Exact => self.string_match(Sieve::Exact),
                Sieve::Relaxed => self.string_match(Sieve::Relaxed),
                Sieve::Appos => self.appos(),
                Sieve::Pred => self.pred(),
                Sieve::Relpron => self.relpron(),
                Sieve::Acronym => self.acronym(),
                Sieve::Reflexive => self.reflexive(),
                Sieve::StrictA | Sieve::StrictB | Sieve::StrictC | Sieve::ProperHead | Sieve::RelaxedHead => self.head_match(s),
                Sieve::Pronoun => self.pronoun(),
                Sieve::Nearest => self.nearest(),
            }
        }
    }

    // ── sieves ──────────────────────────────────────────────────────────────────────────────────

    /// Speaker: "I" of the same voice is one person; "I" in a quote is its author; "we" and "you" of the same voice;
    /// "you" in a conversation is the addressee's previous "I" (# addressee).
    fn speaker(&mut self) {
        let mut last_i: HashMap<Voice, usize> = HashMap::new();
        let mut last_we: HashMap<Voice, usize> = HashMap::new();
        let mut last_you: HashMap<(Voice, Option<String>), usize> = HashMap::new();
        for m in 0..self.ms.len() {
            let x = &self.ms[m];
            if !x.is_ppr() || x.person == P3 || x.person == 0 {
                continue;
            }
            let (person, number, hl) = (x.person, x.number, x.head_low.clone());
            let v = self.voice(m);
            let addressee = self.sent(m).addressee.clone();
            if person == P1 && number == SG {
                if let Voice::Quote(q) = v
                    && let Some(&a) = self.quote_author.get(&q)
                    && self.of[a] != self.of[m]
                {
                    let why = format!("«{}» in quote #{q}; quote author — {} (subject of the speech verb)", hl, self.describe(a));
                    self.merge(Sieve::Speaker, m, a, why);
                }
                if let Some(&p) = last_i.get(&v)
                    && self.of[p] != self.of[m]
                {
                    let why = format!("«{}» and previous «{}» — same voice ({v:?})", hl, self.ms[p].head_low);
                    self.merge(Sieve::Speaker, m, p, why);
                }
                last_i.insert(v, m);
            } else if person == P1 {
                if let Some(&p) = last_we.get(&v)
                    && self.of[p] != self.of[m]
                {
                    let why = format!("«{}» and previous «{}» — same voice ({v:?})", hl, self.ms[p].head_low);
                    self.merge(Sieve::Speaker, m, p, why);
                }
                last_we.insert(v, m);
            } else {
                // you: in a conversation with an addressee — their "I"
                // person differs here on purpose (you ~ I) — features are not checked, as for the quote author
                if let (Voice::Speaker(_), Some(ad)) = (&v, &addressee)
                    && let Some(&p) = last_i.get(&Voice::Speaker(ad.clone()))
                    && self.of[p] != self.of[m]
                {
                    let why = format!("«{}» to addressee {ad}; their previous «{}»", hl, self.ms[p].head_low);
                    self.merge(Sieve::Speaker, m, p, why);
                }
                let key = (v.clone(), addressee);
                if let Some(&p) = last_you.get(&key)
                    && self.of[p] != self.of[m]
                    && self.compatible(m, p).is_ok()
                {
                    let why = format!("«{}» and previous «{}» — same voice and addressee ({:?})", hl, self.ms[p].head_low, key.0);
                    self.merge(Sieve::Speaker, m, p, why);
                }
                last_you.insert(key, m);
            }
        }
    }

    /// Exact match (all nominals, even indefinite) and relaxed match up to the head (first in cluster, not indefinite).
    fn string_match(&mut self, sieve: Sieve) {
        for m in 0..self.ms.len() {
            let x = &self.ms[m];
            if !matches!(x.kind, Kind::Proper | Kind::Nominal) || !self.first_in_cluster(m) {
                continue;
            }
            if sieve == Sieve::Relaxed && (x.indefinite || x.pre_head == x.text) {
                continue;
            }
            if sieve == Sieve::Exact && x.indefinite && x.kind != Kind::Proper && std::env::var("COREF_X").is_ok_and(|v| v.contains("exact-noindef")) {
                continue;
            }
            let key = if sieve == Sieve::Exact { x.text.clone() } else { x.pre_head.clone() };
            for c in self.candidates(m, None) {
                let y = &self.ms[c];
                if !matches!(y.kind, Kind::Proper | Kind::Nominal) {
                    continue;
                }
                if sieve == Sieve::Relaxed && (y.quantified || !(y.post_clausal_only && x.post_clausal_only) && !opt_x("relaxed-any")) {
                    continue;
                }
                let other = if sieve == Sieve::Exact { &y.text } else { &y.pre_head };
                if *other == key && self.compatible(m, c).is_ok() && !self.possessor_clash(m, c) {
                    let why = if sieve == Sieve::Exact { format!("same text «{key}»") } else { format!("same text up to the head «{key}»; {} ~ {}", self.describe(m), self.describe(c)) };
                    self.merge(sieve, m, c, why);
                    break;
                }
            }
        }
    }

    fn appos(&mut self) {
        for m in 0..self.ms.len() {
            let x = &self.ms[m];
            if x.coord || x.role != Rel::Appos {
                continue;
            }
            let s = self.sent(m);
            let Some(p) = s.parent(x.head) else { continue };
            let Some(&a) = self.by_head.get(&(x.span.sent, p)) else { continue };
            if self.ms[a].is_pron() || x.is_pron() {
                continue;
            }
            if self.compatible(m, a).is_ok() {
                let why = format!("{} —appos→ {}", self.describe(a), self.describe(m));
                self.merge(Sieve::Appos, m, a, why);
            }
        }
    }

    /// Predicative noun: the head has cop and nsubj; no negation.
    fn pred(&mut self) {
        for m in 0..self.ms.len() {
            let x = &self.ms[m];
            if x.coord || x.kind == Kind::Pronoun {
                continue;
            }
            let s = self.sent(m);
            let h = x.head;
            let Some(cop) = s.kids_rel(h, Rel::Cop).next() else { continue };
            if s.kids[h].iter().any(|&k| matches!(s.low(k).as_str(), "not" | "n't" | "never") || s.has(k, Feat::PolarityNeg)) {
                continue;
            }
            let Some(subj) = s.kids_rel(h, Rel::Nsubj).next() else { continue };
            let Some(&a) = self.by_head.get(&(x.span.sent, subj)) else { continue };
            let subj_it = self.ms[a].pron == Some(Pron::Demonstrative) || self.ms[a].head_low == "it";
            if matches!(self.ms[a].pron, Some(Pron::Interrogative)) || subj_it && std::env::var("COREF_X").is_ok_and(|v| v.contains("pred-noit")) {
                continue;
            }
            if self.compatible(m, a).is_ok() {
                let why = format!("{} —cop «{}»— {}", self.describe(a), s.toks[cop].form, self.describe(m));
                self.merge(Sieve::Pred, m, a, why);
            }
        }
    }

    /// Relative pronoun → the noun the relative clause (acl:relcl) is attached to.
    fn relpron(&mut self) {
        for m in 0..self.ms.len() {
            let x = &self.ms[m];
            if x.pron != Some(Pron::Relative) {
                continue;
            }
            let s = self.sent(m);
            let mut i = x.head;
            let mut noun = None;
            let mut guard = 0;
            while let Some(p) = s.parent(i) {
                if s.rel(i).base() == Rel::Acl {
                    noun = Some(p);
                    break;
                }
                i = p;
                guard += 1;
                if guard > s.len() {
                    break;
                }
            }
            let Some(n) = noun else { continue };
            let Some(&a) = self.by_head.get(&(x.span.sent, n)) else { continue };
            if self.compatible(m, a).is_ok() {
                let why = format!("relative «{}» in the clause attached to {}", x.head_low, self.describe(a));
                self.merge(Sieve::Relpron, m, a, why);
            }
        }
    }

    fn acronym(&mut self) {
        for m in 0..self.ms.len() {
            let x = &self.ms[m];
            if x.kind != Kind::Proper || !self.first_in_cluster(m) {
                continue;
            }
            let s = self.sent(m);
            let form = &s.toks[x.head].form;
            if form.len() < 2 || !form.chars().all(|c| c.is_ascii_uppercase()) {
                continue;
            }
            for c in 0..m {
                let y = &self.ms[c];
                if y.kind != Kind::Proper || y.span.end == y.span.start {
                    continue;
                }
                let t = self.sent(c);
                let initials: String = (y.span.start..=y.span.end).filter_map(|i| t.toks[i].form.chars().next().filter(|ch| ch.is_uppercase())).collect();
                if initials == *form && self.compatible(m, c).is_ok() {
                    let why = format!("«{form}» — capital letters of {}", self.describe(c));
                    self.merge(Sieve::Acronym, m, c, why);
                    break;
                }
            }
        }
    }

    /// Reflexive pronoun → subject of its predicate (principle A of binding theory).
    fn reflexive(&mut self) {
        for m in 0..self.ms.len() {
            let x = &self.ms[m];
            if x.pron != Some(Pron::Reflexive) || x.person != P3 {
                continue;
            }
            let s = self.sent(m);
            let mut i = x.head;
            let mut guard = 0;
            let mut found = None;
            while let Some(p) = s.parent(i) {
                if let Some(subj) = s.kids_rel(p, Rel::Nsubj).next()
                    && subj != x.head
                {
                    found = Some((p, subj));
                    break;
                }
                i = p;
                guard += 1;
                if guard > s.len() {
                    break;
                }
            }
            let Some((p, subj)) = found else { continue };
            let Some(&a) = self.by_head.get(&(x.span.sent, subj)) else { continue };
            if self.compatible(m, a).is_ok() {
                let why = format!("«{}» → subject {} of predicate «{}»", x.head_low, self.describe(a), s.toks[p].form);
                self.merge(Sieve::Reflexive, m, a, why);
            }
        }
    }

    fn cluster_words(&self, m: usize) -> HashSet<&str> {
        self.clusters[self.of[m]].members.iter().filter(|&&x| !self.ms[x].is_pron()).flat_map(|&x| self.ms[x].words.iter().map(String::as_str)).collect()
    }

    fn cluster_heads(&self, m: usize) -> HashSet<&str> {
        self.clusters[self.of[m]].members.iter().filter(|&&x| !self.ms[x].is_pron()).map(|&x| self.ms[x].head_low.as_str()).collect()
    }

    /// Different possessors: "my jeans" ≠ "your jeans" (different persons or different clusters).
    fn possessor_clash(&self, m: usize, c: usize) -> bool {
        let (x, y) = (&self.ms[m], &self.ms[c]);
        let (Some(px), Some(py)) = (x.possessor, y.possessor) else { return false };
        let (sx, sy) = (&self.doc.sents[x.span.sent], &self.doc.sents[y.span.sent]);
        let (Some(&mx), Some(&my)) = (self.by_head.get(&(x.span.sent, px)), self.by_head.get(&(y.span.sent, py))) else {
            return sx.low(px) != sy.low(py);
        };
        if self.of[mx] == self.of[my] {
            return false;
        }
        let (a, b) = (&self.ms[mx], &self.ms[my]);
        if a.is_ppr() && b.is_ppr() {
            return a.person != b.person || a.number != b.number || self.voice(mx) != self.voice(my) && a.person != P3;
        }
        sx.low(px) != sy.low(py)
    }

    /// Head-based sieves: strict_a/b/c, proper_head, relaxed_head.
    fn head_match(&mut self, sieve: Sieve) {
        for m in 0..self.ms.len() {
            let x = &self.ms[m];
            if !matches!(x.kind, Kind::Proper | Kind::Nominal) || !self.first_in_cluster(m) || x.indefinite {
                continue;
            }
            if matches!(sieve, Sieve::ProperHead | Sieve::RelaxedHead) && x.kind != Kind::Proper {
                continue;
            }
            let mwords: HashSet<String> = self.cluster_words(m).into_iter().map(str::to_string).collect();
            for c in self.candidates(m, None) {
                let y = &self.ms[c];
                if !matches!(y.kind, Kind::Proper | Kind::Nominal) || y.quantified && !opt_x("quant-ante") {
                    continue;
                }
                let x = &self.ms[m];
                let heads = self.cluster_heads(c);
                let cwords = self.cluster_words(c);
                let incl = mwords.iter().all(|w| cwords.contains(w.as_str()));
                let mods_ok = x.mods.iter().all(|w| y.mods.contains(w));
                let hk = x.head_low.as_str();
                if self.possessor_clash(m, c) {
                    continue;
                }
                let ok = match sieve {
                    Sieve::StrictA => heads.contains(hk) && incl && mods_ok,
                    Sieve::StrictB => heads.contains(hk) && incl,
                    Sieve::StrictC => heads.contains(hk) && mods_ok,
                    Sieve::ProperHead => y.kind == Kind::Proper && y.head_key == x.head_key && x.proper_mods.iter().all(|w| y.proper_mods.contains(w)),
                    Sieve::RelaxedHead => y.kind == Kind::Proper && cwords.contains(x.head_key.as_str()) && incl,
                    _ => false,
                };
                if ok && self.compatible(m, c).is_ok() {
                    let why = match sieve {
                        Sieve::StrictA => format!("head «{}» is in cluster; words ⊆ {:?}; modifiers {:?} ⊆ {:?}", x.head_low, sorted(&cwords), x.mods, y.mods),
                        Sieve::StrictB => format!("head «{}» is in cluster; words {:?} ⊆ cluster", x.head_low, sorted_s(&mwords)),
                        Sieve::StrictC => format!("head «{}» is in cluster; modifiers {:?} ⊆ {:?}", x.head_low, x.mods, y.mods),
                        Sieve::ProperHead => format!("same proper head «{}»; proper names/numbers in modifiers: {:?} ⊆ {:?}", x.head_key, x.proper_mods, y.proper_mods),
                        _ => format!("proper head «{}» is among the words of cluster {}; words ⊆", x.head_key, self.describe(c)),
                    };
                    let why = format!("{why}; {} ~ {}", self.describe(m), self.describe(c));
                    self.merge(sieve, m, c, why);
                    break;
                }
            }
        }
    }

    /// Principle B: a personal pronoun is not bound in its own clause — no mention of the candidate's cluster may
    /// be its co-argument (subject, object, oblique of the same predicate).
    fn principle_b(&self, m: usize, c: usize) -> bool {
        let x = &self.ms[m];
        if x.pron != Some(Pron::Personal) {
            return true;
        }
        let s = self.sent(m);
        let core = |r: Rel| matches!(r.base(), Rel::Nsubj | Rel::Obj | Rel::Iobj | Rel::Obl);
        let px = s.parent(x.head);
        if px.is_none() || !core(s.rel(x.head)) {
            return true;
        }
        !self.clusters[self.of[c]].members.iter().any(|&y| {
            let y = &self.ms[y];
            y.span.sent == x.span.sent && !y.coord && s.parent(y.head) == px && core(s.rel(y.head)) && y.head != x.head
        })
    }

    fn is_animate_cluster(&self, c: usize) -> bool {
        self.clusters[self.of[c]].animacy & ANIM != 0
    }

    /// 3rd-person pronouns: cluster agreement, principle B, Hobbs order, sentence window.
    fn pronoun(&mut self) {
        for m in 0..self.ms.len() {
            let x = &self.ms[m];
            let dem = x.pron == Some(Pron::Demonstrative);
            if !(x.is_ppr() || dem && self.cfg.demonstratives) || x.person != P3 || !self.first_in_cluster(m) {
                continue;
            }
            if dem && x.role == Rel::Det {
                continue;
            }
            let x_env = std::env::var("COREF_X").unwrap_or_default();
            let mut win = x_env.split(',').find_map(|t| t.strip_prefix("win=")).and_then(|v| v.parse().ok()).unwrap_or(self.cfg.pron_window);
            if let Some(w) = x_env.split(',').find_map(|t| t.strip_prefix("itwin=")).and_then(|v| v.parse::<usize>().ok())
                && matches!(self.ms[m].head_low.as_str(), "it" | "its" | "itself")
            {
                win = w;
            }
            let mut cands = self.candidates(m, Some(win));
            if x_env.contains("pron-recency") {
                cands.sort_by_key(|&c| (std::cmp::Reverse(self.ms[c].span.sent), std::cmp::Reverse(self.ms[c].span.start)));
            }
            let personal_anim = x.animacy & ANIM != 0 && x.gender & (MASC | FEM) != 0;
            let mut rejected: Vec<String> = Vec::new();
            let mut pick: Option<(usize, usize, bool)> = None;
            let passes: &[bool] = if self.cfg.prefer_animate && personal_anim && !x_env.contains("no-anim-pref") { &[true, false] } else { &[false] };
            'outer: for &need_anim in passes {
                for (rank, &c) in cands.iter().enumerate() {
                    let y = &self.ms[c];
                    if matches!(y.pron, Some(Pron::Interrogative)) {
                        continue;
                    }
                    // quantified pronouns (everyone, nobody, anything…) are not antecedents (bound variable)
                    if (y.pron == Some(Pron::Indefinite) || y.quantified) && !opt_x("indef-ante") {
                        continue;
                    }
                    if need_anim && !self.is_animate_cluster(c) {
                        continue;
                    }
                    if !self.principle_b(m, c) {
                        if rejected.len() < 3 {
                            rejected.push(format!("{}: principle B", self.describe(c)));
                        }
                        continue;
                    }
                    match self.compatible(m, c) {
                        Ok(()) => {
                            pick = Some((c, rank, need_anim));
                            break 'outer;
                        }
                        Err(e) => {
                            if rejected.len() < 3 && e != "already together" {
                                rejected.push(format!("{}: {e}", self.describe(c)));
                            }
                        }
                    }
                }
            }
            if let Some((c, rank, anim)) = pick {
                let x = &self.ms[m];
                let cl = &self.clusters[self.of[c]];
                let d = x.span.sent - self.ms[c].span.sent;
                let mut why = format!(
                    "«{}» → {}: candidate #{} in Hobbs order, {} sent. back; cluster Number={} Gender={} Animacy={}",
                    x.head_low,
                    self.describe(c),
                    rank + 1,
                    d,
                    feat_name("n", cl.number),
                    feat_name("g", cl.gender),
                    feat_name("a", cl.animacy)
                );
                if anim {
                    why.push_str("; animacy preference");
                }
                if !rejected.is_empty() {
                    why.push_str(&format!("; rejected: {}", rejected.join("; ")));
                }
                self.merge(Sieve::Pronoun, m, c, why);
            }
        }
    }

    /// Baseline: pronoun (personal, possessive, reflexive) → the nearest preceding mention with agreeing
    /// mention features (no clusters or syntax).
    fn nearest(&mut self) {
        for m in 0..self.ms.len() {
            let x = &self.ms[m];
            if !x.is_ppr() {
                continue;
            }
            for c in (0..m).rev() {
                let y = &self.ms[c];
                if self.nested(c, m) || y.internal {
                    continue;
                }
                if agree(x.number, y.number) && agree(x.gender, y.gender) && agree(x.animacy, y.animacy) && agree(x.person, y.person) {
                    let why = format!("nearest agreeing mention {}", self.describe(c));
                    self.merge(Sieve::Nearest, m, c, why);
                    break;
                }
            }
        }
    }

    // ── output ──────────────────────────────────────────────────────────────────────────────────

    /// Clusters as lists of mentions in text order (clusters ordered by first mention).
    pub fn clusters(&self) -> Vec<Vec<usize>> {
        let mut out: Vec<Vec<usize>> = self.clusters.iter().filter(|c| !c.members.is_empty()).map(|c| c.members.clone()).collect();
        out.sort_by_key(|c| c[0]);
        out
    }

    /// Who attached the mention to a cluster (the link where it is `from`).
    pub fn link_of(&self) -> HashMap<usize, &Link> {
        self.links.iter().map(|l| (l.from, l)).collect()
    }

    /// Cluster features of a mention — for explanations.
    pub fn cluster_feats(&self, m: usize) -> String {
        let c = &self.clusters[self.of[m]];
        format!("Number={} Gender={} Animacy={} Person={}", feat_name("n", c.number), feat_name("g", c.gender), feat_name("a", c.animacy), feat_name("p", c.person))
    }

    pub fn cluster_id(&self, m: usize) -> usize {
        self.root(m)
    }
}

fn sorted(s: &HashSet<&str>) -> Vec<String> {
    let mut v: Vec<String> = s.iter().map(|x| x.to_string()).collect();
    v.sort();
    v
}

fn sorted_s(s: &HashSet<String>) -> Vec<String> {
    let mut v: Vec<String> = s.iter().cloned().collect();
    v.sort();
    v
}
