//! Report of the document-context experiment (`en ctx-report`): test-set annotation by several systems (each
//! from several seeds, `EN_PRED_DIR` files from `en ud-eval`) against the gold standard.
//! - CoNLL 2018 metrics (UPOS, UFeats, LAS, MLAS): mean ± spread over seeds;
//! - paired bootstrap over sentences: 95% interval of the "system − baseline" difference (mean over seeds);
//! - where the difference comes from: by gold relation and UPOS, by sentence type and genre;
//! - sentences the system fixed and broke the most relative to the baseline.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};

use crate::conllu::{self, Sentence};
use crate::ctx::Ctx;
use crate::gram::Rel;
use crate::hash::FastMap;
use crate::ud::{Scores, Word};

/// One system: name and test-set annotation from each seed.
pub struct System {
    pub name: String,
    pub runs: Vec<Vec<Sentence>>,
}

impl System {
    pub fn load(name: &str, files: &[PathBuf], gold: &[Sentence]) -> Result<System> {
        let mut runs = Vec::new();
        for f in files {
            let v = conllu::read(f)?;
            if v.len() != gold.len() {
                bail!("{}: {} sentences, but the gold has {}", f.display(), v.len(), gold.len());
            }
            for (a, b) in v.iter().zip(gold) {
                if a.id != b.id || a.tokens.len() != b.tokens.len() {
                    bail!("{}: sentence {} does not match the gold ({})", f.display(), a.id, b.id);
                }
            }
            runs.push(v);
        }
        if runs.is_empty() {
            bail!("system {name}: no files");
        }
        Ok(System { name: name.to_string(), runs })
    }
}

fn words(s: &Sentence) -> Vec<Word> {
    s.tokens.iter().map(|t| Word { upos: t.upos, tag: t.tag, feats: t.feats, lemma: t.lemma.clone(), head: t.head, rel: t.rel }).collect()
}

/// Counters of each sentence for each seed of a system.
fn per_sentence(gold: &[Sentence], sys: &System) -> Vec<Vec<Scores>> {
    sys.runs
        .iter()
        .map(|run| {
            gold.iter()
                .zip(run)
                .map(|(g, p)| {
                    let mut sc = Scores::default();
                    sc.add(&words(g), &words(p), g.has_feats, g.has_lemmas);
                    sc
                })
                .collect()
        })
        .collect()
}

fn total(v: &[Scores]) -> Scores {
    let mut t = Scores::default();
    for x in v {
        t += x;
    }
    t
}

fn mean_sd(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64;
    let m = v.iter().sum::<f64>() / n;
    let sd = if v.len() > 1 { (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (n - 1.0)).sqrt() } else { 0.0 };
    (m, sd)
}

/// System metrics: mean over seeds of the four metrics on the sentence sample `idx` (with repeats).
fn metrics_on(ps: &[Vec<Scores>], idx: &[usize]) -> [f64; 4] {
    let mut acc = [0.0f64; 4];
    for seed in ps {
        let mut t = Scores::default();
        for &i in idx {
            t += &seed[i];
        }
        for (a, x) in acc.iter_mut().zip(t.main4()) {
            *a += x;
        }
    }
    acc.map(|a| a / ps.len() as f64)
}

/// Paired bootstrap over sentences: 95% intervals of the "system − baseline" difference for the four metrics.
fn bootstrap(base: &[Vec<Scores>], sys: &[Vec<Scores>], n: usize, b: usize, seed: u64) -> [(f64, f64); 4] {
    let mut rng = crate::ctx::Rng::new(seed);
    let mut diffs: [Vec<f64>; 4] = Default::default();
    let mut idx = vec![0usize; n];
    for _ in 0..b {
        for x in idx.iter_mut() {
            *x = rng.below(n);
        }
        let (a, s) = (metrics_on(base, &idx), metrics_on(sys, &idx));
        for k in 0..4 {
            diffs[k].push(s[k] - a[k]);
        }
    }
    let q = |v: &mut Vec<f64>| {
        v.sort_by(|a, b| a.total_cmp(b));
        let lo = v[((v.len() as f64) * 0.025).floor() as usize];
        let hi = v[(((v.len() as f64) * 0.975).ceil() as usize).min(v.len()) - 1];
        (lo, hi)
    };
    let mut out = [(0.0, 0.0); 4];
    for k in 0..4 {
        out[k] = q(&mut diffs[k]);
    }
    out
}

/// Correct attachments (head and universal relation) and UPOS of each token for each seed.
fn token_hits(gold: &[Sentence], sys: &System) -> Vec<Vec<Vec<(bool, bool)>>> {
    sys.runs
        .iter()
        .map(|run| gold.iter().zip(run).map(|(g, p)| g.tokens.iter().zip(&p.tokens).map(|(a, b)| (a.head == b.head && a.rel.base() == b.rel.base(), a.upos == b.upos)).collect()).collect())
        .collect()
}

/// Mean over seeds of correct attachments and UPOS in token groups (key taken from the gold).
fn by_token_key<K: std::hash::Hash + Eq + Clone>(gold: &[Sentence], hits: &[Vec<Vec<(bool, bool)>>], key: impl Fn(usize, usize) -> K) -> FastMap<K, (usize, f64, f64)> {
    let mut m: FastMap<K, (usize, f64, f64)> = FastMap::default();
    let r = hits.len() as f64;
    for (si, g) in gold.iter().enumerate() {
        for ti in 0..g.tokens.len() {
            let e = m.entry(key(si, ti)).or_insert((0, 0.0, 0.0));
            e.0 += 1;
            for run in hits {
                let (l, u) = run[si][ti];
                e.1 += l as u8 as f64 / r;
                e.2 += u as u8 as f64 / r;
            }
        }
    }
    m
}

/// Report: the baseline is the first system; breakdown and examples are for `focus` against the baseline.
pub fn report(gold_path: &Path, systems: &[System], focus: usize, boot: usize) -> Result<String> {
    let gold = conllu::read(gold_path)?;
    let doc = conllu::Doc::read(gold_path)?;
    let (ctx, _) = crate::ctx::gold(&doc, false);
    if ctx.len() != gold.len() {
        bail!("{}: context for {} sentences, but there are {} sentences", gold_path.display(), ctx.len(), gold.len());
    }
    let n = gold.len();
    let ps: Vec<Vec<Vec<Scores>>> = systems.iter().map(|s| per_sentence(&gold, s)).collect();
    let mut out = String::new();
    let tokens: usize = gold.iter().map(|s| s.tokens.len()).sum();
    writeln!(out, "gold: {} — {n} sentences, {tokens} tokens; bootstrap: {boot} sentence resamples\n", gold_path.display())?;

    writeln!(out, "## Systems (mean ± std. deviation over seeds)\n")?;
    writeln!(out, "| system | seeds | UPOS | UFeats | LAS | MLAS | LAS per seed |")?;
    writeln!(out, "|---|---:|---:|---:|---:|---:|---|")?;
    for (s, p) in systems.iter().zip(&ps) {
        let per: Vec<[f64; 4]> = p.iter().map(|seed| total(seed).main4()).collect();
        let cells: Vec<String> = (0..4)
            .map(|k| {
                let (m, sd) = mean_sd(&per.iter().map(|x| x[k]).collect::<Vec<_>>());
                format!("{m:.2} ± {sd:.2}")
            })
            .collect();
        let las: Vec<String> = per.iter().map(|x| format!("{:.2}", x[2])).collect();
        writeln!(out, "| {} | {} | {} | {} |", s.name, p.len(), cells.join(" | "), las.join(" / "))?;
    }

    writeln!(out, "\n## Difference from baseline '{}': mean over seeds, 95% paired bootstrap interval\n", systems[0].name)?;
    writeln!(out, "| system | ΔUPOS | ΔUFeats | ΔLAS | ΔMLAS |")?;
    writeln!(out, "|---|---:|---:|---:|---:|")?;
    let all: Vec<usize> = (0..n).collect();
    let base_m = metrics_on(&ps[0], &all);
    for (k, s) in systems.iter().enumerate().skip(1) {
        let m = metrics_on(&ps[k], &all);
        let ci = bootstrap(&ps[0], &ps[k], n, boot, 0xB007 + k as u64);
        let cells: Vec<String> = (0..4).map(|j| format!("{:+.2} [{:+.2}, {:+.2}]", m[j] - base_m[j], ci[j].0, ci[j].1)).collect();
        writeln!(out, "| {} | {} |", s.name, cells.join(" | "))?;
    }

    if focus == 0 || focus >= systems.len() {
        return Ok(out);
    }
    let (b, f) = (&systems[0], &systems[focus]);
    let (hb, hf) = (token_hits(&gold, b), token_hits(&gold, f));
    writeln!(out, "\n## Where the difference comes from: '{}' vs '{}'\n", f.name, b.name)?;
    writeln!(out, "Contribution — change in whole-test LAS (pts) from this token group: (correct in system − in baseline) / all tokens.\n")?;

    // by gold relation
    let rel_key = |si: usize, ti: usize| gold[si].tokens[ti].rel.base();
    let (mb, mf) = (by_token_key(&gold, &hb, rel_key), by_token_key(&gold, &hf, rel_key));
    let mut rows: Vec<(Rel, usize, f64, f64)> = mb.iter().map(|(r, &(c, l, _))| (*r, c, l, mf.get(r).map_or(0.0, |x| x.1))).collect();
    rows.sort_by(|a, b| (b.3 - b.2).abs().total_cmp(&(a.3 - a.2).abs()).then(a.0.cmp(&b.0)));
    writeln!(out, "### By gold relation (top 15 by contribution)\n")?;
    writeln!(out, "| relation | tokens | LAS baseline | LAS system | Δ | contribution, pts |")?;
    writeln!(out, "|---|---:|---:|---:|---:|---:|")?;
    for (r, c, l, lf) in rows.iter().take(15) {
        writeln!(out, "| {r} | {c} | {:.2} | {:.2} | {:+.2} | {:+.3} |", 100.0 * l / *c as f64, 100.0 * lf / *c as f64, 100.0 * (lf - l) / *c as f64, 100.0 * (lf - l) / tokens as f64)?;
    }

    // by gold UPOS
    let upos_key = |si: usize, ti: usize| gold[si].tokens[ti].upos;
    let (ub, uf) = (by_token_key(&gold, &hb, upos_key), by_token_key(&gold, &hf, upos_key));
    let mut rows: Vec<_> = ub.iter().map(|(u, &(c, l, p))| (*u, c, l, p, uf.get(u).map_or((0.0, 0.0), |x| (x.1, x.2)))).collect();
    rows.sort_by(|a, b| (b.4.0 - b.2).abs().total_cmp(&(a.4.0 - a.2).abs()).then(a.0.cmp(&b.0)));
    writeln!(out, "\n### By gold UPOS\n")?;
    writeln!(out, "| UPOS | tokens | UPOS baseline | UPOS system | LAS baseline | LAS system | LAS contribution, pts |")?;
    writeln!(out, "|---|---:|---:|---:|---:|---:|---:|")?;
    for (u, c, l, p, (lf, pf)) in &rows {
        let name = u.map_or("_", |x| x.name());
        let c = *c as f64;
        writeln!(out, "| {name} | {c} | {:.2} | {:.2} | {:.2} | {:.2} | {:+.3} |", 100.0 * p / c, 100.0 * pf / c, 100.0 * l / c, 100.0 * lf / c, 100.0 * (lf - l) / tokens as f64)?;
    }

    // by sentence type and genre: LAS and MLAS of sentence groups
    let group_table = |out: &mut String, title: &str, key: &dyn Fn(&Ctx) -> String| -> Result<()> {
        let mut groups: FastMap<String, Vec<usize>> = FastMap::default();
        for (i, c) in ctx.iter().enumerate() {
            groups.entry(key(c)).or_default().push(i);
        }
        let mut g: Vec<(String, Vec<usize>)> = groups.into_iter().collect();
        g.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(&b.0)));
        writeln!(out, "\n### {title}\n")?;
        writeln!(out, "| group | sentences | tokens | LAS baseline | LAS system | ΔLAS | MLAS baseline | MLAS system | ΔMLAS | LAS contribution, pts |")?;
        writeln!(out, "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|")?;
        for (name, idx) in &g {
            let (a, s) = (metrics_on(&ps[0], idx), metrics_on(&ps[focus], idx));
            let toks: usize = idx.iter().map(|&i| gold[i].tokens.len()).sum();
            writeln!(
                out,
                "| {name} | {} | {toks} | {:.2} | {:.2} | {:+.2} | {:.2} | {:.2} | {:+.2} | {:+.3} |",
                idx.len(),
                a[2],
                s[2],
                s[2] - a[2],
                a[3],
                s[3],
                s[3] - a[3],
                (s[2] - a[2]) * toks as f64 / tokens as f64
            )?;
        }
        Ok(())
    };
    group_table(&mut out, "By sentence type (gold s_type)", &|c: &Ctx| c.stype.map_or("—".into(), |x| x.name().to_string()))?;
    group_table(&mut out, "By genre", &|c: &Ctx| c.genre.map_or("—".into(), |x| x.name().to_string()))?;

    // fixed and broken sentences: mean number of LAS errors over seeds
    let errs = |h: &[Vec<Vec<(bool, bool)>>], si: usize| h.iter().map(|run| run[si].iter().filter(|x| !x.0).count() as f64).sum::<f64>() / h.len() as f64;
    let mut delta: Vec<(f64, usize)> = (0..n).map(|i| (errs(&hf, i) - errs(&hb, i), i)).collect();
    delta.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    let show = |out: &mut String, title: &str, items: &mut dyn Iterator<Item = &(f64, usize)>| -> Result<()> {
        writeln!(out, "\n### {title}\n")?;
        for &(d, i) in items {
            let g = &gold[i];
            let c = &ctx[i];
            writeln!(
                out,
                "- **{}** ({}, {}; {} tok.): LAS errors {:.1} → {:.1} (Δ {:+.1}). \"{}\"",
                g.id,
                c.genre.map_or("—", |x| x.name()),
                c.stype.map_or("—", |x| x.name()),
                g.tokens.len(),
                errs(&hb, i),
                errs(&hf, i),
                d,
                g.text
            )?;
            // changes in the first seed: word — baseline → system (gold)
            let (pb, pf) = (&b.runs[0][i], &f.runs[0][i]);
            let mut ch = Vec::new();
            for (k, t) in g.tokens.iter().enumerate() {
                let (x, y) = (&pb.tokens[k], &pf.tokens[k]);
                if (x.head, x.rel) != (y.head, y.rel) {
                    let w = |h: usize| if h == 0 { "ROOT".to_string() } else { g.tokens[h - 1].form.clone() };
                    ch.push(format!("{}: {}→{} ⇒ {}→{} (gold {}→{})", t.form, x.rel, w(x.head), y.rel, w(y.head), t.rel, w(t.head)));
                }
            }
            if !ch.is_empty() {
                writeln!(out, "  - seed 0: {}", ch.iter().take(5).cloned().collect::<Vec<_>>().join("; "))?;
            }
        }
        Ok(())
    };
    show(&mut out, "10 sentences the context fixed the most", &mut delta.iter().filter(|x| x.0 < 0.0).take(10))?;
    show(&mut out, "10 sentences the context broke the most", &mut delta.iter().rev().filter(|x| x.0 > 0.0).take(10))?;
    let (fixed, broken) = (delta.iter().filter(|x| x.0 < 0.0).count(), delta.iter().filter(|x| x.0 > 0.0).count());
    writeln!(out, "\nsentences with fewer LAS errors: {fixed}, more: {broken}, unchanged: {}"
, n - fixed - broken)?;
    Ok(out)
}
