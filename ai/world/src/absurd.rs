//! Absurdity check of dependency trees with the absurdity matrix (`docs/absurdity.md`): each verb's agent and
//! patient noun get a graded score 0–4 from the matrix cell `verb|noun` (SUBJ or OBJ column, real-world scale).
//! Every judgment is logged — absurd and plausible alike — with the cell it used; what the matrix does not know
//! (verb or noun missing) goes to a gap journal with counts and examples, as the reader's gap journal does.
//!
//!   world absurd-check [<scores.json>] <in.conllu> [<gold.conllu>] --out <dir> [--domain real|tale]
//!
//! Without a scores file the level-1 matrix compiled into `global` is used; `--domain tale` switches to the
//! fairy-tale layer (talking animals are normal there).
//!
//! Writes `<dir>/judgments.jsonl` (one line per judged argument), `<dir>/gaps.jsonl` (unknown verbs and nouns,
//! most frequent first) and `<dir>/report.txt` (the printed summary). With gold: the parse-error rate per score —
//! the matrix is useful for error search and reranking if high scores mean more errors.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result, bail};
use en::gram::UPos;
use serde_json::json;

use crate::sense::{Node, arguments, nodes};

/// The matrix: `verb → noun → [SUBJ, OBJ]` — the level-1 table compiled into `global`, or a scores file
/// (experiments with a matrix not compiled yet).
pub struct Matrix {
    cells: BTreeMap<String, BTreeMap<String, [u8; 2]>>,
    nouns: BTreeSet<String>,
    global: bool,
    domain: String,
}

impl Matrix {
    /// The level-1 matrix (`global/data/absurdity-roles.tsv`, compiled by build.rs).
    pub fn global() -> Matrix {
        Matrix::global_in("real")
    }

    /// The level-1 matrix in a domain (`real`, `tale`): the domain layer where it has the cell, else real.
    pub fn global_in(domain: &str) -> Matrix {
        Matrix { cells: BTreeMap::new(), nouns: BTreeSet::new(), global: true, domain: domain.to_string() }
    }

    pub fn name(&self) -> String {
        if self.global { format!("global, domain {} ({} verbs × {} nouns)", self.domain, global::ABSURD_VERBS.len(), global::ABSURD_NOUNS.len()) } else { format!("file ({} verbs × {} nouns)", self.cells.len(), self.nouns.len()) }
    }

    pub fn has_noun(&self, noun: &str) -> bool {
        if self.global { global::absurd_has_noun(noun) } else { self.nouns.contains(noun) }
    }

    pub fn load(path: &Path) -> Result<Matrix> {
        Matrix::parse(&std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?)
    }

    pub fn parse(text: &str) -> Result<Matrix> {
        let v: BTreeMap<String, [u8; 2]> = serde_json::from_str(text)?;
        let mut cells: BTreeMap<String, BTreeMap<String, [u8; 2]>> = BTreeMap::new();
        let mut nouns = BTreeSet::new();
        for (k, s) in v {
            let (verb, noun) = k.split_once('|').with_context(|| format!("bad cell key {k}"))?;
            if s.iter().any(|&x| x > 4) {
                bail!("cell {k}: score out of 0–4");
            }
            nouns.insert(noun.to_string());
            cells.entry(verb.to_string()).or_default().insert(noun.to_string(), s);
        }
        Ok(Matrix { cells, nouns, global: false, domain: "real".into() })
    }

    pub fn has_verb(&self, verb: &str) -> bool {
        if self.global { global::absurd_has_verb(verb) } else { self.cells.contains_key(verb) }
    }

    pub fn score(&self, verb: &str, noun: &str, role: usize) -> Option<u8> {
        if self.global {
            // a known verbal multiword expression is not literally absurd ("take place", "pay attention")
            let lit = global::absurdity_in(&self.domain, verb, noun).map(|s| s[role]);
            if role == 1 && matches!(global::idiom(verb, noun), Some("idiom" | "light-verb" | "collocation")) {
                return lit.map(|x| x.min(1));
            }
            return lit;
        }
        self.cells.get(verb)?.get(noun).map(|s| s[role])
    }
}

const ROLE: [&str; 2] = ["SUBJ", "OBJ"];

pub fn verdict(s: u8) -> &'static str {
    match s {
        0 | 1 => "plausible",
        2 => "odd",
        _ => "absurd",
    }
}

/// One judged argument.
#[derive(Clone, Debug)]
pub struct Judgment {
    pub verb: usize,
    pub noun: usize,
    /// 0 = SUBJ (agent), 1 = OBJ (patient)
    pub role: usize,
    pub score: u8,
    /// score of the same noun in the other role (for a role-swap reading), if known
    pub swapped: Option<u8>,
}

/// A thing the matrix does not know: `("verb", lemma)` or `("noun", lemma)`.
pub type Gap = (&'static str, String);

/// Judge one tree: judgments for known cells, gaps for unknown verbs/nouns (only verbs with a noun argument count).
pub fn judge(m: &Matrix, t: &[Node]) -> (Vec<Judgment>, Vec<Gap>) {
    let (mut js, mut gaps) = (Vec::new(), Vec::new());
    for (v, vn) in t.iter().enumerate() {
        if vn.upos != Some(UPos::VERB) {
            continue;
        }
        let (agent, patient) = arguments(t, v);
        if agent.is_none() && patient.is_none() {
            continue;
        }
        if !m.has_verb(&vn.lemma) {
            gaps.push(("verb", vn.lemma.clone()));
            continue;
        }
        // with an idiomatic object the verb means something else ("the marriage took place"): skip the doer
        let idiom = patient.is_some_and(|p| matches!(global::idiom(&vn.lemma, &t[p].lemma), Some("idiom" | "light-verb")));
        for (role, arg) in [(0usize, agent), (1, patient)] {
            let Some(a) = arg else { continue };
            if role == 0 && idiom {
                continue;
            }
            let noun = &t[a].lemma;
            if !m.has_noun(noun) {
                gaps.push(("noun", noun.clone()));
                continue;
            }
            if let Some(score) = m.score(&vn.lemma, noun, role) {
                js.push(Judgment { verb: v, noun: a, role, score, swapped: m.score(&vn.lemma, noun, 1 - role) });
            }
        }
    }
    (js, gaps)
}

fn text_of(s: &en::conllu::Sentence, t: &[Node]) -> String {
    if s.text.is_empty() { t.iter().map(|n| n.form.as_str()).collect::<Vec<_>>().join(" ") } else { s.text.clone() }
}

/// `world absurd-check`.
pub fn run(scores: Option<&Path>, input: &Path, gold: Option<&Path>, out: &Path, domain: &str) -> Result<()> {
    let m = match scores {
        Some(p) => Matrix::load(p)?,
        None => Matrix::global_in(domain),
    };
    let sys = en::conllu::read(input)?;
    let gold: Option<BTreeMap<String, Vec<Node>>> = match gold {
        Some(g) => Some(en::conllu::read(g)?.iter().filter(|s| !s.id.is_empty()).map(|s| (s.id.clone(), nodes(s))).collect()),
        None => None,
    };
    std::fs::create_dir_all(out)?;
    let mut jf = std::io::BufWriter::new(std::fs::File::create(out.join("judgments.jsonl"))?);
    // gap → (count, examples)
    let mut gaps: BTreeMap<Gap, (usize, Vec<String>)> = BTreeMap::new();
    // score → (judgments, with gold aligned, parse errors at the noun)
    let mut by_score: BTreeMap<u8, (usize, usize, usize)> = BTreeMap::new();
    let mut by_role: BTreeMap<(usize, &str), usize> = BTreeMap::new();
    let (mut verbs_with_args, mut verbs_known, mut swaps) = (0usize, 0usize, 0usize);
    let mut absurd_cells: BTreeMap<String, usize> = BTreeMap::new();
    for s in &sys {
        let t = nodes(s);
        let (js, gs) = judge(&m, &t);
        for v in 0..t.len() {
            if t[v].upos == Some(UPos::VERB) && arguments(&t, v) != (None, None) {
                verbs_with_args += 1;
                verbs_known += m.has_verb(&t[v].lemma) as usize;
            }
        }
        let text = text_of(s, &t);
        for g in gs {
            let e = gaps.entry(g).or_default();
            e.0 += 1;
            if e.1.len() < 3 {
                e.1.push(format!("{}: {}", s.id, text));
            }
        }
        let g = gold.as_ref().and_then(|gm| gm.get(&s.id)).filter(|g| g.len() == t.len() && g.iter().zip(&t).all(|(a, b)| a.form == b.form));
        for j in &js {
            let (verb, noun) = (&t[j.verb].lemma, &t[j.noun].lemma);
            let wrong = g.map(|g| g[j.noun].head != t[j.noun].head || g[j.noun].rel != t[j.noun].rel);
            let e = by_score.entry(j.score).or_default();
            e.0 += 1;
            if let Some(w) = wrong {
                e.1 += 1;
                e.2 += w as usize;
            }
            *by_role.entry((j.role, verdict(j.score))).or_default() += 1;
            // a role swap: absurd now, plausible in the other role
            let swap = j.score >= 3 && j.swapped.is_some_and(|x| x <= 1);
            swaps += swap as usize;
            if j.score >= 3 {
                *absurd_cells.entry(format!("{verb}|{noun} {}", ROLE[j.role])).or_default() += 1;
            }
            let why = format!(
                "cell {verb}|{noun} {}={} ({}){}",
                ROLE[j.role],
                j.score,
                verdict(j.score),
                match j.swapped {
                    Some(x) => format!("; other role {}={x}{}", ROLE[1 - j.role], if swap { " — reading as a role swap" } else { "" }),
                    None => String::new(),
                }
            );
            let line = json!({
                "sent": s.id, "text": text, "verb": t[j.verb].form, "verb_lemma": verb,
                "noun": t[j.noun].form, "noun_lemma": noun, "rel": t[j.noun].rel.to_string(),
                "role": ROLE[j.role], "score": j.score, "verdict": verdict(j.score),
                "other_role_score": j.swapped, "role_swap": swap,
                "cell": format!("{verb}|{noun}"), "source": format!("absurdity matrix, {}", m.name()),
                "why": why, "parse_error": wrong,
            });
            writeln!(jf, "{line}")?;
        }
    }
    jf.flush()?;
    let mut gl: Vec<(&Gap, &(usize, Vec<String>))> = gaps.iter().collect();
    gl.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
    let mut gf = std::io::BufWriter::new(std::fs::File::create(out.join("gaps.jsonl"))?);
    for ((kind, lemma), (n, ex)) in &gl {
        let why = if *kind == "verb" { "verb has no row in the matrix" } else { "noun has no column in the matrix" };
        writeln!(gf, "{}", json!({"kind": kind, "lemma": lemma, "count": n, "why": why, "examples": ex}))?;
    }
    gf.flush()?;

    let mut r = String::new();
    let total: usize = by_score.values().map(|x| x.0).sum();
    let pct = |a: usize, b: usize| if b == 0 { 0.0 } else { 100.0 * a as f64 / b as f64 };
    r += &format!("input {}  sentences {}  matrix: {}\n", input.display(), sys.len(), m.name());
    r += &format!("verbs with a noun argument {verbs_with_args}, verb in the matrix {verbs_known} ({:.1}%)\n", pct(verbs_known, verbs_with_args));
    r += &format!("judgments {total}  (role swaps: absurd now, plausible in the other role: {swaps})\n");
    for role in 0..2 {
        let get = |v: &str| by_role.get(&(role, v)).copied().unwrap_or(0);
        r += &format!("  {:4}  plausible {}  odd {}  absurd {}\n", ROLE[role], get("plausible"), get("odd"), get("absurd"));
    }
    r += "score  judgments";
    if gold.is_some() {
        r += "  with gold  parse errors at the noun";
    }
    r += "\n";
    for (sc, (n, ng, ne)) in &by_score {
        r += &format!("  {sc}    {n:6}");
        if gold.is_some() {
            r += &format!("  {ng:6}  {ne:5} = {:.1}%", pct(*ne, *ng));
        }
        r += "\n";
    }
    let mut ac: Vec<(&String, &usize)> = absurd_cells.iter().collect();
    ac.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    r += &format!("most frequent absurd cells: {}\n", ac.iter().take(15).map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", "));
    let (gv, gn): (Vec<&(&Gap, &(usize, Vec<String>))>, Vec<_>) = gl.iter().partition(|x| x.0.0 == "verb");
    r += &format!("gaps: unknown verbs {} ({} uses), unknown nouns {} ({} uses)\n", gv.len(), gv.iter().map(|x| x.1.0).sum::<usize>(), gn.len(), gn.iter().map(|x| x.1.0).sum::<usize>());
    r += &format!("  top verbs: {}\n", gv.iter().take(12).map(|x| format!("{} {}", x.0.1, x.1.0)).collect::<Vec<_>>().join(", "));
    r += &format!("  top nouns: {}\n", gn.iter().take(12).map(|x| format!("{} {}", x.0.1, x.1.0)).collect::<Vec<_>>().join(", "));
    print!("{r}");
    std::fs::write(out.join("report.txt"), r)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use en::gram::Rel;

    fn tree(ws: &[(&str, &str, UPos, usize, Rel)]) -> Vec<Node> {
        ws.iter().map(|&(f, l, u, h, r)| Node { form: f.into(), lemma: l.into(), upos: Some(u), head: h, rel: r }).collect()
    }

    fn matrix() -> Matrix {
        Matrix::parse(r#"{"eat|cheese":[4,0],"eat|mouse":[0,1]}"#).unwrap()
    }

    #[test]
    fn cheese_eats_mouse_is_absurd_and_a_swap() {
        let m = matrix();
        let t = tree(&[("cheese", "cheese", UPos::NOUN, 2, Rel::Nsubj), ("eats", "eat", UPos::VERB, 0, Rel::Root), ("mouse", "mouse", UPos::NOUN, 2, Rel::Obj)]);
        let (js, gaps) = judge(&m, &t);
        assert!(gaps.is_empty());
        assert_eq!(js.len(), 2);
        assert_eq!((js[0].role, js[0].score, js[0].swapped), (0, 4, Some(0)));
        assert_eq!((js[1].role, js[1].score), (1, 1));
    }

    #[test]
    fn plausible_reading_and_gaps() {
        // negative control: "the mouse eats the cheese" is plausible in both roles
        let m = matrix();
        let t = tree(&[("mouse", "mouse", UPos::NOUN, 2, Rel::Nsubj), ("eats", "eat", UPos::VERB, 0, Rel::Root), ("cheese", "cheese", UPos::NOUN, 2, Rel::Obj)]);
        let (js, _) = judge(&m, &t);
        assert!(js.iter().all(|j| j.score <= 1), "{js:?}");
        // unknown verb and unknown noun go to the gap journal, never to a judgment
        let t = tree(&[("mouse", "mouse", UPos::NOUN, 2, Rel::Nsubj), ("gnaws", "gnaw", UPos::VERB, 0, Rel::Root), ("cheese", "cheese", UPos::NOUN, 2, Rel::Obj)]);
        let (js, gaps) = judge(&m, &t);
        assert!(js.is_empty());
        assert_eq!(gaps, vec![("verb", "gnaw".to_string())]);
        let t = tree(&[("zorb", "zorb", UPos::NOUN, 2, Rel::Nsubj), ("eats", "eat", UPos::VERB, 0, Rel::Root)]);
        let (js, gaps) = judge(&m, &t);
        assert!(js.is_empty());
        assert_eq!(gaps, vec![("noun", "zorb".to_string())]);
    }
}

/// `world absurd-events <events.tsv> [--domain real|tale] [--top N]`: corpus events (`world events`) scored by the
/// matrix, absurd ones (a role scored ≥ 3) ranked by count — systematic errors of the tokenizer, tagger,
/// lemmatizer or parser show up as frequent absurd events.
pub fn events(path: &Path, domain: &str, top: usize) -> Result<()> {
    let m = Matrix::global_in(domain);
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    let mut rows: Vec<(usize, usize, String, String)> = Vec::new();
    let (mut judged, mut absurd_n) = (0usize, 0usize);
    for l in text.lines().skip(1) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 6 {
            continue;
        }
        let (agent, verb, patient) = (c[0], c[1], c[2]);
        let (n, books): (usize, usize) = (c[4].parse().unwrap_or(0), c[5].parse().unwrap_or(0));
        let mut why = Vec::new();
        let mut any = false;
        let idiom = !patient.is_empty() && matches!(global::idiom(verb, patient), Some("idiom" | "light-verb"));
        for (noun, role) in [(agent, 0usize), (patient, 1)] {
            if noun.is_empty() || (role == 0 && idiom) {
                continue;
            }
            if let Some(s) = m.score(verb, noun, role) {
                any = true;
                if s >= 3 {
                    why.push(format!("{verb}|{noun} {}={s}", ROLE[role]));
                }
            }
        }
        judged += any as usize * n;
        if !why.is_empty() {
            absurd_n += n;
            let ev = format!("{} {verb}{}", if agent.is_empty() { "_" } else { agent }, if patient.is_empty() { String::new() } else { format!(" {patient}") });
            rows.push((n, books, ev, why.join(", ")));
        }
    }
    rows.sort_by(|a, b| b.0.cmp(&a.0).then(a.2.cmp(&b.2)));
    println!("matrix {}  judged event uses {judged}, absurd {absurd_n} ({:.1}%)", m.name(), 100.0 * absurd_n as f64 / judged.max(1) as f64);
    for (n, b, ev, why) in rows.iter().take(top) {
        println!("{n:7} {b:4} books  {ev:40} {why}");
    }
    Ok(())
}
