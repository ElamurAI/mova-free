//! Sea Turtle's standing loop with its native work in bulk.
//!
//! A lesson check takes milliseconds, so pacing every lesson like a teacher question (one step every SECS) would take
//! a month over the school. The loop here alternates two kinds of step:
//!
//! - **a sweep** (`sweep`): one pass over the open lessons in the job's order (`LessonJob::lessons`; held-out and
//!   control sets are never listed), each read and held to `check::lesson`; a lesson that keeps the rules is closed
//!   with the same journal line the `nautilus::work` loop writes (`{"event":"native","key","note","unix"}`), at most
//!   `batch` of them per sweep, all appended in one write; a lesson with a problem stays open for the teacher. A
//!   sweep that closed something is followed at once by the next sweep, with no pause.
//! - **a teacher step** (`nautilus::work::Loop::step`), only when a sweep closed nothing: it picks the first open
//!   lesson (one that needs the teacher, or a new file) and keeps all of that loop's gates — the question pacing
//!   (`min_gap_secs`), the operator's word, limits, the review-only proposals. After it the loop pauses SECS (four
//!   times that while idle or waiting).
//!
//! A `stopped` line without a later `resumed` holds the sweeps too; a `STOP` file in the state folder ends the run.
use crate::make::LessonJob;
use anyhow::{Result, ensure};
use nautilus::teacher::PrivateTerminal;
use nautilus::work::{Loop, Native};
use serde_json::{Value, json};
use std::io::Write;
use std::path::Path;

fn journal_lines(state: &Path) -> Result<Vec<Value>> {
    match std::fs::read_to_string(state.join("work.jsonl")) {
        Ok(t) => Ok(t.lines().filter_map(|l| serde_json::from_str(l).ok()).collect()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(e.into()),
    }
}

/// One sweep at time `now`: `{"event":"sweep","native":…,"open":…,"checked":…}` (open: lessons left that need the
/// teacher), or `{"event":"stopped"}` while a stop holds.
pub fn sweep(l: &Loop, job: &LessonJob, batch: usize, now: u64) -> Result<Value> {
    ensure!(l.state.is_absolute(), "the state folder is absolute");
    for f in &l.frozen {
        ensure!(!l.state.starts_with(f), "the state folder {} is inside the frozen path {}", l.state.display(), f.display());
    }
    ensure!(l.ledger.is_none(), "the Turtle's sweep writes its own journal only");
    let lines = journal_lines(&l.state)?;
    let stop = lines.iter().rposition(|x| x["event"] == "stopped");
    let resume = lines.iter().rposition(|x| x["event"] == "resumed");
    if let Some(i) = stop.filter(|i| resume.is_none_or(|r| r < *i)) {
        return Ok(json!({"event":"stopped","reason":lines[i]["reason"]}));
    }
    let seen = nautilus::work::seen(&lines, &[]);
    let (mut out, mut native, mut open, mut checked) = (String::new(), 0usize, 0usize, 0usize);
    for (key, path) in job.lessons()? {
        if seen.contains(&key) {
            continue;
        }
        if native >= batch {
            open += 1;
            continue;
        }
        let body = std::fs::read_to_string(&path)?;
        checked += 1;
        match nautilus::work::Job::native(job, &nautilus::work::Item { key: key.clone(), body }) {
            Native::Done(note) => {
                out.push_str(&format!("{}\n", json!({"event":"native","key":key,"note":note,"unix":now})));
                native += 1;
            }
            Native::Unsure(_) => open += 1,
        }
    }
    if !out.is_empty() {
        std::fs::create_dir_all(&l.state)?;
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(l.state.join("work.jsonl"))?;
        f.write_all(out.as_bytes())?;
        f.sync_data()?;
    }
    Ok(json!({"event":"sweep","native":native,"open":open,"checked":checked,"unix":now}))
}

/// Run the loop: sweeps without a pause while they close lessons, then a teacher step and a pause of `every_secs`
/// (four times that while idle or waiting); at most `steps` steps (sweeps and teacher steps) if given. Prints each
/// step's line and returns the journal's summary.
pub fn run(l: &Loop, job: &LessonJob, term: &dyn PrivateTerminal, every_secs: u64, steps: Option<u64>, batch: usize) -> Result<Value> {
    let now = || std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut n = 0u64;
    let done = |n: u64| steps.is_some_and(|k| n >= k);
    loop {
        if l.state.join("STOP").exists() {
            break;
        }
        let s = sweep(l, job, batch.max(1), now())?;
        println!("{s}");
        n += 1;
        if s["event"] == "stopped" || done(n) {
            break;
        }
        if s["native"].as_u64().unwrap_or(0) > 0 {
            continue;
        }
        let v = l.step(job, term, now())?;
        println!("{v}");
        n += 1;
        if v["event"] == "stopped" || done(n) {
            break;
        }
        let pause = if matches!(v["event"].as_str(), Some("idle" | "waiting")) { every_secs * 4 } else { every_secs };
        std::thread::sleep(std::time::Duration::from_secs(pause.max(1)));
    }
    nautilus::work::summary(&l.state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nautilus::teacher::{Config as Teachers, TerminalError};
    struct Never;
    impl PrivateTerminal for Never {
        fn id(&self) -> &str {
            "never"
        }
        fn ask(&self, _: &str) -> std::result::Result<String, TerminalError> {
            panic!("the sweep never asks the teacher")
        }
        fn ready(&self) -> bool {
            false
        }
    }
    fn tmp(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("turtle-{name}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }
    fn good_lesson(n: usize) -> String {
        let mut s = format!("course: c\nlesson: {n}\ntitle: T\nlevel: primary-4\nstars: 0\nexplain: E.\n");
        for i in 0..5 {
            s.push_str(&format!("fact: Fact {i}.\n"));
        }
        for (kind, k) in [("exercise", 12), ("test", 8)] {
            for i in 0..k {
                let mut o: Vec<String> = (0..4).map(|j| format!("option {j}")).collect();
                o[i % 4] = format!("*{}", o[i % 4]);
                s.push_str(&format!("{kind}: Question {kind} {i}? | {}\n", o.join(" | ")));
            }
        }
        s
    }

    #[test]
    fn sweeps_close_native_lessons_in_bulk_and_leave_teacher_work_open() {
        let d = tmp("sweep");
        let (course, held, state) = (d.join("c"), d.join("c-test"), d.join("state"));
        std::fs::create_dir_all(&course).unwrap();
        std::fs::create_dir_all(&held).unwrap();
        for n in 1..=25 {
            std::fs::write(course.join(format!("{n:03}.lesson")), good_lesson(n)).unwrap();
        }
        // lesson 7 breaks the rule: it stays open for the teacher
        std::fs::write(course.join("007.lesson"), good_lesson(7).replace("test: Question test 0?", "fact: x\nnote: Question test 0?")).unwrap();
        std::fs::write(held.join("001.lesson"), good_lesson(1)).unwrap();
        let job = LessonJob { courses: vec![held.display().to_string(), course.display().to_string()] };
        let t: Teachers = serde_json::from_value(json!({"journal": d.join("t.jsonl"), "profiles": []})).unwrap();
        let l = Loop { state: state.clone(), teachers: &t, profile: "p".into(), min_gap_secs: 600, frozen: vec![course.clone()], ledger: None, member: "p".into() };
        let s = sweep(&l, &job, 10, 100).unwrap();
        assert_eq!((s["native"].as_u64(), s["checked"].as_u64(), s["open"].as_u64()), (Some(10), Some(11), Some(15)), "{s}");
        let s = sweep(&l, &job, 1000, 101).unwrap();
        assert_eq!((s["native"].as_u64(), s["open"].as_u64()), (Some(14), Some(1)), "{s}");
        let lines = journal_lines(&state).unwrap();
        assert_eq!(lines.len(), 24);
        // the journal line is the loop's own: the nautilus loop reads it as closed
        assert_eq!(lines[0], json!({"event":"native","key":format!("{}/001.lesson", course.display()),"note":"20 questions keep the rules","unix":100}));
        let seen = nautilus::work::seen(&lines, &[]);
        assert!(!seen.iter().any(|k| k.contains("c-test")), "the held-out course is never read");
        let next = nautilus::work::Job::next(&job, &seen).unwrap().unwrap();
        assert!(next.key.ends_with("c/007.lesson"), "{}", next.key);
        // a bounded run of two steps: a sweep closes the new lesson and the next follows without a pause
        std::fs::write(course.join("026.lesson"), good_lesson(26)).unwrap();
        let v = run(&l, &job, &Never, 1, Some(2), 1000).unwrap();
        assert_eq!(v["events"]["native"], 25, "{v}");
        assert!(v["events"].get("held").is_none() && v["events"].get("waiting").is_none(), "no teacher step while sweeps close lessons: {v}");
        // a stop holds the sweeps; the state folder may not be inside a course
        std::fs::write(state.join("work.jsonl"), format!("{}\n", json!({"event":"stopped","reason":"limit"}))).unwrap();
        assert_eq!(sweep(&l, &job, 10, 102).unwrap()["event"], "stopped");
        let inside = Loop { state: course.join("state"), ..l };
        assert!(sweep(&inside, &job, 10, 103).is_err());
    }
}
