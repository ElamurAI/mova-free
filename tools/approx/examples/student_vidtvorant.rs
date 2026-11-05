//! Runner of the student's vidtvorant (`world/src/knowledge.rs.groupant/student-vidtvorant.groupant`).
//!
//! `student_vidtvorant SCHOOL_DIR PROBE_ROOT OUT_DIR COURSE...`
//!
//! PROBE_ROOT holds repetitions `1/`, `2/`, … of `world`'s `student_probe` traces (`<course>.trace.tsv`). The
//! student's choices must be identical in every repetition; its answer time is the median over them. The lessons'
//! teaching lines come from `SCHOOL_DIR/<course>/*.lesson`. Folds are by lesson parity (fit on even lessons, test on
//! odd, and the reverse, so every question is predicted once out of fold) and by course (fit on half the courses,
//! test on the other half). Writes `OUT_DIR/semantic.json` (deterministic: agreement, disagreement classes, examples,
//! fitted parameters) and `OUT_DIR/timing.json` (the vidtvorant's own time against the student's). Nothing here
//! reads a held-out set: the school's test and exam questions are the course pupil's diagnostic score.
#[path = "../../../world/src/knowledge.rs.groupant/student-vidtvorant.groupant/rust_approximant.rs"]
mod vid;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;
use vid::{CourseData, Example, LessonData, Params, Prediction, TimeModel};

type Res<T> = Result<T, String>;
/// Which lessons a fold admits.
type Admit = Box<dyn Fn(&CourseData, &LessonData) -> bool>;

fn read(p: &Path) -> Res<String> {
    std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))
}

/// The teaching lines of each lesson file, in the student's reading order (explanations, then facts).
fn lessons(dir: &Path) -> Res<BTreeMap<String, (usize, Vec<String>)>> {
    let mut out = BTreeMap::new();
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?
        .flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "lesson")).collect();
    files.sort();
    for f in files {
        let text = read(&f)?;
        let (mut n, mut ex, mut fa) = (0usize, Vec::new(), Vec::new());
        for l in text.lines() {
            if let Some(v) = l.strip_prefix("lesson:") { n = v.trim().parse().map_err(|_| format!("{}: lesson number", f.display()))?; }
            if let Some(v) = l.strip_prefix("explain:") { ex.push(v.trim().to_string()); }
            if let Some(v) = l.strip_prefix("fact:") { fa.push(v.trim().to_string()); }
        }
        ex.extend(fa);
        out.insert(f.file_name().unwrap().to_string_lossy().into_owned(), (n, ex));
    }
    Ok(out)
}

struct Row { file: String, lesson: usize, ex: Example, mova_scores: Vec<Option<i64>> }

fn trace(p: &Path) -> Res<Vec<Row>> {
    let text = read(p)?;
    let mut rows = Vec::new();
    for l in text.lines().skip(1) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 15 { return Err(format!("{}: short row", p.display())); }
        let num = |i: usize| c[i].parse::<usize>().map_err(|_| format!("{}: column {i}", p.display()));
        rows.push(Row {
            file: c[1].into(), lesson: num(2)?,
            ex: Example { kind: c[3].into(), n: num(4)?, question: c[5].into(), options: c[6].split(" | ").map(String::from).collect(),
                choice: num(7)? - 1, understood: c[9] == "1", right: num(8)? - 1, ns: c[14].parse().map_err(|_| "ns".to_string())? },
            mova_scores: c[10].split(" | ").map(|s| s.parse().ok()).collect(),
        });
    }
    Ok(rows)
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    if v.is_empty() { 0.0 } else if v.len() % 2 == 1 { v[v.len() / 2] } else { (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.0 }
}
fn quantile(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(f64::total_cmp);
    if v.is_empty() { 0.0 } else { v[((v.len() - 1) as f64 * q).round() as usize] }
}

/// The student's decision classes, read from its own trace: how it decided.
fn mova_route(r: &Row) -> &'static str {
    let s = r.mova_scores.get(r.ex.choice).copied().flatten();
    if !r.ex.understood { return "guess"; }
    match s {
        None => "rule-or-count",
        Some(top) => if r.mova_scores.iter().enumerate().any(|(j, x)| j != r.ex.choice && *x == Some(top)) { "support-tie" } else { "support" },
    }
}

#[derive(Default, Clone)]
struct Tally { n: usize, choice: usize, und: usize, correct_same: usize, pred_right: usize, mova_right: usize }

fn main() -> Res<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 4 { return Err("usage: student_vidtvorant SCHOOL_DIR PROBE_ROOT OUT_DIR COURSE...".into()); }
    let (school, root, out) = (Path::new(&args[0]), Path::new(&args[1]), Path::new(&args[2]));
    let names = &args[3..];
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let mut reps: Vec<PathBuf> = std::fs::read_dir(root).map_err(|e| e.to_string())?.flatten().map(|e| e.path())
        .filter(|p| p.is_dir() && p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.parse::<u32>().is_ok())).collect();
    reps.sort();
    if reps.is_empty() { return Err("no repetitions".into()); }
    // the courses, with every repetition's times; the choices must agree across repetitions
    let mut courses: Vec<CourseData> = Vec::new();
    let mut rows_of: Vec<Vec<Row>> = Vec::new();
    for name in names {
        let mut base: Option<Vec<Row>> = None;
        let mut times: Vec<Vec<f64>> = Vec::new();
        for rep in &reps {
            let p = rep.join(format!("{name}.trace.tsv"));
            if !p.exists() { continue; }
            let rows = trace(&p)?;
            if let Some(b) = &base {
                if b.len() != rows.len() || b.iter().zip(&rows).any(|(a, c)| a.ex.choice != c.ex.choice || a.ex.understood != c.ex.understood) {
                    return Err(format!("{name}: the student's choices differ between repetitions"));
                }
            } else {
                times = vec![Vec::new(); rows.len()];
                base = Some(rows.iter().map(|r| Row { file: r.file.clone(), lesson: r.lesson, ex: r.ex.clone(), mova_scores: r.mova_scores.clone() }).collect());
            }
            for (t, r) in times.iter_mut().zip(&rows) { t.push(r.ex.ns); }
        }
        let Some(mut rows) = base else { return Err(format!("{name}: no trace")); };
        for (r, t) in rows.iter_mut().zip(times.iter_mut()) { r.ex.ns = median(t); }
        let files = lessons(&school.join(name))?;
        let mut by_lesson: BTreeMap<usize, LessonData> = BTreeMap::new();
        for (n, lines) in files.values() {
            by_lesson.insert(*n, LessonData { n: *n, lines: lines.clone(), examples: Vec::new() });
        }
        for r in &rows {
            by_lesson.get_mut(&r.lesson).ok_or(format!("{name}: lesson {} of {} not found", r.lesson, r.file))?.examples.push(r.ex.clone());
        }
        courses.push(CourseData { name: name.clone(), lessons: by_lesson.into_values().collect() });
        rows_of.push(rows);
    }
    let reps_used = reps.len();
    // the folds: lesson parity both ways, then courses split in two
    let grid: Vec<(usize, usize)> = [2usize, 3].iter().flat_map(|&a| [2usize, 3].map(|o| (a, o))).collect();
    let agree = |p: &Params, test: &dyn Fn(&CourseData, &LessonData) -> bool| -> (usize, usize) {
        let (mut ok, mut n) = (0, 0);
        for c in &courses {
            vid::replay(c, p, |_, l, ex, pr| if test(c, l) { ok += (pr.choice == ex.choice && pr.understood == ex.understood) as usize; n += 1; });
        }
        (ok, n)
    };
    // per fold: the structural thresholds from a small grid, the weights by the perceptron, then the support gate
    // (the least counted score) from the train fold's score quantiles; the best train agreement wins, the earlier on a tie
    let fold_fit = |train: &dyn Fn(&CourseData, &LessonData) -> bool| -> Params {
        let mut best: Option<(usize, Params)> = None;
        for &(a, o) in &grid {
            let start = Params { anchor_min: a, own_min: o, ..Params::default() };
            let p = vid::fit(&courses, train, &start, 5);
            let mut scores: Vec<i64> = Vec::new();
            for c in &courses {
                vid::replay(c, &p, |_, l, _, pr| if train(c, l) {
                    let pool = if pr.lesson_first { &pr.recent } else { &pr.best };
                    if let Some(s) = pool.get(pr.choice).and_then(|x| x.as_ref()) { scores.push(s.score); }
                });
            }
            scores.sort();
            let mut cands: Vec<i64> = vec![i64::MIN];
            cands.extend((0..20).filter_map(|k| scores.get(k * scores.len() / 20).copied()));
            cands.dedup();
            // the gap gate first (both gates equal), then the no-gap gate with the gap gate fixed
            let mut local: Option<(usize, Params)> = None;
            for &th in &cands {
                let q = Params { min_score: th, min_score_nogap: th, ..p.clone() };
                let (ok, _) = agree(&q, train);
                if local.as_ref().is_none_or(|(k, _)| ok > *k) { local = Some((ok, q)); }
            }
            let (_, base) = local.clone().unwrap();
            for &th in &cands {
                let q = Params { min_score_nogap: th, ..base.clone() };
                let (ok, _) = agree(&q, train);
                if local.as_ref().is_none_or(|(k, _)| ok > *k) { local = Some((ok, q)); }
            }
            let (ok, q) = local.unwrap();
            if best.as_ref().is_none_or(|(k, _)| ok > *k) { best = Some((ok, q)); }
        }
        best.unwrap().1
    };
    let folds: Vec<(String, Admit, Admit)> = vec![
        ("fit even lessons, test odd".into(), Box::new(|_, l| l.n % 2 == 0), Box::new(|_, l| l.n % 2 == 1)),
        ("fit odd lessons, test even".into(), Box::new(|_, l| l.n % 2 == 1), Box::new(|_, l| l.n % 2 == 0)),
    ];
    let half: Vec<String> = names.iter().enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, n)| n.clone()).collect();
    let half2 = half.clone();
    let cross: Vec<(String, Admit, Admit)> = vec![
        (format!("fit courses {}, test the others", half.join(",")), Box::new(move |c, _| half.contains(&c.name)), Box::new(move |c, _| !half2.contains(&c.name))),
    ];
    let mut sem = String::new();
    let _ = writeln!(sem, "{{\n  \"courses\": {:?},\n  \"repetitions\": {reps_used},", names);
    let mut timing = String::from("{\n");
    // the hand start without any fit, for reference
    let (ok0, n0) = agree(&Params::default(), &|_, _| true);
    let _ = writeln!(sem, "  \"hand_start_agreement\": [{ok0}, {n0}],");
    // per question, out of fold: the prediction of the parity folds
    type Key = (usize, usize, String, usize);
    let mut pred_of: BTreeMap<Key, (Prediction, [f64; vid::NT], usize)> = BTreeMap::new();
    let mut params_json = Vec::new();
    let mut fitted: Vec<Params> = Vec::new();
    for (label, train, test) in &folds {
        let p = fold_fit(train.as_ref());
        fitted.push(p.clone());
        params_json.push(format!("{{\"fold\": {label:?}, \"anchor_min\": {}, \"own_min\": {}, \"min_score\": {}, \"min_score_nogap\": {}, \"w\": {:?}}}", p.anchor_min, p.own_min, p.min_score, p.min_score_nogap, p.w));
        for (ci, c) in courses.iter().enumerate() {
            vid::replay(c, &p, |mem, l, ex, pr| {
                if test(c, l) {
                    let x = vid::time_features(mem.lines.len(), &ex.question, &ex.options, pr);
                    pred_of.insert((ci, l.n, ex.kind.clone(), ex.n), (pr.clone(), x, 0));
                }
            });
        }
    }
    // the time model, fitted per parity fold on the other fold's questions
    let mut time_pred: BTreeMap<(usize, usize, String, usize), f64> = BTreeMap::new();
    for parity in [0usize, 1] {
        let (mut xs, mut ys) = (Vec::new(), Vec::new());
        for (ci, c) in courses.iter().enumerate() {
            for l in c.lessons.iter().filter(|l| l.n % 2 != parity) {
                for ex in &l.examples {
                    if let Some((_, x, _)) = pred_of.get(&(ci, l.n, ex.kind.clone(), ex.n)) { xs.push(*x); ys.push(ex.ns); }
                }
            }
        }
        let tm = TimeModel::fit(&xs, &ys);
        let _ = writeln!(timing, "  \"time_model_fit_on_parity_{}\": {:?},", 1 - parity, tm.beta.map(|b| (b * 1e4).round() / 1e4));
        for (ci, c) in courses.iter().enumerate() {
            for l in c.lessons.iter().filter(|l| l.n % 2 == parity) {
                for ex in &l.examples {
                    let key = (ci, l.n, ex.kind.clone(), ex.n);
                    if let Some((_, x, _)) = pred_of.get(&key) { time_pred.insert(key, tm.predict_ns(x)); }
                }
            }
        }
    }
    let _ = writeln!(sem, "  \"parity_fold_params\": [{}],", params_json.join(", "));
    // agreement, by course and overall; disagreement classes with examples
    let mut total = Tally::default();
    let mut per_course: Vec<(String, Tally, f64, f64)> = Vec::new();
    let mut classes: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_route: BTreeMap<&'static str, (usize, usize)> = BTreeMap::new();
    let mut examples: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut rel_time: Vec<f64> = Vec::new();
    let mut course_time: Vec<(String, f64, f64)> = Vec::new();
    for (ci, c) in courses.iter().enumerate() {
        let mut t = Tally::default();
        let (mut real_ns, mut pred_ns) = (0.0, 0.0);
        let (mut tests, mut mova_tr, mut pred_tr) = (0usize, 0usize, 0usize);
        for r in &rows_of[ci] {
            let key = (ci, r.lesson, r.ex.kind.clone(), r.ex.n);
            let Some((pr, _, _)) = pred_of.get(&key) else { continue };
            let ex = &r.ex;
            t.n += 1;
            let same = pr.choice == ex.choice;
            t.choice += same as usize;
            t.und += (pr.understood == ex.understood) as usize;
            t.correct_same += ((pr.choice == ex.right) == (ex.choice == ex.right)) as usize;
            t.pred_right += (pr.choice == ex.right) as usize;
            t.mova_right += (ex.choice == ex.right) as usize;
            if ex.kind != "exam" { tests += 1; mova_tr += (ex.choice == ex.right) as usize; pred_tr += (pr.choice == ex.right) as usize; }
            let route = mova_route(r);
            let e = by_route.entry(route).or_default();
            e.0 += same as usize;
            e.1 += 1;
            if let Some(tp) = time_pred.get(&key) {
                real_ns += ex.ns;
                pred_ns += tp;
                rel_time.push((tp - ex.ns).abs() / ex.ns.max(1.0));
            }
            if !same {
                let class = match (ex.understood, pr.understood) {
                    (false, true) => "student guessed, vidtvorant found support".to_string(),
                    (true, false) => format!("student decided ({route}), vidtvorant found none"),
                    (true, true) => format!("both decided, other option (student: {route})"),
                    (false, false) => "both guessed, other option".to_string(),
                };
                *classes.entry(class.clone()).or_default() += 1;
                let v = examples.entry(class).or_default();
                if v.len() < 4 && (ci + r.lesson) % 3 == 0 {
                    v.push(format!("{} L{} {} #{}: {} || options: {} || student {} / vidtvorant {} / right {}", c.name, r.lesson, ex.kind, ex.n, ex.question, ex.options.join(" | "), ex.choice + 1, pr.choice + 1, ex.right + 1));
                }
            }
        }
        let mova_score = mova_tr as f64 / tests.max(1) as f64;
        let pred_score = pred_tr as f64 / tests.max(1) as f64;
        course_time.push((c.name.clone(), real_ns, pred_ns));
        total.n += t.n; total.choice += t.choice; total.und += t.und; total.correct_same += t.correct_same; total.pred_right += t.pred_right; total.mova_right += t.mova_right;
        per_course.push((c.name.clone(), t, mova_score, pred_score));
    }
    let pct = |a: usize, n: usize| (1e4 * a as f64 / n.max(1) as f64).round() / 100.0;
    let _ = writeln!(sem, "  \"parity_out_of_fold\": {{\"questions\": {}, \"choice_agreement_pct\": {}, \"understood_agreement_pct\": {}, \"correctness_agreement_pct\": {}, \"student_right_pct\": {}, \"vidtvorant_right_pct\": {}}},",
        total.n, pct(total.choice, total.n), pct(total.und, total.n), pct(total.correct_same, total.n), pct(total.mova_right, total.n), pct(total.pred_right, total.n));
    let mut pc = Vec::new();
    let mut max_score_rel: f64 = 0.0;
    for (name, t, ms, ps) in &per_course {
        let rel = if *ms > 0.0 { (ps - ms).abs() / ms } else { ps.abs() };
        max_score_rel = max_score_rel.max(rel);
        pc.push(format!("    {{\"course\": {name:?}, \"questions\": {}, \"choice_agreement_pct\": {}, \"understood_agreement_pct\": {}, \"correctness_agreement_pct\": {}, \"student_test_score_pct\": {}, \"vidtvorant_test_score_pct\": {}, \"score_relative_error_pct\": {}}}",
            t.n, pct(t.choice, t.n), pct(t.und, t.n), pct(t.correct_same, t.n), (ms * 1e4).round() / 100.0, (ps * 1e4).round() / 100.0, (rel * 1e4).round() / 100.0));
    }
    let _ = writeln!(sem, "  \"per_course\": [\n{}\n  ],", pc.join(",\n"));
    let _ = writeln!(sem, "  \"max_course_score_relative_error_pct\": {},", (max_score_rel * 1e4).round() / 100.0);
    let rt: Vec<String> = by_route.iter().map(|(k, (a, n))| format!("{k:?}: [{a}, {n}]")).collect();
    let _ = writeln!(sem, "  \"agreement_by_student_route\": {{{}}},", rt.join(", "));
    let cl: Vec<String> = classes.iter().map(|(k, v)| format!("{k:?}: {v}")).collect();
    let _ = writeln!(sem, "  \"disagreements\": {{{}}},", cl.join(", "));
    let exs: Vec<String> = examples.iter().map(|(k, v)| format!("{k:?}: {:?}", v)).collect();
    let _ = writeln!(sem, "  \"disagreement_examples\": {{{}}},", exs.join(",\n    "));
    // the course split
    for (label, train, test) in &cross {
        let p = fold_fit(train.as_ref());
        let (ok, n) = agree(&p, test.as_ref());
        let (mut ch, mut cs) = (0usize, 0usize);
        for c in &courses {
            vid::replay(c, &p, |_, l, ex, pr| if test(c, l) { ch += (pr.choice == ex.choice) as usize; cs += ((pr.choice == ex.right) == (ex.choice == ex.right)) as usize; });
        }
        let _ = writeln!(sem, "  \"course_split\": {{\"fold\": {label:?}, \"anchor_min\": {}, \"own_min\": {}, \"min_score\": {}, \"min_score_nogap\": {}, \"w\": {:?}, \"questions\": {n}, \"choice_and_understood_agreement_pct\": {}, \"choice_agreement_pct\": {}, \"correctness_agreement_pct\": {}}}",
            p.anchor_min, p.own_min, p.min_score, p.min_score_nogap, p.w, pct(ok, n), pct(ch, n), pct(cs, n));
    }
    sem += "}\n";
    std::fs::write(out.join("semantic.json"), &sem).map_err(|e| e.to_string())?;
    // timing: the vidtvorant's own predict time against the student's answer time (same questions), with the first
    // parity fold's parameters
    let p = fitted[0].clone();
    let mut vid_ns: Vec<f64> = Vec::new();
    let mut replay_s = 0.0;
    let mut real_ns: Vec<f64> = rows_of.iter().flatten().map(|r| r.ex.ns).collect();
    for c in &courses {
        let t0 = Instant::now();
        let mut mem = vid::Memory::default();
        for l in &c.lessons {
            for t in &l.lines { mem.read_line(l.n, t); }
            for ex in &l.examples {
                let t = Instant::now();
                let pr = vid::predict(&mem, l.n, &ex.question, &ex.options, &p);
                vid_ns.push(t.elapsed().as_nanos() as f64);
                std::hint::black_box(pr);
            }
        }
        replay_s += t0.elapsed().as_secs_f64();
    }
    let q = vid_ns.len().max(1) as f64;
    let (vm, vp) = (median(&mut vid_ns.clone()), quantile(&mut vid_ns.clone(), 0.95));
    let (rm, rp) = (median(&mut real_ns.clone()), quantile(&mut real_ns, 0.95));
    let ct: Vec<String> = course_time.iter().map(|(n, r, p)| format!("{{\"course\": {n:?}, \"student_answer_s\": {:.4}, \"predicted_s\": {:.4}, \"relative_error_pct\": {:.2}}}", r / 1e9, p / 1e9, 100.0 * (p - r).abs() / r.max(1.0))).collect();
    let max_course_time_rel = course_time.iter().map(|(_, r, p)| (p - r).abs() / r.max(1.0)).fold(0.0, f64::max);
    let _ = writeln!(timing, "  \"student_answer_ns_median\": {rm:.0}, \"student_answer_ns_p95\": {rp:.0},\n  \"vidtvorant_predict_ns_median\": {vm:.0}, \"vidtvorant_predict_ns_p95\": {vp:.0},\n  \"speedup_median\": {:.1},\n  \"vidtvorant_replay_s_total\": {replay_s:.4}, \"vidtvorant_replay_ns_per_question\": {:.0},",
        rm / vm.max(1.0), replay_s * 1e9 / q);
    let _ = writeln!(timing, "  \"time_prediction_per_question_relative_error\": {{\"median_pct\": {:.2}, \"p90_pct\": {:.2}}},", 100.0 * median(&mut rel_time.clone()), 100.0 * quantile(&mut rel_time, 0.9));
    let _ = writeln!(timing, "  \"course_answer_time\": [{}],\n  \"max_course_answer_time_relative_error_pct\": {:.2}\n}}", ct.join(", "), 100.0 * max_course_time_rel);
    std::fs::write(out.join("timing.json"), &timing).map_err(|e| e.to_string())?;
    print!("{sem}{timing}");
    Ok(())
}
