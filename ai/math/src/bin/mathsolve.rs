//! math v2 — curriculum-based solver: the LLM writes a plan, the MMM executes and verifies it.
//!   mathsolve prep                         — level sets from raw files (data/raw/mathsolve)
//!   mathsolve prep-train [N]                — training level 5: N GSM8K train problems (default 1000)
//!   mathsolve arith [--train N] [--epochs E] [--leaves L] [--beam B] — v3: subproblem trees on GSM8K (no LLM)
//!   mathsolve run <level> <A|B> [--split dev|held|all] [--limit N] [--batch K] [--par P] [--reserve $]
//!   mathsolve c <level>                    — configuration C (MMM alone) on the held-out split
//!   mathsolve memory                       — memory of verified solutions (JSONL) → DuckDB
//!   mathsolve report                       — table: level × configuration
//!   mathsolve plan <file.json>             — execute a plan locally, show the trace
//!   mathsolve calc "<expr>"                — evaluate an expression with steps and checks
//!   mathsolve controls                     — negative controls of the gate (must be red)
//! Run directory: MATH2_RUN or data/runs/math2-2026-09-26.

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use math::calc;
use math::curric::{self, Problem};
use math::plan::{self, Expect};
use math::solve::{self, Record, Run};
use math::vmm::{self, Vmm};

fn home() -> PathBuf {
    PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into()))
}

fn run_dir() -> PathBuf {
    std::env::var("MATH2_RUN").map(PathBuf::from).unwrap_or_else(|_| home().join("runs/math2-2026-09-26"))
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else { bail!("usage: mathsolve prep | run <level> <A|B> | c <level> | memory | report | plan <file> | calc <expr> | controls") };
    let dir = run_dir();
    std::fs::create_dir_all(&dir)?;
    let records = dir.join("records.jsonl");
    let calls = dir.join("calls.jsonl");
    let cap: f64 = std::env::var("MATH2_CAP").ok().and_then(|s| s.parse().ok()).unwrap_or(15.0); // design: "cheap skeleton" — wave cap $15
    match cmd.as_str() {
        "prep" => {
            let raw = home().join("raw/mathsolve");
            for (l, n) in curric::prep(&raw, &dir.join("levels"))? {
                println!("level {l}: {n} problems");
            }
        }
        "prep-train" => {
            let n: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1000);
            let k = curric::prep_train(&home().join("raw/mathsolve"), &dir.join("levels"), n)?;
            println!("level {} (training): {k} problems", curric::TRAIN_LEVEL);
        }
        "qread-text" => {
            let ann = en::annotate::Annotator::load(std::path::Path::new(math::understand::MODEL))?;
            let (v, w) = math::qread::answer_text(&ann, &args[1..].join(" "));
            println!("{v:?} unread {:?}\n{}", w.unread, w.log.join("\n"));
        }
        "qread" => {
            // world v2: qread [svamp|svamp-train] [show N] — coverage and accuracy when the world answers
            let set = args.get(1).cloned().unwrap_or_else(|| "svamp".into());
            let show: usize = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0);
            let f = if set == "svamp-train" { "mwpdata-svamp/svamp-train.json" } else { "mwpdata-svamp/svamp-test.json" };
            let ann = en::annotate::Annotator::load(std::path::Path::new(math::understand::MODEL))?;
            let d: Vec<serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(home().join("raw").join(f))?)?;
            let (mut ans, mut ok, mut shown) = (0, 0, 0);
            let mut dump = String::new();
            for (i, x) in d.iter().enumerate() {
                let body = x["Body"].as_str().unwrap_or("");
                let q = x["Question"].as_str().unwrap_or("");
                let g = x["Answer"].as_f64().unwrap_or(f64::NAN);
                let (v, w) = math::qread::answer(&ann, body, q);
                // strict gate: the world has read every number
                let v = if std::env::var("QREAD_STRICT").is_ok() && !w.unread.is_empty() { None } else { v };
                if let Some(v) = v {
                    ans += 1;
                    let good = (v - g).abs() < 1e-6;
                    ok += usize::from(good);
                    dump.push_str(&format!("{i}\t{}\t{v}\n", u8::from(good)));
                    if !good && shown < show {
                        shown += 1;
                        println!("✗ {body} {q}\n  → {v} (gold {g}, {})\n  {}", x["Equation"].as_str().unwrap_or(""), w.log.join(" | "));
                    }
                }
            }
            if let Ok(p) = std::env::var("QREAD_DUMP") {
                std::fs::write(p, dump)?;
            }
            println!("{set}: the world answered {ans}/{}, of them correct {ok} ({:.1}%)", d.len(), 100.0 * ok as f64 / ans.max(1) as f64);
        }
        "steps" => {
            // v4: transition system (SHIFT/SKIP/REDUCE) with UD features; gold — arith trees (+ supervision from LLM plans)
            let n_train: usize = flag(&args, "--train").and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
            let epochs: usize = flag(&args, "--epochs").and_then(|s| s.parse().ok()).unwrap_or(5);
            let beam: usize = flag(&args, "--beam").and_then(|s| s.parse().ok()).unwrap_or(16);
            let maxq: usize = flag(&args, "--maxq").and_then(|s| s.parse().ok()).unwrap_or(9);
            let show: usize = flag(&args, "--show").and_then(|s| s.parse().ok()).unwrap_or(0);
            let ann = en::annotate::Annotator::load(std::path::Path::new(math::understand::MODEL))?;
            let set = flag(&args, "--set").unwrap_or_else(|| "gsm8k".into());
            let exact = args.iter().any(|a| a == "--exact");
            // (id, question, gold, annotation: SVAMP equation or GSM8K solution with <<…>>)
            let rd = home().join("raw");
            let load_x = |f: &str| -> Result<Vec<(String, String, f64, String)>> {
                let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(rd.join(f))?)?;
                Ok(v.as_array().context("array")?.iter().enumerate().filter_map(|(i, x)| {
                    if let Some(body) = x["Body"].as_str() {
                        let body = body.trim();
                        let sep = if body.ends_with(['.', '?', '!']) { " " } else { ". " };
                        Some((format!("{f}-{i}"), format!("{body}{sep}{}", x["Question"].as_str()?.trim()), x["Answer"].as_f64()?, x["Equation"].as_str()?.to_string()))
                    } else {
                        let a = x["answer"].as_str()?;
                        let g: f64 = a.rsplit("####").next()?.trim().replace(',', "").parse().ok()?;
                        Some((format!("{f}-{i}"), x["question"].as_str()?.to_string(), g, a.to_string()))
                    }
                }).collect())
            };
            // mix: training SVAMP + GSM8K, test — SVAMP
            // tiny: training GSM8K train + a TinyGSM sample (MIT, Python functions), test — GSM8K
            let (tr_f, te_f) = if set == "tiny" { ("mwpdata-gsm8k-socratic/gsm8k-socratic-train.json", "mwpdata-gsm8k-socratic/gsm8k-socratic-test.json") } else if set == "mix" { ("mwpdata-svamp/svamp-train.json", "mwpdata-svamp/svamp-test.json") } else if set == "svamp" { ("mwpdata-svamp/svamp-train.json", "mwpdata-svamp/svamp-test.json") } else { ("mwpdata-gsm8k-socratic/gsm8k-socratic-train.json", "mwpdata-gsm8k-socratic/gsm8k-socratic-test.json") };
            let load = |f: &str| -> Result<Vec<(String, String, f64)>> {
                let text = std::fs::read_to_string(home().join("raw/mathsolve/gsm8k").join(f))?;
                Ok(text.lines().enumerate().filter_map(|(i, l)| {
                    let v: serde_json::Value = serde_json::from_str(l).ok()?;
                    let g: f64 = v["answer"].as_str()?.rsplit("####").next()?.trim().replace(',', "").parse().ok()?;
                    Some((format!("{f}-{i}"), v["question"].as_str()?.to_string(), g))
                }).collect())
            };
            let mut used: HashMap<String, Vec<f64>> = HashMap::new();
            for r in solve::load_records(&records) {
                if r.level == curric::TRAIN_LEVEL && r.config == "B" && r.verified && r.correct {
                    let i: usize = r.id.trim_start_matches("gsm8k-train-").parse().unwrap_or(usize::MAX);
                    used.insert(format!("train.jsonl-{i}"), math::arith::quantities(&r.plan.to_string()).0.iter().map(|x| x.val).collect());
                }
            }
            let t0 = std::time::Instant::now();
            let am = math::arith::Model::new(4, 20);
            let mut data = Vec::new();
            let (mut no_tree, mut no_oracle) = (0, 0);
            let mut no_exact = 0;
            let dag = args.iter().any(|a| a == "--dag");
            let mut dag_data: Vec<(math::steps::Prob, math::deduct::Pre, math::deduct::Gold)> = Vec::new();
            let mut qtexts: Vec<String> = Vec::new();
            let mut shapes_lib: Option<std::sync::Arc<HashMap<String, f64>>> = None;
            let mut star_pool: Vec<(math::steps::Prob, f64)> = Vec::new();
            let mut train_x = load_x(tr_f)?;
            // distractor augmentation (Anantheswaran 2024, Yang 2025 — training on noisy problems): for each
            // training problem — a copy with a foreign sentence containing a number before the question; same equation
            if args.iter().any(|a| a == "--augment") {
                let n0 = train_x.len();
                let sent_with_num = |t: &str| -> Option<String> {
                    t.split_inclusive(['.', '!']).map(|x| x.trim()).find(|x| x.chars().any(|c| c.is_ascii_digit()) && !x.ends_with('?')).map(String::from)
                };
                let mut extra = Vec::new();
                for i in 0..n0 {
                    let (id, q, g, eq) = &train_x[i];
                    let donor = &train_x[(i * 7 + 3) % n0].1;
                    let Some(ds) = sent_with_num(donor) else { continue };
                    // the question sentence is the last one; the distractor goes before it
                    let qs_pos = q.rfind(|c: char| c == '.' || c == '!').map(|p| p + 1).unwrap_or(0);
                    let aug = format!("{} {} {}", q[..qs_pos].trim(), ds, q[qs_pos..].trim());
                    extra.push((format!("{id}-aug"), aug, *g, eq.clone()));
                }
                println!("distractor augmentation: +{} problems", extra.len());
                train_x.extend(extra);
            }
            if set == "mix" { train_x.extend(load_x("mwpdata-gsm8k-socratic/gsm8k-socratic-train.json")?); }
            if set == "tiny" {
                let n_tiny: usize = flag(&args, "--tiny").and_then(|s| s.parse().ok()).unwrap_or(20000);
                let text = std::fs::read_to_string(home().join("raw/mathsolve/tinygsm/sample60k.jsonl"))?;
                for (i, l) in text.lines().take(n_tiny).enumerate() {
                    let v: serde_json::Value = serde_json::from_str(l)?;
                    train_x.push((format!("tiny-{i}"), v["question"].as_str().unwrap_or("").to_string(), f64::NAN, v["code"].as_str().unwrap_or("").to_string()));
                }
            }
            for (id, q, g, ann_text) in train_x.into_iter().take(n_train) {
                let ap = math::arith::prepare(&q, maxq);
                if exact {
                    let t = if ann_text.contains("def simple_math_problem") { math::steps::tree_from_python(&ann_text, &ap.qs) } else if ann_text.contains("<<") { math::steps::tree_from_calcs(&ann_text, &ap.qs) } else { math::steps::tree_from_equation(&ann_text, &ap.qs) };
                    // TinyGSM gold is unknown (NaN): then we require a non-negative integer tree answer
                    let t = t.filter(|t| t.exact(&ap.qs).map(|v| { let x = math::big::to_f64(&v); if g.is_nan() { x >= 0.0 && (x - x.round()).abs() < 1e-9 } else { (x - g).abs() < 1e-6 } }).unwrap_or(false));
                    let Some(t) = t else {
                        no_exact += 1;
                        // STaR: a problem without an exact tree goes to the reserve; the snake will try to solve it by itself
                        if !g.is_nan() { star_pool.push((math::steps::prepare(&ann, &q, maxq), g)); }
                        continue
                    };
                    if dag {
                        // v5: DAG — a tree with repeats is fine, text order is not required
                        let Some(g) = math::deduct::gold_of(&t, ap.qs.len()) else { no_oracle += 1; continue };
                        let pp = math::steps::prepare(&ann, &q, maxq);
                        let pre = math::deduct::precompute(&pp);
                        dag_data.push((pp, pre, g));
                        continue;
                    }
                    let Some(acts) = math::steps::oracle(&t, ap.qs.len()) else { no_oracle += 1; continue };
                    data.push((math::steps::prepare(&ann, &q, maxq), acts));
                    qtexts.push(q.clone());
                    continue;
                }
                let id = if set == "gsm8k" { id.replace("mwpdata-gsm8k-socratic/gsm8k-socratic-train.json", "train.jsonl") } else { id };
                let cands = am.gold_trees(&ap, g, used.get(&id).map(|u| u.as_slice()));
                if cands.is_empty() { no_tree += 1; continue }
                // among equally good — the first one that composes in text order
                let Some(acts) = cands.iter().take(200).find_map(|t| math::steps::oracle(t, ap.qs.len())) else { no_oracle += 1; continue };
                data.push((math::steps::prepare(&ann, &q, maxq), acts));
            }
            println!("{set}{}: training with gold {}; no tree {no_tree}; no exact tree {no_exact}; tree not in text order {no_oracle}; {:.1} s", if exact { " (exact gold)" } else { "" }, data.len(), t0.elapsed().as_secs_f64());
            if dag {
                println!("DAG: training {}", dag_data.len());
                let mut m = math::deduct::Model::new(beam);
                for e in 0..epochs {
                    let ok = dag_data.iter().filter(|(p, pre, g)| m.learn(p, pre, g)).count();
                    println!("epoch {}: correct on training {ok}/{}; {:.1} s", e + 1, dag_data.len(), t0.elapsed().as_secs_f64());
                }
                let am = m.averaged();
                let rk: usize = flag(&args, "--rerank").and_then(|s| s.parse().ok()).unwrap_or(0);
                // judge: candidates from models trained without their own fold
                let judge = if rk > 0 {
                    let folds = 4;
                    let mut items: Vec<Vec<(Vec<String>, bool)>> = Vec::new();
                    for f in 0..folds {
                        let mut mf = math::deduct::Model::new(beam);
                        for _ in 0..epochs {
                            for (i, (p, pre, g)) in dag_data.iter().enumerate() {
                                if i % folds != f { mf.learn(p, pre, g); }
                            }
                        }
                        let mf = mf.averaged();
                        for (i, (p, pre, g)) in dag_data.iter().enumerate() {
                            if i % folds != f { continue; }
                            let gold_tree = { let mut t = None; let mut pool: Vec<math::arith::Tree> = p.qs.iter().enumerate().map(|(k, _)| math::arith::Tree::Leaf(k)).collect(); for st in &g.steps { if let math::deduct::Step::Comb(a, b, op) = st { let n = math::arith::Tree::Node(*op, Box::new(pool[*a as usize].clone()), Box::new(pool[*b as usize].clone())); pool.push(n.clone()); t = Some(n); } } t };
                            let gv = gold_tree.and_then(|t| t.exact(&p.qs)).map(|x| math::big::to_f64(&x));
                            let cs = mf.topk(p, pre, rk);
                            let top = cs.first().map(|c| c.1).unwrap_or(0.0);
                            items.push(cs.iter().enumerate().map(|(r, c)| (math::steps::cand_feats(p, &math::steps::Cand { tree: c.0.clone(), score: c.1, val: c.2, acts: Vec::new(), votes: 1 }, r, top), gv.is_some_and(|g| (g - c.2).abs() < 1e-6))).collect());
                        }
                    }
                    println!("judge: training {}; correct in top-{rk}: {}", items.len(), items.iter().filter(|c| c.iter().any(|x| x.1)).count());
                    Some(math::steps::Rerank::train(&items, 10))
                } else { None };
                let test: Vec<(String, String, f64)> = load_x(te_f)?.into_iter().map(|(a, b, c, _)| (a, b, c)).collect();
                let (mut acc, mut intop, mut top1) = (0, 0, 0);
                for (_, q, g) in &test {
                    let p = math::steps::prepare(&ann, q, maxq);
                    let pre = math::deduct::precompute(&p);
                    let cs = am.topk(&p, &pre, rk.max(1));
                    if cs.iter().any(|c| (c.2 - g).abs() < 1e-6) { intop += 1; }
                    if cs.first().is_some_and(|c| (c.2 - g).abs() < 1e-6) { top1 += 1; }
                    let pick = match &judge {
                        Some(j) if !cs.is_empty() => {
                            let top = cs[0].1;
                            let fs: Vec<Vec<String>> = cs.iter().enumerate().map(|(r, c)| math::steps::cand_feats(&p, &math::steps::Cand { tree: c.0.clone(), score: c.1, val: c.2, acts: Vec::new(), votes: 1 }, r, top)).collect();
                            Some(j.pick(&fs))
                        }
                        _ => (!cs.is_empty()).then_some(0),
                    };
                    if pick.is_some_and(|i| (cs[i].2 - g).abs() < 1e-6) { acc += 1; }
                }
                let n = test.len();
                println!("{set} test: MMM alone (DAG) correct {acc}/{n} ({:.1}%); first {top1}; correct in top-{} {intop}; {:.1} s", 100.0 * acc as f64 / n as f64, rk.max(1), t0.elapsed().as_secs_f64());
                return Ok(());
            }
            // sentence principles from world scripts (world::quant): a classifier, cross-validated on training
            let princ_all = flag(&args, "--princ").map(|path| -> Result<math::steps::Princ> {
                let mut scripts: HashMap<String, String> = HashMap::new();
                for l in std::fs::read_to_string(&path)?.lines() {
                    let v: serde_json::Value = serde_json::from_str(l)?;
                    if v["ok"].as_bool() == Some(true) {
                        scripts.insert(v["question"].as_str().unwrap_or("").to_string(), v["script"].as_str().unwrap_or("").to_string());
                    }
                }
                let kinds: Vec<String> = ["total", "gain", "loss", "transfer", "rate", "compare", "part", "convert", "combine", "unit"].iter().map(|s| s.to_string()).collect();
                let labels: Vec<Option<Vec<Vec<String>>>> = data.iter().zip(&qtexts).map(|((p, _), q)| scripts.get(q).map(|sc| math::steps::script_kinds(sc, p.sent_feats.len()))).collect();
                let (mut tp, mut fp, mut fneg) = (0, 0, 0);
                for f in 0..5 {
                    let tr: Vec<(&Vec<String>, &Vec<String>)> = data.iter().zip(&labels).enumerate().filter(|(i, _)| i % 5 != f).filter_map(|(_, ((p, _), l))| l.as_ref().map(|l| p.sent_feats.iter().zip(l.iter()).collect::<Vec<_>>())).flatten().collect();
                    let pm = math::steps::Princ::train(&tr, &kinds, 10);
                    for (i, ((p, _), l)) in data.iter_mut().zip(&labels).enumerate() {
                        if i % 5 != f { continue; }
                        let preds: Vec<Vec<String>> = p.sent_feats.iter().map(|fs| pm.predict(fs)).collect();
                        if let Some(l) = l {
                            for (pr, y) in preds.iter().zip(l) {
                                tp += pr.iter().filter(|k| y.contains(k)).count();
                                fp += pr.iter().filter(|k| !y.contains(k)).count();
                                fneg += y.iter().filter(|k| !pr.contains(k)).count();
                            }
                        }
                        p.sent_k = preds;
                    }
                }
                let n_lab = labels.iter().filter(|l| l.is_some()).count();
                println!("principles: scripts for training {n_lab}; cross-validated P {:.1}% R {:.1}%", 100.0 * tp as f64 / (tp + fp).max(1) as f64, 100.0 * tp as f64 / (tp + fneg).max(1) as f64);
                let tr: Vec<(&Vec<String>, &Vec<String>)> = data.iter().zip(&labels).filter_map(|((p, _), l)| l.as_ref().map(|l| p.sent_feats.iter().zip(l.iter()).collect::<Vec<_>>())).flatten().collect();
                Ok(math::steps::Princ::train(&tr, &kinds, 10))
            }).transpose()?;
            // training the snake's world on verified LLM scripts (world::quant): number action and shared cells
            let world_model = flag(&args, "--world").map(|path| -> Result<(math::qworld::WorldModel, math::qworld::AskModel)> {
                let mut scripts: HashMap<String, String> = HashMap::new();
                for l in std::fs::read_to_string(&path)?.lines() {
                    let v: serde_json::Value = serde_json::from_str(l)?;
                    if v["ok"].as_bool() == Some(true) { scripts.insert(v["question"].as_str().unwrap_or("").to_string(), v["script"].as_str().unwrap_or("").to_string()); }
                }
                let items: Vec<(&math::steps::Prob, Vec<Option<(char, String)>>)> = data.iter().zip(&qtexts).filter_map(|((p, _), q)| scripts.get(q).map(|sc| (p, math::qworld::script_labels(sc, p)))).collect();
                let lab_n: usize = items.iter().map(|x| x.1.iter().filter(|l| l.is_some()).count()).sum();
                println!("world: training problems with scripts {}, number labels {lab_n}", items.len());
                let tm: Vec<(&math::steps::Prob, math::qworld::Tmpl)> = data.iter().zip(&qtexts).filter_map(|((p, _), q)| scripts.get(q).and_then(|sc| math::qworld::script_template(sc)).map(|t| (p, t))).collect();
                let mut cnt: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
                for (_, t) in &tm { *cnt.entry(format!("{t:?}")).or_default() += 1; }
                println!("query: templates from scripts {} — {cnt:?}", tm.len());
                Ok((math::qworld::WorldModel::train(&items), math::qworld::AskModel::train(&tm)))
            }).transpose()?;
            // the world's answer goes into the problem (a judge feature): for both training and test
            let use_world_feat = args.iter().any(|a| a == "--world-feat");
            // world v2 (qread) as a judge feature: answers only after reading every number
            let use_qread = args.iter().any(|a| a == "--qread");
            let qread_of = |text: &str| -> Option<(f64, String)> {
                let (v, w) = math::qread::answer_text(&ann, text);
                if !w.unread.is_empty() { return None; }
                v.map(|v| (v, format!("qread:{}", math::qread::kind(&w))))
            };
            if use_qread {
                for ((p, _), text) in data.iter_mut().zip(qtexts.iter()) {
                    p.world_ans = qread_of(text);
                }
            }
            if use_world_feat {
                if let Some((wm, am)) = &world_model {
                    for (p, _) in data.iter_mut() {
                        let w = math::qworld::write_learned(p, wm);
                        let t = am.predict(p);
                        p.world_ans = math::qworld::ask_tmpl(p, &w, t).map(|r| (math::big::to_f64(&r.0), format!("{t:?}")));
                    }
                }
            }
            // learned binding of a number to the question: labels — used in the gold tree; threshold — on held-out train folds
            let bind_items: Vec<(usize, Vec<String>, bool)> = data.iter().enumerate().flat_map(|(di, (p, a))| {
                let used = math::steps::used_leaves(a);
                (0..p.qs.len()).filter(|&i| !p.qs[i].name.starts_with("const")).map(move |i| (di, math::steps::bind_feats(p, i), used.contains(&i))).collect::<Vec<_>>()
            }).collect();
            let mut cv: Vec<(f64, bool)> = Vec::new();
            for f in 0..4 {
                let tr: Vec<(Vec<String>, bool)> = bind_items.iter().filter(|x| x.0 % 4 != f).map(|x| (x.1.clone(), x.2)).collect();
                let bm = math::steps::BindModel::train(&tr, 10);
                cv.extend(bind_items.iter().filter(|x| x.0 % 4 == f).map(|x| (bm.score(&x.1), x.2)));
            }
            // threshold: the highest recall of "irrelevant" numbers at precision ≥ 0.9
            let n_dis = cv.iter().filter(|x| !x.1).count();
            let mut bind_th = f64::NEG_INFINITY;
            let mut sorted: Vec<f64> = cv.iter().map(|x| x.0).collect();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for &th in &sorted {
                let flagged: Vec<&(f64, bool)> = cv.iter().filter(|x| x.0 <= th).collect();
                if flagged.is_empty() { continue; }
                let prec = flagged.iter().filter(|x| !x.1).count() as f64 / flagged.len() as f64;
                if prec >= 0.9 { bind_th = th; }
            }
            // curve: precision of "irrelevant" numbers at recall 10–50%
            {
                let mut v = cv.clone();
                v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let mut line = String::new();
                for rec in [0.1, 0.2, 0.3, 0.5] {
                    let need = (rec * n_dis as f64).ceil() as usize;
                    let (mut dis, mut tot) = (0, 0);
                    for x in &v { tot += 1; if !x.1 { dis += 1; } if dis >= need { break; } }
                    line.push_str(&format!("recall {:.0}% → precision {:.0}%; ", rec * 100.0, 100.0 * dis as f64 / tot.max(1) as f64));
                }
                println!("binding (cross-validated on train): {line}");
            }
            let flagged = cv.iter().filter(|x| x.0 <= bind_th).count();
            println!("binding: numbers {}, irrelevant {n_dis}; threshold {bind_th:.2} flags {flagged} (precision ≥ 0.9, recall {:.0}%)", cv.len(), 100.0 * cv.iter().filter(|x| x.0 <= bind_th && !x.1).count() as f64 / n_dis.max(1) as f64);
            let bind_model = math::steps::BindModel::train(&bind_items.iter().map(|x| (x.1.clone(), x.2)).collect::<Vec<_>>(), 10);
            // curriculum (Bengio 2009; Sweller — worked examples from simple ones): first problems with fewer operations
            let staged = args.iter().any(|a| a == "--curriculum");
            let n_ops = |a: &Vec<math::steps::Act>| a.iter().filter(|x| matches!(x, math::steps::Act::Reduce(..) | math::steps::Act::Far(..))).count();
            let mut m = math::steps::Model::new(beam);
            if staged {
                // staged: 3 epochs each — first 1-operation problems, then ≤ 2, then the usual order of all
                for cap in [1usize, 2] {
                    for _ in 0..3 { for (p, a) in data.iter().filter(|x| n_ops(&x.1) <= cap) { m.learn(p, a); } }
                }
                println!("staged curriculum: 3 epochs ≤ 1 operation, 3 epochs ≤ 2 operations, then all");
            }
            for e in 0..epochs {
                let ok = data.iter().filter(|(p, a)| m.learn(p, a)).count();
                println!("epoch {}: correct on training {ok}/{}; {:.1} s", e + 1, data.len(), t0.elapsed().as_secs_f64());
            }
            // STaR (Zelikman 2022): self-training rounds — the snake's solutions with answer = gold (exact core) go into training
            let star_rounds: usize = flag(&args, "--star").and_then(|s| s.parse().ok()).unwrap_or(0);
            for round in 0..star_rounds {
                let am_r = m.averaged();
                let mut added = 0;
                let mut rest = Vec::new();
                for (p, g) in star_pool.drain(..) {
                    let found = am_r.topk(&p, 5).into_iter().find(|c| (c.val - g).abs() < 1e-6);
                    match found {
                        Some(c) => {
                            if let Some(acts) = math::steps::oracle(&c.tree, p.qs.len()) { data.push((p, acts)); added += 1; } else { rest.push((p, g)); }
                        }
                        None => rest.push((p, g)),
                    }
                }
                star_pool = rest;
                println!("STaR round {}: added {added} snake solutions ({} left); training {}", round + 1, star_pool.len(), data.len());
                m = math::steps::Model::new(beam);
                for _ in 0..epochs { for (p, a) in &data { m.learn(p, a); } }
            }
            // abstraction library: shapes of gold train trees (without numbers) with frequencies — into each problem
            if args.iter().any(|a| a == "--shapes") {
                let mut cnt: HashMap<String, f64> = HashMap::new();
                for (p, a) in &data {
                    let mut st = Vec::new();
                    // a tree from the sequence of operations
                    let _ = p;
                    for act in a { st.push(*act); }
                    if let Some(t) = math::steps::tree_of(p, a) { *cnt.entry(math::steps::shape(&t)).or_default() += 1.0; }
                }
                let tot: f64 = cnt.values().sum::<f64>().max(1.0);
                let lib: std::sync::Arc<HashMap<String, f64>> = std::sync::Arc::new(cnt.into_iter().map(|(k, v)| (k, v / tot)).collect());
                println!("shape library: {} distinct shapes", lib.len());
                for (p, _) in data.iter_mut() { p.shapes = lib.clone(); }
                shapes_lib = Some(lib);
            }
            // another model for the ensemble: v3 subproblem trees (arith) — trained on the same exact trees
            let arith_m: Option<math::arith::Model> = if args.iter().any(|a| a == "--arith-feat") {
                let mut am_ = math::arith::Model::new(4, 20);
                let arith_data: Vec<(math::arith::Prob, math::arith::Tree, f64)> = data.iter().zip(&qtexts).filter_map(|((p, a), q)| {
                    let t = math::steps::tree_of(p, a)?;
                    let v = math::big::to_f64(&t.exact(&p.qs)?);
                    Some((math::arith::prepare(q, maxq), t, v))
                }).collect();
                for _ in 0..8 { for (p, t, v) in &arith_data { am_.learn(p, t, *v); } }
                let am_ = am_.averaged();
                // answers on training — from models without their own fold (otherwise the judge is overconfident)
                for f in 0..4usize {
                    let mut mf = math::arith::Model::new(4, 20);
                    for _ in 0..8 { for (i, (p, t, v)) in arith_data.iter().enumerate() { if i % 4 != f { mf.learn(p, t, *v); } } }
                    let mf = mf.averaged();
                    for (i, ((p, _), q)) in data.iter_mut().zip(&qtexts).enumerate() {
                        if i % 4 != f { continue; }
                        let ap = math::arith::prepare(q, maxq);
                        p.arith_ans = mf.predict(&ap, true).and_then(|(t, _, _)| t.exact(&ap.qs)).map(|x| math::big::to_f64(&x));
                    }
                }
                println!("arith as a second model: trained on {} trees", arith_data.len());
                Some(am_)
            } else { None };
            let am2 = m.averaged();
            // bagging (Breiman 1996): two more models — reversed and shuffled training order; their top-K goes into the shared pool for the judge
            let bag = args.iter().any(|a| a == "--bag" || a == "--bag-judge");
            let extra: Vec<math::steps::Model> = if bag {
                let mut out = Vec::new();
                for variant in 0..2 {
                    let mut idx: Vec<usize> = (0..data.len()).collect();
                    if variant == 0 { idx.reverse(); } else { idx.sort_by_key(|&i| (i.wrapping_mul(2654435761)) % 1_000_003); }
                    let mut mv = math::steps::Model::new(beam);
                    for _ in 0..epochs { for &i in &idx { mv.learn(&data[i].0, &data[i].1); } }
                    out.push(mv.averaged());
                }
                println!("bagging: 2 more models (reversed and shuffled order)");
                out
            } else { Vec::new() };
            // top-K reranking: candidates on training — from models trained without the corresponding fold (to avoid inflation)
            let rk: usize = flag(&args, "--rerank").and_then(|s| s.parse().ok()).unwrap_or(0);
            let reranker = if rk > 0 {
                let folds = 4;
                let mut items: Vec<Vec<(Vec<String>, bool)>> = Vec::new();
                for f in 0..folds {
                    let mut mf = math::steps::Model::new(beam);
                    for _ in 0..epochs {
                        for (i, (p, a)) in data.iter().enumerate() {
                            if i % folds != f { mf.learn(p, a); }
                        }
                    }
                    let mf = mf.averaged();
                    // ensemble within the fold: 2 more models with a different order (only with --bag-judge)
                    let mut mfx: Vec<math::steps::Model> = Vec::new();
                    if args.iter().any(|a| a == "--bag-judge") {
                        for variant in 0..2 {
                            let mut idx: Vec<usize> = (0..data.len()).filter(|i| i % folds != f).collect();
                            if variant == 0 { idx.reverse(); } else { idx.sort_by_key(|&i| (i.wrapping_mul(2654435761)) % 1_000_003); }
                            let mut mv = math::steps::Model::new(beam);
                            for _ in 0..epochs { for &i in &idx { mv.learn(&data[i].0, &data[i].1); } }
                            mfx.push(mv.averaged());
                        }
                    }
                    for (i, (p, a)) in data.iter().enumerate() {
                        if i % folds != f { continue; }
                        let gold_v = math::steps::oracle_value(a, p);
                        let mut lists = vec![mf.topk(p, rk)];
                        for mv in &mfx { lists.push(mv.topk(p, rk)); }
                        let cs = math::steps::merge_votes(lists);
                        let top = cs.first().map(|c| c.score).unwrap_or(0.0);
                        items.push(cs.iter().enumerate().map(|(r, c)| (math::steps::cand_feats(p, c, r, top), gold_v.is_some_and(|g| (g - c.val).abs() < 1e-6))).collect());
                    }
                }
                let hit = items.iter().filter(|cs| cs.iter().any(|c| c.1)).count();
                let top1 = items.iter().filter(|cs| cs.first().is_some_and(|c| c.1)).count();
                println!("reranking: training {}; correct among top-{rk}: {hit}; first: {top1}", items.len());
                Some(math::steps::Rerank::train(&items, 10))
            } else { None };
            let (mut in_topk, mut top1_base) = (0, 0);
            let mut miss_shown = 0usize;
            let test: Vec<(String, String, f64)> = load_x(te_f)?.into_iter().map(|(a, b, c, _)| (a, b, c)).collect();
            let mut rows: Vec<(bool, f64)> = Vec::new();
            let mut dump = String::new();
            let mut selfs: Vec<(global::SelfReport, bool)> = Vec::new();
            let rethink = args.iter().any(|a| a == "--rethink");
            let explain_n: usize = flag(&args, "--explain").and_then(|s| s.parse().ok()).unwrap_or(0);
            let hints_n: usize = flag(&args, "--hints").and_then(|s| s.parse().ok()).unwrap_or(0);
            let mut hint_cases: Vec<(String, String, f64, Vec<f64>)> = Vec::new();
            let mut reports: Vec<String> = Vec::new();
            let mut trig: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
            let mut ign_trig: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
            let mut wtrig: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
            let mut teach_verbs: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
            let mut teach_nouns: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
            math::steps::ACTIONS.store(0, std::sync::atomic::Ordering::Relaxed);
            for (k, (id, q, g)) in test.iter().enumerate() {
                let t_item = std::time::Instant::now();
                let mut p = math::steps::prepare(&ann, q, maxq);
                if let Some(l) = &shapes_lib { p.shapes = l.clone(); }
                if let Some(am_) = &arith_m {
                    let ap = math::arith::prepare(q, maxq);
                    p.arith_ans = am_.predict(&ap, true).and_then(|(t, _, _)| t.exact(&ap.qs)).map(|x| math::big::to_f64(&x));
                }
                if let Some(pm) = &princ_all {
                    p.sent_k = p.sent_feats.iter().map(|fs| pm.predict(fs)).collect();
                }
                if use_qread {
                    p.world_ans = qread_of(q);
                }
                if use_world_feat {
                    if let Some((wm, am)) = &world_model {
                        let w = math::qworld::write_learned(&p, wm);
                        let t = am.predict(&p);
                        p.world_ans = math::qworld::ask_tmpl(&p, &w, t).map(|r| (math::big::to_f64(&r.0), format!("{t:?}")));
                    }
                }
                let pred = match &reranker {
                    Some(rr) => {
                        let mut lists = vec![am2.topk(&p, rk)];
                        for mv in &extra { lists.push(mv.topk(&p, rk)); }
                        let mut cs = math::steps::merge_votes(lists);
                        // trigger "ignore irrelevant numbers": search again without the flagged numbers (hard ban)
                        // MATH_IGNORE=1 — soft (branches without flagged numbers first), =2 — hard (search again without them)
                        let mode_ign = std::env::var("MATH_IGNORE").ok().and_then(|x| x.parse::<u8>().ok()).unwrap_or(0);
                        let ign: Vec<usize> = if mode_ign >= 3 {
                            // v3: learned binding with a threshold chosen on train (3 — soft, 4 — hard)
                            let real: Vec<usize> = (0..p.qs.len()).filter(|&i| !p.qs[i].name.starts_with("const")).collect();
                            let fl: Vec<usize> = real.iter().copied().filter(|&i| bind_model.score(&math::steps::bind_feats(&p, i)) <= bind_th).collect();
                            if real.len() - fl.len() >= 2 { fl } else { Vec::new() }
                        } else if mode_ign > 0 { math::steps::ignore_set(&p) } else { Vec::new() };
                        let mode_ign = match mode_ign { 3 => 1, 4 => 2, x => x };
                        if !ign.is_empty() && (mode_ign == 2 || cs.iter().any(|c| !math::steps::uses_any(&c.tree, &ign))) {
                            let before_ok = cs.first().is_some_and(|c| (c.val - g).abs() < 1e-6);
                            if mode_ign == 2 {
                                let mut p2 = math::steps::prepare(&ann, q, maxq);
                                p2.sent_k = p.sent_k.clone();
                                p2.forbid = (0..p2.qs.len()).map(|i| ign.contains(&i)).collect();
                                let cs2 = am2.topk(&p2, rk);
                                if !cs2.is_empty() { cs = cs2; }
                            } else {
                                cs.sort_by_key(|c| math::steps::uses_any(&c.tree, &ign));
                            }
                            let after_ok = cs.first().is_some_and(|c| (c.val - g).abs() < 1e-6);
                            *ign_trig.entry(match (before_ok, after_ok) { (false, true) => "ignore — helpful", (true, false) => "ignore — harmful", (true, true) => "ignore — redundant (correct anyway)", (false, false) => "ignore — did not help" }).or_default() += 1;
                        }
                        if cs.iter().any(|c| (c.val - g).abs() < 1e-6) { in_topk += 1; }
                        if cs.first().is_some_and(|c| (c.val - g).abs() < 1e-6) { top1_base += 1; }
                        let top = cs.first().map(|c| c.score).unwrap_or(0.0);
                        let fs: Vec<Vec<String>> = cs.iter().enumerate().map(|(r, c)| math::steps::cand_feats(&p, c, r, top)).collect();
                        if cs.is_empty() { None } else {
                            let i = rr.pick(&fs);
                            // self-assessment: confidence — softmax of the judge's scores; check — exact recomputation by the core
                            let sc = rr.scores(&fs);
                            let mx = sc.iter().cloned().fold(f64::MIN, f64::max);
                            let z: f64 = sc.iter().map(|x| (x - mx).exp()).sum();
                            let conf = (sc[i] - mx).exp() / z;
                            // second opinion: the first model (operation tree, top-1) versus the judge (global features)
                            let mut agree = i == 0;
                            let pick_val = cs[i].val;
                            let mut rethought = false;
                            if !agree && rethink {
                                // rethinking: a second pass with a wider beam and no judge — whose opinion holds up?
                                rethought = true;
                                let wide = am2.with_beam(beam * 4);
                                if let Some(pr2) = wide.predict(&p) {
                                    let v2 = pr2.tree.exact(&p.qs).map(|x| math::big::to_f64(&x)).unwrap_or(f64::NAN);
                                    // the second pass only confirms the judge; matching the first opinion is not yet agreement
                                    if (v2 - cs[i].val).abs() < 1e-6 {
                                        agree = true;
                                    }
                                }
                            }
                            let ok_i = (pick_val - g).abs() < 1e-6;
                            // third opinion — the world written down by the snake
                            let wld = match &world_model { Some((wm, _)) => math::qworld::write_learned(&p, wm), None => math::qworld::write(&p) };
                            let wans = match &world_model { Some((_, am)) => math::qworld::ask_tmpl(&p, &wld, am.predict(&p)), None => math::qworld::ask(&p, &wld) };
                            match wans {
                                Some((wv, why, _)) => {
                                    let wok = (math::big::to_f64(&wv) - g).abs() < 1e-6;
                                    let agree_w = (math::big::to_f64(&wv) - pick_val).abs() < 1e-6;
                                    *wtrig.entry(match (wok, agree_w) { (true, true) => "world: correct, agrees with judge", (true, false) => "world: correct, judge not", (false, true) => "world: wrong, agrees with judge", (false, false) => "world: wrong, disagrees" }).or_default() += 1;
                                    if k < explain_n { println!("    snake's world: {} → {} ({why}) {}", wld.log.join("; "), math::big::show(&wv), if wok { "✓" } else { "✗" }); }
                                }
                                None => { *wtrig.entry("world: don't know").or_default() += 1; }
                            }
                            if k < explain_n {
                                println!("EXPLANATION {id}: {q}");
                                for line in math::steps::explain_steps(&p, &cs[i].tree) { println!("    {line}"); }
                                println!("    {} (gold {g}); opinions {}", if ok_i { "✓" } else { "✗" }, if agree { "agree" } else { "disagree — filing a report" });
                            }
                            let secs = t_item.elapsed().as_secs_f64();
                            let actions = math::steps::ACTIONS.swap(0, std::sync::atomic::Ordering::Relaxed);
                            let rss_kb = std::fs::read_to_string("/proc/self/statm").ok().and_then(|t| t.split_whitespace().nth(1).and_then(|x| x.parse::<u64>().ok())).unwrap_or(0) * 4;
                            if !agree && hint_cases.len() < hints_n {
                                // explanation mode for the LLM: problem, branches with steps, choice — wide input; the hint is dense
                                let mut pk = format!("PROBLEM: {q}\nMY BRANCHES (step-by-step, as I see them):\n");
                                for (bi, c) in cs.iter().take(3).enumerate() {
                                    pk.push_str(&format!("  branch {bi} = {}:\n", c.val));
                                    for line in math::steps::explain_steps(&p, &c.tree) { pk.push_str(&format!("    {line}\n")); }
                                }
                                pk.push_str(&format!("MY FIRST THOUGHT: branch 0; MY JUDGE: branch {i}; they disagree, so I report instead of answering.\n"));
                                hint_cases.push((id.clone(), pk, *g, cs.iter().take(3).map(|c| c.val).collect::<Vec<f64>>()));
                            }
                            if !agree {
                                // a report instead of making things up: what I tried, my judgments, what to teach me
                                let (vs, ns) = math::steps::unknowns(&p);
                                for v in &vs { *teach_verbs.entry(v.clone()).or_default() += 1; }
                                for n in &ns { *teach_nouns.entry(n.clone()).or_default() += 1; }
                                if reports.len() < 3 {
                                    reports.push(format!("REPORT {id}: not confident.\n    tried: {}\n    first opinion {} = {}, judge {} = {}{}\n    teach me: verbs {:?}, nouns {:?}", cs.iter().take(3).map(|c| format!("{} = {}", c.tree.show(&p.qs), c.val)).collect::<Vec<_>>().join("; "), cs[0].tree.show(&p.qs), cs[0].val, cs[i].tree.show(&p.qs), cs[i].val, if rethought { "; rethought with a wider pass — no agreement" } else { "" }, vs, ns));
                                }
                            }
                            // trigger debug: did the judge change the first opinion? did the second opinion produce a report?
                            let base_ok = (cs[0].val - g).abs() < 1e-6;
                            let judge_ok = (cs[i].val - g).abs() < 1e-6;
                            let e = trig.entry(if i != 0 { if judge_ok { "judge changed — helpful" } else if base_ok { "judge changed — harmful" } else { "judge changed — both wrong" } } else { "judge did not change" }).or_default();
                            *e += 1;
                            if !agree {
                                *trig.entry(if base_ok || judge_ok { "report — lost the correct answer" } else { "report — justified (both opinions wrong)" }).or_default() += 1;
                            }
                            // retuned trigger (after debugging): report only when the opinions disagree AND the judge is unsure
                            let report2 = !agree && conf < 0.9;
                            if report2 {
                                *trig.entry(if base_ok || judge_ok { "report v2 — lost the correct one" } else { "report v2 — justified" }).or_default() += 1;
                            } else {
                                *trig.entry(if judge_ok { "v2: answered — correct" } else { "v2: answered — wrong" }).or_default() += 1;
                            }
                            selfs.push((global::SelfReport { branches: cs.len(), verified: agree, confidence: conf, escalated: false, secs, usual_secs: 0.0, correct: Some(ok_i), actions, usual_actions: 0, rss_kb, rethought }, ok_i));
                            let miss_show: usize = std::env::var("MATH_SHOW_MISS").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
                            if (cs[i].val - g).abs() > 1e-6 && miss_shown < miss_show {
                                if let Some(j) = cs.iter().position(|c| (c.val - g).abs() < 1e-6) {
                                    miss_shown += 1;
                                    println!("MISS {id}: {q}\n  chosen #{i} {} = {}\n  correct #{j} {} = {}", cs[i].tree.show(&p.qs), cs[i].val, cs[j].tree.show(&p.qs), cs[j].val);
                                }
                            }
                            let c = &cs[i];
                            Some(math::steps::Pred { tree: c.tree.clone(), score: c.score, margin: if i == 0 { cs.get(1).map(|x| c.score - x.score).unwrap_or(10.0) } else { 0.0 }, acts: c.acts.clone() })
                        }
                    }
                    None => am2.predict(&p),
                };
                match pred {
                    Some(pr) => {
                        let v = pr.tree.exact(&p.qs).map(|x| math::big::to_f64(&x)).unwrap_or(f64::NAN);
                        let ok = (v - g).abs() < 1e-6;
                        if k < show {
                            println!("{id} {} = {v} (gold {g}) {} margin {:.1}", pr.tree.show(&p.qs), if ok { "✓" } else { "✗" }, pr.margin);
                            for e in am2.explain(&p, &pr.acts) { println!("    {e}"); }
                        }
                        rows.push((ok, pr.margin));
                        dump.push_str(&format!("{id}\t{}\t{v}\t{}\n", u8::from(ok), pr.tree.show(&p.qs)));
                    }
                    None => {
                        rows.push((false, f64::NEG_INFINITY));
                        dump.push_str(&format!("{id}\t0\tNaN\t-\n"));
                    }
                }
            }
            if let Ok(f) = std::env::var("MATH_DUMP") {
                std::fs::write(&f, &dump)?;
            }
            let n = rows.len();
            let acc = rows.iter().filter(|r| r.0).count();
            println!("{set} test: MMM alone (v4) correct {acc}/{n} ({:.1}%)", 100.0 * acc as f64 / n as f64);
            if rk > 0 { println!("  top-{rk}: correct among them {in_topk}, first without reranking {top1_base}"); }
            if !selfs.is_empty() {
                // humility as routing: below the threshold — I do not answer, I call the LLM
                let usual = { let mut t: Vec<f64> = selfs.iter().map(|s| s.0.secs).collect(); t.sort_by(|a, b| a.partial_cmp(b).unwrap()); t[t.len() / 2] };
                println!("  self-assessment (humility: below the threshold — I do not answer, I file a report):");
                for th in [0.0, 0.3, 0.5, 0.7, 0.9] {
                    let ans: Vec<&(global::SelfReport, bool)> = selfs.iter().filter(|s| s.0.confidence >= th).collect();
                    let ok = ans.iter().filter(|s| s.1).count();
                    println!("    threshold {th:.1}: answers {}/{} ({:.0}%), accuracy {:.1}%; the rest — report", ans.len(), selfs.len(), 100.0 * ans.len() as f64 / selfs.len() as f64, 100.0 * ok as f64 / ans.len().max(1) as f64);
                }
                // second opinion as routing: I answer by myself only when the opinions agree
                let ag: Vec<&(global::SelfReport, bool)> = selfs.iter().filter(|s| s.0.verified).collect();
                let ok = ag.iter().filter(|s| s.1).count();
                println!("    opinions agree{}: answers {}/{} ({:.0}%), accuracy {:.1}%; the rest — report", if rethink { " (with rethinking)" } else { "" }, ag.len(), selfs.len(), 100.0 * ag.len() as f64 / selfs.len() as f64, 100.0 * ok as f64 / ag.len().max(1) as f64);
                let ag9: Vec<&(global::SelfReport, bool)> = ag.iter().copied().filter(|s| s.0.confidence >= 0.9).collect();
                println!("    agree and confidence ≥ 0.9: {}/{} ({:.0}%), accuracy {:.1}%", ag9.len(), selfs.len(), 100.0 * ag9.len() as f64 / selfs.len() as f64, 100.0 * ag9.iter().filter(|s| s.1).count() as f64 / ag9.len().max(1) as f64);
                let mut acts: Vec<usize> = selfs.iter().map(|s| s.0.actions).collect();
                acts.sort();
                let usual_actions = acts[acts.len() / 2];
                let th: f64 = std::env::var("MATH_HUMBLE").ok().and_then(|s| s.parse().ok()).unwrap_or(0.7);
                let mut counts: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
                let mut example: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
                for (r, _) in &selfs {
                    let mut r = r.clone();
                    r.usual_secs = usual;
                    r.usual_actions = usual_actions;
                    r.escalated = !r.verified || r.confidence < th;
                    if r.escalated { r.correct = None; }
                    for j in global::self_judge(&r) {
                        let key = format!("{} \"{}\"", if j.violated { "✗ against" } else { "✓ in spirit" }, j.name);
                        *counts.entry(key.clone()).or_default() += 1;
                        example.entry(key).or_insert(j.because.clone());
                    }
                }
                for r in &reports { println!("  {r}"); }
                if !hint_cases.is_empty() {
                    // the LLM as a separate tool (not training): a dense hint — what I forgot to look at
                    let v = math::vmm::Vmm::new(run_dir().join("hints-cwd"), run_dir().join("hints-calls.jsonl"), 5.0)?;
                    let mut helped = 0;
                    for (bi, ch) in hint_cases.chunks(10).enumerate() {
                        let mut pr = String::from("A small transparent solver explains how it sees each word problem and where it is unsure. For EACH case give ONE dense hint (max 15 words): what it forgot to look at or which branch is right and why. Do not compute a long solution. Answer ONLY lines `<n>: branch=<0|1|2|none> hint=<text>`.\n\n");
                        for (k, c) in ch.iter().enumerate() { pr.push_str(&format!("CASE {k}\n{}\n", c.1)); }
                        let (text, _) = v.ask("hints", bi, ch.len(), &pr, 0.5)?;
                        for l in text.lines() {
                            let Some((n, rest)) = l.split_once(':') else { continue };
                            let Ok(k) = n.trim().parse::<usize>() else { continue };
                            let Some(c) = ch.get(k) else { continue };
                            let br = rest.split_whitespace().find_map(|w| w.strip_prefix("branch=")).and_then(|b| b.parse::<usize>().ok());
                            let hint = rest.split("hint=").nth(1).unwrap_or("").trim();
                            let ok = br.and_then(|b| c.3.get(b)).is_some_and(|v| (v - c.2).abs() < 1e-6);
                            if ok { helped += 1; }
                            println!("  HINT {}: {hint} (branch {:?} {})", c.0, br, if ok { "✓ correct" } else { "✗" });
                        }
                    }
                    println!("  LLM hints: {} cases, hinted branch correct in {helped}; spent ${:.2} (equiv.)", hint_cases.len(), v.spent());
                }
                println!("  who I am (level 1): {}", global::self_describe("mmm").join(" → "));
                println!("  trigger debug — module \"judge\": {}", global::self_describe("module_judge").first().copied().unwrap_or(""));
                println!("                  module \"second opinion\": {}", global::self_describe("module_second_opinion").first().copied().unwrap_or(""));
                for (k, n) in &trig { println!("    {k}: {n}"); }
                for (k, n) in &ign_trig { println!("    {k}: {n}"); }
                for (k, n) in &wtrig { println!("    {k}: {n}"); }
                let top = |m: &std::collections::BTreeMap<String, usize>| { let mut v: Vec<(&String, &usize)> = m.iter().collect(); v.sort_by(|a, b| b.1.cmp(a.1)); v.iter().take(15).map(|(k, n)| format!("{k}×{n}")).collect::<Vec<_>>().join(", ") };
                println!("  teach me (from reports): verbs — {}\n                           nouns — {}", top(&teach_verbs), top(&teach_nouns));
                println!("  self-assessment by principles (threshold {th}):");
                for (k, n) in &counts { println!("    {k}: {n} — e.g.: {}", example[k]); }
            }
            rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            for frac in [0.1, 0.2, 0.3, 0.5] {
                let k = ((n as f64) * frac) as usize;
                println!("  most confident {:.0}% ({k}): accuracy {:.1}%", frac * 100.0, 100.0 * rows[..k].iter().filter(|r| r.0).count() as f64 / k.max(1) as f64);
            }
            println!("total {:.1} s", t0.elapsed().as_secs_f64());
        }
        "arith" => {
            // v3: training subproblem trees on GSM8K train (weak supervision from the gold answer), measured on GSM8K test
            let n_train: usize = flag(&args, "--train").and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
            let epochs: usize = flag(&args, "--epochs").and_then(|s| s.parse().ok()).unwrap_or(3);
            let leaves: usize = flag(&args, "--leaves").and_then(|s| s.parse().ok()).unwrap_or(4);
            let beam: usize = flag(&args, "--beam").and_then(|s| s.parse().ok()).unwrap_or(20);
            let maxq: usize = flag(&args, "--maxq").and_then(|s| s.parse().ok()).unwrap_or(9);
            let show: usize = flag(&args, "--show").and_then(|s| s.parse().ok()).unwrap_or(0);
            let load = |f: &str| -> Result<Vec<(String, String, f64)>> {
                let text = std::fs::read_to_string(home().join("raw/mathsolve/gsm8k").join(f))?;
                Ok(text.lines().enumerate().filter_map(|(i, l)| {
                    let v: serde_json::Value = serde_json::from_str(l).ok()?;
                    let a = v["answer"].as_str()?;
                    let g: f64 = a.rsplit("####").next()?.trim().replace(',', "").parse().ok()?;
                    Some((format!("{f}-{i}"), v["question"].as_str()?.to_string(), g))
                }).collect())
            };
            let only_plans = args.iter().any(|a| a == "--plans");
            // LLM plan supervision: the numbers that a verified level-5 plan actually used
            let mut used: HashMap<String, Vec<f64>> = HashMap::new();
            for r in solve::load_records(&records) {
                if r.level == curric::TRAIN_LEVEL && r.config == "B" && r.verified && r.correct {
                    let i: usize = r.id.trim_start_matches("gsm8k-train-").parse().unwrap_or(usize::MAX);
                    let txt = r.plan.to_string();
                    let vals: Vec<f64> = math::arith::quantities(&txt.replace("\\\"", " ")).0.iter().map(|x| x.val).collect();
                    used.insert(format!("train.jsonl-{i}"), vals);
                }
            }
            let train: Vec<_> = load("train.jsonl")?.into_iter().filter(|(id, _, _)| !only_plans || used.contains_key(id)).take(n_train).collect();
            println!("LLM plan supervision: {} problems{}", used.len(), if only_plans { " (only them)" } else { "" });
            let test = load("test.jsonl")?;
            let t0 = std::time::Instant::now();
            let prep: Vec<_> = train.iter().map(|(id, q, g)| (id, math::arith::prepare(q, maxq), *g)).collect();
            let mut m = math::arith::Model::new(leaves, beam);
            // coverage ceiling: whether a tree with ≤ leaves leaves exists at all that yields the gold answer
            let golds: Vec<Option<math::arith::Tree>> = prep.iter().map(|(id, p, g)| match used.get(*id) { Some(u) => m.gold_tree_used(p, *g, u), None => m.gold_tree(p, *g) }).collect();
            let cover = golds.iter().filter(|g| g.is_some()).count();
            println!("training {}; a tree of up to {leaves} leaves exists for {cover} ({:.1}%); {:.1} s", prep.len(), 100.0 * cover as f64 / prep.len().max(1) as f64, t0.elapsed().as_secs_f64());
            for e in 0..epochs {
                let mut ok = 0;
                for ((id, p, g), gt) in prep.iter().zip(&golds) {
                    // the gold tree is recomputed with the current model (the most probable among the smallest)
                    if gt.is_none() { continue; }
                    let gt2 = match used.get(*id) { Some(_) => gt.clone().unwrap(), None => m.gold_tree(p, *g).unwrap_or_else(|| gt.clone().unwrap()) };
                    if m.learn(p, &gt2, *g) { ok += 1; }
                }
                println!("epoch {}: correct on training {ok}/{cover}; {:.1} s", e + 1, t0.elapsed().as_secs_f64());
            }
            let am = m.averaged();
            let mut rows: Vec<(bool, f64)> = Vec::new();
            let mut cover_t = 0;
            let mut by_size: std::collections::BTreeMap<usize, (usize, usize)> = std::collections::BTreeMap::new();
            for (k, (id, q, g)) in test.iter().enumerate() {
                let p = math::arith::prepare(q, maxq);
                let gsz = am.gold_tree(&p, *g).map(|t| t.show(&p.qs).matches('(').count() + 1);
                if gsz.is_some() { cover_t += 1; }
                let e = by_size.entry(gsz.unwrap_or(99)).or_insert((0usize, 0usize));
                e.1 += 1;
                match am.predict(&p, true) {
                    Some((t, _, margin)) => {
                        let v = t.exact(&p.qs).map(|x| math::big::to_f64(&x)).unwrap_or(f64::NAN);
                        let ok = (v - g).abs() < 1e-6;
                        if k < show {
                            println!("{id} {} = {v} (gold {g}) {} margin {margin:.1}", t.show(&p.qs), if ok { "✓" } else { "✗" });
                            for e in am.explain(&p, &t) { println!("    {e}"); }
                        }
                        rows.push((ok, margin));
                        if ok { by_size.get_mut(&gsz.unwrap_or(99)).unwrap().0 += 1; }
                    }
                    None => rows.push((false, f64::NEG_INFINITY)),
                }
            }
            let n = rows.len();
            let acc = rows.iter().filter(|r| r.0).count();
            println!("GSM8K test: MMM alone correct {acc}/{n} ({:.1}%); a tree exists for {cover_t} ({:.1}%)", 100.0 * acc as f64 / n as f64, 100.0 * cover_t as f64 / n as f64);
            for (sz, (ok, n)) in &by_size {
                println!("  smallest tree {} leaves: {ok}/{n} ({:.1}%)", if *sz == 99 { "none ≤L".to_string() } else { sz.to_string() }, 100.0 * *ok as f64 / (*n).max(1) as f64);
            }
            // accuracy versus coverage by margin: the MMM answers only when confident, the rest goes to the LLM
            rows.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            for frac in [0.1, 0.2, 0.3, 0.5, 1.0] {
                let k = ((n as f64) * frac) as usize;
                let c = rows[..k].iter().filter(|r| r.0).count();
                println!("  most confident {:.0}% ({k}): accuracy {:.1}%", frac * 100.0, 100.0 * c as f64 / k.max(1) as f64);
            }
            println!("total {:.1} s", t0.elapsed().as_secs_f64());
        }
        "run" => {
            let level: u8 = args.get(1).context("level")?.parse()?;
            let config = args.get(2).context("configuration A|B")?.to_uppercase();
            let split = flag(&args, "--split").unwrap_or_else(|| "all".into());
            let limit: usize = flag(&args, "--limit").and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
            let batch: usize = flag(&args, "--batch").and_then(|s| s.parse().ok()).unwrap_or(10);
            let par: usize = flag(&args, "--par").and_then(|s| s.parse().ok()).unwrap_or(2);
            let reserve: f64 = flag(&args, "--reserve").and_then(|s| s.parse().ok()).unwrap_or(1.0);
            let tag = flag(&args, "--tag").unwrap_or_else(|| format!("L{level}"));
            let done: std::collections::HashSet<(String, String)> = solve::load_records(&records).into_iter().filter(|r| r.error.is_none()).map(|r| (r.id, r.config)).collect();
            let ps: Vec<Problem> = curric::load_level(&dir.join("levels"), level)?.into_iter().filter(|p| split == "all" || p.split == split).filter(|p| !done.contains(&(p.id.clone(), config.clone()))).take(limit).collect();
            let v = Vmm::new(dir.join("cwd"), calls.clone(), cap)?;
            eprintln!("level {level}, configuration {config}: {} problems; spent before start ${:.2} of the ${cap:.2} cap", ps.len(), v.spent());
            let r = Run { vmm: &v, records: records.clone(), batch, par, reserve, max_retries: 2, plan_secs: 20.0 };
            let recs = match config.as_str() {
                "A" => solve::run_a(&r, &ps, &tag),
                "B" => solve::run_b(&r, &ps, &tag),
                _ => bail!("configuration A or B"),
            };
            let refs: Vec<&Record> = recs.iter().collect();
            let s = solve::summarize(&refs);
            println!("{config}: {} problems, correct {} ({:.1}%), verified {}, verified and correct {}, call errors {}, ${:.3}, output tokens {}, {:.0} s", s.n, s.correct, 100.0 * s.correct as f64 / s.n.max(1) as f64, s.verified, s.verified_correct, s.errors, s.cost, s.out_tok, s.secs);
            println!("spent in total ${:.2}", v.spent());
        }
        "c" => {
            let level: u8 = args.get(1).context("level")?.parse()?;
            let min_margin: f64 = flag(&args, "--margin").and_then(|s| s.parse().ok()).unwrap_or(0.5);
            let min_sim: f64 = flag(&args, "--sim").and_then(|s| s.parse().ok()).unwrap_or(0.9);
            let all = solve::load_records(&records);
            let mut probs: Vec<Problem> = Vec::new();
            for l in curric::MEMORY_LEVELS {
                if let Ok(ps) = curric::load_level(&dir.join("levels"), l) {
                    probs.extend(ps);
                }
            }
            // memory: verified B of all levels, except the held-out split of the current one
            let b_train: Vec<Record> = all.iter().filter(|r| r.config == "B" && !(r.level == level && r.split == "held")).cloned().collect();
            let memos = solve::memos_from(&probs, &b_train);
            let held: Vec<Problem> = probs.iter().filter(|p| p.level == level && p.split == "held").cloned().collect();
            let b: HashMap<String, Record> = all.iter().filter(|r| r.config == "B" && r.level == level).map(|r| (r.id.clone(), r.clone())).collect();
            let recs = solve::run_c(&held, &memos, &b, min_margin, min_sim, 20.0);
            // C — in a separate file (A/B runs append to records.jsonl in parallel); replace C of this level
            let cpath = dir.join("records-c.jsonl");
            let keep: Vec<Record> = solve::load_records(&cpath).into_iter().filter(|r| r.level != level).collect();
            let mut text = String::new();
            for r in keep.iter().chain(recs.iter()) {
                text.push_str(&serde_json::to_string(r)?);
                text.push('\n');
            }
            std::fs::write(&cpath, text)?;
            let refs: Vec<&Record> = recs.iter().collect();
            let s = solve::summarize(&refs);
            let agree = recs.iter().filter(|r| r.mmm_method.is_some() && b.get(&r.id).is_some_and(|br| Some(&br.method) == r.mmm_method.as_ref())).count();
            println!("C, level {level}: memory {} solutions; held-out {}; MMM alone {} (correct {}); perceptron method = LLM method in {agree}; correct overall {}", memos.len(), s.n, s.by_mmm, s.by_mmm_correct, s.correct);
            for r in recs.iter().filter(|r| r.by_mmm) {
                println!("  {} — method {} (margin {:.2}; features {}), template from {}, answer {} {}", r.id, r.method, r.mmm_margin.unwrap_or(0.0), r.mmm_why.iter().map(|(f, w)| format!("{f}:{w:.2}")).collect::<Vec<_>>().join(" "), r.mmm_template_from.clone().unwrap_or_default(), r.answer.clone().unwrap_or_default(), if r.correct { "✓" } else { "✗" });
            }
        }
        "memory" => {
            let all = solve::load_records(&records);
            let mut probs = Vec::new();
            for l in curric::MEMORY_LEVELS {
                if let Ok(ps) = curric::load_level(&dir.join("levels"), l) {
                    probs.extend(ps);
                }
            }
            let memos = solve::memos_from(&probs, &all);
            let mpath = dir.join("memory.jsonl");
            let mut text = String::new();
            for m in &memos {
                text.push_str(&serde_json::to_string(m)?);
                text.push('\n');
            }
            std::fs::write(&mpath, text)?;
            println!("memory: {} verified solutions → {}", memos.len(), mpath.display());
            let db = home().join("db/mathsolve.duckdb");
            let sql = format!(
                "CREATE OR REPLACE TABLE solutions AS SELECT id, level, split, source, method, answer, correct, checks_ok, checks_total, template IS NOT NULL AS has_template, question, feats, plan FROM read_json_auto('{}', maximum_object_size=67108864);\
                 CREATE OR REPLACE TABLE records AS SELECT id, level, split, config, answer, option, verified, correct, rounds, method, cost_usd, input_tokens, output_tokens, secs, by_mmm, mmm_method, mmm_margin, error FROM read_json_auto(['{}', '{}'], maximum_object_size=67108864, union_by_name=true);\
                 CREATE OR REPLACE TABLE calls AS SELECT * FROM read_json_auto('{}');\
                 SELECT (SELECT count(*) FROM solutions) AS solutions, (SELECT count(*) FROM records) AS records, (SELECT count(*) FROM calls) AS calls;",
                mpath.display(), records.display(), dir.join("records-c.jsonl").display(), calls.display()
            );
            let out = std::process::Command::new(home().join("bin/duckdb")).arg(&db).arg("-c").arg(&sql).output()?;
            println!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
        }
        "report" => {
            let mut all = solve::load_records(&records);
            all.extend(solve::load_records(&dir.join("records-c.jsonl")));
            println!("| level | split | configuration | problems | correct | % | verified | MMM alone (correct) | $ | $/problem | output tokens | s |");
            println!("|---|---|---|---|---|---|---|---|---|---|---|---|");
            for level in 0..=4u8 {
                for split in ["dev", "held"] {
                    for config in ["A", "B", "C"] {
                        let rs: Vec<&Record> = all.iter().filter(|r| r.level == level && r.split == split && r.config == config).collect();
                        if rs.is_empty() {
                            continue;
                        }
                        let s = solve::summarize(&rs);
                        println!("| {level} | {split} | {config} | {} | {} | {:.1} | {} | {} ({}) | {:.2} | {:.4} | {} | {:.0} |", s.n, s.correct, 100.0 * s.correct as f64 / s.n as f64, s.verified, s.by_mmm, s.by_mmm_correct, s.cost, s.cost / s.n as f64, s.out_tok, s.secs);
                    }
                }
            }
            let cs = vmm::read_calls(&calls)?;
            println!("\nLLM calls: {}, spent ${:.2}", cs.len(), cs.iter().map(|c| c.cost_usd).sum::<f64>());
        }
        "plan" => {
            let text = std::fs::read_to_string(args.get(1).context("plan file")?)?;
            for p in plan::parse_plans(&text) {
                match p {
                    Ok(p) => {
                        let t = plan::run(&p, &Expect::default(), 20.0);
                        println!("plan {} ({}): {}", p.id, p.method, if t.verified() { "VERIFIED" } else { "FAILED THE GATE" });
                        print!("{}", plan::show_trace(&t));
                    }
                    Err(e) => println!("{e}"),
                }
            }
        }
        "calc" => {
            for src in &args[1..] {
                let e = calc::parse(src).map_err(anyhow::Error::msg)?;
                let mut env = calc::Env::new(math::nt::Limits::with_secs(20.0));
                let v = calc::eval(&e, &mut env).map_err(anyhow::Error::msg)?;
                println!("expr:    {e}");
                for s in &env.subs {
                    println!("  · {} = {}", s.rule, s.result);
                    for c in &s.checks {
                        println!("      {} {}{}", if c.ok { "✓" } else { "✗" }, c.name, if c.detail.is_empty() { String::new() } else { format!(" — {}", c.detail) });
                    }
                }
                if let Some(c) = calc::independent(&e, &env.vars, &v) {
                    println!("  {} {} {}", if c.ok { "✓" } else { "✗" }, c.name, c.detail);
                }
                println!("answer: {}", v.exact());
            }
        }
        "controls" => {
            let mut red = 0;
            let mut total = 0;
            let mut ctl = |name: &str, caught: bool| {
                total += 1;
                red += caught as usize;
                println!("{} {name}", if caught { "RED (caught)" } else { "GREEN — GATE DID NOT FIRE" });
            };
            let p = plan::plan_from_json(r#"{"id":"t","steps":[{"id":"a","op":"compute","expr":"3^40 + 17"},{"id":"b","op":"compute","expr":"a mod 1000"}],"answer":"b"}"#).map_err(anyhow::Error::msg)?;
            let bad = plan::run_with_tamper(&p, &Expect::default(), "a", "12157665459056928819");
            ctl("tampered intermediate result in the trace (3^40+17 → +1)", !bad.fails.is_empty());
            let p = plan::plan_from_json(r#"{"id":"g","steps":[{"id":"x","op":"compute","expr":"48/2"},{"id":"t","op":"compute","expr":"48 + x"}],"answer":"t","type":"integer"}"#).map_err(anyhow::Error::msg)?;
            let bad = plan::run_with_tamper(&p, &Expect { integer: true, ..Default::default() }, "x", "25");
            ctl("tampering in a GSM trace (48/2 → 25)", !bad.fails.is_empty());
            let p = plan::plan_from_json(r#"{"id":"h","steps":[{"op":"claim","vars":[["n",0,60]],"expr":"isprime(n^2+n+41)","why":"Euler's polynomial"}],"answer":"1"}"#).map_err(anyhow::Error::msg)?;
            let t = plan::run(&p, &Expect::default(), 10.0);
            ctl(&format!("false conjecture n²+n+41 is prime ({})", t.fails.join("; ")), !t.verified());
            let p = plan::plan_from_json(r#"{"id":"h2","steps":[{"op":"claim","vars":[["n",1,40]],"expr":"2^(2^n) + 1 < 10 or isprime(2^(2^n)+1)","why":"Fermat"}],"answer":"1"}"#).map_err(anyhow::Error::msg);
            if let Ok(p) = p {
                let t = plan::run(&p, &Expect::default(), 20.0);
                ctl(&format!("false Fermat conjecture: 2^(2^n)+1 is prime ({})", t.fails.join("; ").chars().take(160).collect::<String>()), !t.verified());
            }
            let p = plan::plan_from_json(r#"{"id":"w","steps":[{"id":"r","op":"solve","eqs":["x^2 - 5x + 6 = 0"],"vars":["x"]},{"op":"check","expr":"r == [2, 4]","why":"wrong root"}],"answer":"r"}"#).map_err(anyhow::Error::msg)?;
            let t = plan::run(&p, &Expect::default(), 10.0);
            ctl("false LLM check (roots [2,4])", !t.verified());
            let p = plan::plan_from_json(r#"{"id":"o","steps":[{"id":"a","op":"compute","expr":"2*7"}],"answer":"a","option":"A"}"#).map_err(anyhow::Error::msg)?;
            let t = plan::run(&p, &Expect { integer: false, options: vec![("A".into(), "$$12$$".into()), ("B".into(), "$$14$$".into())] }, 10.0);
            ctl("option A=12 chosen, but 14 computed", !t.verified());
            let p = plan::plan_from_json(r#"{"id":"i","steps":[{"id":"a","op":"compute","expr":"17/4"}],"answer":"a"}"#).map_err(anyhow::Error::msg)?;
            let t = plan::run(&p, &Expect { integer: true, ..Default::default() }, 10.0);
            ctl("non-integer result where a count of items is expected (17/4)", !t.verified());
            println!("negative controls red: {red} of {total}");
            if red != total {
                bail!("the gate let a false result through");
            }
        }
        "show" => {
            // problem, plan, trace with checks, gate history
            let id = args.get(1).context("id")?;
            let config = args.get(2).cloned().unwrap_or_else(|| "B".into());
            let mut all = solve::load_records(&records);
            all.extend(solve::load_records(&dir.join("records-c.jsonl")));
            let r = all.iter().rev().find(|r| &r.id == id && r.config == config).context("no record")?;
            for l in curric::MEMORY_LEVELS {
                if let Some(p) = curric::load_level(&dir.join("levels"), l).ok().and_then(|ps| ps.into_iter().find(|p| &p.id == id)) {
                    println!("problem ({}, {}): {}", p.source, p.license, p.question.trim());
                    for (k, c) in &p.options {
                        println!("  ({k}) {c}");
                    }
                    println!("gold: {}", p.gold);
                }
            }
            println!("method: {}; rounds: {}; verified: {}; correct: {}; ${:.4}", r.method, r.rounds, r.verified, r.correct, r.cost_usd);
            for h in &r.history {
                println!("gate returned to the LLM: {h}");
            }
            println!("plan: {}", serde_json::to_string(&r.plan)?);
            if let Some(t) = &r.trace {
                print!("{}", plan::show_trace(t));
            }
        }
        "lean-prep" => {
            // ProofNet# (MIT): Lean 4 statements from textbooks; no Putnam (competition problems are a grey zone).
            // We take only those that elaborate in our Mathlib (check `:= by sorry` → warnings only).
            let raw = home().join("raw/mathsolve/proofnet-sharp");
            let per: usize = flag(&args, "--per").and_then(|s| s.parse().ok()).unwrap_or(5);
            let mut out = String::new();
            let tmp = dir.join("lean-elab");
            for (file, split) in [("valid.jsonl", "dev"), ("test.jsonl", "held")] {
                let text = std::fs::read_to_string(raw.join(file))?;
                let all: Vec<serde_json::Value> = text.lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
                let ths: Vec<math::lean::Theorem> = all.iter().filter(|v| !v["id"].as_str().unwrap_or("").starts_with("Putnam")).map(|v| math::lean::Theorem {
                    id: v["id"].as_str().unwrap_or("").to_string(),
                    split: split.into(),
                    header: v["lean4_src_header"].as_str().unwrap_or("").to_string(),
                    statement: v["lean4_formalization"].as_str().unwrap_or("").to_string(),
                    nl: v["nl_statement"].as_str().unwrap_or("").to_string(),
                }).collect();
                let order = curric::sample(&ths, |t| t.id.clone(), ths.len(), &format!("L4{split}"));
                let mut kept = 0;
                for th in order {
                    if kept >= per {
                        break;
                    }
                    // statement elaboration: sorry gives only the warning "declaration uses 'sorry'"
                    let src = math::lean::assemble(&th, "sorry");
                    std::fs::create_dir_all(&tmp)?;
                    let safe: String = th.id.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect();
                    let f = tmp.join(format!("{safe}.lean"));
                    std::fs::write(&f, src)?;
                    let o = std::process::Command::new("timeout").arg("180").arg(home().join(".elan/bin/lake")).args(["env", "lean"]).arg(&f).current_dir(math::lean::project()).output()?;
                    let text = format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
                    let has_err = text.contains("error");
                    println!("{} {}: {}", split, th.id, if has_err { "statement does not elaborate — skipping" } else { "ok" });
                    if !has_err {
                        out.push_str(&serde_json::to_string(&th)?);
                        out.push('\n');
                        kept += 1;
                    }
                }
            }
            std::fs::write(dir.join("levels/level4.jsonl"), out)?;
        }
        "lean-controls" => {
            let th = math::lean::Theorem { id: "ctl|two_plus_two".into(), split: "ctl".into(), header: "import Mathlib".into(), statement: "theorem ctl (a b : ℝ) : a ^ 2 + b ^ 2 ≥ 2 * a * b :=".into(), nl: String::new() };
            let d = dir.join("lean-controls");
            let good = math::lean::check(&th, "nlinarith [sq_nonneg (a - b)]", &d, "good", 180);
            println!("{} positive control: correct proof ({:.1} s) {:?}", if good.ok { "GREEN" } else { "RED — FAILURE" }, good.secs, good.errors);
            let mut red = 0;
            for (name, proof) in [("sorry", "sorry"), ("admit", "admit"), ("wrong tactic", "linarith"), ("unrelated lemma", "exact two_mul_le_add_sq a b c")] {
                let v = math::lean::check(&th, proof, &d, &format!("bad{red}"), 180);
                println!("{} negative control \"{name}\": {}", if v.ok { "GREEN — GATE DID NOT FIRE" } else { "RED (caught)" }, v.errors.first().map(|e| e.lines().next().unwrap_or("").to_string()).unwrap_or_default());
                red += (!v.ok) as usize;
            }
            println!("Lean negative controls red: {red} of 4; positive: {}", good.ok);
            if red != 4 || !good.ok {
                bail!("the Lean gate does not work");
            }
        }
        "lean-run" => {
            let text = std::fs::read_to_string(dir.join("levels/level4.jsonl"))?;
            let done: std::collections::HashSet<String> = solve::load_records(&records).into_iter().filter(|r| r.level == 4 && r.error.is_none()).map(|r| r.id).collect();
            let ths: Vec<math::lean::Theorem> = text.lines().filter_map(|l| serde_json::from_str(l).ok()).filter(|t: &math::lean::Theorem| !done.contains(&t.id)).collect();
            let batch: usize = flag(&args, "--batch").and_then(|s| s.parse().ok()).unwrap_or(5);
            let reserve: f64 = flag(&args, "--reserve").and_then(|s| s.parse().ok()).unwrap_or(2.0);
            let v = Vmm::new(dir.join("cwd"), calls.clone(), cap)?;
            eprintln!("level 4 (Lean): {} theorems; spent before start ${:.2}", ths.len(), v.spent());
            let r = Run { vmm: &v, records: records.clone(), batch, par: 1, reserve, max_retries: 2, plan_secs: 20.0 };
            let recs = solve::run_lean(&r, &ths, &dir.join("lean"), "L4");
            for c in ["A", "B"] {
                let rs: Vec<&Record> = recs.iter().filter(|r| r.config == c).collect();
                let s = solve::summarize(&rs);
                println!("{c}: {} theorems, proved {} ({:.0}%), ${:.3}, output tokens {}", s.n, s.correct, 100.0 * s.correct as f64 / s.n.max(1) as f64, s.cost, s.out_tok);
            }
            println!("spent in total ${:.2}", v.spent());
        }
        _ => bail!("unknown command {cmd}"),
    }
    Ok(())
}
