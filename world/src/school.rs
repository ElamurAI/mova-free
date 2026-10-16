//! The school: lessons with a teacher for the small ones.
//!
//! A teacher is Mova projected for one domain and tuned to one lesson: it knows the right answer exactly, explains the
//! lesson in a few sentences with worked examples, gives a hint by the lesson's key words when the student errs, and
//! corrects. The student is the small one: it reads a word problem, chooses what to do with its numbers (add,
//! subtract, multiply, divide) by the words of the problem, and computes exactly. What it knows is a table of weights
//! «word → operation» (integers), its state; it learns from the teacher's corrections (a perceptron step).
//!
//! A lesson: a short explanation, then many exercises with repetitions (an exercise answered wrongly comes back later),
//! then a test on phrasings the exercises never used. Everything is written down (`<hot>/school/<lesson>/`): the text
//! chat of teacher and student (`chat.log`), every exercise with its hints and corrections (`exercises.tsv`), the test
//! before and after (`tests.tsv`), and the change of the student's state (`state-change.md`). Deterministic: a fixed
//! seed, ordered maps, integer weights.
//!
//!   world school list            — the lessons
//!   world school lesson <n>      — teach lesson n to the student (its state carries over to the next lesson)
//!   world school reset           — the student forgets everything (a negative control: tests drop back)

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use anyhow::{Context, Result, bail};

/// The operations of school arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

const OPS: [Op; 4] = [Op::Add, Op::Sub, Op::Mul, Op::Div];

impl Op {
    fn name(self) -> &'static str {
        match self {
            Op::Add => "add",
            Op::Sub => "subtract",
            Op::Mul => "multiply",
            Op::Div => "divide",
        }
    }
    fn sign(self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "−",
            Op::Mul => "×",
            Op::Div => "÷",
        }
    }
    fn apply(self, a: i64, b: i64) -> Option<i64> {
        match self {
            Op::Add => Some(a + b),
            Op::Sub => (a >= b).then_some(a - b),
            Op::Mul => Some(a * b),
            Op::Div => (b != 0 && a % b == 0).then(|| a / b),
        }
    }
}

/// A deterministic random source (xorshift64*).
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }
    fn pick<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        &v[(self.next() % v.len() as u64) as usize]
    }
}

/// A problem: its text, its two numbers, the operation that solves it.
#[derive(Clone, Debug)]
pub struct Problem {
    pub text: String,
    pub a: i64,
    pub b: i64,
    pub op: Op,
}

impl Problem {
    pub fn answer(&self) -> i64 {
        self.op.apply(self.a, self.b).unwrap_or(0)
    }
}

const NAMES: [&str; 8] = ["Tom", "Anna", "Sam", "Lily", "Max", "Mia", "Ben", "Zoe"];
const THINGS: [&str; 10] = ["apples", "pencils", "stickers", "marbles", "books", "cookies", "balloons", "stamps", "shells", "cards"];

/// A template: (operation, phrasing with {n} name, {t} thing, {a} {b} numbers).
type Tmpl = (Op, &'static str);

/// Phrasings the exercises use.
const PRACTICE: [Tmpl; 20] = [
    (Op::Add, "{n} has {a} {t}. {n} gets {b} more {t}. How many {t} does {n} have now?"),
    (Op::Add, "There are {a} {t} on the table and {b} {t} in the box. How many {t} are there in all?"),
    (Op::Add, "{n} found {a} {t} and then found {b} more. How many {t} did {n} find altogether?"),
    (Op::Add, "{n} bought {a} {t} on Monday and {b} {t} on Tuesday. How many {t} did {n} buy in total?"),
    (Op::Sub, "{n} has {a} {t}. {n} gives {b} {t} to a friend. How many {t} are left?"),
    (Op::Sub, "There were {a} {t}. {n} ate {b} of them. How many {t} are left?"),
    (Op::Sub, "{n} had {a} {t} and lost {b}. How many {t} does {n} have now?"),
    (Op::Sub, "{n} has {a} {t}. {n} has {b} fewer {t} than that box. Take {b} away from {a}. How many remain?"),
    (Op::Mul, "There are {a} bags. Each bag has {b} {t}. How many {t} are there?"),
    (Op::Mul, "{n} has {a} boxes with {b} {t} in each box. How many {t} does {n} have?"),
    (Op::Mul, "There are {a} rows of {t} with {b} {t} in every row. How many {t} are there?"),
    (Op::Mul, "{n} buys {a} packs of {t}. A pack holds {b} {t}. How many {t} does {n} buy?"),
    (Op::Div, "{n} has {a} {t} and shares them equally among {b} friends. How many {t} does each friend get?"),
    (Op::Div, "{a} {t} are put into {b} equal groups. How many {t} are in each group?"),
    (Op::Div, "{n} splits {a} {t} equally into {b} boxes. How many {t} go in each box?"),
    (Op::Div, "{a} {t} are shared between {b} children equally. How many does each child get?"),
    (Op::Add, "{n} has {a} {t}. A friend gives {n} {b} more {t}. How many {t} does {n} have now?"),
    (Op::Sub, "{n} has {a} {t} and gives {b} {t} to a friend. How many {t} does {n} have now?"),
    (Op::Sub, "{n} has {a} {t}. Ben has {b} {t}. How many more {t} does {n} have than Ben?"),
    (Op::Sub, "A box has {a} {t} and a bag has {b} {t}. How many fewer {t} are in the bag than in the box?"),
];

/// Phrasings only the tests use (never seen in practice).
const TEST: [Tmpl; 14] = [
    (Op::Add, "{n} picked {a} {t}. Then {n} picked {b} more. What is the total number of {t}?"),
    (Op::Add, "A shelf holds {a} {t} and another shelf holds {b} {t}. How many {t} are on both shelves altogether?"),
    (Op::Add, "{n} had {a} {t}. A friend gave {n} {b} more. How many {t} does {n} have in all?"),
    (Op::Sub, "{n} had {a} {t}. {b} {t} broke. How many {t} are left?"),
    (Op::Sub, "There are {a} {t} in a jar. {n} takes {b} out. How many {t} are left in the jar?"),
    (Op::Sub, "{n} had {a} {t} and spent {b} of them. How many {t} remain?"),
    (Op::Mul, "Each of {a} children has {b} {t}. How many {t} do the children have?"),
    (Op::Mul, "A box has {b} {t}. How many {t} are in {a} such boxes, each with {b}?"),
    (Op::Mul, "There are {a} tables with {b} {t} on each table. How many {t} are there?"),
    (Op::Div, "{a} {t} are shared equally by {b} people. How many {t} does each person get?"),
    (Op::Div, "{n} puts {a} {t} into bags of {b}, equally. Each bag gets the same. How many bags, if each bag gets {b}? Share {a} into {b} equal parts: how many in each?"),
    (Op::Div, "{a} {t} are divided equally into {b} piles. How many {t} are in each pile?"),
    (Op::Sub, "{n} has {a} {t} and Ben has {b} {t}. How many more {t} does {n} have than Ben?"),
    (Op::Sub, "There are {a} {t} on the shelf and {b} {t} on the desk. How many fewer {t} are on the desk than on the shelf?"),
];

/// Does a phrasing belong to a lesson: comparisons («than») only from the lesson on comparing, giving to someone
/// («gives Tom») only from the lesson on who gives to whom, the revision takes everything of its operations.
fn fits(t: &Tmpl, l: &Lesson) -> bool {
    let than = t.1.contains(" than ");
    let give = t.1.contains("gives {n}") || t.1.contains("gives {b} {t} to a friend. How many {t} does {n} have now");
    if !l.ops.contains(&t.0) {
        return false;
    }
    if l.title.starts_with("Revision") {
        return true;
    }
    match l.n {
        8 => than,
        7 => !than,
        _ => !than && !give,
    }
}

/// A lesson: its number, title, the operations it teaches, the explanation, the key words for hints, the numbers.
pub struct Lesson {
    pub n: usize,
    pub title: &'static str,
    pub ops: &'static [Op],
    pub explain: &'static [&'static str],
    pub cues: &'static [(&'static str, Op)],
    pub max: i64,
}

pub fn lessons() -> Vec<Lesson> {
    let mut v = lessons_unsorted();
    // the revision of everything closes the course
    v.sort_by_key(|l| if l.title.starts_with("Revision") { usize::MAX } else { l.n });
    v
}

fn lessons_unsorted() -> Vec<Lesson> {
    vec![
        Lesson {
            n: 1,
            title: "Adding within 20",
            ops: &[Op::Add],
            explain: &["When we put two groups together, we add.", "The words «more», «in all», «altogether» and «in total» tell us to add.", "Example: Tom has 3 apples and gets 4 more; 3 + 4 = 7, so Tom has 7 apples.", "Example: there are 5 books on the table and 2 in the box; 5 + 2 = 7 books in all."],
            cues: &[("more", Op::Add), ("all", Op::Add), ("altogether", Op::Add), ("total", Op::Add), ("gets", Op::Add), ("found", Op::Add)],
            max: 10,
        },
        Lesson {
            n: 2,
            title: "Taking away within 20",
            ops: &[Op::Sub],
            explain: &["When some of a group goes away, we subtract.", "The words «left», «gave», «lost», «ate», «fewer» and «take away» tell us to subtract.", "Example: Anna has 9 stickers and gives 4 to a friend; 9 − 4 = 5 stickers are left.", "We always subtract the smaller number from the bigger one."],
            cues: &[("left", Op::Sub), ("give", Op::Sub), ("lost", Op::Sub), ("ate", Op::Sub), ("fewer", Op::Sub), ("away", Op::Sub), ("remain", Op::Sub)],
            max: 20,
        },
        Lesson {
            n: 3,
            title: "Adding or taking away?",
            ops: &[Op::Add, Op::Sub],
            explain: &["First decide: are things coming together, or going away?", "Coming together means adding; going away means subtracting.", "Example: Sam had 8 marbles and lost 3; going away, so 8 − 3 = 5.", "Example: Sam had 8 marbles and found 3 more; coming together, so 8 + 3 = 11."],
            cues: &[("more", Op::Add), ("all", Op::Add), ("total", Op::Add), ("left", Op::Sub), ("lost", Op::Sub), ("give", Op::Sub)],
            max: 20,
        },
        Lesson {
            n: 4,
            title: "Multiplying as equal groups",
            ops: &[Op::Mul],
            explain: &["When there are several equal groups, we multiply the number of groups by the size of one group.", "The words «each», «every», «in each» and «packs of» tell us about equal groups.", "Example: 3 bags with 4 apples in each bag; 3 × 4 = 12 apples.", "Multiplying is adding the same number again and again: 4 + 4 + 4 = 12."],
            cues: &[("each", Op::Mul), ("every", Op::Mul), ("rows", Op::Mul), ("packs", Op::Mul), ("bags", Op::Mul), ("boxes", Op::Mul)],
            max: 9,
        },
        Lesson {
            n: 5,
            title: "Sharing equally",
            ops: &[Op::Div],
            explain: &["When we share a number of things equally into groups, we divide.", "The words «shared», «equally», «equal groups» and «split» tell us to divide.", "Example: 12 cookies shared equally among 3 friends; 12 ÷ 3 = 4 cookies each.", "Dividing undoes multiplying: 3 × 4 = 12, so 12 ÷ 3 = 4."],
            cues: &[("share", Op::Div), ("equally", Op::Div), ("equal", Op::Div), ("split", Op::Div), ("divided", Op::Div), ("equally+each", Op::Div), ("share+each", Op::Div)],
            max: 9,
        },
        Lesson {
            n: 7,
            title: "Who gives to whom?",
            ops: &[Op::Add, Op::Sub],
            explain: &["Giving can make you have more or less: it depends on who gives to whom.", "If a friend gives you more, you have more: add.", "If you give some away to a friend, you have less: subtract.", "Example: Tom has 4 cards and a friend gives Tom 3 more; 4 + 3 = 7.", "Example: Tom has 4 cards and gives 3 to a friend; 4 − 3 = 1."],
            cues: &[("give+more", Op::Add), ("gave+more", Op::Add), ("give+friend", Op::Sub), ("left", Op::Sub), ("more", Op::Add)],
            max: 20,
        },
        Lesson {
            n: 8,
            title: "How many more? How many fewer?",
            ops: &[Op::Sub],
            explain: &["To find how many more one has than another, we subtract the smaller from the bigger.", "«How many more … than» and «how many fewer … than» both ask for the difference: subtract.", "Example: Anna has 9 shells and Ben has 5; Anna has 9 − 5 = 4 more shells than Ben.", "Here the word «more» does not mean adding: it asks how big the difference is."],
            cues: &[("more+than", Op::Sub), ("fewer+than", Op::Sub), ("many+more", Op::Sub), ("difference", Op::Sub)],
            max: 20,
        },
        Lesson {
            n: 6,
            title: "Revision: which operation?",
            ops: &[Op::Add, Op::Sub, Op::Mul, Op::Div],
            explain: &["Today we practise all four operations together.", "Coming together: add. Going away: subtract.", "Equal groups («each», «every»): multiply. Sharing equally («shared», «equally», «split»): divide.", "Read the whole problem first, then choose."],
            // «each» is no key word here: it belongs to equal groups and to sharing alike
            cues: &[("more", Op::Add), ("all", Op::Add), ("total", Op::Add), ("left", Op::Sub), ("lost", Op::Sub), ("give", Op::Sub), ("every", Op::Mul), ("rows", Op::Mul), ("share", Op::Div), ("equally", Op::Div), ("split", Op::Div), ("divided", Op::Div), ("more+than", Op::Sub)],
            max: 9,
        },
    ]
}

fn make(rng: &mut Rng, t: &Tmpl, max: i64) -> Problem {
    let (a, b) = match t.0 {
        Op::Add => (rng.range(1, max), rng.range(1, max)),
        Op::Sub => {
            let b = rng.range(1, max - 1);
            (rng.range(b + 1, max.max(b + 1)), b)
        }
        Op::Mul => (rng.range(2, max), rng.range(2, max)),
        Op::Div => {
            let (q, b) = (rng.range(2, max), rng.range(2, max));
            (q * b, b)
        }
    };
    // the actor is not the other one named in the phrasing («… than Ben»)
    let n = loop {
        let x = rng.pick(&NAMES);
        if !t.1.contains(x) {
            break x;
        }
    };
    let th = rng.pick(&THINGS);
    let text = t.1.replace("{n}", n).replace("{t}", th).replace("{a}", &a.to_string()).replace("{b}", &b.to_string());
    Problem { text, a, b, op: t.0 }
}

/// A word as the student sees it at school: lower case, a plain plural made singular; only the articles go — «than»,
/// «how», «many» matter here, unlike in the nerve graph.
fn school_norm(w: &str) -> Option<String> {
    let w: String = w.chars().filter(|c| c.is_alphabetic()).flat_map(char::to_lowercase).collect();
    // the words of every question («how many», «does … have», «there are») are no clue to the operation
    if w.is_empty() || matches!(w.as_str(), "a" | "an" | "the" | "how" | "many" | "much" | "what" | "is" | "are" | "was" | "were" | "do" | "does" | "did" | "have" | "has" | "had" | "there" | "then" | "of" | "in" | "on" | "at" | "to" | "and" | "with" | "for" | "it" | "them" | "they" | "their" | "now" | "number") {
        return None;
    }
    // «cookies» → «cookie», «stories» → «story»: the lexicon says which singular is a word
    let w = if w.len() > 4 && w.ends_with("ies") && en::dict::form(&w[..w.len() - 1]).is_some() {
        w[..w.len() - 1].to_string()
    } else if w.len() > 4 && w.ends_with("ies") {
        format!("{}y", &w[..w.len() - 3])
    } else if w.len() > 4 && (w.ends_with("xes") || w.ends_with("ches") || w.ends_with("shes") || w.ends_with("sses")) {
        w[..w.len() - 2].to_string()
    } else if w.len() > 3 && w.ends_with('s') && !w.ends_with("ss") && !w.ends_with("us") && !w.ends_with("is") && w != "does" && w != "has" {
        w[..w.len() - 1].to_string()
    } else {
        w
    };
    Some(w)
}

/// The words of a problem as the student sees them (plain singular words, numbers out), and the pairs of
/// neighbouring words («give+more», «how+many», «more+than»): a phrase can mean what its words alone do not.
fn words(text: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for sentence in text.split(['.', '?', '!']) {
        // names and the things being counted say nothing about what to do with the numbers: the student learns to
        // look past them, as a child does
        let counted = |w: &str| THINGS.iter().chain(TABLE_THINGS.iter().map(|t| &t.0)).any(|t| school_norm(t).as_deref() == Some(w));
        let ws: Vec<String> = sentence.split(|c: char| !c.is_alphabetic()).filter_map(school_norm).filter(|w| !NAMES.iter().any(|n| n.to_lowercase() == *w) && !counted(w)).collect();
        // the words, and every pair of words in one sentence, in order («more…than», «give…more»)
        for (i, w) in ws.iter().enumerate() {
            v.push(w.clone());
            for n in &ws[i + 1..] {
                v.push(format!("{w}+{n}"));
            }
        }
    }
    v.sort();
    v.dedup();
    v
}

/// The student: weights «word → operation» (integers), its whole knowledge of school arithmetic.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Student {
    pub w: BTreeMap<String, [i64; 4]>,
    /// the sum of the weights over every exercise lived through: the student answers by these averaged weights, so one
    /// fresh mistake does not overturn an old lesson (an averaged perceptron)
    pub acc: BTreeMap<String, [i64; 4]>,
}

impl Student {
    fn path() -> std::path::PathBuf {
        crate::state::root().join("school").join("student.tsv")
    }

    pub fn load() -> Student {
        let mut s = Student::default();
        for l in std::fs::read_to_string(Student::path()).unwrap_or_default().lines().filter(|l| !l.starts_with('#')) {
            let c: Vec<&str> = l.split('\t').collect();
            let n = |i: usize| c.get(i).and_then(|x| x.parse().ok()).unwrap_or(0);
            if c.len() >= 5 {
                s.w.insert(c[0].to_string(), [n(1), n(2), n(3), n(4)]);
            }
            if c.len() >= 9 {
                s.acc.insert(c[0].to_string(), [n(5), n(6), n(7), n(8)]);
            }
        }
        s
    }

    pub fn save(&self) -> Result<()> {
        let mut t = String::from("# the student's weights: word, add, subtract, multiply, divide, then the summed weights (the same four)\n");
        for (k, v) in &self.w {
            let a = self.acc.get(k).copied().unwrap_or_default();
            if v.iter().chain(a.iter()).any(|&x| x != 0) {
                writeln!(t, "{k}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", v[0], v[1], v[2], v[3], a[0], a[1], a[2], a[3])?;
            }
        }
        std::fs::create_dir_all(Student::path().parent().unwrap())?;
        crate::store::write_atomic(&Student::path(), t.as_bytes())
    }

    /// The operation chosen for a problem and why (the words that weighed most), among the operations allowed.
    fn choose(&self, p: &Problem, allowed: &[Op]) -> (Op, Vec<String>) {
        let ws = words(&p.text);
        // the averaged weights once there are any, else the current ones
        let table = if self.acc.is_empty() { &self.w } else { &self.acc };
        let score = |o: Op| ws.iter().map(|w| table.get(w).map(|v| v[o as usize]).unwrap_or(0)).sum::<i64>();
        // a tie (or nothing known) goes to the first allowed operation: a beginner adds
        let best = allowed.iter().copied().max_by(|a, b| score(*a).cmp(&score(*b)).then((*b as u8).cmp(&(*a as u8)))).unwrap_or(Op::Add);
        let mut why: Vec<(i64, String)> = ws.iter().filter_map(|w| table.get(w).map(|v| (v[best as usize], w.clone()))).filter(|x| x.0 > 0).collect();
        why.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        (best, why.into_iter().take(3).map(|x| x.1).collect())
    }

    fn answer(&self, p: &Problem, allowed: &[Op]) -> (Option<i64>, Op, Vec<String>) {
        let (op, why) = self.choose(p, allowed);
        (op.apply(p.a, p.b), op, why)
    }

    /// A correction from the teacher: towards the right operation, away from the chosen one, on the problem's words.
    /// One exercise lived through: the current weights are added to the sums.
    fn tick(&mut self) {
        for (k, v) in &self.w {
            let a = self.acc.entry(k.clone()).or_default();
            for i in 0..4 {
                a[i] += v[i];
            }
        }
    }

    fn learn(&mut self, p: &Problem, chosen: Op, hinted: Option<&str>) {
        if chosen == p.op {
            self.tick();
            return;
        }
        // credit goes where the teacher pointed: when the hint names a phrase («more … than»), its single words are
        // not to blame — the phrase is
        let parts: Vec<&str> = hinted.filter(|h| h.contains('+')).map(|h| h.split('+').collect()).unwrap_or_default();
        for w in words(&p.text) {
            if parts.contains(&w.as_str()) {
                continue;
            }
            let e = self.w.entry(w).or_default();
            e[p.op as usize] += 1;
            e[chosen as usize] -= 1;
        }
        // the word the teacher pointed at weighs more: the student listens to the hint
        if let Some(h) = hinted {
            let e = self.w.entry(h.to_string()).or_default();
            e[p.op as usize] += 3;
        }
        self.tick();
    }
}

/// The teacher: Mova projected for school arithmetic and tuned to one lesson — it knows the answer, and hints by the
/// lesson's key words.
struct Teacher<'a> {
    lesson: &'a Lesson,
    /// the lessons before this one: their key words, for the revision
    earlier: Vec<&'a Lesson>,
}

impl Teacher<'_> {
    /// The hint and the word it points at (from the program of this lesson and the lessons before).
    fn hint(&self, p: &Problem) -> (String, Option<&'static str>) {
        let ws = words(&p.text);
        let cue = self.lesson.cues.iter().chain(self.earlier.iter().flat_map(|l| l.cues.iter())).find(|(w, o)| *o == p.op && ws.iter().any(|x| x == w));
        match cue {
            Some((w, o)) => (format!("Look at the word «{w}»: it tells us to {}.", o.name()), Some(w)),
            None => (format!("Think: what happens to the {}? Here we {}.", if p.op == Op::Add || p.op == Op::Mul { "groups" } else { "things" }, p.op.name()), None),
        }
    }
    fn correct(&self, p: &Problem) -> String {
        format!("The answer is {} {} {} = {}.", p.a, p.op.sign(), p.b, p.answer())
    }
}

/// The test of a lesson: problems in phrasings the exercises never used; (right, of, the student's answers).
fn test(student: &Student, lesson: &Lesson, seed: u64, n: usize) -> (usize, usize, Vec<(Problem, Option<i64>, Op)>) {
    let mut rng = Rng(seed);
    let tmpls: Vec<&Tmpl> = TEST.iter().filter(|t| fits(t, lesson)).collect();
    let mut out = Vec::new();
    let mut right = 0;
    for i in 0..n {
        let p = make(&mut rng, tmpls[i % tmpls.len()], lesson.max);
        let (a, op, _) = student.answer(&p, &OPS);
        if a == Some(p.answer()) {
            right += 1;
        }
        out.push((p, a, op));
    }
    (right, n, out)
}

/// Teach one lesson: explanation, exercises with repetitions, the test before and after, everything written down.
pub fn teach(n: usize) -> Result<String> {
    let all = lessons();
    let lesson = all.iter().find(|l| l.n == n).with_context(|| format!("no lesson {n}"))?;
    let earlier: Vec<&Lesson> = all.iter().filter(|l| l.n < n).collect();
    let teacher = Teacher { lesson, earlier: earlier.clone() };
    let dir = crate::state::root().join("school").join(format!("lesson-{n:02}"));
    std::fs::create_dir_all(&dir)?;
    let mut student = Student::load();
    let before = student.clone();
    let seed = 0x5EED_0000 + n as u64;
    let (b_right, b_of, _) = test(&student, lesson, seed ^ 0xA11, 60);
    let mut chat = String::new();
    let mut ex = String::from("# round\texercise\tproblem\tstudent's operation\tstudent's answer\tright answer\thint\tcorrection\n");
    let say = |chat: &mut String, who: &str, t: &str| {
        let _ = writeln!(chat, "{who}: {t}");
    };
    say(&mut chat, "Teacher", &format!("Today's lesson: {}.", lesson.title));
    for l in lesson.explain {
        say(&mut chat, "Teacher", l);
    }
    // the student listens: each key word the lesson names gets a first weight for its operation
    for (w, o) in lesson.cues {
        let e = student.w.entry(w.to_string()).or_default();
        if e[*o as usize] < 2 {
            e[*o as usize] += 2;
        }
    }
    say(&mut chat, "Student", &format!("I am listening. I will remember: {}.", lesson.cues.iter().map(|(w, o)| format!("«{}» — {}", w.replace('+', " … "), o.name())).collect::<Vec<_>>().join(", ")));
    // exercises: rounds of new problems, and every problem answered wrongly comes back in the next round
    let mut rng = Rng(seed);
    let tmpls: Vec<&Tmpl> = PRACTICE.iter().filter(|t| fits(t, lesson)).collect();
    // revision: the operations of the lessons before come back in a quarter of the exercises
    let old_ops: Vec<Op> = earlier.iter().flat_map(|l| l.ops.iter().copied()).filter(|o| !lesson.ops.contains(o)).collect();
    let old_tmpls: Vec<&Tmpl> = PRACTICE.iter().filter(|t| old_ops.contains(&t.0)).collect();
    if !old_tmpls.is_empty() {
        say(&mut chat, "Teacher", &format!("First, a little revision of what we learned before: {}.", earlier.iter().map(|l| l.title).collect::<Vec<_>>().join("; ")));
    }
    let mut again: Vec<Problem> = Vec::new();
    let (mut done, mut first_right) = (0, 0);
    for round in 1..=8 {
        let mut todo: Vec<Problem> = std::mem::take(&mut again);
        for i in 0..20 {
            if !old_tmpls.is_empty() && i % 4 == 3 {
                let t = old_tmpls[(round * 20 + i) % old_tmpls.len()];
                let max = earlier.iter().find(|l| l.ops.contains(&t.0)).map(|l| l.max).unwrap_or(lesson.max);
                todo.push(make(&mut rng, t, max));
            } else {
                todo.push(make(&mut rng, tmpls[(round * 20 + i) % tmpls.len()], lesson.max));
            }
        }
        let mut right = 0;
        for (i, p) in todo.iter().enumerate() {
            // the student chooses among all operations, as a real pupil does: it can be wrong, so it can learn
            let (a, op, why) = student.answer(p, &OPS);
            done += 1;
            say(&mut chat, "Teacher", &p.text);
            let ans = a.map(|x| x.to_string()).unwrap_or_else(|| "I cannot do that".into());
            say(&mut chat, "Student", &format!("I {} : {ans}{}", op.name(), if why.is_empty() { String::new() } else { format!(" (because of «{}»)", why.join("», «")) }));
            if a == Some(p.answer()) {
                right += 1;
                student.tick();
                say(&mut chat, "Teacher", "Right!");
                writeln!(ex, "{round}\t{i}\t{}\t{}\t{ans}\t{}\t\t", p.text, op.name(), p.answer())?;
            } else {
                let ((h, cue), c) = (teacher.hint(p), teacher.correct(p));
                say(&mut chat, "Teacher", &format!("Not quite. {h} {c}"));
                student.learn(p, op, cue);
                again.push(p.clone());
                writeln!(ex, "{round}\t{i}\t{}\t{}\t{ans}\t{}\t{h}\t{c}", p.text, op.name(), p.answer())?;
            }
        }
        if round == 1 {
            first_right = right;
        }
        say(&mut chat, "Teacher", &format!("Round {round}: {right} of {} right.", todo.len()));
    }
    let (a_right, a_of, after_items) = test(&student, lesson, seed ^ 0xA11, 60);
    say(&mut chat, "Teacher", &format!("Test: {a_right} of {a_of} right (before the lesson: {b_right} of {b_of})."));
    let mut tests = String::from("# problem\tright answer\tstudent's operation\tstudent's answer\tright?\n");
    for (p, a, op) in &after_items {
        writeln!(tests, "{}\t{}\t{}\t{}\t{}", p.text, p.answer(), op.name(), a.map(|x| x.to_string()).unwrap_or("-".into()), a == &Some(p.answer()))?;
    }
    // the change of the student's state, for debugging
    let mut change = format!("# Lesson {n}: {} — the change of the student's state\n\nTest before {b_right}/{b_of}, after {a_right}/{a_of}. Exercises {done}, the first round {first_right}/20.\n\n| word | before (add, sub, mul, div) | after |\n|---|---|---|\n", lesson.title);
    let keys: std::collections::BTreeSet<&String> = before.w.keys().chain(student.w.keys()).collect();
    let mut changed = 0;
    for k in keys {
        let (x, y) = (before.w.get(k).copied().unwrap_or_default(), student.w.get(k).copied().unwrap_or_default());
        if x != y {
            changed += 1;
            writeln!(change, "| {k} | {x:?} | {y:?} |")?;
        }
    }
    writeln!(change, "\n{changed} words changed their weights.")?;
    std::fs::write(dir.join("chat.log"), &chat)?;
    std::fs::write(dir.join("exercises.tsv"), &ex)?;
    std::fs::write(dir.join("tests.tsv"), &tests)?;
    std::fs::write(dir.join("state-change.md"), &change)?;
    student.save()?;
    let summary = format!("Lesson {n} «{}»: test {b_right}/{b_of} → {a_right}/{a_of}; {done} exercises (first round {first_right}/20); {changed} words changed; logs in {}", lesson.title, dir.display());
    crate::triggers::activate("training", 0.0, &summary);
    Ok(summary)
}

// ---- The classroom: a table and a board ---------------------------------------------------------------------------
//
// The teacher puts things on the table and takes them off, says so in English, and writes the sums on the board. The
// student reads every sentence itself and keeps its own table and its own journal of what happened. It answers what
// it sees, how many there are, how many of a kind (an apple is a fruit — from its worlds), what would be if one more
// came, and what would be if, back then, something else had happened (it replays its journal with that event
// changed). At first it does not know what a verb does to the table; the teacher's corrections teach it.

/// Things on the table and their kinds (the kind is what the student must find in its own worlds).
const TABLE_THINGS: [(&str, &str); 8] = [("apple", "fruit"), ("pear", "fruit"), ("banana", "fruit"), ("plum", "fruit"), ("pencil", "tool"), ("book", "book"), ("cup", "cup"), ("spoon", "spoon")];
const PUT_VERBS: [&str; 3] = ["put", "place", "add"];
const TAKE_VERBS: [&str; 3] = ["take", "remove", "eat"];
const NUMBER: [&str; 6] = ["one", "two", "three", "four", "five", "six"];

/// One event: a verb, how many, of what.
#[derive(Clone, Debug, PartialEq)]
struct Event {
    verb: String,
    n: i64,
    thing: String,
}

fn plural(t: &str, n: i64) -> String {
    if n == 1 { t.to_string() } else { format!("{t}s") }
}

fn say_event(e: &Event) -> String {
    let put = PUT_VERBS.contains(&e.verb.as_str());
    let prep = if put { "on" } else { "off" };
    let verb = if e.verb == "eat" { "eat".to_string() } else { e.verb.clone() };
    let tail = if e.verb == "eat" { " from the table".to_string() } else { format!(" {prep} the table") };
    format!("I {verb} {} {}{tail}.", NUMBER[(e.n - 1) as usize], plural(&e.thing, e.n))
}

/// The student's reading of the teacher's sentence: the verb, the number, the thing.
fn read_event(sentence: &str) -> Option<Event> {
    let ws: Vec<String> = sentence.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect();
    let i = ws.iter().position(|w| NUMBER.contains(&w.as_str()) || w.parse::<i64>().is_ok())?;
    let n = crate::triggers::number_word(&ws[i]).map(|x| x as i64).or_else(|| ws[i].parse().ok())?;
    let thing = crate::nerves::norm(ws.get(i + 1)?)?;
    let verb = ws.get(i.checked_sub(1)?)?.clone();
    Some(Event { verb, n, thing })
}

/// The table after a journal, by what the student believes each verb does (+1 adds, −1 takes away, 0 unknown).
fn replay(journal: &[Event], effect: &BTreeMap<String, i64>) -> BTreeMap<String, i64> {
    let mut t: BTreeMap<String, i64> = BTreeMap::new();
    for e in journal {
        let s = effect.get(&e.verb).copied().unwrap_or(0).signum();
        *t.entry(e.thing.clone()).or_default() += s * e.n;
    }
    t.retain(|_, n| *n > 0);
    t
}

fn truth(journal: &[Event]) -> BTreeMap<String, i64> {
    let mut t: BTreeMap<String, i64> = BTreeMap::new();
    for e in journal {
        *t.entry(e.thing.clone()).or_default() += if PUT_VERBS.contains(&e.verb.as_str()) { e.n } else { -e.n };
    }
    t.retain(|_, n| *n > 0);
    t
}

fn describe(t: &BTreeMap<String, i64>) -> String {
    if t.is_empty() {
        return "nothing".into();
    }
    t.iter().map(|(k, n)| format!("{} {}", NUMBER.get((*n - 1) as usize).map(|x| x.to_string()).unwrap_or(n.to_string()), plural(k, *n))).collect::<Vec<_>>().join(" and ")
}

/// The kind of a thing, as the student finds it in its own worlds (an «is a» fact), if it can.
fn kind_in_worlds(thing: &str) -> Option<String> {
    let k = crate::know::knowledge();
    let fs = k.about(thing);
    let kinds: Vec<&str> = TABLE_THINGS.iter().map(|x| x.1).collect();
    fs.iter().filter(|f| f.link == crate::know::Link::IsA && !f.negated).map(|f| f.obj.split_whitespace().last().unwrap_or("").to_string()).find(|o| kinds.contains(&o.as_str()))
}

/// The table lesson: scenes of putting and taking, questions, the counterfactual, the student learning the verbs.
pub fn table_lesson() -> Result<String> {
    let dir = crate::state::root().join("school").join("lesson-00-table");
    std::fs::create_dir_all(&dir)?;
    let effect_path = crate::state::root().join("school").join("verbs.tsv");
    let mut effect: BTreeMap<String, i64> = std::fs::read_to_string(&effect_path).unwrap_or_default().lines().filter_map(|l| l.split_once('\t').and_then(|(v, n)| n.parse().ok().map(|n| (v.to_string(), n)))).collect();
    let before = effect.clone();
    let mut chat = String::new();
    let mut log = String::from("# scene\tquestion\tstudent\tright\tcorrect?\n");
    let say = |chat: &mut String, who: &str, t: &str| {
        let _ = writeln!(chat, "{who}: {t}");
    };
    say(&mut chat, "Teacher", "Today we have a table and a board. I will put things on the table and take them off. Watch, count, and imagine.");
    let mut rng = Rng(0x7AB1E);
    let mut kinds_cache: BTreeMap<String, Option<String>> = BTreeMap::new();
    let (mut asked, mut right) = (0, 0);
    let mut per_scene: Vec<usize> = Vec::new();
    for scene in 1..=40 {
        say(&mut chat, "Teacher", &format!("Scene {scene}. The table is empty."));
        let mut journal: Vec<Event> = Vec::new();
        let mut true_journal: Vec<Event> = Vec::new();
        let mut scene_right = 0;
        let steps = 2 + (scene % 3);
        for _ in 0..steps {
            let have = truth(&true_journal);
            let take = !have.is_empty() && rng.next() % 3 == 0;
            let e = if take {
                let (thing, n) = have.iter().nth((rng.next() % have.len() as u64) as usize).map(|(k, n)| (k.clone(), *n)).unwrap();
                // one eats fruit, not spoons: the teacher's program keeps to the possible
                let fruit = TABLE_THINGS.iter().any(|(t, k)| *t == thing && *k == "fruit");
                let verb = loop {
                    let v = *rng.pick(&TAKE_VERBS);
                    if v != "eat" || fruit {
                        break v;
                    }
                };
                Event { verb: verb.to_string(), n: rng.range(1, n), thing }
            } else {
                Event { verb: rng.pick(&PUT_VERBS).to_string(), n: rng.range(1, 3), thing: rng.pick(&TABLE_THINGS).0.to_string() }
            };
            let sentence = say_event(&e);
            say(&mut chat, "Teacher", &sentence);
            true_journal.push(e);
            if let Some(read) = read_event(&sentence) {
                journal.push(read);
            }
        }
        let t = truth(&true_journal);
        // what does the student see?
        let mut ask = |chat: &mut String, log: &mut String, q: &str, said: String, right_answer: String| -> bool {
            say(chat, "Teacher", q);
            say(chat, "Student", &said);
            let ok = said == right_answer;
            say(chat, "Teacher", &if ok { "Right!".to_string() } else { format!("Not quite: {right_answer}.") });
            let _ = writeln!(log, "{scene}\t{q}\t{said}\t{right_answer}\t{ok}");
            ok
        };
        let mine = replay(&journal, &effect);
        let ok1 = ask(&mut chat, &mut log, "What do you see on the table?", describe(&mine), describe(&t));
        let thing = t.keys().next().cloned().unwrap_or_else(|| "apple".into());
        let ok2 = ask(&mut chat, &mut log, &format!("How many {}s are on the table?", thing), mine.get(&thing).copied().unwrap_or(0).to_string(), t.get(&thing).copied().unwrap_or(0).to_string());
        // a kind the student must look up in its worlds
        let kind_of = |x: &str, cache: &mut BTreeMap<String, Option<String>>| cache.entry(x.to_string()).or_insert_with(|| kind_in_worlds(x)).clone();
        let fruit_true: i64 = t.iter().filter(|(k, _)| TABLE_THINGS.iter().any(|(n, c)| n == k && *c == "fruit")).map(|x| x.1).sum();
        let fruit_mine: i64 = mine.iter().filter(|(k, _)| kind_of(k, &mut kinds_cache).as_deref() == Some("fruit")).map(|x| x.1).sum();
        let ok3 = ask(&mut chat, &mut log, "How many fruits are on the table?", fruit_mine.to_string(), fruit_true.to_string());
        // one more
        let more = Event { verb: "put".into(), n: 1, thing: thing.clone() };
        let mut j2 = journal.clone();
        j2.push(more.clone());
        let mut t2j = true_journal.clone();
        t2j.push(more);
        let ok4 = ask(&mut chat, &mut log, &format!("If I put one more {thing} on the table, how many {thing}s will there be?"), replay(&j2, &effect).get(&thing).copied().unwrap_or(0).to_string(), truth(&t2j).get(&thing).copied().unwrap_or(0).to_string());
        // the counterfactual: back then, something else
        let alt = Event { verb: "put".into(), n: 2, thing: if true_journal[0].thing == "pear" { "plum".into() } else { "pear".into() } };
        let mut jc = journal.clone();
        jc[0] = alt.clone();
        let mut tc = true_journal.clone();
        tc[0] = alt.clone();
        let q = format!("What if at the start I had put two {}s instead of what I put? What would be on the table now?", alt.thing);
        let ok5 = ask(&mut chat, &mut log, &q, describe(&replay(&jc, &effect)), describe(&truth(&tc)));
        let oks = [ok1, ok2, ok3, ok4, ok5];
        scene_right = oks.iter().filter(|x| **x).count();
        asked += oks.len();
        right += scene_right;
        per_scene.push(scene_right);
        // the board, and the lesson of the scene: what each verb did
        say(&mut chat, "Teacher", &format!("On the board: {}.", t.iter().map(|(k, n)| format!("{} = {n}", plural(k, 2))).collect::<Vec<_>>().join(", ")));
        let pred = replay(&journal, &effect);
        for e in &journal {
            let want = if PUT_VERBS.contains(&e.verb.as_str()) { 1 } else { -1 };
            if pred.get(&e.thing).copied().unwrap_or(0) != t.get(&e.thing).copied().unwrap_or(0) || effect.get(&e.verb).copied().unwrap_or(0).signum() != want {
                *effect.entry(e.verb.clone()).or_default() += want;
            }
        }
    }
    let mut verbs = String::new();
    for (v, n) in &effect {
        writeln!(verbs, "{v}\t{n}")?;
    }
    std::fs::write(&effect_path, &verbs)?;
    let mut change = format!("# Lesson 0: the table — the change of the student's state\n\n{right} of {asked} answers right. By scenes of five questions: first ten {}, last ten {}.\n\n| verb | before | after |\n|---|---|---|\n", per_scene.iter().take(10).sum::<usize>(), per_scene.iter().rev().take(10).sum::<usize>());
    for v in PUT_VERBS.iter().chain(TAKE_VERBS.iter()) {
        writeln!(change, "| {v} | {} | {} |", before.get(*v).copied().unwrap_or(0), effect.get(*v).copied().unwrap_or(0))?;
    }
    std::fs::write(dir.join("chat.log"), &chat)?;
    std::fs::write(dir.join("answers.tsv"), &log)?;
    std::fs::write(dir.join("state-change.md"), &change)?;
    Ok(format!("Lesson 0 «The table»: {right}/{asked} right; first ten scenes {}/50, last ten {}/50; logs in {}", per_scene.iter().take(10).sum::<usize>(), per_scene.iter().rev().take(10).sum::<usize>(), dir.display()))
}

// ---- The student of the second generation: actors as vectors ------------------------------------------------------
//
// A problem is a little world: every actor (and every place) holds a vector of things. A sentence is read along its
// tree: who acts (the subject), who receives (the indirect object, or «to X»), how many of what. What a verb does is
// a pair — to the subject's vector and to the receiver's (give: −, +; find: +, 0) — and that pair is what the student
// learns from the teacher's corrections. Helpers are read together with their context: «Ben does the same» repeats
// the last action with Ben acting. The question asks about one vector, the difference of two, the sum of all, or an
// equal share.

/// A verb's effect: (on the subject, on the receiver), each −1, 0 or +1.
type Effect = (i64, i64);

/// What the student believes each verb does, by counts of the corrections that showed it (the most counted wins).
#[derive(Clone, Default)]
pub struct Actors {
    pub votes: BTreeMap<String, BTreeMap<(i64, i64), i64>>,
}

const EFFECTS: [Effect; 6] = [(1, 0), (-1, 0), (-1, 1), (1, -1), (0, 1), (0, 0)];

/// A side of an effect the evidence did not show.
const NONE: i64 = 9;

impl Actors {
    fn path() -> std::path::PathBuf {
        crate::state::root().join("school").join("actors.tsv")
    }
    pub fn load() -> Actors {
        let mut a = Actors::default();
        for l in std::fs::read_to_string(Actors::path()).unwrap_or_default().lines().filter(|l| !l.starts_with('#')) {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() == 4 {
                *a.votes.entry(c[0].to_string()).or_default().entry((c[1].parse().unwrap_or(0), c[2].parse().unwrap_or(0))).or_default() += c[3].parse::<i64>().unwrap_or(0);
            }
        }
        a
    }
    pub fn save(&self) -> Result<()> {
        let mut t = String::from("# what a verb does: verb, to the subject, to the receiver, votes\n");
        for (v, m) in &self.votes {
            for ((a, b), n) in m {
                writeln!(t, "{v}\t{a}\t{b}\t{n}")?;
            }
        }
        std::fs::create_dir_all(Actors::path().parent().unwrap())?;
        crate::store::write_atomic(&Actors::path(), t.as_bytes())
    }
    /// The effect on the subject and on the receiver, each by its own votes: a vote (a, b) counts for both sides;
    /// a vote (a, NONE) only for the subject, (NONE, b) only for the receiver (what an answer showed of one side).
    fn effect(&self, verb: &str) -> Option<Effect> {
        let m = self.votes.get(verb)?;
        let side = |pick: &dyn Fn(&Effect) -> i64| -> Option<i64> {
            let mut t: BTreeMap<i64, i64> = BTreeMap::new();
            for (e, n) in m {
                let x = pick(e);
                if x != NONE {
                    *t.entry(x).or_default() += n;
                }
            }
            t.into_iter().max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0))).map(|x| x.0)
        };
        let a = side(&|e| e.0)?;
        Some((a, side(&|e| e.1).unwrap_or(0)))
    }
}

/// One read event: a verb, who acts, who receives, how many of what.
#[derive(Clone, Debug, Default)]
struct Ev {
    verb: String,
    subj: String,
    to: String,
    n: i64,
    thing: String,
    /// «N more T»: the actor has more after it
    more: bool,
    /// «half of his», «the rest», «doubles»: a part of what the actor holds when it happens (instead of n)
    frac: Option<(i64, i64)>,
    /// «keeps 3 and gives the rest»: what the actor keeps out of «the rest»
    keep: i64,
    /// «found some more», «lost a few of them»: how many is what is asked
    unknown: bool,
}

/// What a problem says, read along the trees: the starting holdings, the events, groups («N bags, each with M»), and
/// the question.
#[derive(Clone, Debug, Default)]
struct Reading {
    holds: Vec<(String, String, i64)>,
    events: Vec<Ev>,
    groups: Vec<(i64, i64)>,
    share: Option<(i64, i64)>,
    ask_whose: Option<String>,
    ask_than: Option<String>,
    /// «how many do Max and Tom have together»: the actors summed
    ask_some: Vec<String>,
    /// prices: thing → cents for one
    prices: BTreeMap<String, i64>,
    /// «how many cents does Tom pay / spend»
    ask_spent: bool,
    /// «reads 3 pages every day»: so many each day; the question names the days
    rate: Option<i64>,
    days: Option<i64>,
    /// «26 eggs are packed in boxes of 6»: (things, one box); asked for the full boxes or what is left over
    pack: Option<(i64, i64)>,
    ask_full: bool,
    ask_over: bool,
    /// «… shares the rest equally among 3 friends»: the actor's rest, divided
    share_rest: Option<(String, i64)>,
    /// a line: how many in it, the place from the front and from the back; asked: in front, behind, how many in all
    line: Option<i64>,
    front: Option<i64>,
    back: Option<i64>,
    ask_front: bool,
    ask_behind: bool,
    /// a span of days on a calendar, both ends counted
    span: Option<(i64, i64)>,
    /// a clock: the hour it starts and how many hours it lasts; asked for the hour, or for minutes in hours
    clock: Option<(i64, i64)>,
    ask_minutes: Option<i64>,
    /// «in 5 years», «3 years ago»: the question moves the time
    shift: i64,
    /// «Tom had some apples»: the start unknown; «now he has 5»: the end told
    unknown_start: Option<String>,
    now: Option<(String, i64)>,
    /// the kind of thing the question asks about, and the kind it compares with («more wrappers than bottle caps»)
    ask_thing: Option<String>,
    than_thing: Option<String>,
    /// the numbers of the story that found a place in the reading
    used: BTreeSet<i64>,
    /// «keeps 3»: held back from «the rest» that follows
    kept: i64,
    /// relations waiting for a count: (a, b, thing, p, q, d) — a = b × p / q + d
    rels: Vec<(String, String, String, i64, i64, i64)>,
    /// a number the question itself holds and nothing else does («How many socks are in 12 socks?»)
    q_num: Option<i64>,
    ask_all: bool,
    ask_each: bool,
    /// the question asks what is left: what the actors lost is gone
    ask_left: bool,
}

/// The words the skeleton reads by itself (each, every, equally, than, left, in all, the same, to …): a registry, each
/// one can be switched off to measure its role (an ablation), so words are bound into the skeleton carefully.
pub const SKELETON_WORDS: [&str; 56] = ["relations", "sums", "ops", "mentions", "things", "names", "line", "calendar", "clock", "years", "now", "some", "measure", "dozen pair", "numbers", "cost", "week", "full", "left over", "pack", "third", "he she", "as many", "twice", "half", "rest", "double", "keep", "each of", "together", "between", "combined", "more", "dictionary", "each", "every", "equally", "equal", "than", "left", "remain", "all", "altogether", "total", "both", "there", "such", "each with", "the same", "so did", "too", "to", "from", "by", "and", "then"];

static OFF: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// Is a skeleton word switched on (all are, except the one an ablation switches off)?
fn on(w: &str) -> bool {
    OFF.lock().map(|o| o.as_deref() != Some(w.trim())).unwrap_or(true)
}

/// After a sentence or clause: if it changed the reading, its numbers found their place.
fn settle(r: &Reading, fp: &mut String, pending: &mut Vec<i64>, used: &mut BTreeSet<i64>) {
    let now = format!("{r:?}");
    if now != *fp {
        used.extend(pending.iter().copied());
        *fp = now;
    }
    pending.clear();
}

/// «He», «she»: the last man or woman named (the names' sex is in the skeleton — a name is a word of the dictionary
/// whose sense the small one does not derive from the problems).
fn resolve_pronouns(text: &str) -> String {
    if !on("he she") {
        return text.to_string();
    }
    // the last one named of each sex, and the last subject of each sex (a name opening a sentence): the subject is
    // preferred — «Ben had some cards. He gave 8 to Tom. Now he has 1» is Ben both times
    let (mut man, mut woman, mut man_subj, mut woman_subj) = (String::new(), String::new(), String::new(), String::new());
    let mut out = String::new();
    let mut opens = true;
    for tok in text.split_inclusive(' ') {
        let w: String = tok.chars().filter(|c| c.is_alphanumeric()).collect();
        let rest: String = tok.chars().skip_while(|c| c.is_alphanumeric()).collect();
        let mut said = w.clone();
        match w.to_lowercase().as_str() {
            "he" if !man.is_empty() => said = if man_subj.is_empty() { man.clone() } else { man_subj.clone() },
            "she" if !woman.is_empty() => said = if woman_subj.is_empty() { woman.clone() } else { woman_subj.clone() },
            _ => {}
        }
        if let Some(n) = KNOWN_NAMES.iter().find(|n| **n == said) {
            let (last, subj) = if FEMALE.contains(n) { (&mut woman, &mut woman_subj) } else { (&mut man, &mut man_subj) };
            *last = n.to_string();
            if opens {
                *subj = n.to_string();
            }
        } else if on("names") && opens && said.len() > 2 && said.chars().next().is_some_and(char::is_uppercase) && !SENTENCE_OPENERS.contains(&said.to_lowercase().as_str()) && crate::numbers::correct(&said.to_lowercase()).is_none() && crate::triggers::number_word(&said).is_none() {
            // a name the school does not know: its sex unknown, so it stands for «he» and «she» both
            for x in [&mut man, &mut man_subj, &mut woman, &mut woman_subj] {
                *x = said.clone();
            }
        }
        out += &if said != w { format!("{said}{rest}") } else { tok.to_string() };
        // «Now he has»: a sentence opened by a word of time still has its subject next
        opens = tok.trim_end().ends_with(['.', '!', '?']) || (opens && matches!(w.to_lowercase().as_str(), "now" | "then" | "later"));
    }
    out
}

const FEMALE: [&str; 8] = ["Anna", "Lily", "Mia", "Zoe", "Emma", "Ruby", "Ella", "Ivy"];

/// The names the student knows from its lessons (the scenarios' and the lessons' as data).
const KNOWN_NAMES: [&str; 16] = ["Tom", "Anna", "Sam", "Lily", "Max", "Mia", "Ben", "Zoe", "Jack", "Emma", "Leo", "Ruby", "Noah", "Ella", "Omar", "Ivy"];

/// Words that open a sentence with a capital and are not names.
const SENTENCE_OPENERS: [&str; 52] = ["the", "a", "an", "there", "it", "he", "she", "they", "we", "you", "i", "if", "every", "each", "how", "what", "after", "later", "then", "now", "some", "this", "that", "these", "those", "his", "her", "their", "its", "my", "our", "your", "one", "all", "both", "together", "during", "before", "when", "while", "since", "so", "but", "and", "also", "being", "after", "on", "in", "at", "for", "mum"];

/// Words that measure a thing rather than name it: «cups of flour» is flour, «pieces of candy» is candy. Containers
/// («packs of cards») stay things of their own: a pack is counted, and what it holds is told apart.
const MEASURES: [&str; 16] = ["piece", "pieces", "cup", "cups", "pound", "pounds", "kilogram", "kilograms", "slice", "slices", "ounce", "ounces", "gallon", "gallons", "liter", "liters"];

/// The kind of thing named from word `k` on: the words up to a word of grammar, a number or the end, the last of
/// them made singular («15 bottle caps and …» → cap, «7 ripe apples from» → apple); «cups of flour» → flour.
fn thing_phrase(low: &[String], k: usize) -> String {
    let mut k = k;
    while k < low.len() && matches!(low[k].as_str(), "more" | "fewer" | "less" | "big" | "small" | "new" | "old" | "of") {
        k += 1;
    }
    if low.get(k + 1).is_some_and(|w| w == "of") && MEASURES.contains(&low[k].as_str()) {
        k += 2;
    }
    let stop = |w: &str| w.parse::<i64>().is_ok() || matches!(w, "and" | "from" | "to" | "in" | "on" | "at" | "of" | "for" | "with" | "each" | "every" | "than" | "more" | "left" | "a" | "the" | "that" | "which" | "he" | "she" | "it" | "they" | "his" | "her" | "their" | "then" | "but" | "while" | "if" | "per" | "into" | "is" | "are" | "was" | "were" | "has" | "have" | "had" | "does" | "did" | "do" | "will" | "would" | "can" | "could" | "there" | "now" | "today" | "yesterday" | "altogether" | "together" | "total" | "all" | "this" | "these" | "those" | "after" | "before" | "over" | "out" | "away" | "up" | "down" | "by" | "as" | "so" | "still" | "already" | "also");
    // the phrase ends at its first plural («pencils broke», «bottle caps and»)
    let mut words: Vec<&String> = Vec::new();
    for w in low[k.min(low.len())..].iter().take_while(|w| !stop(w)).take(3) {
        words.push(w);
        if school_norm(w).is_some_and(|n| n != **w) {
            break;
        }
    }
    words.last().and_then(|w| school_norm(w)).unwrap_or_default()
}

/// An object's name: its last word made singular, with the adjective before it («blue ribbon»).
fn object_name(words: &[String]) -> String {
    let mut v: Vec<String> = words.to_vec();
    if let Some(l) = v.last_mut() {
        *l = school_norm(l).unwrap_or(l.clone());
    }
    v.join(" ")
}

/// «3 dozen» → 36, «9 and a half dozen» → 114, «half a dozen» → 6, «7 pairs of gloves» → «14 gloves»: words that are
/// groups of a known size, counted out.
fn groups_of_words(text: &str) -> String {
    let w: Vec<&str> = text.split(' ').collect();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < w.len() {
        let n: Option<i64> = w[i].parse().ok();
        let low = |k: usize| w.get(k).map(|x| x.trim_end_matches(['.', ',', '?']).to_lowercase()).unwrap_or_default();
        match n {
            Some(n) if low(i + 1) == "and" && low(i + 2) == "a" && low(i + 3) == "half" && low(i + 4) == "dozen" => {
                out.push(format!("{}{}", 12 * n + 6, &w[i + 4][5..]));
                i += 5;
            }
            Some(n) if low(i + 1) == "dozen" => {
                out.push(format!("{}{}", 12 * n, &w[i + 1][5..]));
                i += 2;
            }
            Some(n) if matches!(low(i + 1).as_str(), "pairs" | "pair") && low(i + 2) == "of" => {
                out.push(format!("{}", 2 * n));
                i += 3;
            }
            _ if low(i) == "half" && low(i + 1) == "a" && low(i + 2) == "dozen" => {
                out.push(format!("6{}", &w[i + 2][5..]));
                i += 3;
            }
            _ => {
                out.push(w[i].to_string());
                i += 1;
            }
        }
    }
    out.join(" ")
}


impl Reading {
    /// The words that ask for a sum: «in all», «altogether», «in total», «both», «together», «between them», «combined».
    fn ask_all_words(&self, q: &str) -> bool {
        [("all", " all"), ("altogether", "altogether"), ("total", "total"), ("both", "both"), ("together", "together"), ("between", "between"), ("combined", "combined")].iter().any(|(w, x)| on(w) && q.contains(x))
    }
}

/// The world after the reading, by the verbs' effects as the student believes them: actor → count.
fn play(r: &Reading, eff: &dyn Fn(&str) -> Option<Effect>) -> Option<BTreeMap<String, i64>> {
    let mut w: BTreeMap<String, i64> = BTreeMap::new();
    for (who, _, n) in &r.holds {
        *w.entry(who.clone()).or_default() += n;
    }
    for e in &r.events {
        // paying: the cents leave the payer (the skeleton's, as «cost» is)
        let (a, b) = if e.verb == "pay" { (-1, 0) } else { eff(&e.verb)? };
        let n = match e.frac {
            Some((p, q)) => (w.get(&e.subj).copied().unwrap_or(0) - e.keep) * p / q,
            None => e.n,
        };
        let e = &Ev { n, ..e.clone() };
        *w.entry(e.subj.clone()).or_default() += a * e.n;
        // the other party: the receiver named, or else the one who holds the things when the actor holds none
        // («Lily takes 15 out» of the jar that holds them)
        let other = if !e.to.is_empty() { Some(e.to.clone()) } else if !r.holds.iter().any(|h| h.0 == e.subj) { r.holds.iter().map(|h| h.0.clone()).find(|h| *h != e.subj) } else { None };
        if let Some(o) = other {
            *w.entry(o).or_default() += b * e.n;
        }
    }
    Some(w)
}

/// The answer the reading asks for, from the world.
fn answer_of(r: &Reading, w: &BTreeMap<String, i64>) -> Option<i64> {
    // a line
    if let (Some(f), Some(b)) = (r.front, r.back) {
        return Some(f + b - 1);
    }
    if let Some(f) = r.front {
        if r.ask_front {
            return Some(f - 1);
        }
        if r.ask_behind {
            return r.line.map(|n| n - f);
        }
    }
    if let Some((s, e)) = r.span {
        return Some(e - s + 1);
    }
    if let Some((h, d)) = r.clock {
        return Some((h + d - 1) % 12 + 1);
    }
    if let Some(h) = r.ask_minutes {
        return Some(60 * h);
    }
    // the unknown start: the end told, less what happened (played from nothing)
    if let (Some(who), Some((x, n))) = (&r.unknown_start, &r.now) {
        if who == x {
            return Some(n - w.get(who).copied().unwrap_or(0));
        }
    }
    // the unknown change: the difference between the end told and the story without it
    if let (Some((x, n)), Some(_)) = (&r.now, r.events.iter().find(|e| e.unknown)) {
        return Some((n - w.get(x).copied().unwrap_or(0)).abs());
    }
    if let Some((n, k)) = r.pack {
        if k > 0 && r.ask_full {
            return Some(n / k);
        }
        if k > 0 && r.ask_over {
            return Some(n % k);
        }
    }
    if let (Some(x), Some(d)) = (r.rate, r.days) {
        return Some(x * d);
    }
    if let Some((who, k)) = &r.share_rest {
        let have = *w.get(who)?;
        return (*k > 0 && have % k == 0).then(|| have / k);
    }
    if r.ask_spent {
        let who = r.ask_whose.clone();
        return Some(r.events.iter().filter(|e| e.verb == "pay" && who.as_ref().is_none_or(|x| *x == e.subj)).map(|e| e.n).sum());
    }
    if let Some((n, m)) = r.share {
        return (m != 0 && n % m == 0).then(|| n / m);
    }
    if let Some((k, m)) = r.groups.last() {
        return Some(k * m);
    }
    if !r.ask_some.is_empty() {
        return r.ask_some.iter().map(|x| w.get(x).copied()).sum();
    }
    if let (Some(x), Some(y)) = (&r.ask_whose, &r.ask_than) {
        return Some((w.get(x)? - w.get(y)?).abs());
    }
    if let Some(y) = &r.ask_than {
        // «how many fewer … in the bag than in the box»: the two holders
        let other = w.keys().find(|k| *k != y)?;
        return Some((w.get(other)? - w.get(y)?).abs());
    }
    if let Some(x) = &r.ask_whose {
        return w.get(x).map(|v| v + r.shift);
    }
    if r.ask_all {
        return Some(w.values().sum());
    }
    // a question that holds its own number and nothing else does («How many socks are in 12 socks?»)
    if r.holds.is_empty() && r.events.is_empty() && r.q_num.is_some() {
        return r.q_num;
    }
    // the one the story is about: the first holder
    let first = r.holds.first().map(|h| h.0.clone()).or_else(|| r.events.first().map(|e| e.subj.clone()))?;
    w.get(&first).copied()
}

/// An unknown verb looked up in the dictionary: its meaning for a pupil names a verb the student knows («To buy is to
/// get something…» — as «get»), or says the thing is no longer had («To break is to … so it can no longer be used»).
fn from_dictionary(actors: &Actors, v: &str) -> Option<Effect> {
    if !on("dictionary") {
        return None;
    }
    static M: std::sync::OnceLock<BTreeMap<String, [String; 6]>> = std::sync::OnceLock::new();
    // the first sentence of the first sense only: a rich entry's other senses and examples name other verbs by the way
    let meaning = M.get_or_init(crate::dictionary::improved).get(v)?[2].to_lowercase().split(" 2) ").next().unwrap_or("").split(['.', ';', ':']).next().unwrap_or("").to_string();
    // a verb's own meaning («to buy is to get …»); a noun's description names verbs only by the way
    let Some(body) = meaning.split_once(" is to ").map(|x| x.1) else {
        return (meaning.contains("no longer") || meaning.contains(" gone")).then_some((-1, 0));
    };
    let words: Vec<&str> = body.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).collect();
    for w in &words {
        let l = match *w { "lost" => "lose", "got" | "gets" => "get", "gave" => "give", other => other };
        if l != v {
            if let Some(e) = actors.effect(l) {
                return Some(e);
            }
        }
    }
    (body.contains("no longer") || body.contains(" gone")).then_some((-1, 0))
}

/// The data directory (`$MOVA_DATA`, default `~/mova-data`); the research data is under `research/`.
fn mova_data_dir() -> String {
    std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))
}

#[path = "school_tree.rs"]
mod school_tree;

/// The tree reader is the only reader (15.10: the old window-of-words reader was removed once the tree reader matched
/// it — lessons 2367 → 2380, data 1123 → 1338, exam 627 → 635, cv world 297 → 510).
fn tree_reader() -> bool {
    true
}

/// A problem read into a world, along its dependency trees (`school_tree`).
fn reading_of(text: &str) -> Reading {
    school_tree::read_tree(text)
}

/// The world reader's answer; the tree reader adds its schemas (`treeread`) where the world it read has none.
fn world_answer_of(actors: &Actors, text: &str, r: Reading) -> (Option<i64>, String) {
    let (a, why) = world_answer(actors, r);
    // only where the world has no answer at all: a guess is how the student learns a verb (the teacher confirms it)
    // (the lessons as data have the learned operation for that: it answers next, the schemas would cut it off)
    if tree_reader() && a.is_none() && !*LESSONS.lock().unwrap() {
        if let Some((v, w)) = crate::treeread::answer(text) {
            return (Some(v), format!("the tree reader: {w}"));
        }
    }
    (a, why)
}

/// The student's answer with the verbs it believes in; unknown verbs make it say it does not know.
fn actor_answer(actors: &Actors, p: &Problem) -> (Option<i64>, String) {
    let r = reading_of(&p.text);
    // «how many more A than B», where B is no one in the story's world: the two told quantities the question is about
    if on("mentions") {
        let holders: BTreeSet<&str> = r.holds.iter().map(|h| h.0.as_str()).chain(r.events.iter().map(|e| e.subj.as_str())).collect();
        let than_holder = r.ask_than.as_ref().is_some_and(|t| holders.contains(t.as_str()));
        if !than_holder {
            let question = p.text.rsplit_once(['.', '!']).map(|x| x.1).unwrap_or(&p.text);
            if let Some((v, why)) = crate::mentions::more_than(&p.text, question) {
                return (Some(v), format!("the told quantities: {why}"));
            }
        }
    }
    if on("sums") && *BOOK.lock().unwrap() {
        if let Some((v, why)) = crate::mentions::sum_of(&p.text) {
            return (Some(v), format!("the told quantities the question gathers: {why}"));
        }
    }
    if on("ops") && *LESSONS.lock().unwrap() {
        // the tree reader reads more of a lesson's story than the old one and so answers more of it, also where it does
        // not understand the question; the learned operation goes first where its margin reaches MOVA_TREE_SURE
        // (15.10, lessons as data / exam: margin ≥ 0 1338 / 635, ≥ 2 1317 / 640, ≥ 4 1296 / 631, ≥ 8 1253 / 646,
        // never 1096 / 633; the old reader 1123 / 627) — 0 by default: the learned operation first
        if tree_reader() {
            let ops = crate::opchoice::current();
            let sure: i64 = std::env::var("MOVA_TREE_SURE").ok().and_then(|x| x.parse().ok()).unwrap_or(0);
            if crate::opchoice::margin(&ops, &p.text) >= sure {
                if let Some((v, w)) = crate::opchoice::answer(&ops, &p.text) {
                    return (Some(v), format!("the operation I learned, sure: {w}"));
                }
            }
        }
        let r2 = r.clone();
        let (a, why) = world_answer_of(actors, &p.text, r2);
        if a.is_some_and(|v| v >= 0) && !why.starts_with("I guess") {
            return (a, why);
        }
        let ops = crate::opchoice::current();
        if let Some((v, w)) = crate::opchoice::answer(&ops, &p.text) {
            return (Some(v), format!("the operation I learned: {w}"));
        }
        return (a, why);
    }
    if on("ops") && *BOOK.lock().unwrap() {
        // a book's problem: the book's way — the told quantities, and the operation learned from the book's answers;
        // the world reader only where the learned choice has nothing to offer
        // the tree reader first, for the schemas named in MOVA_TREESCHEMAS (comma-separated, e.g. «each,the change»)
        if let Ok(list) = std::env::var("MOVA_TREESCHEMAS") {
            if let Some((v, w)) = crate::treeread::answer(&p.text) {
                if list.split(',').any(|x| w.split(':').next() == Some(x)) {
                    return (Some(v), format!("the tree reader: {w}"));
                }
            }
        }
        let ops = crate::opchoice::current();
        if let Some((v, w)) = crate::opchoice::answer(&ops, &p.text) {
            let m = crate::opchoice::margin(&ops, &p.text);
            let sure = if m >= crate::opchoice::SURE { format!("I am sure (a margin of {m})") } else { format!("I am not sure (a margin of {m}): my best guess") };
            return (Some(v), format!("{sure}: the operation I learned: {w}"));
        }
    }
    world_answer_of(actors, &p.text, r)
}

/// Whether the student is reading a book's problems (the book's lessons and the held-out check) or the school's
/// stories: the lesson tells it, as a pupil knows whether the page before them is a story told at the table or an
/// exercise of a book.
static BOOK: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

/// The lessons written as data: the world answers what it reads without a guess, the learned operation the rest.
static LESSONS: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

fn set_lessons(on: bool) {
    *LESSONS.lock().unwrap() = on;
}

fn set_book(on: bool) {
    *BOOK.lock().unwrap() = on;
}

fn world_answer(actors: &Actors, r: Reading) -> (Option<i64>, String) {
    if !on("things") {
        return answer_reading(actors, r);
    }
    // the world kept by things: when the story counts several kinds of things, the question's kind is read alone;
    // «how many more wrappers than bottle caps» reads each kind alone and compares
    let kinds: BTreeSet<&str> = r.holds.iter().map(|h| h.1.as_str()).chain(r.events.iter().map(|e| e.thing.as_str())).filter(|t| !t.is_empty()).collect();
    if kinds.len() >= 2 {
        if let (Some(t1), Some(t2)) = (&r.ask_thing, &r.than_thing) {
            if kinds.contains(t1.as_str()) && kinds.contains(t2.as_str()) {
                let (a, wa) = answer_reading(actors, focus(&r, t1, true));
                let (b, wb) = answer_reading(actors, focus(&r, t2, true));
                return (a.zip(b).map(|(a, b)| (a - b).abs()), format!("{t1}: {wa}; {t2}: {wb}"));
            }
        }
        if let Some(t) = &r.ask_thing {
            if kinds.contains(t.as_str()) {
                return answer_reading(actors, focus(&r, t, false));
            }
        }
    }
    answer_reading(actors, r)
}

/// The answer of a reading with the given effects of the verbs, the world kept by things as in `actor_answer`.
fn solve(r: &Reading, eff: &dyn Fn(&str) -> Option<Effect>) -> Option<i64> {
    let one = |r: &Reading| play(r, eff).and_then(|w| answer_of(r, &w));
    if on("things") {
        let kinds: BTreeSet<&str> = r.holds.iter().map(|h| h.1.as_str()).chain(r.events.iter().map(|e| e.thing.as_str())).filter(|t| !t.is_empty()).collect();
        if kinds.len() >= 2 {
            if let (Some(t1), Some(t2)) = (&r.ask_thing, &r.than_thing) {
                if kinds.contains(t1.as_str()) && kinds.contains(t2.as_str()) {
                    return one(&focus(r, t1, true)).zip(one(&focus(r, t2, true))).map(|(a, b)| (a - b).abs());
                }
            }
            if let Some(t) = r.ask_thing.as_ref().filter(|t| kinds.contains(t.as_str())) {
                return one(&focus(r, t, false));
            }
        }
    }
    one(r)
}

/// The reading of one kind of thing: its holdings and events only (and, when asked to compare kinds, a plain
/// question about how many there are).
fn focus(r: &Reading, t: &str, plain: bool) -> Reading {
    let mut f = r.clone();
    f.holds.retain(|h| h.1 == t);
    f.events.retain(|e| e.thing == t);
    if plain {
        f.ask_than = None;
        f.than_thing = None;
    }
    f
}

fn answer_reading(actors: &Actors, r: Reading) -> (Option<i64>, String) {
    // a verb it does not know yet is guessed from the context of the question: «… how many are left» — the actor
    // lost them; else the actor got them (the guess is said, and the teacher's answer confirms or corrects it)
    let takes_from_holder = r.events.iter().any(|e| e.to.is_empty() && !r.holds.iter().any(|h| h.0 == e.subj) && !r.holds.is_empty());
    // causality in the skeleton (an axiom, not learned from a few examples): what is passed to someone, that one has
    // more of, and the one who passed it has less — so an unknown verb with a named receiver is a transfer
    let transfer: std::collections::BTreeSet<&str> = r.events.iter().filter(|e| !e.to.is_empty()).map(|e| e.verb.as_str()).collect();
    // «finds 3 more»: whatever the verb, the actor has more after it
    let gains: std::collections::BTreeSet<&str> = r.events.iter().filter(|e| e.more).map(|e| e.verb.as_str()).collect();
    let guess = |v: &str| {
        actors.effect(v).or(Some(if transfer.contains(v) {
            (-1, 1)
        } else if gains.contains(v) {
            (1, 0)
        } else if r.ask_left && takes_from_holder {
            // «Lily takes 15 out» of the jar that holds them, and the question asks what the jar has left: whatever the
            // verb means elsewhere, here it moves things from the holder to the actor
            (1, -1)
        } else if let Some(e) = from_dictionary(actors, v) {
            e
        } else if r.ask_left {
            (-1, 0)
        } else {
            (1, 0)
        }))
    };
    let unknown: Vec<&str> = r.events.iter().filter(|e| actors.effect(&e.verb).is_none()).map(|e| e.verb.as_str()).collect();
    if !unknown.is_empty() {
        if let Some(w) = play(&r, &guess) {
            return (answer_of(&r, &w), format!("I guess what «{}» does from the question: {:?}", unknown.join("», «"), w));
        }
    }
    match play(&r, &|v| actors.effect(v)) {
        Some(w) => (answer_of(&r, &w), format!("{:?}", w)),
        None => (None, format!("I do not know what «{}» does", r.events.iter().find(|e| actors.effect(&e.verb).is_none()).map(|e| e.verb.as_str()).unwrap_or("?"))),
    }
}

/// Learn from the right answer: the effects of the problem's verbs that would give it (all combinations of the
/// unknown or doubted verbs, the simplest first); each verb's found effect gets a vote.
fn actor_learn(actors: &mut Actors, p: &Problem) {
    actor_learn_answer(actors, p, p.answer());
}

// ---- Scenario lessons: little worlds made by the teacher, told in many phrasings ---------------------------------
//
// The teacher makes a little world — actors, what each holds, who passes what to whom, who finds or loses — and computes
// the right answer from its own world, never from the text. The student must read the text. Every kind of event can be
// said in several ways; the tests use only phrasings the exercises never did. Starred lessons are harder.

#[derive(Clone, Copy, PartialEq)]
enum Act {
    Give,
    Find,
    Lose,
}

/// One event of the teacher's world: what happens, who, to whom, how many.
#[derive(Clone)]
struct SEv {
    act: Act,
    who: usize,
    to: usize,
    n: i64,
}

/// The phrasings of a transfer «{a} → {b}, {n} {t}»: the first ones for exercises, the last ones only for tests.
const GIVE_SAY: [&str; 8] = ["{a} gives {b} {n} {t}.", "{a} gives {n} {t} to {b}.", "{a} hands {n} {t} to {b}.", "{b} gets {n} {t} from {a}.", "{a} passes {n} {t} to {b}.", "{b} receives {n} {t} from {a}.", "{b} is given {n} {t} by {a}.", "{a} hands {b} {n} {t}."];
const GIVE_TEST_FROM: usize = 5;
const FIND_SAY: [&str; 4] = ["{a} finds {n} more {t}.", "{a} gets {n} more {t}.", "{a} picks up {n} more {t}.", "{a} buys {n} more {t}."];
const LOSE_SAY: [&str; 4] = ["{a} loses {n} {t}.", "{a} eats {n} {t}.", "{a} drops {n} {t}.", "{a} breaks {n} {t}."];

/// The second course of scenarios (lessons 15–20): holdings told against another's («3 more than Anna», «twice as
/// many as»), giving to each of several, halves and doubles, «the rest», and totals of some of the actors.
fn scenario2(rng: &mut Rng, lesson: usize, test: bool) -> Problem {
    let mut names: Vec<&str> = NAMES.to_vec();
    for i in (1..names.len()).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        names.swap(i, j);
    }
    let his = |x: &str| if matches!(x, "Anna" | "Lily" | "Mia" | "Zoe") { "her" } else { "his" };
    let t = *rng.pick(&THINGS);
    let pick2 = |rng: &mut Rng, a: &[&'static str], b: &[&'static str]| -> &'static str { if test { *rng.pick(b) } else { *rng.pick(a) } };
    let (a, b, c) = (names[0], names[1], names[2]);
    let mut text = String::new();
    let (q, ans);
    match lesson {
        15 => {
            // «Ben has 3 more apples than Anna»
            let x = rng.range(4, 12);
            let d = rng.range(1, 3.min(x - 1).max(1));
            let more = rng.next() % 2 == 0;
            let y = if more { x + d } else { x - d };
            text += &format!("{a} has {x} {t}. ");
            let w = if more { "more" } else { "fewer" };
            text += &pick2(rng, &["{b} has {d} {w} {t} than {a}. "], &["{b} has {d} {t} {w} than {a}. ", "{b} has {w} {t} than {a}, {d} {w}. "]).replace("{a}", a).replace("{b}", b).replace("{d}", &d.to_string()).replace("{w}", w).replace("{t}", t);
            if rng.next() % 2 == 0 {
                q = format!("How many {t} does {b} have?");
                ans = y;
            } else {
                q = format!("How many {t} do they have in all?");
                ans = x + y;
            }
        }
        16 => {
            // «twice as many as», «three times as many as», «half as many as»
            let x = 2 * rng.range(2, 8);
            let (k, word_ex, word_test, y) = match rng.next() % 3 {
                0 => (2, "twice as many", "two times as many", 2 * x),
                1 => (3, "three times as many", "3 times as many", 3 * x),
                _ => (0, "half as many", "half as many", x / 2),
            };
            let _ = k;
            text += &format!("{a} has {x} {t}. {b} has {} {t} as {a}. ", if test { word_test } else { word_ex });
            if rng.next() % 2 == 0 {
                q = format!("How many {t} does {b} have?");
                ans = y;
            } else {
                q = format!("How many {t} do {a} and {b} have altogether?");
                ans = x + y;
            }
        }
        17 => {
            // «Tom gives 2 apples to each of his 3 friends»
            let (n, k) = (rng.range(2, 4), rng.range(2, 4));
            let x = n * k + rng.range(1, 9);
            text += &format!("{a} has {x} {t}. ");
            text += &(if test { format!("{a} gives each of {} {k} friends {n} {t}. ", his(a)) } else { format!("{a} gives {n} {t} to each of {} {k} friends. ", his(a)) });
            q = if test { format!("How many {t} does {a} have left?") } else { format!("How many {t} does {a} have now?") };
            ans = x - n * k;
        }
        18 => {
            // halves and doubles
            let x = 2 * rng.range(3, 10);
            let y = rng.range(1, 9);
            text += &format!("{a} has {x} {t}. {b} has {y} {t}. ");
            match rng.next() % 3 {
                0 => {
                    text += &(if test { format!("{a} gives {b} half of {} {t}. ", his(a)) } else { format!("{a} gives half of {} {t} to {b}. ", his(a)) });
                    q = format!("How many {t} does {b} have now?");
                    ans = y + x / 2;
                }
                1 => {
                    text += &(if test { format!("{a} loses half of {} {t}. ", his(a)) } else { format!("{a} eats half of {} {t}. ", his(a)) });
                    q = format!("How many {t} does {a} have now?");
                    ans = x / 2;
                }
                _ => {
                    text += &format!("{b} doubles {} {t}. ", his(b));
                    q = if test { format!("How many {t} do they have in total?") } else { format!("How many {t} does {b} have now?") };
                    ans = if test { x + 2 * y } else { 2 * y };
                }
            }
        }
        19 => {
            // «Tom gives 3 to Anna and the rest to Ben»
            let x = rng.range(6, 15);
            let n = rng.range(1, x - 2);
            let (y, z) = (rng.range(1, 6), rng.range(1, 6));
            text += &format!("{a} has {x} {t}. {b} has {y} {t}. {c} has {z} {t}. ");
            if test {
                text += &format!("{a} keeps {n} {t} and gives the rest to {c}. ");
                q = format!("How many {t} does {c} have now?");
                ans = z + x - n;
            } else {
                text += &format!("{a} gives {n} {t} to {b} and the rest to {c}. ");
                if rng.next() % 2 == 0 {
                    q = format!("How many {t} does {c} have now?");
                    ans = z + x - n;
                } else {
                    q = format!("How many {t} does {b} have now?");
                    ans = y + n;
                }
            }
        }
        _ => {
            // 20: totals of some of the actors
            let (x, y, z) = (rng.range(2, 12), rng.range(2, 12), rng.range(2, 12));
            text += &format!("{a} has {x} {t}. {b} has {y} {t}. {c} has {z} {t}. ");
            let n = rng.range(1, x - 1);
            text += &format!("{a} gives {n} {t} to {c}. ");
            let tail = pick2(rng, &["altogether", "together", "in all"], &["in total", "between them", "combined"]);
            q = format!("How many {t} do {b} and {c} have {tail}?");
            ans = y + z + n;
        }
    }
    text += &q;
    Problem { text, a: ans, b: 0, op: Op::Add }
}

const ITEMS: [&str; 6] = ["pencil", "apple", "sticker", "book", "cookie", "card"];
const WORD_NUMBERS: [&str; 21] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty"];

/// The third course of scenarios (lessons 21–26): prices and money, rates in time, full boxes and what is left over,
/// thirds and quarters, numbers in words with «he» and «she», and a long problem that joins them.
fn scenario3(rng: &mut Rng, lesson: usize, test: bool) -> Problem {
    let mut names: Vec<&str> = NAMES.to_vec();
    for i in (1..names.len()).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        names.swap(i, j);
    }
    let female = |x: &str| matches!(x, "Anna" | "Lily" | "Mia" | "Zoe");
    let (a, b) = (names[0], names[1]);
    let item = *rng.pick(&ITEMS);
    let t = *rng.pick(&THINGS);
    let (text, ans): (String, i64) = match lesson {
        21 => {
            let (p, k) = (rng.range(2, 9), rng.range(2, 6));
            if rng.next() % 2 == 0 {
                if test {
                    (format!("{item}s cost {p} cents each. {a} buys {k} of them. How many cents does {a} spend?"), p * k)
                } else {
                    (format!("A {item} costs {p} cents. {a} buys {k} {item}s. How many cents does {a} pay?"), p * k)
                }
            } else {
                let m = p * k + rng.range(1, 20);
                if test {
                    (format!("{a} has {m} cents and buys {k} {item}s for {p} cents each. How many cents does {a} have left?"), m - p * k)
                } else {
                    (format!("{a} has {m} cents. {a} buys {k} {item}s at {p} cents each. How many cents does {a} have left?"), m - p * k)
                }
            }
        }
        22 => {
            let r = rng.range(2, 9);
            let (span, d) = match rng.next() % 3 {
                0 => { let d = rng.range(2, 6); (format!("{d} days"), d) }
                1 => ("a week".to_string(), 7),
                _ => ("two weeks".to_string(), 14),
            };
            if test {
                (format!("{a} saves {r} cents a day. How many cents does {a} save in {span}?"), r * d)
            } else {
                (format!("{a} reads {r} pages every day. How many pages does {a} read in {span}?"), r * d)
            }
        }
        23 => {
            let k = rng.range(3, 8);
            let n = k * rng.range(2, 6) + rng.range(1, k - 1);
            let full = rng.next() % 2 == 0;
            let start = if test { format!("{a} puts {n} eggs into boxes that hold {k} each. ") } else { format!("{n} eggs are packed in boxes of {k}. ") };
            let q = match (full, test) {
                (true, false) => "How many full boxes are there?".to_string(),
                (true, true) => format!("How many boxes can {a} fill?"),
                (false, false) => "How many eggs are left over?".to_string(),
                (false, true) => "How many eggs remain outside the full boxes?".to_string(),
            };
            (start + &q, if full { n / k } else { n % k })
        }
        24 => {
            let n = 12 * rng.range(1, 4);
            let (ex, te, num, den) = *rng.pick(&[("a third", "one third", 1, 3), ("a quarter", "a fourth", 1, 4), ("two thirds", "2 thirds", 2, 3), ("three quarters", "3 quarters", 3, 4), ("a half", "one half", 1, 2)]);
            let y = rng.range(1, 9);
            let part = n * num / den;
            let fr = if test { te } else { ex };
            if rng.next() % 2 == 0 {
                (format!("{a} has {n} {t}. {b} has {y} {t}. {a} gives {fr} of them to {b}. How many {t} does {b} have now?"), y + part)
            } else {
                (format!("{a} has {n} {t}. {a} loses {fr} of them. How many {t} does {a} have left?"), n - part)
            }
        }
        25 => {
            // «he» and «she» for two people of different sexes, so the pronoun is never ambiguous
            let b = names.iter().copied().find(|n| female(n) != female(a)).unwrap_or(b);
            let x = rng.range(8, 20);
            let g = rng.range(2, x - 2);
            let e = rng.range(1, g);
            let (pa, pb) = (if female(a) { "She" } else { "He" }, if female(b) { "She" } else { "He" });
            let w = |n: i64| WORD_NUMBERS[n as usize];
            if test {
                (format!("{a} has {} {t}. {pa} gives {b} {}. Then {} loses {} of them. How many {t} does {b} have now?", w(x), w(g), pb.to_lowercase(), w(e)), g - e)
            } else {
                (format!("{a} has {} {t}. {pa} gives {} to {b}. {pb} eats {} of them. How many {t} does {b} have now?", w(x), w(g), w(e)), g - e)
            }
        }
        _ => {
            // 26: packs, a gift, and the rest shared equally
            let (k, m, s) = (rng.range(2, 5), rng.range(3, 8), rng.range(2, 4));
            let total = k * m;
            let each = rng.range(1, (total - 1) / s);
            let g = total - each * s;
            let pack = if test { format!("There are {m} {t} in each pack.") } else { format!("Each pack holds {m} {t}.") };
            (format!("{a} buys {k} packs of {t}. {pack} {a} gives {g} {t} to {b} and shares the rest equally among {s} friends. How many {t} does each friend get?"), each)
        }
    };
    Problem { text, a: ans, b: 0, op: Op::Add }
}

/// A typo the teacher makes on purpose in a number word: a letter dropped, doubled or swapped with the next — never a
/// real word, and never the word itself.
fn typo(rng: &mut Rng, w: &str) -> String {
    let c: Vec<char> = w.chars().collect();
    for _ in 0..20 {
        let i = (rng.next() % c.len() as u64) as usize;
        let t: String = match rng.next() % 3 {
            0 if c.len() > 4 => c.iter().enumerate().filter(|x| x.0 != i).map(|x| x.1).collect(),
            1 if i + 1 < c.len() => { let mut d = c.clone(); d.swap(i, i + 1); d.into_iter().collect() }
            _ => c.iter().enumerate().flat_map(|x| if x.0 == i { vec![*x.1, *x.1] } else { vec![*x.1] }).collect(),
        };
        if t != w && en::dict::form(&t).is_none() && crate::numbers::correct(&t).is_some() {
            return t;
        }
    }
    w.to_string()
}

/// The fourth course of scenarios (lessons 27–38): big numbers in words, typos in them, ordinals in a line and on a
/// calendar, hours and minutes, ages, an unknown start, an unknown change, lengths, pairs and dozens, and a long
/// problem that joins them.
fn scenario4(rng: &mut Rng, lesson: usize, test: bool) -> Problem {
    let mut names: Vec<&str> = NAMES.to_vec();
    for i in (1..names.len()).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        names.swap(i, j);
    }
    let (a, b) = (names[0], names[1]);
    let her = |x: &str| if FEMALE.contains(&x) { "her" } else { "him" };
    let she = |x: &str| if FEMALE.contains(&x) { "she" } else { "he" };
    let t = *rng.pick(&THINGS);
    let words = |n: i64| crate::numbers::to_words(n);
    let ord = |n: i64| format!("{n}{}", crate::numbers::ordinal_suffix(n));
    let ordw = |n: i64| crate::numbers::to_ordinal_words(n);
    let (text, ans): (String, i64) = match lesson {
        27 => {
            let (x, y) = (rng.range(21, 500), rng.range(11, 99));
            let say = |n: i64| if test { words(n).replace('-', " ").replace(" and ", " ") } else { words(n) };
            if rng.next() % 2 == 0 {
                (format!("{a} has {} {t}. {a} gets {} more {t}. How many {t} does {a} have now?", say(x), say(y)), x + y)
            } else {
                (format!("{a} has {} {t}. {a} loses {} {t}. How many {t} does {a} have now?", say(x + y), say(y)), x)
            }
        }
        28 => {
            let (x, y) = (rng.range(21, 99), rng.range(2, 19));
            let mut say = |n: i64| {
                let w = words(n);
                if !test {
                    return w;
                }
                // one typo in one of the number's words
                let parts: Vec<&str> = w.split(['-', ' ']).collect();
                let k = (rng.next() % parts.len() as u64) as usize;
                let wrong = typo(rng, parts[k]);
                w.replacen(parts[k], &wrong, 1)
            };
            let (sx, sy) = (say(x), say(y));
            (format!("{a} has {sx} {t}. {a} gives {sy} {t} to {b}. How many {t} does {a} have now?"), x - y)
        }
        29 => {
            let n = rng.range(8, 30);
            let k = rng.range(2, n - 1);
            let front = rng.next() % 2 == 0;
            let place = if test { ordw(k) } else { ord(k) };
            let q = if front { format!("How many people are in front of {}?", her(a)) } else { format!("How many people are behind {}?", her(a)) };
            if test {
                (format!("There are {} children in a queue. {a} is the {place}. {q}", words(n)), if front { k - 1 } else { n - k })
            } else {
                (format!("{n} children stand in a line. {a} is the {place} in the line. {q}"), if front { k - 1 } else { n - k })
            }
        }
        30 => {
            let (f, k) = (rng.range(2, 15), rng.range(2, 15));
            let (pf, pk) = if test { (ordw(f), ordw(k)) } else { (ord(f), ord(k)) };
            (format!("{a} is the {pf} from the front of a line and the {pk} from the back. How many people are in the line?"), f + k - 1)
        }
        31 => {
            let s = rng.range(1, 20);
            let e = s + rng.range(1, 10);
            let month = *rng.pick(&["May", "June", "March", "October"]);
            if test {
                (format!("A fair is open from the {} to the {} of {month}. How many days is it open?", ordw(s), ordw(e)), e - s + 1)
            } else {
                (format!("A fair starts on the {} of {month} and ends on the {} of {month}. How many days does it last?", ord(s), ord(e)), e - s + 1)
            }
        }
        32 => {
            if rng.next() % 2 == 0 {
                let (h, d) = (rng.range(1, 6), rng.range(1, 5));
                if test {
                    (format!("A film begins at {} o'clock and runs for {} hours. At what hour does it end?", words(h), words(d)), h + d)
                } else {
                    (format!("A film starts at {h} o'clock and lasts {d} hours. At what hour does it end?"), h + d)
                }
            } else {
                let h = rng.range(2, 6);
                if test {
                    (format!("How many minutes are there in {} hours?", words(h)), 60 * h)
                } else {
                    (format!("An hour has 60 minutes. How many minutes are in {h} hours?"), 60 * h)
                }
            }
        }
        33 => {
            let x = rng.range(5, 12);
            let d = rng.range(1, 4);
            match rng.next() % 3 {
                0 => {
                    let older = rng.next() % 2 == 0;
                    let w = if older { "older" } else { "younger" };
                    (format!("{a} is {x} years old. {b} is {d} years {w} than {a}. How old is {b}?"), if older { x + d } else { x - d })
                }
                1 => (if test { format!("{a} is {x}. How old will {a} be in {} years' time?", words(d)) } else { format!("{a} is {x} years old. How old will {a} be in {d} years?") }, x + d),
                _ => (if test { format!("{a} is {x} now. How old was {a} {} years ago?", words(d)) } else { format!("{a} is {x} years old. How old was {a} {d} years ago?") }, x - d),
            }
        }
        34 => {
            let (n, now) = (rng.range(1, 9), rng.range(1, 12));
            let start = if test { *rng.pick(&["at the start", "in the beginning"]) } else { "at first" };
            if rng.next() % 2 == 0 {
                (format!("{a} had some {t}. {} gave {n} {t} to {b}. Now {} has {now} {t}. How many {t} did {a} have {start}?", she(a), she(a)), now + n)
            } else {
                (format!("{a} had some {t}. {} found {n} more {t}. Now {} has {} {t}. How many {t} did {a} have {start}?", she(a), she(a), now + n), now)
            }
        }
        35 => {
            let x = rng.range(3, 12);
            let n = rng.range(1, 9);
            let some = if test { "a few" } else { "some" };
            if rng.next() % 2 == 0 {
                (format!("{a} had {x} {t}. {} found {some} more. Now {} has {} {t}. How many {t} did {a} find?", she(a), she(a), x + n), n)
            } else {
                (format!("{a} had {} {t}. {} lost {some} of them. Now {} has {x} {t}. How many {t} did {a} lose?", x + n, she(a), she(a)), n)
            }
        }
        36 => {
            let (x, d) = (rng.range(8, 30), rng.range(1, 7));
            let unit = if test { *rng.pick(&["metres", "centimetres"]) } else { *rng.pick(&["meters", "centimeters"]) };
            match rng.next() % 3 {
                0 => (format!("A rope is {x} {unit} long. {a} cuts off {d} {unit}. How long is the rope now?"), x - d),
                1 => (format!("The red ribbon is {x} {unit} long. The blue ribbon is {d} {unit} longer. How long is the blue ribbon?"), x + d),
                _ => (format!("The red ribbon is {x} {unit} long. The blue ribbon is {d} {unit} shorter. How long is the blue ribbon?"), x - d),
            }
        }
        37 => {
            let k = rng.range(2, 9);
            match (rng.next() % 2, test) {
                (0, false) => (format!("How many socks are in {k} pairs of socks?"), 2 * k),
                (0, true) => (format!("{a} has {} pairs of gloves. How many gloves is that?", words(k)), 2 * k),
                (_, false) => (format!("Mum buys {k} dozen eggs. How many eggs does she buy?"), 12 * k),
                (_, true) => (format!("Mum buys {} and a half dozen eggs. How many eggs does she buy?", words(k)), 12 * k + 6),
            }
        }
        _ => {
            // 38: an unknown start told in words with a typo, and a dozen
            let (n, now) = (rng.range(2, 9), rng.range(13, 60));
            let w = words(now);
            let w = if test { let p = w.split(['-', ' ']).next().unwrap_or("").to_string(); w.replacen(&p, &typo(rng, &p), 1) } else { w };
            (format!("{a} had some {t}. {} bought {n} dozen more. Now {} has {} {t}. How many {t} did {a} have at first?", she(a), she(a), w.replace('-', " ")), now - 12 * n)
        }
    };
    Problem { text, a: ans, b: 0, op: Op::Add }
}

/// A scenario: the text, and the right answer from the teacher's own world — a world where no one ends with less
/// than nothing (a story that would need it is told again).
fn scenario(rng: &mut Rng, lesson: usize, test: bool) -> Problem {
    loop {
        let p = scenario_once(rng, lesson, test);
        if p.a >= 0 && !p.text.contains(" -") {
            return p;
        }
    }
}

/// The seventh course (lessons 49–60): chains of comparisons, a comparison read backwards, a change after a
/// comparison, perimeters and areas, minutes between two clock times, change from a note, what is left after sharing,
/// a change unknown in the middle, averages, and a review.
fn scenario5(rng: &mut Rng, lesson: usize, test: bool) -> Problem {
    let mut names: Vec<&str> = NAMES.to_vec();
    for i in (1..names.len()).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        names.swap(i, j);
    }
    let (a, b, c) = (names[0], names[1], names[2]);
    let t = *rng.pick(&THINGS);
    let he = |x: &str| if FEMALE.contains(&x) { "she" } else { "he" };
    let cap = |x: &str| x[..1].to_uppercase() + &x[1..];
    let (text, ans): (String, i64) = match lesson {
        49 => {
            // A = B ± d1, B = C ± d2, C told
            let cv = rng.range(10, 30);
            let (d1, d2) = (rng.range(1, 6), rng.range(1, 6));
            let (m1, m2) = (rng.next() % 2 == 0, rng.next() % 2 == 0);
            let bv = if m2 { cv + d2 } else { cv - d2 };
            let av = if m1 { bv + d1 } else { bv - d1 };
            let w = |m: bool| if m { "more" } else { "fewer" };
            if test {
                (format!("{a} has {d1} {} {t} than {b}, who has {d2} {} {t} than {c}. {c} has {cv} {t}. How many {t} does {a} have?", w(m1), w(m2)), av)
            } else {
                (format!("{c} has {cv} {t}. {b} has {d2} {} {t} than {c}. {a} has {d1} {} {t} than {b}. How many {t} does {a} have?", w(m2), w(m1)), av)
            }
        }
        50 => {
            // A = B + d told with A's count; ask B
            let bv = rng.range(5, 30);
            let d = rng.range(1, 9);
            let more = rng.next() % 2 == 0;
            let av = if more { bv + d } else { bv - d };
            let w = if more { "more" } else { "fewer" };
            if test {
                (format!("{a} has {d} {w} {t} than {b}. If {a} has {av} {t}, how many {t} does {b} have?"), bv)
            } else {
                (format!("{a} has {d} {w} {t} than {b}. {a} has {av} {t}. How many {t} does {b} have?"), bv)
            }
        }
        51 => {
            // ★ A = B + d at first; B loses or finds n; A told; ask B now
            let bv = rng.range(8, 30);
            let d = rng.range(1, 9);
            let n = rng.range(1, 7);
            let av = bv + d;
            let lose = rng.next() % 2 == 0;
            let now = if lose { bv - n } else { bv + n };
            let act = if lose { if test { "drops" } else { "loses" } } else if test { "picks up" } else { "finds" };
            let more = if lose { "" } else { " more" };
            (format!("{a} had {d} more {t} than {b}. {a} had {av} {t}. Then {b} {act} {n}{more} {t}. How many {t} does {b} have now?"), now)
        }
        52 => {
            let (l, w) = (rng.range(3, 20), rng.range(2, 12));
            let unit = *rng.pick(&["cm", "metres", "inches", "feet"]);
            if rng.next() % 3 == 0 {
                if test { (format!("Each side of a square garden is {l} {unit}. How long is the fence around it?"), 4 * l) } else { (format!("A square has sides of {l} {unit}. What is its perimeter?"), 4 * l) }
            } else if test {
                (format!("A rectangular field is {l} {unit} long and {w} {unit} wide. How far is it all the way around the field?"), 2 * (l + w))
            } else {
                (format!("A rectangle is {l} {unit} long and {w} {unit} wide. What is its perimeter?"), 2 * (l + w))
            }
        }
        53 => {
            let (l, w) = (rng.range(2, 15), rng.range(2, 12));
            let unit = *rng.pick(&["cm", "metres", "feet"]);
            if rng.next() % 3 == 0 {
                if test { (format!("A square tile has sides of {l} {unit}. How many square {unit} does it cover?"), l * l) } else { (format!("A square has sides of {l} {unit}. What is its area?"), l * l) }
            } else if test {
                (format!("A rug is {l} {unit} long and {w} {unit} wide. How many square {unit} of floor does it cover?"), l * w)
            } else {
                (format!("A rectangle is {l} {unit} long and {w} {unit} wide. What is its area?"), l * w)
            }
        }
        54 => {
            let h1 = rng.range(1, 10);
            let m1 = 5 * rng.range(0, 11);
            let dur = 5 * rng.range(3, 30);
            let tot = h1 * 60 + m1 + dur;
            let (h2, m2) = (tot / 60, tot % 60);
            let hm = |h: i64, m: i64| format!("{h}:{m:02}");
            if test {
                (format!("{a}'s piano lesson began at {} and finished at {}. How many minutes did it last?", hm(h1, m1), hm(h2, m2)), dur)
            } else {
                (format!("A film starts at {} and ends at {}. How many minutes long is the film?", hm(h1, m1), hm(h2, m2)), dur)
            }
        }
        55 => {
            let (k, p) = (rng.range(2, 5), rng.range(2, 6));
            let note = *rng.pick(&[20i64, 50]);
            let item = *rng.pick(&ITEMS);
            if test {
                (format!("{a} buys {k} {item}s at {p} dollars each and pays with a {note}-dollar note. How much change does {a} get back?"), note - k * p)
            } else {
                (format!("{a} has a {note}-dollar note. {a} buys {k} {item}s for {p} dollars each. How many dollars of change does {a} get?"), note - k * p)
            }
        }
        56 => {
            let k = rng.range(3, 8);
            let n = k * rng.range(2, 8) + rng.range(1, k - 1);
            if test {
                (format!("{a} shares {n} {t} equally among {k} friends and keeps the ones that are left. How many does {a} keep?"), n % k)
            } else {
                (format!("{a} has {n} {t} to share equally among {k} children. How many {t} are left over?"), n % k)
            }
        }
        57 => {
            // packs and extras: k packs of m and e more
            let (k, m, e) = (rng.range(2, 6), rng.range(3, 10), rng.range(1, 9));
            if test {
                (format!("{a} has {k} boxes with {m} {t} in each box, and {e} loose {t}. How many {t} does {a} have in all?"), k * m + e)
            } else {
                (format!("{a} buys {k} packs of {m} {t} and {e} more {t}. How many {t} does {a} have?"), k * m + e)
            }
        }
        58 => {
            // ★ the change in the middle unknown: had x, gave some, found f, now y
            let x = rng.range(10, 30);
            let g = rng.range(1, x - 2);
            let f = rng.range(1, 9);
            let now = x - g + f;
            let p = cap(he(a));
            if test {
                (format!("{a} started with {x} {t}. After giving some to {b} and then finding {f} more, {a} had {now} {t}. How many {t} did {a} give to {b}?"), g)
            } else {
                (format!("{a} had {x} {t}. {p} gave some {t} to {b}. Then {} found {f} more {t}. Now {} has {now} {t}. How many {t} did {a} give to {b}?", he(a), he(a)), g)
            }
        }
        59 => {
            let k = rng.range(2, 4);
            let avg = rng.range(4, 15);
            let mut v: Vec<i64> = (0..k).map(|_| avg + rng.range(-3, 3)).collect();
            let s: i64 = v.iter().sum();
            let last = avg * (k + 1) - s;
            v.push(last);
            if last < 0 {
                v[0] += -last;
            }
            let tot: i64 = v.iter().sum();
            let avg = if tot % v.len() as i64 == 0 { tot / v.len() as i64 } else { tot / v.len() as i64 };
            let list = v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
            let joined = format!("{} and {}", list[..list.len() - 1].join(", "), list[list.len() - 1]);
            if tot % v.len() as i64 != 0 {
                return Problem { text: String::new(), a: -1, b: 0, op: Op::Add };
            }
            if test {
                (format!("In {} games {a} scored {joined} points. What was {a}'s mean score per game?", v.len()), avg)
            } else {
                (format!("{a} scored {joined} points in {} games. What is the average score?", v.len()), avg)
            }
        }
        _ => {
            // 60: ★★ review — a chain, then a change, then a share
            let cv = rng.range(6, 20);
            let d = rng.range(2, 8);
            let bv = cv + d;
            let k = rng.range(2, 4);
            let bv = bv - bv % k + k;
            let cv = bv - d;
            (format!("{c} has {cv} {t}. {b} has {d} more {t} than {c}. {b} shares all of {}'s {t} equally among {k} friends. How many {t} does each friend get?", he(b).replace("she", "her").replace("he", "his").replace("his", b)), bv / k)
        }
    };
    let _ = c;
    Problem { text, a: ans, b: 0, op: Op::Add }
}

fn scenario_once(rng: &mut Rng, lesson: usize, test: bool) -> Problem {
    if lesson >= 49 {
        return scenario5(rng, lesson, test);
    }
    if lesson >= 27 {
        return scenario4(rng, lesson, test);
    }
    if lesson >= 21 {
        return scenario3(rng, lesson, test);
    }
    if lesson >= 15 {
        return scenario2(rng, lesson, test);
    }
    let mut names: Vec<&str> = NAMES.to_vec();
    // a fixed shuffle by the rng
    for i in (1..names.len()).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        names.swap(i, j);
    }
    let actors = match lesson { 14 => 4, l if l >= 10 => 3, _ => 2 };
    let t = *rng.pick(&THINGS);
    let mut hold: Vec<i64> = (0..actors).map(|_| rng.range(3, 12)).collect();
    let mut text = String::new();
    for (i, h) in hold.iter().enumerate() {
        text += &format!("{} has {h} {t}. ", names[i]);
    }
    let say = |e: &SEv, rng: &mut Rng| -> String {
        let pool: Vec<&str> = match e.act {
            Act::Give if lesson == 13 => if test { GIVE_SAY[GIVE_TEST_FROM..].to_vec() } else { GIVE_SAY[..GIVE_TEST_FROM].to_vec() },
            Act::Give => if test { GIVE_SAY[1..3].to_vec() } else { GIVE_SAY[..2].to_vec() },
            Act::Find => if test { FIND_SAY[2..].to_vec() } else { FIND_SAY[..2].to_vec() },
            Act::Lose => if test { LOSE_SAY[2..].to_vec() } else { LOSE_SAY[..2].to_vec() },
        };
        rng.pick(&pool).replace("{a}", names[e.who]).replace("{b}", names[e.to]).replace("{n}", &e.n.to_string()).replace("{t}", t)
    };
    let steps = if lesson == 14 { 4 } else { 2 };
    let mut evs: Vec<SEv> = Vec::new();
    for k in 0..steps {
        let who = (rng.next() % actors as u64) as usize;
        let act = match (lesson, k) {
            (12, 1) => evs[0].act,
            (9, 0) => Act::Find,
            (9, _) => Act::Give,
            _ => match rng.next() % 4 { 0 => Act::Find, 1 => Act::Lose, _ => Act::Give },
        };
        let to = (who + 1 + (rng.next() % (actors as u64 - 1)) as usize) % actors;
        let n = rng.range(1, hold[who].max(2) - 1).max(1);
        let e = if lesson == 12 && k == 1 {
            // «Ben does the same»: the first action again, by another actor
            let first = evs[0].clone();
            let other = (0..actors).find(|&i| i != first.who && i != first.to).unwrap_or((first.who + 1) % actors);
            SEv { who: other, ..first }
        } else {
            SEv { act, who, to, n }
        };
        if lesson == 12 && k == 1 {
            text += &(if test { format!("So does {}. ", names[e.who]) } else { format!("{} does the same. ", names[e.who]) });
        } else {
            text += &format!("{} ", say(&e, rng));
        }
        match e.act {
            Act::Give => {
                hold[e.who] -= e.n;
                hold[e.to] += e.n;
            }
            Act::Find => hold[e.who] += e.n,
            Act::Lose => hold[e.who] -= e.n,
        }
        evs.push(e);
    }
    // the question
    let (q, ans) = match lesson {
        11 => {
            let (x, y) = if hold[0] >= hold[1] { (0, 1) } else { (1, 0) };
            (format!("How many more {t} does {} have than {}?", names[x], names[y]), hold[x] - hold[y])
        }
        10 if rng.next() % 2 == 0 => (format!("How many {t} do they have in all?"), hold.iter().sum()),
        _ => {
            let x = if lesson == 14 { (rng.next() % actors as u64) as usize } else { evs.last().map(|e| if e.act == Act::Give { e.to } else { e.who }).unwrap_or(0) };
            (format!("How many {t} does {} have now?", names[x]), hold[x])
        }
    };
    text += &q;
    Problem { text, a: ans, b: 0, op: Op::Add }
}

/// The scenario lessons' titles.
pub const SCENARIO_TITLES: [(usize, &str); 42] = [(9, "Two steps"), (10, "Three actors"), (11, "Comparing after passing things"), (12, "★ «does the same», «so does»"), (13, "★ Paraphrases of giving"), (14, "★★ Four actors, four steps, any of them asked"), (15, "More than, fewer than"), (16, "Twice as many, half as many"), (17, "To each of several"), (18, "Halves and doubles"), (19, "★ The rest"), (20, "★ How many do some of them have together"), (21, "Prices and money"), (22, "Every day, in a week"), (23, "Full boxes and what is left over"), (24, "Thirds and quarters"), (25, "★ Numbers in words, «he» and «she»"), (26, "★★ Packs, a gift, and the rest shared equally"), (27, "Big numbers in words"), (28, "★ Number words with typos"), (29, "Who is in front, who is behind"), (30, "★ From the front and from the back"), (31, "Days on a calendar"), (32, "Hours and minutes"), (33, "Ages"), (34, "The unknown start"), (35, "The unknown change"), (36, "Lengths"), (37, "Pairs and dozens"), (38, "★★ An unknown start, in words, with a typo"), (49, "Chains of comparisons"), (50, "A comparison read backwards"), (51, "★ A change after a comparison"), (52, "Perimeters"), (53, "Areas"), (54, "Minutes between two clock times"), (55, "Change from a note"), (56, "What is left after sharing"), (57, "Packs and a few more"), (58, "★ The change in the middle"), (59, "Averages"), (60, "★★ A chain, then a share")];

pub const SCENARIO_LESSONS: [usize; 42] = [9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60];

/// The scenario lessons for the second student: exercises with the teacher's corrections, then a test.
pub fn scenario_course(ns: &[usize]) -> Result<String> {
    let titles = SCENARIO_TITLES;
    let mut actors = Actors::load();
    let mut out = String::new();
    for &n in ns {
        let title = titles.iter().find(|t| t.0 == n).map(|t| t.1).unwrap_or("?");
        let score = |actors: &Actors, seed: u64, test: bool, k: usize| -> (usize, Vec<String>) {
            let mut rng = Rng(seed);
            let mut bad = Vec::new();
            let mut right = 0;
            for _ in 0..k {
                let p = scenario(&mut rng, n, test);
                let (a, why) = actor_answer(actors, &p);
                if a == Some(p.a) { right += 1 } else if bad.len() < 2 { bad.push(format!("   ✗ {} → {:?} ({why}); right {}", p.text, a, p.a)) }
            }
            (right, bad)
        };
        let (before, _) = score(&actors, 0xC0DE + n as u64, true, 60);
        let votes_before = actors.votes.clone();
        let mut chat = format!("Teacher: Today's lesson: {title}.\n");
        for l in scenario_explain(n) {
            chat += &format!("Teacher: {l}\n");
        }
        let mut rng = Rng(0xFACE + n as u64);
        let (mut first_right, mut fixed) = (0, 0);
        for k in 0..120 {
            let p = scenario(&mut rng, n, false);
            let (a, why) = actor_answer(&actors, &p);
            if k < 12 {
                let fixes: Vec<String> = crate::numbers::normalize(&p.text).1.into_iter().flat_map(|f| f.corrected).map(|(a, b)| format!("«{a}» is «{b}»")).collect();
                let fixes = if fixes.is_empty() { String::new() } else { format!("{}; ", fixes.join(", ")) };
                chat += &format!("Teacher: {}\nStudent: {fixes}{} → {}\n", p.text, say_reading(&reading_of(&p.text)), a.map(|x| x.to_string()).unwrap_or_else(|| format!("I do not know ({why})")));
                chat += &(if a == Some(p.a) { "Teacher: Right!\n".to_string() } else { format!("Teacher: Not quite: it is {}. Look again at who has what after each step.\n", p.a) });
            }
            if a == Some(p.a) {
                first_right += (k < 20) as usize;
            } else {
                fixed += 1;
            }
            // the teacher's answer — confirming or correcting — shows what the verbs must do
            let q = Problem { text: p.text.clone(), a: p.a, b: 0, op: Op::Add };
            actor_learn_answer(&mut actors, &q, p.a);
        }
        let (after, bad) = score(&actors, 0xC0DE + n as u64, true, 60);
        chat += &format!("Teacher: Now the test: 60 problems told in words you have not met in the exercises.\nStudent: {after} of 60 right.\n");
        let mut tr = Rng(0xC0DE + n as u64);
        for _ in 0..4 {
            let p = scenario(&mut tr, n, true);
            let (a, _) = actor_answer(&actors, &p);
            let fixes: Vec<String> = crate::numbers::normalize(&p.text).1.into_iter().flat_map(|f| f.corrected).map(|(a, b)| format!("«{a}» is «{b}»")).collect();
            let fixes = if fixes.is_empty() { String::new() } else { format!("{}; ", fixes.join(", ")) };
            chat += &format!("Teacher: {}\nStudent: {fixes}{} → {}\nTeacher: {}\n", p.text, say_reading(&reading_of(&p.text)), a.map(|x| x.to_string()).unwrap_or("?".into()), if a == Some(p.a) { "Right!".to_string() } else { format!("Not quite: it is {}.", p.a) });
        }
        let mut change = format!("# Lesson {n}: {title} — the change of the student's state\n\nTest before {before}/60, after {after}/60. Exercises 120, the first round {first_right}/20, corrections {fixed}.\n\n| verb | before (votes) | after |\n|---|---|---|\n");
        for (v, m) in &actors.votes {
            if votes_before.get(v) != Some(m) {
                let show = |m: Option<&BTreeMap<Effect, i64>>| m.map(|m| m.iter().map(|(e, c)| format!("{e:?}×{c}")).collect::<Vec<_>>().join(" ")).unwrap_or_else(|| "—".into());
                change += &format!("| {v} | {} | {} |\n", show(votes_before.get(v)), show(Some(m)));
            }
        }
        change += "\nThe words of the skeleton this lesson leans on are measured by switching each off (`world school actors ablate`).\n";
        let dir = crate::state::root().join("school").join(format!("lesson-{n:02}"));
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join("chat.log"), &chat)?;
        std::fs::write(dir.join("state-change.md"), &change)?;
        writeln!(out, "{n}. {title}: test {before}/60 → {after}/60")?;
        if std::env::var("SCHOOL_SHOW").is_ok() {
            for b in bad {
                writeln!(out, "{b}")?;
            }
        }
    }
    actors.save()?;
    Ok(out)
}

/// The teacher's short explanation of a scenario lesson.
fn scenario_explain(n: usize) -> &'static [&'static str] {
    match n {
        9 => &["A story can have two steps. Follow them in order: after each step, ask who has what.", "Example: Tom has 5 apples. Tom finds 2 more: 7. Tom gives 3 to Anna: Tom 4, Anna gets 3 more."],
        10 => &["Now three people. Keep a count for each of them, like three heaps on the table.", "When one gives to another, one heap gets smaller and the other gets bigger by the same number."],
        11 => &["First play the story, then compare: how many more has one than the other? Take the smaller from the bigger."],
        12 => &["«Ben does the same» and «So does Ben» mean: Ben does what was just done, with the same number, to the same person."],
        13 => &["Giving can be said in many ways: «Tom gives Anna 3», «Anna gets 3 from Tom», «Anna receives 3 from Tom», «Anna is given 3 by Tom». In all of them 3 go from Tom to Anna.", "The one after «from» or «by» is the one who gives."],
        14 => &["Four people and four steps. Keep four heaps and change them one step at a time; at the end answer for whoever is asked."],
        15 => &["«Ben has 3 more than Anna»: take Anna's count and add 3. «3 fewer than Anna»: take 3 away from Anna's count."],
        16 => &["«Twice as many as Anna» is Anna's count two times. «Three times as many» — three times. «Half as many» — Anna's count shared into two equal parts, one part."],
        17 => &["«Tom gives 2 to each of his 3 friends»: each friend gets 2, so Tom gives 2 × 3 = 6."],
        18 => &["«Half of his apples» depends on how many he has at that moment: count first, then halve.", "«Lily doubles her shells»: she gets as many again as she has."],
        19 => &["«The rest» is everything still left with the person. «Tom gives 3 to Anna and the rest to Ben»: first take 3 away, then all that remains goes to Ben.", "«Keeps 3 and gives the rest»: the 3 kept stay, all the others go."],
        20 => &["«How many do Max and Tom have together?» Add only the people named, not everyone in the story."],
        21 => &["A price tells how much one thing costs. To find what many cost, multiply: 4 pencils at 3 cents each cost 4 × 3 = 12 cents.", "What is paid leaves the payer: 20 cents − 12 cents = 8 cents left."],
        22 => &["«3 pages every day» for 5 days is 3 × 5. A week has 7 days, two weeks 14."],
        23 => &["Packing 26 eggs in boxes of 6: 6 + 6 + 6 + 6 = 24 fill 4 boxes, and 2 eggs are left over. 26 ÷ 6 = 4, remainder 2."],
        24 => &["«A third of 12» is 12 shared into 3 equal parts, one part: 4. «Two thirds» — two such parts: 8. A quarter (a fourth) is one of 4 equal parts."],
        25 => &["«He» and «she» point back to the last man or woman named. Numbers can be written in words: «seven» is 7, «twelve» is 12."],
        26 => &["A long problem: first find what was bought (packs × things in a pack), then take away the gift, then share the rest equally."],
        27 => &["Numbers can be written in words: «fifty-five» is 55, «one hundred and twenty» is 120, «three hundred forty one» is 341. Read the number first, then the story."],
        28 => &["Sometimes a number word is misspelt: «fivty», «tne», «seventeeen». Find the number word it is closest to, and read the story with it."],
        29 => &["In a line of N people, the 6th from the front has 5 people in front of them and N − 6 behind them."],
        30 => &["The 3rd from the front and the 5th from the back: 2 people before, 4 after, and the person themself: 2 + 4 + 1 = 3 + 5 − 1 = 7."],
        31 => &["From the 3rd to the 9th of May: count both days, 9 − 3 + 1 = 7 days."],
        32 => &["A film that starts at 3 o'clock and lasts 2 hours ends at 5 o'clock. After 12 o'clock comes 1 o'clock again.", "An hour has 60 minutes, so 3 hours have 3 × 60 = 180 minutes."],
        33 => &["«Anna is 3 years older than Tom»: Anna's age is Tom's age + 3; «younger»: − 3.", "«In 5 years» everyone is 5 years older; «2 years ago» everyone was 2 years younger."],
        34 => &["«Tom had some apples. He gave 3 away. Now he has 5.» Go backwards: before giving 3 he had 5 + 3 = 8."],
        35 => &["«Tom had 5. He found some more. Now he has 9.» What he found is the difference: 9 − 5 = 4."],
        36 => &["Lengths are counted like things: a rope of 12 metres with 5 metres cut off is 7 metres long.", "«The blue ribbon is 3 cm longer» — longer than the ribbon just told."],
        37 => &["A pair is 2, a dozen is 12, half a dozen is 6: 3 pairs of socks are 6 socks, 2 dozen eggs are 24 eggs."],
        38 => &["Read the numbers (in words, maybe misspelt), change the dozens into things, then go backwards from the end to the start."],
        _ => &[],
    }
}

/// The student's reading of a problem in its own words: what it holds as the world, step by step.
fn say_reading(r: &Reading) -> String {
    let mut v: Vec<String> = Vec::new();
    for (who, t, n) in &r.holds {
        v.push(format!("{who} {n} {t}"));
    }
    for (item, p) in &r.prices {
        v.push(format!("a {item} {p} cents"));
    }
    for e in &r.events {
        let n = match e.frac { Some((1, 1)) if e.keep > 0 => format!("all but {}", e.keep), Some((1, 1)) => if e.more { "as many again".into() } else { "all the rest".into() }, Some((p, q)) => format!("{p}/{q}"), None => e.n.to_string() };
        v.push(format!("{} {} {n}{}", e.subj, e.verb, if e.to.is_empty() { String::new() } else { format!(" → {}", e.to) }));
    }
    for (k, m) in &r.groups {
        v.push(format!("{k} groups of {m}"));
    }
    if let Some((n, k)) = r.pack {
        v.push(format!("{n} into boxes of {k}"));
    }
    if let Some(x) = r.rate {
        v.push(format!("{x} a day for {} days", r.days.unwrap_or(0)));
    }
    if let Some((who, k)) = &r.share_rest {
        v.push(format!("{who}'s rest shared by {k}"));
    }
    if let Some(n) = r.line {
        v.push(format!("a line of {n}"));
    }
    if let Some(f) = r.front {
        v.push(format!("{f} from the front"));
    }
    if let Some(b) = r.back {
        v.push(format!("{b} from the back"));
    }
    if let Some((a, b)) = r.span {
        v.push(format!("days {a} to {b}"));
    }
    if let Some((h, d)) = r.clock {
        v.push(format!("{h} o'clock + {d} hours"));
    }
    if let Some(h) = r.ask_minutes {
        v.push(format!("{h} hours in minutes"));
    }
    if r.shift != 0 {
        v.push(format!("{:+} years", r.shift));
    }
    if let Some(w) = &r.unknown_start {
        v.push(format!("{w} started with an unknown number"));
    }
    if let Some((w, n)) = &r.now {
        v.push(format!("now {w} {n}"));
    }
    if r.events.iter().any(|e| e.unknown) {
        v.push("one change unknown".into());
    }
    format!("I read: {}", v.join("; "))
}

/// The lessons written as data (`world/data/school/lessons`): for each, its explanation, 120 exercises with the
/// teacher's answers (the student learns from every one), and 60 test problems told only in the test's phrasings.
pub fn data_course(ns: &[usize]) -> Result<String> {
    let mut actors = Actors::load();
    let mut out = String::new();
    let (mut all_after, mut all_n) = (0, 0);
    let (mut exam_all, mut exam_n) = (0, 0);
    // the lessons are a book too: the student answers the book's way where its world has no answer, and the choice of
    // operations learns from every exercise of the lessons so far (with the book's train part it already knew)
    let mut learner = crate::opchoice::Learner::default();
    let first = learner.add(&crate::opchoice::book(std::path::Path::new(&format!("{}/research/raw/mwpdata-svamp/svamp-train.json", mova_data_dir()))));
    learner.run(first, 10);
    let use_ops = std::env::var("MOVA_LESSON_OPS").map(|v| v != "0").unwrap_or(true);
    if use_ops {
        set_lessons(true);
    }
    for l in crate::lessons::all()?.into_iter().filter(|l| ns.is_empty() || ns.contains(&l.n)) {
        let title = format!("{}{}", "★".repeat(l.stars), l.title);
        let test = l.test(60);
        let score = |actors: &Actors| test.iter().filter(|d| actor_answer(actors, &Problem { text: d.text.clone(), a: d.answer, b: 0, op: Op::Add }).0 == Some(d.answer)).count();
        let t0 = std::time::Instant::now();
        let before = score(&actors);
        let t_test = t0.elapsed();
        let mut chat = format!("Teacher: Today's lesson: {title}.\n");
        for e in &l.explain {
            chat += &format!("Teacher: {e}\n");
        }
        let (mut first_right, mut fixed) = (0, 0);
        let (mut t_ans, mut t_learn) = (std::time::Duration::ZERO, std::time::Duration::ZERO);
        for (k, d) in l.exercises(120).iter().enumerate() {
            let p = Problem { text: d.text.clone(), a: d.answer, b: 0, op: Op::Add };
            let ta = std::time::Instant::now();
            let (a, why) = actor_answer(&actors, &p);
            t_ans += ta.elapsed();
            if k < 12 {
                chat += &format!("Teacher: {}\nStudent: {} → {}\n", d.text, say_reading(&reading_of(&d.text)), a.map(|x| x.to_string()).unwrap_or_else(|| format!("I do not know ({why})")));
                chat += &(if a == Some(d.answer) { "Teacher: Right!\n".to_string() } else { format!("Teacher: Not quite: it is {}.\n", d.answer) });
            }
            if a == Some(d.answer) {
                first_right += (k < 20) as usize;
            } else {
                fixed += 1;
            }
            let tl = std::time::Instant::now();
            actor_learn_answer(&mut actors, &p, d.answer);
            t_learn += tl.elapsed();
        }
        let tops = std::time::Instant::now();
        if use_ops {
            // the lesson's own exercises, learned on top of what was learned before
            let new = learner.add(&l.exercises(120).into_iter().map(|d| (d.text, d.answer)).collect::<Vec<_>>());
            learner.run(new, 5);
            crate::opchoice::set(learner.weights());
        }
        let t_ops = tops.elapsed();
        let after = score(&actors);
        if std::env::var("SCHOOL_TIME").is_ok() {
            eprintln!("lesson {}: test {:?}, answers {:?}, verbs {:?}, operations {:?}", l.n, t_test, t_ans, t_learn, t_ops);
        }
        all_after += after;
        all_n += test.len();
        // the final exam: the same problems in formal, official language (the lessons with `exam:` lines)
        let exam = l.exam(30);
        let exam_right = exam.iter().filter(|d| actor_answer(&actors, &Problem { text: d.text.clone(), a: d.answer, b: 0, op: Op::Add }).0 == Some(d.answer)).count();
        exam_all += exam_right;
        exam_n += exam.len();
        chat += &format!("Teacher: Now the test: {} problems told in words you have not met in the exercises.\nStudent: {after} of {} right.\n", test.len(), test.len());
        for d in test.iter().take(4) {
            let (a, _) = actor_answer(&actors, &Problem { text: d.text.clone(), a: d.answer, b: 0, op: Op::Add });
            chat += &format!("Teacher: {}\nStudent: {} → {}\nTeacher: {}\n", d.text, say_reading(&reading_of(&d.text)), a.map(|x| x.to_string()).unwrap_or("?".into()), if a == Some(d.answer) { "Right!".to_string() } else { format!("Not quite: it is {}.", d.answer) });
        }
        let change = format!("# Lesson {}: {title} — the change of the student's state\n\nTest before {before}/{}, after {after}/{}. Exercises 120, the first round {first_right}/20, corrections {fixed}.\n", l.n, test.len(), test.len());
        let dir = crate::state::root().join("school").join(format!("lesson-{:03}", l.n));
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join("chat.log"), &chat)?;
        std::fs::write(dir.join("state-change.md"), &change)?;
        writeln!(out, "{}. {title}: test {before}/{} → {after}/{}", l.n, test.len(), test.len())?;
        if std::env::var("SCHOOL_SHOW").is_ok() {
            for d in test.iter().filter(|d| actor_answer(&actors, &Problem { text: d.text.clone(), a: d.answer, b: 0, op: Op::Add }).0 != Some(d.answer)).take(2) {
                let (a, why) = actor_answer(&actors, &Problem { text: d.text.clone(), a: d.answer, b: 0, op: Op::Add });
                writeln!(out, "   ✗ {} → {a:?} ({}); right {}", d.text, why.chars().take(140).collect::<String>(), d.answer)?;
            }
        }
    }
    set_lessons(false);
    writeln!(out, "the lessons as data: {all_after} of {all_n} test problems right")?;
    if exam_n > 0 {
        writeln!(out, "the exam in formal language: {exam_all} of {exam_n} right")?;
    }
    actors.save()?;
    if use_ops {
        crate::opchoice::save(&crate::opchoice::current())?;
    }
    Ok(out)
}

/// Learn verb effects from a right answer given directly (scenario lessons).
fn actor_learn_answer(actors: &mut Actors, p: &Problem, right: i64) {
    let r = reading_of(&p.text);
    let verbs: Vec<String> = r.events.iter().filter(|e| e.verb != "pay").map(|e| e.verb.clone()).collect::<BTreeSet<_>>().into_iter().collect();
    if verbs.is_empty() || verbs.len() > 3 {
        return;
    }
    // every assignment of effects that gives the right answer; what they all agree on is what the answer showed
    let mut fits: Vec<BTreeMap<&str, Effect>> = Vec::new();
    for code in 0..EFFECTS.len().pow(verbs.len() as u32) {
        let mut c = code;
        let pick: BTreeMap<&str, Effect> = verbs.iter().map(|v| { let e = EFFECTS[c % EFFECTS.len()]; c /= EFFECTS.len(); (v.as_str(), e) }).collect();
        if solve(&r, &|v| pick.get(v).copied()) == Some(right) {
            fits.push(pick);
        }
    }
    // each side of each verb gets a vote when every fitting meaning agrees on it
    for v in &verbs {
        let subj: BTreeSet<i64> = fits.iter().map(|f| f[v.as_str()].0).collect();
        let recv: BTreeSet<i64> = fits.iter().map(|f| f[v.as_str()].1).collect();
        let a = if subj.len() == 1 { *subj.first().unwrap() } else { NONE };
        let b = if recv.len() == 1 { *recv.first().unwrap() } else { NONE };
        if fits.is_empty() || (a == NONE && b == NONE) {
            continue;
        }
        if std::env::var("SCHOOL_VOTES").is_ok_and(|x| x == *v) {
            eprintln!("vote {v} ({a}, {b}): {} = {right}", p.text);
        }
        *actors.votes.entry(v.clone()).or_default().entry((a, b)).or_default() += 1;
    }
}

/// The held-out check: math word problems the school never saw (SVAMP's json: Body, Question, Answer), read by the
/// second student. Right, answered, and a negative control — the same answers checked against the next problem's
/// answer (a reader that guesses common numbers would score there too).
pub fn held_out(path: &std::path::Path, show: usize) -> Result<String> {
    let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    let items: Vec<(String, f64, String)> = v.as_array().map(|a| a.iter().filter_map(|x| Some((format!("{} {}", x["Body"].as_str()?, x["Question"].as_str()?), x["Answer"].as_f64()?, x["Type"].as_str().unwrap_or("").to_string()))).collect()).unwrap_or_default();
    let actors = Actors::load();
    set_book(true);
    let (mut right, mut answered, mut control) = (0, 0, 0);
    let mut sure = (0usize, 0usize);
    let mut by_type: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut out = String::new();
    let mut shown = 0;
    for (k, (text, ans, ty)) in items.iter().enumerate() {
        let p = Problem { text: text.clone(), a: 0, b: 0, op: Op::Add };
        let (a, why) = actor_answer(&actors, &p);
        let t = by_type.entry(ty.clone()).or_default();
        t.1 += 1;
        if let Some(a) = a {
            answered += 1;
            if why.starts_with("I am sure") {
                sure.0 += 1;
                sure.1 += (a as f64 == *ans) as usize;
            }
            if a as f64 == *ans {
                right += 1;
                t.0 += 1;
            } else if shown < show {
                shown += 1;
                writeln!(out, "  ✗ {text} → {a} ({}); right {ans}", why.chars().take(120).collect::<String>())?;
            }
            if a as f64 == items[(k + 1) % items.len()].1 {
                control += 1;
            }
        }
    }
    set_book(false);
    let mut head = format!("held out {}: {right} right ({}%), {answered} answered; negative control {control}; sure {} times, right {} of them\n", items.len(), 100 * right / items.len().max(1), sure.0, sure.1);
    for (t, (r, n)) in by_type {
        head += &format!("  {t}: {r}/{n}\n");
    }
    Ok(head + &out)
}

/// The course from books (lessons 39–): each lesson gathers the problems of one kind from a real source — SVAMP's
/// train part for the exercises, its test part (never read while the reader is built) for the test. The teacher's
/// answers are the book's.
pub const BOOK_LESSONS: [(usize, &str); 10] = [
    (39, "Two kinds of things: how many more of one than of the other"),
    (40, "Some: the unknown start and the unknown change"),
    (41, "What is left"),
    (42, "Each, every, per: equal groups"),
    (43, "Equal shares"),
    (44, "In all: adding what is told"),
    (45, "Before and after: going backwards"),
    (46, "Ages, heights and lengths"),
    (47, "The old primers: one step"),
    (48, "The old primers: two steps"),
];

/// The old primers' problems (`world primers extract`): every other one for the exercises, the rest for the test;
/// a lesson of one step or of two, by what one operation on two told quantities can reach.
fn primer_items() -> (Vec<(String, i64, String)>, Vec<(String, i64, String)>) {
    let all = book_items(std::path::Path::new(&format!("{}/raw/primers/primers.json", mova_data_dir())));
    let tag = |x: &(String, i64, String)| -> (String, i64, String) {
        let ms = crate::mentions::mentions_all(&x.0);
        let one = crate::opchoice::candidates(&ms).iter().any(|c| c.then.is_none() && c.value == x.1);
        (x.0.clone(), x.1, if one { "primer one".into() } else { "primer two".into() })
    };
    (all.iter().step_by(2).map(tag).collect(), all.iter().skip(1).step_by(2).map(tag).collect())
}

fn book_items(path: &std::path::Path) -> Vec<(String, i64, String)> {
    let v: serde_json::Value = std::fs::read_to_string(path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    v.as_array().map(|a| a.iter().filter_map(|x| {
        let ans = x["Answer"].as_f64()?;
        if ans.fract() != 0.0 {
            return None;
        }
        Some((format!("{} {}", x["Body"].as_str()?, x["Question"].as_str()?), ans as i64, x["Type"].as_str().unwrap_or("").to_string()))
    }).collect()).unwrap_or_default()
}

/// Which book lesson a problem belongs to (the first that fits), by its words and its kind.
fn book_lesson_of(text: &str, ty: &str) -> Option<usize> {
    match ty {
        "primer one" => return Some(47),
        "primer two" => return Some(48),
        _ => {}
    }
    let t = text.to_lowercase();
    let q = t.rsplit_once(". ").map(|x| x.1).unwrap_or(&t);
    let has = |w: &str| t.split(|c: char| !c.is_alphanumeric()).any(|x| x == w);
    if q.contains(" than ") && q.contains("more") {
        return Some(39);
    }
    if has("some") {
        return Some(40);
    }
    if ty.starts_with("Common-Div") && (has("each") || has("per") || has("every") || has("equal")) {
        return Some(43);
    }
    if ty == "Multiplication" || ((has("each") || has("per") || has("every")) && ty != "Subtraction") {
        return Some(42);
    }
    if ["old", "older", "younger", "taller", "shorter", "long", "inches", "feet", "tall"].iter().any(|w| has(w)) {
        return Some(46);
    }
    if has("first") || has("before") || has("begin") || has("start") {
        return Some(45);
    }
    if has("left") || has("remain") || has("remaining") {
        return Some(41);
    }
    if ty == "Addition" {
        return Some(44);
    }
    None
}

/// `world school actors books [lessons…]`: the book lessons — exercises from SVAMP's train part with the teacher's
/// answers (the student learns its verbs from them), then the test from SVAMP's test part.
pub fn book_course(ns: &[usize]) -> Result<String> {
    let mut train = book_items(std::path::Path::new(&format!("{}/research/raw/mwpdata-svamp/svamp-train.json", mova_data_dir())));
    let mut test = book_items(std::path::Path::new(&format!("{}/research/raw/mwpdata-svamp/svamp-test.json", mova_data_dir())));
    let (ptrain, ptest) = primer_items();
    train.extend(ptrain);
    test.extend(ptest);
    let mut actors = Actors::load();
    let mut out = String::new();
    let (mut all_right, mut all_n) = (0, 0);
    let mut seen: Vec<(String, i64)> = Vec::new();
    crate::opchoice::set(crate::opchoice::Weights::default());
    set_book(true);
    for &(n, title) in BOOK_LESSONS.iter().filter(|l| ns.is_empty() || ns.contains(&l.0)) {
        let ex: Vec<&(String, i64, String)> = train.iter().filter(|x| book_lesson_of(&x.0, &x.2) == Some(n)).collect();
        let te: Vec<&(String, i64, String)> = test.iter().filter(|x| book_lesson_of(&x.0, &x.2) == Some(n)).collect();
        let score = |actors: &Actors, set: &[&(String, i64, String)]| set.iter().filter(|x| actor_answer(actors, &Problem { text: x.0.clone(), a: 0, b: 0, op: Op::Add }).0 == Some(x.1)).count();
        let before = score(&actors, &te);
        let mut chat = format!("Teacher: Today's lesson: {title}. The problems are from a book of problems; the answers are the book's.\n");
        let mut ex_right = 0;
        for (k, x) in ex.iter().enumerate() {
            let p = Problem { text: x.0.clone(), a: x.1, b: 0, op: Op::Add };
            let (a, _) = actor_answer(&actors, &p);
            ex_right += (a == Some(x.1)) as usize;
            // the developer's view: the exercises only (the test part stays unread)
            if a != Some(x.1) && std::env::var("SCHOOL_DEV").is_ok_and(|d| d == n.to_string()) {
                eprintln!("✗ {} → {:?}; right {}\n   {}", x.0, a, x.1, say_reading(&reading_of(&x.0)));
            }
            if k < 10 {
                chat += &format!("Teacher: {}\nStudent: {} → {}\nTeacher: {}\n", x.0, say_reading(&reading_of(&x.0)), a.map(|v| v.to_string()).unwrap_or("I do not know".into()), if a == Some(x.1) { "Right!".to_string() } else { format!("Not quite: it is {}.", x.1) });
            }
            // a book's problem teaches the choice of the operation, not the verbs: its readings are too often
            // partial for the votes of the verbs (the school's lessons teach those)
            let _ = &mut actors;
        }
        // the operations: learned anew from every exercise of the course so far
        seen.extend(ex.iter().map(|x| (x.0.clone(), x.1)));
        crate::opchoice::set(crate::opchoice::train(&seen, 10));
        let after = score(&actors, &te);
        all_right += after;
        all_n += te.len();
        chat += &format!("Teacher: Now the test: {} problems from the other part of the book.\nStudent: {after} of {} right.\n", te.len(), te.len());
        let change = format!("# Lesson {n}: {title} — the change of the student's state\n\nExercises {} from the book (right at first sight {ex_right}), test {} problems never seen: {before} → {after}.\n", ex.len(), te.len());
        let dir = crate::state::root().join("school").join(format!("lesson-{n:02}"));
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join("chat.log"), &chat)?;
        std::fs::write(dir.join("state-change.md"), &change)?;
        writeln!(out, "{n}. {title}: exercises {ex_right}/{}, test {before}/{} → {after}/{}", ex.len(), te.len(), te.len())?;
    }
    set_book(false);
    // after the course, the operations learned from the whole train part of the book (the problems no lesson took too)
    crate::opchoice::set(crate::opchoice::train(&train.iter().map(|x| (x.0.clone(), x.1)).collect::<Vec<_>>(), 10));
    writeln!(out, "the book course: {all_right} of {all_n} test problems right")?;
    actors.save()?;
    crate::opchoice::save(&crate::opchoice::current())?;
    Ok(out)
}

/// The school's exercises, as the lessons give them (the same seeds): the problems and the teachers' answers.
fn school_exercises() -> Vec<(String, i64)> {
    let mut v = Vec::new();
    for l in lessons() {
        let mut rng = Rng(0x5EED_0000 + l.n as u64);
        let tm: Vec<&Tmpl> = PRACTICE.iter().filter(|t| fits(t, &l)).collect();
        for i in 0..80 {
            let p = make(&mut rng, tm[i % tm.len()], l.max);
            v.push((p.text.clone(), p.answer()));
        }
    }
    for n in SCENARIO_LESSONS {
        let mut rng = Rng(0xFACE + n as u64);
        for _ in 0..120 {
            let p = scenario(&mut rng, n, false);
            v.push((p.text, p.a));
        }
    }
    for l in crate::lessons::all().unwrap_or_default() {
        v.extend(l.exercises(120).into_iter().map(|d| (d.text, d.answer)));
    }
    v
}

/// Two-fold cross-validation on the book's train part (the test part stays unread): the operation chooser learns
/// on one half, and on the other half each way of answering is counted — the world first, the told quantities, the
/// learned operation alone.
/// `world school actors big [n]`: a pilot of the small one asking the big one (Opus, through the subscription, medium
/// effort) where it is not sure. One fold of SVAMP's train part is held out; the chooser learns on the rest; of the
/// held-out problems it is not sure of (margin below SURE), n go to the big one in batches of 10, each with the small
/// one's reading: the told quantities and its three best expressions. Counts: the small one alone, the big one, and how
/// often the big one took one of the small one's expressions.
pub fn big_pilot(n: usize) -> Result<String> {
    let train = crate::opchoice::book(std::path::Path::new(&format!("{}/research/raw/mwpdata-svamp/svamp-train.json", mova_data_dir())));
    let learn: Vec<(String, i64)> = train.iter().enumerate().filter(|(i, _)| i % 5 != 0).map(|x| x.1.clone()).collect();
    let w = crate::opchoice::train(&learn, 10);
    let unsure: Vec<&(String, i64)> = train.iter().enumerate().filter(|(i, _)| i % 5 == 0).map(|x| x.1).filter(|(t, _)| crate::opchoice::margin(&w, t) < crate::opchoice::SURE).take(n).collect();
    let dir = std::path::PathBuf::from(format!("{}/runs/big-pilot", mova_data_dir()));
    std::fs::create_dir_all(&dir)?;
    let vmm = math::vmm::Vmm::new(dir.join("empty"), dir.join("calls.jsonl"), 5.0)?;
    let (mut small, mut big, mut took, mut asked) = (0, 0, 0, 0);
    let mut log = String::new();
    for (bi, chunk) in unsure.chunks(10).enumerate() {
        let mut prompt = String::from("You help a small transparent model solve school word problems. For each problem it gives its reading: the quantities it found and its three best expressions (best first). Solve each problem yourself. Answer ONLY with lines `id<TAB>answer<TAB>k`, where answer is a whole number and k is 1, 2 or 3 if your answer equals that expression's value, else 0.\n\n");
        let tops: Vec<Vec<(i64, String, i64)>> = chunk.iter().map(|(t, _)| crate::opchoice::top(&w, t, 3)).collect();
        for (i, ((t, _), tp)) in chunk.iter().zip(&tops).enumerate() {
            let qs: Vec<String> = crate::mentions::mentions_all(t).iter().map(|m| format!("{} {}", m.value, m.thing_word)).collect();
            prompt += &format!("id {i}: {t}\n  quantities: {}\n  expressions: {}\n\n", qs.join(", "), tp.iter().enumerate().map(|(k, x)| format!("{}) {} = {}", k + 1, x.1, x.0)).collect::<Vec<_>>().join("; "));
        }
        let (out, call) = vmm.ask("svamp-unsure", bi, chunk.len(), &prompt, 0.5)?;
        for (i, (t, a)) in chunk.iter().enumerate() {
            asked += 1;
            let mine = tops[i].first().map(|x| x.0);
            small += (mine == Some(*a)) as usize;
            let line = out.lines().find(|l| l.split('\t').next().map(str::trim) == Some(&i.to_string()));
            let f: Vec<&str> = line.map(|l| l.split('\t').map(str::trim).collect()).unwrap_or_default();
            let theirs: Option<i64> = f.get(1).and_then(|x| x.parse::<f64>().ok()).map(|x| x as i64);
            big += (theirs == Some(*a)) as usize;
            took += f.get(2).is_some_and(|k| *k != "0") as usize;
            log += &format!("{t}\n  right {a}; small {mine:?}; big {theirs:?} (took {})\n", f.get(2).unwrap_or(&"?"));
        }
        log += &format!("call {bi}: {:.0} s, ${:.3}\n", call.secs, call.cost_usd);
    }
    std::fs::write(dir.join("log.txt"), &log)?;
    Ok(format!("{asked} problems the small one was not sure of: the small one right {small}, the big one right {big}; the big one took one of the small one's three expressions {took} times; spent ${:.2}\n", vmm.spent()))
}

/// Where the world reader fails on SVAMP's train part (the world reader alone, as the two-fold check runs it): by the
/// problem's type, answered right, answered wrong, or no answer; and whether its reading placed every told number.
/// Examples to a file. (`world school actors worlddiag <examples.md>`)
pub fn world_diag(examples: &str) -> Result<String> {
    let raw: Vec<serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(format!("{}/research/raw/mwpdata-svamp/svamp-train.json", mova_data_dir()))?)?;
    let actors = Actors::load();
    set_book(false);
    let mut by: BTreeMap<String, [usize; 4]> = BTreeMap::new();
    let mut ex: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for x in &raw {
        let text = if x["Body"].as_str().unwrap_or("").trim_end().ends_with('.') { format!("{} {}", x["Body"].as_str().unwrap_or(""), x["Question"].as_str().unwrap_or("")) } else { format!("{}. {}", x["Body"].as_str().unwrap_or("").trim_end(), x["Question"].as_str().unwrap_or("")) };
        let a = x["Answer"].as_f64().unwrap_or(-1.0) as i64;
        let ty = x["Type"].as_str().unwrap_or("?").replace("Divison", "Division");
        let p = Problem { text: text.clone(), a: 0, b: 0, op: Op::Add };
        let r = reading_of(&text);
        let placed = r.holds.len() + r.events.len();
        let (got, why) = if std::env::var("MOVA_TREEREAD").as_deref() == Ok("1") { crate::treeread::answer(&text).map(|(v, w)| (Some(v), w)).unwrap_or((None, "no schema".into())) } else { actor_answer(&actors, &p) };
        let k = match got {
            Some(v) if v == a => 0,
            Some(_) => 1,
            None => 2,
        };
        if std::env::var("MOVA_TREEREAD").as_deref() == Ok("1") && got.is_some() {
            let e = by.entry(format!("schema {}", why.split(':').next().unwrap_or(""))).or_insert([0; 4]);
            e[k] += 1;
        }
        let e = by.entry(ty.clone()).or_insert([0; 4]);
        e[k] += 1;
        e[3] += (placed == 0) as usize;
        let key = format!("{ty} {}", ["right", "wrong", "no answer"][k]);
        let v = ex.entry(key).or_default();
        if k > 0 && v.len() < 12 {
            v.push(format!("- {text}\n  answer {a} ({}); got {got:?}: {why}\n  reading: holds {:?}; events {:?}; ask whose {:?}, than {:?}", x["Equation"].as_str().unwrap_or(""), r.holds, r.events.iter().map(|e| format!("{} {} {} {} to {}", e.subj, e.verb, e.n, e.thing, e.to)).collect::<Vec<_>>(), r.ask_whose, r.ask_than));
        }
    }
    let mut out = String::from("| type | problems | right | wrong | no answer | nothing read |\n|---|---|---|---|---|---|\n");
    let mut tot = [0usize; 4];
    for (t, c) in &by {
        let n = c[0] + c[1] + c[2];
        out += &format!("| {t} | {n} | {} | {} | {} | {} |\n", c[0], c[1], c[2], c[3]);
        for i in 0..4 {
            tot[i] += c[i];
        }
    }
    out += &format!("| all | {} | {} | {} | {} | {} |\n", tot[0] + tot[1] + tot[2], tot[0], tot[1], tot[2], tot[3]);
    let mut e = String::new();
    for (k, v) in ex {
        e += &format!("## {k}\n\n{}\n\n", v.join("\n"));
    }
    std::fs::write(examples, e)?;
    Ok(out)
}

pub fn cross_check() -> Result<String> {
    cross_check_k(std::env::var("MOVA_FOLDS").ok().and_then(|x| x.parse().ok()).unwrap_or(5))
}

/// The k-fold check: each k-th part of the train part checked by what learned on the rest.
pub fn cross_check_k(k: usize) -> Result<String> {
    let train = crate::opchoice::book(std::path::Path::new(&format!("{}/research/raw/mwpdata-svamp/svamp-train.json", mova_data_dir())));
    let actors = Actors::load();
    // more problems to learn from (MOVA_EXTRA: SVAMP-form json files, comma-separated): only those one or two operations
    // on their told quantities can answer; they join every fold's learning, never its check
    let extra: Vec<(String, i64)> = std::env::var("MOVA_EXTRA").map(|v| v.split(',').flat_map(|p| crate::opchoice::book(std::path::Path::new(p))).filter(|(t, a)| crate::opchoice::candidates(&crate::mentions::mentions_all(t)).iter().any(|c| c.value == *a)).collect()).unwrap_or_default();
    let (mut world, mut book_way, mut ops_only, mut n) = (0, 0, 0, 0);
    let mut margins: Vec<(i64, bool)> = Vec::new();
    // the folds repeated with other splits (MOVA_REPEATS, 3 by default): a problem's fold is (i × a prime + r) mod k,
    // so the measure does not hang on one order of learning
    let repeats: usize = std::env::var("MOVA_REPEATS").ok().and_then(|x| x.parse().ok()).unwrap_or(3);
    let primes = [1usize, 7, 13, 31, 101];
    // grouped (MOVA_GROUPED=1): the variants of one problem — the same first six words with the numbers taken out —
    // stay in one part, so the check measures what carries over to problems not seen in any form
    let grouped = std::env::var("MOVA_GROUPED").is_ok();
    let group_of = |t: &str| -> u64 {
        let key: String = t.to_lowercase().split_whitespace().filter(|w| !w.chars().any(|c| c.is_ascii_digit() || c == '$')).take(6).collect::<Vec<_>>().join(" ");
        key.bytes().fold(1469598103934665603u64, |h, b| (h ^ b as u64).wrapping_mul(1099511628211))
    };
    for (rep, fold) in (0..repeats).flat_map(|r| (0..k).map(move |f| (r, f))) {
        let part = |i: usize| if grouped { ((group_of(&train[i].0) ^ (rep as u64 * 0x9e3779b97f4a7c15)) % k as u64) as usize } else { (i * primes[rep % primes.len()] + rep) % k };
        let learn: Vec<(String, i64)> = train.iter().enumerate().filter(|(i, _)| part(*i) != fold).map(|x| x.1.clone()).chain(extra.iter().cloned()).collect();
        let check: Vec<&(String, i64)> = train.iter().enumerate().filter(|(i, _)| part(*i) == fold).map(|x| x.1).collect();
        let w = crate::opchoice::train(&learn, std::env::var("MOVA_EPOCHS").ok().and_then(|x| x.parse().ok()).unwrap_or(10));
        crate::opchoice::set(w.clone());
        for (t, a) in check {
            n += 1;
            let p = Problem { text: t.clone(), a: 0, b: 0, op: Op::Add };
            set_book(false);
            world += (actor_answer(&actors, &p).0 == Some(*a)) as usize;
            set_book(true);
            let bw = actor_answer(&actors, &p);
            book_way += (bw.0 == Some(*a)) as usize;
            // SCHOOL_CVSHOW=book: the book's way's mistakes, with its reasons
            if std::env::var("SCHOOL_CVSHOW").is_ok_and(|v| v == "book") && bw.0 != Some(*a) {
                eprintln!("✗ {t} → {:?} ({}); right {a}", bw.0, bw.1);
            }
            let o = crate::opchoice::answer(&w, t);
            ops_only += (o.as_ref().map(|x| x.0) == Some(*a)) as usize;
            margins.push((crate::opchoice::margin(&w, t), o.as_ref().map(|x| x.0) == Some(*a)));
            if std::env::var("SCHOOL_CVSHOW").is_ok_and(|v| v != "book") && o.as_ref().map(|x| x.0) != Some(*a) {
                let reach = crate::opchoice::candidates(&crate::mentions::mentions_all(t)).iter().any(|c| c.value == *a);
                eprintln!("✗ [{}] {t} → {}; right {a}", if reach { "chose wrong" } else { "out of reach" }, o.map(|x| x.1).unwrap_or_default());
            }
        }
    }
    set_book(false);
    // the operation alone, answering only when its margin reaches a threshold: answered and right
    let mut curve = String::new();
    for t in [0, 1, 2, 4, 8, 16, 32] {
        let ans: Vec<&(i64, bool)> = margins.iter().filter(|m| m.0 >= t).collect();
        let r = ans.iter().filter(|m| m.1).count();
        curve += &format!("  margin ≥ {t}: answers {}, right {r} ({}%)\n", ans.len(), 100 * r / ans.len().max(1));
    }
    Ok(curve + &format!("train part, {k} folds × {repeats} splits{}, {n} checks (and {} more to learn from): the world reader {world}, the book's way (the told quantities, then the operation) {book_way}, the operation alone {ops_only}\n", if grouped { " by problem" } else { "" }, extra.len()))
}

/// A lesson's exam problems (as `readdiff` draws them): (text, answer).
fn exam_problems(n: usize) -> Vec<(String, i64)> {
    let mut probs: Vec<(String, i64)> = Vec::new();
    if let Some(l) = lessons().into_iter().find(|l| l.n == n) {
        let mut rng = Rng(0x5EED_0000 + l.n as u64 ^ 0xA11);
        let tm: Vec<&Tmpl> = TEST.iter().filter(|t| fits(t, &l)).collect();
        for i in 0..60 {
            let p = make(&mut rng, tm[i % tm.len()], l.max);
            probs.push((p.text.clone(), p.answer()));
        }
    } else if SCENARIO_LESSONS.contains(&n) {
        let mut rng = Rng(0x5EED_0000 + n as u64 ^ 0xA11);
        for _ in 0..60 {
            let p = scenario(&mut rng, n, true);
            probs.push((p.text.clone(), p.a));
        }
    } else if let Some(l) = crate::lessons::all().unwrap_or_default().iter().find(|l| l.n == n) {
        probs = l.test(60).iter().map(|d| (d.text.clone(), d.answer)).collect();
    }
    probs
}

fn lesson_answers(n: usize) -> String {
    let actors = Actors::load();
    exam_problems(n).iter().map(|(t, a)| {
        let (g, why) = actor_answer(&actors, &Problem { text: t.clone(), a: *a, b: 0, op: Op::Add });
        format!("{}\t{t}\t{a}\t{g:?}\t{why}\n", if g == Some(*a) { "+" } else { "-" })
    }).collect()
}


/// The ablation of the skeleton's words: the exam of every lesson with each word switched off in turn — how much of
/// the score rests on it.
pub fn ablate() -> Result<String> {
    let actors = Actors::load();
    let all = lessons();
    let data_tests: Vec<(usize, Vec<crate::lessons::Drawn>)> = crate::lessons::all().unwrap_or_default().iter().map(|l| (l.n, l.test(60))).collect();
    let ns: Vec<usize> = all.iter().map(|l| l.n).chain(SCENARIO_LESSONS).chain(data_tests.iter().map(|x| x.0)).collect();
    let total_n: usize = 60 * (all.len() + SCENARIO_LESSONS.len()) + data_tests.iter().map(|x| x.1.len()).sum::<usize>();
    let exam = |actors: &Actors| -> Vec<usize> {
        let mut v: Vec<usize> = all.iter().map(|l| {
            let mut rng = Rng(0x5EED_0000 + l.n as u64 ^ 0xA11);
            let tm: Vec<&Tmpl> = TEST.iter().filter(|t| fits(t, l)).collect();
            (0..60).filter(|i| { let p = make(&mut rng, tm[i % tm.len()], l.max); actor_answer(actors, &p).0 == Some(p.answer()) }).count()
        }).collect();
        for n in SCENARIO_LESSONS {
            let mut rng = Rng(0x5EED_0000 + n as u64 ^ 0xA11);
            v.push((0..60).filter(|_| { let p = scenario(&mut rng, n, true); actor_answer(actors, &p).0 == Some(p.a) }).count());
        }
        for (_, test) in &data_tests {
            v.push(test.iter().filter(|d| actor_answer(actors, &Problem { text: d.text.clone(), a: d.answer, b: 0, op: Op::Add }).0 == Some(d.answer)).count());
        }
        v
    };
    let base = exam(&actors);
    let total: usize = base.iter().sum();
    let mut out = format!("| word switched off | exam (of {total_n}) | lost | lessons that lose |\n|---|---|---|---|\n| — | {total} | 0 | |\n");
    // MOVA_PERLESSON=1: the exam of each lesson, to compare two states of the dictionary or the code
    if std::env::var("MOVA_PERLESSON").is_ok() {
        return Ok(ns.iter().zip(&base).map(|(n, b)| format!("{n} {b}\n")).collect());
    }
    let mut rows = Vec::new();
    for w in SKELETON_WORDS {
        *OFF.lock().unwrap() = Some(w.to_string());
        let r = exam(&actors);
        *OFF.lock().unwrap() = None;
        let t: usize = r.iter().sum();
        let hit: Vec<String> = ns.iter().zip(r.iter().zip(&base)).filter(|(_, (a, b))| a < b).map(|(n, (a, b))| format!("{n} {b}→{a}")).collect();
        rows.push((total as i64 - t as i64, format!("| «{w}» | {t} | {} | {} |", total as i64 - t as i64, hit.join(", "))));
    }
    rows.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    for (_, r) in rows {
        out += &r;
        out.push('\n');
    }
    Ok(out)
}

/// `world school actors <lesson…>`: the second-generation student through the lessons' exercises and tests.
pub fn actors_course(ns: &[usize]) -> Result<String> {
    let mut actors = Actors::load();
    let mut out = String::new();
    let all = lessons();
    for &n in ns {
        let Some(l) = all.iter().find(|l| l.n == n) else { continue };
        let test_score = |actors: &Actors| {
            let mut rng = Rng(0x5EED_0000 + l.n as u64 ^ 0xA11);
            let tm: Vec<&Tmpl> = TEST.iter().filter(|t| fits(t, l)).collect();
            (0..60).filter(|i| { let p = make(&mut rng, tm[i % tm.len()], l.max); actor_answer(actors, &p).0 == Some(p.answer()) }).count()
        };
        let before = test_score(&actors);
        let mut rng = Rng(0x5EED_0000 + n as u64);
        let tm: Vec<&Tmpl> = PRACTICE.iter().filter(|t| fits(t, l)).collect();
        for i in 0..80 {
            let p = make(&mut rng, tm[i % tm.len()], l.max);
            // a right answer the teacher confirms teaches as much as a corrected one
            actor_learn(&mut actors, &p);
        }
        let after = test_score(&actors);
        writeln!(out, "{n}. {}: test {before}/60 → {after}/60", l.title)?;
        if std::env::var("SCHOOL_SHOW").is_ok() {
            let mut rng = Rng(0x5EED_0000 + l.n as u64 ^ 0xA11);
            let tm: Vec<&Tmpl> = TEST.iter().filter(|t| fits(t, l)).collect();
            let mut shown = std::collections::BTreeSet::new();
            for i in 0..60 {
                let p = make(&mut rng, tm[i % tm.len()], l.max);
                let (a, why) = actor_answer(&actors, &p);
                if a != Some(p.answer()) && shown.insert(i % tm.len()) {
                    writeln!(out, "   ✗ {} → {:?} ({why}); right {}\n     {:?}", p.text, a, p.answer(), reading_of(&p.text))?;
                }
            }
        }
    }
    actors.save()?;
    Ok(out)
}

// ---- A course, its exam, a proposal, the graduation into a new state ----------------------------------------------
//
// What the student learns in lessons lives first in the school's scratch files. After a course (a run of lessons) comes
// an exam on every lesson with a threshold, and a negative control (a student who knows nothing must fail). Then the
// student writes a proposal: what it learned, as English sentences with their weights, the words it learned that no
// lesson taught (possibly spurious, like «apple» for subtracting), the exam and the control. The teacher of the
// teachers (the large model) reads it; only then `graduate` makes it a new state: `school.tsv` in the state's files
// (versioned as a delta, rolled back like any rule), a letter, and «What I learned at school» in the brain.

const PASS: usize = 45;

fn scratch_stamp() -> u64 {
    let a = std::fs::read_to_string(Student::path()).unwrap_or_default();
    let b = std::fs::read_to_string(crate::state::root().join("school").join("verbs.tsv")).unwrap_or_default();
    crate::induce::fnv(&format!("{a}|{b}"))
}

/// The school knowledge as one file of the state: words with their operation weights, verbs with their effect.
fn as_state_file() -> String {
    let mut t = String::from("# what the small ones learned at school: kind, word, weights (add, subtract, multiply, divide) or the effect on the table\n");
    for (k, v) in &Student::load().w {
        if v.iter().any(|&x| x != 0) {
            let _ = writeln!(t, "word\t{k}\t{}\t{}\t{}\t{}", v[0], v[1], v[2], v[3]);
        }
    }
    for l in std::fs::read_to_string(crate::state::root().join("school").join("verbs.tsv")).unwrap_or_default().lines() {
        let _ = writeln!(t, "verb\t{l}");
    }
    t
}

/// The student's knowledge as English sentences (the strongest first), and the words no lesson taught.
fn in_english(s: &Student) -> (Vec<String>, Vec<String>) {
    let taught: std::collections::BTreeSet<&str> = lessons().iter().flat_map(|l| l.cues.iter().map(|c| c.0)).collect();
    let mut rows: Vec<(i64, String, bool)> = Vec::new();
    for (w, v) in &s.w {
        let (i, best) = v.iter().enumerate().max_by_key(|x| *x.1).map(|(i, b)| (i, *b)).unwrap_or((0, 0));
        if best >= 2 {
            rows.push((best, format!("The word «{w}» tells me to {} (weight {best}).", OPS[i].name()), !taught.contains(w.as_str())));
        }
    }
    rows.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let good = rows.iter().filter(|r| !r.2).map(|r| r.1.clone()).collect();
    let odd = rows.iter().filter(|r| r.2).map(|r| r.1.clone()).collect();
    (good, odd)
}

/// A course: lessons from..=to, the exam, the control, the proposal.
pub fn course(from: usize, to: usize) -> Result<String> {
    let mut out = String::new();
    // in the order of the course: the revision of everything comes last
    let order: Vec<usize> = lessons().iter().map(|l| l.n).filter(|&n| n >= from && n <= to).collect();
    let order: Vec<usize> = if from == 0 { std::iter::once(0).chain(order).collect() } else { order };
    for n in order {
        writeln!(out, "{}", if n == 0 { table_lesson()? } else { teach(n)? })?;
    }
    writeln!(out, "{}", write_proposal(from, to, "")?)?;
    Ok(out)
}

/// The proposal for the course from..=to, written from what the student knows now (after lessons, or after the
/// reviewer's pruning, which it lists).
pub fn write_proposal(from: usize, to: usize, pruned: &str) -> Result<String> {
    let mut out = String::new();
    let s = Student::load();
    let fresh = Student::default();
    let mut md = format!("# Proposal: graduate from the school course, lessons {from}–{to}\n\nWritten by the student after the course; to be read by the teacher of the teachers before `world school graduate`.\n\n## Exam (all lessons, after the whole course) and the negative control\n\n| lesson | exam | a student who knows nothing | passed (≥ {PASS}/60) |\n|---|---|---|---|\n");
    let mut all_pass = true;
    for l in lessons().iter().filter(|l| l.n >= from.max(1) && l.n <= to) {
        let seed = 0x5EED_0000 + l.n as u64 ^ 0xA11;
        let (r, n, _) = test(&s, l, seed, 60);
        let (c, _, _) = test(&fresh, l, seed, 60);
        // passed: the threshold, and no worse than a student who knows nothing (a test the empty student passes
        // proves nothing by itself, but must not be failed)
        let pass = r >= PASS && r >= c;
        all_pass &= pass;
        writeln!(md, "| {}. {} | {r}/{n} | {c}/{n} | {} |", l.n, l.title, if pass { "yes" } else { "NO" })?;
    }
    let (good, odd) = in_english(&s);
    writeln!(md, "\n## What I learned (words a lesson taught)\n\n{}\n\n## Words I learned that no lesson taught (possibly spurious)\n\n{}\n", good.iter().map(|x| format!("- {x}")).collect::<Vec<_>>().join("\n"), if odd.is_empty() { "none".to_string() } else { odd.iter().map(|x| format!("- {x}")).collect::<Vec<_>>().join("\n") })?;
    let verbs = std::fs::read_to_string(crate::state::root().join("school").join("verbs.tsv")).unwrap_or_default();
    if !verbs.is_empty() {
        writeln!(md, "## What the verbs do to the table\n\n{}\n", verbs.lines().map(|l| l.split_once('\t').map(|(v, n)| format!("- «{v}» {} the table (weight {n}).", if n.parse::<i64>().unwrap_or(0) > 0 { "puts things on" } else { "takes things off" })).unwrap_or_default()).collect::<Vec<_>>().join("\n"))?;
    }
    if !pruned.is_empty() {
        writeln!(md, "## Pruned by the reviewer\n\n{pruned}\n")?;
    }
    writeln!(md, "## Verdict of the exam\n\ncourse {from} {to}\n\n{}\n\nstamp {:016x}", if all_pass { "Every lesson passed; ready for the teacher's review." } else { "Not every lesson passed: no graduation." }, scratch_stamp())?;
    std::fs::write(crate::state::root().join("school").join("proposal.md"), &md)?;
    write!(out, "proposal: {} (exam {})", crate::state::root().join("school").join("proposal.md").display(), if all_pass { "passed" } else { "NOT passed" })?;
    Ok(out)
}

/// The reviewer prunes false links: the words (and every pair holding them) lose their weights; the exam and the
/// proposal are written anew, so graduation still needs every lesson passed.
pub fn prune(words_: &[String]) -> Result<String> {
    let mut s = Student::load();
    let hit = |k: &str| words_.iter().any(|w| k == w || k.split('+').any(|p| p == w));
    let n = s.w.keys().filter(|k| hit(k)).count();
    s.w.retain(|k, _| !hit(k));
    s.acc.retain(|k, _| !hit(k));
    s.save()?;
    let prop = std::fs::read_to_string(crate::state::root().join("school").join("proposal.md")).unwrap_or_default();
    let (from, to) = prop.lines().find_map(|l| l.strip_prefix("course ")).and_then(|x| x.split_once(' ')).and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?))).unwrap_or((1, 8));
    let note = format!("{} ({} weights removed)", words_.iter().map(|w| format!("«{w}»")).collect::<Vec<_>>().join(", "), n);
    write_proposal(from, to, &note)
}

/// The graduation: only with a fresh proposal whose exam passed — a new state with the school's knowledge.
pub fn graduate(note: &str) -> Result<String> {
    let prop = std::fs::read_to_string(crate::state::root().join("school").join("proposal.md")).context("no proposal: run `world school course` first")?;
    if !prop.contains("Every lesson passed") {
        bail!("the proposal's exam did not pass");
    }
    if !prop.contains(&format!("stamp {:016x}", scratch_stamp())) {
        bail!("the student changed since the proposal: run the course and the review again");
    }
    let file = as_state_file();
    crate::store::write_atomic(&crate::state::active_file("school.tsv"), file.as_bytes())?;
    let (good, _) = in_english(&Student::load());
    let id = crate::state::save_with_letter(&format!("graduated from school: {note}"), None, "school", Some(&format!("I graduated from the school course: {note}.")))?;
    let body = format!("{}\n{}", good.join("\n"), std::fs::read_to_string(crate::state::root().join("school").join("verbs.tsv")).unwrap_or_default().lines().map(|l| l.split_once('\t').map(|(v, n)| format!("The verb «{v}» {} the table (weight {n}).", if n.parse::<i64>().unwrap_or(0) > 0 { "puts things on" } else { "takes things off" })).unwrap_or_default()).collect::<Vec<_>>().join("\n"));
    crate::state::brain_section_of(&id, "What I learned at school", &body)?;
    crate::triggers::activate("training", 0.0, &format!("graduated from school into state {id}: {note}"));
    Ok(format!("graduated: new state {id} — school.tsv in its files, «What I learned at school» in its brain ({} sentences)", good.len()))
}

// ---- The school's pages (the dev site) --------------------------------------------------------------------------------

fn esc(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

const CSS: &str = ":root{--bg:#f7f5f0;--ink:#1c1d21;--muted:#62646c;--line:#e3dfd6;--accent:#0f766e;--panel:#fff;--t:#e6f2f0;--s:#fff4e0;--bad:#b91c1c;--good:#15803d}
@media (prefers-color-scheme:dark){:root{--bg:#141518;--ink:#ebeae6;--muted:#a1a1aa;--line:#2e3036;--accent:#2dd4bf;--panel:#1c1d21;--t:#12302c;--s:#33280f;--bad:#f87171;--good:#4ade80}}
body{margin:0;background:var(--bg);color:var(--ink);font:16px/1.55 system-ui,-apple-system,\"Segoe UI\",Roboto,sans-serif}
.wrap{max-width:900px;margin:0 auto;padding:40px 16px}a{color:var(--accent);text-decoration:none}h1{margin:0 0 .2em}
p.lead{color:var(--muted)}table{border-collapse:collapse;margin:12px 0;width:100%}td,th{border-bottom:1px solid var(--line);padding:6px 8px;text-align:left}
.chat{display:flex;flex-direction:column;gap:6px;margin:16px 0}.m{max-width:78%;padding:8px 12px;border-radius:14px;border:1px solid var(--line)}
.m small{display:block;color:var(--muted);font-size:.75em}.T{background:var(--t);align-self:flex-start}.S{background:var(--s);align-self:flex-end}
.ok{color:var(--good);font-weight:600}.no{color:var(--bad);font-weight:600}.card{background:var(--panel);border:1px solid var(--line);border-radius:14px;padding:12px 16px;margin:10px 0}
pre{white-space:pre-wrap;background:var(--panel);border:1px solid var(--line);border-radius:10px;padding:10px 14px;font-size:.88em}";

fn page(title: &str, body: &str) -> String {
    format!("<!doctype html>\n<html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><meta name=\"robots\" content=\"noindex, nofollow, noarchive\"><title>{} — Mova</title><style>{CSS}</style></head><body><div class=\"wrap\">{body}</div></body></html>\n", esc(title))
}

/// A chat log as bubbles (the first `max` lines, then a note how many more).
fn chat_html(log: &str, max: usize) -> String {
    let lines: Vec<&str> = log.lines().collect();
    let mut h = String::from("<div class=\"chat\">");
    for l in lines.iter().take(max) {
        let (who, text) = l.split_once(": ").unwrap_or(("", l));
        let cls = if who == "Student" { "S" } else { "T" };
        let mark = if text.starts_with("Right!") { "<span class=\"ok\">Right!</span>".to_string() } else if let Some(r) = text.strip_prefix("Not quite") { format!("<span class=\"no\">Not quite</span>{}", esc(r)) } else { esc(text) };
        h += &format!("<div class=\"m {cls}\"><small>{}</small>{mark}</div>", esc(who));
    }
    if lines.len() > max {
        h += &format!("<p class=\"lead\">… and {} more lines in the full log.</p>", lines.len() - max);
    }
    h + "</div>"
}

/// `world school courses <out dir>`: the courses by sciences — for now mathematics, and in it the primary school
/// course: every lesson of the small ones numbered, with its level and stars, the student's last test, and a link to
/// the lesson's page (its chat with the teacher).
pub fn courses_site(out: &std::path::Path) -> Result<()> {
    let school = crate::state::root().join("school");
    // the student's last test of a lesson, from its state change («Test before 3/60, after 45/60»)
    let last = |n: usize| -> String {
        let dirs = [format!("lesson-{n:02}"), format!("lesson-{n:03}")];
        dirs.iter().find_map(|d| std::fs::read_to_string(school.join(d).join("state-change.md")).ok()).and_then(|t| t.split("after ").nth(1).map(|x| x.split([',', '.']).next().unwrap_or("").trim().to_string())).unwrap_or_default()
    };
    let link = |n: usize| -> Option<String> { [format!("lesson-{n:02}"), format!("lesson-{n:03}")].into_iter().find(|d| school.join(d).exists()).map(|d| format!("../../../school/{d}.html")) };
    // (number, title, level, stars)
    let mut all: Vec<(usize, String, String, usize)> = Vec::new();
    for l in lessons() {
        all.push((l.n, l.title.to_string(), "primary-1".into(), 0));
    }
    for (n, t) in SCENARIO_TITLES {
        let stars = t.chars().filter(|c| *c == '★').count();
        all.push((n, t.trim_start_matches(['★', ' ']).to_string(), if n < 27 { "primary-2" } else { "primary-3" }.into(), stars));
    }
    for (n, t) in BOOK_LESSONS {
        all.push((n, format!("{t} (a book of problems)"), "primary-4".into(), 0));
    }
    for l in crate::lessons::all()? {
        all.push((l.n, l.title.clone(), if l.level.is_empty() { "primary".into() } else { l.level.clone() }, l.stars));
    }
    all.sort_by_key(|x| x.0);
    let mut rows = String::new();
    for (n, t, level, stars) in &all {
        let title = format!("{} {}", esc(t), "★".repeat(*stars));
        let t = match link(*n) { Some(h) => format!("<a href=\"{h}\">{title}</a>"), None => title };
        rows += &format!("<tr><td>{n}</td><td>{t}</td><td>{level}</td><td>{}</td></tr>", esc(&last(*n)));
    }
    let primary = out.join("mathematics").join("primary");
    std::fs::create_dir_all(&primary)?;
    std::fs::write(primary.join("index.html"), page("Mathematics — the primary school course", &format!(
        "<p><a href=\"../\">← mathematics</a></p><h1>The primary school course</h1><p class=\"lead\">{} lessons for the small ones, grades 1–4, at the level of strong school programmes (Ukraine, China, Singapore): each lesson explains, gives 120 exercises with the teacher's answers, and tests in phrasings the exercises never used. ★ harder, ★★ hard, ★★★ olympiad.</p><table><tr><th>#</th><th>lesson</th><th>level</th><th>the student's last test</th></tr>{rows}</table>",
        all.len()
    )))?;
    std::fs::write(out.join("mathematics").join("index.html"), page("Mathematics", &format!("<p><a href=\"../\">← the courses</a></p><h1>Mathematics</h1><div class=\"card\"><a href=\"primary/\"><b>The primary school course</b></a><br><small>{} lessons, grades 1–4, with olympiad problems</small></div><p class=\"lead\">Next: middle school, high school, bachelor's, master's, doctoral, researcher, professor, distinguished professor, genius.</p>", all.len())))?;
    // the knowledge courses: their lessons and the student's last test of each
    let knowledge = crate::knowledge::COURSES;
    let mut cards = String::new();
    let mut later = String::new();
    for &(name, dir, title, about) in knowledge {
        let ls = crate::knowledge::course(name).unwrap_or_default();
        let results = std::fs::read_to_string(crate::state::root().join("school").join(name).join("results.txt")).unwrap_or_default();
        let mut rows = String::new();
        for l in &ls {
            let res = results.lines().find(|x| x.starts_with(&format!("{}. ", l.n))).and_then(|x| x.rsplit("→ ").next()).unwrap_or("").to_string();
            rows += &format!("<tr><td>{}</td><td>{} {}</td><td>{}</td><td>{}</td></tr>", l.n, esc(&l.title), "★".repeat(l.stars), esc(&l.level), esc(&res));
        }
        let d = out.join(dir);
        std::fs::create_dir_all(&d)?;
        std::fs::write(d.join("index.html"), page(title, &format!(
            "<p><a href=\"../\">← the courses</a></p><h1>{title}</h1><p class=\"lead\">{} {about}. Each lesson teaches facts and asks about them; the test asks in other words. ★ why and how, ★★ a new situation, ★★★ clever reasoning.</p><table><tr><th>#</th><th>lesson</th><th>level</th><th>the student's last test</th></tr>{rows}</table>",
            ls.len()
        )))?;
        let card = format!("<div class=\"card\"><a href=\"{dir}/\"><b>{title}</b></a><br><small>{} lessons</small></div>", ls.len());
        if crate::knowledge::grade(name) == crate::knowledge::PRIMARY { cards += &card } else { later += &card }
    }
    let later = if later.is_empty() { String::new() } else { format!("<h2>The middle school — later</h2><p>Too hard for a child of 9–10; the small one does not learn these yet.</p>{later}") };
    std::fs::write(out.join("index.html"), page("The courses", &format!("<p><a href=\"../\">← home</a></p><h1>The courses</h1><p class=\"lead\">Mova learns in schools: courses by sciences, each lesson with a teacher who checks every answer. For now the small one is taught the primary school only (a child of 9–10), and we look at the results.</p><h2>The primary school</h2><div class=\"card\"><a href=\"mathematics/\"><b>Mathematics</b></a><br><small>the primary school course</small></div>{cards}{later}")))?;
    Ok(())
}

/// `world school site <out dir>`: the lessons as pages — the chat of teacher and student, the state change, the exam.
pub fn site(out: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(out)?;
    let root = crate::state::root().join("school");
    let mut cards = String::new();
    let mut dirs: Vec<std::path::PathBuf> = std::fs::read_dir(&root).map(|r| r.flatten().map(|e| e.path()).filter(|p| p.is_dir() && p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("lesson-"))).collect()).unwrap_or_default();
    // in the lessons' order: «lesson-9», «lesson-61», «lesson-100» by their numbers
    let num = |p: &std::path::PathBuf| p.file_name().map(|x| x.to_string_lossy().trim_start_matches("lesson-").split('-').next().unwrap_or("").parse::<usize>().unwrap_or(0)).unwrap_or(0);
    dirs.sort_by_key(|p| (num(p), p.clone()));
    for d in dirs {
        let name = d.file_name().unwrap().to_string_lossy().to_string();
        let chat = std::fs::read_to_string(d.join("chat.log")).unwrap_or_default();
        let change = std::fs::read_to_string(d.join("state-change.md")).unwrap_or_default();
        let title = change.lines().next().unwrap_or("").trim_start_matches("# ").split(" — ").next().unwrap_or(&name).to_string();
        let summary = change.lines().nth(2).unwrap_or("").to_string();
        let body = format!(
            "<p><a href=\"./\">← the school</a></p><h1>{}</h1><p class=\"lead\">{}</p><h2>The lesson, as it was spoken</h2>{}<h2>How the student changed</h2><pre>{}</pre><p><a href=\"{name}.chat.log\">the whole chat</a></p>",
            esc(&title),
            esc(&summary),
            chat_html(&chat, 160),
            esc(&change)
        );
        std::fs::write(out.join(format!("{name}.html")), page(&title, &body))?;
        std::fs::write(out.join(format!("{name}.chat.log")), &chat)?;
        cards += &format!("<div class=\"card\"><a href=\"{name}.html\"><b>{}</b></a><br><small>{}</small></div>", esc(&title), esc(&summary));
    }
    let prop = std::fs::read_to_string(root.join("proposal.md")).unwrap_or_default();
    let body = format!(
        "<p><a href=\"../\">← home</a></p><h1>The school of the small ones</h1><p class=\"lead\">A teacher is Mova projected for one domain and tuned to one lesson: it follows the program, knows the answer, gives one hint and corrects. The student is the small one: it reads, chooses, answers, and learns from the corrections. After a course comes an exam with a negative control, the student's proposal, a review, and only then a new state.</p>{cards}<h2>The student's proposal after the course</h2><pre>{}</pre>",
        esc(&prop)
    );
    std::fs::write(out.join("index.html"), page("The school of the small ones", &body))?;
    Ok(())
}

/// `world school list | lesson <n> | reset`.
pub fn main_args(args: &[String]) -> Result<()> {
    match args.get(1).map(String::as_str) {
        Some("list") | None => {
            println!("0. The table (put, take, count, imagine)");
            for l in lessons() {
                println!("{}. {} ({})", l.n, l.title, l.ops.iter().map(|o| o.name()).collect::<Vec<_>>().join(", "));
            }
        }
        Some("lesson") => {
            let n: usize = args.get(2).and_then(|x| x.parse().ok()).context("usage: world school lesson <n>")?;
            println!("{}", if n == 0 { table_lesson()? } else { teach(n)? });
        }
        Some("course") => {
            let a: usize = args.get(2).and_then(|x| x.parse().ok()).context("usage: world school course <from> <to>")?;
            let b: usize = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(a);
            print!("{}", course(a, b)?);
        }
        Some("actors") => {
            if args.get(2).map(String::as_str) == Some("reset") {
                let _ = std::fs::remove_file(Actors::path());
                let _ = std::fs::remove_file(crate::state::root().join("school").join("ops.tsv"));
                println!("the second student forgot its verbs and its choice of operations");
            } else if args.get(2).map(String::as_str) == Some("data") {
                let ns: Vec<usize> = args.iter().skip(3).filter_map(|x| x.parse().ok()).collect();
                print!("{}", data_course(&ns)?);
            } else if args.get(2).map(String::as_str) == Some("big") {
                let n: usize = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(40);
                print!("{}", big_pilot(n)?);
            } else if args.get(2).map(String::as_str) == Some("worlddiag") {
                print!("{}", world_diag(args.get(3).map(String::as_str).unwrap_or("worlddiag-examples.md"))?);
            } else if args.get(2).map(String::as_str) == Some("cv") {
                print!("{}", cross_check()?);
            } else if args.get(2).map(String::as_str) == Some("books") {
                let ns: Vec<usize> = args.iter().skip(3).filter_map(|x| x.parse().ok()).collect();
                print!("{}", book_course(&ns)?);
            } else if args.get(2).map(String::as_str) == Some("held") {
                let path = args.get(3).cloned().unwrap_or_else(|| format!("{}/research/raw/mwpdata-svamp/svamp-test.json", mova_data_dir()));
                print!("{}", held_out(std::path::Path::new(&path), args.get(4).and_then(|x| x.parse().ok()).unwrap_or(0))?);
            } else if args.get(2).map(String::as_str) == Some("scenarios") {
                let ns: Vec<usize> = args.iter().skip(3).filter_map(|x| x.parse().ok()).collect();
                print!("{}", scenario_course(&ns)?);
            } else if args.get(2).map(String::as_str) == Some("ablate") {
                print!("{}", ablate()?);
            } else if args.get(2).map(String::as_str) == Some("answers") {
                // `school actors answers <lesson>`: the exam's problems, the right answer, the student's and why
                let n: usize = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(1);
                print!("{}", lesson_answers(n));
            } else if args.get(2).map(String::as_str) == Some("read") {
                let text = args.iter().skip(3).cloned().collect::<Vec<_>>().join(" ");
                let p = Problem { text: text.clone(), a: 0, b: 0, op: Op::Add };
                println!("{:?}", reading_of(&text));
                println!("{:?}", actor_answer(&Actors::load(), &p));
            } else {
                let ns: Vec<usize> = args.iter().skip(2).filter_map(|x| x.parse().ok()).collect();
                print!("{}", actors_course(&ns)?);
            }
        }
        Some("prune") => println!("{}", prune(&args[2..])?),
        Some("graduate") => println!("{}", graduate(&args.iter().skip(2).cloned().collect::<Vec<_>>().join(" "))?),
        Some("courses") => {
            let out = args.get(2).cloned().unwrap_or_else(|| format!("{}/www/courses", mova_data_dir()));
            courses_site(std::path::Path::new(&out))?;
            println!("the courses written");
        }
        Some("site") => {
            site(std::path::Path::new(args.get(2).context("usage: world school site <out dir>")?))?;
            println!("school pages written");
        }
        Some("exam") => {
            // every lesson's test again, with what the student knows now: does it still remember?
            let s = Student::load();
            let show: usize = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0);
            for l in lessons() {
                let (r, n, items) = test(&s, &l, 0x5EED_0000 + l.n as u64 ^ 0xA11, 60);
                println!("{}. {}: {r}/{n}", l.n, l.title);
                for (p, a, op) in items.iter().filter(|x| x.1 != Some(x.0.answer())).take(show) {
                    let (_, why) = s.choose(p, &OPS);
                    println!("   ✗ {} → {} {:?} (because of {:?}); right: {}", p.text, op.name(), a, why, p.op.name());
                }
            }
        }
        Some("reset") => {
            let _ = std::fs::remove_file(Student::path());
            let _ = std::fs::remove_file(crate::state::root().join("school").join("verbs.tsv"));
            println!("the student forgot school arithmetic");
        }
        Some(x) => bail!("unknown: school {x}"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_lesson_teaches_and_forgetting_is_the_control() {
        let all = lessons();
        let l = &all[1];
        // a student who knows nothing adds: wrong on taking away
        let fresh = Student::default();
        let (r0, n, _) = test(&fresh, l, 7, 40);
        // one lesson of corrections
        let mut s = Student::default();
        let mut rng = Rng(1);
        let tmpls: Vec<&Tmpl> = PRACTICE.iter().filter(|t| t.0 == Op::Sub).collect();
        for i in 0..80 {
            let p = make(&mut rng, tmpls[i % tmpls.len()], l.max);
            let (_, op, _) = s.answer(&p, &OPS);
            s.learn(&p, op, None);
        }
        let (r1, _, _) = test(&s, l, 7, 40);
        assert!(r1 > r0 + n / 2, "before {r0}, after {r1} of {n}");
        // the same test twice gives the same answers
        assert_eq!(test(&s, l, 7, 40).0, r1);
    }
}
