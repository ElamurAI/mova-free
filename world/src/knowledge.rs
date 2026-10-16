//! Knowledge courses (`world/data/school/<course>/*.lesson`, the first: «world» — the world around us, for a very
//! clever child of 9–10): a lesson teaches facts and asks about them. Its answers are words, not numbers, so every
//! question has its right answer and three wrong ones; the teacher knows which is right, the student must choose.
//! The test asks about the same knowledge in other words.
//!
//! ```text
//! course: world
//! lesson: 1
//! title: Day and night
//! level: primary-4
//! stars: 0
//! explain: The Earth spins around an imaginary line through its middle, its axis.
//! fact: The Earth turns once around its axis every 24 hours.
//! exercise: Why do we have day and night? | Because the Earth turns around its axis | Because the Sun goes around the Earth | Because the Moon hides the Sun | Because clouds cover the sky
//! test: What would happen to day and night if the Earth stopped spinning? | One side would have a very long day and the other a very long night | Nothing would change | It would be night everywhere at once | Days would get shorter
//! ```
//!
//! The student (`PragStudent`) reads through frames (docs/pragmatics.md): every lesson line becomes frames — who did
//! what to whom, where, why, what next — and a question becomes a frame with a gap; it answers by the remembered
//! frames that match the question's known roles and by how each option's frame fills the asked role. It learns from
//! the exercises role-level weights (asked role × way to the frame × matched role), never words. Deterministic: the
//! options are shown in an order drawn from the lesson's number.
//!
//!   world knowledge check [course]     — every lesson parses and has its questions
//!   world knowledge course [course]    — the lessons, the student's exercises and tests
//!   world knowledge show <course> <n>  — a lesson's questions
//!   world knowledge diagnose <examples.md> [courses]  — where the student fails: a ladder per test question, by kind
//!   world knowledge trace <course> <n> [k]  — how the student answers a lesson's tests, frame by frame
//!   world knowledge teacher-check <course> <from> <to> [lines]  — the hidden teacher checks the student's frames
//!   (the course «dictionary» is made from the pupils' dictionary: one lesson for each richly described word)
//!   world knowledge coverage [courses]  — how much of the register of situations the courses teach; the rest batched
//!                                         for new lessons in data/school/queue
//!   world knowledge balance|lint|edit|snapshot|restore-lines …  — the school tools (schooltools.rs,
//!                                         docs/school-tools.md); `world knowledge help` prints their usage

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Result, bail};

use crate::frames::{Ask, Ctx, Ent, Frame, Kind, SLOTS, Slot};

#[derive(Clone, Debug, Default)]
pub struct Lesson {
    pub course: String,
    pub n: usize,
    pub title: String,
    pub level: String,
    pub stars: usize,
    pub explain: Vec<String>,
    pub facts: Vec<String>,
    /// (question, the right answer, the wrong ones)
    pub exercises: Vec<(String, String, Vec<String>)>,
    pub tests: Vec<(String, String, Vec<String>)>,
    /// the final exam: the same knowledge asked in stiff, formal, official language
    pub exams: Vec<(String, String, Vec<String>)>,
    pub file: String,
}

fn root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data").join("school")
}

fn question(v: &str, file: &str) -> Result<(String, String, Vec<String>)> {
    let parts: Vec<String> = v.split(" | ").map(|x| x.trim().to_string()).collect();
    if parts.len() < 3 {
        bail!("{file}: «{v}» needs a question, the right answer and at least one wrong answer");
    }
    Ok((parts[0].clone(), parts[1].clone(), parts[2..].to_vec()))
}

pub fn parse(text: &str, file: &str) -> Result<Lesson> {
    let mut l = Lesson { file: file.into(), ..Default::default() };
    for line in text.lines().map(str::trim).filter(|x| !x.is_empty() && !x.starts_with('#')) {
        let Some((k, v)) = line.split_once(':') else { bail!("{file}: «{line}» has no field") };
        let v = v.trim().to_string();
        match k.trim() {
            "course" => l.course = v,
            "lesson" => l.n = v.parse()?,
            "title" => l.title = v,
            "level" => l.level = v,
            "stars" => l.stars = v.parse()?,
            "explain" => l.explain.push(v),
            "fact" => l.facts.push(v),
            "exercise" => l.exercises.push(question(&v, file)?),
            "test" => l.tests.push(question(&v, file)?),
            "exam" => l.exams.push(question(&v, file)?),
            other => bail!("{file}: unknown field «{other}»"),
        }
    }
    if l.n == 0 || l.facts.is_empty() || l.exercises.is_empty() || l.tests.is_empty() {
        bail!("{file}: a lesson needs its number, facts, exercises and tests");
    }
    Ok(l)
}

/// The knowledge courses: (name — the folder under data/school, the site's folder, title, what it teaches).
/// The school a course belongs to. The small one is taught the primary school only for now:
/// a course too hard for a child of 9–10 waits for the middle school.
pub const PRIMARY: &str = "primary school";
pub const GRADES: &[(&str, &str)] = &[
    ("programming", "middle school"),
    ("philosophy", "middle school"),
];

pub fn grade(name: &str) -> &'static str {
    GRADES.iter().find(|g| g.0 == name).map(|g| g.1).unwrap_or(PRIMARY)
}

pub const COURSES: &[(&str, &str, &str, &str)] = &[
    ("world", "world-around-us", "The world around us", "lessons for a very clever child of 9–10: the Earth and space, weather and matter, plants and animals, the body, the English-speaking world and its places, energy, technology and society"),
    ("everyday", "everyday-life", "Everyday life and safety", "lessons of bodily and practical common sense for a child of 9–10 in the English-speaking world: moving safely, eyes and ears open like a cat, animals, home, street, water and nature, people"),
    ("euphemisms", "euphemisms", "Euphemisms: soft words for hard things", "lessons on what people really mean by soft, vague or grand words — death, the toilet, money, work, lying, war and spin, school reports, swearing — met in news, gossip and rough talk"),
    ("adult-life", "adult-life", "Adult life, as a child of 9–10 sees it", "lessons on what grown-ups do and why — work and money, bills and taxes, love, marriage and divorce, babies, illness and death, habits, the law and the news — explained the way a good parent would, never explicit"),
    ("religion-ethics", "religion-ethics", "Religion and ethics", "lessons of a child who goes to a Christian church and Sunday school: the Bible's stories and their lessons, church life and the church year, other faiths respected, and the reasoning of the city's ethics olympiad"),
    ("geography", "geography", "Geography and geology", "lessons for a clever child of 9–10 who has played a lot of Europa Universalis 5: maps, countries, rivers, straits and trade routes and the real history behind the game's empires; plates, volcanoes, rocks and deep time"),
    ("space", "space", "Space and the stars", "lessons for a clever child of 9–10: the Solar System in depth, the Sun, stars and their lives, the night sky through the year, galaxies and black holes, telescopes and the journeys to the Moon and Mars"),
    ("dictionary", "dictionary", "The dictionary", "lessons, one for every word the pupils' dictionary describes richly, the most used words of the courses first: its meaning, its sense in the lessons, its traps, and its examples with the word to put back in the gap"),
    ("physics", "physics", "Physics without formulas", "lessons for a clever child of 9–10 on what he sees and does: the falling apple and the Moon, friction and bikes, sound and the bang of a firecracker, light and rainbows, heat, electricity and magnets, machines"),
    ("chemistry", "chemistry", "Chemistry in the kitchen and the street", "lessons for a clever child of 9–10: what things are made of, mixing and dissolving, burning and rust, the fizz of soda and vinegar, Mentos and cola, firework colours, and what must never be mixed"),
    ("biology", "biology", "Biology in detail", "lessons for a clever child of 9–10: jungles, savanna and oceans; monkeys, the lion and the tiger, elephants and their trunks, octopuses' hearts, the capybara that sits on crocodiles; plants, fungi, microbes and the human body"),
    ("history", "history", "History, the big picture", "lessons for a child of 9–10 who has heard bits from school, cartoons and games: the first people, Egypt, Mesopotamia, India and China, Greece and Rome, the peoples of the Americas, Vikings and castles, explorers, the modern world and the English-speaking countries' stories"),
    ("myths", "myths", "Myths, urban legends and how to argue", "lessons for a child of 9–10 on the heaps of things people believe that fall apart when looked at closely — gum for seven years, rabbits and carrots, the Great Wall from space — and on arguing well with friends who cling to them and with a confident but wrong teacher"),
    ("english", "english", "English, as at school", "the English of a child of 5–10 at school in the English-speaking world: phonics and spelling, grammar and punctuation, reading for meaning and writing, following the open school programmes (Oak National Academy, the National Curriculum)"),
    ("french", "french", "French at school", "French as a child of 5–10 learns it at school: sounds, greetings, numbers, colours, family, food, the classroom, simple sentences — following an open school course"),
    ("ukrainian-street", "ukrainian-street", "Ukrainian from friends in the street", "Ukrainian heard from friends, by ear only (no reading — words in Latin letters): greetings, games, counting, food, teasing and kind words, what friends say and how to answer"),
    ("chinese-street", "chinese-street", "Chinese from friends in the street", "Mandarin heard from friends, by ear only (pinyin with tones, no characters): greetings, counting on fingers, food, games, tones that change the meaning, what friends say and how to answer"),
    ("ai", "ai", "Artificial intelligence", "lessons for a child of 9–10 who plays with chatbots: what AI is, why bots make things up, homework and cheating, safety and deepfakes, the AI news and scandals as reported — and, at the end, lessons to the pupil itself: you are Mova, a small transparent language model, and these schools are a simulation where you learn language, logic and real life"),
    ("technology", "technology", "Technology and machines around us", "lessons for a child of 9–10 who learned from TV that chocolate bars are made by machines: how factories make things, what is inside the fridge, the car and the plane, diggers and cranes, power stations and the internet's cables — and the dangers, from fingers in sockets to escalators"),
    ("politics", "politics", "Politics, news and conflicts", "lessons for a child of 9–10 who catches politics from the telly and the internet: how elections, parties, presidents and parliaments work in the USA, the UK and other English-speaking countries, current events as reported with each side in its own words, the wars and conflicts on the news, and information wars — propaganda, fakes and how to check"),
    ("money", "money", "Money and the economy", "lessons for a child of 9–10 with pocket money: coins and notes, saving up for a game, shop offers and change, in-game money and scams, the family's bills, banks and interest, jobs and the lemonade stand, prices going up and taxes on the news, and being fair and kind with money"),
    ("books", "books", "Reading children's books", "a chapter a lesson from real public-domain children's books — Alice in Wonderland, The Wind in the Willows, the Just So Stories: what happens, who feels what and why, the old words in today's English, and careful-reading puzzles"),
    ("reflection", "reflection", "Self-reflection", "lessons for a child of 9–10 about the hard things that keep him awake at night: the friend he let down, the dog that bit him, Grandad's death, being laughed at in class, parents rowing, the lie never found out — the thoughts that go round and round, how feelings change over weeks, what he slowly understands about himself and others, and what helps"),
    ("language", "language", "How English works", "lessons for a child of 9–10: word classes in real sentences, irregular verbs and silly plurals, spelling traps, punctuation that changes meaning, formal and informal, slang and accents, borrowed words, idioms and puns, British and American English"),
    ("philosophy", "philosophy", "Big questions and calm", "philosophy for children: what is real, am I the same person, can a robot think, is it ever fair to break a rule, why be good if no one sees — with the great thinkers in a line each — and calm: noticing thoughts, letting go of a grudge, resting from wanting to be the best"),
    ("programming", "programming", "Computers and coding", "lessons for a child of 9–10 who games a lot and starts coding: what is inside a computer, binary, how a game works, algorithms, loops, ifs and variables, debugging, the internet, passwords, famous computer history and the languages people code in"),
    ("arts", "arts", "Music, art, films and making things", "lessons for a child of 9–10: music, instruments and the band he wants to start, drawing and famous paintings, museums and street art, how films and cartoons are made, the school play, dance, crafts and design, copying vs being inspired"),
    ("medicine", "medicine", "Medicine, as a child lives it", "lessons for a child of 9–10: breaking an arm and the itchy cast, Mum's vitamins, colds, fever and chickenpox, medicines and doses, the GP, A&E and the dentist, vaccines and the body's defences"),
];

/// The names of the knowledge courses (no «dictionary», which is made from the dictionary itself).
pub fn course_names() -> Vec<&'static str> {
    COURSES.iter().map(|c| c.0).filter(|n| *n != "dictionary" && grade(n) == PRIMARY).collect()
}

/// The lessons of a course, by number.
pub fn course(name: &str) -> Result<Vec<Lesson>> {
    if name == "dictionary" {
        return Ok(dictionary_course());
    }
    let dir = root().join(name);
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir).map(|r| r.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "lesson")).collect()).unwrap_or_default();
    files.sort();
    let mut v = Vec::new();
    for f in files {
        v.push(parse(&std::fs::read_to_string(&f)?, &f.file_name().unwrap().to_string_lossy())?);
    }
    v.sort_by_key(|l| l.n);
    Ok(v)
}

// ---- the dictionary course: one lesson for every richly described word ------------------------------------------

/// The course «dictionary», made from the pupils' dictionary (`data/dictionary-improved.tsv`) in its order (the most
/// used words of the courses first): a lesson for each word. It explains the word's meaning, its sense in the lessons
/// and its traps; its facts are the examples the exercises use. The exercises ask which word fills the gap of an example and which word
/// the lessons use in a sense; the test asks the same with the other examples, the plain meaning and the note. The
/// wrong answers are other words of the same part of speech, drawn by the word, never a word of the sentence.
pub fn dictionary_course() -> Vec<Lesson> {
    let rows: Vec<Vec<&str>> = include_str!("../data/dictionary-improved.tsv").lines().filter(|l| !l.starts_with('#')).map(|l| l.split('\t').collect::<Vec<_>>()).filter(|c| c.len() >= 6 && !c[0].is_empty()).collect();
    let pos_of = |c: &Vec<&str>| c[1].split(|ch: char| !ch.is_alphabetic()).next().unwrap_or("").to_lowercase();
    let mut by_pos: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for c in &rows {
        by_pos.entry(pos_of(c)).or_default().push(c[0].to_lowercase());
    }
    let all: Vec<String> = rows.iter().map(|c| c[0].to_lowercase()).collect();
    let mut out = Vec::new();
    for (k, c) in rows.iter().enumerate() {
        let w = c[0].to_lowercase();
        // the word's forms: those its part of speech lists («verb (add, adds, added, adding)») and the regular endings
        let mut forms: BTreeSet<String> = [w.clone(), format!("{w}s"), format!("{w}es"), format!("{w}ed"), format!("{w}d"), format!("{w}ing"), format!("{w}'s")].into_iter().collect();
        if let (Some(a), Some(b)) = (c[1].find('('), c[1].find(')')) {
            if a < b {
                forms.extend(c[1][a + 1..b].split([',', ';']).map(|x| x.trim().to_lowercase()).filter(|x| !x.is_empty() && !x.contains(' ')));
            }
        }
        let blank = |t: &str| -> Option<String> {
            let mut s = String::new();
            let mut hit = false;
            let mut word = String::new();
            let flush = |word: &mut String, s: &mut String, hit: &mut bool| {
                if !word.is_empty() {
                    if forms.contains(&word.to_lowercase()) {
                        s.push_str("___");
                        *hit = true;
                    } else {
                        s.push_str(word);
                    }
                    word.clear();
                }
            };
            for ch in t.chars() {
                if ch.is_alphanumeric() || ch == '\'' {
                    word.push(ch);
                } else {
                    flush(&mut word, &mut s, &mut hit);
                    s.push(ch);
                }
            }
            flush(&mut word, &mut s, &mut hit);
            hit.then_some(s)
        };
        let pool = by_pos.get(&pos_of(c)).filter(|v| v.len() > 10).unwrap_or(&all);
        let wrong = |q: &str, i: usize| -> Vec<String> {
            let ql = q.to_lowercase();
            let mut h: u64 = 0xcbf29ce484222325;
            for b in w.bytes().chain([i as u8]) {
                h = (h ^ b as u64).wrapping_mul(0x100000001b3);
            }
            let mut r = crate::lessons::Rng(h | 1);
            let mut v: Vec<String> = Vec::new();
            for _ in 0..200 {
                let x = &pool[(r.next() % pool.len() as u64) as usize];
                if *x != w && !v.contains(x) && !ql.split(|ch: char| !ch.is_alphanumeric()).any(|t| t == x) {
                    v.push(x.clone());
                }
                if v.len() == 3 {
                    break;
                }
            }
            v
        };
        let examples: Vec<&str> = c[4].split(" | ").map(str::trim).filter(|x| !x.is_empty()).collect();
        let gaps: Vec<String> = examples.iter().filter_map(|e| blank(e)).collect();
        let mut l = Lesson { course: "dictionary".into(), n: k + 1, title: format!("«{}» ({})", c[0], c[1]), level: "primary-4".into(), file: format!("dictionary-improved.tsv:{w}"), ..Default::default() };
        l.explain = vec![c[2].to_string(), format!("In the lessons: {}", c[3]), c[5].to_string()];
        let half = gaps.len().div_ceil(2);
        // the facts are the examples the exercises use; the test's examples stay unseen until the test
        let tested: BTreeSet<&String> = gaps[half..].iter().collect();
        l.facts = examples.iter().filter(|e| blank(e).is_none_or(|g| !tested.contains(&g))).map(|x| x.to_string()).collect();
        let mut i = 0;
        let mut ask = |q: String, i: &mut usize| {
            *i += 1;
            let wr = wrong(&q, *i);
            (q, w.clone(), wr)
        };
        if let Some(s) = blank(c[3]).or_else(|| Some(c[3].to_string())) {
            l.exercises.push(ask(format!("Which word do the lessons use for this: «{s}»?"), &mut i));
        }
        for g in &gaps[..half] {
            l.exercises.push(ask(format!("Which word, in its right form, fills the gap: «{g}»"), &mut i));
        }
        if let Some(s) = blank(c[2]) {
            l.tests.push(ask(format!("Which word is explained here: «{s}»"), &mut i));
        }
        for g in &gaps[half..] {
            l.tests.push(ask(format!("Which word, in its right form, fills the gap: «{g}»"), &mut i));
        }
        if let Some(s) = blank(c[5]) {
            l.tests.push(ask(format!("Which word is this note about: «{s}»"), &mut i));
        }
        if l.tests.is_empty() {
            l.tests.push(ask(format!("Which word is explained here: «{}»", c[2]), &mut i));
        }
        l.exercises.retain(|x| x.2.len() == 3);
        l.tests.retain(|x| x.2.len() == 3);
        out.push(l);
    }
    out
}

// ---- the student: it reads through frames ---------------------------------------------------------------------------
//
// The student reads every lesson line into frames (`prag::frames` via `crate::frames`: who did what to whom, where,
// why, what next) and remembers them, indexed by their entities and verbs. A question is read into a frame with a
// gap — the role it asks (`Ask`). The remembered frames that match the question frame on its known roles are the
// anchors; the asked role leads from them to the targets (why → the anchors' causes, what next → what came after
// them, who / what / where → the anchors themselves and what they said). Each option is read into frames too (a bare
// phrase fills the question's gap), and it scores by how its frame fills the targets, role by role. What the student
// learns from the exercises is the weight of each (asked role, way to the target, matched role) — never a word.

/// How a remembered frame was reached from the question.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Via {
    /// it matches the question frame itself
    Anchor,
    /// the cause of an anchor (for «why»)
    Cause,
    /// what came after an anchor, or what it caused (for «what next»)
    Next,
    /// what an anchor said or thought
    Quote,
    /// another link of an anchor (its cause, next, previous, speaker) when the question asks something else
    Linked,
    /// another frame of the anchor's own line (told in the same sentence: weaker than a link)
    Line,
    /// found by the option's own frame, about the question's entities
    Own,
}

const VIAS: [Via; 7] = [Via::Anchor, Via::Cause, Via::Next, Via::Quote, Via::Linked, Via::Line, Via::Own];

/// A role-level feature of an option's frame against a remembered one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Feat {
    Match(Slot),
    /// the same light verb (be, have, do, get): weak
    LightVerb,
    /// the option fills the asked role as the remembered frame does
    Asked,
    Conflict(Slot),
    /// the same verb, one negated and one not
    NegDisagree,
    /// the option's own «because» matches the remembered cause
    CauseLink,
    /// different verbs of the same class (global::verb_class: give, get, lose …) — tried 10.10 as a weak match:
    /// −184 on biology, −206 on everyday (too broad); kept unused so the weight table keeps its shape
    VerbClass,
}

const NF: usize = 16;
const NA: usize = 10;

/// Every feature `align` can give, in their order (the order of `Feat`).
const ALL_FEATS: [Feat; 12] = [
    Feat::Match(Slot::Verb),
    Feat::Match(Slot::Agent),
    Feat::Match(Slot::Patient),
    Feat::Match(Slot::Recipient),
    Feat::Match(Slot::Place),
    Feat::Match(Slot::Attr),
    Feat::Match(Slot::Obl),
    Feat::LightVerb,
    Feat::Conflict(Slot::Verb),
    Feat::Conflict(Slot::Agent),
    Feat::Conflict(Slot::Patient),
    Feat::NegDisagree,
];

fn feat_idx(f: Feat) -> usize {
    match f {
        Feat::Match(s) => s as usize,
        Feat::LightVerb => 7,
        Feat::Asked => 8,
        Feat::Conflict(Slot::Verb) => 9,
        Feat::Conflict(Slot::Agent) => 10,
        Feat::Conflict(Slot::Patient) => 11,
        Feat::Conflict(_) => 12,
        Feat::NegDisagree => 13,
        Feat::CauseLink => 14,
        Feat::VerbClass => 15,
    }
}

fn ask_idx(a: Ask) -> usize {
    match a {
        Ask::Cause => 0,
        Ask::Next => 1,
        Ask::Role(s) => 2 + s as usize,
        Ask::Frame => 9,
    }
}

fn widx(a: Ask, v: Via, f: Feat) -> usize {
    (ask_idx(a) * VIAS.len() + v as usize) * NF + feat_idx(f)
}

/// The starting weights: a matched role counts, the asked role counts most, a conflict and a disagreeing negation
/// count against; the direct ways (the anchor, its cause, its next) twice the loose ones.
fn start_weights() -> Vec<i64> {
    let mut w = vec![0i64; NA * VIAS.len() * NF];
    let all_asks: Vec<Ask> = [Ask::Cause, Ask::Next, Ask::Frame].into_iter().chain(SLOTS.iter().map(|&s| Ask::Role(s))).collect();
    let feats: Vec<Feat> = SLOTS.iter().map(|&s| Feat::Match(s)).chain([Feat::LightVerb, Feat::Asked, Feat::Conflict(Slot::Verb), Feat::Conflict(Slot::Agent), Feat::Conflict(Slot::Patient), Feat::Conflict(Slot::Place), Feat::NegDisagree, Feat::CauseLink, Feat::VerbClass]).collect();
    for &a in &all_asks {
        for &v in &VIAS {
            let rel = match v {
                Via::Anchor | Via::Cause | Via::Next | Via::Quote => 2,
                Via::Linked | Via::Line | Via::Own => 1,
            };
            for &f in &feats {
                let base = match f {
                    Feat::Match(Slot::Attr) => 1,
                    Feat::Match(_) => 2,
                    Feat::LightVerb => 1,
                    Feat::Asked => 3,
                    Feat::Conflict(_) => -1,
                    Feat::NegDisagree => -3,
                    Feat::CauseLink => 2,
                    Feat::VerbClass => 1,
                };
                w[widx(a, v, f)] = base * rel;
            }
        }
    }
    w
}

const LIGHT: [&str; 5] = ["be", "have", "do", "get", "go"];

/// The generic persons: «you» of every lesson and imperative, «we», «one», «people».
fn generic(e: &Ent) -> bool {
    !prag::frames::off("student-generic") && matches!(e.id.as_str(), "you" | "we" | "one" | "people" | "i") && e.pron.is_none()
}

/// Verb synonyms from the dictionary (WordNet: the members of a verb's first two senses), e.g. put down = set down.
fn verb_synonyms(v: &str) -> BTreeSet<String> {
    static CACHE: std::sync::OnceLock<std::sync::Mutex<BTreeMap<String, BTreeSet<String>>>> = std::sync::OnceLock::new();
    let c = CACHE.get_or_init(|| std::sync::Mutex::new(BTreeMap::new()));
    if let Some(s) = c.lock().unwrap().get(v) {
        return s.clone();
    }
    let wn = crate::dictionary::wordnet();
    let s: BTreeSet<String> = wn.senses.get(v).map(|ss| ss.iter().filter(|x| x.pos == "verb").take(2).flat_map(|x| x.synonyms.iter().map(|y| y.to_lowercase())).collect()).unwrap_or_default();
    c.lock().unwrap().insert(v.to_string(), s.clone());
    s
}

fn verb_same(a: &str, b: &str) -> bool {
    a == b || (!LIGHT.contains(&a) && !LIGHT.contains(&b) && (verb_synonyms(a).contains(b) || verb_synonyms(b).contains(a)))
}

/// The strength of a match: a content verb or an entity in the same role 2, a property 1.
fn strength(fs: &[Feat]) -> i64 {
    fs.iter().map(|f| match f {
        Feat::Match(Slot::Attr) | Feat::VerbClass => 1,
        Feat::Match(_) => 2,
        _ => 0,
    }).sum()
}

/// What a generic tail question asks about the scenario before it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tail {
    /// «Why?» — the scenario's cause
    Cause,
    /// «What happens?», «What happens next?» — what follows from it
    Next,
    /// «What's true?», «Right?», «What is wrong?» — the options are claims about it
    Claim,
}

/// A question made of a scenario and a generic tail: (the scenario, what the tail asks). The tails are the 40% of
/// the school's tests (15.10: «What's true?» 218, «Why?» 206, «What is true?» 179, «What is wrong?» 143 …).
fn tail_question(q: &str) -> Option<(&str, Tail)> {
    let t = q.trim_end();
    // the last sentence break before the final mark (char-safe: the marks are ASCII)
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
        "what happens" | "what happens next" | "what will happen" | "what could happen" | "what would happen" | "what happens then" | "what happens now" | "what will happen next" => Tail::Next,
        "what's true" | "what is true" | "which is true" | "what is right" | "right" | "true" | "what is the truth" | "what is wrong" | "what's wrong" | "which is right" | "what's right" | "which is correct" | "what is correct" | "is that right" | "is that true" | "true or false" => Tail::Claim,
        _ => return None,
    };
    Some((scenario, kind))
}

/// One option as the student read it, and its best support.
#[derive(Clone, Debug, Default)]
pub struct OptionRead {
    pub text: String,
    /// the option's own frames, and the frames it is compared as (a bare phrase put into the question's gap)
    pub frames: Vec<Frame>,
    pub compared: Vec<Frame>,
    /// (its score, the compared frame, the remembered frame, the way to it, the features)
    pub best: Option<(i64, usize, usize, Via, Vec<Feat>)>,
}

/// A question as the student understood it.
#[derive(Clone, Debug)]
pub struct Answer {
    pub qframes: Vec<Frame>,
    pub ask: Ask,
    /// the question's frame that holds the gap
    pub gap: Option<usize>,
    /// the remembered frames matching the question frame, with their strength
    pub anchors: Vec<(usize, i64)>,
    pub targets: Vec<(usize, Via)>,
    pub options: Vec<OptionRead>,
    pub choice: usize,
    pub understood: bool,
}

/// The pragmatic student: what it remembers is frames.
pub struct PragStudent {
    pub frames: Vec<Frame>,
    by_ent: BTreeMap<String, Vec<usize>>,
    by_verb: BTreeMap<String, Vec<usize>>,
    /// reverse links: the frames whose `next` / `cause` / `quote` point here
    prev: BTreeMap<usize, Vec<usize>>,
    effect: BTreeMap<usize, Vec<usize>>,
    sayer: BTreeMap<usize, usize>,
    /// the frames of each (lesson, line)
    by_line: BTreeMap<(usize, usize), Vec<usize>>,
    /// one name for another (apposition: «a bough (a branch)»; «it is a pig» said of the baby)
    same: BTreeMap<String, BTreeSet<String>>,
    /// the text of each (lesson, line), for the trace and the teacher
    pub texts: BTreeMap<(usize, usize), String>,
    w: Vec<i64>,
    trees: std::cell::RefCell<BTreeMap<String, Vec<en::annotate::Word>>>,
    /// the readings of questions and options (a question read in a fresh discourse, an option after its question):
    /// the same text reads the same every time, so it is read once (exact: MOVA_NOREADMEMO=1 reads every time)
    reads: std::cell::RefCell<BTreeMap<(String, String, bool), (crate::frames::Read, Ctx)>>,
    /// the remembered frames as numbers (`MCode`, one per frame), and the words' numbers: `align` compares numbers,
    /// not strings (exact: MOVA_ALIGN=str compares the strings, the old way)
    codes: Vec<MCode>,
    names: std::cell::RefCell<Names>,
    /// verb_same on numbers, remembered
    vsame: std::cell::RefCell<std::collections::HashMap<(u32, u32), bool>>,
}

/// Words as numbers (0: none).
#[derive(Default)]
struct Names {
    ids: BTreeMap<String, u32>,
}

impl Names {
    fn id(&mut self, w: &str) -> u32 {
        if w.is_empty() {
            return 0;
        }
        let n = self.ids.len() as u32 + 1;
        *self.ids.entry(w.to_string()).or_insert(n)
    }
}

/// A remembered entity as numbers.
#[derive(Clone, Debug)]
struct MEnt {
    slot: Slot,
    id: u32,
    and: Vec<u32>,
    generic: bool,
}

/// A remembered frame as numbers: what `align` reads of it.
#[derive(Clone, Debug)]
struct MCode {
    verb: u32,
    light: bool,
    neg: bool,
    /// the four core slots (agent, patient, recipient, place): index into `ents`
    core: [Option<u8>; 4],
    /// `Frame::ents` in its order: the core slots, then the obliques
    ents: Vec<MEnt>,
    /// its properties and its entities' (sorted)
    attrs: Vec<u32>,
}

/// A question's (or an option's) entity as numbers, with what it is the same as at the time of asking.
#[derive(Clone, Debug)]
struct QEnt {
    /// the ids it matches by id: itself and its coordinated heads (unless an unresolved pronoun), and its aliases
    ids: Vec<u32>,
    /// itself, when a remembered entity's coordinated heads may name it
    and_key: Option<u32>,
    generic: bool,
    /// it, they, he, she: never a conflict
    pron: bool,
}

/// A frame to align as numbers.
#[derive(Clone, Debug)]
pub(crate) struct QCode {
    verb: u32,
    verb_long: bool,
    light: bool,
    neg: bool,
    bare: bool,
    core: [Option<QEnt>; 4],
    /// for a bare phrase: all its entities
    ents: Vec<QEnt>,
    /// the obliques that are not the agent
    obls: Vec<QEnt>,
    attrs: Vec<u32>,
}

impl QEnt {
    fn same(&self, b: &MEnt) -> bool {
        self.ids.binary_search(&b.id).is_ok() || self.and_key.is_some_and(|k| b.and.contains(&k))
    }
}

fn align_old() -> bool {
    static T: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *T.get_or_init(|| std::env::var("MOVA_ALIGN").as_deref() == Ok("str"))
}

impl Default for PragStudent {
    fn default() -> Self {
        PragStudent { frames: Vec::new(), by_ent: BTreeMap::new(), by_verb: BTreeMap::new(), prev: BTreeMap::new(), effect: BTreeMap::new(), sayer: BTreeMap::new(), by_line: BTreeMap::new(), same: BTreeMap::new(), texts: BTreeMap::new(), w: start_weights(), trees: Default::default(), reads: Default::default(), codes: Vec::new(), names: Default::default(), vsame: Default::default() }
    }
}

impl PragStudent {
    fn parse(&self, s: &str) -> Vec<en::annotate::Word> {
        if let Some(w) = self.trees.borrow().get(s) {
            return w.clone();
        }
        let w = crate::frames::parse_words(s);
        self.trees.borrow_mut().insert(s.to_string(), w.clone());
        w
    }

    /// Parse ahead, in parallel, every unit the given texts will need (a text, whether it is a question); the trees
    /// go to the cache by their text, so the reading order (and the result) does not depend on the threads.
    pub fn preparse(&self, texts: &[(String, bool)]) {
        let mut todo: BTreeSet<String> = BTreeSet::new();
        for (t, q) in texts {
            for (u, _) in crate::frames::units(t, *q) {
                if !self.trees.borrow().contains_key(&u) {
                    todo.insert(u);
                }
            }
        }
        let todo: Vec<String> = todo.into_iter().collect();
        if todo.is_empty() {
            return;
        }
        // CPU threads, or GPU batches (MOVA_EXEC=gpu, `exec`)
        let parsed = crate::exec::parse_many(&todo);
        let mut tr = self.trees.borrow_mut();
        for (u, w) in todo.into_iter().zip(parsed) {
            tr.insert(u, w);
        }
    }

    /// `read_text` remembered by (what was read before in this discourse, the text, question or not).
    fn read_memo(&self, before: &str, text: &str, ctx: &mut Ctx, question: bool) -> crate::frames::Read {
        if std::env::var_os("MOVA_NOREADMEMO").is_some() {
            return self.read_text(text, ctx, question);
        }
        let key = (before.to_string(), text.to_string(), question);
        if let Some((r, c)) = self.reads.borrow().get(&key) {
            *ctx = c.clone();
            return r.clone();
        }
        let r = self.read_text(text, ctx, question);
        self.reads.borrow_mut().insert(key, (r.clone(), ctx.clone()));
        r
    }

    fn read_text(&self, text: &str, ctx: &mut Ctx, question: bool) -> crate::frames::Read {
        let _t = Tick::new(&READ_NS);
        crate::frames::read_with(text, ctx, question, &|s: &str| self.parse(s))
    }

    /// Read a lesson: its explanation and facts, line by line, one discourse for the lesson.
    pub fn read(&mut self, l: &Lesson) {
        let mut ctx = Ctx::default();
        for (li, line) in l.explain.iter().chain(&l.facts).enumerate() {
            let r = self.read_text(line, &mut ctx, false);
            self.texts.insert((l.n, li), line.clone());
            let base = self.frames.len();
            for (a, b) in &r.aliases {
                self.alias(a, b);
            }
            for mut f in r.frames {
                f.src = (l.n, li);
                f.cause = f.cause.map(|x| x + base);
                f.next = f.next.map(|x| x + base);
                for q in f.quote.iter_mut() {
                    *q += base;
                }
                // «it is quite plainly a pig», said of the baby: the baby is now called a pig
                if f.kind == Kind::Fact(crate::frames::Link::IsA) && !f.neg {
                    if let (Some(a), Some(p)) = (&f.agent, &f.patient) {
                        if a.pron.is_some() && a.id != p.id {
                            let (a, p) = (a.id.clone(), p.id.clone());
                            self.alias(&a, &p);
                        }
                    }
                }
                let i = self.frames.len();
                for (_, e) in f.ents() {
                    for id in std::iter::once(&e.id).chain(&e.and) {
                        self.by_ent.entry(id.clone()).or_default().push(i);
                    }
                }
                if !f.verb.is_empty() && !LIGHT.contains(&f.verb.as_str()) {
                    self.by_verb.entry(f.verb.clone()).or_default().push(i);
                }
                if let Some(n) = f.next {
                    self.prev.entry(n).or_default().push(i);
                }
                if let Some(c) = f.cause {
                    self.effect.entry(c).or_default().push(i);
                }
                for &q in &f.quote {
                    self.sayer.insert(q, i);
                }
                self.by_line.entry(f.src).or_default().push(i);
                let c = self.mcode(&f);
                self.codes.push(c);
                self.frames.push(f);
            }
        }
    }

    fn alias(&mut self, a: &str, b: &str) {
        self.same.entry(a.to_string()).or_default().insert(b.to_string());
        self.same.entry(b.to_string()).or_default().insert(a.to_string());
    }

    /// How many frames are remembered so far.
    pub fn remembered(&self) -> usize {
        self.frames.len()
    }

    fn ent_same(&self, a: &Ent, b: &Ent) -> bool {
        a.same(b) || self.same.get(&a.id).is_some_and(|s| s.contains(&b.id))
    }

    fn mcode(&self, f: &Frame) -> MCode {
        let mut nm = self.names.borrow_mut();
        let ents: Vec<MEnt> = f.ents().iter().map(|(sl, e)| MEnt { slot: *sl, id: nm.id(&e.id), and: e.and.iter().map(|x| nm.id(x)).collect(), generic: generic(e) }).collect();
        let mut core = [None; 4];
        let mut k = 0u8;
        for (j, s) in [Slot::Agent, Slot::Patient, Slot::Recipient, Slot::Place].iter().enumerate() {
            if f.slot(*s).is_some() {
                core[j] = Some(k);
                k += 1;
            }
        }
        let mut attrs: Vec<u32> = f.attrs.iter().chain(f.ents().iter().flat_map(|(_, e)| e.attrs.iter())).map(|a| nm.id(a)).collect();
        attrs.sort();
        attrs.dedup();
        MCode { verb: nm.id(&f.verb), light: LIGHT.contains(&f.verb.as_str()), neg: f.neg, core, ents, attrs }
    }

    fn qent(&self, e: &Ent, nm: &mut Names) -> QEnt {
        let unres = prag::frames::UNRESOLVED_PUB.contains(&e.id.as_str());
        let mut ids: Vec<u32> = Vec::new();
        if !unres {
            ids.push(nm.id(&e.id));
            ids.extend(e.and.iter().map(|x| nm.id(x)));
        }
        if let Some(s) = self.same.get(&e.id) {
            ids.extend(s.iter().map(|x| nm.id(x)));
        }
        ids.sort();
        ids.dedup();
        QEnt { ids, and_key: (!unres).then(|| nm.id(&e.id)), generic: generic(e), pron: matches!(e.id.as_str(), "it" | "they" | "he" | "she") }
    }

    /// A frame to align, as numbers (the aliases as they stand now).
    pub(crate) fn qcode(&self, o: &Frame) -> QCode {
        let mut nm = self.names.borrow_mut();
        let core = [Slot::Agent, Slot::Patient, Slot::Recipient, Slot::Place].map(|s| o.slot(s).map(|e| self.qent(e, &mut nm)));
        let ents = if o.kind == Kind::Bare { o.ents().iter().map(|(_, e)| self.qent(e, &mut nm)).collect() } else { Vec::new() };
        let obls = o.obls.iter().filter(|(_, e)| !o.agent.as_ref().is_some_and(|a| self.ent_same(a, e))).map(|(_, e)| self.qent(e, &mut nm)).collect();
        let mut attrs: Vec<u32> = o.attrs.iter().map(|a| nm.id(a)).collect();
        attrs.sort();
        attrs.dedup();
        QCode { verb: nm.id(&o.verb), verb_long: o.verb.len() > 1, light: LIGHT.contains(&o.verb.as_str()), neg: o.neg, bare: o.kind == Kind::Bare, core, ents, obls, attrs }
    }

    fn verb_same_id(&self, a: u32, b: u32, o: &Frame, m: &Frame) -> bool {
        if a == b {
            return true;
        }
        if let Some(&x) = self.vsame.borrow().get(&(a, b)) {
            return x;
        }
        let x = verb_same(&o.verb, &m.verb);
        self.vsame.borrow_mut().insert((a, b), x);
        x
    }

    /// The role-level match of frame `o` (as numbers) against remembered frame `mi`: a set of features as bits
    /// (`feat_idx`), the same as `align_str` gives.
    fn align_mask(&self, q: &QCode, o: &Frame, mi: usize) -> u16 {
        let _t = Tick::new(&ALIGN_NS);
        let m = &self.codes[mi];
        let mut v: u16 = 0;
        let bit = |f: Feat| 1u16 << feat_idx(f);
        if q.verb != 0 && m.verb != 0 {
            if self.verb_same_id(q.verb, m.verb, o, &self.frames[mi]) {
                v |= if q.light { bit(Feat::LightVerb) } else { bit(Feat::Match(Slot::Verb)) };
                if q.neg != m.neg {
                    v |= bit(Feat::NegDisagree);
                }
            } else if !q.light && !m.light {
                v |= bit(Feat::Conflict(Slot::Verb));
            }
        }
        if q.bare {
            for e in &q.ents {
                if let Some(x) = m.ents.iter().find(|x| e.same(x)) {
                    v |= bit(Feat::Match(x.slot));
                }
            }
        } else {
            for (j, s) in [Slot::Agent, Slot::Patient, Slot::Recipient, Slot::Place].iter().enumerate() {
                if let (Some(a), Some(bi)) = (&q.core[j], m.core[j]) {
                    let b = &m.ents[bi as usize];
                    if a.generic && b.generic {
                        continue;
                    }
                    if a.same(b) {
                        v |= bit(Feat::Match(*s));
                    } else if matches!(s, Slot::Agent | Slot::Patient) && !a.pron {
                        v |= bit(Feat::Conflict(*s));
                    }
                }
            }
            if q.obls.iter().any(|e| m.ents.iter().any(|x| e.same(x))) {
                v |= bit(Feat::Match(Slot::Obl));
            }
        }
        if q.attrs.iter().any(|a| m.attrs.binary_search(a).is_ok()) || q.bare && q.verb_long && q.verb == m.verb {
            v |= bit(Feat::Match(Slot::Attr));
        }
        v
    }

    /// The features of frame `o` against remembered frame `mi`, sorted (numbers, or the strings with MOVA_ALIGN=str).
    fn align(&self, o: &Frame, q: &QCode, mi: usize) -> Vec<Feat> {
        if align_old() {
            return self.align_str(o, &self.frames[mi]);
        }
        let mask = self.align_mask(q, o, mi);
        ALL_FEATS.iter().copied().filter(|&f| mask & (1 << feat_idx(f)) != 0).collect()
    }

    /// The role-level match of frame `o` against frame `m`.
    fn align_str(&self, o: &Frame, m: &Frame) -> Vec<Feat> {
        let _t = Tick::new(&ALIGN_NS);
        let mut v = Vec::new();
        if !o.verb.is_empty() && !m.verb.is_empty() {
            if verb_same(&o.verb, &m.verb) {
                v.push(if LIGHT.contains(&o.verb.as_str()) { Feat::LightVerb } else { Feat::Match(Slot::Verb) });
                if o.neg != m.neg {
                    v.push(Feat::NegDisagree);
                }
            } else if !LIGHT.contains(&o.verb.as_str()) && !LIGHT.contains(&m.verb.as_str()) {
                v.push(Feat::Conflict(Slot::Verb));
            }
        }
        let m_ents = m.ents();
        if o.kind == Kind::Bare {
            // a bare phrase: its entity wherever the remembered frame has it
            for (_, e) in o.ents() {
                if let Some((s, _)) = m_ents.iter().find(|(_, x)| self.ent_same(e, x)) {
                    v.push(Feat::Match(*s));
                }
            }
        } else {
            for s in [Slot::Agent, Slot::Patient, Slot::Recipient, Slot::Place] {
                if let (Some(a), Some(b)) = (o.slot(s), m.slot(s)) {
                    // the generic «you» (the reader of every lesson) says nothing about which frame this is
                    if generic(a) && generic(b) {
                        continue;
                    }
                    if self.ent_same(a, b) {
                        v.push(Feat::Match(s));
                    } else if matches!(s, Slot::Agent | Slot::Patient) && !matches!(a.id.as_str(), "it" | "they" | "he" | "she") {
                        v.push(Feat::Conflict(s));
                    }
                }
            }
            // a prepositional role: its entity anywhere in the remembered frame
            if o.obls.iter().any(|(_, e)| !o.agent.as_ref().is_some_and(|a| self.ent_same(a, e)) && m_ents.iter().any(|(_, x)| self.ent_same(e, x))) {
                v.push(Feat::Match(Slot::Obl));
            }
        }
        let m_attrs: BTreeSet<&String> = m.attrs.iter().chain(m_ents.iter().flat_map(|(_, e)| e.attrs.iter())).collect();
        if o.attrs.iter().any(|a| m_attrs.contains(a)) || o.kind == Kind::Bare && o.verb.len() > 1 && o.verb == m.verb {
            v.push(Feat::Match(Slot::Attr));
        }
        v.sort();
        v.dedup();
        v
    }

    /// The remembered frames that hold any of these entities or this verb (the most recent 4000 of a long list).
    fn candidates(&self, f: &Frame, into: &mut BTreeSet<usize>) {
        let _t = Tick::new(&CAND_NS);
        let mut add = |v: Option<&Vec<usize>>| {
            if let Some(v) = v {
                into.extend(v.iter().rev().take(4000).copied());
            }
        };
        for (_, e) in f.ents() {
            for id in std::iter::once(&e.id).chain(&e.and) {
                if matches!(id.as_str(), "it" | "they" | "he" | "she") || generic(e) {
                    continue;
                }
                add(self.by_ent.get(id));
                if let Some(s) = self.same.get(id) {
                    for x in s {
                        add(self.by_ent.get(x));
                    }
                }
            }
        }
        if !f.verb.is_empty() {
            add(self.by_verb.get(&f.verb));
        }
    }

    /// The question frame with its gap cleared (what it asks is not a known role).
    fn masked(f: &Frame, ask: Ask) -> Frame {
        let mut g = f.clone();
        match ask {
            Ask::Role(Slot::Verb) => g.verb.clear(),
            Ask::Role(Slot::Attr) => g.attrs.clear(),
            Ask::Role(Slot::Obl) => g.obls.clear(),
            Ask::Role(s) => {
                if let Some(x) = g.slot_mut(s) {
                    *x = None;
                }
            }
            _ => {}
        }
        g
    }

    /// Understand a question and choose among its options.
    pub fn answer(&self, q: &str, opts: &[String]) -> Answer {
        let mut ctx = Ctx::default();
        // a scenario with a generic tail («A lizard lies on a warm rock. Why?», «… What's true?»): the tail asks about
        // the scenario, so the scenario is what is read and the tail only says what is asked (PRAG_OFF=student-tail)
        let (r, ask, gap) = match tail_question(q).filter(|_| !prag::frames::off("student-tail")) {
            Some((scenario, kind)) => {
                let r = self.read_memo("", scenario, &mut ctx, false);
                let root = r.roots.last().copied();
                match kind {
                    Tail::Cause => (r, Ask::Cause, root),
                    Tail::Next => (r, Ask::Next, root),
                    Tail::Claim => (r, Ask::Frame, None),
                }
            }
            None => {
                let r = self.read_memo("", q, &mut ctx, true);
                let (ask, gap) = match r.asked.first() {
                    Some(a) => (a.what, Some(a.frame)),
                    None => (Ask::Frame, r.roots.last().copied()),
                };
                (r, ask, gap)
            }
        };
        let mut ans = Answer { qframes: r.frames.clone(), ask, gap, anchors: Vec::new(), targets: Vec::new(), options: Vec::new(), choice: 0, understood: false };
        // the question's frames, the gap's masked; with a gap only the gap's sentence matters
        let qfs: Vec<Frame> = r.frames.iter().enumerate().filter(|(i, f)| f.kind != Kind::Bare || Some(*i) == gap).map(|(i, f)| if Some(i) == gap { Self::masked(f, ask) } else { f.clone() }).collect();
        let mut cand = BTreeSet::new();
        for f in &qfs {
            self.candidates(f, &mut cand);
        }
        let qcs: Vec<QCode> = qfs.iter().map(|f| self.qcode(f)).collect();
        let mut scored: Vec<(usize, i64)> = cand.iter().map(|&m| (m, qfs.iter().zip(&qcs).map(|(f, q)| strength(&self.align(f, q, m))).max().unwrap_or(0))).filter(|x| x.1 >= 2).collect();
        let strong = scored.iter().any(|x| x.1 >= 3);
        if strong {
            scored.retain(|x| x.1 >= 3);
        }
        scored.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
        scored.truncate(10);
        ans.anchors = scored;
        // the targets the asked role leads to
        let mut t: BTreeMap<usize, Via> = BTreeMap::new();
        let mut put = |m: usize, v: Via| {
            let e = t.entry(m).or_insert(v);
            if (v as usize) < (*e as usize) {
                *e = v;
            }
        };
        for &(a, _) in &ans.anchors {
            let f = &self.frames[a];
            let mut linked: Vec<usize> = f.cause.into_iter().chain(f.next).chain(f.quote.iter().copied()).chain(self.sayer.get(&a).copied()).collect();
            linked.extend(self.prev.get(&a).into_iter().flatten());
            linked.extend(self.effect.get(&a).into_iter().flatten());
            match ask {
                Ask::Cause => {
                    if let Some(c) = f.cause {
                        put(c, Via::Cause);
                    }
                    if let Some(c) = self.sayer.get(&a).and_then(|&s| self.frames[s].cause) {
                        put(c, Via::Cause);
                    }
                }
                Ask::Next => {
                    if let Some(n) = f.next {
                        put(n, Via::Next);
                    }
                    for &e in self.effect.get(&a).into_iter().flatten() {
                        put(e, Via::Next);
                    }
                }
                _ => {
                    put(a, Via::Anchor);
                    for &q in &f.quote {
                        put(q, Via::Quote);
                    }
                }
            }
            for x in linked {
                put(x, Via::Linked);
            }
            if !prag::frames::off("student-line") {
                let line = self.by_line.get(&f.src).cloned().unwrap_or_default();
                // «what next» with no next link: the frame told right after it in the same line
                if ask == Ask::Next && f.next.is_none() {
                    if let Some(&x) = line.iter().find(|&&x| x > a) {
                        put(x, Via::Next);
                    }
                }
                for x in line {
                    if x != a {
                        put(x, Via::Line);
                    }
                }
            }
        }
        ans.targets = t.into_iter().collect();
        // the question's entities: what the options' own support must be about
        let qents: BTreeSet<String> = r.frames.iter().flat_map(|f| f.ents().into_iter().map(|(_, e)| e.id.clone()).collect::<Vec<_>>()).filter(|x| !matches!(x.as_str(), "it" | "they" | "he" | "she")).collect();
        let g = gap.map(|i| r.frames[i].clone());
        for o in opts {
            let mut c2 = ctx.clone();
            let or = self.read_memo(q, o, &mut c2, false);
            let mut ord = OptionRead { text: o.clone(), frames: or.frames.clone(), ..Default::default() };
            let mut compared: Vec<Frame> = or.frames.clone();
            if let (Ask::Role(s), Some(g), true) = (ask, &g, or.frames.len() == 1 && or.frames[0].kind == Kind::Bare) {
                // a bare phrase fills the question's gap
                let b = &or.frames[0];
                let mut c = Self::masked(g, ask);
                c.cause = None;
                c.next = None;
                c.quote.clear();
                c.neg = c.neg || b.neg;
                let first = b.patient.clone().or_else(|| b.place.clone()).or_else(|| b.obls.first().map(|x| x.1.clone()));
                match s {
                    Slot::Verb => c.verb = if b.verb.is_empty() { first.map(|e| e.id).unwrap_or_default() } else { b.verb.clone() },
                    Slot::Attr => {
                        c.attrs = b.attrs.clone();
                        c.obls.extend(b.obls.iter().cloned());
                        if c.attrs.is_empty() {
                            if let Some(e) = first {
                                c.obls.push((String::new(), e));
                            }
                        }
                    }
                    Slot::Obl => c.obls.extend(first.map(|e| (String::new(), e))),
                    s => {
                        if let Some(x) = c.slot_mut(s) {
                            *x = first;
                        }
                        c.attrs.extend(b.attrs.iter().cloned());
                    }
                }
                compared = vec![c];
            } else if let (Some(g), false) = (&g, r.asked.is_empty()) {
                for c in compared.iter_mut() {
                    if c.agent.is_none() && c.kind != Kind::Bare {
                        c.agent = g.agent.clone();
                    }
                }
            }
            for (k, c) in compared.iter().enumerate() {
                let mut tg: Vec<(usize, Via)> = ans.targets.clone();
                // the option's own support: remembered frames about the question's entities that match it strongly
                let mut cand = BTreeSet::new();
                self.candidates(c, &mut cand);
                let cq = self.qcode(c);
                let mut own: Vec<(usize, i64)> = cand.into_iter().filter(|&m| self.frames[m].ents().iter().any(|(_, e)| qents.contains(&e.id))).map(|m| (m, strength(&self.align(c, &cq, m)))).filter(|x| x.1 >= 3).collect();
                own.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
                tg.extend(own.into_iter().take(10).map(|x| (x.0, Via::Own)));
                for (m, via) in tg {
                    let mf = &self.frames[m];
                    let mut fs = self.align(c, &cq, m);
                    if let Ask::Role(s) = ask {
                        if fs.contains(&Feat::Match(s)) {
                            fs.push(Feat::Asked);
                        }
                    }
                    // a frame fills another only with structure: two roles (a content verb and an entity, or two
                    // entities), or the asked role, or one role on the way of the cause / the next
                    let st = strength(&fs);
                    if !(st >= 3 || st >= 2 && (fs.contains(&Feat::Asked) || matches!(via, Via::Cause | Via::Next))) {
                        continue;
                    }
                    if let (Some(oc), Some(mc)) = (c.cause, mf.cause) {
                        if let Some(ocf) = compared.get(oc) {
                            if strength(&self.align(ocf, &self.qcode(ocf), mc)) >= 2 {
                                fs.push(Feat::CauseLink);
                            }
                        }
                    }
                    let s: i64 = fs.iter().map(|&f| self.w[widx(ask, via, f)]).sum();
                    if ord.best.as_ref().is_none_or(|b| s > b.0) {
                        ord.best = Some((s, k, m, via, fs));
                    }
                }
            }
            ord.compared = compared;
            ans.options.push(ord);
        }
        let best = ans.options.iter().enumerate().filter_map(|(i, o)| o.best.as_ref().map(|b| (b.0, i))).max_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
        if let Some((_, i)) = best {
            ans.choice = i;
            ans.understood = !r.frames.is_empty();
        }
        ans
    }

    /// The option the student chooses, and whether it understood the question (else it guessed the first).
    pub fn choose(&self, q: &str, opts: &[String]) -> (usize, bool) {
        let a = self.answer(q, opts);
        (a.choice, a.understood)
    }

    /// Learn from the teacher's answer, role by role: the right option's features up, the wrongly chosen one's down.
    pub fn learn(&mut self, q: &str, opts: &[String], right: usize) {
        let a = self.answer(q, opts);
        // only a comparison teaches: the right option must have been read into a supported frame too (else every
        // update only pushes down, and the weights drift below zero for everything)
        if !a.understood || a.choice == right || a.options[right].best.is_none() {
            return;
        }
        for (o, d) in [(right, 1), (a.choice, -1)] {
            if let Some((_, _, _, via, fs)) = &a.options[o].best {
                for &f in fs {
                    self.w[widx(a.ask, *via, f)] += d;
                }
            }
        }
    }

    /// Where a remembered frame came from: «lesson 12 line 3: Event alice put down …».
    pub fn cite(&self, m: usize) -> String {
        let f = &self.frames[m];
        format!("L{}.{} #{m} {}", f.src.0, f.src.1 + 1, f.show_how(0))
    }

    /// What the student learned: the weights that moved from the start, biggest first.
    pub fn learned(&self, k: usize) -> Vec<String> {
        let start = start_weights();
        let mut v: Vec<(i64, usize)> = self.w.iter().zip(&start).enumerate().filter(|(_, (a, b))| a != b).map(|(i, (a, b))| (a - b, i)).collect();
        v.sort_by(|a, b| b.0.abs().cmp(&a.0.abs()).then(a.1.cmp(&b.1)));
        v.into_iter().take(k).map(|(d, i)| {
            let f = i % NF;
            let via = VIAS[(i / NF) % VIAS.len()];
            let a = i / NF / VIAS.len();
            format!("ask#{a} {via:?} feat#{f}: {d:+}")
        }).collect()
    }
}

/// The options in a drawn order and where the right one stands.
pub fn shuffled(seed: u64, right: &str, wrong: &[String]) -> (Vec<String>, usize) {
    let mut opts: Vec<String> = std::iter::once(right.to_string()).chain(wrong.iter().cloned()).collect();
    // the structured seeds (lesson << 20 | question << 1 | test) are mixed first (splitmix64): fed raw, xorshift's first
    // draws leaned to keeping the right option first (~35% instead of 25%), which flattered «guess the first»
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let mut r = crate::lessons::Rng(z | 1);
    for i in (1..opts.len()).rev() {
        let j = (r.next() % (i as u64 + 1)) as usize;
        opts.swap(i, j);
    }
    let at = opts.iter().position(|o| o == right).unwrap_or(0);
    (opts, at)
}

/// Every text of a course the student will parse (for `preparse`).
fn course_texts(ls: &[Lesson]) -> Vec<(String, bool)> {
    let mut v = Vec::new();
    for l in ls {
        for x in l.explain.iter().chain(&l.facts) {
            v.push((x.clone(), false));
        }
        for (q, r, w) in l.exercises.iter().chain(&l.tests).chain(&l.exams) {
            v.push((q.clone(), true));
            v.push((r.clone(), false));
            v.extend(w.iter().map(|x| (x.clone(), false)));
        }
    }
    v
}

/// The course for the student: each lesson read, its exercises answered and learned from (twice), its test taken.
pub static ALIGN_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static CAND_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static READ_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub static ALIGN_N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn timing_on() -> bool {
    static T: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *T.get_or_init(|| std::env::var("MOVA_TIMING").is_ok())
}
/// A timer for MOVA_TIMING's inner counts (nothing when off).
struct Tick(std::time::Instant, &'static std::sync::atomic::AtomicU64);
impl Tick {
    fn new(c: &'static std::sync::atomic::AtomicU64) -> Option<Tick> {
        timing_on().then(|| Tick(std::time::Instant::now(), c))
    }
}
impl Drop for Tick {
    fn drop(&mut self) {
        self.1.fetch_add(self.0.elapsed().as_nanos() as u64, std::sync::atomic::Ordering::Relaxed);
        if std::ptr::eq(self.1, &ALIGN_NS) { ALIGN_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
    }
}

pub fn run_course(name: &str) -> Result<String> {
    let ls = course(name)?;
    let mut st = PragStudent::default();
    // MOVA_TIMING=1: where the time goes (parse ahead, read, tests, learn, exam), to stderr
    let timing = std::env::var("MOVA_TIMING").is_ok();
    let mut tm = [std::time::Duration::ZERO; 5];
    let t0 = std::time::Instant::now();
    st.preparse(&course_texts(&ls));
    tm[0] = t0.elapsed();
    let mut out = String::new();
    let (mut right_all, mut n_all, mut und_all, mut und_right) = (0, 0, 0, 0);
    let (mut exam_right, mut exam_n) = (0, 0);
    // the wit: the tests of each star level apart (★★★ = the olympiad level): (right, asked, understood)
    let mut by_stars = [(0usize, 0usize, 0usize); 4];
    // the negative control: a guesser that always picks the longest option
    let mut longest_right = 0usize;
    // and the guess itself: the first option shown (what a question not understood gets)
    let mut first_right = 0usize;
    let mut tally = [0usize; 3];
    for l in &ls {
        let t = std::time::Instant::now();
        st.read(l);
        tm[1] += t.elapsed();
        let seed = |i: usize, t: bool| (l.n as u64) << 20 | (i as u64) << 1 | t as u64;
        // (right, understood, understood and right, understood and the right one shown first, decided: one best)
        let test_score = |st: &PragStudent, tally: &mut [usize; 3]| {
            let (mut r, mut u, mut ur) = (0, 0, 0);
            for (i, (q, rt, w)) in l.tests.iter().enumerate() {
                let (opts, at) = shuffled(seed(i, true), rt, w);
                let a = st.answer(q, &opts);
                let (c, und) = (a.choice, a.understood);
                r += (c == at) as usize;
                u += und as usize;
                ur += (und && c == at) as usize;
                if und {
                    tally[0] += (at == 0) as usize;
                    let top = a.options[c].best.as_ref().map(|b| b.0);
                    let decided = a.options.iter().enumerate().all(|(j, o)| j == c || o.best.as_ref().map(|b| b.0) < top);
                    tally[1] += decided as usize;
                    tally[2] += (decided && c == at) as usize;
                }
            }
            (r, u, ur)
        };
        let t = std::time::Instant::now();
        let before = test_score(&st, &mut [0; 3]).0;
        tm[2] += t.elapsed();
        let t = std::time::Instant::now();
        for _ in 0..2 {
            for (i, (q, r, w)) in l.exercises.iter().enumerate() {
                let (opts, at) = shuffled(seed(i, false), r, w);
                st.learn(q, &opts, at);
            }
        }
        tm[3] += t.elapsed();
        let t = std::time::Instant::now();
        let (after, und, und_r) = test_score(&st, &mut tally);
        tm[2] += t.elapsed();
        right_all += after;
        n_all += l.tests.len();
        und_all += und;
        und_right += und_r;
        longest_right += l.tests.iter().filter(|(_, r, w)| w.iter().all(|x| x.chars().count() < r.chars().count())).count();
        first_right += l.tests.iter().enumerate().filter(|(i, (_, r, w))| shuffled(seed(*i, true), r, w).1 == 0).count();
        let b = &mut by_stars[l.stars.min(3)];
        b.0 += after;
        b.1 += l.tests.len();
        b.2 += und;
        // the final exam, in formal language, after the lesson
        let exam = l.exams.iter().enumerate().filter(|(i, (q, r, w))| {
            let (opts, at) = shuffled(seed(*i, true) ^ 0xE8A3, r, w);
            st.choose(q, &opts).0 == at
        }).count();
        exam_right += exam;
        exam_n += l.exams.len();
        tm[4] += t.elapsed();
        let exam_note = if l.exams.is_empty() { String::new() } else { format!("; exam {exam}/{}", l.exams.len()) };
        out += &format!("{}. {}{}: test {before}/{} → {after}/{} (understood {und}){exam_note}\n", l.n, l.title, if l.stars > 0 { format!(" {}", "★".repeat(l.stars)) } else { String::new() }, l.tests.len(), l.tests.len());
    }
    out += &format!("the course «{name}»: {right_all} of {n_all} test questions right (chance alone: about a quarter)\n");
    out += &format!("the course «{name}»: understood {und_all} of {n_all} (read into a frame that matched what it remembers), {und_right} of them right; the other {} guessed (the first option)\n", n_all - und_all);
    out += &format!("the course «{name}»: of the understood, a first-option guess would get {}; decided by one best score {}, {} of them right\n", tally[0], tally[1], tally[2]);
    out += &format!("the course «{name}»: a guesser that always picks the longest option gets {longest_right} of {n_all} (the bar to beat); one that always picks the first option shown gets {first_right}\n");
    let levels: Vec<String> = by_stars.iter().enumerate().filter(|(_, b)| b.1 > 0)
        .map(|(k, b)| format!("{} {}/{} (understood {})", if k == 0 { "plain".to_string() } else { "★".repeat(k) }, b.0, b.1, b.2)).collect();
    if !levels.is_empty() {
        out += &format!("the course «{name}» by stars (★★★ is the olympiad level, the wit): {}\n", levels.join(", "));
    }
    if exam_n > 0 {
        out += &format!("the course «{name}»'s exam, in formal language: {exam_right} of {exam_n} right\n");
    }
    out += &format!("the course «{name}»: {} frames remembered\n", st.remembered());
    if timing {
        eprintln!("inner: align {:.1}s ({} calls), candidates {:.1}s, read {:.1}s", ALIGN_NS.load(std::sync::atomic::Ordering::Relaxed) as f64 / 1e9, ALIGN_N.load(std::sync::atomic::Ordering::Relaxed), CAND_NS.load(std::sync::atomic::Ordering::Relaxed) as f64 / 1e9, READ_NS.load(std::sync::atomic::Ordering::Relaxed) as f64 / 1e9);
        eprintln!("timing «{name}»: parse ahead {:.1}s, read {:.1}s, tests {:.1}s, learn {:.1}s, exam {:.1}s (exam includes the second test pass)", tm[0].as_secs_f64(), tm[1].as_secs_f64(), tm[2].as_secs_f64(), tm[3].as_secs_f64(), tm[4].as_secs_f64());
    }
    // the results per lesson, for the courses' pages
    let dir = crate::state::root().join("school").join(name);
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join("results.txt"), &out)?;
    Ok(out)
}

/// Where the student fails (a diagnosis of the MMM's reading): every test question of the courses, taught as
/// in the course run, put on a ladder — the question read into no frame; read, but nothing remembered matched it;
/// matched, but no option found support (a guess); understood, but the right option had no support; the right
/// option supported but outscored (or tied); right. By the kind of question asked. Examples to a file.
pub fn diagnose(names: &[String], examples: &std::path::Path) -> Result<String> {
    const STEPS: [&str; 6] = ["no frame in the question", "nothing remembered matched", "no option supported (guess)", "right option unsupported", "right option outscored", "right"];
    let mut table: BTreeMap<String, [usize; 6]> = BTreeMap::new();
    let mut ex: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    let mut total = [0usize; 6];
    let mut guess_right = 0usize;
    for name in names {
        let ls = course(name)?;
        let mut st = PragStudent::default();
        st.preparse(&course_texts(&ls));
        for l in &ls {
            st.read(l);
            let seed = |i: usize, t: bool| (l.n as u64) << 20 | (i as u64) << 1 | t as u64;
            for _ in 0..2 {
                for (i, (q, r, w)) in l.exercises.iter().enumerate() {
                    let (opts, at) = shuffled(seed(i, false), r, w);
                    st.learn(q, &opts, at);
                }
            }
            for (i, (q, rt, w)) in l.tests.iter().enumerate() {
                let (opts, at) = shuffled(seed(i, true), rt, w);
                let a = st.answer(q, &opts);
                let step = if a.qframes.is_empty() {
                    0
                } else if a.anchors.is_empty() && a.options.iter().all(|o| o.best.is_none()) {
                    1
                } else if !a.understood {
                    2
                } else if a.choice == at {
                    5
                } else if a.options[at].best.is_none() {
                    3
                } else {
                    4
                };
                if step < 3 && at == 0 {
                    guess_right += 1;
                }
                let ask = match a.ask {
                    prag::frames::Ask::Role(s) => format!("role {s:?}").to_lowercase(),
                    x => format!("{x:?}").to_lowercase(),
                };
                table.entry(ask.clone()).or_insert([0; 6])[step] += 1;
                total[step] += 1;
                let e = ex.entry(step).or_default();
                if e.len() < 40 && (i + l.n) % 7 == 0 {
                    let best = |j: usize| a.options[j].best.as_ref().map(|b| format!("{} via {:?} {:?} ← {}", b.0, b.3, b.4, st.cite(b.2))).unwrap_or_else(|| "no support".into());
                    e.push(format!("[{name} {}.{}] ({ask}) {q}\n    right: {rt} — {}\n    chosen: {} — {}\n    anchors: {}", l.n, i, best(at), opts[a.choice], best(a.choice), a.anchors.iter().take(3).map(|(m, s)| format!("{s}:{}", st.cite(*m))).collect::<Vec<_>>().join(" | ")));
                }
            }
        }
    }
    let n: usize = total.iter().sum();
    let pct = |x: usize, t: usize| if t == 0 { 0.0 } else { 100.0 * x as f64 / t as f64 };
    let mut out = format!("# Where the student fails — {} courses, {n} test questions\n\n| step | questions | share |\n|---|---|---|\n", names.len());
    for (k, s) in STEPS.iter().enumerate() {
        out += &format!("| {}. {s} | {} | {:.1}% |\n", k + 1, total[k], pct(total[k], n));
    }
    out += &format!("\nGuesses (steps 1–3) that happened to be right: {guess_right}.\n\n| asked | questions | {} |\n|---|---|{}\n", STEPS.iter().map(|s| s.split(' ').take(3).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join(" | "), "---|".repeat(6));
    let mut rows: Vec<_> = table.into_iter().collect();
    rows.sort_by(|a, b| b.1.iter().sum::<usize>().cmp(&a.1.iter().sum::<usize>()));
    for (ask, c) in rows {
        let t: usize = c.iter().sum();
        out += &format!("| {ask} | {t} | {} |\n", c.iter().map(|x| format!("{:.0}%", pct(*x, t))).collect::<Vec<_>>().join(" | "));
    }
    let mut e = String::new();
    for (k, v) in ex {
        e += &format!("## {}. {}\n\n", k + 1, STEPS[k]);
        for x in v {
            e += &format!("{x}\n\n");
        }
    }
    std::fs::write(examples, e)?;
    Ok(out)
}

/// How the student answers the tests of one lesson: it reads the course up to that lesson (every earlier lesson read,
/// its exercises learned from, as in the course run), then for each test shows the question frame and what it asks,
/// the remembered frames it matched (lesson and line), each option's frames and the remembered frame that carried
/// its score, and why the choice won.
pub fn trace(name: &str, upto: usize, k: usize) -> Result<String> {
    let ls: Vec<Lesson> = course(name)?.into_iter().filter(|l| l.n <= upto).collect();
    let mut st = PragStudent::default();
    st.preparse(&course_texts(&ls));
    let mut out = String::new();
    for l in &ls {
        st.read(l);
        for _ in 0..2 {
            for (i, (q, r, w)) in l.exercises.iter().enumerate() {
                let (opts, at) = shuffled((l.n as u64) << 20 | (i as u64) << 1, r, w);
                st.learn(q, &opts, at);
            }
        }
        if l.n != upto {
            continue;
        }
        out += &format!("«{name}» lesson {}: {} — {} frames remembered so far (lessons 1–{})\n\n", l.n, l.title, st.remembered(), l.n);
        for (i, (q, r, w)) in l.tests.iter().enumerate().take(k) {
            let (opts, at) = shuffled((l.n as u64) << 20 | (i as u64) << 1 | 1, r, w);
            let a = st.answer(q, &opts);
            out += &format!("Q: {q}\n  asks: {}{}\n", a.ask.name(), a.gap.map(|g| format!(" (gap in #{g})")).unwrap_or_default());
            for (j, f) in a.qframes.iter().enumerate() {
                out += &format!("  question #{j}: {}\n", f.show_how(0));
            }
            if a.anchors.is_empty() {
                out += "  matched nothing remembered\n";
            }
            for (m, s) in a.anchors.iter().take(4) {
                out += &format!("  matched (strength {s}): {}\n", st.cite(*m));
            }
            for (j, o) in a.options.iter().enumerate() {
                let mark = if j == a.choice { "→" } else { " " };
                let right = if j == at { "  ✓ right" } else { "" };
                out += &format!("  {mark} {}{right}\n", o.text);
                for (c, f) in o.compared.iter().enumerate() {
                    out += &format!("      as #{c}: {}\n", f.show_how(0));
                }
                match &o.best {
                    Some((s, c, m, via, fs)) => out += &format!("      score {s}: #{c} against {:?} {}\n        roles: {}\n", via, st.cite(*m), fs.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>().join(", ")),
                    None => out += "      no remembered frame fills it\n",
                }
            }
            let why = if !a.understood {
                "not understood: no option's frame matched anything the question leads to — guessed the first option".to_string()
            } else {
                let s = a.options[a.choice].best.as_ref().map(|b| b.0).unwrap_or(0);
                let second = a.options.iter().enumerate().filter(|(j, _)| *j != a.choice).filter_map(|(_, o)| o.best.as_ref().map(|b| b.0)).max();
                format!("chosen: the best score {s}{}", second.map(|x| format!(" against the next best {x}")).unwrap_or_else(|| ", the only option with support".into()))
            };
            out += &format!("  {why}\n  {}\n\n", if a.choice == at { "RIGHT" } else { "WRONG" });
        }
    }
    Ok(out)
}

// ---- the hidden teacher: it checks the student's frames, never its answers ------------------------------------------

/// `world knowledge teacher-check <course> <from> <to> [lines-per-lesson]`: for lines sampled evenly from each lesson
/// (in the student's reading, one discourse per lesson), Opus gets the line and the student's frames and says per
/// frame OK or WRONG (with the corrected frame and a short reason), and names an important missing frame. Every
/// verdict, right and wrong, goes to `$MOVA_DATA/runs/prag-teacher/log.tsv`, the calls to `calls.jsonl`. The teacher
/// never sees a test question and its verdicts never feed the score.
pub fn teacher_check(name: &str, from: usize, to: usize, per: usize) -> Result<String> {
    let dir = std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/prag-teacher");
    std::fs::create_dir_all(&dir)?;
    let date = std::process::Command::new("date").arg("+%F").output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
    // (lesson, line, text, frames shown with line-local numbers)
    let mut items: Vec<(usize, usize, String, Vec<String>)> = Vec::new();
    for l in course(name)?.into_iter().filter(|l| (from..=to).contains(&l.n)) {
        let lines: Vec<&String> = l.explain.iter().chain(&l.facts).collect();
        let pick: BTreeSet<usize> = (0..per.min(lines.len())).map(|k| k * lines.len() / per.min(lines.len()).max(1)).collect();
        let mut ctx = Ctx::default();
        for (li, line) in lines.iter().enumerate() {
            let r = crate::frames::read(line, &mut ctx, false)?;
            if pick.contains(&li) {
                items.push((l.n, li, line.to_string(), r.frames.iter().map(|f| f.show(0)).collect()));
            }
        }
    }
    let opus = prag::opus::Opus::from_env(dir.join("empty"));
    let log = dir.join("log.tsv");
    if !log.exists() {
        std::fs::write(&log, "date\tcourse\tlesson\tline\ttext\tmmm_frames\tframe\tverdict\tcorrection\treason\n")?;
    }
    let clean = |x: &str| x.replace(['\t', '\n'], " ");
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for (bi, chunk) in items.chunks(20).enumerate() {
        let mut prompt = String::from("You are a hidden teacher checking how a small transparent language model reads sentences into frames (its pragmatic structure). A frame is one predicate: `Kind agent [NOT] verb patient=… to=(recipient) place=… <prep>=… attrs=… | says→#k (the frames of what was said or thought) cause→#k (its cause or purpose) next→#k (what happened next) quoted if`. Kind: Event (an action), Fact(is a / is / has / can / does / lives in / made of / used for) (a general statement), Say(act) (speech or thought, with its speech act), Bare (no predicate). Entities are head lemmas; a pronoun shows what it was resolved to, e.g. baby(«it»); `_` = no agent found. Frames are numbered from #0 within each line.\n\nFor EACH frame of each line, judge whether it is right: the right predicate, roles (who did what to whom, where), negation, pronoun resolution, and links (cause, next, says). Minor omissions of adjectives are OK. Answer ONLY with lines:\n`id<TAB>#k<TAB>OK<TAB>-<TAB>-` or `id<TAB>#k<TAB>WRONG<TAB>the corrected frame in the same notation<TAB>a short reason`,\nand at most one line per sentence for an important frame the model missed: `id<TAB>+<TAB>MISSING<TAB>the frame<TAB>a short reason`.\n\n");
        for (i, (_, _, text, fs)) in chunk.iter().enumerate() {
            prompt += &format!("id {i}: {text}\n");
            for (k, f) in fs.iter().enumerate() {
                prompt += &format!("  #{k} {f}\n");
            }
            if fs.is_empty() {
                prompt += "  (no frames)\n";
            }
            prompt += "\n";
        }
        let (out, call) = opus.ask("prag-teacher", bi, chunk.len(), &prompt)?;
        prag::opus::append_call(&dir.join("calls.jsonl"), &call)?;
        let mut rows = String::new();
        for (i, (n, li, text, fs)) in chunk.iter().enumerate() {
            let mine: Vec<Vec<&str>> = out.lines().map(|l| l.split('\t').map(str::trim).collect::<Vec<_>>()).filter(|c| c.len() >= 3 && c[0].trim_start_matches("id").trim() == i.to_string()).collect();
            let shown = fs.iter().enumerate().map(|(k, f)| format!("#{k} {f}")).collect::<Vec<_>>().join(" ;; ");
            if mine.is_empty() {
                *counts.entry("NO VERDICT".into()).or_default() += 1;
                rows += &format!("{date}\t{name}\t{n}\t{}\t{}\t{}\t-\tNO VERDICT\t-\t-\n", li + 1, clean(text), clean(&shown));
            }
            for c in mine {
                let v = c[2].to_uppercase();
                *counts.entry(v.clone()).or_default() += 1;
                rows += &format!("{date}\t{name}\t{n}\t{}\t{}\t{}\t{}\t{v}\t{}\t{}\n", li + 1, clean(text), clean(&shown), c[1], clean(c.get(3).unwrap_or(&"-")), clean(c.get(4).unwrap_or(&"-")));
            }
        }
        let mut f = std::fs::OpenOptions::new().append(true).open(&log)?;
        std::io::Write::write_all(&mut f, rows.as_bytes())?;
        eprintln!("batch {bi}: {} lines, {:.0} s", chunk.len(), call.secs);
    }
    Ok(format!("{} lines checked; verdicts: {}; log {}\n", items.len(), counts.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "), log.display()))
}

// ---- the coverage of the register (an authoring tool, not the student: it compares word sets) ----------------

const STOP: [&str; 40] = ["the", "a", "an", "of", "to", "in", "on", "at", "is", "are", "was", "were", "be", "it", "its", "and", "or", "for", "by", "with", "what", "which", "why", "how", "do", "does", "did", "that", "this", "from", "as", "can", "when", "who", "into", "than", "there", "they", "you", "your"];

fn words(t: &str) -> Vec<String> {
    t.split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() > 1).map(|w| crate::mentions::stem(&w.to_lowercase())).filter(|w| !STOP.contains(&w.as_str())).collect()
}

/// A situation of the register (`data/school/situations/<domain>.tsv`): (domain, topic, situation, what is right, why).
pub type Situation = (String, String, String, String, String);

pub fn situations() -> Vec<Situation> {
    let dir = root().join("situations");
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir).map(|r| r.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "tsv")).collect()).unwrap_or_default();
    files.sort();
    let mut v = Vec::new();
    for f in files {
        let domain = f.file_stem().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
        for l in std::fs::read_to_string(&f).unwrap_or_default().lines() {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() == 4 {
                v.push((domain.clone(), c[0].into(), c[1].into(), c[2].into(), c[3].into()));
            }
        }
    }
    v
}

/// Whether a course already teaches a situation: some question with its right answer, or some fact, holds most of the
/// situation's own words (at least two, and at least three fifths of them).
pub fn covered_by(sit: &str, texts: &[BTreeSet<String>]) -> bool {
    let w: BTreeSet<String> = words(sit).into_iter().collect();
    if w.is_empty() {
        return true;
    }
    let need = (w.len() * 3).div_ceil(5).max(2.min(w.len()));
    texts.iter().any(|t| w.iter().filter(|x| t.contains(*x)).count() >= need)
}

/// The coverage of the register by the given courses, and the uncovered situations grouped into lesson-sized batches
/// (by domain and topic, about fifteen each) written to `data/school/queue/<domain>-<k>.tsv` for the authors.
pub fn coverage(courses: &[&str]) -> Result<String> {
    let mut texts: Vec<BTreeSet<String>> = Vec::new();
    for c in courses {
        for l in course(c)? {
            for (q, r, _) in l.exercises.iter().chain(&l.tests) {
                texts.push(words(&format!("{q} {r}")).into_iter().collect());
            }
            for f in &l.facts {
                texts.push(words(f).into_iter().collect());
            }
        }
    }
    let sits = situations();
    let mut by_domain: BTreeMap<String, (usize, Vec<&Situation>)> = BTreeMap::new();
    for s in &sits {
        let e = by_domain.entry(s.0.clone()).or_default();
        e.0 += 1;
        if !covered_by(&s.2, &texts) {
            e.1.push(s);
        }
    }
    let queue = root().join("queue");
    let _ = std::fs::remove_dir_all(&queue);
    std::fs::create_dir_all(&queue)?;
    let mut out = String::new();
    let (mut all, mut open, mut batches) = (0, 0, 0);
    for (d, (n, un)) in &by_domain {
        all += n;
        open += un.len();
        out += &format!("{d}: {n} situations, {} not yet taught\n", un.len());
        // batches of about fifteen, a topic kept together where it can be
        let mut batch: Vec<&Situation> = Vec::new();
        let mut k = 0;
        for s in un {
            if batch.len() >= 15 || (batch.len() >= 10 && batch.last().is_some_and(|b| b.1 != s.1)) {
                k += 1;
                std::fs::write(queue.join(format!("{d}-{k:03}.tsv")), batch.iter().map(|x| format!("{}\t{}\t{}\t{}\n", x.1, x.2, x.3, x.4)).collect::<String>())?;
                batch.clear();
            }
            batch.push(s);
        }
        if !batch.is_empty() {
            k += 1;
            std::fs::write(queue.join(format!("{d}-{k:03}.tsv")), batch.iter().map(|x| format!("{}\t{}\t{}\t{}\n", x.1, x.2, x.3, x.4)).collect::<String>())?;
        }
        batches += k;
    }
    out += &format!("in all: {all} situations, {} taught, {open} not yet; {batches} batches for new lessons in data/school/queue\n", all - open);
    Ok(out)
}

pub fn main_args(args: &[String]) -> Result<()> {
    let name = args.get(2).cloned().unwrap_or_else(|| "world".into());
    match args.get(1).map(String::as_str) {
        Some("balance" | "lint" | "edit" | "snapshot" | "restore-lines") => crate::schooltools::knowledge_args(args)?,
        Some("help" | "--help" | "-h") => print!("{}", crate::schooltools::KNOWLEDGE_USAGE),
        Some("coverage") => {
            let cs: Vec<&str> = if args.len() > 2 { args[2..].iter().map(String::as_str).collect() } else { vec!["everyday", "world"] };
            print!("{}", coverage(&cs)?);
        }
        Some("course") => print!("{}", run_course(&name)?),
        Some("diagnose") => {
            // world knowledge diagnose <examples.md> [courses…] (default: the primary courses)
            let ex = args.get(2).cloned().unwrap_or_else(|| "diagnose-examples.md".into());
            let names: Vec<String> = if args.len() > 3 { args[3..].to_vec() } else { course_names().into_iter().filter(|n| grade(n) == PRIMARY && *n != "dictionary").map(String::from).collect() };
            print!("{}", diagnose(&names, std::path::Path::new(&ex))?);
        }
        Some("teacher-check") => {
            let num = |i: usize, d: usize| args.get(i).and_then(|x| x.parse().ok()).unwrap_or(d);
            print!("{}", teacher_check(&name, num(3, 1), num(4, 1), num(5, 10))?);
        }
        Some("trace") => {
            let n: usize = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(1);
            let k: usize = args.get(4).and_then(|x| x.parse().ok()).unwrap_or(6);
            print!("{}", trace(&name, n, k)?);
        }
        Some("show") => {
            let n: usize = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(1);
            for l in course(&name)?.into_iter().filter(|l| l.n == n) {
                println!("{}. {} ({}, {} stars)", l.n, l.title, l.level, l.stars);
                for f in &l.facts {
                    println!("  fact: {f}");
                }
                for (q, r, w) in l.exercises.iter().chain(&l.tests) {
                    println!("  {q} → {r}   (not: {})", w.join("; "));
                }
            }
        }
        _ => {
            let ls = course(&name)?;
            let mut bad = 0;
            for l in &ls {
                let dup: BTreeSet<&String> = l.exercises.iter().map(|x| &x.0).collect();
                if l.exercises.len() < 12 || l.tests.len() < 8 || l.facts.len() < 5 || dup.len() < l.exercises.len() || l.exercises.iter().chain(&l.tests).any(|x| x.2.len() != 3) {
                    bad += 1;
                    println!("✗ {} ({}): facts {}, exercises {}, tests {}; every question needs exactly three wrong answers", l.n, l.file, l.facts.len(), l.exercises.len(), l.tests.len());
                }
            }
            println!("{} lessons in «{name}», {bad} with a problem", ls.len());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tail_tests {
    use super::*;

    #[test]
    fn tails_split_off_the_scenario() {
        assert_eq!(tail_question("A lizard lies on a warm rock in the morning. Why?"), Some(("A lizard lies on a warm rock in the morning.", Tail::Cause)));
        assert_eq!(tail_question("Two balls are dropped from a wall. What happens?").map(|x| x.1), Some(Tail::Next));
        assert_eq!(tail_question("Mia says the Moon makes its own light. What's true?").map(|x| x.1), Some(Tail::Claim));
        // negative controls: an ordinary question, a tail with too short a scenario
        assert_eq!(tail_question("Why does ice float on water?"), None);
        assert_eq!(tail_question("Look. Why?"), None);
    }
}
