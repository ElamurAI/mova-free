//! The student's vidtvorant: a fast text-level model of `PragStudent::answer` (world/src/knowledge.rs).
//!
//! Subject: the course pupil's test answer after it has read a course's lessons up to the current one. Observations:
//! the option it chooses, whether it «understood» (a choice backed by support rather than the first-option guess) and
//! the answer's wall time. It is not a solver: it predicts Mova's choice, right or wrong. Standard library only; the
//! runner (`tools/approx/examples/student_vidtvorant.rs`) feeds it traces and lesson files and measures it.
//!
//! The decision structure mirrors the student's code, with frames replaced by bags of stemmed content words
//! (`knowledge::words`: `mentions::stem`, the same stop list):
//!
//! 1. Reading the question. A generic tail («… Why?», «… What happens next?», «… What's true?», `tail_question`) is cut
//!    off and its scenario read; «why» routes to the next line's cause, «what next» to the following line. A question
//!    with no content word is not understood (the student: no frame, first option).
//! 2. Anchors. Remembered lines sharing at least `anchor_min` content words with the question; the current lesson first
//!    when it has any (`student-anchor-first`); only the strongest (≥ 3 shared) when there are any; ten at most, the
//!    stronger and later first.
//! 3. Targets. Each anchor line itself (route Anchor) and its neighbouring lines in the same lesson (route Linked);
//!    the next line for «what next» (route Next).
//! 4. Own support. Lines about the question's things (sharing one of its words) that share at least `own_min` words
//!    with the option (route Own, ten at most), as the student's option-side candidates.
//! 5. Support. Per option and target: integer features (words of the option the line holds beyond the question — the
//!    asked part —, words shared with the question too, the option's words the line lacks, a negation mismatch,
//!    route, current lesson). A target counts only with structure (at least one asked word, or `own_min` overlap on
//!    the Own route) and a score of at least the support gate (`min_score`; `min_score_nogap` for a question without
//!    a wh-word before its final «?» and without a generic tail), standing for the student's strength gates. The score
//!    is a linear function of the features (the student's role-match weights, here fitted to its choices).
//! 6. Choice. The lesson just read first (`student-lesson-first`): where some option has current-lesson support, the
//!    best current-lesson score decides; else the best score overall, ties to the current lesson, then the first
//!    shown. Between tied options, one whose words are a strict subset of the leader's and that claims less unsupported
//!    content wins (`student-extra`). No supported option: not understood, first option.
//! 7. Counting. «How many / how much» without the question's own numbers: when exactly one option's number stands in an
//!    anchor line, it decides (the kind base and the arithmetic reader, much simplified).
//!
//! Not modelled (explicit gaps): the parser and the frame reader (roles, polarity, quotes, clause links), WordNet
//! synonyms and kinds, the verb gate, compiled rules, the kind base's sums, the arithmetic tree reader, the exercise
//! perceptron's per-ask, per-route weights, and every switch other than the defaults.
use std::collections::{BTreeMap, BTreeSet};

/// The stop words of `knowledge::words`.
pub const STOP: [&str; 40] = [
    "the", "a", "an", "of", "to", "in", "on", "at", "is", "are", "was", "were", "be", "it", "its", "and", "or", "for",
    "by", "with", "what", "which", "why", "how", "do", "does", "did", "that", "this", "from", "as", "can", "when", "who",
    "into", "than", "there", "they", "you", "your",
];
const NEG: [&str; 8] = ["not", "no", "never", "nobody", "nothing", "none", "neither", "nor"];

/// `mentions::stem`: irregular forms to their base, then one ending cut.
pub fn stem(w: &str) -> String {
    let w = w.to_lowercase();
    let base = match w.as_str() {
        "ate" | "eaten" => "eat", "gave" | "given" => "give", "got" | "gotten" => "get", "found" => "find",
        "lost" => "lose", "bought" => "buy", "sold" => "sell", "made" => "make", "spent" => "spend",
        "took" | "taken" => "take", "grew" | "grown" => "grow", "threw" | "thrown" => "throw", "flew" | "flown" => "fly",
        "won" => "win", "ran" => "run", "drove" | "driven" => "drive", "wrote" | "written" => "write", "read" => "read",
        "put" => "put", "cut" => "cut", "left" => "leave", "went" | "gone" => "go", "came" => "come",
        "saw" | "seen" => "see", "broke" | "broken" => "break", "children" => "child", "people" => "person",
        "men" => "man", "women" => "woman", "feet" => "foot", "teeth" => "tooth", "mice" => "mouse",
        "geese" => "goose", "leaves" => "leaf",
        other => other,
    };
    let mut s = base.to_string();
    for suf in ["ing", "ies", "ed", "es", "s", "e"] {
        if s.len() > suf.len() + 2 && s.ends_with(suf) {
            s.truncate(s.len() - suf.len());
            if suf == "ies" {
                s.push('y');
            }
            break;
        }
    }
    s
}

/// `knowledge::words`, as a set: stemmed content words.
pub fn words(t: &str) -> BTreeSet<String> {
    t.split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 1)
        .map(|w| w.to_lowercase())
        .filter(|w| !STOP.contains(&w.as_str()) && !NEG.contains(&w.as_str()))
        .map(|w| stem(&w))
        .filter(|w| !STOP.contains(&w.as_str()))
        .collect()
}

fn negated(t: &str) -> bool {
    t.split(|c: char| !c.is_alphanumeric() && c != '\'')
        .any(|w| { let w = w.to_lowercase(); NEG.contains(&w.as_str()) || w.ends_with("n't") })
}

fn numbers(t: &str) -> Vec<i64> {
    const NAMES: [&str; 13] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve"];
    t.split(|c: char| !c.is_alphanumeric())
        .filter_map(|w| w.parse::<i64>().ok().or_else(|| NAMES.iter().position(|n| n.eq_ignore_ascii_case(w)).map(|i| i as i64)))
        .collect()
}

/// A remembered teaching line (`explain:` or `fact:`), the vidtvorant's stand-in for the line's frames.
#[derive(Clone, Debug)]
pub struct Line {
    pub lesson: usize,
    pub words: BTreeSet<String>,
    pub neg: bool,
    pub nums: Vec<i64>,
}

/// What the student has read: lines in reading order, with a word index.
#[derive(Clone, Debug, Default)]
pub struct Memory {
    pub lines: Vec<Line>,
    index: BTreeMap<String, Vec<usize>>,
}

impl Memory {
    pub fn read_line(&mut self, lesson: usize, text: &str) {
        let i = self.lines.len();
        let line = Line { lesson, words: words(text), neg: negated(text), nums: numbers(text) };
        for w in &line.words {
            self.index.entry(w.clone()).or_default().push(i);
        }
        self.lines.push(line);
    }

    /// Lines sharing a word with `ws`, with the count shared, in line order.
    fn sharing(&self, ws: &BTreeSet<String>) -> BTreeMap<usize, usize> {
        let mut m: BTreeMap<usize, usize> = BTreeMap::new();
        for w in ws {
            for &i in self.index.get(w).into_iter().flatten() {
                *m.entry(i).or_default() += 1;
            }
        }
        m
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Route {
    Anchor = 0,
    Next = 1,
    Linked = 2,
    Own = 3,
}

/// The number of support features.
pub const NF: usize = 9;

/// Support features of one option against one target line.
fn features(ow: &BTreeSet<String>, qw: &BTreeSet<String>, oneg: bool, line: &Line, route: Route, current: bool) -> [i64; NF] {
    let asked = ow.iter().filter(|w| line.words.contains(*w) && !qw.contains(*w)).count() as i64;
    let echo = ow.iter().filter(|w| line.words.contains(*w) && qw.contains(*w)).count() as i64;
    let missing = ow.iter().filter(|w| !line.words.contains(*w)).count() as i64;
    [
        asked.min(6),
        echo.min(6),
        missing.min(8),
        (oneg != line.neg) as i64,
        (route == Route::Anchor) as i64,
        (route == Route::Next) as i64,
        (route == Route::Linked) as i64,
        (route == Route::Own) as i64,
        current as i64,
    ]
}

/// The vidtvorant's parameters: structural thresholds and the support weights.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Params {
    pub anchor_min: usize,
    pub own_min: usize,
    /// the least score a target must reach to count as support (the student's strength gates: «st ≥ 3, or ≥ 2 with
    /// the asked role»), fitted after the weights
    pub min_score: i64,
    /// the gate for a question without a readable gap (no wh-word before a final «?», no generic tail): the student
    /// reads such a line as a statement with no asked role and decides less often
    pub min_score_nogap: i64,
    pub w: [i64; NF],
}

impl Default for Params {
    fn default() -> Self {
        // a hand start: asked words count most, the option's unsupported words and a negation mismatch count against
        Params { anchor_min: 2, own_min: 2, min_score: i64::MIN, min_score_nogap: i64::MIN, w: [8, 2, -2, -3, 2, 2, 0, 0, 1] }
    }
}

/// The best support of an option: its score, the features that gave it, and whether it was in the current lesson.
#[derive(Clone, Debug)]
pub struct Support {
    pub score: i64,
    pub feats: [i64; NF],
    pub current: bool,
}

#[derive(Clone, Debug)]
pub struct Prediction {
    pub choice: usize,
    pub understood: bool,
    /// per option: the best support overall and the best in the current lesson
    pub best: Vec<Option<Support>>,
    pub recent: Vec<Option<Support>>,
    pub lesson_first: bool,
    pub counted: bool,
    /// the anchors and targets found (inputs of the time model)
    pub anchors: usize,
    pub targets: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tail {
    Cause,
    Next,
    Claim,
}

/// `knowledge::tail_question`.
fn tail_question(q: &str) -> Option<(&str, Tail)> {
    let t = q.trim_end();
    let body = &t[..t.char_indices().last().map_or(0, |(i, _)| i)];
    let cut = body.rfind(['.', '!', '?', ':', ';']).filter(|&i| i + 1 < t.len())?;
    let (scenario, tail) = (t[..=cut].trim(), t[cut + 1..].trim());
    if scenario.split_whitespace().count() < 3 {
        return None;
    }
    let tl = tail.to_lowercase();
    let tl = tl.trim_end_matches(['?', '!', '.']).trim();
    let kind = match tl {
        "why" | "why is that" | "why so" | "why does this happen" | "why does that happen" => Tail::Cause,
        "what happens" | "what happens next" | "what will happen" | "what could happen" | "what would happen"
        | "what happens then" | "what happens now" | "what will happen next" => Tail::Next,
        "what's true" | "what is true" | "which is true" | "what is right" | "right" | "true" | "what is the truth"
        | "what is wrong" | "what's wrong" | "which is right" | "what's right" | "which is correct" | "what is correct"
        | "is that right" | "is that true" | "true or false" => Tail::Claim,
        _ => return None,
    };
    Some((scenario, kind))
}

/// Whether a question has an asked role the reader can find: a wh-word and a final question mark.
pub fn has_gap(q: &str) -> bool {
    const WH: [&str; 9] = ["who", "whom", "whose", "what", "which", "where", "when", "why", "how"];
    q.trim_end().ends_with('?') && q.split(|c: char| !c.is_alphanumeric()).any(|w| WH.contains(&w.to_lowercase().as_str()))
}

/// The student's answer to `question` with `options` shown, having read `mem` up to lesson `lesson` (the current one).
pub fn predict(mem: &Memory, lesson: usize, question: &str, options: &[String], p: &Params) -> Prediction {
    let (text, tail) = match tail_question(question) {
        Some((s, k)) => (s, Some(k)),
        None => (question, None),
    };
    let qw = words(text);
    let gap = tail.is_some() || has_gap(question);
    let gate = if gap { p.min_score } else { p.min_score_nogap };
    let n = options.len();
    let mut pred = Prediction { choice: 0, understood: false, best: vec![None; n], recent: vec![None; n], lesson_first: false, counted: false, anchors: 0, targets: 0 };
    if qw.is_empty() || n == 0 {
        return pred;
    }
    // 2. anchors
    let shared = mem.sharing(&qw);
    let mut scored: Vec<(usize, usize)> = shared.iter().filter(|x| *x.1 >= p.anchor_min).map(|(&i, &c)| (i, c)).collect();
    if scored.iter().any(|&(i, _)| mem.lines[i].lesson == lesson) {
        scored.retain(|&(i, _)| mem.lines[i].lesson == lesson);
    }
    if scored.iter().any(|x| x.1 >= 3) {
        scored.retain(|x| x.1 >= 3);
    }
    scored.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
    scored.truncate(10);
    pred.anchors = scored.len();
    // 3. targets
    let mut targets: BTreeMap<usize, Route> = BTreeMap::new();
    let mut put = |i: usize, r: Route| {
        let e = targets.entry(i).or_insert(r);
        if r < *e {
            *e = r;
        }
    };
    for &(a, _) in &scored {
        let la = mem.lines[a].lesson;
        let near = |j: usize| mem.lines.get(j).filter(|l| l.lesson == la).map(|_| j);
        match tail {
            Some(Tail::Next) => {
                if let Some(j) = near(a + 1) { put(j, Route::Next) }
            }
            _ => put(a, Route::Anchor),
        }
        for j in [a.checked_sub(1).and_then(near), near(a + 1)].into_iter().flatten() {
            put(j, Route::Linked);
        }
    }
    pred.targets = targets.len();
    // 4.–5. support per option
    for (k, o) in options.iter().enumerate() {
        let ow = words(o);
        if ow.is_empty() {
            continue;
        }
        let oneg = negated(o);
        let mut tg: Vec<(usize, Route)> = targets.iter().map(|(&i, &r)| (i, r)).collect();
        let mut own: Vec<(usize, usize)> = mem
            .sharing(&ow)
            .into_iter()
            .filter(|&(i, c)| c >= p.own_min && mem.lines[i].words.iter().any(|w| qw.contains(w)) && !targets.contains_key(&i))
            .collect();
        own.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
        tg.extend(own.into_iter().take(10).map(|(i, _)| (i, Route::Own)));
        for (i, r) in tg {
            let line = &mem.lines[i];
            let current = line.lesson == lesson;
            let f = features(&ow, &qw, oneg, line, r, current);
            let structured = f[0] >= 1 || (r == Route::Own && (f[0] + f[1]) as usize >= p.own_min);
            if !structured {
                continue;
            }
            let s: i64 = f.iter().zip(&p.w).map(|(a, b)| a * b).sum();
            if s < gate {
                continue;
            }
            let sup = Support { score: s, feats: f, current };
            if pred.best[k].as_ref().is_none_or(|b| s > b.score) {
                pred.best[k] = Some(sup.clone());
            }
            if current && pred.recent[k].as_ref().is_none_or(|b| s > b.score) {
                pred.recent[k] = Some(sup);
            }
        }
    }
    // 6. choice
    pred.lesson_first = pred.recent.iter().any(|r| r.is_some());
    let pool = if pred.lesson_first { &pred.recent } else { &pred.best };
    let mut best: Option<(i64, bool, usize)> = None;
    for (k, s) in pool.iter().enumerate() {
        if let Some(s) = s {
            let key = (s.score, s.current);
            if best.is_none_or(|(bs, bc, _)| key > (bs, bc)) {
                best = Some((s.score, s.current, k));
            }
        }
    }
    if let Some((s, _, i)) = best {
        let mut choice = i;
        if !pred.lesson_first {
            let wi = words(&options[i]);
            let extra = |k: usize| pool[k].as_ref().map_or(0, |x| x.feats[2]);
            if let Some((_, j)) = (0..n)
                .filter(|&j| j != i && pool[j].as_ref().is_some_and(|x| x.score == s) && extra(j) < extra(i))
                .filter(|&j| { let w = words(&options[j]); w.len() < wi.len() && w.is_subset(&wi) })
                .map(|j| (extra(j), j))
                .min()
            {
                choice = j;
            }
        }
        pred.choice = choice;
        pred.understood = true;
    }
    // 7. counting
    let ql = question.to_lowercase();
    if (ql.contains("how many") || ql.contains("how much")) && numbers(question).is_empty() {
        let held: BTreeSet<i64> = scored.iter().flat_map(|&(a, _)| mem.lines[a].nums.iter().copied()).collect();
        let hit: Vec<usize> = (0..n).filter(|&k| numbers(&options[k]).first().is_some_and(|v| held.contains(v))).collect();
        if let [k] = hit[..] {
            pred.choice = k;
            pred.understood = true;
            pred.counted = true;
        }
    }
    pred
}

/// One traced test or exam question: the student's choice and understanding are the labels; `right` and `ns` are
/// kept for the reports (correctness of the predicted choice, time).
#[derive(Clone, Debug)]
pub struct Example {
    pub kind: String,
    pub n: usize,
    pub question: String,
    pub options: Vec<String>,
    pub choice: usize,
    pub understood: bool,
    pub right: usize,
    pub ns: f64,
}

/// A lesson: its teaching lines (read first) and its traced questions (asked after the exercises).
#[derive(Clone, Debug)]
pub struct LessonData {
    pub n: usize,
    pub lines: Vec<String>,
    pub examples: Vec<Example>,
}

#[derive(Clone, Debug)]
pub struct CourseData {
    pub name: String,
    pub lessons: Vec<LessonData>,
}

/// Read a course lesson by lesson and predict every question in the state the student answered it in.
pub fn replay(c: &CourseData, p: &Params, mut seen: impl FnMut(&Memory, &LessonData, &Example, &Prediction)) {
    let mut mem = Memory::default();
    for l in &c.lessons {
        for t in &l.lines {
            mem.read_line(l.n, t);
        }
        for ex in &l.examples {
            let pr = predict(&mem, l.n, &ex.question, &ex.options, p);
            seen(&mem, l, ex, &pr);
        }
    }
}

/// Fit the support weights to the student's choices with an averaged perceptron (Collins 2002), courses and lessons
/// in reading order, `epochs` passes, on the lessons `train` admits; only questions the student understood and where
/// both its choice and the predicted one have support teach (the student's own update rule, aimed at its choices
/// instead of the right answers). Integer weights; the average is rounded to the nearest integer.
pub fn fit(courses: &[CourseData], train: impl Fn(&CourseData, &LessonData) -> bool, start: &Params, epochs: usize) -> Params {
    let mut w = start.w;
    let mut sum = [0i64; NF];
    let mut steps = 0i64;
    for _ in 0..epochs {
        for c in courses {
            let mut mem = Memory::default();
            for l in &c.lessons {
                for t in &l.lines {
                    mem.read_line(l.n, t);
                }
                if !train(c, l) {
                    continue;
                }
                for ex in &l.examples {
                    let pr = predict(&mem, l.n, &ex.question, &ex.options, &Params { w, ..start.clone() });
                    if ex.understood && pr.understood && !pr.counted && pr.choice != ex.choice {
                        let pool = if pr.lesson_first { &pr.recent } else { &pr.best };
                        if let (Some(a), Some(b)) = (&pool[ex.choice], &pool[pr.choice]) {
                            for ((x, fa), fb) in w.iter_mut().zip(a.feats).zip(b.feats) {
                                *x += fa - fb;
                            }
                        }
                    }
                    for (s, x) in sum.iter_mut().zip(w) {
                        *s += x;
                    }
                    steps += 1;
                }
            }
        }
    }
    let mut avg = w;
    if steps > 0 {
        for (a, s) in avg.iter_mut().zip(sum) {
            *a = (s as f64 / steps as f64).round() as i64;
        }
    }
    Params { w: avg, ..start.clone() }
}

/// Time model: ln(answer ns) as a linear function of cheap question features (least squares).
pub const NT: usize = 7;

pub fn time_features(mem_lines: usize, question: &str, options: &[String], pr: &Prediction) -> [f64; NT] {
    let qwords = question.split_whitespace().count() as f64;
    let owords: f64 = options.iter().map(|o| o.split_whitespace().count() as f64).sum();
    [1.0, (1.0 + mem_lines as f64).ln(), qwords.ln_1p(), owords.ln_1p(), (pr.anchors as f64).ln_1p(), (pr.targets as f64).ln_1p(), pr.understood as u8 as f64]
}

#[derive(Clone, Debug)]
pub struct TimeModel {
    pub beta: [f64; NT],
}

impl TimeModel {
    /// Ordinary least squares on ln(ns) with a small ridge for stability (Gaussian elimination, partial pivoting).
    pub fn fit(x: &[[f64; NT]], ns: &[f64]) -> TimeModel {
        let mut a = [[0.0f64; NT]; NT];
        let mut b = [0.0f64; NT];
        for (row, &y) in x.iter().zip(ns) {
            let y = y.max(1.0).ln();
            for i in 0..NT {
                b[i] += row[i] * y;
                for j in 0..NT {
                    a[i][j] += row[i] * row[j];
                }
            }
        }
        for (i, r) in a.iter_mut().enumerate() {
            r[i] += 1e-6;
        }
        for c in 0..NT {
            let p = (c..NT).max_by(|&i, &j| a[i][c].abs().total_cmp(&a[j][c].abs())).unwrap_or(c);
            a.swap(c, p);
            b.swap(c, p);
            let d = a[c][c];
            if d.abs() < 1e-12 {
                continue;
            }
            for r in 0..NT {
                if r != c {
                    let f = a[r][c] / d;
                    for k in 0..NT {
                        a[r][k] -= f * a[c][k];
                    }
                    b[r] -= f * b[c];
                }
            }
        }
        let mut beta = [0.0; NT];
        for i in 0..NT {
            beta[i] = if a[i][i].abs() < 1e-12 { 0.0 } else { b[i] / a[i][i] };
        }
        TimeModel { beta }
    }

    pub fn predict_ns(&self, x: &[f64; NT]) -> f64 {
        x.iter().zip(&self.beta).map(|(a, b)| a * b).sum::<f64>().exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem(lines: &[(usize, &str)]) -> Memory {
        let mut m = Memory::default();
        for (l, t) in lines {
            m.read_line(*l, t);
        }
        m
    }

    #[test]
    fn stems_and_stop_words_follow_the_student() {
        assert_eq!(stem("cookies"), "cooky");
        assert_eq!(stem("received"), "receiv");
        assert_eq!(stem("ate"), "eat");
        assert!(words("What is the dog eating?").contains("dog"));
        assert!(!words("What is the dog eating?").contains("what"));
    }

    #[test]
    fn supported_option_wins_and_nothing_read_guesses_first() {
        let m = mem(&[(1, "Bees make honey from nectar."), (1, "Spiders spin webs.")]);
        let opts: Vec<String> = ["webs", "honey", "milk", "silk"].iter().map(|s| s.to_string()).collect();
        let p = predict(&m, 1, "What do bees make from nectar?", &opts, &Params::default());
        assert!(p.understood);
        assert_eq!(p.choice, 1);
        let p = predict(&m, 1, "Where do penguins live?", &opts, &Params::default());
        assert!(!p.understood);
        assert_eq!(p.choice, 0);
    }

    #[test]
    fn the_support_gate_turns_support_into_a_guess() {
        let m = mem(&[(1, "Bees make honey from nectar.")]);
        let opts: Vec<String> = ["webs", "honey"].iter().map(|s| s.to_string()).collect();
        let open = Params::default();
        assert_eq!(predict(&m, 1, "What do bees make from nectar?", &opts, &open).choice, 1);
        let shut = Params { min_score: i64::MAX, ..Params::default() };
        let p = predict(&m, 1, "What do bees make from nectar?", &opts, &shut);
        assert!(!p.understood && p.choice == 0);
        // the no-gap gate applies only to a question without a readable gap
        let nogap = Params { min_score_nogap: i64::MAX, ..Params::default() };
        assert!(predict(&m, 1, "What do bees make from nectar?", &opts, &nogap).understood);
        assert!(!predict(&m, 1, "Bees make something from nectar. Pick it.", &opts, &nogap).understood);
    }

    #[test]
    fn current_lesson_comes_first() {
        let m = mem(&[(1, "Bees make honey from nectar."), (2, "Bees make wax from nectar.")]);
        let opts: Vec<String> = ["honey", "wax"].iter().map(|s| s.to_string()).collect();
        assert_eq!(predict(&m, 2, "What do bees make from nectar?", &opts, &Params::default()).choice, 1);
        let first = mem(&[(1, "Bees make honey from nectar.")]);
        assert_eq!(predict(&first, 1, "What do bees make from nectar?", &opts, &Params::default()).choice, 0);
    }

    #[test]
    fn fit_moves_towards_the_students_choice() {
        let ex = Example { kind: "plain".into(), n: 1, question: "What do bees make from nectar?".into(), options: vec!["honey".into(), "wax and honey".into()], choice: 1, understood: true, right: 0, ns: 1.0 };
        let course = |choice: usize| CourseData { name: "t".into(), lessons: vec![LessonData { n: 1, lines: vec!["Bees make honey and wax from nectar.".into()], examples: vec![Example { choice, ..ex.clone() }] }] };
        let start = Params { w: [1, 0, 0, 0, 0, 0, 0, 0, 0], ..Params::default() };
        let mut chosen = Vec::new();
        replay(&course(1), &start, |_, _, _, p| chosen.push(p.choice));
        assert_eq!(chosen, [1], "two asked words beat one");
        let p = fit(&[course(0)], |_, _| true, &start, 3);
        assert_ne!(p.w, start.w, "a disagreement must move the weights");
        let q = fit(&[course(1)], |_, _| true, &start, 3);
        assert_eq!(q.w, start.w, "agreement teaches nothing");
        let none = fit(&[course(0)], |_, _| false, &start, 3);
        assert_eq!(none.w, start.w, "lessons outside the training fold teach nothing");
    }

    #[test]
    fn time_model_recovers_a_planted_law() {
        let x: Vec<[f64; NT]> = (1..60).map(|i| [1.0, (i as f64).ln(), 1.0, 2.0, 0.0, 0.0, (i % 2) as f64]).collect();
        let y: Vec<f64> = x.iter().map(|r| (10.0 + 0.5 * r[1] + 0.3 * r[6]).exp()).collect();
        let t = TimeModel::fit(&x, &y);
        for (r, v) in x.iter().zip(&y) {
            assert!((t.predict_ns(r) / v - 1.0).abs() < 1e-3);
        }
    }
}
