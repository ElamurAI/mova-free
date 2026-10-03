//! Evaluation of the pragmatics SLM (small language model): features (`feats`), three models behind a shared interface (`model`), 80/20 split —
//! tales by document, Tatoeba by sentence. For each field:
//! - accuracy and macro-F1 against held-out silver, baseline — majority class of the training set;
//! - negative control — the same models on shuffled labels must score at baseline level;
//! - agreement of perceptron and memory, with accuracy separately on agreeing and disagreeing cases (disagreement — a signal to ask the LLM);
//! - speed: training, prediction per sentence, model size;
//! - confusion matrix for `act`; `form` vs `s_type` on GUM test — evaluation only (GUM is NC).
//! Predictions for the database are cross-validated (5 folds by document): each sentence is predicted by a model that has not
//! seen it: `rust-<model>.tsv` with a decision trace.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use en::annotate::Annotator;
use serde_json::json;

use crate::data::{self, Item, Src, h64};
use crate::feats::{self, Cues, Dict, Row, prev_slot};
use crate::model::{self, Classifier, Kind, Pred, Train};
use crate::schema::{Act, Field, Labels};
use crate::silver::read_silver;

/// Models of one type — one per field (order of `Field::ALL`).
pub struct Set {
    pub kind: Kind,
    pub models: Vec<Box<dyn Classifier>>,
    pub train_secs: f64,
}

/// Prediction of all fields of a sentence.
#[derive(Clone)]
pub struct Out {
    pub class: [usize; 6],
    pub preds: Vec<Pred>,
}

/// Training rows: features with the previous sentence's act taken from silver.
fn train_rows(rows: &[Row], prev: &[Option<usize>], gold: &HashMap<usize, Labels>, idx: &[usize]) -> Vec<Row> {
    let ps = prev_slot();
    idx.iter()
        .map(|&i| {
            let mut r = rows[i].clone();
            r[ps] = prev[i].and_then(|p| gold.get(&p)).map(|l| l.act.name().to_string()).unwrap_or_else(|| "start".into());
            r
        })
        .collect()
}

/// Train all fields of one type.
pub fn fit_set(kind: Kind, x: &[Vec<u32>], ys: &[Vec<usize>], keys: &[String], dict: &Dict, seed: u64) -> Set {
    let t0 = Instant::now();
    let names: Vec<Vec<&str>> = Field::ALL.iter().map(|f| f.classes()).collect();
    let models = Field::ALL
        .iter()
        .enumerate()
        .map(|(k, _)| model::fit(kind, &Train { x, y: &ys[k], names: &names[k], keys, dict, seed }))
        .collect();
    Set { kind, models, train_secs: t0.elapsed().as_secs_f64() }
}

/// Prediction in sentence order: the previous sentence's act is the model's own prediction.
pub fn predict_seq(set: &Set, rows: &[Row], prev: &[Option<usize>], idx: &[usize], dict: &Dict) -> HashMap<usize, Out> {
    let ps = prev_slot();
    let mut out: HashMap<usize, Out> = HashMap::new();
    for &i in idx {
        let pa = prev[i].and_then(|p| out.get(&p)).map(|o| Act::ALL[o.class[1]].name()).unwrap_or("start");
        let mut x = dict.encode(&rows[i]);
        x[ps] = dict.id(ps, pa);
        let preds: Vec<Pred> = set.models.iter().map(|m| m.predict(&x)).collect();
        let mut class = [0usize; 6];
        for (k, p) in preds.iter().enumerate() {
            class[k] = p.class;
        }
        out.insert(i, Out { class, preds });
    }
    out
}

/// Accuracy and macro-F1 (classes — union of those present in gold and prediction).
pub fn scores(gold: &[usize], pred: &[usize], nclass: usize) -> (f64, f64) {
    let n = gold.len().max(1) as f64;
    let acc = gold.iter().zip(pred).filter(|(a, b)| a == b).count() as f64 / n;
    let mut f1s = Vec::new();
    for c in 0..nclass {
        let tp = gold.iter().zip(pred).filter(|&(&g, &p)| g == c && p == c).count() as f64;
        let gp = gold.iter().filter(|&&g| g == c).count() as f64;
        let pp = pred.iter().filter(|&&p| p == c).count() as f64;
        if gp + pp == 0.0 {
            continue;
        }
        let f = if tp == 0.0 { 0.0 } else { 2.0 * tp / (gp + pp) };
        f1s.push(f);
    }
    (acc, if f1s.is_empty() { 0.0 } else { f1s.iter().sum::<f64>() / f1s.len() as f64 })
}

fn pct(x: f64) -> String {
    format!("{:.1}", 100.0 * x)
}

/// Label permutation (seeded) — negative control.
fn shuffle(y: &[usize], seed: u64) -> Vec<usize> {
    let mut o: Vec<usize> = (0..y.len()).collect();
    o.sort_by_key(|&i| h64(&i.to_string(), seed));
    o.iter().map(|&i| y[i]).collect()
}

/// Decision trace for the database: {"form": {"v": "q", "conf": 0.93, "why": [["w1=can", 1.2], …]}, …}.
fn trace(o: &Out) -> String {
    let mut m = serde_json::Map::new();
    for (k, f) in Field::ALL.iter().enumerate() {
        let p = &o.preds[k];
        let why: Vec<serde_json::Value> = p.why.iter().map(|(s, w)| json!([s, (w * 1000.0).round() / 1000.0])).collect();
        m.insert(f.name().into(), json!({"v": f.classes()[p.class], "conf": (p.conf * 1000.0).round() / 1000.0, "why": why}));
    }
    serde_json::Value::Object(m).to_string()
}

fn labels_of(o: &Out) -> Labels {
    let mut l = Labels { means: String::new(), ..Labels::default() };
    for (k, f) in Field::ALL.iter().enumerate() {
        f.set(&mut l, o.class[k]);
    }
    l
}

/// GUM test sentences with `# s_type` (only for evaluating form): document, paragraphs, order.
fn read_gum(path: &Path) -> Result<Vec<(Item, String)>> {
    let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    let (mut doc, mut para, mut ord) = (String::new(), 0i64, 0i64);
    let mut out: Vec<(Item, String)> = Vec::new();
    let (mut s_type, mut sid, mut txt, mut newpar) = (String::new(), String::new(), String::new(), false);
    for l in text.lines() {
        if let Some(v) = l.strip_prefix("# newdoc id = ") {
            doc = v.trim().to_string();
            para = 0;
            ord = 0;
        } else if l.starts_with("# newpar") {
            newpar = true;
        } else if let Some(v) = l.strip_prefix("# s_type = ") {
            s_type = v.trim().to_string();
        } else if let Some(v) = l.strip_prefix("# sent_id = ") {
            sid = v.trim().to_string();
        } else if let Some(v) = l.strip_prefix("# text = ") {
            txt = v.trim().to_string();
        } else if l.is_empty() && !sid.is_empty() {
            if newpar || para == 0 {
                para += 1;
                newpar = false;
            }
            ord += 1;
            out.push((Item { key: sid.clone(), src: Src::Gum, doc: doc.clone(), title: String::new(), para, ord, pos: 0, plen: 1, text: txt.clone(), batch: 0, ctx: Vec::new() }, s_type.clone()));
            sid.clear();
            s_type.clear();
        }
    }
    // position in paragraph
    let mut k = 0;
    while k < out.len() {
        let mut e = k;
        while e < out.len() && out[e].0.doc == out[k].0.doc && out[e].0.para == out[k].0.para {
            e += 1;
        }
        for (j, x) in out[k..e].iter_mut().enumerate() {
            x.0.pos = j;
            x.0.plen = e - k;
        }
        k = e;
    }
    Ok(out)
}

/// GUM `s_type` → our form; `multiple` — excluded from comparison.
pub fn gum_form(s: &str) -> Option<&'static str> {
    Some(match s {
        "decl" => "decl",
        "q" => "q",
        "wh" => "wh",
        "imp" => "imp",
        "intj" => "intj",
        "frag" | "ger" | "inf" | "sub" => "frag",
        "other" => "other",
        _ => return None,
    })
}

pub fn run(dir: &Path, model_path: &Path, gum: Option<&Path>, seed: u64) -> Result<()> {
    let items = data::read_items(&dir.join("items.jsonl"))?;
    let silver = read_silver(&dir.join("silver.tsv"))?;
    let lab: Vec<usize> = (0..items.len()).filter(|&i| silver.contains_key(&items[i].key)).collect();
    if lab.is_empty() {
        bail!("no silver");
    }
    // commercial license gate: training only on tales and Tatoeba CC0
    let srcs: Vec<Src> = lab.iter().map(|&i| items[i].src).collect();
    data::license_gate(&srcs)?;
    let gold: HashMap<usize, Labels> = lab.iter().map(|&i| (i, silver[&items[i].key].clone())).collect();
    let t0 = Instant::now();
    let a = Annotator::load(model_path)?;
    let load_secs = t0.elapsed().as_secs_f64();
    let cues = Cues::load();
    let t0 = Instant::now();
    let rows = feats::extract(&a, &cues, &items);
    let feat_ms = 1e3 * t0.elapsed().as_secs_f64() / items.len() as f64;
    let prev = feats::prev_of(&items);
    let is_test = |it: &Item| match it.src {
        Src::Tale => h64(&it.doc, seed) % 5 == 0,
        _ => h64(&it.key, seed) % 5 == 0,
    };
    let train: Vec<usize> = lab.iter().copied().filter(|&i| !is_test(&items[i])).collect();
    let test: Vec<usize> = lab.iter().copied().filter(|&i| is_test(&items[i])).collect();
    let tr_rows = train_rows(&rows, &prev, &gold, &train);
    let dict = Dict::build(&tr_rows);
    let x: Vec<Vec<u32>> = tr_rows.iter().map(|r| dict.encode(r)).collect();
    let keys: Vec<String> = train.iter().map(|&i| items[i].key.clone()).collect();
    let ys: Vec<Vec<usize>> = Field::ALL.iter().map(|f| train.iter().map(|&i| f.get(&gold[&i])).collect()).collect();
    let gy: Vec<Vec<usize>> = Field::ALL.iter().map(|f| test.iter().map(|&i| f.get(&gold[&i])).collect()).collect();

    let mut rep = String::new();
    let _ = writeln!(rep, "# Pragmatics SLM evaluation (seed {seed})\n");
    let _ = writeln!(
        rep,
        "Silver: {} sentences (tales {}, Tatoeba {}); train {} / test {} (tales split by document, Tatoeba by sentence).",
        lab.len(),
        srcs.iter().filter(|s| **s == Src::Tale).count(),
        srcs.iter().filter(|s| **s == Src::Tatoeba).count(),
        train.len(),
        test.len()
    );
    let _ = writeln!(rep, "Features: {} slots, {} values in the dictionary; extraction (en: tokens, UD tree, cues) — {feat_ms:.2} ms per sentence; the en model loads in {load_secs:.2} s.\n", feats::SLOTS.len(), dict.values.iter().map(Vec::len).sum::<usize>());

    // label distribution
    let _ = writeln!(rep, "## Silver label distribution\n");
    for (k, f) in Field::ALL.iter().enumerate() {
        let names = f.classes();
        let mut c = vec![0usize; names.len()];
        for i in &lab {
            c[f.get(&gold[i])] += 1;
        }
        let mut v: Vec<(usize, &str)> = c.iter().copied().zip(names.iter().copied()).filter(|x| x.0 > 0).collect();
        v.sort_by(|a, b| b.0.cmp(&a.0));
        let _ = writeln!(rep, "- **{}**: {}", f.name(), v.iter().map(|(n, s)| format!("{s} {n} ({:.1}%)", 100.0 * *n as f64 / lab.len() as f64)).collect::<Vec<_>>().join(", "));
        let _ = k;
    }
    let _ = writeln!(rep);

    // training and prediction
    let sets: Vec<Set> = Kind::ALL.iter().map(|&k| fit_set(k, &x, &ys, &keys, &dict, seed)).collect();
    let mut outs: Vec<HashMap<usize, Out>> = Vec::new();
    let mut pred_ms: Vec<f64> = Vec::new();
    for s in &sets {
        let t0 = Instant::now();
        outs.push(predict_seq(s, &rows, &prev, &test, &dict));
        pred_ms.push(1e3 * t0.elapsed().as_secs_f64() / test.len().max(1) as f64);
    }
    // negative control: shuffled training labels
    let ys_shuf: Vec<Vec<usize>> = ys.iter().enumerate().map(|(k, y)| shuffle(y, seed ^ (0xC0 + k as u64))).collect();
    let ctrl: Vec<HashMap<usize, Out>> = Kind::ALL.iter().map(|&k| predict_seq(&fit_set(k, &x, &ys_shuf, &keys, &dict, seed), &rows, &prev, &test, &dict)).collect();

    let _ = writeln!(rep, "## Results on held-out silver (test: {} sentences)\n", test.len());
    let _ = writeln!(rep, "Accuracy / macro-F1, %. Baseline — majority class of the training set. Control — the same model on shuffled labels.\n");
    let _ = writeln!(rep, "| field | baseline | perceptron | IGTree | k-NN | control: perceptron | IGTree | k-NN |");
    let _ = writeln!(rep, "|---|---|---|---|---|---|---|---|");
    let mut results = serde_json::Map::new();
    for (k, f) in Field::ALL.iter().enumerate() {
        let nc = f.classes().len();
        let maj = (0..nc).max_by_key(|&c| ys[k].iter().filter(|&&y| y == c).count()).unwrap();
        let base = scores(&gy[k], &vec![maj; gy[k].len()], nc);
        let got: Vec<(f64, f64)> = outs.iter().map(|o| scores(&gy[k], &test.iter().map(|i| o[i].class[k]).collect::<Vec<_>>(), nc)).collect();
        let neg: Vec<(f64, f64)> = ctrl.iter().map(|o| scores(&gy[k], &test.iter().map(|i| o[i].class[k]).collect::<Vec<_>>(), nc)).collect();
        let cell = |x: (f64, f64)| format!("{} / {}", pct(x.0), pct(x.1));
        let _ = writeln!(
            rep,
            "| {} | {} ({}) | {} | {} | {} | {} | {} | {} |",
            f.name(),
            cell(base),
            f.classes()[maj],
            cell(got[0]),
            cell(got[1]),
            cell(got[2]),
            cell(neg[0]),
            cell(neg[1]),
            cell(neg[2])
        );
        results.insert(
            f.name().into(),
            json!({"base": [base.0, base.1], "majority": f.classes()[maj], "perceptron": [got[0].0, got[0].1], "igtree": [got[1].0, got[1].1], "knn": [got[2].0, got[2].1],
                   "control": {"perceptron": [neg[0].0, neg[0].1], "igtree": [neg[1].0, neg[1].1], "knn": [neg[2].0, neg[2].1]}}),
        );
    }

    // speed and size
    let _ = writeln!(rep, "\n## Speed and size (all 6 fields together)\n");
    let _ = writeln!(rep, "| model | training, s | prediction, ms per sentence | size, KB | note |");
    let _ = writeln!(rep, "|---|---|---|---|---|");
    let mut speed = serde_json::Map::new();
    for (s, ms) in sets.iter().zip(&pred_ms) {
        let bytes: usize = s.models.iter().map(|m| m.bytes()).sum();
        let info: Vec<String> = Field::ALL.iter().zip(&s.models).map(|(f, m)| format!("{} {}", f.name(), m.info())).collect();
        let note = match s.kind {
            Kind::Perceptron => format!("12 epochs, f32 weights; {}", info.join(", ")),
            Kind::IgTree => format!("nodes: {}", info.join(", ")),
            Kind::Knn => format!("full memory, k (leave-one-out): {}", info.join(", ")),
        };
        let _ = writeln!(rep, "| {} | {:.3} | {:.3} | {:.1} | {note} |", s.kind.name(), s.train_secs, ms, bytes as f64 / 1024.0);
        speed.insert(s.kind.name().into(), json!({"train_secs": s.train_secs, "predict_ms": ms, "bytes": bytes}));
    }
    let _ = writeln!(rep, "\nFeature extraction (shared by all: tokenizer, tagger, en parser, cues): {feat_ms:.2} ms per sentence.");

    // model agreement
    let _ = writeln!(rep, "\n## Perceptron and memory agreement (test)\n");
    let _ = writeln!(rep, "Share of sentences where predictions match; perceptron accuracy on agreeing cases; on disagreeing cases — accuracy of each of the two models. 'all three': agreement of all three and perceptron accuracy outside it.\n");
    let mut agree_j = serde_json::Map::new();
    let refs: Vec<&HashMap<usize, Out>> = outs.iter().collect();
    rep.push_str(&agreement_md(&refs, &test, &gy, &mut agree_j));

    // act confusion
    for (s, o) in sets.iter().zip(&outs) {
        let _ = writeln!(rep, "\n## act confusion matrix — {} (test; rows — silver, columns — prediction)\n", s.kind.name());
        rep.push_str(&confusion_md(o, &test, &gy[1], 1));
    }

    // GUM: form vs s_type (evaluation only)
    if let Some(g) = gum {
        let gi = read_gum(g)?;
        let gitems: Vec<Item> = gi.iter().map(|x| x.0.clone()).collect();
        let t0 = Instant::now();
        let grows = feats::extract(&a, &cues, &gitems);
        let g_feat_ms = 1e3 * t0.elapsed().as_secs_f64() / gitems.len().max(1) as f64;
        let gprev = feats::prev_of(&gitems);
        let gidx: Vec<usize> = (0..gitems.len()).collect();
        let form_names = Field::Form.classes();
        let _ = writeln!(rep, "\n## form vs s_type on GUM test (evaluation only; GUM is CC BY-NC-SA, not used for training)\n");
        let _ = writeln!(rep, "Mapping: decl, q, wh, imp, intj, frag, other — identical; ger, inf, sub → frag (no finite main clause); multiple — excluded (we have one form per utterance). Sentences: {}, of them compared: {}; features — {g_feat_ms:.2} ms per sentence.\n", gi.len(), gi.iter().filter(|x| gum_form(&x.1).is_some()).count());
        let cmp: Vec<usize> = gidx.iter().copied().filter(|&i| gum_form(&gi[i].1).is_some()).collect();
        let ggold: Vec<usize> = cmp.iter().map(|&i| form_names.iter().position(|n| Some(*n) == gum_form(&gi[i].1)).unwrap()).collect();
        let _ = writeln!(rep, "| model | accuracy, % | macro-F1, % | accuracy on decl/q/wh/imp, % |");
        let _ = writeln!(rep, "|---|---|---|---|");
        let core: Vec<usize> = (0..cmp.len()).filter(|&t| matches!(form_names[ggold[t]], "decl" | "q" | "wh" | "imp")).collect();
        let base = scores(&ggold, &vec![0; ggold.len()], form_names.len());
        let _ = writeln!(rep, "| baseline (all decl) | {} | {} | {} |", pct(base.0), pct(base.1), pct(core.iter().filter(|&&t| ggold[t] == 0).count() as f64 / core.len().max(1) as f64));
        let mut gum_j = serde_json::Map::new();
        for s in &sets {
            let o = predict_seq(s, &grows, &gprev, &gidx, &dict);
            let pred: Vec<usize> = cmp.iter().map(|i| o[i].class[0]).collect();
            let sc = scores(&ggold, &pred, form_names.len());
            let core_acc = core.iter().filter(|&&t| pred[t] == ggold[t]).count() as f64 / core.len().max(1) as f64;
            let _ = writeln!(rep, "| {} | {} | {} | {} |", s.kind.name(), pct(sc.0), pct(sc.1), pct(core_acc));
            gum_j.insert(s.kind.name().into(), json!({"acc": sc.0, "macro_f1": sc.1, "core_acc": core_acc}));
            if s.kind == Kind::Perceptron {
                // form confusion on GUM for the perceptron
                let used: Vec<usize> = (0..form_names.len()).filter(|&c| ggold.contains(&c) || pred.contains(&c)).collect();
                let mut m = String::new();
                let _ = writeln!(m, "\nform confusion on GUM — perceptron (rows — GUM, columns — prediction):\n");
                let _ = writeln!(m, "| GUM \\ prediction | {} |", used.iter().map(|&c| form_names[c]).collect::<Vec<_>>().join(" | "));
                let _ = writeln!(m, "|---|{}", "---|".repeat(used.len()));
                for &gc in &used {
                    let row: Vec<String> = used.iter().map(|&pc| (0..cmp.len()).filter(|&t| ggold[t] == gc && pred[t] == pc).count()).map(|n| if n == 0 { "·".into() } else { n.to_string() }).collect();
                    let _ = writeln!(m, "| **{}** | {} |", form_names[gc], row.join(" | "));
                }
                gum_j.insert("confusion_md".into(), json!(m));
            }
        }
        if let Some(m) = gum_j.get("confusion_md").and_then(|v| v.as_str()) {
            rep.push_str(m);
        }
        gum_j.insert("base".into(), json!({"acc": base.0, "macro_f1": base.1}));
        results.insert("gum_form".into(), serde_json::Value::Object(gum_j));
    }

    // examples with explanations (test)
    let mut ex = String::new();
    let _ = writeln!(ex, "# Examples of SLM decisions on test (LLM silver vs three models)\n");
    let show = |i: usize, why_title: &str, ex: &mut String| {
        let it = &items[i];
        let g = &gold[&i];
        let _ = writeln!(ex, "### {why_title}: {}\n\n> {}\n", it.key, it.text);
        let _ = writeln!(ex, "- silver: form {}, act {}, indirect {}, hedge {}, polarity {}, voice {}; means: {}", g.form, g.act, g.indirect as u8, g.hedge as u8, g.polarity, g.voice, g.means);
        for (s, o) in sets.iter().zip(&outs) {
            let l = labels_of(&o[&i]);
            let _ = writeln!(ex, "- **{}**: form {}, act {}, indirect {}, hedge {}, polarity {}, voice {}", s.kind.name(), l.form, l.act, l.indirect as u8, l.hedge as u8, l.polarity, l.voice);
            for (k, f) in [(1usize, "act"), (2, "indirect")] {
                let p = &o[&i].preds[k];
                let why: Vec<String> = match s.kind {
                    Kind::Knn => p.why.iter().map(|(key, d)| {
                        let t = items.iter().find(|x| &x.key == key).map(|x| x.text.as_str()).unwrap_or("");
                        let lab = silver.get(key).map(|l| if k == 1 { l.act.name().to_string() } else { (l.indirect as u8).to_string() }).unwrap_or_default();
                        format!("«{}» ({lab}, distance {d:.3})", t.chars().take(90).collect::<String>())
                    }).collect(),
                    _ => p.why.iter().map(|(s, w)| format!("{s} ({w:+.2})")).collect(),
                };
                let _ = writeln!(ex, "  - {f} = {} (conf. {:.2}): {}", Field::ALL[k].classes()[p.class], p.conf, why.join("; "));
            }
        }
        let _ = writeln!(ex);
    };
    let pick = |cond: &dyn Fn(usize) -> bool, n: usize| -> Vec<usize> { test.iter().copied().filter(|&i| cond(i)).take(n).collect() };
    for i in pick(&|i| gold[&i].act == Act::Request && gold[&i].indirect, 4) {
        show(i, "indirect request", &mut ex);
    }
    for i in pick(&|i| { let m = gold[&i].means.to_lowercase(); m.contains("iron") || m.contains("sarcas") || m.contains("mock") }, 3) {
        show(i, "irony", &mut ex);
    }
    for i in pick(&|i| outs.iter().all(|o| o[&i].class[1] == Field::Act.get(&gold[&i])) && !matches!(gold[&i].act, Act::Narrate | Act::Assert), 3) {
        show(i, "correct (all three)", &mut ex);
    }
    for i in pick(&|i| outs[0][&i].class[1] != Field::Act.get(&gold[&i]) && outs[2][&i].class[1] == Field::Act.get(&gold[&i]), 2) {
        show(i, "perceptron wrong, k-NN right", &mut ex);
    }
    for i in pick(&|i| outs.iter().all(|o| o[&i].class[1] != Field::Act.get(&gold[&i])), 3) {
        show(i, "all three wrong", &mut ex);
    }
    std::fs::write(dir.join("examples.md"), &ex)?;

    // cross-validated predictions for the database: 5 folds by document (Tatoeba by sentence)
    let fold = |it: &Item| (match it.src {
        Src::Tale => h64(&it.doc, seed ^ 0xF01D),
        _ => h64(&it.key, seed ^ 0xF01D),
    } % 5) as usize;
    let mut cross: Vec<HashMap<usize, Out>> = vec![HashMap::new(); 3];
    // baseline: majority class of the training folds
    let mut base_cross: HashMap<usize, [usize; 6]> = HashMap::new();
    for fo in 0..5 {
        let tr: Vec<usize> = lab.iter().copied().filter(|&i| fold(&items[i]) != fo).collect();
        let te: Vec<usize> = lab.iter().copied().filter(|&i| fold(&items[i]) == fo).collect();
        let trr = train_rows(&rows, &prev, &gold, &tr);
        let d = Dict::build(&trr);
        let xx: Vec<Vec<u32>> = trr.iter().map(|r| d.encode(r)).collect();
        let kk: Vec<String> = tr.iter().map(|&i| items[i].key.clone()).collect();
        let yy: Vec<Vec<usize>> = Field::ALL.iter().map(|f| tr.iter().map(|&i| f.get(&gold[&i])).collect()).collect();
        let mut maj = [0usize; 6];
        for (k, f) in Field::ALL.iter().enumerate() {
            maj[k] = (0..f.classes().len()).max_by_key(|&c| yy[k].iter().filter(|&&y| y == c).count()).unwrap();
        }
        for &i in &te {
            base_cross.insert(i, maj);
        }
        for (j, &k) in Kind::ALL.iter().enumerate() {
            let s = fit_set(k, &xx, &yy, &kk, &d, seed);
            cross[j].extend(predict_seq(&s, &rows, &prev, &te, &d));
        }
    }
    let lgy: Vec<Vec<usize>> = Field::ALL.iter().map(|f| lab.iter().map(|&i| f.get(&gold[&i])).collect()).collect();
    let mut cross_j = serde_json::Map::new();
    for (j, k) in Kind::ALL.iter().enumerate() {
        let mut s = String::from("sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tmeans\ttrace\n");
        for &i in &lab {
            let o = &cross[j][&i];
            let l = labels_of(o);
            let _ = writeln!(s, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t\t{}", items[i].key, l.form, l.act, l.indirect as u8, l.hedge as u8, l.polarity, l.voice, trace(o));
        }
        std::fs::write(dir.join(format!("rust-{}.tsv", k.name())), s)?;
    }
    let _ = writeln!(rep, "\n## Cross-validation on all silver ({} sentences, 5 folds: tales by document, Tatoeba by sentence)\n", lab.len());
    let _ = writeln!(rep, "Each sentence is predicted by a model that has not seen it; these predictions go into the database. Accuracy / macro-F1, %; baseline — majority class of the training folds.\n");
    let _ = writeln!(rep, "| field | baseline | perceptron | IGTree | k-NN |");
    let _ = writeln!(rep, "|---|---|---|---|---|");
    for (k, f) in Field::ALL.iter().enumerate() {
        let nc = f.classes().len();
        let bp: Vec<usize> = lab.iter().map(|i| base_cross[i][k]).collect();
        let b = scores(&lgy[k], &bp, nc);
        let got: Vec<(f64, f64)> = cross.iter().map(|o| scores(&lgy[k], &lab.iter().map(|i| o[i].class[k]).collect::<Vec<_>>(), nc)).collect();
        let cell = |x: (f64, f64)| format!("{} / {}", pct(x.0), pct(x.1));
        let _ = writeln!(rep, "| {} | {} | {} | {} | {} |", f.name(), cell(b), cell(got[0]), cell(got[1]), cell(got[2]));
        cross_j.insert(f.name().into(), json!({"base": [b.0, b.1], "perceptron": [got[0].0, got[0].1], "igtree": [got[1].0, got[1].1], "knn": [got[2].0, got[2].1]}));
    }
    let _ = writeln!(rep, "\n### Model agreement (cross-validated, {} sentences)\n", lab.len());
    let refs: Vec<&HashMap<usize, Out>> = cross.iter().collect();
    let mut cross_agree = serde_json::Map::new();
    rep.push_str(&agreement_md(&refs, &lab, &lgy, &mut cross_agree));
    let _ = writeln!(rep, "\n### act confusion matrix — perceptron (cross-validated)\n");
    rep.push_str(&confusion_md(&cross[0], &lab, &lgy[1], 1));
    let _ = writeln!(rep, "\n### act confusion matrix — k-NN (cross-validated)\n");
    rep.push_str(&confusion_md(&cross[2], &lab, &lgy[1], 1));

    std::fs::write(dir.join("report.md"), &rep)?;
    let res = json!({"seed": seed, "silver": lab.len(), "train": train.len(), "test": test.len(), "feat_ms": feat_ms, "fields": results, "speed": speed, "agreement": agree_j, "cross": cross_j, "cross_agreement": cross_agree});
    std::fs::write(dir.join("results.json"), serde_json::to_string_pretty(&res)?)?;
    print!("{rep}");
    Ok(())
}

/// Agreement table of perceptron and memory on sentences `idx` (gold `gy[field][t]`): agreement share, accuracy
/// on agreeing cases, number of disagreeing cases and accuracy of each model on them. `outs` — perceptron, IGTree, k-NN.
fn agreement_md(outs: &[&HashMap<usize, Out>], idx: &[usize], gy: &[Vec<usize>], j: &mut serde_json::Map<String, serde_json::Value>) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "| field | pair | agreement, % | acc. on agreeing, % | disagreeing, count | perceptron on disagreeing, % | memory on disagreeing, % |");
    let _ = writeln!(s, "|---|---|---|---|---|---|---|");
    let r = |a: usize, b: usize| if b == 0 { "—".to_string() } else { pct(a as f64 / b as f64) };
    for (k, f) in Field::ALL.iter().enumerate() {
        for (m, name) in [(1usize, "perceptron ~ IGTree"), (2, "perceptron ~ k-NN")] {
            let (mut ag, mut ag_ok, mut dis, mut dis_p, mut dis_m) = (0usize, 0usize, 0usize, 0usize, 0usize);
            for (t, i) in idx.iter().enumerate() {
                let (p, q, g) = (outs[0][i].class[k], outs[m][i].class[k], gy[k][t]);
                if p == q {
                    ag += 1;
                    ag_ok += (p == g) as usize;
                } else {
                    dis += 1;
                    dis_p += (p == g) as usize;
                    dis_m += (q == g) as usize;
                }
            }
            let _ = writeln!(s, "| {} | {name} | {} | {} | {dis} | {} | {} |", f.name(), r(ag, idx.len()), r(ag_ok, ag), r(dis_p, dis), r(dis_m, dis));
            j.insert(format!("{}:{}", f.name(), Kind::ALL[m].name()), json!({"agree": ag as f64 / idx.len().max(1) as f64, "acc_agree": ag_ok as f64 / ag.max(1) as f64, "disagree": dis, "perceptron_on_disagree": dis_p as f64 / dis.max(1) as f64, "memory_on_disagree": dis_m as f64 / dis.max(1) as f64}));
        }
        let (mut all3, mut all3_ok, mut rest_ok) = (0usize, 0usize, 0usize);
        for (t, i) in idx.iter().enumerate() {
            let c = outs[0][i].class[k];
            if outs[1][i].class[k] == c && outs[2][i].class[k] == c {
                all3 += 1;
                all3_ok += (c == gy[k][t]) as usize;
            } else {
                rest_ok += (c == gy[k][t]) as usize;
            }
        }
        let _ = writeln!(s, "| {} | all three | {} | {} | {} | {} | — |", f.name(), r(all3, idx.len()), r(all3_ok, all3), idx.len() - all3, r(rest_ok, idx.len() - all3));
    }
    s
}

/// Confusion matrix of field `k` (rows — silver, columns — prediction).
fn confusion_md(o: &HashMap<usize, Out>, idx: &[usize], gy: &[usize], k: usize) -> String {
    let names = Field::ALL[k].classes();
    let used: Vec<usize> = (0..names.len()).filter(|&c| gy.contains(&c) || idx.iter().any(|i| o[i].class[k] == c)).collect();
    let mut s = String::new();
    let _ = writeln!(s, "| silver \\ prediction | {} |", used.iter().map(|&c| names[c]).collect::<Vec<_>>().join(" | "));
    let _ = writeln!(s, "|---|{}", "---|".repeat(used.len()));
    for &g in &used {
        let row: Vec<String> = used.iter().map(|&p| idx.iter().enumerate().filter(|&(t, i)| gy[t] == g && o[i].class[k] == p).count()).map(|n| if n == 0 { "·".into() } else { n.to_string() }).collect();
        let _ = writeln!(s, "| **{}** | {} |", names[g], row.join(" | "));
    }
    s
}
