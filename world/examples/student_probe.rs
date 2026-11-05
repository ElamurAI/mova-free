//! The school student, run through its public API with a clock on every stage and every answer.
//!
//! `student_probe OUT_DIR COURSE...` reads, practises and tests each course the way `world knowledge course` does with
//! `PRAG_OFF=course-controls` and `MOVA_COURSE_BEFORE=0` (no controls, no score before the exercises): parse ahead,
//! the body plans, then per lesson read, two exercise passes (`learn`, the recomputation reference of
//! `learn_lesson`), the tests and the exam. It writes `OUT_DIR/<course>.trace.tsv` (one row per test or exam
//! question: the shown options, the student's choice, understanding, scores, anchor and target counts and the answer's
//! wall time) and `OUT_DIR/<course>.stages.json` (stage times, counts, peak resident memory). Nothing is learned from
//! tests; nothing is saved into the shared state. The choices must equal the course trace's (`--check TRACE`).
use anyhow::{Context, Result, bail, ensure};
use std::fmt::Write as _;
use std::time::Instant;
use world::knowledge::{PragStudent, course, shuffled};

fn peak_rss_kib() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| s.lines().find(|l| l.starts_with("VmHWM:")).and_then(|l| l.split_whitespace().nth(1)?.parse().ok()))
        .unwrap_or(0)
}

/// User + system CPU seconds of this process (clock ticks of 1/100 s, as Linux reports them).
fn cpu_seconds() -> f64 {
    std::fs::read_to_string("/proc/self/stat")
        .ok()
        .and_then(|s| {
            let rest = s.rsplit_once(')')?.1.split_whitespace().collect::<Vec<_>>();
            Some((rest.get(11)?.parse::<f64>().ok()? + rest.get(12)?.parse::<f64>().ok()?) / 100.0)
        })
        .unwrap_or(0.0)
}

fn cell(s: &str) -> String {
    s.replace(['\t', '\n', '\r'], " ")
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        bail!("usage: student_probe OUT_DIR COURSE... (one process per course gives a per-course memory peak)");
    }
    let out = std::path::Path::new(&args[0]);
    std::fs::create_dir_all(out)?;
    for name in &args[1..] {
        probe(out, name)?;
    }
    Ok(())
}

fn probe(out: &std::path::Path, name: &str) -> Result<()> {
    let ls = course(name)?;
    ensure!(!ls.is_empty(), "course {name}: no lessons");
    let mut texts: Vec<(String, bool)> = Vec::new();
    let (mut lines, mut options) = (0usize, 0usize);
    for l in &ls {
        for x in l.explain.iter().chain(&l.facts) {
            texts.push((x.clone(), false));
            lines += 1;
        }
        for (q, r, w) in l.exercises.iter().chain(&l.tests).chain(&l.exams) {
            texts.push((q.clone(), true));
            texts.push((r.clone(), false));
            texts.extend(w.iter().map(|x| (x.clone(), false)));
            options += 1 + w.len();
        }
    }
    // what parse ahead parses: the distinct units (sentences) of every text, and their words
    let units: std::collections::BTreeSet<String> = texts.iter().flat_map(|(t, q)| prag::frames::units(t, *q).into_iter().map(|u| u.0)).collect();
    let words: usize = units.iter().map(|u| u.split_whitespace().count()).sum();
    let wall = Instant::now();
    let cpu0 = cpu_seconds();
    let mut st = PragStudent::default();
    let t = Instant::now();
    st.preparse(&texts)?;
    let parse_s = t.elapsed().as_secs_f64();
    let t = Instant::now();
    st.read_kinds();
    let kinds_s = t.elapsed().as_secs_f64();
    let (mut read_s, mut learn_s, mut test_s, mut exam_s) = (0f64, 0f64, 0f64, 0f64);
    let (mut right, mut n, mut und, mut exercises) = (0usize, 0usize, 0usize, 0usize);
    let mut trace = String::from("course\tfile\tlesson\tkind\tn\tquestion\toptions\tchoice\tright\tunderstood\tscores\tanchors\ttargets\tmemory\tanswer_ns\n");
    for l in &ls {
        let t = Instant::now();
        let before = st.remembered();
        st.read(l);
        read_s += t.elapsed().as_secs_f64();
        let _ = before;
        let t = Instant::now();
        for _ in 0..2 {
            for (i, (q, r, w)) in l.exercises.iter().enumerate() {
                let (opts, at) = shuffled((l.n as u64) << 20 | (i as u64) << 1, r, w);
                st.learn(q, &opts, at);
            }
        }
        exercises += 2 * l.exercises.len();
        learn_s += t.elapsed().as_secs_f64();
        let memory = st.remembered();
        let seed = |i: usize| (l.n as u64) << 20 | (i as u64) << 1 | 1;
        for (kind, qs) in [(if l.stars == 0 { "plain".to_string() } else { "★".repeat(l.stars.min(3)) }, &l.tests), ("exam".to_string(), &l.exams)] {
            for (i, (q, r, w)) in qs.iter().enumerate() {
                let s = if kind == "exam" { seed(i) ^ 0xE8A3 } else { seed(i) };
                let (opts, at) = shuffled(s, r, w);
                let t = Instant::now();
                let a = st.answer(q, &opts);
                let ns = t.elapsed().as_nanos();
                if kind == "exam" { exam_s += ns as f64 / 1e9 } else { test_s += ns as f64 / 1e9 }
                if kind != "exam" {
                    right += (a.choice == at) as usize;
                    n += 1;
                    und += a.understood as usize;
                }
                let scores: Vec<String> = a.options.iter().map(|o| o.best.as_ref().map(|b| b.0.to_string()).unwrap_or_else(|| "–".into())).collect();
                let _ = writeln!(
                    trace,
                    "{name}\t{}\t{}\t{kind}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{memory}\t{ns}",
                    l.file, l.n, i + 1, cell(q),
                    opts.iter().map(|o| cell(o)).collect::<Vec<_>>().join(" | "),
                    a.choice + 1, at + 1, a.understood as u8, scores.join(" | "), a.anchors.len(), a.targets.len()
                );
            }
        }
    }
    let wall_s = wall.elapsed().as_secs_f64();
    let cpu_s = cpu_seconds() - cpu0;
    std::fs::write(out.join(format!("{name}.trace.tsv")), &trace)?;
    let stages = serde_json::json!({
        "course": name, "lessons": ls.len(), "teaching_lines": lines, "questions_all_kinds": texts.len() - lines - options,
        "options": options, "parse_units": units.len(), "parse_words_whitespace": words,
        "seconds": {"parse_ahead_with_model_load": parse_s, "body_plans": kinds_s, "read": read_s, "learn": learn_s,
                    "tests": test_s, "exam": exam_s, "wall": wall_s, "cpu": cpu_s},
        "tests": n, "tests_right": right, "tests_understood": und, "exercise_answers": exercises,
        "frames_remembered": st.remembered(), "peak_rss_kib": peak_rss_kib(),
    });
    std::fs::write(out.join(format!("{name}.stages.json")), serde_json::to_string_pretty(&stages)?)
        .with_context(|| format!("writing {name}.stages.json"))?;
    println!("{name}: {right}/{n} right, wall {wall_s:.1}s, parse {parse_s:.1}s, read {read_s:.1}s, learn {learn_s:.1}s, tests {test_s:.2}s, exam {exam_s:.2}s");
    Ok(())
}
