//! `turtle check`: a course or a folder of lessons against the school's own rules.
//!
//! Two parts. **The format**: every `.lesson` file is parsed by the school itself (`world::knowledge::parse`) and
//! held to the rule of `world knowledge check` (`world::knowledge::lesson_problem`: facts, exercises, tests, three
//! wrong answers each) — the same function, so the two checks cannot drift. **The marks** (the convention of
//! `lessons mark`, docs/school.md#how-courses-are-made): the right option carries a leading `*` at a drawn place; a lesson whose
//! questions are unmarked uses the old form (right option first), and a lesson whose marks all stand at one place
//! gives the answer away. Marks are read from the raw lines, since parsing drops them.
use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The fields that hold questions.
const QUESTIONS: [&str; 3] = ["exercise", "test", "exam"];

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq)]
pub struct LessonReport {
    pub file: String,
    pub n: usize,
    pub questions: usize,
    pub marked: usize,
    pub unmarked: usize,
    /// marked questions by the place of the right option (1 = first option)
    pub places: BTreeMap<usize, usize>,
    pub problems: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq)]
pub struct Report {
    pub course: String,
    pub lessons: Vec<LessonReport>,
    pub questions: usize,
    pub with_problems: usize,
}

/// The marks of one lesson's raw text: (questions, marked, unmarked, places).
pub fn marks(text: &str) -> (usize, usize, usize, BTreeMap<usize, usize>) {
    let (mut q, mut m, mut u) = (0, 0, 0);
    let mut places = BTreeMap::new();
    for line in text.lines().map(str::trim) {
        let Some((k, v)) = line.split_once(':') else { continue };
        if !QUESTIONS.contains(&k.trim()) {
            continue;
        }
        q += 1;
        let opts: Vec<&str> = v.split(" | ").skip(1).map(str::trim).collect();
        match opts.iter().position(|o| o.starts_with('*')) {
            Some(i) => {
                m += 1;
                *places.entry(i + 1).or_default() += 1;
            }
            None => u += 1,
        }
    }
    (q, m, u, places)
}

/// Check one lesson file's text.
pub fn lesson(file: &str, text: &str) -> LessonReport {
    let (questions, marked, unmarked, places) = marks(text);
    let mut r = LessonReport { file: file.into(), questions, marked, unmarked, places, ..Default::default() };
    match world::knowledge::parse(text, file) {
        Ok(l) => {
            r.n = l.n;
            r.problems.extend(world::knowledge::lesson_problem(&l));
        }
        Err(e) => r.problems.push(format!("✗ {file}: {e}")),
    }
    if marked > 0 && unmarked > 0 {
        r.problems.push(format!("✗ {file}: {marked} questions are marked with * and {unmarked} are not"));
    }
    if marked >= 8 && r.places.len() == 1 {
        let p = r.places.keys().next().copied().unwrap_or(0);
        r.problems.push(format!("✗ {file}: every right option stands at place {p}, which gives it away"));
    }
    r
}

/// A course name (`courses/<name>` of the school data, `$MOVA_DATA/school`) or a folder.
pub fn folder(course: &str) -> PathBuf {
    let p = Path::new(course);
    if p.is_absolute() || course.contains('/') { p.to_path_buf() } else { world::knowledge::course_dir(course) }
}

/// Check every `.lesson` file of a course or folder, in name order.
pub fn course(course: &str) -> Result<Report> {
    let dir = folder(course);
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .with_context(|| format!("{}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "lesson"))
        .collect();
    files.sort();
    let mut rep = Report { course: course.into(), ..Default::default() };
    for f in files {
        let name = f.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();
        let r = lesson(&name, &std::fs::read_to_string(&f)?);
        rep.questions += r.questions;
        rep.with_problems += usize::from(!r.problems.is_empty());
        rep.lessons.push(r);
    }
    Ok(rep)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn lesson_text(marked_at: impl Fn(usize) -> usize, exercises: usize) -> String {
        let mut s = String::from("course: t\nlesson: 3\ntitle: T\nlevel: primary-4\nstars: 0\nexplain: E.\n");
        for i in 0..5 {
            s.push_str(&format!("fact: Fact {i}.\n"));
        }
        let q = |kind: &str, i: usize| {
            let mut o: Vec<String> = (0..4).map(|k| format!("option {k}")).collect();
            let at = marked_at(i);
            if at > 0 {
                o[at - 1] = format!("*{}", o[at - 1]);
            }
            format!("{kind}: Question {kind} {i}? | {}\n", o.join(" | "))
        };
        for i in 0..exercises {
            s.push_str(&q("exercise", i));
        }
        for i in 0..8 {
            s.push_str(&q("test", i));
        }
        s
    }
    #[test]
    fn a_good_lesson_passes() {
        let r = lesson("003.lesson", &lesson_text(|i| i % 4 + 1, 12));
        assert_eq!(r.problems, Vec::<String>::new());
        assert_eq!((r.n, r.questions, r.marked, r.unmarked), (3, 20, 20, 0));
        assert_eq!(r.places.len(), 4);
    }
    #[test]
    fn controls_turn_red() {
        // too few exercises: the school's own rule
        let r = lesson("a.lesson", &lesson_text(|i| i % 4 + 1, 11));
        assert!(r.problems[0].starts_with("✗ 3 (a.lesson): facts 5, exercises 11"), "{:?}", r.problems);
        // every right option at one place
        let r = lesson("b.lesson", &lesson_text(|_| 2, 12));
        assert!(r.problems.iter().any(|p| p.contains("place 2")), "{:?}", r.problems);
        // marked and unmarked mixed
        let r = lesson("c.lesson", &lesson_text(|i| if i == 0 { 0 } else { i % 4 + 1 }, 12));
        assert!(r.problems.iter().any(|p| p.contains("are not")), "{:?}", r.problems);
        // two marks in one question: the school refuses to parse it
        let bad = lesson_text(|i| i % 4 + 1, 12);
        let r = lesson("d.lesson", &bad.replace("Question exercise 0? | *option 0 | option 1", "Question exercise 0? | *option 0 | *option 1"));
        assert!(r.problems.iter().any(|p| p.contains("more than one right option")), "{:?}", r.problems);
    }
    #[test]
    fn the_command_course_as_it_is() {
        let r = course("commands").unwrap();
        assert_eq!(r.lessons.len(), 30);
        assert!(r.questions > 1000, "{}", r.questions);
    }
}
