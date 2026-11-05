//! What Sea Turtle makes: natively, gap-filling questions from a 🐚 Nautilus dictionary layer, as draft lessons in
//! the school's format; and, for a teacher terminal behind the operator's word, the task of writing lessons.
//!
//! **Gap questions.** For each entry of a layer, each example that holds the word itself as a whole word becomes a
//! question: the sentence with the word replaced by «___», «Which word fills the gap?». The three wrong options are
//! other words of the layer (its words of the same part of speech first), drawn by a hash of the word and the
//! sentence, each held to the fit check (`fit`): never a near-synonym of the answer (WordNet, the meanings), and
//! shown in a form that breaks the gap's slot (agreement or subcategorisation), never a form of the
//! sentence. A gap with fewer than three such options is dropped. The right option stands at a drawn place and
//! carries the leading `*` (`lessons mark`).
//!
//! **Lessons keep the school's rule.** Entries are taken in order into a lesson, their questions dealt to exercises
//! and tests (three to two), the facts their meanings' first sentences, and the lesson closes after the first entry
//! at which it holds 12 exercises, 8 tests and 5 facts (`world::knowledge::lesson_problem`). A tail too small for the
//! rule joins the lesson before it; a layer too small for one lesson makes none.
//!
//! **The journal.** Every making appends one line to the Turtle's journal (`made.jsonl`): the source and its hash,
//! the model's hash, the folder, the lessons and questions made, the gaps dropped and the candidates refused by
//! reason. «How many questions did you make?» is answered from it.
use crate::fit::{Lex, Slot};
use anyhow::{Context, Result};
use nautilus::layer::{Row, parse_rows};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::Path;

/// FNV-1a, 64 bits: a deterministic draw.
pub fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// The part-of-speech class of an entry: the first alphabetic word of its column, lower-cased.
pub fn pos_class(r: &Row) -> String {
    r.pos.split(|c: char| !c.is_alphabetic()).find(|w| !w.is_empty()).unwrap_or("").to_lowercase()
}

/// An example without its source id: `text «id»` → `text`.
pub fn example_text(e: &str) -> &str {
    match e.rfind(" «") {
        Some(i) if e.ends_with('»') => &e[..i],
        _ => e,
    }
}

fn words_of(s: &str) -> BTreeSet<String> {
    s.split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '-').filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
}

/// The sentence with the first whole-word occurrence of `word` (any case) as «___»; None without one.
pub fn gap(sentence: &str, word: &str) -> Option<String> {
    let lower = sentence.to_lowercase();
    let w = word.to_lowercase();
    let mut from = 0;
    while let Some(i) = lower[from..].find(&w).map(|i| i + from) {
        let before = lower[..i].chars().next_back();
        let after = lower[i + w.len()..].chars().next();
        let edge = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric() && c != '\'' && c != '-');
        if edge(before) && edge(after) && sentence.is_char_boundary(i) && sentence.is_char_boundary(i + w.len()) {
            return Some(format!("{}___{}", &sentence[..i], &sentence[i + w.len()..]));
        }
        from = i + w.len().max(1);
        if from >= lower.len() {
            break;
        }
    }
    None
}

/// One question line's body: `question | options` with the right one starred at its drawn place.
pub fn question(sentence: &str, right: &str, wrong: &[String]) -> String {
    let at = (fnv(&format!("{right}\u{1}{sentence}")) % 4) as usize;
    let mut opts: Vec<String> = wrong.iter().take(3).cloned().collect();
    opts.insert(at.min(opts.len()), format!("*{right}"));
    format!("«{}» Which word fills the gap? | {}", sentence.replace(" | ", " / "), opts.join(" | "))
}

/// The fit check's inputs: the lexicons, and the reader of a gap's slot (Mova's annotator in `make_gap_course`).
pub struct Fit<'a> {
    pub lex: &'a Lex,
    /// the slot of the answer (second) in the sentence (first), or None when the reader does not find it
    pub slot: &'a dyn Fn(&str, &str) -> Option<Slot>,
}

/// The coarse reason of a refusal, for the journal's counts.
pub fn reason_kind(why: &str) -> &'static str {
    if why.starts_with("WordNet") {
        "near-synonym (WordNet)"
    } else if why.starts_with("dictionary") {
        "near-synonym (meanings)"
    } else if why.contains("can stand") {
        "fits the gap"
    } else if why.contains("in the sentence") {
        "in the sentence"
    } else {
        "other"
    }
}

/// Three wrong options for `word` in `sentence` (its words), with the reasons of the candidates refused on the way:
/// the layer's other words, its class first, each in the hash order of the word and the sentence, judged until
/// twelve are accepted; of those, the three shown least so far (`uses`, by form), in that order.
pub fn wrong_options(rows: &[Row], word: &str, class: &str, sentence: &str, slot: Slot, lex: &Lex, uses: &BTreeMap<String, usize>) -> (Vec<String>, Vec<String>) {
    let in_sentence = words_of(sentence);
    let answer = word.to_lowercase();
    let mut pool: Vec<&Row> = rows.iter().filter(|r| r.word.to_lowercase() != answer).collect();
    pool.sort_by_key(|r| (pos_class(r) != class, fnv(&format!("{word}\u{1}{sentence}\u{1}{}", r.word)), r.word.clone()));
    let (mut ok, mut refused): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    let mut lemmas = BTreeSet::new();
    for r in pool {
        let cand = r.word.to_lowercase();
        if !lemmas.insert(cand.clone()) {
            continue;
        }
        match lex.judge(&answer, &cand, slot, &in_sentence) {
            Ok((form, _)) if !ok.contains(&form) => ok.push(form),
            Ok(_) => {}
            Err(why) => refused.push(why),
        }
        if ok.len() == 12 {
            break;
        }
    }
    let mut order: Vec<(usize, usize)> = ok.iter().enumerate().map(|(i, f)| (uses.get(f).copied().unwrap_or(0), i)).collect();
    order.sort();
    (order.iter().take(3).map(|&(_, i)| ok[i].clone()).collect(), refused)
}

fn first_sentence(s: &str) -> String {
    let s = s.trim_start_matches("1) ");
    let s = s.split(" 2) ").next().unwrap_or(s);
    let end = s.find(". ").map(|i| i + 1).unwrap_or(s.len());
    s[..end].trim().to_string()
}

/// What a making produced: the lessons (file name, text), their questions, the gaps dropped (no slot, or fewer
/// than three fitting wrong options) and the refused candidates by reason.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Made {
    pub lessons: Vec<(String, String)>,
    pub questions: usize,
    pub dropped: usize,
    pub refused: BTreeMap<String, usize>,
}

struct Draft {
    facts: Vec<String>,
    exercises: Vec<String>,
    tests: Vec<String>,
}

impl Draft {
    fn keeps_the_rule(&self) -> bool {
        self.exercises.len() >= 12 && self.tests.len() >= 8 && self.facts.len() >= 5
    }
    fn push(&mut self, q: String) {
        // three exercises to two tests
        if self.tests.len() * 3 < self.exercises.len() * 2 { self.tests.push(q) } else { self.exercises.push(q) }
    }
}

/// The draft lessons of a layer, held to the fit check and the school's rule.
pub fn gap_lessons(rows: &[Row], course: &str, source: &str, fit: &Fit) -> Made {
    let mut made = Made::default();
    let mut drafts: Vec<Draft> = Vec::new();
    let mut cur = Draft { facts: vec![], exercises: vec![], tests: vec![] };
    // a gapped sentence asked once per making (two words can leave the same gap)
    let mut asked = BTreeSet::new();
    // how often each wrong option has been shown, to spread them
    let mut uses: BTreeMap<String, usize> = BTreeMap::new();
    for r in rows {
        let class = pos_class(r);
        let meaning = if r.meaning.is_empty() { first_sentence(&r.sense) } else { first_sentence(&r.meaning) };
        let mut qs = Vec::new();
        for e in &r.examples {
            let s = example_text(e);
            let Some(g) = gap(s, &r.word) else { continue };
            if asked.contains(&g) {
                continue;
            }
            let Some(slot) = (fit.slot)(s, &r.word.to_lowercase()) else {
                made.dropped += 1;
                continue;
            };
            let (wrong, refused) = wrong_options(rows, &r.word, &class, s, slot, fit.lex, &uses);
            for why in &refused {
                *made.refused.entry(reason_kind(why).to_string()).or_default() += 1;
            }
            if wrong.len() < 3 {
                made.dropped += 1;
                continue;
            }
            asked.insert(g.clone());
            for w in &wrong {
                *uses.entry(w.clone()).or_default() += 1;
            }
            qs.push(question(&g, &r.word, &wrong));
        }
        if qs.is_empty() {
            continue;
        }
        cur.facts.push(format!("«{}» ({class}): {meaning}", r.word));
        for q in qs {
            cur.push(q);
        }
        if cur.keeps_the_rule() {
            drafts.push(std::mem::replace(&mut cur, Draft { facts: vec![], exercises: vec![], tests: vec![] }));
        }
    }
    // a tail too small for the rule joins the lesson before it
    if !cur.facts.is_empty() {
        match drafts.last_mut() {
            Some(last) => {
                last.facts.extend(cur.facts);
                last.exercises.extend(cur.exercises);
                last.tests.extend(cur.tests);
            }
            None if cur.keeps_the_rule() => drafts.push(cur),
            None => made.dropped += cur.exercises.len() + cur.tests.len(),
        }
    }
    for (k, d) in drafts.iter().enumerate() {
        let n = k + 1;
        let mut t = format!(
            "# draft: made natively by Sea Turtle from {source}; not reviewed\ncourse: {course}\nlesson: {n}\ntitle: Words of {source}, part {n}\nlevel: primary-4\nstars: 0\nexplain: Each question is a sentence of {source} with one word taken out; choose the word that fills the gap.\n"
        );
        for f in &d.facts {
            t.push_str(&format!("fact: {f}\n"));
        }
        for q in &d.exercises {
            t.push_str(&format!("exercise: {q}\n"));
        }
        for q in &d.tests {
            t.push_str(&format!("test: {q}\n"));
        }
        made.questions += d.exercises.len() + d.tests.len();
        made.lessons.push((format!("{n:03}-words-{n}.lesson"), t));
    }
    made
}

/// Make the gap lessons of a layer file into a new folder and journal it, reading the slots with the annotator
/// model at `model` (Mova's `en.bin`). The folder must not exist yet.
pub fn make_gap_course(layer: &Path, course: &str, source: &str, out: &Path, journal: &Path, now: u64, model: &Path) -> Result<Value> {
    let text = std::fs::read_to_string(layer).with_context(|| format!("{}", layer.display()))?;
    let rows = parse_rows(&text);
    let bytes = std::fs::read(model).with_context(|| format!("the annotator model {}", model.display()))?;
    let annotator = en::annotate::Annotator::from_bytes(&bytes).with_context(|| format!("{}", model.display()))?;
    let words: BTreeSet<String> = rows.iter().map(|r| r.word.to_lowercase()).collect();
    let meanings: BTreeMap<String, String> = rows.iter().map(|r| (r.word.to_lowercase(), r.meaning.clone())).collect();
    let lex = Lex::load(&words, &meanings)?;
    let slot = |s: &str, w: &str| crate::fit::slot_of(&annotator, s, w);
    let m = gap_lessons(&rows, course, source, &Fit { lex: &lex, slot: &slot });
    let shas = (mova_octopus::digest(text.as_bytes()), mova_octopus::digest(&bytes));
    write_gap_course(&m, layer, shas, course, out, journal, now)
}

/// Write a making into the new folder `out` and journal it with the layer's and the model's hashes.
pub fn write_gap_course(m: &Made, layer: &Path, shas: (String, String), course: &str, out: &Path, journal: &Path, now: u64) -> Result<Value> {
    std::fs::create_dir_all(out.parent().context("the folder needs a parent")?)?;
    std::fs::create_dir(out).with_context(|| format!("{} must be a new folder", out.display()))?;
    for (f, t) in &m.lessons {
        std::fs::write(out.join(f), t)?;
    }
    let v = json!({"event":"made","kind":"gap","source":layer,"source_sha256":shas.0,"model_sha256":shas.1,"course":course,"out":out,
        "lessons":m.lessons.len(),"questions":m.questions,"dropped":m.dropped,"refused":m.refused,"unix":now});
    if let Some(d) = journal.parent() {
        std::fs::create_dir_all(d)?;
    }
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(journal)?;
    f.write_all(format!("{v}\n").as_bytes())?;
    f.sync_data()?;
    Ok(v)
}

/// What the journal says the Turtle has made: makings, lessons, questions.
pub fn made(journal: &Path) -> Result<Value> {
    let text = match std::fs::read_to_string(journal) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e.into()),
    };
    let (mut n, mut l, mut q) = (0u64, 0u64, 0u64);
    for v in text.lines().filter_map(|x| serde_json::from_str::<Value>(x).ok()).filter(|v| v["event"] == "made") {
        n += 1;
        l += v["lessons"].as_u64().unwrap_or(0);
        q += v["questions"].as_u64().unwrap_or(0);
    }
    Ok(json!({"makings":n,"lessons":l,"questions":q}))
}

/// A teacher task: write `n` lessons of a course in the school's format (for a teacher terminal, behind the
/// operator's word; see `nautilus::teacher`). The brief is the school's own (docs/school.md#how-courses-are-made).
pub fn lessons_task(course: &str, topic: &str, first: usize, n: usize) -> String {
    format!(
        "You are a lesson writer for Mova's school (docs/school.md#how-courses-are-made). Write {n} lessons of the course «{course}», \
numbered from {first}, on: {topic}. Use exactly the lesson format: lines `course:`, `lesson:`, `title:`, `level: primary-4`, \
`stars: 0`, then 4-5 `explain:`, 11-14 `fact:`, 15-18 `exercise:`, 10-12 `test:` and 4 `exam:` lines. A question line is \
`question | option | option | option | option` with exactly one right option marked by a leading `*` at a varied \
place and three wrong ones (myths make good wrong ones). Lessons in real speech, tests in casual talk, exams in stiff \
official language. No sexual content, no slurs, no how-to for anything dangerous; real people only as public record. \
Return only the lesson files, each starting with its `course:` line. This is a draft for review, not approval.\n"
    )
}

/// Sea Turtle's standing job (`nautilus::work`): the lessons of the configured courses, course by course, file by
/// file. Held-out and control sets (a course name with `test` or `shuffled`) are never read. Mova's own reading is
/// `check::lesson`: a lesson that keeps the rules is done natively; one with a problem goes to the teacher with its
/// problems, and the answer is kept as a proposal only when it is the same lesson (course and number) and passes the
/// same check. The course files are never written: the proposal is a review-only file in the loop's state.
pub struct LessonJob {
    pub courses: Vec<String>,
}

/// A lesson's identity: its `course:` and `lesson:` values.
pub fn identity(text: &str) -> (String, String) {
    let field = |k: &str| text.lines().find_map(|l| l.trim().strip_prefix(k).map(|v| v.trim().to_string())).unwrap_or_default();
    (field("course:"), field("lesson:"))
}

impl LessonJob {
    /// Every lesson of the configured courses in the loop's order (course by course, file names sorted), with its
    /// key (`course/file`); held-out and control sets are never listed.
    pub fn lessons(&self) -> Result<Vec<(String, std::path::PathBuf)>> {
        let mut out = Vec::new();
        for c in self.courses.iter().filter(|c| !held_out(c)) {
            let dir = crate::check::folder(c);
            let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
                .with_context(|| format!("{}", dir.display()))?
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x == "lesson"))
                .collect();
            files.sort();
            out.extend(files.into_iter().map(|f| (format!("{c}/{}", f.file_name().and_then(|x| x.to_str()).unwrap_or("")), f)));
        }
        Ok(out)
    }
}

pub fn held_out(course: &str) -> bool {
    course.contains("test") || course.contains("shuffled")
}

impl nautilus::work::Job for LessonJob {
    fn purpose(&self) -> &'static str {
        "questions"
    }
    fn next(&self, seen: &std::collections::BTreeSet<String>) -> Result<Option<nautilus::work::Item>> {
        match self.lessons()?.into_iter().find(|(k, _)| !seen.contains(k)) {
            Some((key, f)) => Ok(Some(nautilus::work::Item { key, body: std::fs::read_to_string(&f)? })),
            None => Ok(None),
        }
    }
    fn open(&self, seen: &std::collections::BTreeSet<String>) -> Result<usize> {
        Ok(self.lessons()?.iter().filter(|(k, _)| !seen.contains(k)).count())
    }
    fn native(&self, item: &nautilus::work::Item) -> nautilus::work::Native {
        let r = crate::check::lesson(&item.key, &item.body);
        if r.problems.is_empty() {
            return nautilus::work::Native::Done(format!("{} questions keep the rules", r.questions));
        }
        if item.body.len() > 9_000 {
            return nautilus::work::Native::Done("has a problem but is too long for one teacher task; left for a person".into());
        }
        nautilus::work::Native::Unsure(format!(
            "This lesson of Mova's school breaks the school's rules: {}\nRewrite the whole lesson so that it keeps them, \
keeping its course, number, title and facts; fix only the questions. Rules: 12 or more different exercises, 8 or \
more tests, every question with exactly four options, one right option marked by a leading * at a varied place.\n\n{}",
            r.problems.join("; "),
            item.body
        ))
    }
    fn validate(&self, item: &nautilus::work::Item, answer: &str) -> std::result::Result<String, String> {
        let was = identity(&item.body);
        let r = crate::check::lesson(&item.key, answer);
        if !r.problems.is_empty() {
            return Err(r.problems.join("; "));
        }
        let now = identity(answer);
        if was != now {
            return Err(format!("the answer is lesson {:?}, not lesson {:?}", now, was));
        }
        Ok(answer.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn row(w: &str, pos: &str, ex: &[&str]) -> Row {
        Row { word: w.into(), pos: pos.into(), meaning: format!("1) {w} means something. 2) More."), sense: String::new(), examples: ex.iter().map(|s| s.to_string()).collect(), note: String::new() }
    }
    #[test]
    fn gaps_are_whole_words() {
        assert_eq!(gap("Pass the salt, please.", "pass").as_deref(), Some("___ the salt, please."));
        assert_eq!(gap("A passage; then pass it.", "pass").as_deref(), Some("A passage; then ___ it."));
        assert_eq!(gap("Bypass it.", "pass"), None);
        assert_eq!(example_text("Pass it. «commands:1:fact:2»"), "Pass it.");
    }
    const VERBS: [&str; 12] = ["pass", "shut", "fetch", "grab", "hold", "carry", "stop", "wait", "check", "bring", "drop", "lift"];
    fn verb_rows(n: usize) -> Vec<Row> {
        VERBS.iter().take(n).enumerate().map(|(i, w)| row(w, "verb (x)", &[&format!("Please {w} it now, number {i}. «a:{i}»"), &format!("Did you {w} the bag {i} times? «b:{i}»"), &format!("Did you {w} the bag? «d:{i}»"), &format!("They {w} things {i} times a day. «c:{i}»")])).collect()
    }
    /// The lexicons of the tests: «fetch» and «bring» share a synset; every gap is a base-form verb slot.
    fn lex() -> Lex {
        let mut l = Lex::default();
        l.synsets.insert("fetch".into(), ["v-bring".to_string()].into());
        l.synsets.insert("bring".into(), ["v-bring".to_string()].into());
        l
    }
    fn base(_: &str, _: &str) -> Option<Slot> {
        Some(Slot::of(en::gram::Tag::VB))
    }
    #[test]
    fn lessons_keep_the_rule_and_options_break_the_slot() {
        let rows = verb_rows(12);
        let l = lex();
        let m = gap_lessons(&rows, "words-test", "a test source", &Fit { lex: &l, slot: &base });
        // the fourth example of each word leaves the same gap for every word: asked once per making
        assert_eq!(m.questions, 12 * 3 + 1);
        assert_eq!(m.lessons.len(), 1, "a tail too small for the rule joins the lesson before it");
        for (f, t) in &m.lessons {
            let r = crate::check::lesson(f, t);
            assert!(r.problems.is_empty(), "{:?}", r.problems);
            assert!(r.places.len() > 1, "{:?}", r.places);
            let l = world::knowledge::parse(t, f).unwrap();
            assert!(l.exercises.len() >= 12 && l.tests.len() >= 8 && l.facts.len() >= 5);
            for (qq, right, wrong) in l.exercises.iter().chain(&l.tests) {
                assert!(qq.contains("___"));
                assert_eq!(wrong.len(), 3);
                // none in the sentence, none a base form that would fill the verb slot, never the near-synonym
                for w in wrong {
                    assert!(!qq.contains(&format!(" {w} ")), "{w} in {qq}");
                    assert!(!VERBS.contains(&w.as_str()), "the base form {w} fits the gap of {right}: {qq}");
                    assert!(!crate::fit::tags_of(w).contains("VB"), "{w}");
                }
                if right == "fetch" || right == "bring" {
                    assert!(wrong.iter().all(|w| !w.starts_with("fetch") && !w.starts_with("brought") && !w.starts_with("bring")), "{right}: {wrong:?}");
                }
            }
        }
        // deterministic
        assert_eq!(gap_lessons(&rows, "words-test", "a test source", &Fit { lex: &l, slot: &base }), m);
        // a layer too small for one lesson makes none
        let small = gap_lessons(&verb_rows(4), "words-test", "a test source", &Fit { lex: &l, slot: &base });
        assert!(small.lessons.is_empty() && small.questions == 0 && small.dropped > 0, "{small:?}");
    }
    #[test]
    fn the_fit_check_is_what_refuses_red_control() {
        // without the fit check (every candidate accepted in its base form), wrong options fill the slot
        let rows = verb_rows(12);
        let s = "Please pass it now, number 0.";
        let (wrong, refused) = wrong_options(&rows, "pass", "verb", s, Slot::of(en::gram::Tag::VB), &lex(), &BTreeMap::new());
        assert_eq!(wrong.len(), 3);
        assert!(wrong.iter().all(|w| !crate::fit::tags_of(w).contains("VB")), "{wrong:?}");
        assert!(refused.iter().all(|r| r.contains("can stand")), "only a verb whose real forms all fit is refused: {refused:?}");
        let naive: Vec<&str> = rows.iter().filter(|r| r.word != "pass").map(|r| r.word.as_str()).take(3).collect();
        assert!(naive.iter().all(|w| crate::fit::tags_of(w).contains("VB")), "the old draw would show forms that fit: {naive:?}");
        // a slot no form can break: every candidate refused, the gap dropped
        let adverbs: Vec<Row> = ["now", "out", "just", "still", "again", "once"].iter().map(|w| row(w, "adverb", &[])).collect();
        let (wrong, refused) = wrong_options(&adverbs, "now", "adverb", "The door opened for her now.", Slot::of(en::gram::Tag::RB), &Lex::default(), &BTreeMap::new());
        assert!(wrong.is_empty(), "{wrong:?}");
        assert_eq!(refused.len(), 5);
        assert!(refused.iter().all(|r| r.contains("can stand")), "{refused:?}");
        // a planted near-synonym is refused
        assert!(lex().judge("fetch", "bring", Slot::of(en::gram::Tag::VB), &Default::default()).unwrap_err().contains("shares the synset"));
    }
    #[test]
    fn lesson_job_reads_checks_and_validates() {
        use nautilus::work::{Job, Native};
        let d = std::env::temp_dir().join(format!("turtle-job-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let (good, bad) = (d.join("good"), d.join("x-test"));
        std::fs::create_dir_all(&good).unwrap();
        std::fs::create_dir_all(&bad).unwrap();
        let l = lex();
        let lessons = gap_lessons(&verb_rows(10), "good", "a test source", &Fit { lex: &l, slot: &base }).lessons;
        std::fs::write(good.join("001.lesson"), &lessons[0].1).unwrap();
        // the same lesson with seven tests only, one too few
        let mut kept = 0;
        let broken: String = lessons[0].1.lines().filter(|l| { let keep = !l.starts_with("test:") || kept < 7; kept += usize::from(l.starts_with("test:")); keep }).map(|l| format!("{l}\n")).collect();
        std::fs::write(good.join("002.lesson"), &broken).unwrap();
        std::fs::write(bad.join("001.lesson"), &lessons[0].1).unwrap();
        let job = LessonJob { courses: vec![bad.display().to_string(), good.display().to_string()] };
        let first = job.next(&Default::default()).unwrap().unwrap();
        assert!(first.key.ends_with("good/001.lesson"), "the held-out course is never read: {}", first.key);
        assert!(matches!(job.native(&first), Native::Done(_)));
        let second = job.next(&[first.key.clone()].into_iter().collect()).unwrap().unwrap();
        let Native::Unsure(task) = job.native(&second) else { panic!("a broken lesson goes to the teacher") };
        assert!(task.contains("breaks the school's rules"));
        // the teacher's fixed lesson is kept; a still broken one, or another lesson, is rejected
        assert!(job.validate(&second, &lessons[0].1).is_ok());
        assert!(job.validate(&second, &broken).is_err());
        assert!(job.validate(&second, &lessons[0].1.replace("lesson: 1", "lesson: 7")).unwrap_err().contains("not lesson"));
    }
    #[test]
    fn journal_counts() {
        let d = std::env::temp_dir().join(format!("turtle-made-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&d).unwrap();
        let l = lex();
        let m = gap_lessons(&verb_rows(10), "t", "a test", &Fit { lex: &l, slot: &base });
        let layer = d.join("layer.tsv");
        assert_eq!(made(&d.join("made.jsonl")).unwrap()["questions"], 0);
        let shas = || ("s".to_string(), "m".to_string());
        let v = write_gap_course(&m, &layer, shas(), "t", &d.join("out"), &d.join("made.jsonl"), 1).unwrap();
        assert_eq!((v["lessons"].as_u64(), v["questions"].as_u64()), (Some(1), Some(31)));
        // a second making into the same folder is refused, and nothing is journaled for it
        assert!(write_gap_course(&m, &layer, shas(), "t", &d.join("out"), &d.join("made.jsonl"), 2).is_err());
        assert_eq!(made(&d.join("made.jsonl")).unwrap(), json!({"makings":1,"lessons":1,"questions":31}));
        assert!(lessons_task("commands", "pets", 31, 2).contains("numbered from 31"));
    }
}
