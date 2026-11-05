//! Full clauses from a parsed sentence, restated in the describer's perspective.
//!
//! Where `world::read::skeleton` keeps only subject, verb and heads (it drops determiners, most
//! prepositions and auxiliaries), this realizer keeps whole clauses from Mova's UD tree and changes
//! them only by operations that keep their meaning:
//!
//! - clause splitting: coordinated verbs (`conj`), side clauses (`parataxis`) and subject relative
//!   clauses (`acl:relcl` with a relative pronoun subject) become sentences of their own; a clause
//!   without its own subject takes the subject (and, for participles, the auxiliaries) of the clause
//!   it was coordinated with, and a relative clause takes its antecedent in place of the pronoun;
//! - perspective: the authors' «we / our / us» becomes «the authors / their / them»;
//! - parenthesized and bracketed spans (citations, examples, numbering) are left out;
//! - a stretch longer than the verbatim cap that the clause shares word for word with its source
//!   is broken by leaving out optional modifiers inside it (adverbs, obliques, nominal modifiers,
//!   appositions, adjectival clauses); a clause that cannot be brought under the cap is not used.
//!
//! Every word of a realized clause is a word of its source sentence, except the perspective words
//! (`PERSPECTIVE`). Nothing is invented and nothing is padded.
use en::gram::{Rel, Tag, UPos};
use en::tok::Tok;
use world::tree::Tree;

/// Words a clause may contain beyond its source sentence: the describer's perspective.
pub const PERSPECTIVE: [&str; 6] = ["the", "authors", "their", "them", "theirs", "themselves"];

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Clause {
    pub text: String,
    /// `main`, `conj`, `parataxis` or `relcl`.
    pub kind: &'static str,
    /// Longest stretch of alphanumeric tokens shared word for word with the source sentence.
    pub verbatim: usize,
    /// Modifier subtrees left out to respect the verbatim cap.
    pub dropped: usize,
    /// The source tokens the clause restates.
    #[serde(skip)]
    pub src: Vec<usize>,
}

/// One written token: its form and the source token it is (None: a perspective word or an
/// inserted comma).
#[derive(Clone, Debug)]
struct W {
    form: String,
    src: Option<usize>,
}

/// A piece of a clause arrangement: a source token, or a comma that closes a fronted modifier.
#[derive(Clone, Copy, Debug, PartialEq)]
enum P {
    T(usize),
    Comma,
}

fn alnum(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
}

fn has_subject(t: &Tree, h: usize) -> bool {
    t.kids(h).iter().any(|&c| matches!(t.base(c), Rel::Nsubj | Rel::Csubj | Rel::Expl))
}
fn has_cop(t: &Tree, h: usize) -> bool {
    t.kids(h).iter().any(|&c| t.w[c].rel == Rel::Cop)
}
fn predicate(t: &Tree, h: usize) -> bool {
    t.w[h].upos == UPos::VERB || has_cop(t, h)
}
fn finite(t: &Tree, i: usize) -> bool {
    matches!(t.w[i].tag, Tag::VBD | Tag::VBZ | Tag::VBP | Tag::MD)
}

/// The clause heads of a tree: the root and every node that starts a clause of its own.
fn heads(t: &Tree) -> Vec<(usize, &'static str)> {
    let Some(root) = t.root() else { return vec![] };
    // «Note that X»: the imperative only points at X; X is the main clause.
    let root = noted(t, root).unwrap_or(root);
    let mut out = vec![(root, "main")];
    // A coordinated clause counts only when what it is coordinated with is a clause head itself
    // (not an infinitive or a complement clause): repeat until nothing changes.
    loop {
        let before = out.len();
        for i in 0..t.w.len() {
            if i == root || !predicate(t, i) || out.iter().any(|h| h.0 == i) {
                continue;
            }
            let kind = match t.w[i].rel {
                Rel::Conj if t.head(i).is_some_and(|h| out.iter().any(|x| x.0 == h)) => "conj",
                // «X not only does A, but can also do B»: the parse often hangs the «but» half
                // as an adverbial clause; with its own coordinator and no subordinator it is the
                // second half of the coordination.
                Rel::Advcl if correlate(t, i) && t.head(i).is_some_and(|h| out.iter().any(|x| x.0 == h)) => "conj",
                Rel::Parataxis if has_subject(t, i) => "parataxis",
                Rel::AclRelcl if relative_subject(t, i).is_some() && t.head(i).is_some_and(|a| matches!(t.w[a].upos, UPos::NOUN | UPos::PROPN)) => "relcl",
                _ => continue,
            };
            out.push((i, kind));
        }
        if out.len() == before {
            break;
        }
    }
    out.sort();
    out
}

/// The complement of a sentence-initial imperative «Note that …» (no subject of its own): the
/// dependent of «note» that is a clause of its own (a predicate with a subject) opened by «that».
/// The relation label is not read: the 0.4 parse gives `ccomp`, the 0.5 model (2483516a) `obl`
/// for «Note that this is a representative example …»; what makes it the complement is the clause
/// shape and its marker.
fn noted(t: &Tree, root: usize) -> Option<usize> {
    if t.lemma(root) != "note" || t.w[root].tag != Tag::VB || has_subject(t, root) || root != 0 {
        return None;
    }
    let that = |c: usize| t.kids(c).iter().any(|&m| t.w[m].rel == Rel::Mark && t.form(m).eq_ignore_ascii_case("that"));
    t.kids(root).iter().copied().find(|&c| that(c) && (t.base(c) == Rel::Ccomp || (predicate(t, c) && has_subject(t, c))))
}

/// The relative pronoun that is the subject of a relative clause («which», «that», «who»).
fn relative_subject(t: &Tree, h: usize) -> Option<usize> {
    t.kids(h).iter().copied().find(|&c| matches!(t.base(c), Rel::Nsubj) && (t.w[c].tag.is_wh() || t.lemma(c) == "that"))
}

/// An adverbial clause that is really the second half of a coordination: it has its own
/// coordinator («but», «and») and no subordinator or subject of its own.
fn correlate(t: &Tree, i: usize) -> bool {
    t.base(i) == Rel::Advcl
        && t.kids(i).iter().any(|&c| t.w[c].rel == Rel::Cc && matches!(t.lemma(c), "but" | "and"))
        && !t.kids(i).iter().any(|&c| t.w[c].rel == Rel::Mark)
        && !has_subject(t, i)
}

/// «not only» of a clause head: the first half of a correlative. Its second half («but also …»)
/// becomes a sentence of its own, so the pair is left out of the first («This not only facilitates
/// X, but can also …» -> «This facilitates X.» + «This can also …»), never «only» alone (which
/// would leave «This not facilitates X»).
fn not_only(t: &Tree, h: usize) -> Vec<usize> {
    let kids = t.kids(h);
    for &k in kids {
        if t.lemma(k) == "not" && kids.contains(&(k + 1)) && t.form(k + 1).eq_ignore_ascii_case("only") {
            return vec![k, k + 1];
        }
    }
    vec![]
}

/// The clause head a coordinated clause shares its subject with: up the chain of `conj` (and of
/// a correlative's second half, see `correlate`).
fn governor(t: &Tree, mut h: usize) -> usize {
    for _ in 0..t.w.len() {
        if t.w[h].rel != Rel::Conj && !correlate(t, h) {
            break;
        }
        match t.head(h) {
            Some(p) => h = p,
            None => break,
        }
    }
    h
}

/// Words of `ix` in order, without bracketed spans.
fn unbracket(t: &Tree, ix: &[usize]) -> Vec<usize> {
    let mut depth = 0i32;
    let mut out = vec![];
    for &i in ix {
        match t.form(i) {
            "(" | "[" | "-lrb-" | "-lsb-" => depth += 1,
            ")" | "]" | "-rrb-" | "-rsb-" => depth = (depth - 1).max(0),
            _ if depth == 0 => out.push(i),
            _ => {}
        }
    }
    out
}

fn perspective(form: &str) -> Option<&'static str> {
    Some(match form {
        "we" | "We" | "WE" => "the authors",
        "our" | "Our" => "their",
        "us" => "them",
        "ours" => "theirs",
        "ourselves" => "themselves",
        _ => return None,
    })
}

fn punct_like(s: &str) -> bool {
    !s.chars().any(char::is_alphanumeric)
}

/// Join written tokens with the source's own spacing where they were adjacent.
fn join(ws: &[W], toks: &[Tok]) -> String {
    let mut out = String::new();
    for (k, w) in ws.iter().enumerate() {
        if k > 0 {
            let prev = &ws[k - 1];
            let adjacent = matches!((prev.src, w.src), (Some(a), Some(b)) if b == a + 1);
            let space = if adjacent {
                toks.get(prev.src.unwrap_or(0)).map(|t| t.space_after).unwrap_or(true)
            } else {
                !(matches!(w.form.as_str(), "," | "." | ";" | ":" | ")" | "%" | "'s" | "n't" | "'") || prev.form == "(")
            };
            if space {
                out.push(' ');
            }
        }
        out.push_str(&w.form);
    }
    out
}

/// Longest run of written alphanumeric tokens that are the source's own, consecutive there.
fn verbatim(ws: &[W], toks: &[Tok]) -> usize {
    let mut pos = vec![0usize; toks.len() + 1];
    for (i, t) in toks.iter().enumerate() {
        pos[i + 1] = pos[i] + alnum(&t.form).len();
    }
    let mut best = 0;
    let mut run = 0;
    let mut next: Option<usize> = None;
    for w in ws {
        let n = alnum(&w.form).len();
        if n == 0 {
            continue;
        }
        match w.src {
            Some(i) if next == Some(pos[i]) => run += n,
            Some(_) => run = n,
            None => run = 0,
        }
        next = w.src.map(|i| pos[i + 1]);
        best = best.max(run);
    }
    best
}

/// Written tokens of an arrangement: perspective applied, stray punctuation and dangling words
/// at the edges trimmed, doubled commas collapsed.
fn build(t: &Tree, toks: &[Tok], h: usize, ps: &[P]) -> Vec<W> {
    let mut ws: Vec<W> = vec![];
    for p in ps {
        match *p {
            P::Comma => ws.push(W { form: ",".into(), src: None }),
            P::T(i) => {
                let f = toks.get(i).map(|x| x.form.clone()).unwrap_or_else(|| t.form(i).to_string());
                match perspective(&f) {
                    Some(p) => ws.extend(p.split(' ').map(|x| W { form: x.into(), src: None })),
                    None => ws.push(W { form: f, src: Some(i) }),
                }
            }
        }
    }
    // The clause's own coordinating conjunction is left out wherever it stands (the clause is a
    // sentence of its own now); its own marker does not open it.
    let own_cc = |i: usize| t.w[i].rel == Rel::Cc && t.head(i) == Some(h) && (t.w[h].rel == Rel::Conj || correlate(t, h));
    // «Note that X»: X's «that» goes with the imperative, wherever moving put it.
    if t.root().and_then(|r| noted(t, r)) == Some(h) {
        ws.retain(|w| !w.src.is_some_and(|i| t.w[i].rel == Rel::Mark && t.head(i) == Some(h) && t.form(i).eq_ignore_ascii_case("that")));
    }
    // The comma before that conjunction goes with it («…, and achieve» -> «… achieve»).
    ws.retain(|w| !w.src.is_some_and(|i| own_cc(i) || (t.w[i].upos == UPos::PUNCT && i + 1 < t.w.len() && own_cc(i + 1))));
    let own = |w: &W| w.src.is_some_and(|i| matches!(t.w[i].rel, Rel::Cc | Rel::Mark) && t.head(i) == Some(h));
    while ws.first().is_some_and(|w| punct_like(&w.form) || own(w)) {
        ws.remove(0);
    }
    while ws.last().is_some_and(|w| punct_like(&w.form) || w.src.is_some_and(|i| matches!(t.w[i].upos, UPos::CCONJ | UPos::SCONJ | UPos::ADP | UPos::DET))) {
        ws.pop();
    }
    let mut k = 1;
    while k < ws.len() {
        if punct_like(&ws[k].form) && ws[k].form == ws[k - 1].form {
            ws.remove(k);
        } else {
            k += 1;
        }
    }
    // A capitalized sentence opener that is no longer first takes lower case (not names).
    for (k, w) in ws.iter_mut().enumerate() {
        if k > 0 && w.src == Some(0) && !matches!(t.w[0].upos, UPos::PROPN) && w.form.chars().skip(1).all(|c| !c.is_uppercase()) {
            w.form = w.form.to_lowercase();
        }
    }
    ws
}

/// How readily a modifier may be moved or left out (lower first): adverbs, adverbial clauses,
/// obliques, appositions, adjectival clauses, phrasal adjectives, nominal modifiers.
fn optional(t: &Tree, i: usize) -> Option<u8> {
    // Negation is never left out.
    if matches!(t.lemma(i), "not" | "n't" | "never" | "no" | "neither" | "nor") || t.form(i) == "n't" {
        return None;
    }
    // «only» after «not» is half of a correlative (see `not_only`), never left out alone.
    if t.form(i).eq_ignore_ascii_case("only") && i > 0 && t.lemma(i - 1) == "not" {
        return None;
    }
    Some(match t.base(i) {
        Rel::Advmod => 0,
        Rel::Advcl => 1,
        Rel::Obl => 2,
        Rel::Appos => 3,
        Rel::Acl => 4,
        Rel::Amod if !t.kids(i).is_empty() => 5,
        Rel::Nmod => 6,
        _ => return None,
    })
}

/// Movable modifiers: adverbial clauses and obliques of the clause head (fronted or put last).
fn movable(t: &Tree, h: usize, i: usize) -> bool {
    if t.head(i) != Some(h) {
        return false;
    }
    match t.base(i) {
        Rel::Advcl => true,
        // An oblique right after a verb without an object is likely its argument («relies on
        // X»); a «by» agent stays with its passive.
        Rel::Obl => {
            let agent = t.w[i].rel == Rel::OblAgent || t.kids_of(i, Rel::Case).first().is_some_and(|&c| t.form(c) == "by");
            let start = t.sub(i, &[]).first().copied().unwrap_or(i);
            let argument = !t.kids(h).iter().any(|&c| matches!(t.base(c), Rel::Obj | Rel::Xcomp | Rel::Ccomp)) && start > h && start <= h + 2;
            !agent && !argument
        }
        _ => false,
    }
}

/// Move the subtree `sub` of a modifier: to the end when it opens the clause, else to the front
/// (followed by a comma). Commas that only framed it go with it.
fn relocate(t: &Tree, ps: &[P], sub: &[usize]) -> Vec<P> {
    let inside = |p: &P| matches!(p, P::T(i) if sub.contains(i));
    let Some(first) = ps.iter().position(inside) else { return ps.to_vec() };
    let last = ps.iter().rposition(inside).unwrap_or(first);
    let block: Vec<P> = ps[first..=last].iter().copied().filter(|p| !matches!(p, P::Comma)).collect();
    let mut rest: Vec<P> = ps[..first].to_vec();
    let mut after = last + 1;
    // A comma that closed the block when it was fronted is dropped with it.
    while after < ps.len() && matches!(ps[after], P::Comma) {
        after += 1;
    }
    rest.extend_from_slice(&ps[after..]);
    if first == 0 {
        // Before the sentence's closing punctuation, not after it.
        let mut tail = vec![];
        while rest.last().is_some_and(|p| matches!(p, P::T(i) if punct_like(t.form(*i)))) {
            tail.insert(0, rest.pop().unwrap_or(P::Comma));
        }
        if !rest.is_empty() {
            rest.push(P::Comma);
        }
        rest.extend(block);
        rest.extend(tail);
        rest
    } else {
        let mut out = block;
        out.push(P::Comma);
        out.extend(rest);
        out
    }
}

/// «X was made by Y» -> «Y made X», only where the past form equals the participle (so every
/// word stays the source's own) and the agent is an explicit «by» phrase.
fn activize(t: &Tree, toks: &[Tok], h: usize, ps: &[P], cut: &[usize]) -> Option<Vec<P>> {
    let auxpass = t.kids(h).iter().copied().find(|&c| t.w[c].rel == Rel::AuxPass)?;
    if !matches!(t.lemma(auxpass), "be") || !matches!(t.form(auxpass), "was" | "were") || t.kids(h).iter().any(|&c| t.w[c].rel == Rel::Aux) {
        return None;
    }
    let subj = t.kids(h).iter().copied().find(|&c| t.w[c].rel == Rel::NsubjPass)?;
    let agent = t.kids(h).iter().copied().find(|&c| t.w[c].rel == Rel::OblAgent || (t.base(c) == Rel::Obl && t.kids_of(c, Rel::Case).first().is_some_and(|&k| t.form(k) == "by")))?;
    let form = toks.get(h)?.form.to_lowercase();
    if mova_octopus::realize::past(t.lemma(h)) != form || !matches!(t.w[subj].upos, UPos::NOUN | UPos::PROPN) {
        return None;
    }
    let subj_sub = unbracket(t, &t.sub(subj, cut));
    let agent_all = unbracket(t, &t.sub(agent, cut));
    let agent_sub: Vec<usize> = agent_all.iter().copied().filter(|&i| !(t.w[i].rel == Rel::Case && t.head(i) == Some(agent)) && t.w[i].upos != UPos::PUNCT).collect();
    let has = |i: &usize| ps.contains(&P::T(*i));
    if !subj_sub.iter().all(has) || !agent_sub.iter().all(has) {
        return None;
    }
    let pos_subj = ps.iter().position(|p| matches!(p, P::T(i) if subj_sub.contains(i)))?;
    let mut out: Vec<P> = ps[..pos_subj].to_vec();
    out.extend(agent_sub.iter().map(|&i| P::T(i)));
    out.push(P::T(h));
    out.extend(subj_sub.iter().map(|&i| P::T(i)));
    for p in &ps[pos_subj..] {
        if let P::T(i) = p {
            if subj_sub.contains(i) || agent_all.contains(i) || *i == auxpass || *i == h {
                continue;
            }
        }
        out.push(*p);
    }
    Some(out)
}

fn debug(t: &Tree, toks: &[Tok], h: usize, why: &str) {
    if std::env::var_os("WHALE_CLAUSE_DEBUG").is_some() {
        eprintln!("clause {} [{}]: {why}\n  {}", toks.iter().map(|x| x.form.as_str()).collect::<Vec<_>>().join(" "), t.form(h), t.show());
    }
}

/// Realize the clauses of a parsed sentence (`toks` are its tokens, one per tree word).
pub fn clauses(t: &Tree, toks: &[Tok], cap: usize) -> Vec<Clause> {
    if t.w.len() != toks.len() || t.root().is_none() {
        return vec![];
    }
    let hs = heads(t);
    let cut: Vec<usize> = hs.iter().map(|h| h.0).collect();
    let mut out = vec![];
    let mut realized: Vec<(usize, Vec<usize>)> = vec![];
    let mut sub_main: Option<usize> = None;
    for &(h, kind) in &hs {
        if !predicate(t, h) {
            debug(t, toks, h, "not a predicate");
            continue;
        }
        // A main clause that is itself subordinate («As the distance aligns tokens, …» cut off
        // from its main clause) is a fragment.
        if kind == "main" && t.root().and_then(|r| noted(t, r)) != Some(h) && t.kids(h).iter().any(|&c| t.w[c].rel == Rel::Mark && t.form(c) != "to") {
            debug(t, toks, h, "subordinate main clause");
            sub_main = Some(h);
            continue;
        }
        let ix = unbracket(t, &t.sub(h, &cut));
        let mut prefix: Vec<usize> = vec![];
        let correlative = not_only(t, h);
        let mut ix: Vec<usize> = ix.into_iter().filter(|i| !correlative.contains(i)).collect();
        if !has_subject(t, h) {
            match kind {
                "conj" => {
                    let g = governor(t, h);
                    let Some(s) = t.kids(g).iter().copied().find(|&c| matches!(t.base(c), Rel::Nsubj)) else { continue };
                    prefix = unbracket(t, &t.sub(s, &cut));
                    if prefix.len() > 10 {
                        continue;
                    }
                    // A participle coordinated under shared auxiliaries keeps them: «are extracted and classified».
                    if matches!(t.w[h].tag, Tag::VBN | Tag::VBG) && !t.kids(h).iter().any(|&c| t.base(c) == Rel::Aux) {
                        prefix.extend(t.kids(g).iter().copied().filter(|&c| t.base(c) == Rel::Aux));
                    }
                }
                "relcl" | "main" => {}
                _ => continue,
            }
        }
        if kind == "relcl" {
            let (Some(pron), Some(ante)) = (relative_subject(t, h), t.head(h)) else { continue };
            ix.retain(|&i| i != pron);
            while ix.first().is_some_and(|&i| t.w[i].upos == UPos::PUNCT) {
                ix.remove(0);
            }
            let drop: Vec<usize> = std::iter::once(h).chain(t.kids(ante).iter().copied().filter(|&c| matches!(t.base(c), Rel::Case | Rel::Punct | Rel::Acl | Rel::Appos))).collect();
            let np = unbracket(t, &t.sub(ante, &drop));
            prefix = if np.len() <= 8 { np } else { np.into_iter().filter(|&i| i <= ante).collect() };
        }
        let mut ps: Vec<P> = prefix.iter().chain(ix.iter()).map(|&i| P::T(i)).collect();
        let active = activize(t, toks, h, &ps, &cut);
        let activized = active.is_some();
        if let Some(a) = active {
            ps = a;
        }
        let mut dropped = 0;
        let mut moved: Vec<usize> = vec![];
        let result = loop {
            let ws = build(t, toks, h, &ps);
            let v = verbatim(&ws, toks);
            if v <= cap {
                break Some((ws, v));
            }
            // The step that best breaks the longest shared stretch: moving a modifier (keeps every
            // word) before leaving one out; then the least important, then the smallest.
            let mut best: Option<((usize, u8, usize, u8, usize), Vec<P>, Vec<usize>)> = None;
            let present: Vec<usize> = ps.iter().filter_map(|p| if let P::T(i) = p { Some(*i) } else { None }).collect();
            for &k in &present {
                if k == h || prefix.contains(&k) {
                    continue;
                }
                let Some(rank) = optional(t, k) else { continue };
                let sub: Vec<usize> = t.sub(k, &[]).into_iter().filter(|i| present.contains(i)).collect();
                if movable(t, h, k) && !moved.contains(&k) && sub.len() >= 2 {
                    let trial = relocate(t, &ps, &sub);
                    let tv = verbatim(&build(t, toks, h, &trial), toks);
                    let key = (if tv <= cap { 0 } else { tv }, 0, 0, rank, k);
                    if best.as_ref().is_none_or(|b| key < b.0) {
                        best = Some((key, trial, vec![]));
                    }
                }
                let trial: Vec<P> = ps.iter().copied().filter(|p| !matches!(p, P::T(i) if sub.contains(i))).collect();
                let tv = verbatim(&build(t, toks, h, &trial), toks);
                let key = (if tv <= cap { 0 } else { tv }, 1, sub.len(), rank, k);
                if best.as_ref().is_none_or(|b| key < b.0) {
                    best = Some((key, trial, sub));
                }
            }
            match best {
                Some((key, trial, sub)) if dropped + moved.len() < 8 => {
                    if key.1 == 0 {
                        moved.push(key.4);
                    } else {
                        dropped += 1;
                        let _ = sub;
                    }
                    ps = trial;
                }
                _ => break None,
            }
        };
        let Some((ws, v)) = result else {
            debug(t, toks, h, "no arrangement under the cap");
            continue;
        };
        let srcs: Vec<usize> = ws.iter().filter_map(|w| w.src).collect();
        let words = ws.iter().filter(|w| !punct_like(&w.form)).count();
        // A coordinated base form under a present plural governor is that present form: «parsers
        // read words and build trees».
        let present = kind == "conj" && t.w[h].tag == Tag::VB && !t.kids(h).iter().any(|&c| t.base(c) == Rel::Aux) && { let g = governor(t, h); t.w[g].tag == Tag::VBP && !t.kids(g).iter().any(|&c| t.base(c) == Rel::Aux) };
        let has_finite = activized || present || srcs.iter().any(|&i| finite(t, i) || t.w[i].upos == UPos::AUX) || (t.w[h].tag == Tag::VBN && !t.kids(h).iter().any(|&c| t.base(c) == Rel::Aux));
        // The clause keeps its subject: its own, the shared one, the antecedent or «the authors».
        let subject = !prefix.is_empty() || t.kids(h).iter().any(|&c| matches!(t.base(c), Rel::Nsubj | Rel::Csubj | Rel::Expl) && (srcs.contains(&c) || perspective(toks.get(c).map(|x| x.form.as_str()).unwrap_or("")).is_some()));
        // It does not open with a relative word, a conjunction or its own subordinator («That the
        // authors specify …»); a fronted adverbial clause («While X, Y») is fine.
        let opener = ws.first().and_then(|w| w.src).map(|i| t.w[i].tag.is_wh() || t.w[i].upos == UPos::CCONJ || (t.w[i].rel == Rel::Mark && t.head(i) == Some(h))).unwrap_or(false);
        if !(4..=60).contains(&words) || !has_finite || !srcs.contains(&h) || !subject || opener {
            debug(t, toks, h, &format!("rejected: words {words} finite {has_finite} subject {subject} opener {opener}"));
            continue;
        }
        let text = join(&ws, toks);
        let letters = text.chars().filter(|c| c.is_alphabetic()).count();
        if letters * 10 < text.chars().filter(|c| !c.is_whitespace()).count() * 7 || text.matches('"').count() % 2 == 1 || text.matches('“').count() != text.matches('”').count() || text.contains(",,") {
            continue;
        }
        out.push(Clause { text: mova_octopus::realize::sentence(text.trim_end_matches([',', ';', ':', '-'])), kind, verbatim: v, dropped, src: srcs.clone() });
        realized.push((h, prefix));
    }
    // A subordinate clause the parse made the root, with the main clause coordinated under it («In
    // order to guarantee X, the decoder maintains Y»): it is attached to that main clause again.
    if let Some(r) = sub_main {
        let host = out.iter().zip(realized.iter()).position(|(c, (h, _))| matches!(c.kind, "conj" | "parataxis") && (governor(t, *h) == r || t.head(*h) == Some(r)));
        if let (Some(k), Some((frag, srcs))) = (host, fragment(t, toks, r, &cut, cap)) {
            if out[k].text.split_whitespace().count() + frag.split_whitespace().count() <= 60 {
                let c = &mut out[k];
                c.text = format!("{}, {frag}.", c.text.trim_end_matches(['.', '!', '?']));
                c.src.extend(srcs);
                c.kind = "attached";
            }
        }
    }
    // An oblique, adverbial clause or adverb phrase of a realized clause that no clause restates
    // (left out for the cap, or cut off by a wrong attachment) is restated with that clause's core.
    let mut cov: Vec<usize> = out.iter().flat_map(|c| c.src.iter().copied()).collect();
    for (h, prefix) in &realized {
        for &k in t.kids(*h) {
            if !matches!(t.base(k), Rel::Obl | Rel::Advcl | Rel::Advmod) || cut.contains(&k) {
                continue;
            }
            let sub = unbracket(t, &t.sub(k, &cut));
            if sub.iter().filter(|&&i| !punct_like(t.form(i))).count() < 3 || sub.iter().any(|i| cov.contains(i)) {
                continue;
            }
            match split(t, toks, *h, prefix, &cut, &sub, cap) {
                Ok(c) => {
                    cov.extend(c.src.iter().copied());
                    out.push(c);
                }
                Err(why) => debug(t, toks, k, &format!("split: {why}")),
            }
        }
    }
    // An apposition or a past-participle modifier of a restated noun that no clause restates
    // becomes a predication of its own: «GUM, a freely available corpus» -> «GUM is a freely
    // available corpus»; «treebanks converted from constituent trees» -> «The treebanks are
    // converted from constituent trees» (the reduced relative clause written out).
    for n in 0..t.w.len() {
        if !cov.contains(&n) || !matches!(t.w[n].upos, UPos::NOUN | UPos::PROPN) {
            continue;
        }
        for &k in t.kids(n) {
            let appos = t.base(k) == Rel::Appos && matches!(t.w[k].upos, UPos::NOUN | UPos::PROPN);
            let participle = t.base(k) == Rel::Acl && t.w[k].tag == Tag::VBN && !t.kids(k).iter().any(|&c| matches!(t.base(c), Rel::Nsubj | Rel::Aux | Rel::Mark));
            if !(appos || participle) || cut.contains(&k) {
                continue;
            }
            let sub = unbracket(t, &t.sub(k, &cut));
            if sub.iter().filter(|&&i| !punct_like(t.form(i))).count() < 3 || sub.iter().any(|i| cov.contains(i)) {
                continue;
            }
            match predication(t, toks, n, &sub, appos, cap) {
                Ok(c) => {
                    cov.extend(c.src.iter().copied());
                    out.push(c);
                }
                Err(why) => debug(t, toks, k, &format!("predication: {why}")),
            }
        }
    }
    out
}

/// «N is/are M» from a noun `n` and its modifier subtree `sub`: the noun with its determiners,
/// adjectives, compounds and numbers, the copula by the noun's number, then the modifier.
fn predication(t: &Tree, toks: &[Tok], n: usize, sub: &[usize], appos: bool, cap: usize) -> Result<Clause, String> {
    let mut np: Vec<usize> = t.kids(n).iter().copied().filter(|&k| (matches!(t.base(k), Rel::Det | Rel::Compound | Rel::Nummod) || (t.base(k) == Rel::Amod && t.kids(k).is_empty()) || t.w[k].rel == Rel::NmodPoss) && k < n && t.kids(k).len() <= 1).flat_map(|k| t.sub(k, &[])).collect();
    np.push(n);
    np.sort();
    np.dedup();
    let np = unbracket(t, &np);
    if np.len() > 8 || np.iter().any(|&i| sub.contains(&i)) {
        return Err("noun phrase".into());
    }
    let plural = matches!(t.w[n].tag, Tag::NNS | Tag::NNPS);
    let mut ws: Vec<W> = vec![];
    for &i in &np {
        let f = toks[i].form.clone();
        match perspective(&f) {
            Some(p) => ws.extend(p.split(' ').map(|x| W { form: x.into(), src: None })),
            None => ws.push(W { form: f, src: Some(i) }),
        }
    }
    ws.push(W { form: if plural { "are" } else { "is" }.into(), src: None });
    let body: Vec<usize> = sub.iter().copied().skip_while(|&i| punct_like(t.form(i))).collect();
    for &i in &body {
        let f = toks[i].form.clone();
        match perspective(&f) {
            Some(p) => ws.extend(p.split(' ').map(|x| W { form: x.into(), src: None })),
            None => ws.push(W { form: f, src: Some(i) }),
        }
    }
    while ws.last().is_some_and(|w| punct_like(&w.form) || w.src.is_some_and(|i| matches!(t.w[i].upos, UPos::CCONJ | UPos::SCONJ | UPos::ADP | UPos::DET))) {
        ws.pop();
    }
    let v = verbatim(&ws, toks);
    let words = ws.iter().filter(|w| !punct_like(&w.form)).count();
    if v > cap || !(4..=40).contains(&words) {
        return Err(format!("verbatim {v} words {words}"));
    }
    let text = join(&ws, toks);
    if text.matches('"').count() % 2 == 1 || text.matches('“').count() != text.matches('”').count() || text.contains(",,") {
        return Err("quotes".into());
    }
    let _ = appos;
    let src: Vec<usize> = ws.iter().filter_map(|w| w.src).collect();
    Ok(Clause { text: mova_octopus::realize::sentence(text.trim_end_matches([',', ';', ':', '-'])), kind: "predication", verbatim: v, dropped: 0, src })
}

/// The core of a clause: its subject (`prefix`: the shared subject or the antecedent, else its
/// own), auxiliaries, copula, negation, the head and the head of its object with its determiners and
/// adjectives. None when the head has a clausal complement (the core alone would misstate it).
fn core(t: &Tree, h: usize, prefix: &[usize], cut: &[usize]) -> Option<Vec<usize>> {
    if t.kids(h).iter().any(|&c| matches!(t.base(c), Rel::Xcomp | Rel::Ccomp | Rel::Csubj)) {
        return None;
    }
    let mut ix: Vec<usize> = vec![h];
    for &c in t.kids(h) {
        let r = t.base(c);
        if prefix.is_empty() && matches!(r, Rel::Nsubj | Rel::Expl) {
            ix.extend(unbracket(t, &t.sub(c, cut)));
        } else if matches!(r, Rel::Aux | Rel::Cop) || t.w[c].rel == Rel::CompoundPrt || (r == Rel::Advmod && optional(t, c).is_none()) {
            ix.push(c);
        } else if r == Rel::Obj {
            ix.push(c);
            ix.extend(t.kids(c).iter().copied().filter(|&k| (matches!(t.base(k), Rel::Det | Rel::Compound | Rel::Nummod) || (t.base(k) == Rel::Amod && t.kids(k).is_empty()) || t.w[k].rel == Rel::NmodPoss) && t.kids(k).len() <= 1));
        } else if t.w[h].upos != UPos::VERB && (matches!(r, Rel::Det | Rel::Compound | Rel::Nummod | Rel::Case) || (r == Rel::Amod && t.kids(c).is_empty())) {
            ix.push(c);
        }
    }
    ix.sort();
    ix.dedup();
    Some(ix)
}

/// A modifier left out of a clause for the verbatim cap, restated with the clause's core: «The
/// authors extract the edits by means of an alignment algorithm.» Every word is the source's own
/// (or a perspective word); the arrangement obeys the same cap and the same checks as any clause.
fn split(t: &Tree, toks: &[Tok], h: usize, prefix: &[usize], cut: &[usize], sub: &[usize], cap: usize) -> Result<Clause, String> {
    let mut rest = core(t, h, prefix, cut).ok_or("clausal complement")?;
    rest.retain(|i| !prefix.contains(i) && !sub.contains(i));
    rest.extend(sub.iter().copied());
    rest.sort();
    let ps: Vec<P> = prefix.iter().chain(rest.iter()).map(|&i| P::T(i)).collect();
    let mut ws = build(t, toks, h, &ps);
    let mut v = verbatim(&ws, toks);
    if v > cap {
        let moved = relocate(t, &ps, sub);
        ws = build(t, toks, h, &moved);
        v = verbatim(&ws, toks);
        if v > cap {
            return Err(format!("verbatim {v}"));
        }
    }
    let srcs: Vec<usize> = ws.iter().filter_map(|w| w.src).collect();
    let words = ws.iter().filter(|w| !punct_like(&w.form)).count();
    let has_finite = srcs.iter().any(|&i| finite(t, i) || t.w[i].upos == UPos::AUX);
    let subject = !prefix.is_empty() || t.kids(h).iter().any(|&c| matches!(t.base(c), Rel::Nsubj | Rel::Expl) && (srcs.contains(&c) || perspective(toks.get(c).map(|x| x.form.as_str()).unwrap_or("")).is_some()));
    let opener = ws.first().and_then(|w| w.src).map(|i| t.w[i].tag.is_wh() || t.w[i].upos == UPos::CCONJ).unwrap_or(false);
    if !(5..=40).contains(&words) || !has_finite || !srcs.contains(&h) || !subject || opener {
        return Err(format!("words {words} finite {has_finite} subject {subject} opener {opener}"));
    }
    let text = join(&ws, toks);
    if text.matches('"').count() % 2 == 1 || text.matches('“').count() != text.matches('”').count() || text.contains(",,") {
        return Err("quotes".into());
    }
    Ok(Clause { text: mova_octopus::realize::sentence(text.trim_end_matches([',', ';', ':', '-'])), kind: "split", verbatim: v, dropped: 0, src: srcs })
}


/// A question the paper asks («Is one side of the edit in the past tense?»), restated as the
/// paper's question: «The paper asks: is one side of the edit in the past tense?» Never as a
/// statement. Its words are the source's own (brackets left out) under the same verbatim cap; the
/// frame «the paper asks» is the describer's.
pub fn question(t: &Tree, toks: &[Tok], cap: usize) -> Option<Clause> {
    if t.w.len() != toks.len() || toks.last().map(|x| x.form.as_str()) != Some("?") {
        return None;
    }
    let ix = unbracket(t, &(0..toks.len() - 1).collect::<Vec<_>>());
    let mut ws: Vec<W> = ["the", "paper", "asks"].iter().map(|f| W { form: f.to_string(), src: None }).collect();
    for &i in &ix {
        let f = toks[i].form.clone();
        let f = if i == 0 && !matches!(t.w[0].upos, UPos::PROPN) && f.chars().skip(1).all(|c| !c.is_uppercase()) { f.to_lowercase() } else { f };
        match perspective(&f) {
            Some(p) => ws.extend(p.split(' ').map(|x| W { form: x.into(), src: None })),
            None => ws.push(W { form: f, src: Some(i) }),
        }
    }
    while ws.last().is_some_and(|w| punct_like(&w.form)) {
        ws.pop();
    }
    let v = verbatim(&ws, toks);
    let words = ws.iter().filter(|w| !punct_like(&w.form)).count();
    if v > cap || !(7..=40).contains(&words) {
        return None;
    }
    let body = join(&ws[3..], toks);
    let src: Vec<usize> = ws.iter().filter_map(|w| w.src).collect();
    Some(Clause { text: format!("The paper asks: {body}?"), kind: "question", verbatim: v, dropped: 0, src })
}

/// A subordinate clause parsed alone («As the distance only aligns tokens», «Given that corpora
/// are annotated differently», «in order to guarantee a valid tree»): see `fragment`.
pub fn subordinate(t: &Tree, toks: &[Tok], cap: usize) -> Option<(String, Vec<usize>)> {
    if t.w.len() != toks.len() {
        return None;
    }
    fragment(t, toks, t.root()?, &[], cap)
}

/// An adverbial clause opened by its subordinator (not a bare infinitive «to»): its words in
/// source order with the authors' perspective, the subordinator kept, brackets left out, in lower
/// case, ready to follow the clause it modifies after a comma. None unless it is a predicate, its
/// subordinator opens it and its longest stretch shared with the source is within the cap. On its
/// own it is a fragment: the caller attaches it.
fn fragment(t: &Tree, toks: &[Tok], h: usize, cut: &[usize], cap: usize) -> Option<(String, Vec<usize>)> {
    let mark = t.kids(h).iter().copied().find(|&c| t.w[c].rel == Rel::Mark && t.form(c) != "to")?;
    let all = t.sub(h, cut);
    if !predicate(t, h) || mark > h {
        return None;
    }
    let ix = unbracket(t, &all);
    let mut ws: Vec<W> = vec![];
    for &i in &ix {
        let f = toks[i].form.clone();
        match perspective(&f) {
            Some(p) => ws.extend(p.split(' ').map(|x| W { form: x.into(), src: None })),
            None => ws.push(W { form: f, src: Some(i) }),
        }
    }
    while ws.last().is_some_and(|w| punct_like(&w.form)) {
        ws.pop();
    }
    let srcs: Vec<usize> = ws.iter().filter_map(|w| w.src).collect();
    let words = ws.iter().filter(|w| !punct_like(&w.form)).count();
    if !(3..=30).contains(&words) || verbatim(&ws, toks) > cap {
        return None;
    }
    let text = join(&ws, toks);
    if text.matches('"').count() % 2 == 1 || text.matches('“').count() != text.matches('”').count() {
        return None;
    }
    let mut c = text.chars();
    let first = c.next()?;
    let text = if toks[mark].form.chars().skip(1).all(|x| !x.is_uppercase()) { first.to_lowercase().chain(c).collect() } else { text.clone() };
    Some((text, srcs))
}

/// The grammar gate's reading of one written sentence: Mova parses it again on its own. It passes
/// when some verb of it is a finite clause with a subject (a finite verb, or a finite auxiliary or
/// copula of its head; a past form with a subject and no auxiliary counts as finite) or at least
/// some verb has a finite form, and none of
/// the fragment marks of the 2026-11-04 writer is on it: a bare subordinated «-ing» form at its end
/// («…, while maintaining.»), a participle read as the verb right after a determiner («This
/// architecture all the previously derived subtrees»), a plural noun of a compound read as the
/// verb («A number of parsing steps linear in …»), or a function word at its end. Mova's parse of
/// long scientific sentences is unreliable, so the gate looks for these marks rather than asking
/// for a perfect parse. `Err` names what is wrong.
pub fn grammatical(a: &en::annotate::Annotator, text: &str) -> Result<(), &'static str> {
    // The paper's own questions are written «The paper asks: Q?»; the frame is checked, the
    // question is the paper's.
    let body = text.trim();
    if body.starts_with("The paper asks: ") && body.ends_with('?') {
        return Ok(());
    }
    let toks = a.tokenize(body);
    let t = Tree::parse(a, body);
    if t.w.len() != toks.len() || t.root().is_none() {
        return Err("unparsed");
    }
    let past = |i: usize| matches!(t.w[i].tag, Tag::VBD | Tag::VBN) && t.w[i].upos == UPos::VERB && !t.kids(i).iter().any(|&c| matches!(t.base(c), Rel::Aux) || t.w[c].rel == Rel::AuxPass);
    let subject = |h: usize| t.kids(h).iter().any(|&c| matches!(t.base(c), Rel::Nsubj | Rel::Csubj | Rel::Expl));
    let clause = (0..t.w.len()).any(|i| {
        let h = if matches!(t.base(i), Rel::Aux | Rel::Cop) || t.w[i].rel == Rel::AuxPass { t.head(i).unwrap_or(i) } else { i };
        // A base form with its own subject and no auxiliary or subordinator is a present
        // («the authors normalize …» tagged VB).
        let base = t.w[i].tag == Tag::VB && t.w[i].upos == UPos::VERB && !t.kids(i).iter().any(|&c| matches!(t.base(c), Rel::Aux | Rel::Mark));
        (finite(&t, i) || (i == h && (past(i) || base))) && subject(h)
    });
    // Mova often reads a past verb of a whole sentence as a reduced relative clause («An
    // additional graduate student participated in …» as a noun phrase): a verb in a finite past or
    // present form anywhere counts as well.
    let finite_verb = (0..t.w.len()).any(|i| matches!(t.w[i].upos, UPos::VERB | UPos::AUX) && (matches!(t.w[i].tag, Tag::VBD | Tag::VBZ | Tag::VBP | Tag::MD) || (i > 0 && t.w[i].tag == Tag::VBN && t.w[i - 1].upos != UPos::DET && past_form(&toks[i].form.to_lowercase()))));
    // Mova also tags some present verbs as nouns («Segmented Discourse Representation Theory
    // augments DRT …»): a word the dictionary knows as a finite verb form, not inside a noun
    // phrase (no determiner or adjective right before it), counts too.
    let lexical = (1..toks.len()).any(|i| {
        let low = toks[i].form.to_lowercase();
        low.chars().all(char::is_alphabetic) && !matches!(t.w[i - 1].upos, UPos::DET | UPos::ADJ | UPos::ADP) && finite_form(&low)
    });
    if !clause && !finite_verb && !lexical {
        return Err("no finite verb");
    }
    // A participle read as the verb right after a determiner (adverbs between).
    for i in 0..t.w.len() {
        if matches!(t.w[i].tag, Tag::VBD | Tag::VBN) && subject(i) {
            let mut k = i;
            while k > 0 && t.w[k - 1].upos == UPos::ADV {
                k -= 1;
            }
            if k > 0 && t.w[k - 1].upos == UPos::DET {
                return Err("participle after a determiner read as the verb");
            }
        }
    }
    if let Some(root) = t.root() {
        // A plural noun of a compound read as the verb: a present form the dictionary knows mostly
        // as a plural noun, right after a noun or an «-ing» word, without an object, with a bare
        // adjective as its complement.
        if matches!(t.w[root].tag, Tag::VBZ | Tag::VBP)
            && root > 0
            && matches!(t.w[root - 1].tag, Tag::NN | Tag::NNS | Tag::VBG)
            && !t.kids(root).iter().any(|&c| t.base(c) == Rel::Obj)
            && t.kids(root).iter().any(|&c| t.base(c) == Rel::Xcomp && t.w[c].upos == UPos::ADJ)
            && noun_first(&t.form(root).to_lowercase())
        {
            return Err("plural noun read as the verb");
        }
    }
    // The last word before the closing punctuation.
    if let Some(e) = (0..t.w.len()).rev().find(|&i| t.w[i].upos != UPos::PUNCT) {
        let low = toks[e].form.to_lowercase();
        let before = e.checked_sub(1).map(|p| toks[p].form.to_lowercase()).unwrap_or_default();
        let subordinator = matches!(before.as_str(), "while" | "when" | "by" | "for" | "after" | "before" | "without" | "of" | "in" | "on" | "through" | "via" | "since" | "whilst" | "upon");
        if low.ends_with("ing") && low.len() > 4 && subordinator && t.w[e].upos == UPos::VERB {
            return Err("bare subordinated -ing form at the end");
        }
        if matches!(t.w[e].upos, UPos::DET | UPos::CCONJ | UPos::SCONJ | UPos::ADP) && !matches!(low.as_str(), "up" | "out" | "off" | "in" | "on" | "down" | "over" | "back" | "away") {
            return Err("ends in a function word");
        }
    }
    Ok(())
}

/// A form the dictionary knows as a finite verb (present or past), other than the most common
/// function words.
fn finite_form(lower: &str) -> bool {
    !matches!(lower, "a" | "an" | "the" | "and" | "or" | "of" | "to" | "in" | "for" | "as" | "that" | "this")
        && en::dict::form(lower).is_some_and(|f| en::dict::analyses(f).any(|r| matches!(r.tag, Tag::VBZ | Tag::VBD | Tag::VBP | Tag::MD)))
}

/// A form the dictionary knows as a simple past («produced»), not only as a participle.
fn past_form(lower: &str) -> bool {
    en::dict::form(lower).is_some_and(|f| en::dict::analyses(f).any(|r| r.tag == Tag::VBD))
}

/// A form the dictionary's treebank counts know more often as a plural noun than as a present verb.
fn noun_first(lower: &str) -> bool {
    let Some(f) = en::dict::form(lower) else { return false };
    let (mut noun, mut verb) = (0u32, 0u32);
    for r in en::dict::analyses(f) {
        match r.tag {
            Tag::NNS => noun += r.count,
            Tag::VBZ | Tag::VBP => verb += r.count,
            _ => {}
        }
    }
    noun > verb
}

#[cfg(test)]
mod tests {
    use super::*;
    fn toks(s: &[&str]) -> Vec<Tok> {
        s.iter().map(|f| Tok { form: f.to_string(), space_after: !matches!(*f, "(") }).collect()
    }
    #[test]
    fn verbatim_runs_count_alphanumeric_source_stretches() {
        let t = toks(&["We", "applied", "it", "(", "2012", ")", "to", "all", "systems", "."]);
        let ws = |v: &[(&str, Option<usize>)]| v.iter().map(|(f, s)| W { form: f.to_string(), src: *s }).collect::<Vec<_>>();
        // «the authors applied it to all systems»: the bracket was left out, so «it to» is not adjacent in the source.
        let a = ws(&[("the", None), ("authors", None), ("applied", Some(1)), ("it", Some(2)), ("to", Some(6)), ("all", Some(7)), ("systems", Some(8))]);
        assert_eq!(verbatim(&a, &t), 3);
        // Punctuation between kept words does not break a stretch.
        let t2 = toks(&["A", ",", "B", "C"]);
        assert_eq!(verbatim(&ws(&[("A", Some(0)), ("B", Some(2)), ("C", Some(3))]), &t2), 3);
        assert_eq!(join(&a, &t), "the authors applied it to all systems");
        assert_eq!(perspective("Our"), Some("their"));
    }
}

#[cfg(test)]
mod dev {
    /// Development probe: the clauses of every line of `WHALE_DEV_SENTENCES` (a file).
    #[test]
    #[ignore]
    fn probe() {
        std::env::set_var("WORLD_EN_MODEL", std::env::var("MOVA_OCTOPUS_TEST_MODEL").unwrap());
        let a = world::tree::annotator().unwrap();
        for s in std::fs::read_to_string(std::env::var("WHALE_DEV_SENTENCES").unwrap()).unwrap().lines().filter(|l| !l.trim().is_empty()) {
            let toks = a.tokenize(s);
            let t = world::tree::Tree::parse(a, s);
            println!("S {s}\n  {}", t.show());
            for c in super::clauses(&t, &toks, 12) {
                println!("  = {} ({}, v{} d{}) grammar {:?}", c.text, c.kind, c.verbatim, c.dropped, super::grammatical(a, &c.text));
            }
        }
    }
}

#[cfg(test)]
mod correlative_tests {
    /// «not only … but also»: both halves are restated, «not only» is left out of the first (never
    /// «only» alone: «This not means …»), and the fragments the writer of 2026-11-04c wrote on
    /// ma-2018 are refused by the grammar gate (red control).
    #[test]
    fn both_halves_of_a_correlative_and_no_fragments() {
        if !crate::learn::tests::model_ok() {
            return;
        }
        let a = world::tree::annotator().unwrap();
        let all = |s: &str| -> Vec<String> {
            let toks = a.tokenize(s);
            super::clauses(&world::tree::Tree::parse(a, s), &toks, 12).into_iter().map(|c| c.text).collect()
        };
        let c = all("This not only facilitates error type evaluation at different levels of granularity, but can also be used to reduce annotator workload and standardise existing GEC datasets.");
        assert!(c.iter().any(|x| x.starts_with("This facilitates error type evaluation")), "{c:?}");
        assert!(c.iter().any(|x| x.contains("can also be used")), "the «but also» half was dropped: {c:?}");
        assert!(!c.iter().any(|x| x.contains("not only") || x.contains("not facilitates")), "{c:?}");
        let c = all("This not only means hypothesis edits are more likely to match the reference edits, but also that the official corrected sentences will score the maximum 100 F 0.");
        assert!(!c.iter().any(|x| x.starts_with("This not means")), "{c:?}");
        for frag in ["This architecture all the previously derived subtrees, while maintaining.", "A number of parsing steps linear in the sentence length."] {
            assert!(super::grammatical(a, frag).is_err(), "{frag}");
        }
        for good in ["This architecture makes it possible to capture information from the whole sentence.", "An additional graduate student participated in the final debugging of the dataset.", "Prior to annotating their corpus, the authors normalize accented vowels to their unaccented form.", "The paper asks: do both sides of the edit have the same lemma?"] {
            assert_eq!(super::grammatical(a, good), Ok(()), "{good}");
        }
    }
}
