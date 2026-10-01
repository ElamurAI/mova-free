//! v2: solver — three measurement configurations.
//! - A: the LLM alone — answer only;
//! - B: the LLM writes a plan, the MMM executes and checks it; if it fails — back to the LLM with the reason (≤ 2 retries),
//!   then an honest "not solved";
//! - C: the MMM itself chooses the method (perceptron) and the plan (a template from the memory of verified solutions) — without the LLM;
//!   where it cannot — the answer of configuration B.

use std::collections::HashMap;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::curric::{self, Problem, grade};
use crate::memory::{self, Memo, Perceptron};
use crate::plan::{self, Expect, Plan, Trace};
use crate::vmm::{Call, Vmm};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Record {
    pub id: String,
    pub level: u8,
    pub split: String,
    pub config: String,
    pub answer: Option<String>,
    pub option: Option<String>,
    pub verified: bool,
    pub correct: bool,
    pub rounds: u32,
    pub method: String,
    pub plan: Value,
    pub trace: Option<Trace>,
    pub fails: Vec<String>,
    pub cost_usd: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub secs: f64,
    /// C: the answer was given by the MMM alone
    pub by_mmm: bool,
    pub mmm_method: Option<String>,
    pub mmm_margin: Option<f64>,
    pub mmm_why: Vec<(String, f64)>,
    pub mmm_template_from: Option<String>,
    pub error: Option<String>,
    /// gate rejection reasons by round (what the MMM returned to the LLM)
    #[serde(default)]
    pub history: Vec<String>,
}

pub struct Run<'a> {
    pub vmm: &'a Vmm,
    pub records: PathBuf,
    pub batch: usize,
    pub par: usize,
    pub reserve: f64,
    pub max_retries: u32,
    pub plan_secs: f64,
}

pub fn expect_for(p: &Problem) -> Expect {
    Expect { integer: p.gold_kind == "int" || (p.gold_kind == "number" && p.source.starts_with("TheoremQA")), options: p.options.clone() }
}

fn share(c: &Call, n: usize) -> (f64, u64, u64, f64) {
    let n = n.max(1) as f64;
    (c.cost_usd / n, (c.input_tokens + c.cache_read_tokens + c.cache_write_tokens) / n as u64, c.output_tokens / n as u64, c.secs / n)
}

fn append(path: &Path, lock: &Mutex<()>, recs: &[Record]) -> Result<()> {
    let _g = lock.lock().unwrap();
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
    for r in recs {
        writeln!(f, "{}", serde_json::to_string(r)?)?;
    }
    Ok(())
}

/// Batches in parallel (par threads), each batch — function `f`.
fn batched(ps: &[Problem], batch: usize, par: usize, f: &(dyn Fn(usize, &[Problem]) -> Vec<Record> + Sync)) -> Vec<Record> {
    let chunks: Vec<&[Problem]> = ps.chunks(batch.max(1)).collect();
    let next = AtomicUsize::new(0);
    let out = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..par.max(1) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= chunks.len() {
                    break;
                }
                let recs = f(i, chunks[i]);
                out.lock().unwrap().extend(recs);
            });
        }
    });
    let mut v = out.into_inner().unwrap();
    v.sort_by(|a, b| a.id.cmp(&b.id));
    v
}

fn parse_answers(text: &str) -> HashMap<String, String> {
    let mut m = HashMap::new();
    for obj in plan::json_objects(text) {
        if let Ok(v) = serde_json::from_str::<Value>(&obj) {
            let id = v["id"].as_str().unwrap_or_default().to_string();
            let a = match &v["answer"] {
                Value::String(s) => s.clone(),
                Value::Null => continue,
                o => o.to_string(),
            };
            m.insert(id, a);
        }
    }
    m
}

/// A: the LLM alone.
pub fn run_a(r: &Run, ps: &[Problem], tag: &str) -> Vec<Record> {
    let lock = Mutex::new(());
    batched(ps, r.batch, r.par, &|i, chunk| {
        let prompt = curric::prompt_answers(chunk);
        let recs: Vec<Record> = match r.vmm.ask(&format!("{tag} A"), i, chunk.len(), &prompt, r.reserve) {
            Ok((text, call)) => {
                let ans = parse_answers(&text);
                let (c, it, ot, secs) = share(&call, chunk.len());
                chunk.iter().map(|p| {
                    let a = ans.get(&p.id).cloned();
                    let correct = grade(p, a.as_deref(), a.as_deref());
                    Record { id: p.id.clone(), level: p.level, split: p.split.clone(), config: "A".into(), answer: a.clone(), option: a, correct, cost_usd: c, input_tokens: it, output_tokens: ot, secs, ..Default::default() }
                }).collect()
            }
            Err(e) => chunk.iter().map(|p| Record { id: p.id.clone(), level: p.level, split: p.split.clone(), config: "A".into(), error: Some(e.to_string()), ..Default::default() }).collect(),
        };
        let _ = append(&r.records, &lock, &recs);
        recs
    })
}

fn plans_by_id(text: &str) -> HashMap<String, (Plan, Value)> {
    let mut m = HashMap::new();
    for obj in plan::json_objects(text) {
        if let (Ok(p), Ok(v)) = (serde_json::from_str::<Plan>(&obj), serde_json::from_str::<Value>(&obj)) {
            m.insert(p.id.clone(), (p, v));
        }
    }
    m
}

/// B: LLM plan → execution and checking by the MMM → retries with the reason.
pub fn run_b(r: &Run, ps: &[Problem], tag: &str) -> Vec<Record> {
    let lock = Mutex::new(());
    batched(ps, r.batch, r.par, &|i, chunk| {
        let mut recs: HashMap<String, Record> = chunk.iter().map(|p| (p.id.clone(), Record { id: p.id.clone(), level: p.level, split: p.split.clone(), config: "B".into(), ..Default::default() })).collect();
        let mut pending: Vec<(Problem, String, Vec<String>)> = Vec::new();
        let mut round = 0u32;
        loop {
            let (prompt, n, members): (String, usize, Vec<Problem>) = if round == 0 {
                (curric::prompt_plans(chunk, &[]), chunk.len(), chunk.to_vec())
            } else {
                (curric::prompt_retry(&pending), pending.len(), pending.iter().map(|x| x.0.clone()).collect())
            };
            let res = r.vmm.ask(&format!("{tag} B r{round}"), i, n, &prompt, r.reserve);
            let (text, call) = match res {
                Ok(x) => x,
                Err(e) => {
                    for p in &members {
                        let rec = recs.get_mut(&p.id).unwrap();
                        rec.error = Some(e.to_string());
                    }
                    break;
                }
            };
            let (c, it, ot, secs) = share(&call, n);
            let plans = plans_by_id(&text);
            let mut next = Vec::new();
            for p in &members {
                let rec = recs.get_mut(&p.id).unwrap();
                rec.cost_usd += c;
                rec.input_tokens += it;
                rec.output_tokens += ot;
                rec.secs += secs;
                rec.rounds = round + 1;
                match plans.get(&p.id) {
                    Some((pl, raw)) => {
                        let t = plan::run(pl, &expect_for(p), r.plan_secs);
                        rec.method = pl.method.clone();
                        rec.plan = raw.clone();
                        rec.answer = t.answer.clone();
                        rec.option = pl.option.clone();
                        rec.verified = t.verified();
                        rec.fails = t.fails.clone();
                        rec.secs += t.secs;
                        rec.trace = Some(t.clone());
                        if !rec.verified {
                            rec.history.push(format!("r{round}: {} | plan: {}", t.fails.join(" | "), serde_json::to_string(raw).unwrap_or_default()));
                            next.push((p.clone(), serde_json::to_string(raw).unwrap_or_default(), t.fails.clone()));
                        }
                    }
                    None => {
                        rec.fails = vec!["plan not received (no JSON line with this id)".into()];
                        rec.history.push(format!("r{round}: plan not received"));
                        next.push((p.clone(), "(none)".into(), rec.fails.clone()));
                    }
                }
            }
            if next.is_empty() || round >= r.max_retries {
                break;
            }
            pending = next;
            round += 1;
        }
        let mut out: Vec<Record> = recs.into_values().collect();
        for rec in out.iter_mut() {
            let p = chunk.iter().find(|p| p.id == rec.id).unwrap();
            // "not solved" is not an answer: correctness only for verified ones
            rec.correct = rec.verified && grade(p, rec.answer.as_deref(), rec.option.as_deref());
        }
        out.sort_by(|a, b| a.id.cmp(&b.id));
        let _ = append(&r.records, &lock, &out);
        out
    })
}

/// Memory from verified B solutions (records → Memo).
pub fn memos_from(ps: &[Problem], recs: &[Record]) -> Vec<Memo> {
    let by: HashMap<&str, &Problem> = ps.iter().map(|p| (p.id.as_str(), p)).collect();
    recs.iter().filter(|r| r.config == "B" && r.verified).filter_map(|r| {
        let p = by.get(r.id.as_str())?;
        let (template, nums) = match memory::make_template(&r.plan, &p.question) {
            Some((t, n)) => (Some(t), n),
            None => (None, memory::numbers(&p.question).len()),
        };
        let t = r.trace.as_ref();
        Some(Memo { id: r.id.clone(), level: r.level, split: r.split.clone(), source: p.source.clone(), question: p.question.clone(), feats: memory::features(p), method: r.method.clone(), plan: r.plan.clone(), answer: r.answer.clone().unwrap_or_default(), correct: r.correct, checks_ok: t.map(|t| t.checks_ok).unwrap_or(0), checks_total: t.map(|t| t.checks_total).unwrap_or(0), template, nums })
    }).collect()
}

/// C: the MMM alone — the perceptron chooses the method, a template from memory gives the plan; otherwise — the B answer.
pub fn run_c(ps: &[Problem], memos: &[Memo], b: &HashMap<String, Record>, min_margin: f64, min_sim: f64, plan_secs: f64) -> Vec<Record> {
    let refs: Vec<&Memo> = memos.iter().collect();
    let perc = Perceptron::train(&refs, 10);
    ps.iter().map(|p| {
        let feats = memory::features(p);
        let mut rec = Record { id: p.id.clone(), level: p.level, split: p.split.clone(), config: "C".into(), ..Default::default() };
        if let Some(g) = perc.guess(&feats) {
            rec.mmm_method = Some(g.method.clone());
            rec.mmm_margin = Some(g.margin);
            rec.mmm_why = g.why.clone();
            let nums = memory::numbers(&p.question).len();
            if g.margin >= min_margin {
                // the nearest verified solution of the same method with the same number of numbers
                // the problem structure must almost match (skeleton with numbers as "#"), not just the words
                let sk = memory::skeleton(&p.question);
                let mut cands: Vec<(&Memo, f64)> = memos.iter().filter(|m| m.method == g.method && m.template.is_some() && m.nums == nums && m.id != p.id).map(|m| (m, memory::skeleton_sim(&memory::skeleton(&m.question), &sk))).filter(|(_, s)| *s >= min_sim).collect();
                cands.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                for (m, _) in cands.into_iter().take(3) {
                    let Some(pv) = memory::instantiate(m.template.as_ref().unwrap(), &p.question) else { continue };
                    let Ok(mut pl) = serde_json::from_value::<Plan>(pv.clone()) else { continue };
                    pl.id = p.id.clone();
                    let mut ex = expect_for(p);
                    if !p.options.is_empty() {
                        // the MMM chooses the option itself: the only option with a computed value
                        let t0 = plan::run(&pl, &Expect { options: Vec::new(), ..ex.clone() }, plan_secs);
                        let val = t0.answer.as_deref().and_then(crate::big::parse_q);
                        let hits: Vec<String> = p.options.iter().filter(|(_, c)| val.is_some() && plan::option_value(c) == val).map(|(k, _)| k.clone()).collect();
                        if hits.len() != 1 {
                            continue;
                        }
                        pl.option = Some(hits[0].clone());
                        ex = expect_for(p);
                    }
                    let t = plan::run(&pl, &ex, plan_secs);
                    if t.verified() {
                        rec.by_mmm = true;
                        rec.method = g.method.clone();
                        rec.plan = serde_json::to_value(&pl).unwrap_or(Value::Null);
                        rec.answer = t.answer.clone();
                        rec.option = pl.option.clone();
                        rec.verified = true;
                        rec.correct = grade(p, rec.answer.as_deref(), rec.option.as_deref());
                        rec.secs = t.secs;
                        rec.trace = Some(t);
                        rec.mmm_template_from = Some(m.id.clone());
                        return rec;
                    }
                }
            }
        }
        // fallback: the B answer (at B's cost)
        if let Some(br) = b.get(&p.id) {
            rec.answer = br.answer.clone();
            rec.option = br.option.clone();
            rec.verified = br.verified;
            rec.correct = br.correct;
            rec.cost_usd = br.cost_usd;
            rec.input_tokens = br.input_tokens;
            rec.output_tokens = br.output_tokens;
            rec.secs = br.secs;
            rec.method = br.method.clone();
        }
        rec
    }).collect()
}

pub fn load_records(path: &Path) -> Vec<Record> {
    std::fs::read_to_string(path).unwrap_or_default().lines().filter_map(|l| serde_json::from_str(l).ok()).collect()
}

#[derive(Default, Debug, Serialize)]
pub struct Summary {
    pub n: usize,
    pub correct: usize,
    pub verified: usize,
    pub verified_correct: usize,
    pub errors: usize,
    pub by_mmm: usize,
    pub by_mmm_correct: usize,
    pub cost: f64,
    pub in_tok: u64,
    pub out_tok: u64,
    pub secs: f64,
}

pub fn summarize(recs: &[&Record]) -> Summary {
    let mut s = Summary::default();
    for r in recs {
        s.n += 1;
        s.correct += r.correct as usize;
        s.verified += r.verified as usize;
        s.verified_correct += (r.verified && r.correct) as usize;
        s.errors += r.error.is_some() as usize;
        s.by_mmm += r.by_mmm as usize;
        s.by_mmm_correct += (r.by_mmm && r.correct) as usize;
        s.cost += r.cost_usd;
        s.in_tok += r.input_tokens;
        s.out_tok += r.output_tokens;
        s.secs += r.secs;
    }
    s
}

/// Level 4: proofs in Lean. Round 0 — "LLM alone" (first attempt without feedback, configuration A);
/// with retries on Lean errors (≤ `max_retries`) — "LLM + MMM" (configuration B).
pub fn run_lean(r: &Run, ts: &[crate::lean::Theorem], dir: &Path, tag: &str) -> Vec<Record> {
    use crate::lean;
    let lock = Mutex::new(());
    let chunks: Vec<&[lean::Theorem]> = ts.chunks(r.batch.max(1)).collect();
    let mut all = Vec::new();
    for (i, chunk) in chunks.iter().enumerate() {
        let mut state: Vec<(lean::Theorem, Option<(String, Vec<String>)>, Record, Option<Record>)> = chunk.iter().map(|t| (t.clone(), None, Record { id: t.id.clone(), level: 4, split: t.split.clone(), config: "B".into(), method: "lean".into(), ..Default::default() }, None)).collect();
        for round in 0..=r.max_retries {
            let todo: Vec<usize> = (0..state.len()).filter(|k| !state[*k].2.verified && state[*k].2.error.is_none()).collect();
            if todo.is_empty() {
                break;
            }
            let items: Vec<(lean::Theorem, Option<(String, Vec<String>)>)> = todo.iter().map(|k| (state[*k].0.clone(), state[*k].1.clone())).collect();
            let prompt = lean::prompt(&items);
            let (text, call) = match r.vmm.ask(&format!("{tag} lean r{round}"), i, items.len(), &prompt, r.reserve) {
                Ok(x) => x,
                Err(e) => {
                    for k in &todo {
                        state[*k].2.error = Some(e.to_string());
                    }
                    break;
                }
            };
            let proofs = lean::parse_proofs(&text);
            let (c, it, ot, secs) = share(&call, items.len());
            for k in todo {
                let (th, prev, rec, first) = &mut state[k];
                rec.cost_usd += c;
                rec.input_tokens += it;
                rec.output_tokens += ot;
                rec.secs += secs;
                rec.rounds = round + 1;
                let proof = proofs.iter().find(|(id, _)| *id == th.id).map(|(_, p)| p.clone());
                let v = match &proof {
                    Some(p) => lean::check(th, p, dir, &format!("r{round}"), 240),
                    None => lean::LeanVerdict { ok: false, errors: vec!["proof not received".into()], ..Default::default() },
                };
                rec.secs += v.secs;
                rec.verified = v.ok;
                rec.correct = v.ok;
                rec.fails = v.errors.clone();
                if !v.ok {
                    rec.history.push(format!("r{round}: {}", v.errors.iter().map(|e| e.lines().next().unwrap_or("").to_string()).collect::<Vec<_>>().join(" | ")));
                }
                rec.answer = proof.clone();
                rec.plan = serde_json::json!({"file": v.file});
                if round == 0 {
                    *first = Some(Record { config: "A".into(), rounds: 1, ..rec.clone() });
                }
                *prev = proof.map(|p| (p, v.errors.clone()));
            }
        }
        let mut recs = Vec::new();
        for (_, _, rec, first) in state {
            if let Some(f) = first {
                recs.push(f);
            }
            recs.push(rec);
        }
        let _ = append(&r.records, &lock, &recs);
        all.extend(recs);
    }
    all
}
