//! Checking the judge against real execution (after all rounds; not fed into training) and the training log.

use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::sandbox::{self, Exec};
use crate::tasks::Task;
use crate::util::{read_jsonl, run_dir, write_jsonl};
use crate::v0::Solution;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Row {
    pub round: usize,
    pub arch: String,
    pub task: String,
    pub split: String,
    pub skill: String,
    pub source: String,
    pub real_pass: bool,
    pub exec: String,
    /// judge's prediction
    pub judge_pass: Option<bool>,
    pub judge_correct: Option<u8>,
    pub judge_explain: Option<u8>,
    /// stepwise path: executed, passed, agreed with the fast path
    pub step_present: bool,
    pub step_pass: Option<bool>,
    pub paths_agree: Option<bool>,
    pub used: Vec<String>,
}

fn exec_str(e: &Exec) -> (String, Option<String>) {
    match e {
        Exec::Ran(o) => ("ran".into(), Some(o.clone())),
        Exec::Denied(d) => (format!("denied:{}", d.join(",")), None),
        Exec::Timeout => ("timeout".into(), None),
        Exec::Crashed(m) => (format!("crashed:{}", m.chars().take(60).collect::<String>()), None),
    }
}

/// Negative controls of the execution gate: they must be red.
pub fn negative_controls(tasks: &[Task]) -> Result<Vec<(String, usize, usize)>> {
    let mut out = Vec::new();
    let (mut ref_ok, mut mut_red, mut empty_red) = (0, 0, 0);
    for t in tasks {
        let e = sandbox::run(&t.solution)?;
        if let Exec::Ran(o) = &e {
            if sandbox::same_output(o, &t.expected) {
                ref_ok += 1;
            }
            let mut wrong = t.expected.clone();
            wrong.push("X".into());
            if !sandbox::same_output(o, &wrong) {
                mut_red += 1;
            }
        }
        let e0 = sandbox::run("% empty\n")?;
        if let Exec::Ran(o) = &e0 {
            if !sandbox::same_output(o, &t.expected) {
                empty_red += 1;
            }
        }
    }
    out.push(("reference passes (green)".into(), ref_ok, tasks.len()));
    out.push(("reference + extra expected line (red)".into(), mut_red, tasks.len()));
    out.push(("empty program (red)".into(), empty_red, tasks.len()));
    let denied = ["x = readtable('a.csv');", "system('ls')", "fid = fopen('f.txt', 'w');", "exist('f')", "feval('system', 'ls')", "eval('1+1')"]
        .iter()
        .filter(|p| matches!(sandbox::run(p), Ok(Exec::Denied(_))))
        .count();
    out.push(("file and system calls rejected statically".into(), denied, 6));
    let slow = matches!(sandbox::run("while true\n  x = 1;\nend\n")?, Exec::Timeout);
    out.push(("infinite loop stopped within 2 s".into(), slow as usize, 1));
    let big = matches!(sandbox::run("A = zeros(40000);\n")?, Exec::Crashed(_));
    out.push(("12.8 GB of memory — refused under ulimit -v".into(), big as usize, 1));
    Ok(out)
}

pub fn run_control(tasks: &[Task], rounds: &[(usize, Vec<Solution>, Vec<serde_json::Value>)]) -> Result<Vec<Row>> {
    let by_id: HashMap<&str, &Task> = tasks.iter().map(|t| (t.id.as_str(), t)).collect();
    let mut rows = Vec::new();
    let mut memo: HashMap<String, Exec> = HashMap::new();
    let mut run = |src: &str| -> Result<Exec> {
        if let Some(e) = memo.get(src) {
            return Ok(e.clone());
        }
        let e = sandbox::run(src)?;
        memo.insert(src.to_string(), e.clone());
        Ok(e)
    };
    for (r, sols, verdicts) in rounds {
        let vmap: HashMap<&str, &serde_json::Value> = verdicts.iter().filter_map(|v| v["task"].as_str().map(|t| (t, v))).collect();
        for s in sols {
            let t = by_id[s.task.as_str()];
            let e = run(&s.program)?;
            let (es, out) = exec_str(&e);
            let real = out.as_deref().is_some_and(|o| sandbox::same_output(o, &t.expected));
            let (mut sp, mut agree) = (None, None);
            if let Some(st) = &s.stepwise {
                let e2 = run(st)?;
                let (_, o2) = exec_str(&e2);
                sp = Some(o2.as_deref().is_some_and(|o| sandbox::same_output(o, &t.expected)));
                agree = Some(matches!((&out, &o2), (Some(a), Some(b)) if a == b));
            }
            let v = vmap.get(s.task.as_str());
            rows.push(Row {
                round: *r,
                arch: s.arch.clone(),
                task: s.task.clone(),
                split: t.split.clone(),
                skill: t.skill.clone(),
                source: t.source.clone(),
                real_pass: real,
                exec: es,
                judge_pass: v.and_then(|v| v["v"]["pass"].as_bool()),
                judge_correct: v.and_then(|v| v["v"]["correct"].as_u64()).map(|x| x as u8),
                judge_explain: v.and_then(|v| v["v"]["explain"].as_u64()).map(|x| x as u8),
                step_present: s.stepwise.is_some(),
                step_pass: sp,
                paths_agree: agree,
                used: s.used.clone(),
            });
        }
    }
    write_jsonl(&run_dir().join("control.jsonl"), &rows)?;
    Ok(rows)
}

pub fn summary(rows: &[Row]) -> String {
    let mut s = String::new();
    s.push_str("| round | split | actually passed | judge 'pass' | judge accuracy | 'pass' precision | 'pass' recall | TP/FP/FN/TN |\n|---|---|---|---|---|---|---|---|\n");
    let mut rounds: Vec<usize> = rows.iter().map(|r| r.round).collect();
    rounds.sort();
    rounds.dedup();
    for r in &rounds {
        for split in ["train", "held", "all"] {
            let sel: Vec<&Row> = rows.iter().filter(|x| x.round == *r && (split == "all" || x.split == split) && x.judge_pass.is_some()).collect();
            let (mut tp, mut fp, mut fnn, mut tn) = (0, 0, 0, 0);
            for x in &sel {
                match (x.judge_pass.unwrap(), x.real_pass) {
                    (true, true) => tp += 1,
                    (true, false) => fp += 1,
                    (false, true) => fnn += 1,
                    (false, false) => tn += 1,
                }
            }
            let n = sel.len().max(1) as f64;
            let real = sel.iter().filter(|x| x.real_pass).count();
            s.push_str(&format!(
                "| r{r} | {split} | {real}/{} | {}/{} | {:.1}% | {} | {} | {tp}/{fp}/{fnn}/{tn} |\n",
                sel.len(),
                tp + fp,
                sel.len(),
                100.0 * (tp + tn) as f64 / n,
                if tp + fp > 0 { format!("{:.1}%", 100.0 * tp as f64 / (tp + fp) as f64) } else { "—".into() },
                if tp + fnn > 0 { format!("{:.1}%", 100.0 * tp as f64 / (tp + fnn) as f64) } else { "—".into() },
            ));
        }
    }
    // judge's "correctness" score vs real execution
    let mut by: BTreeMap<u8, (usize, usize)> = BTreeMap::new();
    for x in rows {
        if let Some(c) = x.judge_correct {
            let e = by.entry(c).or_default();
            e.1 += 1;
            if x.real_pass {
                e.0 += 1;
            }
        }
    }
    s.push_str("\nJudge's \"correctness\" score vs real execution (all rounds):\n\n| score | programs | actually passed |\n|---|---|---|\n");
    for (c, (p, n)) in by {
        s.push_str(&format!("| {c} | {n} | {p} ({:.0}%) |\n", 100.0 * p as f64 / n.max(1) as f64));
    }
    // two paths
    s.push_str("\nTwo paths (fast and stepwise), real execution:\n\n| round | tasks with both | agreed | disagreed | both correct | agreed but wrong | fast correct, stepwise not | the reverse |\n|---|---|---|---|---|---|---|---|\n");
    for r in &rounds {
        let sel: Vec<&Row> = rows.iter().filter(|x| x.round == *r && x.step_present).collect();
        if sel.is_empty() {
            continue;
        }
        let agree = sel.iter().filter(|x| x.paths_agree == Some(true)).count();
        let both = sel.iter().filter(|x| x.real_pass && x.step_pass == Some(true)).count();
        let agree_wrong = sel.iter().filter(|x| x.paths_agree == Some(true) && !x.real_pass).count();
        let f_only = sel.iter().filter(|x| x.real_pass && x.step_pass == Some(false)).count();
        let s_only = sel.iter().filter(|x| !x.real_pass && x.step_pass == Some(true)).count();
        s.push_str(&format!("| r{r} | {} | {agree} | {} | {both} | {agree_wrong} | {f_only} | {s_only} |\n", sel.len(), sel.len() - agree));
    }
    s
}

pub fn load_rows() -> Result<Vec<Row>> {
    read_jsonl(&run_dir().join("control.jsonl"))
}

/// Training log: skill × round — level, examples, source, judge scores and real execution.
pub fn journal(tasks: &[Task], rounds: &[(usize, Vec<Solution>, Vec<serde_json::Value>)], ctl: &[Row]) -> Vec<serde_json::Value> {
    let skill_of: HashMap<&str, &Task> = tasks.iter().map(|t| (t.id.as_str(), t)).collect();
    let mut out = Vec::new();
    for (r, sols, verdicts) in rounds {
        let vmap: HashMap<&str, &serde_json::Value> = verdicts.iter().filter_map(|v| v["task"].as_str().map(|t| (t, v))).collect();
        let mut skills: Vec<&str> = tasks.iter().map(|t| t.skill.as_str()).collect();
        skills.sort();
        skills.dedup();
        for sk in skills {
            let ss: Vec<&Solution> = sols.iter().filter(|s| skill_of[s.task.as_str()].skill == sk).collect();
            let avg = |split: &str, key: &str| -> Option<f64> {
                let v: Vec<f64> = ss.iter().filter(|s| skill_of[s.task.as_str()].split == split).filter_map(|s| vmap.get(s.task.as_str()).and_then(|v| v["v"][key].as_f64())).collect();
                if v.is_empty() { None } else { Some((v.iter().sum::<f64>() / v.len() as f64 * 100.0).round() / 100.0) }
            };
            let pred = |split: &str| ss.iter().filter(|s| skill_of[s.task.as_str()].split == split).filter(|s| vmap.get(s.task.as_str()).is_some_and(|v| v["v"]["pass"] == true)).count();
            let real = |split: &str| -> Option<usize> {
                let rows: Vec<&Row> = ctl.iter().filter(|x| x.round == *r && x.skill == sk && x.split == split).collect();
                if rows.is_empty() { None } else { Some(rows.iter().filter(|x| x.real_pass).count()) }
            };
            // examples: from the explanations — the memory example (v0) and math quotes (v1)
            let mut ex: HashMap<String, usize> = HashMap::new();
            let mut src: HashMap<String, usize> = HashMap::new();
            for s in &ss {
                for u in &s.used {
                    *src.entry(u.clone()).or_default() += 1;
                }
                for part in s.explanation.split(['[', '(']) {
                    let id: String = part.chars().take_while(|c| !c.is_whitespace() && *c != ';' && *c != ']').collect();
                    if id.contains('-') && id.len() > 5 && (id.starts_with("vmm-") || id.starts_with("gsm8k") || id.starts_with("tal-") || id.starts_with("theoremqa") || id.starts_with("mmlu") || id.starts_with("math") || id.contains("-0") || id.starts_with("syn") ) {
                        *ex.entry(id).or_default() += 1;
                    }
                }
                if let Some(p) = s.explanation.find("example ") {
                    let id: String = s.explanation[p + "example ".len()..].chars().take_while(|c| !c.is_whitespace()).collect();
                    *ex.entry(id).or_default() += 1;
                }
            }
            let mut exv: Vec<(String, usize)> = ex.into_iter().collect();
            exv.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            let mut srcv: Vec<(String, usize)> = src.into_iter().collect();
            srcv.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            let n_train = ss.iter().filter(|s| skill_of[s.task.as_str()].split == "train").count();
            let n_held = ss.len() - n_train;
            out.push(serde_json::json!({
                "skill": sk,
                "round": r,
                "arch": ss.first().map(|s| s.arch.clone()).unwrap_or_default(),
                "level": avg("train", "correct"),
                "n_train": n_train,
                "n_held": n_held,
                "judge_train": avg("train", "correct"),
                "judge_held": avg("held", "correct"),
                "explain_held": avg("held", "explain"),
                "pass_pred_train": pred("train"),
                "pass_pred_held": pred("held"),
                "real_pass_train": real("train"),
                "real_pass_held": real("held"),
                "two_paths": ss.iter().filter(|s| s.stepwise.is_some()).count(),
                "examples": exv.iter().take(5).map(|(k, v)| format!("{k}×{v}")).collect::<Vec<_>>().join(", "),
                "source": srcv.iter().map(|(k, v)| format!("{k}×{v}")).collect::<Vec<_>>().join(", "),
            }));
        }
    }
    out
}
