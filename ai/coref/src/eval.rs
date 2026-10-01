//! Measurements on a corpus with gold coreference (GUM): systems (baselines and the sieve), CoNLL F1 with and without singletons,
//! precision of each sieve, sieve contributions (cumulative), 3rd-person pronouns by form.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;

use anyhow::{Result, bail};
use en::conllu::Doc as UdDoc;

use crate::corefud::{self, OutMention};
use crate::doc::Document;
use crate::mention::{Mention, P3};
use crate::score::{self, MSpan, Matching, Scores};
use crate::sieve::{Config, Link, Resolver, Sieve};

/// Gold for a document: mentions with heads from the gold tree, and their entities.
pub struct Gold {
    pub entities: Vec<Vec<MSpan>>,
    pub mentions: Vec<MSpan>,
    pub entity_of: Vec<usize>,
    /// The mention has a predecessor in its chain (not the first in text order).
    pub anaphoric: Vec<bool>,
}

pub fn gold_of(doc: &Document) -> Result<Gold> {
    let fm = corefud::read_mentions(doc)?;
    let mut ids: HashMap<String, usize> = HashMap::new();
    let mut entities: Vec<Vec<MSpan>> = Vec::new();
    let (mut mentions, mut entity_of) = (Vec::new(), Vec::new());
    for m in &fm {
        let head = corefud::span_head(&doc.sents[m.span.sent], m.span);
        let ms = MSpan { span: m.span, head };
        let n = ids.len();
        let e = *ids.entry(m.eid.clone()).or_insert(n);
        if e == entities.len() {
            entities.push(Vec::new());
        }
        entities[e].push(ms);
        mentions.push(ms);
        entity_of.push(e);
    }
    let key = |m: &MSpan| (m.span.sent, m.span.start, std::cmp::Reverse(m.span.end));
    let mut first: HashMap<usize, (usize, usize, std::cmp::Reverse<usize>)> = HashMap::new();
    for (i, m) in mentions.iter().enumerate() {
        let k = key(m);
        let f = first.entry(entity_of[i]).or_insert(k);
        if k < *f {
            *f = k;
        }
    }
    let anaphoric = mentions.iter().enumerate().map(|(i, m)| key(m) != first[&entity_of[i]]).collect();
    Ok(Gold { entities, mentions, entity_of, anaphoric })
}

/// System output on a document.
pub struct Sys {
    pub ms: Vec<Mention>,
    pub clusters: Vec<Vec<usize>>,
    pub links: Vec<Link>,
}

pub fn resolve(doc: &Document, cfg: &Config) -> Sys {
    let mut r = Resolver::new(doc, cfg.clone());
    r.run();
    Sys { clusters: r.clusters(), ms: r.ms.clone(), links: r.links.clone() }
}

fn mspan(doc: &Document, m: &Mention) -> MSpan {
    MSpan { span: m.span, head: corefud::span_head(&doc.sents[m.span.sent], m.span) }
}

/// A mention goes to the output if it is not internal (relative, interrogative — not annotated in GUM) and not a singleton
/// coordination group (a group is a mention only when something refers to it as a whole).
pub fn visible(sys: &Sys, c: &[usize], m: usize) -> bool {
    let x = &sys.ms[m];
    !x.internal && !(x.coord && c.len() == 1 && !std::env::var("COREF_X").is_ok_and(|v| v.contains("coord-sgl")))
}

/// Output entities for the scorer.
pub fn sys_entities(doc: &Document, sys: &Sys) -> Vec<Vec<MSpan>> {
    sys.clusters.iter().map(|c| c.iter().filter(|&&m| visible(sys, c, m)).map(|&m| mspan(doc, &sys.ms[m])).collect::<Vec<_>>()).filter(|e| !e.is_empty()).collect()
}

/// Mentions for CorefUD output: entity number follows cluster order; the note is the id of the sieve that attached the mention.
pub fn out_mentions(sys: &Sys) -> Vec<OutMention> {
    let by: HashMap<usize, &Link> = sys.links.iter().map(|l| (l.from, l)).collect();
    let mut out = Vec::new();
    let mut eid = 0;
    for c in &sys.clusters {
        let vis: Vec<usize> = c.iter().copied().filter(|&m| visible(sys, c, m)).collect();
        if vis.is_empty() {
            continue;
        }
        eid += 1;
        for &m in &vis {
            let note = match by.get(&m) {
                Some(l) => l.sieve.id().to_string(),
                None if vis.len() == 1 => "sgl".to_string(),
                None => "new".to_string(),
            };
            out.push(OutMention { span: sys.ms[m].span, head: sys.ms[m].head, eid, etype: String::new(), note });
        }
    }
    out
}

/// Sum of metrics over documents.
pub fn score_all(gold: &[Gold], pred: &[Vec<Vec<MSpan>>], how: Matching, singletons: bool) -> Scores {
    let mut s = Scores::default();
    for (g, p) in gold.iter().zip(pred) {
        s.add(&score::score_doc(&g.entities, p, how, singletons));
    }
    s
}

/// Link precision of one sieve.
#[derive(Clone, Copy, Debug, Default)]
pub struct LinkStat {
    pub n: usize,
    pub correct: usize,
    /// Anaphor is not a gold mention.
    pub from_spurious: usize,
    /// Antecedent is not a gold mention.
    pub to_spurious: usize,
}

/// Pronouns of one form.
#[derive(Clone, Copy, Debug, Default)]
pub struct PronStat {
    /// Output pronoun mentions that matched gold ones.
    pub n: usize,
    /// Of these, those with a predecessor in the gold chain.
    pub anaphoric: usize,
    /// Resolved by the system.
    pub resolved: usize,
    /// Antecedent in the same gold chain.
    pub correct: usize,
}

/// Links and pronouns of a document against gold.
pub fn link_stats(doc: &Document, gold: &Gold, sys: &Sys, links: &mut BTreeMap<Sieve, LinkStat>, prons: &mut BTreeMap<String, PronStat>, errors: &mut Vec<String>) {
    let pm: Vec<MSpan> = sys.ms.iter().map(|m| mspan(doc, m)).collect();
    let mt = score::match_mentions(&gold.mentions, &pm, Matching::Head);
    let ent = |m: usize| mt[m].map(|g| gold.entity_of[g]);
    let by: HashMap<usize, &Link> = sys.links.iter().map(|l| (l.from, l)).collect();
    for l in &sys.links {
        let st = links.entry(l.sieve).or_default();
        st.n += 1;
        match (ent(l.from), ent(l.to)) {
            (None, _) => st.from_spurious += 1,
            (_, None) => st.to_spurious += 1,
            (Some(a), Some(b)) if a == b => st.correct += 1,
            _ => {
                if errors.len() < 400 {
                    let s = &doc.sents[sys.ms[l.from].span.sent];
                    let t = &doc.sents[sys.ms[l.to].span.sent];
                    let (x, y) = (&sys.ms[l.from], &sys.ms[l.to]);
                    errors.push(format!("{}\t{}\t«{}»\t«{}»\t{}", l.sieve.id(), s.id, s.text(x.span.start, x.span.end), t.text(y.span.start, y.span.end), l.why));
                }
            }
        }
    }
    for (m, x) in sys.ms.iter().enumerate() {
        if !x.is_ppr() || x.person != P3 {
            continue;
        }
        let Some(g) = mt[m] else { continue };
        let st = prons.entry(x.head_low.clone()).or_default();
        st.n += 1;
        st.anaphoric += gold.anaphoric[g] as usize;
        if let Some(l) = by.get(&m) {
            st.resolved += 1;
            if ent(l.to) == Some(gold.entity_of[g]) {
                st.correct += 1;
            }
        }
    }
}

fn pct(x: f64) -> String {
    format!("{:.2}", 100.0 * x)
}

fn ratio(a: usize, b: usize) -> String {
    if b == 0 { "—".into() } else { format!("{:.1}", 100.0 * a as f64 / b as f64) }
}

/// A row of the metrics table.
pub fn row(name: &str, s: &Scores) -> String {
    format!(
        "| {name} | {} | {} / {} / {} | {} / {} / {} | {} / {} / {} | {} |",
        pct(s.conll()),
        pct(s.muc.r()),
        pct(s.muc.p()),
        pct(s.muc.f()),
        pct(s.b3.r()),
        pct(s.b3.p()),
        pct(s.b3.f()),
        pct(s.ceafe.r()),
        pct(s.ceafe.p()),
        pct(s.ceafe.f()),
        pct(s.ment.f())
    )
}

pub const HEADER: &str = "| system | CoNLL F1 | MUC R / P / F1 | B³ R / P / F1 | CEAF-e R / P / F1 | mentions F1 |\n|---|---|---|---|---|---|";

/// Scorer self-check before measuring (fail-fast): gold against itself gives 100 everywhere; the Pradhan et al. 2014 example.
pub fn self_check(gold: &[Gold]) -> Result<()> {
    let key: score::Part = vec![vec![0, 1, 2], vec![3, 4, 5, 6]];
    let resp: score::Part = vec![vec![0, 1], vec![2, 3], vec![5, 6, 7, 8]];
    let s = score::score_parts(&key, &resp);
    let near = |a: f64, b: f64| (a - b).abs() < 1e-9;
    if !(near(s.muc.f(), 0.4) && near(s.b3.r(), 35.0 / 84.0) && near(s.b3.p(), 0.5) && near(s.ceafe.r(), 0.65) && near(s.ceafe.p(), 1.3 / 3.0)) {
        bail!("scorer does not match the Pradhan et al. 2014 example: {s:?}");
    }
    let pred: Vec<Vec<Vec<MSpan>>> = gold.iter().map(|g| g.entities.clone()).collect();
    for sg in [true, false] {
        let s = score_all(gold, &pred, Matching::Head, sg);
        for (n, c) in [("MUC", s.muc), ("B3", s.b3), ("CEAF-e", s.ceafe)] {
            if c.rd > 0.0 && !(near(c.r(), 1.0) && near(c.p(), 1.0)) {
                bail!("gold against itself (singletons={sg}): {n} = {:.4}/{:.4}", c.r(), c.p());
            }
        }
    }
    Ok(())
}

/// Measured system setups: name → config.
pub fn systems() -> Vec<(String, Config)> {
    let mk = |sieves: Vec<Sieve>| Config { sieves, ..Config::default() };
    vec![
        ("all singletons".into(), mk(vec![])),
        ("exact string match only".into(), mk(vec![Sieve::Exact])),
        ("pronoun → nearest agreeing".into(), mk(vec![Sieve::Nearest])),
        ("sieve (all 14)".into(), mk(Sieve::FULL.to_vec())),
        ("sieve (12, default)".into(), Config::default()),
    ]
}

pub struct Input {
    pub label: String,
    pub ud: UdDoc,
    pub docs: Vec<Document>,
}

/// Full measurement for one set of trees: Markdown tables go to `report`, CoNLL-U output of the full sieve to `out_conllu`.
pub fn run(label: &str, gold_docs: &[Document], input: &Input, report: &mut String, out_conllu: Option<&std::path::Path>, errors_path: Option<&std::path::Path>) -> Result<()> {
    let gold: Vec<Gold> = gold_docs.iter().map(gold_of).collect::<Result<_>>()?;
    self_check(&gold)?;
    if input.docs.len() != gold.len() {
        bail!("documents: gold {}, input {}", gold.len(), input.docs.len());
    }
    let docs = &input.docs;
    writeln!(report, "\n## {label}\n")?;
    let ng: usize = gold.iter().map(|g| g.mentions.len()).sum();
    let ne: usize = gold.iter().map(|g| g.entities.len()).sum();
    let nns: usize = gold.iter().map(|g| g.entities.iter().filter(|e| e.len() > 1).count()).sum();
    writeln!(report, "Documents {}, gold mentions {ng}, entities {ne} (non-singleton {nns}).\n", docs.len())?;
    for (sg, title) in [(true, "with singletons"), (false, "without singletons (main CRAC metric)")] {
        writeln!(report, "### Baselines and sieve — head matching, {title}\n\n{HEADER}")?;
        for (name, cfg) in systems() {
            let pred: Vec<Vec<Vec<MSpan>>> = docs.iter().map(|d| sys_entities(d, &resolve(d, &cfg))).collect();
            writeln!(report, "{}", row(&name, &score_all(&gold, &pred, Matching::Head, sg)))?;
        }
        let cfg = Config::default();
        let pred: Vec<Vec<Vec<MSpan>>> = docs.iter().map(|d| sys_entities(d, &resolve(d, &cfg))).collect();
        writeln!(report, "{}", row("sieve (12) — exact span (for reference)", &score_all(&gold, &pred, Matching::Exact, sg)))?;
        writeln!(report)?;
    }
    // sieve contributions: cumulative
    writeln!(report, "### Sieve contributions (cumulative, head matching)\n\n| + sieve | CoNLL F1 with singletons | Δ | CoNLL F1 without singletons | Δ |\n|---|---|---|---|---|")?;
    let (mut prev_a, mut prev_b) = (0.0, 0.0);
    for k in 0..=Sieve::FULL.len() {
        let cfg = Config { sieves: Sieve::FULL[..k].to_vec(), ..Config::default() };
        let pred: Vec<Vec<Vec<MSpan>>> = docs.iter().map(|d| sys_entities(d, &resolve(d, &cfg))).collect();
        let a = score_all(&gold, &pred, Matching::Head, true).conll();
        let b = score_all(&gold, &pred, Matching::Head, false).conll();
        let name = if k == 0 { "(none)".to_string() } else { Sieve::FULL[k - 1].id().to_string() };
        if k == 0 {
            writeln!(report, "| {name} | {} | | {} | |", pct(a), pct(b))?;
        } else {
            writeln!(report, "| {name} | {} | {:+.2} | {} | {:+.2} |", pct(a), 100.0 * (a - prev_a), pct(b), 100.0 * (b - prev_b))?;
        }
        (prev_a, prev_b) = (a, b);
    }
    // sieve precision — on the full sieve (all 14, so the disabled ones show too); pronouns — on the default sieve and the baseline
    let mut errors = Vec::new();
    for (name, cfg, sieve_table) in [
        ("sieve (all 14)", Config { sieves: Sieve::FULL.to_vec(), ..Config::default() }, true),
        ("sieve (12, default)", Config::default(), false),
        ("pronoun → nearest agreeing", Config { sieves: vec![Sieve::Nearest], ..Config::default() }, false),
    ] {
        let mut links: BTreeMap<Sieve, LinkStat> = BTreeMap::new();
        let mut prons: BTreeMap<String, PronStat> = BTreeMap::new();
        let mut errs = Vec::new();
        for (d, g) in docs.iter().zip(&gold) {
            let sys = resolve(d, &cfg);
            link_stats(d, g, &sys, &mut links, &mut prons, &mut errs);
        }
        if sieve_table {
            errors = errs;
            writeln!(report, "\n### Precision of each sieve ({name})\n\nA link is correct if both mentions are in gold (by head) and in the same chain. relpron is not evaluated: relative pronouns are not annotated in GUM (the anaphor is always \"outside gold\").\n\n| sieve | what it does | links | correct | precision % | anaphor outside gold | antecedent outside gold |\n|---|---|---|---|---|---|---|")?;
            for (s, st) in &links {
                let p = if *s == Sieve::Relpron { "n/a".to_string() } else { ratio(st.correct, st.n) };
                writeln!(report, "| {} | {} | {} | {} | {} | {} | {} |", s.id(), s.about(), st.n, st.correct, p, st.from_spurious, st.to_spurious)?;
            }
            let tot = links.iter().filter(|(s, _)| **s != Sieve::Relpron).fold(LinkStat::default(), |a, (_, b)| LinkStat { n: a.n + b.n, correct: a.correct + b.correct, from_spurious: a.from_spurious + b.from_spurious, to_spurious: a.to_spurious + b.to_spurious });
            writeln!(report, "| **total (without relpron)** | | {} | {} | {} | {} | {} |", tot.n, tot.correct, ratio(tot.correct, tot.n), tot.from_spurious, tot.to_spurious)?;
            continue;
        }
        writeln!(report, "\n### 3rd-person pronouns by form ({name})\n\nPrecision is the share of resolved pronouns whose antecedent is in the same gold chain; recall is the correct ones among those with a predecessor in gold.\n\n| form | in gold | with predecessor | resolved | correct | precision % | recall % |\n|---|---|---|---|---|---|---|")?;
        let mut v: Vec<(&String, &PronStat)> = prons.iter().collect();
        v.sort_by_key(|(f, s)| (std::cmp::Reverse(s.n), (*f).clone()));
        let mut tot = PronStat::default();
        for (f, s) in v {
            if s.n >= 3 {
                writeln!(report, "| {f} | {} | {} | {} | {} | {} | {} |", s.n, s.anaphoric, s.resolved, s.correct, ratio(s.correct, s.resolved), ratio(s.correct, s.anaphoric))?;
            }
            tot.n += s.n;
            tot.anaphoric += s.anaphoric;
            tot.resolved += s.resolved;
            tot.correct += s.correct;
        }
        writeln!(report, "| **total** | {} | {} | {} | {} | {} | {} |", tot.n, tot.anaphoric, tot.resolved, tot.correct, ratio(tot.correct, tot.resolved), ratio(tot.correct, tot.anaphoric))?;
    }
    if let Some(p) = errors_path {
        std::fs::write(p, format!("sieve\tsentence\tanaphor\tantecedent\tevidence\n{}\n", errors.join("\n")))?;
    }
    // CoNLL-U output of the full sieve, and a control: the resolver does not see gold Entity
    let cfg = Config::default();
    let outs: Vec<Vec<OutMention>> = docs.iter().map(|d| out_mentions(&resolve(d, &cfg))).collect();
    let mut stripped = input.ud.clone();
    corefud::strip(&mut stripped);
    let sdocs = crate::doc::documents(&stripped)?;
    let outs2: Vec<Vec<OutMention>> = sdocs.iter().map(|d| out_mentions(&resolve(d, &cfg))).collect();
    let same = outs.len() == outs2.len() && outs.iter().zip(&outs2).all(|(a, b)| a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.span == y.span && x.eid == y.eid && x.note == y.note));
    if !same {
        bail!("output depends on gold Entity in MISC — gold leak");
    }
    if let Some(p) = out_conllu {
        let mut ud = input.ud.clone();
        corefud::write(&mut ud, docs, &outs)?;
        std::fs::write(p, ud.text())?;
    }
    Ok(())
}
