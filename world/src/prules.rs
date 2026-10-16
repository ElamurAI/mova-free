//! The pragmatic reader learns its own rules (`prag::rules`): error-driven, after Brill (1995) as `crate::induce`
//! learns tree repairs. The gold: the frame bench's dev lines (`crate::fbench`) and the teacher's verdicts
//! ($MOVA_DATA/runs/prag-teacher/log.tsv: OK → the frame as it was, WRONG → the corrected frame, MISSING → added).
//! The 60 held-out lines are never read here.
//!
//!   world frames rules learn [cap N] [min N] [prec X] [atoms N] [lex] [sig P] [noteacher]
//!                                                      — learn, write prag/data/rules.tsv (proposed) and the queue
//!   world frames rules control                         — the same with the gold shuffled between lines (must find
//!                                                         ~nothing that helps on dev)
//!   world frames rules cv                              — two folds of dev: learn on teacher + one half, score the other
//!   world frames rules show                            — the rules file in plain English
//!   world frames rules apply-check                     — dev with the file's rules (proposed and accepted) vs none
//!
//! Learning: every site (a frame, a pronoun in a role, a participle on a noun) × every action is tried alone on the
//! current frames; its effect is the first-order change of the micro F1 over all roles of the bench scorer. Every conjunction of 1–3 of the
//! site's atoms is a candidate (action, atoms) and sums the effects of the sites it matches. The best candidate with
//! precision ≥ 0.9 (sites helped / sites changed) and ≥ 10 lines fixed is applied for real to every line; if the real
//! gain is positive it is kept, else it is set aside. Repeat up to the cap. Lemma atoms only with `lex`. The
//! thresholds were chosen by `cv` (the brief's 3 lines / 0.8 overfit). Deterministic: ordered maps.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use prag::fbench::{GFrame, Prf, ROLES, overall, parse_frame, score_line};
use prag::frames::Read;
use prag::rules::{Action, Atom, Rule, actions_for, apply_at, apply_rule, atoms, sites};

use crate::frames::Ctx;

fn run_dir() -> PathBuf {
    PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/prag-rules")
}

/// One training or test line: its reader output without rules, and its gold.
#[derive(Clone)]
pub struct TLine {
    pub key: String,
    pub split: String,
    pub text: String,
    pub base: Read,
    pub gold: Vec<GFrame>,
    /// the teacher judged only the frames the reader made then (and added at most one missing): only the reader's
    /// frames that align with the gold are scored, the others were never judged
    pub judged_only: bool,
}

/// The gold of the teacher's log: (course, lesson, line index) → (text, frames).
fn teacher_gold() -> Result<BTreeMap<(String, usize, usize), (String, Vec<GFrame>)>> {
    let p = PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/prag-teacher/log.tsv");
    let text = std::fs::read_to_string(&p).with_context(|| format!("{}", p.display()))?;
    // per line: the frames as the reader showed them, then the verdicts in order
    let mut lines: BTreeMap<(String, usize, usize), (String, Vec<Option<String>>, Vec<String>)> = BTreeMap::new();
    for l in text.lines().skip(1) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 10 {
            continue;
        }
        let (Ok(lesson), Ok(line)) = (c[2].parse::<usize>(), c[3].parse::<usize>()) else { continue };
        let key = (c[1].to_string(), lesson, line.saturating_sub(1));
        let e = lines.entry(key).or_insert_with(|| {
            let shown: Vec<Option<String>> = c[5].split(" ;; ").filter(|x| !x.trim().is_empty()).map(|x| Some(x.split_once(' ').map(|(_, f)| f.to_string()).unwrap_or_default())).collect();
            (c[4].to_string(), shown, Vec::new())
        });
        match c[7] {
            "WRONG" => {
                if let Some(k) = c[6].trim_start_matches('#').parse::<usize>().ok().filter(|k| *k < e.1.len()) {
                    // an unreadable correction leaves None: the line is skipped (its gold indices would shift)
                    e.1[k] = parse_frame(c[8]).map(|_| c[8].to_string());
                }
            }
            "MISSING" => {
                if parse_frame(c[8]).is_some() {
                    e.2.push(c[8].to_string());
                }
            }
            _ => {}
        }
    }
    Ok(lines.into_iter().filter(|(_, (_, shown, _))| shown.iter().all(|f| f.as_ref().is_some_and(|f| parse_frame(f).is_some()))).map(|(k, (t, shown, missing))| {
        let fs: Vec<GFrame> = shown.into_iter().flatten().chain(missing).filter_map(|f| parse_frame(&f)).collect();
        (k, (t, fs))
    }).collect())
}

/// The reader's frames (no rules) of lines given as (course, lesson, line index), each lesson read from its start.
fn read_lines(want: &BTreeSet<(String, usize, usize)>) -> Result<BTreeMap<(String, usize, usize), (String, Read)>> {
    prag::rules::set_override(Some(Vec::new()));
    let mut by_lesson: BTreeMap<(String, usize), BTreeSet<usize>> = BTreeMap::new();
    for (c, n, li) in want {
        by_lesson.entry((c.clone(), *n)).or_default().insert(*li);
    }
    let mut courses: BTreeMap<String, BTreeMap<usize, crate::knowledge::Lesson>> = BTreeMap::new();
    let mut out = BTreeMap::new();
    for ((c, n), lis) in &by_lesson {
        if !courses.contains_key(c) {
            courses.insert(c.clone(), crate::knowledge::course(c)?.into_iter().map(|l| (l.n, l)).collect());
        }
        let Some(l) = courses[c].get(n) else { continue };
        let last = *lis.iter().max().unwrap();
        let mut ctx = Ctx::default();
        for (li, t) in l.explain.iter().chain(&l.facts).enumerate().take(last + 1) {
            let r = crate::frames::read(t, &mut ctx, false)?;
            if lis.contains(&li) {
                out.insert((c.clone(), *n, li), (t.clone(), r));
            }
        }
    }
    prag::rules::set_override(None);
    Ok(out)
}

/// The training lines (dev of the bench, then the teacher's lines not on the bench) and, apart, nothing of held-out.
pub fn load() -> Result<Vec<TLine>> {
    let items = crate::fbench::load_items()?;
    let gold = crate::fbench::load_gold()?;
    let held: BTreeSet<(String, usize, usize)> = items.iter().filter(|x| x.split == "held").map(|x| (x.course.clone(), x.lesson, x.line)).collect();
    let dev: BTreeMap<(String, usize, usize), usize> = items.iter().filter(|x| x.split == "dev" && gold.contains_key(&x.id)).map(|x| ((x.course.clone(), x.lesson, x.line), x.id)).collect();
    let teacher = teacher_gold()?;
    let mut want: BTreeSet<(String, usize, usize)> = dev.keys().cloned().collect();
    for k in teacher.keys() {
        if !held.contains(k) && !dev.contains_key(k) {
            want.insert(k.clone());
        }
    }
    let reads = read_lines(&want)?;
    let mut out = Vec::new();
    for (k, id) in &dev {
        let (t, r) = &reads[k];
        out.push(TLine { key: format!("dev:{id}"), split: "dev".into(), text: t.clone(), base: r.clone(), gold: gold[id].clone(), judged_only: false });
    }
    for (k, (text, g)) in &teacher {
        if held.contains(k) || dev.contains_key(k) {
            continue;
        }
        let Some((t, r)) = reads.get(k) else { continue };
        if t.trim() != text.trim() {
            eprintln!("teacher line {k:?}: the text changed since the check — skipped");
            continue;
        }
        out.push(TLine { key: format!("teacher:{}:{}:{}", k.0, k.1, k.2 + 1), split: "teacher".into(), text: t.clone(), base: r.clone(), gold: g.clone(), judged_only: true });
    }
    Ok(out)
}

fn sys_frames(r: &Read) -> Vec<GFrame> {
    prag::frames::scored(&r.frames).iter().filter_map(|f| parse_frame(&f.show(0))).collect()
}

fn score(r: &Read, l: &TLine) -> [Prf; 10] {
    let sys = sys_frames(r);
    if !l.judged_only {
        return score_line(&l.gold, &sys);
    }
    // only the frames that align with the judged gold, links remapped
    let keep: BTreeSet<usize> = prag::fbench::align(&l.gold, &sys).values().copied().collect();
    let map: BTreeMap<usize, usize> = keep.iter().enumerate().map(|(k, i)| (*i, k)).collect();
    let sys: Vec<GFrame> = sys.into_iter().enumerate().filter(|(i, _)| keep.contains(i)).map(|(_, mut f)| {
        f.says = f.says.iter().filter_map(|x| map.get(x).copied()).collect();
        f.cause = f.cause.and_then(|x| map.get(&x).copied());
        f.next = f.next.and_then(|x| map.get(&x).copied());
        f
    }).collect();
    score_line(&l.gold, &sys)
}

/// The learner's objective of a line: the first-order change of the micro F1 over all roles,
/// F1 = 2·right / (made + gold) → 1000·right − w·made with w = 500·F1 (the current F1 of the training lines).
fn util(c: &[Prf; 10], w: i64) -> i64 {
    c.iter().map(|p| 1000 * p.tp as i64 - w * p.sys as i64).sum()
}

fn total(lines: &[TLine], reads: &[Read]) -> [Prf; 10] {
    let mut t = [Prf::default(); 10];
    for (l, r) in lines.iter().zip(reads) {
        for (k, p) in score(r, l).iter().enumerate() {
            t[k].add(*p);
        }
    }
    t
}

/// A learned rule with its training record.
#[derive(Clone)]
pub struct Learned {
    pub rule: Rule,
    pub fixed: usize,
    pub broken: usize,
    pub gain: i64,
    /// (line, before, after) of lines it helped and hurt
    pub good: Vec<(String, String, String)>,
    pub bad: Vec<(String, String, String)>,
}

#[derive(Default, Clone, Copy)]
struct Acc {
    pos: u32,
    neg: u32,
    sum: i64,
    lines: u32,
    last: u32,
}

type Key = (Action, [u32; 3]);

/// What changed in a line's frames (before → after), shortly.
fn diff(a: &Read, b: &Read) -> (String, String) {
    let sa: Vec<String> = a.frames.iter().map(|f| f.show(0)).collect();
    let sb: Vec<String> = b.frames.iter().map(|f| f.show(0)).collect();
    let x: Vec<String> = sa.iter().enumerate().filter(|(_, s)| !sb.contains(s)).map(|(i, s)| format!("#{i} {s}")).collect();
    let y: Vec<String> = sb.iter().enumerate().filter(|(_, s)| !sa.contains(s)).map(|(i, s)| format!("#{i} {s}")).collect();
    (if x.is_empty() { "-".into() } else { x.join(" ;; ") }, if y.is_empty() { "-".into() } else { y.join(" ;; ") })
}

/// The learner's settings.
#[derive(Clone, Debug)]
pub struct Opts {
    pub cap: usize,
    /// lines a rule must fix
    pub min: u32,
    /// sites (and lines) helped of those changed
    pub prec: f64,
    /// atoms in a rule, at most
    pub atoms: usize,
    /// no lemma atoms (only closed classes and level-1 classes)
    pub nolex: bool,
    /// the lines fixed vs broken must pass a one-sided sign test at this level (1.0 = off)
    pub sig: f64,
    /// the weight of a deletion's gain when the candidates are ranked (1.0 = as any other; the checker prefers
    /// rules that add or move structure)
    pub delw: f64,
}

/// One-sided sign test: P(at least `k` of `n` by a fair coin).
fn sign_p(k: usize, n: usize) -> f64 {
    let mut p = 0.0;
    for i in k..=n {
        let mut c = 1.0f64;
        for j in 0..i {
            c = c * (n - j) as f64 / (j + 1) as f64;
        }
        p += c * 0.5f64.powi(n as i32);
    }
    p
}

/// Learn rules on `lines` (their gold as given). Deterministic.
pub fn learn(lines: &[TLine], o: &Opts) -> Vec<Learned> {
    let (cap, min_lines, prec) = (o.cap, o.min, o.prec);
    let mut cur: Vec<Read> = lines.iter().map(|l| l.base.clone()).collect();
    let mut out: Vec<Learned> = Vec::new();
    // the rules the checker rejected are never proposed again
    let mut banned: BTreeSet<(Action, Vec<Atom>)> = prag::rules::parse_rules(&std::fs::read_to_string(prag::rules::FILE).unwrap_or_default()).into_iter().filter(|r| r.status == "rejected").map(|r| (r.action, r.atoms)).collect();
    let mut intern: BTreeMap<Atom, u32> = BTreeMap::new();
    let mut names: Vec<Atom> = Vec::new();
    for it in 0..cap {
        let w = (500.0 * overall(&total(lines, &cur)).f1()).round() as i64;
        let cur_u: Vec<i64> = lines.iter().zip(&cur).map(|(l, r)| util(&score(r, l), w)).collect();
        let mut acc: BTreeMap<Key, Acc> = BTreeMap::new();
        for (li, l) in lines.iter().enumerate() {
            let r = &cur[li];
            for s in sites(r) {
                for a in actions_for(s) {
                    let Some(nr) = apply_at(r, s, a) else { continue };
                    let d = util(&score(&nr, l), w) - cur_u[li];
                    if d == 0 {
                        continue;
                    }
                    let mut ids: Vec<u32> = atoms(r, s, a).into_iter().filter(|x| !(o.nolex && x.f == prag::rules::Feat::Lemma)).map(|x| {
                        let n = intern.len() as u32;
                        *intern.entry(x.clone()).or_insert_with(|| {
                            names.push(x);
                            n
                        })
                    }).collect();
                    ids.sort_unstable();
                    ids.dedup();
                    let n = ids.len();
                    let mut add = |k: [u32; 3]| {
                        let e = acc.entry((a, k)).or_default();
                        if d > 0 {
                            e.pos += 1;
                            if e.last != li as u32 + 1 {
                                e.lines += 1;
                                e.last = li as u32 + 1;
                            }
                        } else {
                            e.neg += 1;
                        }
                        e.sum += d;
                    };
                    const NO: u32 = u32::MAX;
                    for i in 0..n {
                        add([ids[i], NO, NO]);
                        if o.atoms < 2 {
                            continue;
                        }
                        for j in i + 1..n {
                            add([ids[i], ids[j], NO]);
                            if o.atoms < 3 {
                                continue;
                            }
                            for k in j + 1..n {
                                add([ids[i], ids[j], ids[k]]);
                            }
                        }
                    }
                }
            }
        }
        let mut cands: Vec<(i64, usize, Key, Acc)> = acc.into_iter().filter(|(_, e)| e.sum > 0 && e.lines >= min_lines && e.pos as f64 / (e.pos + e.neg) as f64 >= prec).map(|(k, e)| (if k.0.deletion() { (e.sum as f64 * o.delw) as i64 } else { e.sum }, k.1.iter().filter(|x| **x != u32::MAX).count(), k, e)).collect();
        cands.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
        let mut took = false;
        for (_, _, (a, ks), _) in cands.into_iter().take(60) {
            let atoms_v: Vec<Atom> = ks.iter().filter(|x| **x != u32::MAX).map(|x| names[*x as usize].clone()).collect();
            if banned.contains(&(a, atoms_v.clone())) {
                continue;
            }
            let rule = Rule { id: format!("r{:02}", out.len() + 1), status: "proposed".into(), atoms: atoms_v.clone(), action: a, reason: String::new() };
            let mut next: Vec<Read> = Vec::with_capacity(cur.len());
            let (mut fixed, mut broken, mut gain) = (0usize, 0usize, 0i64);
            let mut good = Vec::new();
            let mut bad = Vec::new();
            for (li, l) in lines.iter().enumerate() {
                let mut r = cur[li].clone();
                if apply_rule(&mut r, &rule) > 0 {
                    let d = util(&score(&r, l), w) - cur_u[li];
                    gain += d;
                    if d > 0 {
                        fixed += 1;
                        if good.len() < 3 {
                            let (x, y) = diff(&cur[li], &r);
                            good.push((format!("[{}] {}", l.key, l.text), x, y));
                        }
                    } else if d < 0 {
                        broken += 1;
                        if bad.len() < 2 {
                            let (x, y) = diff(&cur[li], &r);
                            bad.push((format!("[{}] {}", l.key, l.text), x, y));
                        }
                    }
                }
                next.push(r);
            }
            if gain > 0 && fixed >= min_lines as usize && fixed as f64 / (fixed + broken) as f64 >= prec && sign_p(fixed, fixed + broken) <= o.sig {
                eprintln!("iter {it}: {} {} — +{gain}, lines fixed {fixed} broken {broken}", rule.action.code(), rule.atoms_code());
                cur = next;
                out.push(Learned { rule, fixed, broken, gain, good, bad });
                took = true;
                break;
            }
            banned.insert((a, atoms_v));
        }
        if !took {
            eprintln!("iter {it}: no candidate passes — stop");
            break;
        }
    }
    out
}

/// The reads of lines with rules applied in order.
fn with_rules(lines: &[TLine], rules: &[Rule]) -> Vec<Read> {
    lines.iter().map(|l| {
        let mut r = l.base.clone();
        for x in rules {
            apply_rule(&mut r, x);
        }
        r
    }).collect()
}

fn table(name: &str, before: &[Prf; 10], after: &[Prf; 10]) -> String {
    let (o0, o1) = (overall(before), overall(after));
    let mut s = format!("{name}: overall F1 {:.3} → {:.3} ({:+.3})\n", o0.f1(), o1.f1(), o1.f1() - o0.f1());
    for (k, r) in ROLES.iter().enumerate() {
        s += &format!("  {r:<10} {:.3} → {:.3} ({:+.3})  right/made {}/{} → {}/{}\n", before[k].f1(), after[k].f1(), after[k].f1() - before[k].f1(), before[k].tp, before[k].sys, after[k].tp, after[k].sys);
    }
    s
}

fn arg<T: std::str::FromStr>(args: &[String], name: &str, d: T) -> T {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).and_then(|x| x.parse().ok()).unwrap_or(d)
}

/// The deterministic shuffle of the gold between lines (the negative control): each line gets the gold of the line
/// `n/2 + 7` places on (mod n), within the same split.
fn shuffled(lines: &[TLine]) -> Vec<TLine> {
    let mut out = lines.to_vec();
    for split in ["dev", "teacher"] {
        let idx: Vec<usize> = (0..lines.len()).filter(|&i| lines[i].split == split).collect();
        let n = idx.len();
        for (k, &i) in idx.iter().enumerate() {
            out[i].gold = lines[idx[(k + n / 2 + 7) % n]].gold.clone();
        }
    }
    out
}

fn tsv(s: &str) -> String {
    s.replace(['\t', '\n'], " ")
}

/// `world frames rules …`.
pub fn main(args: &[String]) -> Result<String> {
    let cmd = args.first().map(String::as_str).unwrap_or("show");
    // defaults chosen by the two-fold CV on dev (worklog 2026-10-10/prag-rules.md): the brief's 3 lines / .8 with
    // lemma atoms overfit (dev in-sample +.073, unseen −.014); 10 lines / .9 / no lemmas: unseen +.007
    let o = Opts { cap: arg(args, "cap", 60), min: arg(args, "min", 10), prec: arg(args, "prec", 0.9), atoms: arg(args, "atoms", 3), nolex: !args.iter().any(|a| a == "lex"), sig: arg(args, "sig", 1.0), delw: arg(args, "delw", 0.5) };
    let noteacher = args.iter().any(|a| a == "noteacher");
    let load = || -> Result<Vec<TLine>> { Ok(load()?.into_iter().filter(|l| !noteacher || l.split == "dev").collect()) };
    let (cap, min, prec) = (o.cap, o.min, o.prec);
    match cmd {
        "show" => {
            let rules = prag::rules::parse_rules(&std::fs::read_to_string(prag::rules::FILE).unwrap_or_default());
            Ok(rules.iter().map(|r| format!("{} [{}] {}\n    {} {}{}\n", r.id, r.status, r.say(), r.action.code(), r.atoms_code(), if r.reason.is_empty() || r.reason == "-" { String::new() } else { format!("\n    reason: {}", r.reason) })).collect())
        }
        "apply-check" => {
            let lines: Vec<TLine> = load()?.into_iter().filter(|l| l.split == "dev").collect();
            let all = prag::rules::parse_rules(&std::fs::read_to_string(prag::rules::FILE).unwrap_or_default());
            let acc: Vec<Rule> = all.iter().filter(|r| r.status == "accepted").cloned().collect();
            let prop: Vec<Rule> = all.iter().filter(|r| r.status == "accepted" || r.status == "proposed").cloned().collect();
            let base = total(&lines, &with_rules(&lines, &[]));
            let mut s = format!("dev: {} lines; rules: {} accepted, {} proposed\n", lines.len(), acc.len(), prop.len() - acc.len());
            s += &table("accepted only", &base, &total(&lines, &with_rules(&lines, &acc)));
            s += &table("accepted + proposed", &base, &total(&lines, &with_rules(&lines, &prop)));
            Ok(s)
        }
        "learn" | "control" => {
            let mut lines = load()?;
            if cmd == "control" {
                lines = shuffled(&lines);
            }
            let real = load_dev_real(&lines, cmd == "control")?;
            let n_dev = lines.iter().filter(|l| l.split == "dev").count();
            eprintln!("training lines: {} (dev {n_dev}, teacher {})", lines.len(), lines.len() - n_dev);
            let learned = learn(&lines, &o);
            let rules: Vec<Rule> = learned.iter().map(|l| l.rule.clone()).collect();
            // dev (the real gold) before and after, and each rule's marginal gain in order
            let base = total(&real, &with_rules(&real, &[]));
            let after = total(&real, &with_rules(&real, &rules));
            let mut s = format!("{} on {} lines (dev {n_dev}): {} rules ({o:?})\n", if cmd == "control" { "NEGATIVE CONTROL (gold shuffled between lines)" } else { "learned" }, lines.len(), rules.len());
            s += &table("dev (real gold, in-sample)", &base, &after);
            if cmd == "control" {
                std::fs::create_dir_all(run_dir())?;
                let mut q = String::new();
                for l in &learned {
                    q += &format!("{}\t{}\t{}\t{}\t{}\n", l.rule.id, l.rule.action.code(), l.rule.atoms_code(), l.fixed, l.broken);
                }
                std::fs::write(run_dir().join("control-rules.tsv"), q)?;
                return Ok(s);
            }
            // the queue and the rules file
            let mut rows: Vec<(f64, String)> = Vec::new();
            let mut prev = base;
            let old = prag::rules::parse_rules(&std::fs::read_to_string(prag::rules::FILE).unwrap_or_default());
            let mut file = format!("{}\n", prag::rules::HEADER);
            for (k, l) in learned.iter().enumerate() {
                // a rule checked before keeps its id, status and reason; a new one is numbered after the old ones
                let was = old.iter().find(|o| o.action == l.rule.action && o.atoms == l.rule.atoms);
                let max_old = old.iter().filter_map(|o| o.id.trim_start_matches('r').parse::<usize>().ok()).max().unwrap_or(0);
                let id = was.map(|o| o.id.clone()).unwrap_or_else(|| format!("r{:02}", max_old + k + 1));
                let now = total(&real, &with_rules(&real, &rules[..=k]));
                let d = overall(&now).f1() - overall(&prev).f1();
                let per: Vec<String> = ROLES.iter().enumerate().filter(|(i, _)| (now[*i].f1() - prev[*i].f1()).abs() > 1e-9).map(|(i, r)| format!("{r} {:+.3}", now[i].f1() - prev[i].f1())).collect();
                let ex = |v: &[(String, String, String)]| v.iter().map(|(a, b, c)| format!("{} :: {} → {}", tsv(a), tsv(b), tsv(c))).collect::<Vec<_>>().join(" ‖ ");
                rows.push((d, format!("{}\t{}\t{}\t{}\t{}\t{}\t{:+.4}\t{}\t{}\t{}\n", id, tsv(&l.rule.say()), l.rule.action.code(), l.rule.atoms_code(), l.fixed, l.broken, d, if per.is_empty() { "-".into() } else { per.join(", ") }, ex(&l.good), if l.bad.is_empty() { "-".into() } else { ex(&l.bad) })));
                let (st, why) = was.map(|o| (o.status.clone(), o.reason.clone())).unwrap_or(("proposed".into(), "-".into()));
                file += &format!("{}\t{st}\t{}\t{}\t{}\t{}\t{}\t{}\n", id, l.rule.action.code(), l.rule.atoms_code(), tsv(&l.rule.say()), l.fixed, l.broken, if why.is_empty() { "-".into() } else { tsv(&why) });
                prev = now;
            }
            // the old rules not learned again stay in the file with their status and reason (the checker's record)
            for o in old.iter().filter(|o| !learned.iter().any(|l| l.rule.action == o.action && l.rule.atoms == o.atoms)) {
                file += &format!("{}\t{}\t{}\t{}\t{}\t-\t-\t{}\n", o.id, o.status, o.action.code(), o.atoms_code(), tsv(&o.say()), if o.reason.is_empty() { "-".into() } else { tsv(&o.reason) });
            }
            rows.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap().then(a.1.cmp(&b.1)));
            std::fs::create_dir_all(run_dir())?;
            let mut q = String::from("id\trule\taction\tatoms\ttrain_fixed\ttrain_broken\tdev_f1_delta\tdev_per_role\tfixes (line :: before → after)\tbreaks\n");
            for (_, r) in &rows {
                q += r;
            }
            std::fs::write(run_dir().join("queue.tsv"), q)?;
            std::fs::write(prag::rules::FILE, file)?;
            s += &format!("queue: {}\nrules file: {}\n", run_dir().join("queue.tsv").display(), prag::rules::FILE);
            Ok(s)
        }
        "cv" => {
            let lines = load()?;
            let mut s = String::new();
            let mut tb = [Prf::default(); 10];
            let mut ta = [Prf::default(); 10];
            for fold in 0..2 {
                let test: Vec<TLine> = lines.iter().enumerate().filter(|(i, l)| l.split == "dev" && i % 2 == fold).map(|(_, l)| l.clone()).collect();
                let train: Vec<TLine> = lines.iter().enumerate().filter(|(i, l)| !(l.split == "dev" && i % 2 == fold)).map(|(_, l)| l.clone()).collect();
                let learned = learn(&train, &o);
                let rules: Vec<Rule> = learned.iter().map(|l| l.rule.clone()).collect();
                let b = total(&test, &with_rules(&test, &[]));
                let a = total(&test, &with_rules(&test, &rules));
                s += &format!("fold {fold}: {} rules learned on {} lines; tested on {} unseen dev lines ({o:?})\n", rules.len(), train.len(), test.len());
                s += &table(&format!("fold {fold} (unseen half of dev)"), &b, &a);
                for k in 0..10 {
                    tb[k].add(b[k]);
                    ta[k].add(a[k]);
                }
            }
            s += &table("both folds (every dev line scored by rules that never saw it)", &tb, &ta);
            Ok(s)
        }
        _ => bail!("world frames rules [learn|control|cv|show|apply-check] [cap N] [min N] [prec X]"),
    }
}

/// The dev lines with their real gold (for the control, the learner saw shuffled gold).
fn load_dev_real(lines: &[TLine], control: bool) -> Result<Vec<TLine>> {
    if !control {
        return Ok(lines.iter().filter(|l| l.split == "dev").cloned().collect());
    }
    Ok(load()?.into_iter().filter(|l| l.split == "dev").collect())
}
