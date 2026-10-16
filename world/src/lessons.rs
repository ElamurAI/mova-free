//! Lessons as data (`world/data/school/lessons/*.lesson`): a teacher's lesson written in a few lines — its words,
//! the little world it draws (names, things, numbers within bounds, conditions), the answer as an expression of that
//! world, the phrasings of its exercises and, apart, the phrasings only its test uses. The teacher computes every
//! answer from the expression, never from the text, so a lesson is right by construction when its phrasing says what
//! the expression computes.
//!
//! The format, one field per line («#» starts a comment):
//!
//! ```text
//! lesson: 61
//! title: What is left over in the boxes
//! level: primary-3
//! stars: 0
//! explain: When things are packed into boxes of 6, the full boxes hold 6 each and the rest is left over.
//! vars: a=name t=thing n=10..60 k=2..9
//! let: full = n / k * k
//! need: n % k != 0
//! answer: n % k
//! exercise: {a} packs {n} {t} into boxes of {k}. How many {t} are left over?
//! test: {a} puts {n} {t} in bags of {k}. How many {t} do not fill a bag?
//! ```
//!
//! Kinds of variables: `name` (a name of the school, all different; `{a.he}`, `{a.He}`, `{a.his}`, `{a.His}`,
//! `{a.him}` follow its sex), `thing` (a plural: apples), `item` (a singular: pencil), `lo..hi` (a whole number).
//! A number can be written `{n.words}` (fifty-five), `{n.ord}` (55th), `{n.ordwords}` (fifty-fifth). `let:` names an
//! expression; `need:` is a condition; expressions have + − × (`*`) ÷ (`/`, exact or the draw is made again), `%`,
//! brackets, comparisons and `&&`. Every draw is deterministic (the lesson's number seeds it).

use std::collections::BTreeMap;

use anyhow::{Result, bail};

#[derive(Clone, Debug, Default)]
pub struct Lesson {
    pub n: usize,
    pub title: String,
    pub stars: usize,
    /// the level of the course: «primary-1» … «primary-4» (grades), later «middle», «high», «bachelor» …
    pub level: String,
    pub explain: Vec<String>,
    vars: Vec<(String, String)>,
    lets: Vec<(String, String)>,
    needs: Vec<String>,
    answer: String,
    pub exercises: Vec<String>,
    pub tests: Vec<String>,
    /// the final exam's phrasings: the same problem in stiff, formal, official language a pupil has never heard
    pub exams: Vec<String>,
    pub file: String,
}

const NAMES: [&str; 16] = ["Tom", "Anna", "Sam", "Lily", "Max", "Mia", "Ben", "Zoe", "Jack", "Emma", "Leo", "Ruby", "Noah", "Ella", "Omar", "Ivy"];
const FEMALE: [&str; 8] = ["Anna", "Lily", "Mia", "Zoe", "Emma", "Ruby", "Ella", "Ivy"];
const THINGS: [&str; 16] = ["apples", "pencils", "stickers", "marbles", "books", "cookies", "balloons", "stamps", "shells", "cards", "eggs", "oranges", "crayons", "buttons", "flowers", "candies"];
const ITEMS: [&str; 10] = ["pencil", "apple", "sticker", "book", "cookie", "card", "toy", "notebook", "ball", "cup"];

fn dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data").join("school").join("lessons")
}

/// Every lesson of the folder, by number.
pub fn all() -> Result<Vec<Lesson>> {
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir()).map(|r| r.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "lesson")).collect()).unwrap_or_default();
    files.sort();
    let mut v = Vec::new();
    for f in files {
        v.push(parse(&std::fs::read_to_string(&f)?, &f.file_name().unwrap().to_string_lossy())?);
    }
    v.sort_by_key(|l| l.n);
    Ok(v)
}

pub fn parse(text: &str, file: &str) -> Result<Lesson> {
    let mut l = Lesson { file: file.into(), ..Default::default() };
    for line in text.lines().map(str::trim).filter(|x| !x.is_empty() && !x.starts_with('#')) {
        let Some((k, v)) = line.split_once(':') else { bail!("{file}: «{line}» has no field") };
        let v = v.trim().to_string();
        match k.trim() {
            "lesson" => l.n = v.parse()?,
            "title" => l.title = v,
            "stars" => l.stars = v.parse()?,
            "level" => l.level = v,
            "explain" => l.explain.push(v),
            "vars" => {
                for x in v.split_whitespace() {
                    let Some((n, kind)) = x.split_once('=') else { bail!("{file}: variable «{x}» has no kind") };
                    l.vars.push((n.into(), kind.into()));
                }
            }
            "let" => {
                let Some((n, e)) = v.split_once('=') else { bail!("{file}: «let: {v}» has no «=»") };
                l.lets.push((n.trim().into(), e.trim().into()));
            }
            "need" => l.needs.push(v),
            "answer" => l.answer = v,
            "exercise" => l.exercises.push(v),
            "test" => l.tests.push(v),
            "exam" => l.exams.push(v),
            other => bail!("{file}: unknown field «{other}»"),
        }
    }
    if l.n == 0 || l.answer.is_empty() || l.exercises.is_empty() || l.tests.is_empty() {
        bail!("{file}: a lesson needs its number, an answer, exercises and tests");
    }
    Ok(l)
}

// ---- expressions ------------------------------------------------------------------------------------------------

fn tokens(e: &str) -> Vec<String> {
    let mut out = Vec::new();
    let c: Vec<char> = e.chars().collect();
    let mut i = 0;
    while i < c.len() {
        let ch = c[i];
        if ch.is_whitespace() {
            i += 1;
        } else if ch.is_ascii_alphanumeric() || ch == '_' {
            let s = i;
            while i < c.len() && (c[i].is_ascii_alphanumeric() || c[i] == '_') {
                i += 1;
            }
            out.push(c[s..i].iter().collect());
        } else {
            let two: String = c[i..(i + 2).min(c.len())].iter().collect();
            if ["==", "!=", "<=", ">=", "&&", "||"].contains(&two.as_str()) {
                out.push(two);
                i += 2;
            } else {
                out.push(ch.to_string());
                i += 1;
            }
        }
    }
    out
}

/// Evaluate an expression over whole numbers; None when a division is not exact or a name is unknown.
pub fn eval(e: &str, env: &BTreeMap<String, i64>) -> Option<i64> {
    let t = tokens(e);
    let mut i = 0;
    let v = expr(&t, &mut i, env, 0)?;
    (i == t.len()).then_some(v)
}

fn prec(op: &str) -> Option<u8> {
    match op {
        "||" => Some(1),
        "&&" => Some(2),
        "==" | "!=" | "<" | ">" | "<=" | ">=" => Some(3),
        "+" | "-" => Some(4),
        "*" | "/" | "%" => Some(5),
        _ => None,
    }
}

fn expr(t: &[String], i: &mut usize, env: &BTreeMap<String, i64>, min: u8) -> Option<i64> {
    let mut lhs = atom(t, i, env)?;
    while let Some(op) = t.get(*i) {
        let Some(p) = prec(op) else { break };
        if p < min {
            break;
        }
        let op = op.clone();
        *i += 1;
        let rhs = expr(t, i, env, p + 1)?;
        lhs = match op.as_str() {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => {
                if rhs == 0 || lhs % rhs != 0 {
                    return None;
                }
                lhs / rhs
            }
            "%" => {
                if rhs == 0 {
                    return None;
                }
                lhs % rhs
            }
            "==" => (lhs == rhs) as i64,
            "!=" => (lhs != rhs) as i64,
            "<" => (lhs < rhs) as i64,
            ">" => (lhs > rhs) as i64,
            "<=" => (lhs <= rhs) as i64,
            ">=" => (lhs >= rhs) as i64,
            "&&" => (lhs != 0 && rhs != 0) as i64,
            "||" => (lhs != 0 || rhs != 0) as i64,
            _ => return None,
        };
    }
    Some(lhs)
}

fn atom(t: &[String], i: &mut usize, env: &BTreeMap<String, i64>) -> Option<i64> {
    let tok = t.get(*i)?.clone();
    *i += 1;
    match tok.as_str() {
        "(" => {
            let v = expr(t, i, env, 0)?;
            (t.get(*i).map(String::as_str) == Some(")")).then(|| {
                *i += 1;
                v
            })
        }
        "-" => atom(t, i, env).map(|v| -v),
        _ => tok.parse().ok().or_else(|| env.get(&tok).copied()),
    }
}

// ---- drawing a problem ------------------------------------------------------------------------------------------

/// A deterministic generator (xorshift).
pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1).max(1) as u64) as i64
    }
}

/// A drawn problem: its text and the teacher's answer.
#[derive(Clone, Debug)]
pub struct Drawn {
    pub text: String,
    pub answer: i64,
}

fn fill(template: &str, words: &BTreeMap<String, String>, nums: &BTreeMap<String, i64>) -> Option<String> {
    let mut out = String::new();
    let mut rest = template;
    while let Some(s) = rest.find('{') {
        out += &rest[..s];
        let e = rest[s..].find('}')? + s;
        let key = &rest[s + 1..e];
        let (base, form) = key.split_once('.').unwrap_or((key, ""));
        let piece = if let Some(w) = words.get(base) {
            let female = FEMALE.contains(&w.as_str());
            match form {
                "" => w.clone(),
                "he" => (if female { "she" } else { "he" }).into(),
                "He" => (if female { "She" } else { "He" }).into(),
                "his" => (if female { "her" } else { "his" }).into(),
                "His" => (if female { "Her" } else { "His" }).into(),
                "him" => (if female { "her" } else { "him" }).into(),
                _ => return None,
            }
        } else {
            let v = nums.get(base).copied().or_else(|| eval(base, nums))?;
            match form {
                "" => v.to_string(),
                "words" => crate::numbers::to_words(v),
                "ord" => format!("{v}{}", crate::numbers::ordinal_suffix(v)),
                "ordwords" => crate::numbers::to_ordinal_words(v),
                _ => return None,
            }
        };
        out += &piece;
        rest = &rest[e + 1..];
    }
    out += rest;
    Some(articles(&out))
}

/// «a 8 m rope» → «an 8 m rope», «an 5» → «a 5»: the article before a number follows how the number is said («eight»,
/// «eleven», «eighteen», «eighty-…» begin with a vowel sound; «one» does not: «a 1 litre carton»).
fn articles(text: &str) -> String {
    let ws: Vec<&str> = text.split(' ').collect();
    let mut out: Vec<String> = Vec::with_capacity(ws.len());
    for (i, w) in ws.iter().enumerate() {
        let next = ws.get(i + 1).copied().unwrap_or("");
        let digits: String = next.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() && matches!(*w, "a" | "an" | "A" | "An") {
            let vowel = digits.parse::<i64>().map(|n| { let w = crate::numbers::to_words(n); w.starts_with(['a', 'e', 'i', 'o', 'u']) && !w.starts_with("one") }).unwrap_or(false);
            let cap = w.starts_with('A');
            out.push(match (vowel, cap) { (true, true) => "An", (true, false) => "an", (false, true) => "A", (false, false) => "a" }.to_string());
        } else {
            out.push(w.to_string());
        }
    }
    out.join(" ")
}

impl Lesson {
    /// One problem from the given templates, drawn until its conditions hold (at most 2000 draws).
    pub fn draw(&self, rng: &mut Rng, templates: &[String], k: usize) -> Option<Drawn> {
        for _ in 0..2000 {
            let mut words: BTreeMap<String, String> = BTreeMap::new();
            let mut nums: BTreeMap<String, i64> = BTreeMap::new();
            let mut used: Vec<&str> = Vec::new();
            for (n, kind) in &self.vars {
                match kind.as_str() {
                    "name" => {
                        let free: Vec<&str> = NAMES.iter().copied().filter(|x| !used.contains(x)).collect();
                        let w = free[(rng.next() % free.len() as u64) as usize];
                        used.push(w);
                        words.insert(n.clone(), w.into());
                    }
                    "thing" => {
                        words.insert(n.clone(), THINGS[(rng.next() % THINGS.len() as u64) as usize].into());
                    }
                    "item" => {
                        words.insert(n.clone(), ITEMS[(rng.next() % ITEMS.len() as u64) as usize].into());
                    }
                    range => {
                        let (lo, hi) = range.split_once("..")?;
                        nums.insert(n.clone(), rng.range(lo.parse().ok()?, hi.parse().ok()?));
                    }
                }
            }
            let mut ok = true;
            for (n, e) in &self.lets {
                match eval(e, &nums) {
                    Some(v) => {
                        nums.insert(n.clone(), v);
                    }
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if !ok || !self.needs.iter().all(|c| eval(c, &nums).is_some_and(|v| v != 0)) {
                continue;
            }
            let Some(answer) = eval(&self.answer, &nums).filter(|a| *a >= 0) else { continue };
            let t = &templates[k % templates.len()];
            if let Some(text) = fill(t, &words, &nums) {
                return Some(Drawn { text, answer });
            }
        }
        None
    }

    /// The exercises (cycling through their phrasings) and the test (through the test's own).
    pub fn exercises(&self, count: usize) -> Vec<Drawn> {
        let mut rng = Rng(0xE7E7_0000 + self.n as u64);
        (0..count).filter_map(|k| self.draw(&mut rng, &self.exercises, k)).collect()
    }

    pub fn test(&self, count: usize) -> Vec<Drawn> {
        let mut rng = Rng(0x7E57_0000 + self.n as u64);
        (0..count).filter_map(|k| self.draw(&mut rng, &self.tests, k)).collect()
    }

    /// The final exam's problems (none when the lesson has no `exam:` lines).
    pub fn exam(&self, count: usize) -> Vec<Drawn> {
        if self.exams.is_empty() {
            return vec![];
        }
        let mut rng = Rng(0xE8A3_0000 + self.n as u64);
        (0..count).filter_map(|k| self.draw(&mut rng, &self.exams, k)).collect()
    }
}

/// `world lessons check` — every lesson parses and draws its exercises and tests; `world lessons show <n>` — a few.
pub fn main_args(args: &[String]) -> Result<()> {
    let ls = all()?;
    match args.get(1).map(String::as_str) {
        // `world lessons export <out.json> [per lesson]`: the lessons' exercises in SVAMP's form, to learn from
        Some("export") => {
            let out = args.get(2).cloned().unwrap_or_else(|| "lessons.json".into());
            let per: usize = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(20);
            let mut v = Vec::new();
            for l in &ls {
                for d in l.exercises(per) {
                    let (body, q) = d.text.rsplit_once(". ").map(|(b, q)| (format!("{b}."), q.to_string())).unwrap_or((String::new(), d.text.clone()));
                    v.push(serde_json::json!({"Body": body, "Question": q, "Answer": d.answer as f64, "Lesson": l.n}));
                }
            }
            std::fs::write(&out, serde_json::to_string(&v)?)?;
            println!("{} exercises written to {out}", v.len());
        }
        Some("show") => {
            let n: usize = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0);
            for l in ls.iter().filter(|l| l.n == n) {
                println!("{}. {}{}", l.n, "★".repeat(l.stars), l.title);
                for d in l.exercises(6) {
                    println!("  ex: {} → {}", d.text, d.answer);
                }
                for d in l.test(4) {
                    println!("  test: {} → {}", d.text, d.answer);
                }
            }
        }
        _ => {
            let mut bad = 0;
            for l in &ls {
                let (e, t) = (l.exercises(120).len(), l.test(60).len());
                if e < 120 || t < 60 {
                    bad += 1;
                    println!("✗ {} ({}): exercises {e}/120, tests {t}/60 — its conditions are too hard to meet", l.n, l.file);
                }
            }
            println!("{} lessons, {bad} with a problem", ls.len());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expressions() {
        let env: BTreeMap<String, i64> = [("n".to_string(), 17), ("k".to_string(), 5)].into();
        assert_eq!(eval("n % k", &env), Some(2));
        assert_eq!(eval("n / k", &env), None);
        assert_eq!(eval("(n - 2) / k * 2 + 1", &env), Some(7));
        assert_eq!(eval("n > k && k != 0", &env), Some(1));
        assert_eq!(eval("-k + n", &env), Some(12));
    }

    #[test]
    fn draws() {
        let l = parse("lesson: 999\ntitle: t\nvars: a=name t=thing n=10..60 k=2..9\nneed: n % k != 0\nanswer: n % k\nexercise: {a} packs {n} {t} into boxes of {k}; {a.he} has {n.words}.\ntest: {a} puts {n} {t} in bags of {k}.", "x").unwrap();
        let e = l.exercises(30);
        assert_eq!(e.len(), 30);
        assert!(e.iter().all(|d| d.answer > 0));
        assert!(e[0].text.contains("packs"));
    }
}
