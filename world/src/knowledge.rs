//! Knowledge courses (`courses/<course>/*.lesson` of the school data, `school_data()`, the first: «world» — the world around us, for a very
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
//!   world knowledge course [course] [--trace <out.tsv>]  — the lessons, the student's exercises and tests; with
//!                                         --trace every test, the student's choice, the scores and why (TRACE_HEADER)
//!   world knowledge show <course> <n>  — a lesson's questions
//!   world knowledge diagnose <examples.md> [courses]  — where the student fails: a ladder per test question, by kind
//!   world knowledge trace <course> <n> [k]  — how the student answers a lesson's tests, frame by frame
//!   world knowledge teacher-check <course> <from> <to> [lines]  — the hidden teacher checks the student's frames
//!   (the course «dictionary» is made from the pupils' dictionary: one lesson for each richly described word)
//!   world knowledge coverage [courses]  — how much of the register of situations the courses teach; the rest batched
//!                                         for new lessons in the courses' queue
//!   world knowledge balance|lint|edit|snapshot|restore-lines …  — the school tools (schooltools.rs,
//!                                         docs/school.md#tools); `world knowledge help` prints their usage

mod learning;
pub mod retention;

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

/// The school's data, outside the repository: `$MOVA_SCHOOL_DATA`, by default `$MOVA_DATA/school` (`$MOVA_DATA` defaults
/// to `~/mova-data`). It holds `courses/<course>/*.lesson` and `dictionaries/*.tsv`, the release archive `school-<version>`
/// (`mova fetch 0.5 school`).
pub fn school_data() -> std::path::PathBuf {
    if let Some(d) = std::env::var_os("MOVA_SCHOOL_DATA").filter(|v| !v.is_empty()) {
        return std::path::PathBuf::from(d);
    }
    let data = std::env::var_os("MOVA_DATA").filter(|v| !v.is_empty()).map(std::path::PathBuf::from).unwrap_or_else(|| {
        std::path::PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join("mova-data")
    });
    data.join("school")
}

/// The folder of the courses: `<school_data>/courses`.
pub fn courses_root() -> std::path::PathBuf {
    school_data().join("courses")
}

/// The folder of the school dictionaries: `<school_data>/dictionaries`.
pub fn dictionaries_dir() -> std::path::PathBuf {
    school_data().join("dictionaries")
}

fn root() -> std::path::PathBuf {
    courses_root()
}

fn question(v: &str, file: &str) -> Result<(String, String, Vec<String>)> {
    let parts: Vec<String> = v.split(" | ").map(|x| x.trim().to_string()).collect();
    if parts.len() < 3 {
        bail!("{file}: «{v}» needs a question, the right answer and at least one wrong answer");
    }
    // the right option carries a leading `*` and stands anywhere (the files must not give it away by its place);
    // an unmarked line is the old form, right option first
    let marked: Vec<usize> = (1..parts.len()).filter(|&i| parts[i].starts_with('*')).collect();
    match marked.as_slice() {
        [] => Ok((parts[0].clone(), parts[1].clone(), parts[2..].to_vec())),
        [r] => Ok((parts[0].clone(), parts[*r].trim_start_matches('*').trim().to_string(), (1..parts.len()).filter(|i| i != r).map(|i| parts[i].clone()).collect())),
        _ => bail!("{file}: «{v}» marks more than one right option with *"),
    }
}

/// A text without its references: «(Genesis 12:1)», «(Exodus 20:2-3)», «(1 Samuel 3:4)» — a book named before a
/// chapter and verse is a citation, not a person of the story (17.10: the student took «Genesis» for the most salient
/// character and resolved «he», «him» to it).
pub fn strip_citations(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('(') {
        let (a, b) = rest.split_at(i);
        let Some(j) = b.find(')') else { break };
        let inner = &b[1..j];
        let words: Vec<&str> = inner.split_whitespace().collect();
        let citation = words.len() >= 2
            && words[..words.len() - 1].iter().all(|w| w.chars().next().is_some_and(|c| c.is_uppercase() || c.is_ascii_digit()))
            && words[words.len() - 1].contains(':')
            && words[words.len() - 1].chars().all(|c| c.is_ascii_digit() || c == ':' || c == '-' || c == ',');
        if citation {
            out.push_str(a.trim_end());
            rest = &b[j + 1..];
        } else {
            out.push_str(a);
            out.push('(');
            rest = &b[1..];
        }
    }
    out.push_str(rest);
    out
}

/// A fact as the student reads it. A fact in layers («Genesis 1:1 in Hebrew: … — word for word: … — in English (JPS
/// 1917): … — the idea: …», the Torah course) gives two facts: the English verse and its idea; the Hebrew and the word
/// for word are for the reader of the lesson, not yet for an English student.
fn fact_lines(v: &str) -> Vec<String> {
    if let (Some(e), Some(i)) = (v.find(" — in English (JPS 1917): "), v.find(" — the idea: ")) {
        if e < i {
            let english = &v[e + " — in English (JPS 1917): ".len()..i];
            let idea = &v[i + " — the idea: ".len()..];
            return vec![english.trim().to_string(), idea.trim().to_string()];
        }
    }
    vec![strip_citations(v)]
}

fn parse_fields(text: &str, file: &str) -> Result<Lesson> {
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
            "explain" => l.explain.push(strip_citations(&v)),
            "fact" => l.facts.extend(fact_lines(&v)),
            "exercise" => l.exercises.push(question(&v, file)?),
            "test" => l.tests.push(question(&v, file)?),
            "exam" => l.exams.push(question(&v, file)?),
            other => bail!("{file}: unknown field «{other}»"),
        }
    }
    Ok(l)
}

pub fn parse(text: &str, file: &str) -> Result<Lesson> {
    let l = parse_fields(text,file)?;
    if l.n == 0 || l.facts.is_empty() || l.exercises.is_empty() || l.tests.is_empty() {
        bail!("{file}: a lesson needs its number, facts, exercises and tests");
    }
    Ok(l)
}

/// The knowledge courses: (name — the folder under the courses of the school data, the site's folder, title, what it teaches).
/// The school a course belongs to. The small one is taught the primary school only for now:
/// a course too hard for a child of 9–10 waits for the middle school.
pub const PRIMARY: &str = "primary school";
/// The mystic school: a separate layer on top of the primary base (the course «mysticism», reading docs/mysticism).
/// It is not in the primary course list, and its dictionary is its own layer over the primary one (dictionary.rs).
pub const MYSTIC: &str = "mystic school";
/// The Judaism school: a layer on top of the primary base, like the mystic school (operator, 17.10: «grow the religious
/// materials — lessons, dictionaries»): the course «torah», one lesson per weekly portion, and the Judaism dictionary;
/// «torah-he», the same lessons adapted into Modern Hebrew (18.10: the old languages are the object of the lessons,
/// never their medium — the quoted verses stay in Biblical Hebrew with their points). The SLM does not read Hebrew yet.
pub const JUDAISM: &str = "judaism school";
/// The grammar school: a layer on top of the primary base, like the mystic and Judaism schools: one course per grammar
/// manual of `languages/<code>/` (`grammar-<code>`), two to four lessons a chapter, the chapter's examples kept with
/// their ids (the dictionary builder's `lessons grammar`). Its courses do not mix into the primary grade.
pub const GRAMMAR: &str = "grammar school";
pub const GRADES: &[(&str, &str)] = &[
    ("programming", "middle school"),
    ("philosophy", "middle school"),
    ("mysticism", MYSTIC),
    ("torah", JUDAISM),
    ("torah-he", JUDAISM),
    ("grammar-en", GRAMMAR),
    ("grammar-en-x-emode", GRAMMAR),
    ("grammar-hbo", GRAMMAR),
    ("grammar-he", GRAMMAR),
    ("grammar-arc", GRAMMAR),
    ("grammar-arc-x-targum", GRAMMAR),
    ("grammar-tmr", GRAMMAR),
    ("grammar-jpa", GRAMMAR),
    ("grammar-arc-x-zohar", GRAMMAR),
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
    ("commands", "commands", "The command school", "lessons for a child of 9–10 on what people want done when they speak: please and could you, questions that are really requests, who is told and who is only mentioned, orders passed on, don't, unless, first and then, right now or later, several jobs at once, vague and unsafe orders, saying no, reporting back — and commands to pets, robots, game characters and voice assistants"),
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
    ("mysticism", "mysticism", "The mystic school", "lessons for an older pupil who reads the family's fifteen mysticism notes (docs/mysticism): the creation myth and its three laws, the Gali working habits, the Medium of the Internet, the Godmother Matrix, Cain's Kyiv, the independent debugger, the holiday in hell, the numbers, the frog code, and the notes on 10 to 15 — what each note says, why, and the misreadings to avoid"),
    ("torah", "torah", "The Torah, portion by portion", "lessons of the Judaism school, one for each of the 54 weekly portions (parashot) read in the synagogue through the year: what happens, the commandments given, the names and places, and three key verses in layers — the Hebrew, word for word, the JPS 1917 English and the tradition's reading"),
    ("torah-he", "torah-he", "The Torah, portion by portion, in Modern Hebrew", "the Torah course adapted into Modern Hebrew as an Israeli Tanakh class teaches it: school Hebrew in the lessons, kids' talk and slang in the tests, matriculation-exam Hebrew in the exams; the key verses stay in Biblical Hebrew with their points, with a word-by-word gloss, the verse in today's Hebrew and the commentators' reading"),
    ("grammar-en", "grammar-en", "The grammar of English", "the grammar school's course on the English manual (languages/en): sounds and spelling, word classes and forms, word formation, syntax, pragmatics and discourse — the chapter's rules, counts and examples with their ids"),
    ("grammar-en-x-emode", "grammar-en-x-emode", "The grammar of Early Modern English", "the grammar school's course on the Early Modern English manual (languages/en-x-emode): the King James Bible and Shakespeare's English — spelling, thou and ye, -eth and -est, strong verbs, do and negation, word order, the subjunctive, biblical style"),
    ("grammar-hbo", "grammar-hbo", "The grammar of Biblical Hebrew", "the grammar school's course on the Biblical Hebrew manual (languages/hbo): the script and the accents, sounds, the noun, pronouns and particles, the verb and the weak verbs, syntax, discourse, the lexicon and Mishnaic Hebrew — with the Torah's verses and their ids"),
    ("grammar-he", "grammar-he", "The grammar of Modern Hebrew", "the grammar school's course on the Modern Hebrew manual (languages/he): script and spelling, sounds, nouns, adjectives and pronouns, the verb and its paradigms, prepositions, syntax, register, politeness and the historical layers"),
    ("grammar-arc", "grammar-arc", "The grammar of Biblical Aramaic", "the grammar school's course on the Aramaic overview and Biblical Aramaic manual (languages/arc): the Aramaic family of the Jewish texts, script and sounds, nouns, pronouns, verbs, particles and syntax, the lexicon"),
    ("grammar-arc-x-targum", "grammar-arc-x-targum", "The Aramaic of the Targums", "the grammar school's course on the manual of the targums' literary Aramaic (languages/arc-x-targum): spelling and sounds, nouns and pronouns, verbs, the syntax of translation, the lexicon"),
    ("grammar-tmr", "grammar-tmr", "The Aramaic of the Babylonian Talmud", "the grammar school's course on the Jewish Babylonian Aramaic manual (languages/tmr): spelling and sounds, nouns, pronouns, verbs, particles and syntax, the Talmud's terms of discussion, the lexicon"),
    ("grammar-jpa", "grammar-jpa", "The Aramaic of the Jerusalem Talmud", "the grammar school's course on the Jewish Palestinian Aramaic manual (languages/jpa): spelling and sounds, nouns and pronouns, verbs, particles and syntax, the lexicon"),
    ("grammar-arc-x-zohar", "grammar-arc-x-zohar", "The Aramaic of the Zohar", "the grammar school's course on the manual of the Zohar's literary Aramaic (languages/arc-x-zohar): its grammar, style and formulas, the lexicon"),
];

/// The names of the knowledge courses (no «dictionary», which is made from the dictionary itself).
pub fn course_names() -> Vec<&'static str> {
    COURSES.iter().map(|c| c.0).filter(|n| *n != "dictionary" && grade(n) == PRIMARY).collect()
}

/// A course's folder: `<courses_root>/<name>`. In the closed build (`peta`) the Wikipedia school's courses
/// (`wikipedia`, `wikipedia-<part>`) are CC BY-SA and live outside the repo, beside their corpus:
/// `$MOVA_WIKISCHOOL/<name>`, by default `$MOVA_DATA/corpus/wikipedia/school/<name>` (an internal tool, not in the public tree, writes
/// them; never in a release).
pub fn course_dir(name: &str) -> std::path::PathBuf {
    #[cfg(feature = "peta")]
    if name == "wikipedia" || name.starts_with("wikipedia-") {
        let school = std::env::var("MOVA_WIKISCHOOL").map(std::path::PathBuf::from).unwrap_or_else(|_| {
            std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("corpus/wikipedia/school")
        });
        return school.join(name);
    }
    root().join(name)
}

/// The lessons of a course, by number.
pub fn course(name: &str) -> Result<Vec<Lesson>> {
    if name == "dictionary" {
        return Ok(dictionary_course());
    }
    let dir = course_dir(name);
    // a missing course folder reads as an empty course (the placeholders), but a missing school is an error
    anyhow::ensure!(dir.is_dir() || root().is_dir() || !dir.starts_with(root()), "course «{name}»: the school data is not installed ({} is missing): mova fetch 0.5 school, or set MOVA_SCHOOL_DATA", root().display());
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
    /// the same verb, one negated and one not. Tried 22.10 also with another verb when the doer and the thing both
    /// match («jaguars never eat turtles» against «jaguars hunt turtles»): 25 courses −22 (odd lessons −42, even
    /// +20; books −30); not kept
    NegDisagree,
    /// the option's own «because» matches the remembered cause
    CauseLink,
    /// different verbs of the same class (global::verb_class: give, get, lose …) — tried 10.10 as a weak match:
    /// −184 on biology, −206 on everyday (too broad); kept unused so the weight table keeps its shape
    VerbClass,
    /// a «how many / how much» question: the option's number is the quantity of the asked thing in the remembered
    /// frame («fifty righteous people», «400 shekels»), or a number standing alone beside it («down to ten»)
    Quantity,
}

const NF: usize = 17;
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

const fn feat_idx(f: Feat) -> usize {
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
        Feat::Quantity => 16,
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
    let feats: Vec<Feat> = SLOTS.iter().map(|&s| Feat::Match(s)).chain([Feat::LightVerb, Feat::Asked, Feat::Conflict(Slot::Verb), Feat::Conflict(Slot::Agent), Feat::Conflict(Slot::Patient), Feat::Conflict(Slot::Place), Feat::NegDisagree, Feat::CauseLink, Feat::VerbClass, Feat::Quantity]).collect();
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
                    // −8 tried 18.10 («the midwives did not kill the boys» lost to «they killed the baby boys»): biology
                    // plain −21, ★ +5, ★★ +2, the rest ±2 — kept at −3
                    Feat::NegDisagree => -3,
                    Feat::CauseLink => 2,
                    Feat::VerbClass => 1,
                    Feat::Quantity => 3,
                };
                w[widx(a, v, f)] = base * rel;
            }
        }
    }
    w
}

const LIGHT: [&str; 5] = ["be", "have", "do", "get", "go"];

/// Lesson sentences compiled into rules, condition ⇒ conclusion over shared placeholders, and applied to questions
/// (`PragStudent::apply_rules`; PRAG_OFF=student-rules). Measured 2026-11-04 on the ten courses of section 29 of
/// docs/training.md: 3 248 → 3 260 (odd +6, even +6), exam 997 → 998; paired +14 / −2 (McNemar z 3.0), all from the 46
/// rows a rule decided; the cross-lesson control flat (2 174 → 2 176).
fn rules_on() -> bool {
    !prag::frames::off("student-rules")
}

/// A placeholder of a rule: someone, anything, X … («if someone asks», «"Don't forget X"»).
fn placeholder(e: &Ent) -> bool {
    matches!(e.id.as_str(), "someone" | "somebody" | "anyone" | "anybody" | "everyone" | "everybody" | "whoever" | "something" | "anything" | "whatever" | "x" | "y" | "z" | "…" | "...")
}

/// A role of a rule's condition that any entity fills: a placeholder, the generic «you» the rule speaks to.
fn pattern_var(e: &Ent) -> bool {
    placeholder(e) || generic(e) || e.id == "you"
}

/// A verb that is a placeholder: «"Can you…?"», «can you X?».
fn verb_var(v: &str) -> bool {
    matches!(v, "" | "…" | "..." | "x" | "y" | "z")
}

/// An entity that says nothing about which one it is: the generic persons and the pronouns.
fn weak_ent(e: &Ent) -> bool {
    generic(e) || prag::frames::UNRESOLVED_PUB.contains(&e.id.as_str()) || matches!(e.id.as_str(), "i" | "you" | "we" | "me" | "us" | "one")
}

/// The verbs of a conclusion that carry their complement and say nothing themselves («they want you to do it», «it
/// means stop»): skipped when the conclusion has a complement.
const CARRIERS: [&str; 8] = ["want", "need", "mean", "ask", "tell", "expect", "like", "wish"];

const CORE4: [Slot; 4] = [Slot::Agent, Slot::Patient, Slot::Recipient, Slot::Place];

/// The binding a question gives a rule's condition: its placeholders → the question's entities, and the question's
/// frame that matched (the action a placeholder verb or a «do it» of the conclusion stands for).
#[derive(Clone, Debug, Default)]
struct Theta {
    ents: BTreeMap<String, Ent>,
    inst: Option<Frame>,
    action: bool,
    /// the condition was a quoted pattern (an utterance)
    quoted: bool,
}

/// The generic persons: «you» of every lesson and imperative, «we», «one», «people».
fn generic(e: &Ent) -> bool {
    !prag::frames::off("student-generic") && matches!(e.id.as_str(), "you" | "we" | "one" | "people" | "i") && e.pron.is_none()
}

/// Other words for a noun in the loaded dictionary's layers with nyms (`dictstore::EXTRA_COLUMNS`, the Wiktionary
/// module): the synonyms of its noun entries and the lemmas they are a form of («brekkie» → breakfast), lower case,
/// in layer and sense order, without the word itself. Only knowledge layers are read (`Store::knowledge`). Cached
/// for the process: the store is fixed while it runs.
fn dict_nyms(id: &str) -> Vec<String> {
    static CACHE: std::sync::OnceLock<std::sync::Mutex<BTreeMap<String, Vec<String>>>> = std::sync::OnceLock::new();
    let c = CACHE.get_or_init(|| std::sync::Mutex::new(BTreeMap::new()));
    if let Some(v) = c.lock().unwrap().get(id) {
        return v.clone();
    }
    let mut v: Vec<String> = Vec::new();
    for e in crate::dictstore::store().knowledge(id).filter(|e| e.pos == "noun") {
        let lemmas = e.col("form of").split("; ").filter_map(|p| p.rsplit_once(':')).map(|(_, l)| l.trim().to_lowercase());
        for w in e.terms("synonyms").into_iter().chain(lemmas) {
            if !w.is_empty() && w != id && !v.contains(&w) {
                v.push(w);
            }
        }
    }
    c.lock().unwrap().insert(id.to_string(), v.clone());
    v
}

/// Verb synonyms from the dictionary (WordNet: the members of a verb's first two senses), e.g. put down = set down.
fn verb_synonyms(v: &str) -> BTreeSet<String> {
    static CACHE: std::sync::OnceLock<std::sync::Mutex<BTreeMap<String, BTreeSet<String>>>> = std::sync::OnceLock::new();
    let c = CACHE.get_or_init(|| std::sync::Mutex::new(BTreeMap::new()));
    if let Some(s) = c.lock().unwrap().get(v) {
        return s.clone();
    }
    let wn = crate::wordnet::wordnet();
    let s: BTreeSet<String> = wn.senses.get(v).map(|ss| ss.iter().filter(|x| x.pos == "verb").take(2).flat_map(|x| x.synonyms.iter().map(|y| y.to_lowercase())).collect()).unwrap_or_default();
    c.lock().unwrap().insert(v.to_string(), s.clone());
    s
}

/// The same verb, or synonyms each in the other's first senses (stricter than `verb_same`: «take» lists «need»
/// among its senses, «need» does not list «take»).
fn verb_strict(a: &str, b: &str) -> bool {
    a == b || !LIGHT.contains(&a) && !LIGHT.contains(&b) && verb_synonyms(a).contains(b) && verb_synonyms(b).contains(a)
}

fn verb_same(a: &str, b: &str) -> bool {
    a == b || (!LIGHT.contains(&a) && !LIGHT.contains(&b) && (verb_synonyms(a).contains(b) || verb_synonyms(b).contains(a)))
}

/// How the question's verb gates an option's support (`PragStudent::answer_with`): the frames that fill the question's
/// gap (a bare option), or every option frame under a role question (PRAG_VN=student-verb-all: ten courses −223); a
/// remembered frame with a light or no verb (a copula, «have») neither states nor contradicts the question's verb and
/// counts, or does not (PRAG_VN=student-verb-strict: the Wikipedia verb control 33.5% instead of 39.4%, the ten courses
/// −11 instead of −1). Measured 2026-11-04 (docs/training.md section 31): the Wikipedia verb control keeps the
/// original answer 39% of the time instead of 80%, the originals rise with the reader's fixes, the ten courses −1
/// (PRAG_OFF=student-verb).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct VerbGate {
    all: bool,
    strict: bool,
}

fn verb_gate() -> Option<VerbGate> {
    (!prag::frames::off("student-verb")).then(|| VerbGate { all: prag::frames::vn_opt("student-verb-all"), strict: prag::frames::vn_opt("student-verb-strict") })
}

/// A predicate that says nothing of the event itself: the light verbs, the copular ones, the event predicates
/// («happen», «occur», «take place») whose subject names the event, and the verbs whose object is a content — the
/// carriers («want», «mean»), the verbs of saying and those VerbNet gives a clause («show that», «characterise»,
/// «decide to»): what they ask is not a participant a remembered frame of the same verb holds.
fn verb_light_q(f: &Frame) -> bool {
    let v = f.verb.as_str();
    let head = verb_spelling(v.split_whitespace().next().unwrap_or(v));
    let head = head.as_str();
    v.is_empty() || verb_var(v) || LIGHT.contains(&head) || matches!(head, "become" | "remain" | "seem" | "happen" | "occur" | "exist")
        || head == "take" && f.patient.as_ref().is_some_and(|p| p.id == "place")
        || CARRIERS.contains(&head) || verb_vn_spelling(head).is_some_and(|x| prag::vn::speech_verb(&x) || prag::vn::clausal(&x))
}

/// A verb as VerbNet spells it (American «-ize»), when VerbNet has it.
fn verb_vn_spelling(v: &str) -> Option<String> {
    let has = |x: &str| !prag::vn::lex().classes_of(x).is_empty();
    let z = v.strip_suffix("ise").map(|b| format!("{b}ize"));
    std::iter::once(v.to_string()).chain(z).find(|x| has(x))
}

/// A verb lemma as the lexicons spell it: the reader lemmatises British «characterises» as «characteris»; a lemma
/// WordNet lacks takes its «-e», then its American «-ize» form when WordNet has that.
fn verb_spelling(v: &str) -> String {
    let wn = crate::wordnet::wordnet();
    let known = |x: &str| wn.senses.get(x).is_some_and(|ss| ss.iter().any(|s| s.pos == "verb"));
    if known(v) {
        return v.to_string();
    }
    let e = format!("{v}e");
    let z = v.strip_suffix("is").or_else(|| v.strip_suffix("ise")).map(|b| format!("{b}ize"));
    [Some(e), z].into_iter().flatten().find(|x| known(x)).unwrap_or_else(|| v.to_string())
}

/// The synsets of a verb's first two senses, and those with their hypernyms up to `up` levels.
fn verb_synsets(v: &str, up: usize) -> BTreeSet<String> {
    static CACHE: std::sync::OnceLock<std::sync::Mutex<BTreeMap<(String, usize), BTreeSet<String>>>> = std::sync::OnceLock::new();
    let c = CACHE.get_or_init(|| std::sync::Mutex::new(BTreeMap::new()));
    let key = (v.to_string(), up);
    if let Some(s) = c.lock().unwrap().get(&key) {
        return s.clone();
    }
    let wn = crate::wordnet::wordnet();
    let mut out: BTreeSet<String> = wn.senses.get(v).map(|ss| ss.iter().filter(|x| x.pos == "verb").take(2).map(|x| x.synset.clone()).collect()).unwrap_or_default();
    let mut level: Vec<String> = out.iter().cloned().collect();
    for _ in 0..up {
        let next: Vec<String> = level.iter().flat_map(|s| wn.up.get(s).map(|x| x.0.clone()).unwrap_or_default()).filter(|h| !out.contains(h)).collect();
        out.extend(next.iter().cloned());
        level = next;
    }
    c.lock().unwrap().insert(key, out.clone());
    out
}

/// The remembered verb `m` states the question's verb `q`: the same, a synonym (`verb_same`), or one a kind of the
/// other within two levels of WordNet's verb hypernyms («found» / «establish», «build» / «construct»).
fn verb_agrees(q: &str, m: &str) -> bool {
    if verb_same(q, m) {
        return true;
    }
    // a verb with its particle and without («pull down» / «pull», «bring along» / «bring»): the same verb
    let (hq, hm) = (q.split_whitespace().next().unwrap_or(q), m.split_whitespace().next().unwrap_or(m));
    if hq != q || hm != m {
        return verb_agrees(hq, hm);
    }
    let (q, m) = (verb_spelling(q), verb_spelling(m));
    let (q, m) = (q.as_str(), m.as_str());
    if q == m {
        return true;
    }
    let (sq, sm) = (verb_synsets(q, 0), verb_synsets(m, 0));
    !sq.is_empty() && !sm.is_empty() && (!verb_synsets(m, 2).is_disjoint(&sq) || !verb_synsets(q, 2).is_disjoint(&sm))
}

/// A polarity contradiction: the lesson denies what the option asserts — the same content verb, a shared doer, thing,
/// recipient or place, the remembered frame negated and the option's frame not («daylight does NOT blind owls»
/// against «the sun blinds it»). An irrealis difference («if you butter …») is no denial. The other direction (the
/// option denies what a lesson affirms) counts only with PRAG_VN=student-neg-both: the reader often puts an option's
/// «not» on a conjunct or a clause it does not govern («It does not help; keep her inside» → NOT keep).
fn contradicts(fs: &[Feat], o_neg: bool, m_neg: bool) -> bool {
    o_neg != m_neg
        && fs.contains(&Feat::Match(Slot::Verb))
        && CORE4.iter().any(|&s| fs.contains(&Feat::Match(s)))
        && (m_neg || prag::frames::vn_opt("student-neg-both"))
}

/// The score of a witness: its weighted features, or, for a polarity contradiction with a positive sum, that sum
/// negated — a matching frame that says the opposite counts against, never for (PRAG_OFF=student-neg-counter).
fn counter(raw: i64, fs: &[Feat], o_neg: bool, m_neg: bool) -> i64 {
    if raw > 0 && contradicts(fs, o_neg, m_neg) && !prag::frames::off("student-neg-counter") {
        -raw
    } else {
        raw
    }
}

/// The strength of a match: a content verb or an entity in the same role 2, a property 1.
fn strength(fs: &[Feat]) -> i64 {
    fs.iter().map(|f| match f {
        Feat::Match(Slot::Attr) | Feat::VerbClass => 1,
        Feat::Match(_) => 2,
        _ => 0,
    }).sum()
}

/// Sum align's set-valued features without allocating a Vec. The seven role bits
/// contribute 2 each except Attr, which contributes 1; all other align bits add 0.
/// Warren (2013), Hacker's Delight, 2nd ed., ch. 5 (population count).
/// An exhaustive u16 test checks the independent feature-list interpretation.
fn strength_mask(mask: u16) -> i64 {
    const ATTR: u16 = 1 << feat_idx(Feat::Match(Slot::Attr));
    const ROLES: u16 = (1 << 7) - 1;
    2 * (mask & (ROLES & !ATTR)).count_ones() as i64 + (mask & ATTR != 0) as i64
}

fn strength_masked() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var("MOVA_STRENGTH").as_deref() == Ok("mask"))
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

/// Score, compared-frame index, remembered-frame index, route and active features.
pub type Support = (i64, usize, usize, Via, Vec<Feat>);

/// One option as the student read it, and its best support.
#[derive(Clone, Debug, Default)]
pub struct OptionRead {
    pub text: String,
    /// the option's own frames, and the frames it is compared as (a bare phrase put into the question's gap)
    pub frames: Vec<Frame>,
    pub compared: Vec<Frame>,
    /// The witness of the active scoring scope: current lesson when it has support,
    /// all remembered lessons otherwise. Used by explanations and learning alike.
    pub best: Option<Support>,
    /// its unsupported content: the verbs, entities and properties of its frames that no remembered frame holds (a
    /// supported frame against its best match, an unsupported frame all of it) — between two tied options, one saying
    /// all the other says and more, the one that claims more beyond the support loses
    pub extra: usize,
    /// its best score against a frame of the lesson just read (the lesson-first choice)
    pub recent: Option<i64>,
    /// the positive best scores of all its compared frames summed: between tied options, the one more of whose claims
    /// are supported (tried as a tie-breaker, PRAG_VN=student-tie-sum; it loses)
    pub sum: i64,
    /// its best score from remembered frames the question's verb gate left out (their verb does not state the
    /// question's): never support, only the order of options tied on their support
    pub assoc: Option<i64>,
}

/// Restrict explanations and learning to the same witnesses as the decision.
/// With current-lesson support, an option supported only by older lessons is
/// ineligible; its old witness must not silently become a training target.
fn current_support(options: &mut [OptionRead], witnesses: Vec<Option<Support>>, enabled: bool) -> bool {
    if !enabled || !witnesses.iter().any(Option::is_some) {
        return false;
    }
    assert_eq!(options.len(), witnesses.len());
    for (o, witness) in options.iter_mut().zip(witnesses) {
        o.best = witness;
    }
    true
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
    /// a counting question worked out from the kind base, when one option holds its number
    pub count: Option<Count>,
    /// the rules applied to it (PRAG_OFF=student-rules): decided when exactly one option says a fired rule's conclusion
    pub rule: Option<RuleUse>,
}

/// The pragmatic student: what it remembers is frames.
pub struct PragStudent {
    /// Lesson-local focus is disabled for persistent, closed-book examinations.
    lesson_focus: bool,
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
    /// the kind base: how many of a part a kind has, as the lessons say it («insects have six legs», «apes have no
    /// tail»): (the kind, the part) → (the number, the line that said it); the latest saying wins
    parts: BTreeMap<(String, String), (i64, (usize, usize))>,
    /// the kinds the lessons name («a woodlouse is a crustacean»): the kind → (its kind, the line)
    isa: BTreeMap<String, (String, (usize, usize))>,
    /// a part counted on each of another part («four toes on each foot»): (the kind, the part) → (the number, the
    /// other part, the line)
    per: BTreeMap<(String, String), (i64, String, (usize, usize))>,
    /// the rules the remembered lines state, condition ⇒ conclusion (PRAG_OFF=student-rules), compiled from the frames
    /// as they come: nothing of its own is kept, so a retained state compiles them again
    rulebase: std::cell::RefCell<RuleBase>,
    /// how the question's verb gates support (`verb_gate`, from the switches at creation; nothing saved)
    verb_gate: Option<VerbGate>,
    /// the question's own frames are being coded for anchors and its things (where the dictionary's kinds apply)
    q_side: std::cell::Cell<bool>,
}

/// A rule a remembered line states (`prag::frames::Implication`, its frames as remembered-frame indices).
#[derive(Clone, Debug)]
struct Rule {
    src: (usize, usize),
    conn: String,
    cond: Vec<usize>,
    concl: Vec<usize>,
    pro: Vec<usize>,
}

/// The compiled rules and their index by the condition's verb (the verb and its synonyms; a pattern whose verb is a
/// placeholder, «Can you…?», under `var_verb`).
#[derive(Default)]
struct RuleBase {
    /// the frames compiled so far (a line is compiled whole: lines are read whole before any question)
    upto: usize,
    rules: Vec<Rule>,
    by_verb: BTreeMap<String, Vec<usize>>,
    var_verb: Vec<usize>,
}

/// One application of a rule to a question: which rule, what its condition matched and bound, and the conclusion the
/// chosen option says.
#[derive(Clone, Debug)]
pub struct RuleUse {
    /// the rules whose condition the question matched, and the options a conclusion fits
    pub fired: usize,
    pub fitting: usize,
    /// the explanation, when a rule decided
    pub how: Option<String>,
    /// for the diagnostic: the first fired rules' lines and conclusions under their bindings
    pub notes: Vec<String>,
}

/// Whether a course skips its tests before the exercises (MOVA_COURSE_BEFORE=0).
fn skip_before() -> bool {
    static C: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *C.get_or_init(|| std::env::var("MOVA_COURSE_BEFORE").as_deref() == Ok("0"))
}

/// The cost model's caps and counters (MOVA_TIMING): for each cap, the candidates every lookup would take under it
/// summed over the run, Σ min(cap, n) of the lists it reads — the work of a cap predicted from one run.
const COST_CAPS: [usize; 6] = [50, 100, 300, 1000, 4000, usize::MAX];
static COST: [std::sync::atomic::AtomicU64; 6] = [const { std::sync::atomic::AtomicU64::new(0) }; 6];
static STRENGTH_N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// How many of the most recent frames of one entity or verb are candidates (MOVA_CAND_CAP, default 100). Measured
/// 2026-10-23 on five courses, controls off: books 7 302 → 7 326 of 24 316 at 300 against 4 000, its frame comparisons
/// 1.51 → 0.62 billion and its time 6 119 → 3 181 s; biology, history and world the same, everyday −2 of 7 211. And
/// 2026-10-24 on the 25 courses, 100 against 300: 19 439 → 19 512 of 49 635 (odd +44, even +29), half the comparisons
/// again, as the cost model had predicted (Books 0.49 of the work, measured 0.51).
fn cand_cap() -> usize {
    static C: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *C.get_or_init(|| std::env::var("MOVA_CAND_CAP").ok().and_then(|x| x.parse().ok()).unwrap_or(100))
}

/// The source number of the body plans' lines (`data/kinds.txt`): not a lesson.
const KINDS_SRC: usize = usize::MAX;

/// Where a kind-base saying came from, in words.
fn src_plain(src: (usize, usize)) -> String {
    if src.0 == KINDS_SRC { format!("the body plans, line {}", src.1 + 1) } else { format!("lesson {}, line {}", src.0, src.1 + 1) }
}

/// A counting question worked out from the kind base: the number, and how («3 lion × 4 leg (lion → … → mammal,
/// L12.3) + 2 tiger × 4 leg … = 20»).
#[derive(Clone, Debug)]
pub struct Count {
    pub value: i64,
    pub how: String,
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
    /// not said to have happened (`Frame::irrealis`)
    irr: bool,
    /// the four core slots (agent, patient, recipient, place): index into `ents`
    core: [Option<u8>; 4],
    /// `Frame::ents` in its order: the core slots, then the obliques
    ents: Vec<MEnt>,
    /// its properties and its entities' (sorted)
    attrs: Vec<u32>,
    /// the quantities of its entities: (the entity, the number) — «400 shekels» (shekel, 400); a number standing
    /// alone («down to ten») has the entity 0
    qtys: Vec<(u32, i64)>,
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
    irr: bool,
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
        PragStudent { lesson_focus: true, frames: Vec::new(), by_ent: BTreeMap::new(), by_verb: BTreeMap::new(), prev: BTreeMap::new(), effect: BTreeMap::new(), sayer: BTreeMap::new(), by_line: BTreeMap::new(), same: BTreeMap::new(), texts: BTreeMap::new(), w: start_weights(), trees: Default::default(), reads: Default::default(), codes: Vec::new(), names: Default::default(), vsame: Default::default(), parts: BTreeMap::new(), isa: BTreeMap::new(), per: BTreeMap::new(), rulebase: Default::default(), verb_gate: verb_gate(), q_side: Default::default() }
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
    pub fn preparse(&self, texts: &[(String, bool)]) -> Result<()> {
        self.preparse_with(texts, crate::exec::parse_many)
    }

    fn preparse_with(&self, texts: &[(String, bool)], parse: impl FnOnce(&[String]) -> Result<Vec<Vec<en::annotate::Word>>>) -> Result<()> {
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
            return Ok(());
        }
        // CPU threads, or GPU batches (MOVA_EXEC=gpu, `exec`)
        let parsed = parse(&todo)?;
        anyhow::ensure!(parsed.len() == todo.len(), "preparse returned a partial batch");
        let mut tr = self.trees.borrow_mut();
        for (u, w) in todo.into_iter().zip(parsed) {
            tr.insert(u, w);
        }
        Ok(())
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
                self.remember_frame(f);
            }
        }
    }

    /// Keep a frame read from a lesson line (its `src` set, its links already shifted): index it by its entities, its
    /// content verb, its links and its line, and tell the kind base.
    fn remember_frame(&mut self, f: Frame) {
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
        self.learn_kind(&f);
        let c = self.mcode(&f);
        self.codes.push(c);
        self.frames.push(f);
    }

    fn alias(&mut self, a: &str, b: &str) {
        self.same.entry(a.to_string()).or_default().insert(b.to_string());
        self.same.entry(b.to_string()).or_default().insert(a.to_string());
    }

    /// What a frame tells the kind base (PRAG_OFF=student-parts): «insects have six legs», «a skunk has four legs»,
    /// «apes have no tail», «they do not have a tail» → (the kind, the part, the number); «a woodlouse is a
    /// crustacean» → the kind's kind.
    fn learn_kind(&mut self, f: &Frame) {
        if prag::frames::off("student-parts") || f.irrealis {
            return;
        }
        let kind_ok = |e: &Ent| e.pron.is_none() && !e.named && !generic(e) && !prag::frames::UNRESOLVED_PUB.contains(&e.id.as_str()) && crate::numbers::quantity(&e.text).is_none();
        let Some(a) = f.agent.as_ref().filter(|a| kind_ok(a)) else { return };
        let kinds: Vec<String> = std::iter::once(a.id.clone()).chain(a.and.iter().cloned()).collect();
        if f.kind == Kind::Fact(crate::frames::Link::IsA) && !f.neg {
            // «a husky is a kind of dog»: the kind is the «of» phrase (with the rules, PRAG_OFF=student-rules)
            let kind_of = f.patient.as_ref().filter(|p| rules_on() && matches!(p.id.as_str(), "kind" | "type" | "sort")).and_then(|p| p.mods.iter().find(|(c, _)| c == "of").map(|x| &x.1));
            if let Some(p) = kind_of.or(f.patient.as_ref()).filter(|p| p.pron.is_none() && !p.named && p.id != a.id) {
                for k in &kinds {
                    self.isa.insert(k.clone(), (p.id.clone(), f.src));
                }
            }
            return;
        }
        if f.verb != "have" {
            return;
        }
        let Some(p) = f.patient.as_ref() else { return };
        for part in std::iter::once(p).chain(&p.conj) {
            if part.pron.is_some() || part.named {
                continue;
            }
            let det = part.det.as_ref().map(|d| d.0.to_lowercase());
            let n = if f.neg || det.as_deref() == Some("no") {
                Some(0)
            } else if let Some(v) = crate::numbers::quantity(&part.text) {
                Some(v)
            } else if !part.plural && matches!(det.as_deref(), Some("a" | "an" | "one")) {
                Some(1)
            } else {
                None
            };
            // «on each foot», «per foot», «on every hand»: counted on each of another part
            let each = |e: &Ent| e.det.as_ref().is_some_and(|d| matches!(d.0.to_lowercase().as_str(), "each" | "every"));
            let unit = part.mods.iter().chain(&f.obls).find(|(c, e)| (c == "per" || matches!(c.as_str(), "on" | "in" | "of") && each(e)) && e.id != part.id).map(|(_, e)| e.id.clone())
                .or_else(|| f.place.as_ref().filter(|e| each(e) && e.id != part.id).map(|e| e.id.clone()));
            if let Some(n) = n {
                for k in &kinds {
                    match &unit {
                        Some(u) => {
                            self.per.insert((k.clone(), part.id.clone()), (n, u.clone(), f.src));
                        }
                        None => {
                            self.parts.insert((k.clone(), part.id.clone()), (n, f.src));
                        }
                    }
                }
            }
        }
    }

    /// The body plans (`data/kinds.txt`), read into the kind base only, before any lesson (PRAG_OFF=student-body-plans):
    /// what every child knows of legs, wings and fingers, which a lesson about a narrower kind overrides.
    pub fn read_kinds(&mut self) {
        if prag::frames::off("student-body-plans") || prag::frames::off("student-parts") {
            return;
        }
        let mut ctx = Ctx::default();
        for (li, line) in include_str!("../data/kinds.txt").lines().enumerate().filter(|(_, l)| !l.starts_with('#') && !l.trim().is_empty()) {
            let r = self.read_text(line, &mut ctx, false);
            for mut f in r.frames {
                f.src = (KINDS_SRC, li);
                self.learn_kind(&f);
            }
        }
    }

    /// How many of `part` a `kind` has: said of the kind itself, or of a kind above it (the lessons' «is a» first,
    /// then WordNet's), with the way up and the line that said it.
    fn part_count(&self, kind: &str, part: &str, local: &BTreeMap<(String, String), i64>) -> Option<(i64, String)> {
        self.part_count_at(kind, part, local, 0)
    }

    fn part_count_at(&self, kind: &str, part: &str, local: &BTreeMap<(String, String), i64>, depth: usize) -> Option<(i64, String)> {
        // what the base says of one kind, by the way that reached it
        let at = |k: &str, way: &str| -> Option<(i64, String)> {
            let key = (k.to_string(), part.to_string());
            if let Some(&n) = local.get(&key) {
                return Some((n, format!("{way} (said in the question)")));
            }
            if let Some((n, src)) = self.parts.get(&key) {
                return Some((*n, format!("{way}: {}", src_plain(*src))));
            }
            let (n, unit, src) = self.per.get(&key).filter(|_| depth < 2)?;
            // the other part's count from the kind asked about, not from this one
            let (u, uway) = self.part_count_at(kind, unit, local, depth + 1)?;
            Some((n * u, format!("{n} on each {unit}: {way}: {}; {u} {unit}: {uway}", src_plain(*src))))
        };
        // the lessons' «is a» first
        let mut path: Vec<String> = vec![kind.to_string()];
        while let Some((u, _)) = self.isa.get(path.last().unwrap()) {
            if path.contains(u) || path.len() > 12 {
                break;
            }
            path.push(u.clone());
        }
        for (i, k) in path.iter().enumerate() {
            if let Some(x) = at(k, &path[..=i].join(" → ")) {
                return Some(x);
            }
        }
        // then WordNet's kinds, by meaning, each sense of each kind on the path in its order, the nearer kinds first
        // («cat» is first a CAT scan in WordNet, then the animal: the sense whose kinds the base knows is the one)
        for (i, k) in path.iter().enumerate() {
            for up in crate::wordnet::senses_up(k, "noun") {
                for (j, u) in up.iter().enumerate() {
                    if let Some(x) = at(u, &format!("{} → {}", path[..=i].join(" → "), up[..=j].join(" → "))) {
                        return Some(x);
                    }
                }
            }
        }
        None
    }

    /// A counting question worked out from the kind base (PRAG_OFF=student-parts): «3 lions and 2 tigers, how many
    /// legs?» → 3 × 4 + 2 × 4. Every counted group must be found in the base, or there is no count. A part the
    /// question itself gives («a robot dog has 4 legs. How many legs do 3 robot dogs have?») counts first.
    fn count(&self, r: &crate::frames::Read, counted: &Ent) -> Option<Count> {
        // «how many front legs», «how many closed eyes»: some of the parts, not counted yet («long ears» are all of them)
        let some_of = |a: &str| matches!(a, "front" | "back" | "hind" | "rear" | "middle" | "outer" | "inner" | "left" | "right" | "top" | "bottom" | "upper" | "lower" | "closed" | "open" | "shut" | "extra" | "spare" | "missing" | "broken" | "first" | "last" | "other");
        if prag::frames::off("student-parts") || !counted.and.is_empty() || !counted.conj.is_empty() || matches!(counted.id.as_str(), "many" | "much") || counted.attrs.iter().any(|a| some_of(a)) {
            return None;
        }
        let part = counted.id.clone();
        let mut local: BTreeMap<(String, String), i64> = BTreeMap::new();
        let mut said: BTreeSet<*const Ent> = BTreeSet::new();
        for f in &r.frames {
            if f.verb == "have" && !f.neg {
                if let (Some(a), Some(p)) = (&f.agent, &f.patient) {
                    if let (None, Some(v)) = (crate::numbers::quantity(&a.text), crate::numbers::quantity(&p.text)) {
                        if !std::ptr::eq(p, counted) && p.id == part {
                            local.insert((a.id.clone(), p.id.clone()), v);
                            said.insert(a as *const Ent);
                            said.insert(p as *const Ent);
                        }
                    }
                }
            }
        }
        let mut groups: Vec<(String, i64)> = Vec::new();
        for f in &r.frames {
            for (_, e) in f.ents() {
                for x in std::iter::once(e).chain(&e.conj) {
                    if said.contains(&(x as *const Ent)) || x.id == part || x.text == counted.text {
                        continue;
                    }
                    // a year or a number standing alone is not a group of things
                    if crate::numbers::quantity(&x.id).is_some() {
                        continue;
                    }
                    let n = crate::numbers::quantity(&x.text).filter(|&n| n > 0);
                    // «2 pairs of storks», «a pair of storks»: twice as many of what the pair is of
                    if x.id == "pair" {
                        let n = n.or_else(|| x.det.as_ref().filter(|d| matches!(d.0.to_lowercase().as_str(), "a" | "one")).map(|_| 1));
                        if let (Some(n), Some((_, of))) = (n, x.mods.iter().find(|(c, _)| c == "of")) {
                            groups.push((of.id.clone(), 2 * n));
                        }
                        continue;
                    }
                    if let Some(n) = n {
                        groups.push((x.id.clone(), n));
                        // «two goldfish and a cat»: one of what is joined to a counted group without a number
                        for y in &x.conj {
                            let one = y.det.as_ref().is_some_and(|d| matches!(d.0.to_lowercase().as_str(), "a" | "an"));
                            if one && !y.plural && y.pron.is_none() && !y.named && crate::numbers::quantity(&y.text).is_none() {
                                groups.push((y.id.clone(), 1));
                            }
                        }
                    }
                }
            }
        }
        // «3 cats … no, 4 cats»: a kind counted again is corrected, the later number stands
        let mut fixed: Vec<(String, i64)> = Vec::new();
        for (k, n) in groups {
            fixed.retain(|x| x.0 != k);
            fixed.push((k, n));
        }
        let mut groups = fixed;
        if std::env::var_os("MOVA_COUNT_DEBUG").is_some() && groups.is_empty() {
            eprintln!("count\tnogroups\t{part}\t{}", counted.text);
        }
        if groups.is_empty() {
            // «A real chimp has how many tails?»: the one thing that has them
            let f = r.frames.iter().find(|f| f.verb == "have" && f.patient.as_ref().is_some_and(|p| p.text == counted.text))?;
            let a = f.agent.as_ref().filter(|a| a.pron.is_none() && !a.plural && !a.named && !generic(a))?;
            groups.push((a.id.clone(), 1));
        }
        let mut value = 0i64;
        let mut how: Vec<String> = Vec::new();
        for (k, n) in &groups {
            let Some((c, way)) = self.part_count(k, &part, &local) else {
                if std::env::var_os("MOVA_COUNT_DEBUG").is_some() {
                    eprintln!("count\tmissing\t{k}\t{part}\t{groups:?}");
                }
                return None;
            };
            value += n * c;
            how.push(format!("{n} {k} × {c} {part} ({way})"));
        }
        Some(Count { value, how: format!("{} = {value}", how.join(" + ")) })
    }

    /// Everything read so far forgotten, the weights learned from the exercises kept (the forgetting diagnostic).
    pub fn forget(&mut self) {
        let w = std::mem::take(&mut self.w);
        let trees = std::mem::take(&mut self.trees);
        *self = PragStudent { w, trees, ..PragStudent::default() };
        self.read_kinds();
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
        // «the serpent spoke to the woman»: with no recipient, the oblique of «to» is the one spoken or given to
        // (PRAG_OFF=student-to)
        if core[2].is_none() && !prag::frames::off("student-to") {
            if let Some(i) = f.obls.iter().position(|(c, _)| c == "to") {
                core[2] = Some(k + i as u8);
            }
        }
        let mut attrs: Vec<u32> = f.attrs.iter().chain(f.ents().iter().flat_map(|(_, e)| e.attrs.iter())).map(|a| nm.id(a)).collect();
        attrs.sort();
        attrs.dedup();
        let qtys: Vec<(u32, i64)> = f.ents().iter().filter_map(|(_, e)| {
            let v = crate::numbers::quantity(&e.text)?;
            Some((if crate::numbers::quantity(&e.id) == Some(v) { 0 } else { nm.id(&e.id) }, v))
        }).collect();
        MCode { verb: nm.id(&f.verb), light: LIGHT.contains(&f.verb.as_str()), neg: f.neg, irr: f.irrealis, core, ents, attrs, qtys }
    }

    /// The kinds of an entity the lessons never named, as the lessons name them: its WordNet hypernyms, up to three
    /// levels («husky» → dog, «calf» → young mammal → animal), kept when some remembered frame has them. A named
    /// entity is matched as itself only (PRAG_OFF=student-kinds).
    fn kinds(&self, id: &str) -> Vec<String> {
        if prag::frames::off("student-kinds") || self.by_ent.contains_key(id) || self.same.contains_key(id) || matches!(id, "it" | "they" | "he" | "she" | "you" | "we" | "i" | "one" | "people") {
            return Vec::new();
        }
        let mut out = Vec::new();
        // the same thing by another word the lessons use («doc» → doctor; PRAG_OFF=student-synonyms)
        if !prag::frames::off("student-synonyms") {
            out.extend(crate::wordnet::synonyms_of(id, "noun").into_iter().filter(|x| x != id && self.by_ent.contains_key(x)));
        }
        let mut w = id.to_string();
        for _ in 0..3 {
            let Some(k) = crate::wordnet::kind_of(&w, "noun").map(|k| k.to_lowercase()) else { break };
            if self.by_ent.contains_key(&k) {
                out.push(k.clone());
            }
            w = k;
        }
        // what the loaded dictionary says the question's thing is («mate» → friend, «telly» → set), and that kind's
        // WordNet kinds, kept when some remembered frame has them. Off: PRAG_VN=student-dict. Measured 2026-11-04
        // (docs/training.md section 32): the ten courses +14 with the controls flat, the twelve other primary courses
        // +20 with the cross-lesson control +42; on the option side too, +11 and +2 with the controls +40; nouns
        // WordNet lacks: no change. Not kept on by the controls rule.
        if prag::frames::vn_opt("student-dict") && self.q_side.get() {
            for k in self.dict_kinds(id) {
                let mut w = k;
                for _ in 0..3 {
                    if self.by_ent.contains_key(&w) {
                        out.push(w.clone());
                    }
                    let Some(up) = crate::wordnet::kind_of(&w, "noun").map(|k| k.to_lowercase()) else { break };
                    w = up;
                }
            }
        }
        // the same thing by another word the lessons use, as a dictionary with nyms says it (the Wiktionary module:
        // «telly» → tube, TV; «brekkie», a diminutive of breakfast): its noun entries' synonyms and form-of lemmas,
        // kept when some remembered frame has them; `student-wikt-lack` only for nouns WordNet lacks,
        // `student-wikt-mutual` only a word whose own entries name this one back (as `verb_strict` does for verbs).
        // Off by default (PRAG_VN=student-wikt, student-wikt-lack or student-wikt-mutual); measured 2026-11-04
        // (docs/training.md, «Building the dictionary»): no variant passed the controls.
        let lack = prag::frames::vn_opt("student-wikt-lack");
        let mutual = prag::frames::vn_opt("student-wikt-mutual");
        if (prag::frames::vn_opt("student-wikt") || lack || mutual) && self.q_side.get() && (!lack || crate::wordnet::kind_of(id, "noun").is_none()) {
            for w in dict_nyms(id) {
                if self.by_ent.contains_key(&w) && !out.contains(&w) && (!mutual || dict_nyms(&w).iter().any(|x| x == id)) {
                    out.push(w);
                }
            }
        }
        out
    }

    /// The kinds the loaded dictionary (`crate::dictstore`) names a noun as, read by the student's own reader from the
    /// first sentence of its first meaning: the predicate noun of «X is a Y» (its «for» or «of» phrase after «word»,
    /// «name», «form», «kind», «type» or «sort»: «Doc is a short word for a doctor» → doctor), its coordinated nouns
    /// («Togs are swimming costumes, swimmers»), the object of «X means Y», or the head of a bare definition («A
    /// friend»). Cached for the process: the store and the reader are fixed while it runs.
    fn dict_kinds(&self, id: &str) -> Vec<String> {
        static CACHE: std::sync::OnceLock<std::sync::Mutex<BTreeMap<String, Vec<String>>>> = std::sync::OnceLock::new();
        let c = CACHE.get_or_init(|| std::sync::Mutex::new(BTreeMap::new()));
        if let Some(v) = c.lock().unwrap().get(id) {
            return v.clone();
        }
        let store = crate::dictstore::store();
        let noun = store.get(id).iter().any(|e| e.pos.starts_with("noun"));
        let v = match store.first_meaning(id).filter(|_| noun) {
            Some((def, _)) => self.genus_of(id, &def),
            None => Vec::new(),
        };
        c.lock().unwrap().insert(id.to_string(), v.clone());
        v
    }

    /// The genus nouns of a definition sentence of `id` (see `dict_kinds`): the head noun only (a coordinated or
    /// modifying noun is as often another thing: «a thick red sauce made from tomatoes, vinegar, sugar»).
    fn genus_of(&self, id: &str, def: &str) -> Vec<String> {
        let r = self.read_text(def, &mut Ctx::default(), false);
        let wordlike = |x: &Ent| matches!(x.id.as_str(), "word" | "name" | "form" | "kind" | "type" | "sort" | "way");
        let ok = |h: &Ent| h.pron.is_none() && !h.named && h.id != id && !h.id.is_empty() && !matches!(h.id.as_str(), "us" | "it" | "they" | "one" | "thing" | "something" | "someone" | "person" | "people") && h.id.chars().all(|ch| ch.is_alphabetic() || ch == ' ' || ch == '-');
        for f in r.frames.iter().filter(|f| !f.neg && !f.irrealis) {
            let head: Option<&Ent> = match f.kind {
                Kind::Fact(crate::frames::Link::IsA) if f.agent.as_ref().is_some_and(|a| a.pron.is_none()) => f.patient.as_ref(),
                _ if f.verb == "mean" && f.agent.as_ref().is_some_and(|a| a.pron.is_none()) => f.patient.as_ref(),
                Kind::Bare => f.ents().first().map(|(_, e)| *e),
                _ => None,
            };
            let Some(p) = head else { continue };
            // «a short word for a doctor»: the «for» or «of» phrase of the noun, or of the clause
            let h = if wordlike(p) { p.mods.iter().chain(&f.obls).find(|(c, _)| c == "for" || c == "of").map(|m| &m.1) } else { Some(p) };
            return h.filter(|h| ok(h)).map(|h| vec![h.id.clone()]).unwrap_or_default();
        }
        Vec::new()
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
        if !unres {
            ids.extend(self.kinds(&e.id).iter().map(|k| nm.id(k)));
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
        QCode { verb: nm.id(&o.verb), verb_long: o.verb.len() > 1, light: LIGHT.contains(&o.verb.as_str()), neg: o.neg, irr: o.irrealis, bare: o.kind == Kind::Bare, core, ents, obls, attrs }
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
                // a wish against a happening disagrees as a negation does (rudinger-2018 I2; PRAG_OFF=student-irrealis)
                if q.neg != m.neg || q.irr != m.irr && !prag::frames::off("student-irrealis") {
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

    /// Exact scalar reduction of align; MOVA_STRENGTH=mask enables the experiment.
    /// The Vec path remains default: course-level speedup was not established.
    fn align_strength(&self, o: &Frame, q: &QCode, mi: usize) -> i64 {
        if timing_on() {
            STRENGTH_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        if !strength_masked() || align_old() {
            strength(&self.align(o, q, mi))
        } else {
            strength_mask(self.align_mask(q, o, mi))
        }
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
                if o.neg != m.neg || o.irrealis != m.irrealis && !prag::frames::off("student-irrealis") {
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
                let mb = m.slot(s).or_else(|| (s == Slot::Recipient && !prag::frames::off("student-to")).then(|| m.obls.iter().find(|(c, _)| c == "to").map(|x| &x.1)).flatten());
                if let (Some(a), Some(b)) = (o.slot(s), mb) {
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

    /// Whether option frame `c` repeats the quoted frame `m`, its words in any role (a quote reads differently alone
    /// and inside a line: «Where is the lamb?» → `lamb be attrs=where` alone, `_ be patient=lamb` after «Isaac
    /// asked»): two of its entities, or one and the same content verb (the generic «you», «we» and the pronouns
    /// left out).
    fn repeats(&self, c: &Frame, m: &Frame) -> bool {
        let content = |e: &&Ent| !generic(e) && !matches!(e.id.as_str(), "it" | "they" | "he" | "she" | "i");
        let me: Vec<&Ent> = m.ents().into_iter().map(|x| x.1).filter(content).collect();
        let shared = c.ents().into_iter().map(|x| x.1).filter(content).filter(|a| me.iter().any(|b| self.ent_same(a, b))).count();
        let verb = !c.verb.is_empty() && !LIGHT.contains(&c.verb.as_str()) && verb_same(&c.verb, &m.verb);
        shared + verb as usize >= 2
    }

    /// The content of frame `o` that remembered frame `m` does not hold (all of it when there is no `m`): its content
    /// verb, its entities (not pronouns, not the generic «you») and its properties.
    fn unsupported(&self, o: &Frame, m: Option<&Frame>) -> usize {
        let mut n = 0;
        if !o.verb.is_empty() && !LIGHT.contains(&o.verb.as_str()) && !m.is_some_and(|m| verb_same(&o.verb, &m.verb)) {
            n += 1;
        }
        let m_ents = m.map(|m| m.ents()).unwrap_or_default();
        for (_, e) in o.ents() {
            if prag::frames::UNRESOLVED_PUB.contains(&e.id.as_str()) || generic(e) {
                continue;
            }
            if !m_ents.iter().any(|(_, x)| self.ent_same(e, x)) {
                n += 1;
            }
        }
        let m_attrs: BTreeSet<&String> = m.map(|m| m.attrs.iter().chain(m_ents.iter().flat_map(|(_, e)| e.attrs.iter())).collect()).unwrap_or_default();
        n += o.attrs.iter().filter(|a| !m_attrs.contains(a)).count();
        n
    }

    /// The remembered frames that hold any of these entities or this verb (the most recent `cand_cap()` of a long
    /// list).
    fn candidates(&self, f: &Frame, into: &mut BTreeSet<usize>) {
        let _t = Tick::new(&CAND_NS);
        let mut add = |v: Option<&Vec<usize>>| {
            if let Some(v) = v {
                into.extend(v.iter().rev().take(cand_cap()).copied());
                // the cost model (MOVA_TIMING): what this key would add under each cap, Σ min(cap, n)
                if timing_on() {
                    for (i, &c) in COST_CAPS.iter().enumerate() {
                        COST[i].fetch_add(v.len().min(c) as u64, std::sync::atomic::Ordering::Relaxed);
                    }
                }
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
                for k in self.kinds(id) {
                    add(self.by_ent.get(&k));
                }
            }
        }
        if !f.verb.is_empty() {
            add(self.by_verb.get(&f.verb));
        }
    }

    /// The rules compiled so far from the remembered lines (`prag::frames::implications` over each line's frames, as
    /// the line's reading gave them), compiling the lines remembered since the last call.
    fn rule_base(&self) -> std::cell::Ref<'_, RuleBase> {
        if self.rulebase.borrow().upto < self.frames.len() {
            let mut rb = self.rulebase.borrow_mut();
            let mut b = rb.upto;
            while b < self.frames.len() {
                let src = self.frames[b].src;
                let mut e = b + 1;
                while e < self.frames.len() && self.frames[e].src == src {
                    e += 1;
                }
                // the line's frames with their links relative to the line again (`link` already is)
                let local: Vec<Frame> = self.frames[b..e].iter().map(|f| {
                    let mut g = f.clone();
                    g.quote = f.quote.iter().filter(|&&q| q >= b && q < e).map(|q| q - b).collect();
                    g.cause = f.cause.filter(|&c| c >= b && c < e).map(|c| c - b);
                    g.next = f.next.filter(|&c| c >= b && c < e).map(|c| c - b);
                    g
                }).collect();
                for imp in prag::frames::implications(&local) {
                    let r = Rule { src, conn: imp.conn, cond: imp.cond.iter().map(|i| i + b).collect(), concl: imp.concl.iter().map(|i| i + b).collect(), pro: imp.pro.iter().map(|i| i + b).collect() };
                    let ri = rb.rules.len();
                    let mut keys: BTreeSet<String> = BTreeSet::new();
                    let mut var = false;
                    for &p in &r.cond {
                        let v = &self.frames[p].verb;
                        if verb_var(v) {
                            var = true;
                        } else {
                            keys.insert(v.clone());
                            if !LIGHT.contains(&v.as_str()) {
                                keys.extend(verb_synonyms(v));
                            }
                        }
                    }
                    for k in keys {
                        rb.by_verb.entry(k).or_default().push(ri);
                    }
                    if var {
                        rb.var_verb.push(ri);
                    }
                    rb.rules.push(r);
                }
                b = e;
            }
            rb.upto = self.frames.len();
        }
        self.rulebase.borrow()
    }

    /// The compiled rules, by what joins condition and conclusion, and the lines that state one (for the course's
    /// diagnostic, MOVA_RULEDIAG=1).
    pub fn rule_stats(&self) -> String {
        let rb = self.rule_base();
        let mut by: BTreeMap<&str, usize> = BTreeMap::new();
        for r in &rb.rules {
            *by.entry(r.conn.as_str()).or_default() += 1;
        }
        let lines: BTreeSet<(usize, usize)> = rb.rules.iter().map(|r| r.src).collect();
        let pro = rb.rules.iter().filter(|r| !r.pro.is_empty()).count();
        format!("{} rules from {} of {} lines ({}); {} with a placeholder verb, {} whose conclusion does the condition's action; {} kinds",
            rb.rules.len(), lines.len(), self.texts.len(), by.iter().map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", "), rb.var_verb.len(), pro, self.isa.len())
    }

    /// Whether `x` is a kind of `k`: the lessons' «is a» chain, or WordNet's kinds of a word the lessons never named.
    fn is_kind(&self, x: &str, k: &str) -> bool {
        let mut w = x;
        for _ in 0..8 {
            match self.isa.get(w) {
                Some((u, _)) if u == k => return true,
                Some((u, _)) => w = u.as_str(),
                None => break,
            }
        }
        self.kinds(x).iter().any(|y| y == k)
    }

    /// The same entity, or one a kind of the other.
    fn ent_fits(&self, a: &Ent, b: &Ent) -> bool {
        self.ent_same(a, b) || self.is_kind(&b.id, &a.id) || self.is_kind(&a.id, &b.id)
    }

    /// The binding a question frame `f` gives a rule's condition frame `p`, if it is an instance. A quoted pattern
    /// («"Can you…?"», «"Don't forget X"») is an utterance and matches only what the question quotes; a «can» pattern
    /// only an asked «can» («can u pass the ketchup», not «you can use the steps»). The verb the same (or a
    /// synonym both ways; or the pattern's verb a placeholder), the same polarity, every named entity of the pattern
    /// found in the question's frame, the placeholders bound to the question's entities in their roles. The match must
    /// carry content of the pattern itself, never a binding alone: an utterance its content verb or one of its named
    /// entities, or an asked «can» with a placeholder action; a situation («if you see smoke») its content verb and one
    /// of its named entities.
    fn match_pattern(&self, p: &Frame, f: &Frame, asked: bool) -> Option<Theta> {
        if f.kind == Kind::Bare || f.verb.is_empty() || p.kind == Kind::Bare || p.quoted && !f.quoted {
            return None;
        }
        let var_verb = verb_var(&p.verb);
        if !var_verb && !verb_strict(&p.verb, &f.verb) || p.neg != f.neg {
            return None;
        }
        let modal = p.kind == Kind::Fact(crate::frames::Link::Can);
        if modal && (f.kind != p.kind || !asked) {
            return None;
        }
        let mut th = Theta { action: var_verb, quoted: p.quoted, inst: Some(f.clone()), ..Default::default() };
        let verb_content = !var_verb && !LIGHT.contains(&p.verb.as_str());
        let mut named = 0;
        for s in CORE4 {
            let Some(pe) = p.slot(s) else { continue };
            if pattern_var(pe) {
                let Some(fe) = f.slot(s) else { continue };
                if prag::frames::UNRESOLVED_PUB.contains(&fe.id.as_str()) {
                    continue;
                }
                match th.ents.get(&pe.id) {
                    Some(x) if !self.ent_same(x, fe) => return None,
                    Some(_) => {}
                    None => {
                        th.ents.insert(pe.id.clone(), fe.clone());
                    }
                }
            } else if !weak_ent(pe) {
                // a named entity of the pattern must be there, in any role
                if !f.ents().iter().any(|(_, fe)| self.ent_fits(pe, fe)) {
                    return None;
                }
                named += 1;
            }
        }
        let ok = if p.quoted { verb_content || named >= 1 || var_verb && modal } else { verb_content && named >= 1 };
        ok.then_some(th)
    }

    /// A rule's conclusion under a binding: «do it» is the question's action (its polarity turned by the
    /// conclusion's), a placeholder verb the action's verb, a placeholder entity the bound one; the carrier verbs
    /// («want», «mean») left out where a complement follows.
    fn conclusion(&self, rule: &Rule, th: &Theta) -> Vec<Frame> {
        let mut out = Vec::new();
        for (j, &k) in rule.concl.iter().enumerate() {
            let kf = &self.frames[k];
            if rule.pro.contains(&k) {
                // «do it» is the pattern's action only when the pattern is an utterance or its verb a placeholder
                if let Some(a) = th.inst.as_ref().filter(|_| th.action || th.quoted) {
                    let mut a = a.clone();
                    a.neg = a.neg != kf.neg;
                    out.push(a);
                }
                continue;
            }
            if CARRIERS.contains(&kf.verb.as_str()) && j + 1 < rule.concl.len() {
                continue;
            }
            let mut c = kf.clone();
            if verb_var(&c.verb) {
                match th.inst.as_ref().filter(|_| th.action) {
                    Some(a) => c.verb = a.verb.clone(),
                    None => continue,
                }
            }
            for s in CORE4 {
                if let Some(slot) = c.slot_mut(s) {
                    if let Some(b) = slot.as_ref().filter(|e| pattern_var(e)).and_then(|e| th.ents.get(&e.id)) {
                        *slot = Some(b.clone());
                    }
                }
            }
            out.push(c);
        }
        out
    }

    /// Whether option frame `o` says conclusion frame `c`: the same content verb, the same polarity, and no core role
    /// filled with another entity than the conclusion's (a placeholder, a pronoun or the generic «you» says nothing).
    /// Some(the roles that agree).
    fn says_conclusion(&self, c: &Frame, o: &Frame) -> Option<usize> {
        if c.verb.is_empty() || LIGHT.contains(&c.verb.as_str()) || verb_var(&c.verb) || o.verb.is_empty() || c.neg != o.neg || !verb_strict(&c.verb, &o.verb) {
            return None;
        }
        let mut m = 0;
        for s in [Slot::Agent, Slot::Patient, Slot::Recipient] {
            if let (Some(ce), Some(oe)) = (c.slot(s), o.slot(s)) {
                if placeholder(ce) || weak_ent(ce) || weak_ent(oe) {
                    continue;
                }
                if !self.ent_fits(ce, oe) {
                    return None;
                }
                m += 1;
            }
        }
        Some(m)
    }

    /// Apply the compiled rules to a question (its frames `qfs`, the asked one left out) and its options: a rule fires
    /// when one of its condition frames matches a question frame (`match_pattern`: the binding comes from the question
    /// only), and an option fits when one of its frames says the conclusion under that binding (`says_conclusion`).
    /// The choice: the one option some fired rule fits (PRAG_VN=student-rules-tie: else the one that fits with the
    /// most agreeing roles, when unique). The most recent rules are tried first; an option keeps its best fit.
    fn apply_rules(&self, qfs: &[(&Frame, bool)], opts: &[OptionRead]) -> (RuleUse, Option<usize>) {
        let rb = self.rule_base();
        let mut cands: BTreeSet<usize> = rb.var_verb.iter().copied().collect();
        for (f, _) in qfs {
            if f.verb.is_empty() {
                continue;
            }
            cands.extend(rb.by_verb.get(&f.verb).into_iter().flatten().copied());
            if !LIGHT.contains(&f.verb.as_str()) {
                for v in verb_synonyms(&f.verb) {
                    cands.extend(rb.by_verb.get(&v).into_iter().flatten().copied());
                }
            }
        }
        let mut fired = 0;
        let mut notes = Vec::new();
        // per option: (agreeing roles, the rule, the condition frame, the question frame, the conclusion frame, the option frame, the binding)
        let mut fit: Vec<Option<(usize, usize, usize, Option<Frame>, Frame, Frame, String)>> = vec![None; opts.len()];
        // the rules of «because», the reader's cause links (PRAG_OFF=student-rules-because leaves them out)
        let because = !prag::frames::off("student-rules-because");
        for &ri in cands.iter().rev() {
            let rule = &rb.rules[ri];
            if !because && rule.conn == "because" {
                continue;
            }
            let hit = rule.cond.iter().find_map(|&p| qfs.iter().find_map(|(f, asked)| self.match_pattern(&self.frames[p], f, *asked).map(|t| (p, t))));
            let Some((p, th)) = hit else { continue };
            fired += 1;
            let concl = self.conclusion(rule, &th);
            if notes.len() < 3 {
                notes.push(format!("L{}.{} {}: {}", rule.src.0, rule.src.1 + 1, rule.conn, concl.iter().map(|c| c.show(0)).collect::<Vec<_>>().join(" ; ")));
            }
            for (oi, o) in opts.iter().enumerate() {
                for c in &concl {
                    for of in &o.frames {
                        if let Some(m) = self.says_conclusion(c, of) {
                            if fit[oi].as_ref().is_none_or(|x| m > x.0) {
                                let mut bind: Vec<String> = th.ents.iter().map(|(k, v)| format!("{k} := {}", v.show())).collect();
                                if th.action {
                                    if let Some(a) = &th.inst {
                                        bind.push(format!("the action := {}", a.show(0)));
                                    }
                                }
                                fit[oi] = Some((m, ri, p, th.inst.clone(), c.clone(), of.clone(), bind.join(", ")));
                            }
                        }
                    }
                }
            }
        }
        let fitting: Vec<usize> = (0..opts.len()).filter(|&i| fit[i].is_some()).collect();
        let pick = match fitting[..] {
            [i] => Some(i),
            [_, _, ..] if prag::frames::vn_opt("student-rules-tie") => {
                let top = fitting.iter().map(|&i| fit[i].as_ref().map_or(0, |x| x.0)).max().unwrap_or(0);
                let best: Vec<usize> = fitting.iter().copied().filter(|&i| fit[i].as_ref().is_some_and(|x| x.0 == top)).collect();
                (best.len() == 1).then(|| best[0])
            }
            _ => None,
        };
        let how = pick.and_then(|i| fit[i].as_ref().map(|(_, ri, p, inst, c, of, bind)| {
            let r = &rb.rules[*ri];
            let text = self.texts.get(&r.src).cloned().unwrap_or_default();
            format!("It applied a rule: lesson {}, line {} («{}»), joined by «{}». Its condition {} matched the question's {}{}. Its conclusion, so: {}. Option {} says it ({}){}.",
                r.src.0, r.src.1 + 1, corpus_en_short(&text, 160), r.conn, self.frames[*p].show(0), inst.as_ref().map(|f| f.show(0)).unwrap_or_default(),
                if bind.is_empty() { String::new() } else { format!(" ({bind})") }, c.show(0), i + 1, of.show(0),
                if fitting.len() > 1 { format!("; {} options fit, it has the most agreeing roles", fitting.len()) } else { ", the only option that does".into() })
        }));
        (RuleUse { fired, fitting: fitting.len(), how, notes }, pick)
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
        self.answer_with(q, opts, rules_on(), true)
    }

    /// The answer the weights give, the compiled rules and the question's verb gate left out: what the exercises teach
    /// the weights from. A rule's decision is knowledge applied at answer time and owes nothing to the weights, so the
    /// weights learn as if it were not there (with them in, the exercises a rule decides stopped teaching the weights:
    /// ten courses, 2026-11-04, the rows no rule decided +1 / −9). The verb gate only says which supports may answer;
    /// the features of every support are learned as before (the strict gate learned with it in: ten courses −14 against
    /// −11 without; PRAG_VN=student-verb-learn learns with it).
    pub(crate) fn answer_by_weights(&self, q: &str, opts: &[String]) -> Answer {
        self.answer_with(q, opts, false, prag::frames::vn_opt("student-verb-learn"))
    }

    fn answer_with(&self, q: &str, opts: &[String], rules: bool, gate: bool) -> Answer {
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
        let mut ans = Answer { qframes: r.frames.clone(), ask, gap, anchors: Vec::new(), targets: Vec::new(), options: Vec::new(), choice: 0, understood: false, count: None, rule: None };
        // the question's frames, the gap's masked; with a gap only the gap's sentence matters
        let qfs: Vec<Frame> = r.frames.iter().enumerate().filter(|(i, f)| f.kind != Kind::Bare || Some(*i) == gap).map(|(i, f)| if Some(i) == gap { Self::masked(f, ask) } else { f.clone() }).collect();
        let mut cand = BTreeSet::new();
        self.q_side.set(true);
        for f in &qfs {
            self.candidates(f, &mut cand);
        }
        let qcs: Vec<QCode> = qfs.iter().map(|f| self.qcode(f)).collect();
        self.q_side.set(false);
        let mut scored: Vec<(usize, i64)> = cand.iter().map(|&m| (m, qfs.iter().zip(&qcs).map(|(f, q)| self.align_strength(f, q, m)).max().unwrap_or(0))).filter(|x| x.1 >= 2).collect();
        // the lesson just read first: where the question matches it, its anchors are taken from it alone
        // (PRAG_OFF=student-anchor-first)
        let last = self.frames.last().filter(|_| self.lesson_focus).map(|f| f.src.0);
        if !prag::frames::off("student-anchor-first") && scored.iter().any(|x| Some(self.frames[x.0].src.0) == last) {
            scored.retain(|x| Some(self.frames[x.0].src.0) == last);
        }
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
        let mut qents: BTreeSet<String> = r.frames.iter().flat_map(|f| f.ents().into_iter().map(|(_, e)| e.id.clone()).collect::<Vec<_>>()).filter(|x| !matches!(x.as_str(), "it" | "they" | "he" | "she")).collect();
        // the things the question names inside its phrases too: a noun compound the lessons name as a thing («lithium
        // mines» is about lithium), a possessor («Alice's cat»), a prepositional modifier («a cup of tea»). Measured
        // 2026-11-04 on the ten courses: +30 tests (odd +15, even +15), exam +20; the cross-lesson control flat (+2), the
        // same-lesson shuffle +20: a reading of the question's topic, not an option prior (PRAG_OFF=student-qents-mods)
        if !prag::frames::off("student-qents-mods") {
            let pron = |x: &str| matches!(x, "it" | "they" | "he" | "she" | "i" | "you" | "we");
            let mut more: Vec<String> = Vec::new();
            for f in &r.frames {
                for (_, e) in f.ents() {
                    more.extend(e.attrs.iter().filter(|a| self.by_ent.contains_key(a.as_str())).cloned());
                    more.extend(e.poss.iter().map(|p| p.id.clone()).filter(|x| !pron(x)));
                    more.extend(e.mods.iter().map(|(_, m)| m.id.clone()).filter(|x| !pron(x)));
                }
            }
            qents.extend(more);
        }
        // and what the lessons call them («doc» is the doctor of the lessons; PRAG_OFF=student-qents-kinds)
        if !prag::frames::off("student-qents-kinds") {
            self.q_side.set(true);
            let more: Vec<String> = qents.iter().flat_map(|x| self.kinds(x)).collect();
            self.q_side.set(false);
            qents.extend(more);
        }
        let g = gap.map(|i| r.frames[i].clone());
        // the question's own predicate gates the support of the frames that fill its gap: a remembered frame supports
        // them only when its verb states the question's (`verb_agrees`; PRAG_OFF=student-verb)
        let vgate: Option<(VerbGate, String)> = match ask {
            Ask::Role(s) if s != Slot::Verb && gate => self.verb_gate.and_then(|vg| g.as_ref().filter(|g| !verb_light_q(g)).map(|g| (vg, g.verb.clone()))),
            _ => None,
        };
        // a «when» question asks the time (the reader's PRAG_OFF=wh-time); a «which N» question in a core role asks for a
        // name the remembered N carries (PRAG_OFF=wh-which): the asked role and N
        let when_q = ask == Ask::Role(Slot::Attr) && r.asked.first().is_some_and(|a| a.word == "when");
        let which_n: Option<(Slot, Ent)> = match (ask, &g) {
            (Ask::Role(s @ (Slot::Agent | Slot::Patient | Slot::Recipient | Slot::Place)), Some(g)) if !prag::frames::off("wh-which") && r.asked.first().is_some_and(|a| matches!(a.word.as_str(), "which" | "what")) => {
                g.slot(s).filter(|e| !matches!(e.id.as_str(), "which" | "what" | "who" | "whom") && !prag::frames::UNRESOLVED_PUB.contains(&e.id.as_str())).map(|e| (s, e.clone()))
            }
            _ => None,
        };
        // «how many / how much»: the asked thing is the entity the wh-word counts («how many righteous people»), when
        // the question names one (PRAG_OFF=student-qty)
        let ql = q.to_lowercase();
        // a question that gives numbers itself («3 sloths … how many legs») asks for a sum, not for a remembered number
        // and so does a scenario told before it («A team scores a touchdown and then the extra point. How many points
        // is that?»)
        let hm = ql.find("how many").or_else(|| ql.find("how much"));
        let how_many = !prag::frames::off("student-qty") && hm.is_some() && crate::numbers::quantity(q).is_none()
            && !ql[..hm.unwrap_or(0)].contains(['.', '!', '?', ':', ';']);
        let counted: Option<Ent> = if how_many { r.frames.iter().flat_map(|f| f.ents().into_iter().map(|(_, e)| e.clone()).collect::<Vec<_>>()).find(|e| e.text.split_whitespace().any(|w| matches!(w.to_lowercase().as_str(), "many" | "much"))) } else { None };
        let counted_q: Option<QEnt> = counted.as_ref().map(|e| self.qent(e, &mut self.names.borrow_mut()));
        // a remembered frame holds the number `v` as the quantity of the counted thing (or of anything, when the
        // question names nothing)
        let holds = |m: usize, v: i64| -> bool {
            let c = &self.codes[m];
            c.qtys.iter().any(|&(n, x)| x == v && match &counted_q {
                Some(e) => if n == 0 { c.ents.iter().any(|y| e.same(y)) } else { e.ids.binary_search(&n).is_ok() },
                None => true,
            })
        };
        // the remembered frames about the counted thing that hold a quantity, the most recent first
        let counted_frames: Vec<usize> = match &counted {
            Some(e) => {
                let mut c: BTreeSet<usize> = BTreeSet::new();
                for id in std::iter::once(&e.id).chain(self.same.get(&e.id).into_iter().flatten()) {
                    c.extend(self.by_ent.get(id).into_iter().flatten().copied());
                }
                c.into_iter().rev().filter(|&m| !self.codes[m].qtys.is_empty()).collect()
            }
            None => Vec::new(),
        };
        let mut recent_witnesses = Vec::with_capacity(opts.len());
        for o in opts {
            let mut recent_witness = None;
            let ov = if how_many { crate::numbers::quantity(o) } else { None };
            let mut c2 = ctx.clone();
            let or = self.read_memo(q, o, &mut c2, false);
            let mut ord = OptionRead { text: o.clone(), frames: or.frames.clone(), ..Default::default() };
            let mut compared: Vec<Frame> = or.frames.clone();
            let mut filled = false;
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
                    // «when»: the option is a time and fills the time the reader keeps as «when:…» (its value, or its
                    // head: «at night» → when:night), nothing else (PRAG_OFF=wh-time)
                    Slot::Attr if when_q => {
                        c.attrs = b.attrs.iter().filter(|a| a.starts_with("when:")).cloned().collect();
                        if c.attrs.is_empty() {
                            c.attrs.extend(first.map(|e| format!("when:{}", e.id)));
                        }
                    }
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
                filled = true;
            } else if let (Some(g), false) = (&g, r.asked.is_empty()) {
                for c in compared.iter_mut() {
                    if c.agent.is_none() && c.kind != Kind::Bare {
                        c.agent = g.agent.clone();
                    }
                }
            }
            // each compared frame's own best support (its score, the remembered frame), for its unsupported content
            let mut per: Vec<Option<(i64, usize)>> = vec![None; compared.len()];
            for (k, c) in compared.iter().enumerate() {
                let mut tg: Vec<(usize, Via)> = ans.targets.clone();
                // the option's own support: remembered frames about the question's entities that match it strongly
                let mut cand = BTreeSet::new();
                self.candidates(c, &mut cand);
                let cq = self.qcode(c);
                // about them: an entity of the frame, or one joined to it («a torch rather than candles» is about
                // candles too; PRAG_OFF=student-own-and)
                let own_and = !prag::frames::off("student-own-and");
                // a question with nothing of its own to be about («Which is true?», «What is right?»): the options' own
                // support may be about anything (the lesson just read still comes first; PRAG_OFF=student-open-claim)
                let open = qents.is_empty() && !prag::frames::off("student-open-claim");
                let mut own: Vec<(usize, i64)> = cand.into_iter().filter(|&m| open || self.frames[m].ents().iter().any(|(_, e)| qents.contains(&e.id) || own_and && e.and.iter().any(|x| qents.contains(x)))).map(|m| (m, self.align_strength(c, &cq, m))).filter(|x| x.1 >= 3).collect();
                own.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
                tg.extend(own.into_iter().take(10).map(|x| (x.0, Via::Own)));
                if let Some(v) = ov {
                    tg.extend(counted_frames.iter().filter(|&&m| holds(m, v)).take(10).map(|&m| (m, Via::Own)));
                }
                let gated = vgate.as_ref().filter(|(vg, _)| filled || vg.all);
                for (m, via) in tg {
                    let mf = &self.frames[m];
                    let gated_out = gated.is_some_and(|(vg, qv)| {
                        let light = mf.verb.is_empty() || LIGHT.contains(&mf.verb.as_str());
                        via != Via::Quote && !(light && !vg.strict) && !verb_agrees(qv, &mf.verb)
                    });
                    let mut fs = self.align(c, &cq, m);
                    // what was said is the patient of «ask» and «say»: an option that repeats the quote fills it
                    if via == Via::Quote && ask == Ask::Role(Slot::Patient) && !prag::frames::off("student-quote") && self.repeats(c, mf) {
                        fs.push(Feat::Match(Slot::Patient));
                        fs.sort();
                        fs.dedup();
                    }
                    // «which line stops at …?» with «Lansdale/Doylestown»: the remembered line is «the Lansdale/Doylestown
                    // Line»; an option that is one of the names of the remembered N in the asked role fills it
                    // (`Ent::names`; PRAG_OFF=wh-which)
                    if let (Some((s, n)), true) = (&which_n, c.kind != Kind::Bare) {
                        let names = |me: &Ent| me.names().iter().map(|x| x.to_lowercase()).collect::<BTreeSet<String>>();
                        if let (Some(oe), Some(me)) = (c.slot(*s), mf.slot(*s)) {
                            if (self.ent_same(n, me) || self.kinds(&me.id).contains(&n.id)) && !self.ent_same(oe, me) && names(me).contains(&oe.id.to_lowercase()) {
                                fs.retain(|f| *f != Feat::Conflict(*s));
                                fs.push(Feat::Match(*s));
                                fs.sort();
                                fs.dedup();
                            }
                        }
                    }
                    if let Ask::Role(s) = ask {
                        if fs.contains(&Feat::Match(s)) {
                            fs.push(Feat::Asked);
                        }
                    }
                    // a frame fills another only with structure: two roles (a content verb and an entity, or two
                    // entities), or the asked role, or one role on the way of the cause / the next
                    if ov.is_some_and(|v| holds(m, v)) {
                        fs.push(Feat::Quantity);
                    }
                    let st = strength(&fs);
                    if !(st >= 3 || st >= 2 && (fs.contains(&Feat::Asked) || matches!(via, Via::Cause | Via::Next)) || fs.contains(&Feat::Quantity)) {
                        continue;
                    }
                    if let (Some(oc), Some(mc)) = (c.cause, mf.cause) {
                        if let Some(ocf) = compared.get(oc) {
                            if self.align_strength(ocf, &self.qcode(ocf), mc) >= 2 {
                                fs.push(Feat::CauseLink);
                            }
                        }
                    }
                    let s = counter(fs.iter().map(|&f| self.w[widx(ask, via, f)]).sum(), &fs, c.neg, mf.neg);
                    if gated_out {
                        if ord.assoc.is_none_or(|a| s > a) {
                            ord.assoc = Some(s);
                        }
                        continue;
                    }
                    if per[k].is_none_or(|b| s > b.0) {
                        per[k] = Some((s, m));
                    }
                    if Some(self.frames[m].src.0) == last && ord.recent.is_none_or(|r| s > r) {
                        ord.recent = Some(s);
                        recent_witness = Some((s, k, m, via, fs.clone()));
                    }
                    if ord.best.as_ref().is_none_or(|b| s > b.0) {
                        ord.best = Some((s, k, m, via, fs));
                    }
                }
            }
            ord.sum = per.iter().flatten().map(|x| x.0.max(0)).sum();
            if ord.best.is_some() {
                ord.extra = compared.iter().zip(&per).map(|(c, p)| self.unsupported(c, p.map(|x| &self.frames[x.1]))).sum();
            }
            ord.compared = compared;
            ans.options.push(ord);
            recent_witnesses.push(recent_witness);
        }
        // the best score, the first shown between equal ones; but a tied option that says all another tied option
        // says and more («Paid 400 shekels of silver and then asked the Hittites to witness» against «Paid 400 shekels
        // of silver»), with more unsupported content, loses to it (PRAG_OFF=student-extra). Measured 18.10: preferring
        // the least unsupported content between any tied options loses (biology −53): the courses' right answers are
        // often the fuller ones.
        // between equal scores, the option supported by the lesson just read: a test asks about it (PRAG_OFF=student-recent)
        let recent = |o: &OptionRead| -> bool { !prag::frames::off("student-recent") && o.best.as_ref().is_some_and(|b| Some(self.frames[b.2].src.0) == last) };
        // off by default, on with PRAG_VN=student-tie-sum: measured 2026-10-24 on the 25 courses, 19 439 → 19 347 (odd −49,
        // even −43); an option of more claims is not the right one more often
        let tie_sum = |o: &OptionRead| if prag::frames::vn_opt("student-tie-sum") { o.sum } else { 0 };
        // between options tied on their support, the one the frames the verb gate left out favour (PRAG_OFF=student-verb)
        let assoc = |o: &OptionRead| o.assoc.unwrap_or(i64::MIN);
        let mut best = ans.options.iter().enumerate().filter_map(|(i, o)| o.best.as_ref().map(|b| ((b.0, recent(o), assoc(o), tie_sum(o)), i))).max_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1))).map(|((s, _, _, _), i)| (s, i));
        // the lesson just read first: where some option has support in it, the choice is made on that support alone;
        // earlier lessons decide only where it says nothing (a diagnostic showed the student scoring higher when it
        // forgot every earlier lesson: old frames gave wrong options support; PRAG_OFF=student-lesson-first)
        let lesson_first = current_support(&mut ans.options, recent_witnesses, !prag::frames::off("student-lesson-first"));
        if lesson_first {
            best = ans.options.iter().enumerate().filter_map(|(i, o)| o.recent.map(|r| ((r, assoc(o), tie_sum(o)), i))).max_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1))).map(|((r, _, _), i)| (r, i));
        }
        if let (Some((s, i)), false, false) = (best, prag::frames::off("student-extra"), lesson_first) {
            let words = |t: &str| -> BTreeSet<String> { t.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(|w| w.to_lowercase()).collect() };
            let wi = words(&ans.options[i].text);
            let alt = ans.options.iter().enumerate()
                .filter(|(j, o)| *j != i && o.best.as_ref().is_some_and(|b| b.0 == s) && o.extra < ans.options[i].extra)
                .filter(|(_, o)| {
                    let w = words(&o.text);
                    w.len() < wi.len() && w.is_subset(&wi)
                })
                .map(|(j, o)| (o.extra, j))
                .min();
            if let Some((_, j)) = alt {
                best = Some((s, j));
            }
        }
        if let Some((_, i)) = best {
            ans.choice = i;
            ans.understood = !r.frames.is_empty();
        }
        // a rule the lessons state, its condition matched by the question and its conclusion said by exactly one
        // option, decides (PRAG_OFF=student-rules; PRAG_VN=student-rules-gap: only where nothing else did)
        if rules {
            let skip = gap.filter(|_| !r.asked.is_empty());
            let qfs: Vec<(&Frame, bool)> = r.frames.iter().enumerate().filter(|(i, f)| Some(*i) != skip && f.kind != Kind::Bare).map(|(i, f)| (f, prag::frames::interrogative(&r, i))).collect();
            let (mut u, pick) = self.apply_rules(&qfs, &ans.options);
            match pick {
                Some(i) if !(prag::frames::vn_opt("student-rules-gap") && ans.understood) => {
                    ans.choice = i;
                    ans.understood = true;
                }
                _ => u.how = None,
            }
            ans.rule = Some(u);
        }
        // a counting question the kind base works out decides, when exactly one option holds its number
        if hm.is_some() {
            let asked = r.frames.iter().flat_map(|f| f.ents().into_iter().map(|(_, e)| e)).find(|e| e.text.split_whitespace().any(|w| matches!(w.to_lowercase().as_str(), "many" | "much")));
            // the kind base first; else a word problem the question tells itself («100 babies on its back, about 5 fell
            // off, how many left»), read by the tree reader of the maths student (PRAG_OFF=student-arith)
            let c = asked.and_then(|e| self.count(&r, e)).or_else(|| {
                if prag::frames::off("student-arith") || crate::numbers::quantity(q).is_none() {
                    return None;
                }
                crate::treeread::answer(q).map(|(value, how)| Count { value, how: format!("the question's own numbers, {how}") })
            });
            if let Some(c) = c {
                let hit: Vec<usize> = opts.iter().enumerate().filter(|(_, o)| crate::numbers::quantity(o) == Some(c.value)).map(|(i, _)| i).collect();
                if let [i] = hit[..] {
                    ans.choice = i;
                    ans.understood = true;
                    ans.count = Some(c);
                    if let Some(u) = ans.rule.as_mut() {
                        u.how = None;
                    }
                }
            }
        }
        ans
    }

    /// A diagnostic of a question not understood (MOVA_GAPDIAG=1, stderr; nothing decided on it): for each frame of
    /// the right option, why no remembered frame supported it. `light-or-noverb`: no content verb; `verb-unknown`: no
    /// remembered frame has the verb (or a synonym); `verb-known-noent`: frames with the verb, none sharing an entity;
    /// else the remembered frame with the verb and a shared entity that aligns best, and why it gave no support:
    /// `reached-gated` (a target, but the strength gate failed), `about-capped` (about the question, beyond the
    /// candidate cap), `about-weak` (about the question, strength below 3: the entity in another role),
    /// `about-other`, `not-about` (it mentions none of the question's entities; `/noanchor` when the question
    /// matched nothing remembered).
    pub fn gap_diag(&self, q: &str, a: &Answer, right: usize) -> Vec<String> {
        let mut out = Vec::new();
        for (oi, o) in a.options.iter().enumerate() {
            out.extend(self.gap_diag_option(q, a, o, oi == right));
        }
        out
    }

    fn gap_diag_option(&self, q: &str, a: &Answer, o: &OptionRead, is_right: bool) -> Vec<String> {
        let mut out = Vec::new();
        let mut qents: BTreeSet<String> = a.qframes.iter().flat_map(|f| f.ents().into_iter().map(|(_, e)| e.id.clone()).collect::<Vec<_>>()).filter(|x| !matches!(x.as_str(), "it" | "they" | "he" | "she")).collect();
        let more: Vec<String> = qents.iter().flat_map(|x| self.kinds(x)).collect();
        qents.extend(more);
        let targets: BTreeSet<usize> = a.targets.iter().map(|x| x.0).collect();
        let content = |e: &Ent| !generic(e) && !prag::frames::UNRESOLVED_PUB.contains(&e.id.as_str()) && !matches!(e.id.as_str(), "it" | "they" | "he" | "she");
        for c in &o.compared {
            let cat = if c.verb.is_empty() || LIGHT.contains(&c.verb.as_str()) {
                ("light-or-noverb".to_string(), None, 0)
            } else {
                let mut verbs: BTreeSet<String> = verb_synonyms(&c.verb);
                verbs.insert(c.verb.clone());
                let ms: BTreeSet<usize> = verbs.iter().flat_map(|v| self.by_verb.get(v).into_iter().flatten().copied()).filter(|&m| verb_same(&c.verb, &self.frames[m].verb)).collect();
                if ms.is_empty() {
                    ("verb-unknown".to_string(), None, 0)
                } else {
                    let cq = self.qcode(c);
                    let shared = |m: usize| c.ents().iter().any(|(_, e)| content(e) && self.frames[m].ents().iter().any(|(_, x)| self.ent_same(e, x)));
                    let best = ms.iter().copied().filter(|&m| shared(m)).map(|m| (self.align_strength(c, &cq, m), m)).max();
                    match best {
                        None => ("verb-known-noent".to_string(), None, 0),
                        Some((st, m)) => {
                            let about = self.frames[m].ents().iter().any(|(_, e)| qents.contains(&e.id) || e.and.iter().any(|x| qents.contains(x)));
                            let cat = if targets.contains(&m) {
                                "reached-gated".to_string()
                            } else if about {
                                let mut cand = BTreeSet::new();
                                self.candidates(c, &mut cand);
                                if !cand.contains(&m) { "about-capped".into() } else if st < 3 { "about-weak".into() } else { "about-other".into() }
                            } else {
                                // the line's other frames about the question; else a question word in the line's text
                                let line = self.by_line.get(&self.frames[m].src).cloned().unwrap_or_default();
                                let on_line = line.iter().any(|&x| self.frames[x].ents().iter().any(|(_, e)| qents.contains(&e.id) || e.and.iter().any(|y| qents.contains(y))));
                                let text = self.texts.get(&self.frames[m].src).map(|t| t.to_lowercase()).unwrap_or_default();
                                let in_text = qents.iter().any(|x| x.len() > 2 && text.contains(x.as_str()));
                                format!("not-about{}{}", if a.anchors.is_empty() { "/noanchor" } else { "" }, if on_line { "/line" } else if in_text { "/text" } else { "" })
                            };
                            (cat, Some(m), st)
                        }
                    }
                }
            };
            let (cat, m, st) = cat;
            let mt = m.map(|m| format!("{} [{}]", self.frames[m].show(0), self.texts.get(&self.frames[m].src).map(|t| corpus_en_short(t, 100)).unwrap_or_default())).unwrap_or_default();
            // how far the frame is from the question's anchors: lines in the same lesson, else «far»
            let dist = m.map(|m| {
                let (l, n) = self.frames[m].src;
                a.anchors.iter().map(|x| self.frames[x.0].src).filter(|s| s.0 == l).map(|s| s.1.abs_diff(n)).min().map(|d| d.to_string()).unwrap_or_else(|| "far".into())
            }).unwrap_or_default();
            out.push(format!("GAPDIAG\t{cat}\t{st}\t{}\t{}\t{}\t{mt}\t{}\t{}\t{dist}", corpus_en_short(q, 100), o.text, c.show(0), qents.iter().cloned().collect::<Vec<_>>().join(","), if is_right { "right" } else { "wrong" }));
        }
        out
    }

    /// The option the student chooses, and whether it understood the question (else it guessed the first).
    pub fn choose(&self, q: &str, opts: &[String]) -> (usize, bool) {
        let a = self.answer(q, opts);
        (a.choice, a.understood)
    }

    /// Learn from the teacher's answer, role by role: the right option's features up, the wrongly chosen one's down.
    pub fn learn(&mut self, q: &str, opts: &[String], right: usize) {
        let a = self.answer_by_weights(q, opts);
        self.learn_answer(&a, right);
    }

    /// Structured perceptron correction using the witnesses that actually scored
    /// this decision. Collins (2002), https://aclanthology.org/W02-1001/.
    /// For fixed witnesses d = phi_right - phi_chosen, the local score gap grows
    /// by ||d||^2. formal/School.lean proves that identity, not global convergence.
    fn learn_answer(&mut self, a: &Answer, right: usize) {
        // A rule-computed count is independent of these weights. Also require
        // both supported alternatives: a one-sided correction is not a comparison.
        if !a.understood || a.count.is_some() || a.choice == right
            || a.options[right].best.is_none()
            || a.options[a.choice].best.is_none() {
            return;
        }
        // a counted-against witness scored minus its features (its stored score is not their weighted sum, taken with
        // the weights the decision used, before either update): its gradient is their negation
        let sign = |o: usize| a.options[o].best.as_ref().map_or(1, |(s, _, _, via, fs)| {
            let raw: i64 = fs.iter().map(|&f| self.w[widx(a.ask, *via, f)]).sum();
            if *s != raw { -1 } else { 1 }
        });
        let (sr, sc) = (sign(right), sign(a.choice));
        for (o, d) in [(right, sr), (a.choice, -sc)] {
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

/// The lesson whose questions the options of lesson `li` (of `n`) are put under in the cross-lesson control: the one half
/// the course away, cyclically. Never the lesson itself (1 ≤ n/2 < n), none when the course has one lesson.
pub fn other_lesson(li: usize, n: usize) -> Option<usize> {
    (n > 1).then(|| (li + n / 2) % n)
}

/// The two question controls of lesson `li` of a course, after its exercises: (right, asked) with each test's options
/// under the next test's question of the same lesson, and (right, asked) under the test at the same place (cyclically)
/// of `other_lesson`. `seed` draws a test's option order as the real run does; `answer` is the student's choice.
/// The next test's question is about the same lesson's topic, so a reading that finds only the topic still scores in
/// the first; in the second neither the question nor its topic fits the options, and what is left is a preference
/// for the options the lessons agree with, whatever the question asks.
pub fn question_controls(ls: &[Lesson], li: usize, seed: impl Fn(usize) -> u64, answer: impl Fn(&str, &[String]) -> usize) -> (usize, usize, usize, usize) {
    let l = &ls[li];
    let (mut shuf, mut shuf_n, mut cross, mut cross_n) = (0, 0, 0, 0);
    if l.tests.len() > 1 {
        for (i, (_, rt, w)) in l.tests.iter().enumerate() {
            let (opts, at) = shuffled(seed(i), rt, w);
            let other = &l.tests[(i + 1) % l.tests.len()].0;
            shuf += (answer(other, &opts) == at) as usize;
            shuf_n += 1;
        }
    }
    if let Some(lo) = other_lesson(li, ls.len()).map(|k| &ls[k]).filter(|lo| !lo.tests.is_empty()) {
        for (i, (_, rt, w)) in l.tests.iter().enumerate() {
            let (opts, at) = shuffled(seed(i), rt, w);
            let other = &lo.tests[i % lo.tests.len()].0;
            cross += (answer(other, &opts) == at) as usize;
            cross_n += 1;
        }
    }
    (shuf, shuf_n, cross, cross_n)
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

/// The lesson line that holds a test's answer (berzak-2020 I3, kaushik-2018 I2): the line of `lines` sharing the most
/// content words (stemmed, without stop words) with the question and its right answer — the first on a tie; none when
/// no line shares a word.
pub fn key_line(lines: &[String], q: &str, right: &str) -> Option<usize> {
    let want: BTreeSet<String> = words(q).into_iter().chain(words(right)).collect();
    let mut best: Option<(usize, usize)> = None;
    for (i, l) in lines.iter().enumerate() {
        let n = words(l).into_iter().collect::<BTreeSet<_>>().intersection(&want).count();
        if n > 0 && best.is_none_or(|(_, b)| n > b) {
            best = Some((i, n));
        }
    }
    best.map(|b| b.0)
}

/// The lesson without the key lines of its tests (`key_line`): what the third control reads.
fn without_keys(l: &Lesson) -> (Lesson, usize) {
    let lines: Vec<String> = l.explain.iter().chain(&l.facts).cloned().collect();
    let keys: BTreeSet<usize> = l.tests.iter().filter_map(|(q, r, _)| key_line(&lines, q, r)).collect();
    let mut x = l.clone();
    let ne = l.explain.len();
    x.explain = l.explain.iter().enumerate().filter(|(i, _)| !keys.contains(i)).map(|(_, s)| s.clone()).collect();
    x.facts = l.facts.iter().enumerate().filter(|(i, _)| !keys.contains(&(ne + i))).map(|(_, s)| s.clone()).collect();
    (x, keys.len())
}

/// A control track of the course (huang-2019 I1, kaushik-2018 I1): a student of its own goes through the lessons —
/// reading nothing (`read = None`), or each lesson as `read` gives it — learns from the exercises as the real student
/// does, and takes the tests. The test questions it gets right.
fn control_track(ls: &[Lesson], mut st: PragStudent, read: Option<&dyn Fn(&Lesson) -> Lesson>) -> usize {
    let mut right = 0;
    for l in ls {
        if let Some(f) = read {
            st.read(&f(l));
        }
        let seed = |i: usize, t: bool| (l.n as u64) << 20 | (i as u64) << 1 | t as u64;
        st.learn_lesson(l, 2);
        right += l.tests.iter().enumerate().filter(|(i, (q, r, w))| {
            let (opts, at) = shuffled(seed(*i, true), r, w);
            st.answer(q, &opts).choice == at
        }).count();
    }
    right
}

// ---- the trace: every test as the student took it, with its reasons (`world knowledge course <c> --trace <tsv>`) --

/// The columns of a trace file, one row per test (the exam's questions too). Options and scores are joined by « | »,
/// which no option holds (it separates them in the lesson files); the numbers of options start at 1; a score is «–»
/// for an option that found no support. Lines starting with `#` after the rows give the course's controls.
pub const TRACE_HEADER: &str = "course\tfile\tlesson\tkind\tn\tquestion\toptions\tchoice\tchosen\tright\tanswer\tresult\tscores\tunderstood\twhy";

/// One test as the student took it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceRow {
    pub course: String,
    pub file: String,
    pub lesson: usize,
    /// plain, ★, ★★, ★★★ (the lesson's stars) or exam
    pub kind: String,
    /// the test's number in its lesson (from 1), the exam's apart
    pub n: usize,
    pub question: String,
    pub options: Vec<String>,
    /// the chosen and the right option (from 0)
    pub choice: usize,
    pub right: usize,
    pub scores: Vec<Option<i64>>,
    pub understood: bool,
    pub why: String,
}

fn cell(s: &str) -> String {
    s.replace(['\t', '\n', '\r'], " ")
}

impl TraceRow {
    pub fn is_right(&self) -> bool {
        self.choice == self.right
    }

    pub fn tsv(&self) -> String {
        let opt = |i: usize| self.options.get(i).map(|s| cell(s)).unwrap_or_default();
        let scores: Vec<String> = self.scores.iter().map(|s| s.map(|x| x.to_string()).unwrap_or_else(|| "–".into())).collect();
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            cell(&self.course), cell(&self.file), self.lesson, self.kind, self.n, cell(&self.question),
            self.options.iter().map(|o| cell(o)).collect::<Vec<_>>().join(" | "),
            self.choice + 1, opt(self.choice), self.right + 1, opt(self.right),
            if self.is_right() { "right" } else { "wrong" }, scores.join(" | "),
            if self.understood { "yes" } else { "no" }, cell(&self.why)
        )
    }
}

/// What the student was asked, in plain words.
fn ask_plain(a: Ask) -> String {
    match a {
        Ask::Cause => "why something happens (its cause)".into(),
        Ask::Next => "what happens next".into(),
        Ask::Frame => "which statement is true".into(),
        Ask::Role(Slot::Verb) => "what is done".into(),
        Ask::Role(Slot::Agent) => "who or what does it".into(),
        Ask::Role(Slot::Patient) => "what it is done to".into(),
        Ask::Role(Slot::Recipient) => "whom it is for".into(),
        Ask::Role(Slot::Place) => "where".into(),
        Ask::Role(Slot::Attr) => "what something is like".into(),
        Ask::Role(Slot::Obl) => "a circumstance (with what, when, how)".into(),
    }
}

/// How the support was reached, in plain words.
fn via_plain(v: Via) -> &'static str {
    match v {
        Via::Anchor => "the remembered sentence the question matched",
        Via::Cause => "the cause of the sentence the question matched",
        Via::Next => "what followed the sentence the question matched",
        Via::Quote => "what was said in the sentence the question matched",
        Via::Linked => "a sentence linked to the one the question matched",
        Via::Line => "the same lesson line as the sentence the question matched",
        Via::Own => "the option's own match among the sentences about the question's things",
    }
}

/// A feature of a match, in plain words, with the remembered frame's word where it has one.
fn feat_plain(f: Feat, m: Option<&Frame>) -> String {
    let word = |s: Slot| m.and_then(|m| m.slot(s)).map(|e| format!(" «{}»", e.text)).unwrap_or_default();
    let role = |s: Slot| match s {
        Slot::Verb => "action",
        Slot::Agent => "doer",
        Slot::Patient => "thing acted on",
        Slot::Recipient => "receiver",
        Slot::Place => "place",
        Slot::Attr => "property",
        Slot::Obl => "circumstance",
    };
    match f {
        Feat::Match(Slot::Verb) => format!("the same action{}", m.map(|m| format!(" «{}»", m.verb)).unwrap_or_default()),
        Feat::Match(s) => format!("the same {}{}", role(s), word(s)),
        Feat::LightVerb => "the same light verb (be, have, do, get, go)".into(),
        Feat::Asked => "it fills the role the question asks for".into(),
        Feat::Conflict(s) => format!("a different {} (counts against)", role(s)),
        Feat::NegDisagree => "one says «not», the other does not (counts against)".into(),
        Feat::CauseLink => "its «because» matches the remembered cause".into(),
        Feat::VerbClass => "an action of the same kind".into(),
        Feat::Quantity => "the number the lesson gave for it".into(),
    }
}

impl PragStudent {
    /// Why the student chose what it chose, in plain words: what it took the question to ask, the score of its choice
    /// against the next best, the remembered sentence that carried the score and the top few features that decided —
    /// and, when wrong, how the right option fared.
    pub fn why(&self, a: &Answer, right: usize) -> String {
        if !a.understood {
            let how = if a.qframes.is_empty() {
                "It could not read the question into a frame"
            } else if a.anchors.is_empty() && a.options.iter().all(|o| o.best.is_none()) {
                "Nothing it remembers matched the question"
            } else {
                "No option found support in what it remembers"
            };
            return format!("{how}, so it guessed: it took the first option shown.");
        }
        if let Some(c) = &a.count {
            return format!("It counted: {}. It chose option {}, the one with that number.", c.how, a.choice + 1);
        }
        if let Some(how) = a.rule.as_ref().and_then(|r| r.how.as_ref()) {
            return how.clone();
        }
        let o = &a.options[a.choice];
        let mut out = format!("It took the question to ask {}.", ask_plain(a.ask));
        if let Some((s, _, m, via, fs)) = &o.best {
            let second = a.options.iter().enumerate().filter(|(j, _)| *j != a.choice).filter_map(|(_, x)| x.best.as_ref().map(|b| b.0)).max();
            out += &format!(" It chose option {} with score {s}{}.", a.choice + 1, match second {
                Some(x) if x == *s => format!(", tied with another option at {x} (the first shown, or the one claiming less beyond what it knows, wins)"),
                Some(x) => format!(", against {x} for the next best"),
                None => ", the only option with support".into(),
            });
            let mf = self.frames.get(*m);
            if let Some(f) = mf {
                let text = self.texts.get(&f.src).cloned().unwrap_or_default();
                out += &format!(" Its support: lesson {}, line {}: «{}», reached through {}.", f.src.0, f.src.1 + 1, corpus_en_short(&text, 160), via_plain(*via));
            } else {
                out += &format!(" Its support was reached through {}.", via_plain(*via));
            }
            // the features by their weight, the strongest first; the ones against last
            let mut ws: Vec<(i64, Feat)> = fs.iter().map(|&f| (self.w.get(widx(a.ask, *via, f)).copied().unwrap_or(0), f)).collect();
            ws.sort_by(|x, y| y.0.cmp(&x.0).then(x.1.cmp(&y.1)));
            ws.dedup_by(|x, y| x.1 == y.1);
            let pro: Vec<String> = ws.iter().filter(|x| x.0 > 0).take(4).map(|x| format!("{} (+{})", feat_plain(x.1, mf), x.0)).collect();
            let con: Vec<String> = ws.iter().filter(|x| x.0 < 0).take(2).map(|x| format!("{} ({})", feat_plain(x.1, mf), x.0)).collect();
            if !pro.is_empty() {
                out += &format!(" What matched: {}.", pro.join("; "));
            }
            if !con.is_empty() {
                out += &format!(" Against it: {}.", con.join("; "));
            }
        }
        if a.choice != right {
            out += &match a.options.get(right).and_then(|r| r.best.as_ref()) {
                Some(b) => format!(" The right option, {}, scored {}.", right + 1, b.0),
                None => format!(" The right option, {}, found no support in what it remembers.", right + 1),
            };
        }
        out
    }

    /// The trace row of a test the student answered (`a`, the right option `right`).
    pub fn trace_row(&self, course: &str, l: &Lesson, kind: &str, n: usize, q: &str, opts: &[String], right: usize, a: &Answer) -> TraceRow {
        TraceRow {
            course: course.into(),
            file: l.file.clone(),
            lesson: l.n,
            kind: kind.into(),
            n,
            question: q.into(),
            options: opts.to_vec(),
            choice: a.choice,
            right,
            scores: a.options.iter().map(|o| o.best.as_ref().map(|b| b.0)).collect(),
            understood: a.understood,
            why: self.why(a, right),
        }
    }
}

fn corpus_en_short(s: &str, n: usize) -> String {
    if s.chars().count() > n { format!("{}…", s.chars().take(n - 1).collect::<String>().trim_end()) } else { s.to_string() }
}

/// The kind of a lesson's tests: plain or its stars.
fn stars_kind(stars: usize) -> String {
    if stars == 0 { "plain".into() } else { "★".repeat(stars.min(3)) }
}

/// Whether a control went red: its share right is below the real one.
pub fn control_red(real: usize, n: usize, ctrl: usize, ctrl_n: usize) -> bool {
    ctrl_n == 0 || (ctrl as u128) * (n as u128) < (real as u128) * (ctrl_n as u128)
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
    run_course_traced(name, None)
}

/// The course run; with `trace`, every test the score counts (each lesson's tests after its exercises, and its exam)
/// is also written there as a row of `TRACE_HEADER`, the controls after the rows. The printed output is the same.
pub fn run_course_traced(name: &str, trace: Option<&std::path::Path>) -> Result<String> {
    let ls = course(name)?;
    run_course_lessons(name, &ls, trace, !prag::frames::off("course-controls"), !skip_before(), None, true)
}

/// Shared engine; benchmark tiers keep original lesson IDs and question indices.
pub(crate) fn run_course_lessons(name: &str, ls: &[Lesson], trace: Option<&std::path::Path>,
    controls: bool, before_tests: bool, cache: Option<&mut crate::parsecache::Cache>, save_school: bool) -> Result<String> {
    let mut rows: Vec<TraceRow> = Vec::new();
    let mut st = PragStudent::default();
    // MOVA_TIMING=1: where the time goes (parse ahead, read, tests, learn, exam), to stderr
    let timing = std::env::var("MOVA_TIMING").is_ok();
    let mut tm = [std::time::Duration::ZERO; 5];
    let mut learning_work = learning::LearnWork::default();
    let t0 = std::time::Instant::now();
    if let Some(cache) = cache {
        st.preparse_with(&course_texts(ls), |texts| cache.parse(texts))?;
    } else {
        st.preparse(&course_texts(ls))?;
    }
    st.read_kinds();
    tm[0] = t0.elapsed();
    // the permanent controls (PRAG_OFF=course-controls): a student that reads no lesson and one that reads each lesson
    // without the key lines of its tests, in their own threads; the questions shuffled are asked of the real student
    let fork = || {
        let mut s = PragStudent::default();
        *s.trees.borrow_mut() = st.trees.borrow().clone();
        // the same skeleton: only the lessons differ
        s.read_kinds();
        s
    };
    let no_lesson = controls.then(&fork);
    let no_key = controls.then(&fork);
    let keys_out: usize = if controls { ls.iter().map(|l| without_keys(l).1).sum() } else { 0 };
    std::thread::scope(|sc| -> Result<String> {
    let lsr: &[Lesson] = ls;
    let h_nol = no_lesson.map(|student| sc.spawn(move || control_track(lsr, student, None)));
    let h_key = no_key.map(|student| sc.spawn(move || control_track(lsr, student, Some(&|l: &Lesson| without_keys(l).0))));
    let (mut shuf_right, mut shuf_n) = (0usize, 0usize);
    let (mut cross_right, mut cross_n) = (0usize, 0usize);
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
    // a diagnostic (MOVA_STUDENT_FORGET=1): the student forgets every lesson before the next one, keeping what it
    // learned from the exercises (its weights); the difference to the real run is what earlier lessons give
    let forget = std::env::var("MOVA_STUDENT_FORGET").as_deref() == Ok("1");
    for (li, l) in ls.iter().enumerate() {
        let t = std::time::Instant::now();
        if forget {
            st.forget();
        }
        st.read(l);
        tm[1] += t.elapsed();
        let seed = |i: usize, t: bool| (l.n as u64) << 20 | (i as u64) << 1 | t as u64;
        // (right, understood, understood and right, understood and the right one shown first, decided: one best)
        let test_score = |st: &PragStudent, tally: &mut [usize; 3], mut rows: Option<&mut Vec<TraceRow>>| {
            let (mut r, mut u, mut ur) = (0, 0, 0);
            for (i, (q, rt, w)) in l.tests.iter().enumerate() {
                let (opts, at) = shuffled(seed(i, true), rt, w);
                let a = st.answer(q, &opts);
                if let Some(rows) = rows.as_deref_mut() {
                    rows.push(st.trace_row(name, l, &stars_kind(l.stars), i + 1, q, &opts, at, &a));
                    if let (Some(u), true) = (&a.rule, std::env::var("MOVA_RULEDIAG").as_deref() == Ok("1")) {
                        eprintln!("RULEDIAG\ttest\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", l.n, i + 1, u.fired, u.fitting, u.how.is_some(), a.choice == at, corpus_en_short(q, 100), u.notes.join(" | "), a.options[at].frames.iter().map(|f| f.show(0)).collect::<Vec<_>>().join(" ; "));
                    }
                    if !a.understood && std::env::var("MOVA_GAPDIAG").as_deref() == Ok("1") {
                        for line in st.gap_diag(q, &a, at) {
                            eprintln!("{line}");
                        }
                    }
                }
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
        // the score before the exercises, for the report only (MOVA_COURSE_BEFORE=0 skips it: a third of a course's
        // time, and nothing is decided on it; the report then says «–»)
        let before = if !before_tests { usize::MAX } else { test_score(&st, &mut [0; 3], None).0 };
        tm[2] += t.elapsed();
        let t = std::time::Instant::now();
        learning_work.add(st.learn_lesson(l, 2));
        tm[3] += t.elapsed();
        if timing {
            eprintln!("learning state: lesson={} weights={}", l.n,
                crate::parsecache::hash(&serde_json::to_vec(&st.w)?));
        }
        let t = std::time::Instant::now();
        let (after, und, und_r) = test_score(&st, &mut tally, trace.is_some().then_some(&mut rows));
        // the questions shuffled within the lesson, and across lessons (`question_controls`)
        if controls {
            let (s, sn, c, cn) = question_controls(ls, li, |i| seed(i, true), |q, o| st.answer(q, o).choice);
            shuf_right += s;
            shuf_n += sn;
            cross_right += c;
            cross_n += cn;
        }
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
        let mut exam = 0;
        for (i, (q, r, w)) in l.exams.iter().enumerate() {
            let (opts, at) = shuffled(seed(i, true) ^ 0xE8A3, r, w);
            let a = st.answer(q, &opts);
            exam += (a.choice == at) as usize;
            if trace.is_some() {
                rows.push(st.trace_row(name, l, "exam", i + 1, q, &opts, at, &a));
                if let (Some(u), true) = (&a.rule, std::env::var("MOVA_RULEDIAG").as_deref() == Ok("1")) {
                    eprintln!("RULEDIAG\texam\t{}\t{}\t{}\t{}\t{}\t{}\t{}", l.n, i + 1, u.fired, u.fitting, u.how.is_some(), a.choice == at, corpus_en_short(q, 100));
                }
            }
        }
        exam_right += exam;
        exam_n += l.exams.len();
        tm[4] += t.elapsed();
        let exam_note = if l.exams.is_empty() { String::new() } else { format!("; exam {exam}/{}", l.exams.len()) };
        let before = if before == usize::MAX { "–".to_string() } else { before.to_string() };
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
    // the trace's lines after the rows: the bars and the controls
    let mut footer = format!("#bar\tthe longest option\t{longest_right}\t{n_all}\n#bar\tthe first option shown\t{first_right}\t{n_all}\n");
    if controls {
        let nol = h_nol.map(|h| h.join().expect("the no-lesson control")).unwrap_or(0);
        let key = h_key.map(|h| h.join().expect("the key-line control")).unwrap_or(0);
        let ctrls = [("no lesson", nol, n_all), ("questions shuffled", shuf_right, shuf_n), ("key sentences removed", key, n_all), ("questions from another lesson", cross_right, cross_n)];
        let not_red: Vec<&str> = ctrls.iter().filter(|(_, c, cn)| !control_red(right_all, n_all, *c, *cn)).map(|(x, _, _)| *x).collect();
        for (x, c, cn) in ctrls {
            footer += &format!("#control\t{x}\t{c}\t{cn}\t{}\n", if control_red(right_all, n_all, c, cn) { "red" } else { "not red" });
        }
        out += &format!(
            "the course «{name}»'s controls (each must go red, below the real {right_all} of {n_all}): no lesson {nol} of {n_all}; questions shuffled {shuf_right} of {shuf_n}; key sentences removed ({keys_out} lines) {key} of {n_all}; questions from another lesson {cross_right} of {cross_n} — {}\n",
            if not_red.is_empty() { "all red".to_string() } else { format!("NOT RED: {}", not_red.join(", ")) }
        );
    }
    out += &format!("the course «{name}»: {} frames remembered\n", st.remembered());
    if rules_on() && std::env::var("MOVA_RULEDIAG").as_deref() == Ok("1") {
        eprintln!("RULESTATS\t{name}\t{}", st.rule_stats());
    }
    if timing {
        let cost: Vec<String> = COST_CAPS.iter().zip(COST.iter()).map(|(c, n)| format!("{}:{}", if *c == usize::MAX { "∞".to_string() } else { c.to_string() }, n.load(std::sync::atomic::Ordering::Relaxed))).collect();
        eprintln!("cost model: Σ min(cap, n) over the candidate lookups, by cap: {}", cost.join(" "));
        eprintln!("strength reductions: {}", STRENGTH_N.load(std::sync::atomic::Ordering::Relaxed));
        eprintln!("learning work: {}", serde_json::to_string(&learning_work)?);
        eprintln!("inner: align {:.1}s ({} calls), candidates {:.1}s, read {:.1}s", ALIGN_NS.load(std::sync::atomic::Ordering::Relaxed) as f64 / 1e9, ALIGN_N.load(std::sync::atomic::Ordering::Relaxed), CAND_NS.load(std::sync::atomic::Ordering::Relaxed) as f64 / 1e9, READ_NS.load(std::sync::atomic::Ordering::Relaxed) as f64 / 1e9);
        eprintln!("timing «{name}»: parse ahead {:.1}s, read {:.1}s, tests {:.1}s, learn {:.1}s, exam {:.1}s (exam includes the second test pass)", tm[0].as_secs_f64(), tm[1].as_secs_f64(), tm[2].as_secs_f64(), tm[3].as_secs_f64(), tm[4].as_secs_f64());
    }
    // the results per lesson, for the courses' pages
    if save_school {
        let dir = crate::state::root().join("school").join(name);
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join("results.txt"), &out)?;
    }
    if let Some(p) = trace {
        let mut t = format!("{TRACE_HEADER}\n");
        for r in &rows {
            t += &r.tsv();
        }
        t += &footer;
        if let Some(d) = p.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(d)?;
        }
        std::fs::write(p, t)?;
    }
    Ok(out)
    })
}

/// Where the student fails (a diagnosis of the SLM's reading): every test question of the courses, taught as
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
        st.preparse(&course_texts(&ls))?;
        for l in &ls {
            st.read(l);
            let seed = |i: usize, t: bool| (l.n as u64) << 20 | (i as u64) << 1 | t as u64;
            st.learn_lesson(l, 2);
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
    st.preparse(&course_texts(&ls))?;
    let mut out = String::new();
    for l in &ls {
        st.read(l);
        st.learn_lesson(l, 2);
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

/// A situation of the register (`situations/<domain>.tsv` of the courses): (domain, topic, situation, what is right, why).
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
/// (by domain and topic, about fifteen each) written to `queue/<domain>-<k>.tsv` of the courses for the authors.
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
    out += &format!("in all: {all} situations, {} taught, {open} not yet; {batches} batches for new lessons in the courses' queue\n", all - open);
    Ok(out)
}

/// The rule of `world knowledge check` for one parsed lesson: at least 5 facts, 12 exercises (no question twice)
/// and 8 tests, every exercise and test with exactly three wrong answers. `None` when the lesson keeps it, else the
/// line the check prints («✗ …»).
pub fn lesson_problem(l: &Lesson) -> Option<String> {
    let dup: BTreeSet<&String> = l.exercises.iter().map(|x| &x.0).collect();
    if l.exercises.len() < 12 || l.tests.len() < 8 || l.facts.len() < 5 || dup.len() < l.exercises.len() || l.exercises.iter().chain(&l.tests).any(|x| x.2.len() != 3) {
        return Some(format!("✗ {} ({}): facts {}, exercises {}, tests {}; every question needs exactly three wrong answers", l.n, l.file, l.facts.len(), l.exercises.len(), l.tests.len()));
    }
    None
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
        Some("kinds-up") => {
            // world knowledge kinds-up <word>…: a noun's kinds by WordNet's meanings, the nearer first
            for w in &args[2..] {
                println!("{w}: {}", crate::wordnet::kinds_up(w, "noun").join(" → "));
            }
        }
        Some("dict-kinds") => {
            // world knowledge dict-kinds <word>…: the kinds the loaded dictionary names a noun as, as the student reads
            // them (`dict_kinds`), with the definition read
            let s = PragStudent::default();
            for w in &args[2..] {
                let w = w.to_lowercase();
                let def = crate::dictstore::store().first_meaning(&w).map(|x| x.0).unwrap_or_default();
                println!("{w}\t{}\t{def}", s.dict_kinds(&w).join(", "));
            }
        }
        Some("retain") => retention::main_args(&args[2..])?,
        Some("bench") => crate::schoolbench::main_args(&args[2..])?,
        Some("course") => {
            // world knowledge course <name> [--trace <out.tsv>]: every test with the student's choice and its reasons
            let trace = args.iter().position(|a| a == "--trace").map(|i| args.get(i + 1).cloned().ok_or_else(|| anyhow::anyhow!("--trace needs a file")));
            let trace = trace.transpose()?.map(std::path::PathBuf::from);
            print!("{}", run_course_traced(&name, trace.as_deref())?)
        }
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
                if let Some(p) = lesson_problem(l) {
                    bad += 1;
                    println!("{p}");
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
    fn the_wikipedia_courses_stay_outside_the_repo() {
        let public = root().join("world");
        assert_eq!(course_dir("world"), public, "an ordinary course is in the courses of the school data");
        let wiki = course_dir("wikipedia-cv-a");
        // the closed build puts the CC BY-SA courses beside their corpus; the free build has no such course
        assert_eq!(wiki.starts_with(root()), !cfg!(feature = "peta"), "{}", wiki.display());
        // negative control: a public course whose name only contains the word is not moved
        assert!(course_dir("wikipedian").starts_with(root()));
    }

    #[test]
    fn the_starred_option_is_the_right_one() {
        let (q, r, w) = question("What is right? | Yes, cheese | *No, seeds and grain | Only meat", "x").unwrap();
        assert_eq!((q.as_str(), r.as_str()), ("What is right?", "No, seeds and grain"));
        assert_eq!(w, vec!["Yes, cheese".to_string(), "Only meat".to_string()], "the wrong options keep their order");
        // the old form: no star, the first option is right
        assert_eq!(question("Q | a | b", "x").unwrap().1, "a");
        // negative control: two stars are an error, not a guess
        assert!(question("Q | *a | *b", "x").is_err());
    }

    #[test]
    fn the_grammar_school_is_its_own_layer() {
        assert_eq!(grade("grammar-en"), GRAMMAR);
        assert!(COURSES.iter().any(|c| c.0 == "grammar-hbo"));
        // every grammar course is registered with its grade, and none mixes into the primary courses
        assert!(COURSES.iter().filter(|c| c.0.starts_with("grammar-")).all(|c| grade(c.0) == GRAMMAR));
        assert!(!course_names().iter().any(|n| n.starts_with("grammar-")));
        // negative control: a primary course stays primary
        assert_eq!(grade("english"), PRIMARY);
    }

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

#[cfg(test)]
mod citation_tests {
    use super::*;

    #[test]
    fn citations_and_layers() {
        assert_eq!(strip_citations("God calls Abram (Genesis 12:1) to go."), "God calls Abram to go.");
        assert_eq!(strip_citations("The ark (a big boat) floats (Genesis 7:17-18)."), "The ark (a big boat) floats.");
        // negative control: a parenthesis that is not a reference stays
        assert_eq!(strip_citations("Ten (10) men came."), "Ten (10) men came.");
        let f = fact_lines("Genesis 1:1 in Hebrew: בְּרֵאשִׁית — word for word: in beginning — in English (JPS 1917): IN THE beginning God created. — the idea: The world has a Creator.");
        assert_eq!(f, vec!["IN THE beginning God created.".to_string(), "The world has a Creator.".to_string()]);
    }
}

#[cfg(test)]
mod trace_tests {
    use super::*;

    fn opt(text: &str, score: Option<i64>) -> OptionRead {
        OptionRead { text: text.into(), best: score.map(|s| (s, 0, 7, Via::Anchor, vec![Feat::Match(Slot::Verb), Feat::Asked])), ..Default::default() }
    }

    fn answer(choice: usize, understood: bool, scores: &[Option<i64>]) -> Answer {
        let options = scores.iter().enumerate().map(|(i, s)| opt(&format!("option {i}"), *s)).collect();
        Answer { qframes: Vec::new(), ask: Ask::Role(Slot::Patient), gap: None, anchors: Vec::new(), targets: Vec::new(), options, choice, understood, count: None, rule: None }
    }

    #[test]
    fn a_trace_row_says_right_or_wrong() {
        let st = PragStudent::default();
        let l = Lesson { course: "x".into(), n: 3, file: "003-x.lesson".into(), stars: 2, ..Default::default() };
        let opts: Vec<String> = ["Seeds", "Cheese", "Meat", "Stones"].iter().map(|s| s.to_string()).collect();
        // understood and right: the scores in the options' order, the reasons name the score and the features
        let a = answer(1, true, &[Some(2), Some(9), None, Some(4)]);
        let r = st.trace_row("birds", &l, &stars_kind(l.stars), 1, "What do birds eat?", &opts, 1, &a);
        assert!(r.is_right());
        let row = r.tsv();
        let cols: Vec<&str> = row.trim_end().split('\t').collect();
        assert_eq!(cols.len(), TRACE_HEADER.split('\t').count());
        assert_eq!((cols[0], cols[2], cols[3], cols[7], cols[8], cols[9], cols[11], cols[12]), ("birds", "3", "★★", "2", "Cheese", "2", "right", "2 | 9 | – | 4"));
        assert!(r.why.contains("score 9, against 4 for the next best") && r.why.contains("the same action"), "{}", r.why);
        // negative control: the same test answered wrongly must say wrong, and how the right option fared
        let r = st.trace_row("birds", &l, "★★", 1, "What do birds eat?", &opts, 0, &a);
        assert!(!r.is_right());
        assert_eq!(r.tsv().split('\t').nth(11), Some("wrong"));
        assert!(r.why.contains("The right option, 1, scored 2."), "{}", r.why);
        // a guess: not understood, wrong, and the reasons say it guessed
        let g = answer(0, false, &[None, None, None, None]);
        let r = st.trace_row("birds", &l, "exam", 2, "Q?", &opts, 3, &g);
        assert_eq!(r.tsv().split('\t').nth(11), Some("wrong"));
        assert!(r.why.contains("guessed"), "{}", r.why);
        // a tab inside a text never breaks the row
        let r = st.trace_row("birds", &l, "plain", 1, "a\tb", &opts, 0, &g);
        assert_eq!(r.tsv().split('\t').count(), TRACE_HEADER.split('\t').count());
    }
}

#[cfg(test)]
mod control_tests {
    use super::*;

    #[test]
    fn the_key_line_and_the_verdict() {
        let lines: Vec<String> = ["Foxes live in dens.", "The fox ate the grapes in the garden.", "Grapes grow on vines."].iter().map(|s| s.to_string()).collect();
        assert_eq!(key_line(&lines, "What did the fox eat in the garden?", "The grapes"), Some(1));
        // nothing shared: no key line
        assert_eq!(key_line(&lines, "Who sang?", "A bird"), None);
        // a control below the real share is red; one level with it or above is not — the gate can show it
        assert!(control_red(50, 100, 30, 100));
        assert!(!control_red(50, 100, 50, 100));
        assert!(!control_red(50, 100, 26, 50), "52% against 50% is not red");
    }

    /// The cross-lesson control separates a reader that finds only the question's topic from one that reads the
    /// question: a topic reader (the option naming a word of the question) is as good under the next test of the same
    /// lesson as under its own, so the shuffled control cannot show it red; under another lesson's question it falls.
    /// The negative control: a student that ignores the question (the longest option) is not red against either.
    #[test]
    fn the_cross_lesson_control_tells_topic_from_question() {
        let t = |q: &str, r: &str, w: [&str; 3]| (q.to_string(), r.to_string(), w.iter().map(|x| x.to_string()).collect::<Vec<_>>());
        let lesson = |n: usize, tests: Vec<(String, String, Vec<String>)>| Lesson { n, tests, ..Default::default() };
        let ls = vec![
            lesson(1, vec![t("What does the fox eat?", "the fox eats grapes", ["owls hunt", "bees make honey", "cows give milk"]), t("Where does the fox sleep?", "the fox sleeps in a den", ["owls nest", "bees live in a hive", "cows rest in a barn"])]),
            lesson(2, vec![t("What do owls hunt?", "owls hunt mice", ["foxes eat", "bees sting", "cows moo"]), t("When do owls fly?", "owls fly at night", ["foxes run", "bees buzz by day", "cows walk"])]),
            lesson(3, vec![t("What do bees make?", "bees make honey", ["a fox eats", "owls see", "cows graze"]), t("Where do bees live?", "bees live in a hive", ["a fox digs", "owls perch", "cows roam"])]),
        ];
        let words = |x: &str| x.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() > 2 && !matches!(*w, "what" | "does" | "where" | "when" | "the")).map(|w| w.trim_end_matches('s').to_string()).collect::<BTreeSet<String>>();
        // the topic reader: the option sharing a content word with the question
        let topic = |q: &str, opts: &[String]| { let qw = words(q); opts.iter().position(|o| !words(o).is_disjoint(&qw)).unwrap_or(0) };
        // the option prior: the longest option, whatever the question
        let prior = |_: &str, opts: &[String]| (0..opts.len()).max_by_key(|&i| (opts[i].len(), std::cmp::Reverse(i))).unwrap_or(0);
        let real = |answer: &dyn Fn(&str, &[String]) -> usize| -> (usize, usize) {
            let mut r = (0, 0);
            for (li, l) in ls.iter().enumerate() {
                for (i, (q, rt, w)) in l.tests.iter().enumerate() {
                    let (opts, at) = shuffled((li as u64) << 20 | (i as u64) << 1 | 1, rt, w);
                    r.0 += (answer(q, &opts) == at) as usize;
                    r.1 += 1;
                }
            }
            r
        };
        let ctrl = |answer: &dyn Fn(&str, &[String]) -> usize| -> (usize, usize, usize, usize) {
            (0..ls.len()).map(|li| question_controls(&ls, li, |i| (li as u64) << 20 | (i as u64) << 1 | 1, |q, o| answer(q, o))).fold((0, 0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3))
        };
        // the pairs: another lesson, always, and none in a course of one lesson
        assert!((0..7).all(|n| (0..n).all(|li| other_lesson(li, n).is_none_or(|k| k != li && k < n))));
        assert_eq!(other_lesson(0, 1), None);
        let (r, n) = real(&topic);
        let (s, sn, c, cn) = ctrl(&topic);
        assert_eq!((r, n, sn, cn), (6, 6, 6, 6));
        assert!(!control_red(r, n, s, sn), "the same-lesson shuffle cannot catch a topic reader: {s} of {sn}");
        assert!(control_red(r, n, c, cn), "the cross-lesson control does: {c} of {cn}");
        // negative control: a student that never reads the question is red against neither
        let (r, n) = real(&prior);
        let (s, sn, c, cn) = ctrl(&prior);
        assert!(!control_red(r, n, s, sn) && !control_red(r, n, c, cn), "{r}/{n}: shuffled {s}, cross {c}");
    }
}

#[cfg(test)]
mod gap_tests {
    use super::*;

    fn ent(id: &str) -> Ent {
        Ent { id: id.into(), text: id.into(), ..Default::default() }
    }

    /// A plain event frame (the reader's constructor is private to it: built through its serialized form).
    fn frame(verb: &str, agent: &str, patient: Option<&str>, src: (usize, usize)) -> Frame {
        let v = serde_json::json!({
            "kind": serde_json::to_value(Kind::Event).unwrap(), "agent": serde_json::to_value(ent(agent)).unwrap(), "verb": verb,
            "neg": false, "patient": serde_json::to_value(patient.map(ent)).unwrap(), "recipient": null, "place": null,
            "attrs": [], "manner": [], "obls": [], "quote": [], "quoted": false, "cause": null, "next": null, "cond": false,
            "src": [src.0, src.1], "how": serde_json::to_value(prag::frames::How::default()).unwrap(), "unsaid": [], "words": [],
            "nps": [], "mark": [], "cc": null, "link": null, "tam": serde_json::to_value(prag::frames::Tam::default()).unwrap(),
            "irrealis": false,
        });
        serde_json::from_value(v).unwrap()
    }

    /// The categories of the not-understood diagnostic: an option frame supported only by a remembered frame about none
    /// of the question's things is `not-about`, with its distance in lines from the question's anchor; an unknown verb
    /// is `verb-unknown`; and (the negative control) once that frame is a target and about the question, it is
    /// `reached-gated`, not `not-about`: the classifier can tell the cases apart.
    #[test]
    fn the_gap_diagnostic_names_why_an_option_found_no_support() {
        let mut st = PragStudent::default();
        st.remember_frame(frame("pump", "heart", Some("blood"), (1, 0)));
        st.remember_frame(frame("press", "nurse", Some("wrist"), (1, 1)));
        let q = frame("press", "nurse", Some("wrist"), (0, 0));
        let opt = |text: &str, f: Frame| OptionRead { text: text.into(), compared: vec![f], ..Default::default() };
        let options = vec![opt("The heart pumps blood", frame("pump", "heart", Some("blood"), (0, 0))), opt("She sings", frame("zzsing", "nurse", None, (0, 0)))];
        let mut a = Answer { qframes: vec![q], ask: Ask::Frame, gap: None, anchors: vec![(1, 4)], targets: vec![(1, Via::Anchor)], options, choice: 0, understood: false, count: None, rule: None };
        let rows = st.gap_diag("A nurse presses your wrist. Why?", &a, 0);
        let field = |r: &str, i: usize| r.split('\t').nth(i).unwrap_or_default().to_string();
        assert_eq!(rows.len(), 2);
        assert_eq!((field(&rows[0], 1), field(&rows[0], 8), field(&rows[0], 9)), ("not-about".into(), "right".into(), "1".into()));
        assert_eq!((field(&rows[1], 1), field(&rows[1], 8)), ("verb-unknown".into(), "wrong".into()));
        // negative control: the same option frame, its support now a target and about the question
        a.qframes = vec![frame("press", "heart", None, (0, 0))];
        a.targets.push((0, Via::Line));
        let rows = st.gap_diag("Why does the heart press?", &a, 0);
        assert_eq!(field(&rows[0], 1), "reached-gated");
    }
}

#[cfg(test)]
mod rule_tests {
    use super::*;

    fn ent(id: &str) -> Ent {
        Ent { id: id.into(), text: id.into(), ..Default::default() }
    }

    /// A frame built through its serialized form (the reader's constructor is private to it).
    fn frame(kind: Kind, verb: &str, agent: Option<&str>, patient: Option<Ent>, src: (usize, usize)) -> Frame {
        let v = serde_json::json!({
            "kind": serde_json::to_value(kind).unwrap(), "agent": serde_json::to_value(agent.map(ent)).unwrap(), "verb": verb,
            "neg": false, "patient": serde_json::to_value(patient).unwrap(), "recipient": null, "place": null,
            "attrs": [], "manner": [], "obls": [], "quote": [], "quoted": false, "cause": null, "next": null, "cond": false,
            "src": [src.0, src.1], "how": serde_json::to_value(prag::frames::How::default()).unwrap(), "unsaid": [], "words": [],
            "nps": [], "mark": [], "cc": null, "link": null, "tam": serde_json::to_value(prag::frames::Tam::default()).unwrap(),
            "irrealis": false,
        });
        serde_json::from_value(v).unwrap()
    }

    /// «"Can you…?" … asks you to do it» compiled and applied: the asked «can u pass the ketchup» binds the action,
    /// and of the options only «pass him the ketchup» says the conclusion. Negative controls: the same words stated
    /// («you can pass the ketchup») fire nothing; under another question («can you open the window?») the rule fires
    /// but no option fits; an option that shares the bound thing with another verb («eat the ketchup») never fits —
    /// a binding alone is no evidence (with the verb check removed it would fit too, and the choice would be lost).
    #[test]
    fn a_compiled_rule_decides_only_by_its_conclusion() {
        let mut st = PragStudent::default();
        let src = (2, 5);
        st.texts.insert(src, "\"Can you…?\" with the job right in front of you asks you to do it.".into());
        let mut pattern = frame(Kind::Fact(crate::frames::Link::Can), "…", Some("you"), None, src);
        pattern.quoted = true;
        let mut asks = frame(Kind::Say(prag::schema::Act::Ask), "ask", None, None, src);
        asks.quote = vec![0];
        let mut doit = frame(Kind::Event, "do", Some("you"), Some(Ent { pron: Some("it".into()), ..ent("it") }), src);
        doit.link = Some((1, prag::frames::Rel::Xcomp, None));
        for f in [pattern, asks, doit] {
            st.remember_frame(f);
        }
        let mut q = frame(Kind::Fact(crate::frames::Link::Can), "pass", Some("you"), Some(ent("ketchup")), (0, 0));
        q.quoted = true;
        let opt = |text: &str, f: Frame| OptionRead { text: text.into(), frames: vec![f], ..Default::default() };
        let options = vec![
            opt("Say yeah I can", frame(Kind::Say(prag::schema::Act::Assert), "say", Some("you"), None, (0, 0))),
            opt("Pass him the ketchup", frame(Kind::Event, "pass", Some("you"), Some(ent("ketchup")), (0, 0))),
            opt("Point to where it is", frame(Kind::Event, "point", Some("you"), None, (0, 0))),
            opt("Eat the ketchup", frame(Kind::Event, "eat", Some("you"), Some(ent("ketchup")), (0, 0))),
        ];
        assert!(st.rule_stats().starts_with("1 rules"), "{}", st.rule_stats());
        let (u, pick) = st.apply_rules(&[(&q, true)], &options);
        assert_eq!((u.fired, u.fitting, pick), (1, 1, Some(1)));
        assert!(u.how.as_deref().is_some_and(|h| h.contains("lesson 2, line 6") && h.contains("Option 2")), "{:?}", u.how);
        // negative controls
        let (u, pick) = st.apply_rules(&[(&q, false)], &options);
        assert_eq!((u.fired, pick), (0, None), "a stated «can» is no request");
        let mut other = frame(Kind::Fact(crate::frames::Link::Can), "open", Some("you"), Some(ent("window")), (0, 0));
        other.quoted = true;
        let (u, pick) = st.apply_rules(&[(&other, true)], &options);
        assert_eq!((u.fired, u.fitting, pick), (1, 0, None), "another question's action fits none of these options");
        let th = st.match_pattern(&st.frames[0].clone(), &q, true).expect("the pattern matches");
        let concl = st.conclusion(&st.rulebase.borrow().rules[0].clone(), &th);
        assert_eq!(concl.len(), 1);
        assert!(st.says_conclusion(&concl[0], &options[1].frames[0]).is_some());
        assert!(st.says_conclusion(&concl[0], &options[3].frames[0]).is_none(), "the bound thing with another verb says nothing");
    }
}

#[cfg(test)]
mod verb_tests {
    use super::*;

    /// The question's verb gates support. The student read «Henry founded the abbey in 1150. Richard visited the abbey
    /// in 1190.»; «Who founded the abbey?» is answered (Henry), and so is «Who established the abbey?» (a WordNet
    /// synonym). The verb control «Who sold the abbey?» finds no support: the frames name the abbey, but no remembered
    /// verb states «sell». Negative control: the same student with the gate off chooses an abbey-person for «sold»,
    /// understood — the association the gate removes. Content verbs of the question are exempt: «want», «take place».
    #[test]
    fn the_question_verb_gates_support() {
        crate::tree::annotator().expect("en model");
        assert!(!crate::wordnet::wordnet().senses.is_empty(), "WordNet");
        let mut st = PragStudent::default();
        st.verb_gate = Some(VerbGate { all: false, strict: false });
        let l = Lesson { course: "x".into(), n: 1, facts: vec!["Henry founded the abbey in 1150.".into(), "Richard visited the abbey in 1190.".into()], ..Default::default() };
        st.read(&l);
        let opts: Vec<String> = ["Richard", "Henry", "John", "Edward"].iter().map(|s| s.to_string()).collect();
        let a = st.answer("Who founded the abbey?", &opts);
        assert_eq!((a.choice, a.understood), (1, true));
        let a = st.answer("Who established the abbey?", &opts);
        assert_eq!((a.choice, a.understood), (1, true), "a synonym states the verb");
        let a = st.answer("Who sold the abbey?", &opts);
        assert!(!a.understood && a.options.iter().all(|o| o.best.is_none()), "no remembered frame states «sell»");
        // negative control: without the gate the entities alone carry an answer
        st.verb_gate = None;
        let a = st.answer("Who sold the abbey?", &opts);
        assert!(a.understood && a.options.iter().any(|o| o.best.is_some()));
        assert!(verb_agrees("found", "establish") && verb_agrees("pull", "pull down") && !verb_agrees("sell", "found") && !verb_agrees("destroy", "found"));
        let f = |v: &str, p: Option<&str>| -> Frame {
            let mut x: Frame = serde_json::from_value(serde_json::json!({
                "kind": serde_json::to_value(Kind::Event).unwrap(), "agent": null, "verb": v, "neg": false, "patient": null,
                "recipient": null, "place": null, "attrs": [], "manner": [], "obls": [], "quote": [], "quoted": false, "cause": null,
                "next": null, "cond": false, "src": [0, 0], "how": serde_json::to_value(prag::frames::How::default()).unwrap(),
                "unsaid": [], "words": [], "nps": [], "mark": [], "cc": null, "link": null,
                "tam": serde_json::to_value(prag::frames::Tam::default()).unwrap(), "irrealis": false,
            })).unwrap();
            x.patient = p.map(|y| Ent { id: y.into(), text: y.into(), ..Default::default() });
            x
        };
        assert!(verb_light_q(&f("want", None)) && verb_light_q(&f("take", Some("place"))) && verb_light_q(&f("be", None)));
        assert!(!verb_light_q(&f("found", None)), "found");
        assert!(!verb_light_q(&f("sell", None)) && !verb_light_q(&f("destroy", None)) && !verb_light_q(&f("build", None)), "sell, destroy, build");
    }

    /// A lesson frame that denies an option is no support. The student read «The fox does not eat grass. The fox
    /// sleeps in a den.»; with the negation's own weight learned away (0 here, as the exercises can move it; every role
    /// match 2 on every way) «The fox eats grass» matches verb, doer and thing (raw 6) and would beat «The fox sleeps»
    /// (4). Counted against, it scores −6 and the right option wins. Negative control: the raw weighted sums of the two
    /// witnesses order them the other way.
    #[test]
    fn a_contradicting_lesson_frame_is_no_support() {
        crate::tree::annotator().expect("en model");
        let mut st = PragStudent::default();
        for a in [Ask::Cause, Ask::Next, Ask::Frame].into_iter().chain(SLOTS.iter().map(|&s| Ask::Role(s))) {
            for v in VIAS {
                st.w[widx(a, v, Feat::NegDisagree)] = 0;
                for s in [Slot::Verb, Slot::Agent, Slot::Patient, Slot::Place] {
                    st.w[widx(a, v, Feat::Match(s))] = 2;
                }
            }
        }
        let l = Lesson { course: "x".into(), n: 1, facts: vec!["The fox does not eat grass.".into(), "The fox sleeps in a den.".into()], ..Default::default() };
        st.read(&l);
        let opts: Vec<String> = ["The fox eats grass", "The fox sleeps"].iter().map(|s| s.to_string()).collect();
        let a = st.answer("A fox is in a field. What is true?", &opts);
        let raw = |o: &OptionRead| o.best.as_ref().map(|(_, _, _, via, fs)| fs.iter().map(|&f| st.w[widx(a.ask, *via, f)]).sum::<i64>());
        let (b0, b1) = (a.options[0].best.as_ref().expect("a witness for the contradicted option"), a.options[1].best.as_ref().expect("support for the right option"));
        assert!(contradicts(&b0.4, false, true), "{:?}", b0.4);
        assert!(b0.0 < 0 && b1.0 > 0, "{} {}", b0.0, b1.0);
        assert_eq!((a.choice, a.understood), (1, true));
        assert!(raw(&a.options[0]) > raw(&a.options[1]), "control: the raw sums prefer the contradicted option: {b0:?} {b1:?} {:?} {:?}", raw(&a.options[0]), raw(&a.options[1]));
    }
}

#[cfg(test)]
mod school_math_tests {
    use super::*;

    fn features(mask: u16) -> Vec<Feat> {
        ALL_FEATS.iter().copied().filter(|&f| mask & (1 << feat_idx(f)) != 0).collect()
    }

    #[test]
    fn mask_strength_matches_every_u16_feature_interpretation() {
        for mask in 0..=u16::MAX {
            assert_eq!(strength_mask(mask), strength(&features(mask)), "mask {mask:#x}");
        }
        assert_eq!(strength_mask(1 << feat_idx(Feat::Match(Slot::Attr))), 1);
        assert_eq!(strength_mask(1 << feat_idx(Feat::Conflict(Slot::Verb))), 0);
    }

    fn fixture() -> (PragStudent, Answer, Vec<Option<Support>>) {
        let (a, b, c) = (Feat::Match(Slot::Agent), Feat::Match(Slot::Patient), Feat::Match(Slot::Verb));
        let support = |score, memory, fs| Some((score, 0, memory, Via::Own, fs));
        let mut st = PragStudent::default();
        st.w.fill(0);
        for (f, v) in [(a, 1), (b, 2), (c, 10)] {
            st.w[widx(Ask::Frame, Via::Own, f)] = v;
        }
        let options = vec![
            OptionRead { best: support(12, 0, vec![b, c]), recent: Some(1), ..Default::default() },
            OptionRead { best: support(11, 1, vec![a, c]), recent: Some(2), ..Default::default() },
            OptionRead { best: support(10, 2, vec![c]), ..Default::default() },
        ];
        // Current-lesson scoring chooses option 1 (2 > 1), despite option 0's
        // higher global score. The third option has only older support.
        let ans = Answer { qframes: vec![], ask: Ask::Frame, gap: None, anchors: vec![], targets: vec![],
            options, choice: 1, understood: true, count: None, rule: None };
        let witnesses = vec![support(1, 3, vec![a]), support(2, 4, vec![b]), None];
        (st, ans, witnesses)
    }

    fn active_gap(st: &PragStudent) -> i64 {
        st.w[widx(Ask::Frame, Via::Own, Feat::Match(Slot::Agent))]
            - st.w[widx(Ask::Frame, Via::Own, Feat::Match(Slot::Patient))]
    }

    #[test]
    fn selected_witness_updates_the_decision_gap_and_explanation() {
        let (mut st, mut a, witnesses) = fixture();
        let before = active_gap(&st);
        assert!(current_support(&mut a.options, witnesses, true));
        assert_eq!(a.options[0].best.as_ref().map(|x| (x.0, x.2)), Some((1, 3)));
        assert_eq!(a.options[1].best.as_ref().map(|x| (x.0, x.2)), Some((2, 4)));
        assert!(a.options[2].best.is_none(), "older-only support escaped the active scope");
        st.learn_answer(&a, 0);
        assert_eq!(active_gap(&st) - before, 2); // ||[1,-1,0]||^2
        // Negative control: the old global witnesses move the active gap backwards.
        let (mut old, a, _) = fixture();
        old.learn_answer(&a, 0);
        assert_eq!(active_gap(&old) - before, -2);
    }

    #[test]
    fn scope_fallback_and_non_weight_decisions_do_not_learn_wrong_features() {
        let (_, mut a, witnesses) = fixture();
        assert!(!current_support(&mut a.options, witnesses, false));
        assert_eq!(a.options[0].best.as_ref().unwrap().0, 12);
        assert!(!current_support(&mut a.options, vec![None; 3], true));
        assert_eq!(a.options[0].best.as_ref().unwrap().0, 12);
        let (mut st, mut a, witnesses) = fixture();
        current_support(&mut a.options, witnesses, true);
        let before = st.w.clone();
        st.learn_answer(&a, 2); // unsupported right option
        assert_eq!(st.w, before);
        a.choice = 2; // unsupported chosen option
        st.learn_answer(&a, 0);
        assert_eq!(st.w, before);
        a.choice = 1;
        a.count = Some(Count { value: 7, how: "rule-computed".into() });
        st.learn_answer(&a, 0);
        assert_eq!(st.w, before, "a count decision is independent of these weights");
    }

    /// A lesson's denial counts against what an option asserts (the same content verb and a shared core role, the
    /// lesson's frame negated): «daylight does NOT blind owls» never supports «the sun blinds it». Controls: the same
    /// polarity, only a light verb, no shared core role, an irrealis-only difference (NegDisagree with equal
    /// polarity), the option's own denial (off by default) — the weighted sum as before; a sum already against stays.
    #[test]
    fn a_contradicting_witness_counts_against() {
        let (v, p, ag, n) = (Feat::Match(Slot::Verb), Feat::Match(Slot::Patient), Feat::Conflict(Slot::Agent), Feat::NegDisagree);
        assert!(contradicts(&[v, p, ag, n], false, true));
        assert_eq!(counter(7, &[v, p, ag, n], false, true), -7);
        assert_eq!(counter(7, &[v, p, ag], false, false), 7, "control: the same polarity");
        assert_eq!(counter(7, &[v, p, ag, n], true, true), 7, "control: both negated, an irrealis difference only");
        assert_eq!(counter(7, &[Feat::LightVerb, p, n], false, true), 7, "control: a light verb only");
        assert_eq!(counter(7, &[v, Feat::Match(Slot::Attr), n], false, true), 7, "control: no shared core role");
        assert_eq!(counter(7, &[v, p, n], true, false), 7, "control: the option's own denial (PRAG_VN=student-neg-both)");
        assert_eq!(counter(-2, &[v, p, n], false, true), -2, "a sum already against stays");
    }

    /// The perceptron step on a counted-against witness follows its sign: the right option's score (minus its
    /// features) rises against the chosen one. Negative control: the plain step (features up) lowers it.
    #[test]
    fn learning_follows_the_sign_of_a_counted_against_witness() {
        let (v, p, n, ag) = (Feat::Match(Slot::Verb), Feat::Match(Slot::Patient), Feat::NegDisagree, Feat::Match(Slot::Agent));
        let mut st = PragStudent::default();
        st.w.fill(0);
        for (f, x) in [(v, 2), (p, 2), (ag, 1)] {
            st.w[widx(Ask::Frame, Via::Own, f)] = x;
        }
        let sup = |s, fs: Vec<Feat>| Some((s, 0, 0, Via::Own, fs));
        // the right option's witness is a lesson's denial: counted against (−4); the chosen one 1
        let options = vec![OptionRead { best: sup(-4, vec![n, p, v]), ..Default::default() }, OptionRead { best: sup(1, vec![ag]), ..Default::default() }];
        let a = Answer { qframes: vec![], ask: Ask::Frame, gap: None, anchors: vec![], targets: vec![], options, choice: 1, understood: true, count: None, rule: None };
        let score = |st: &PragStudent, fs: &[Feat]| counter(fs.iter().map(|&f| st.w[widx(Ask::Frame, Via::Own, f)]).sum(), fs, false, fs.contains(&n));
        let gap = |st: &PragStudent| score(st, &[n, p, v]) - score(st, &[ag]);
        let before = gap(&st);
        let mut plain = st.w.clone();
        st.learn_answer(&a, 0);
        assert!(gap(&st) > before, "{} -> {}", before, gap(&st));
        // negative control: features of the right witness up, of the chosen down, ignoring the sign
        for f in [n, p, v] {
            plain[widx(Ask::Frame, Via::Own, f)] += 1;
        }
        plain[widx(Ask::Frame, Via::Own, ag)] -= 1;
        let mut ctl = PragStudent::default();
        ctl.w = plain;
        assert!(gap(&ctl) < before, "control: the plain step moves the gap the wrong way");
    }

    #[test]
    #[ignore = "bounded Rust microbenchmark, run before course-level measurements"]
    fn strength_kernel_benchmark() {
        use std::{hint::black_box, time::Instant};
        const N: u32 = 2_000_000;
        for trial in 0..4 {
            let mut times = [0u128; 2];
            let mut sums = [0i64; 2];
            for mode in if trial % 2 == 0 { [0, 1] } else { [1, 0] } {
                let start = Instant::now();
                for i in 0..N {
                    let mask = black_box(i.wrapping_mul(40503) as u16);
                    sums[mode] += black_box(if mode == 0 {
                        strength(black_box(&features(mask)))
                    } else {
                        strength_mask(mask)
                    });
                }
                times[mode] = start.elapsed().as_nanos();
            }
            assert_eq!(sums[0], sums[1]);
            println!("{}", serde_json::json!({"trial":trial,"calls":N,"vector_ns":times[0],
                "mask_ns":times[1],"checksum":sums[0],"warmup":trial==0}));
        }
    }
}

#[cfg(test)]
mod wave34_tests {
    use super::*;

    #[test]
    fn citation_scan_preserves_nonreferences_and_utf8_suffixes() {
        for (input, expected) in [
            ("Élan (Genesis 1:2) walks.", "Élan walks."),
            ("A (big boat) floats.", "A (big boat) floats."),
            ("A (Genesis 1:2) (Exodus 2:3).", "A."),
            ("A (genesis 1:2).", "A (genesis 1:2)."),
            ("A (Genesis 1:x).", "A (Genesis 1:x)."),
            ("A (unfinished", "A (unfinished"),
            ("A (outer (Genesis 1:2)) Z", "A (outer) Z"),
        ] {
            assert_eq!(strip_citations(input), expected, "{input}");
        }
    }

    #[test]
    fn layered_fact_projection_has_ordered_marker_precedence() {
        let source = "Hebrew — in English (JPS 1917):  A (Genesis 1:2).  — the idea:  B. ";
        assert_eq!(fact_lines(source), vec!["A (Genesis 1:2).", "B."]);
        assert_eq!(
            fact_lines(" — in English (JPS 1917):  — the idea: "),
            vec!["", ""]
        );
        assert_eq!(fact_lines("A (Genesis 1:2)."), vec!["A."]);
        let reversed = " — the idea: B — in English (JPS 1917): A";
        assert_eq!(fact_lines(reversed), vec![reversed]);
    }

    #[test]
    fn field_fold_overwrites_scalars_appends_lists_and_fails_first() {
        let l = parse_fields("# comment\ncourse: first\ncourse: last\nlesson: 2\nlesson: 3\nfact: A.\nfact: B.\nexercise: Q | *R | W\n", "synthetic").unwrap();
        assert_eq!(
            (l.course.as_str(), l.n, l.file.as_str()),
            ("last", 3, "synthetic")
        );
        assert_eq!(l.facts, vec!["A.", "B."]);
        assert_eq!(
            l.exercises,
            vec![("Q".into(), "R".into(), vec!["W".into()])]
        );
        assert!(parse("lesson: 1\nfact: A.\nexercise: Q | R | W\n", "synthetic").is_err());
        assert!(
            parse(
                "lesson: 1\nfact: A.\nexercise: Q | R | W\ntest: Q | R | W",
                "synthetic"
            )
            .is_ok()
        );
        let e = parse_fields("unknown: x\nlesson: broken", "synthetic")
            .unwrap_err()
            .to_string();
        assert!(e.contains("unknown field"));
        assert!(parse_fields("no delimiter", "synthetic").is_err());
    }

    #[test]
    fn tail_recognition_keeps_scenario_and_rejects_short_or_unknown() {
        for (tail, expected) in [
            ("WHY?!", Tail::Cause),
            ("What happens next?", Tail::Next),
            ("Is that true?", Tail::Claim),
        ] {
            let q = format!("Élan walks home. {tail}   ");
            // A second sentence mark in the tail changes the last-break partition.
            if tail == "WHY?!" {
                assert_eq!(tail_question(&q), None);
            } else {
                assert_eq!(tail_question(&q), Some(("Élan walks home.", expected)));
            }
        }
        assert_eq!(
            tail_question("Élan walks home. WHY?"),
            Some(("Élan walks home.", Tail::Cause))
        );
        for q in [
            "",
            "?",
            "Go now. Why?",
            "Élan walks home. Where?",
            "Élan walks home Why?",
        ] {
            assert_eq!(tail_question(q), None, "{q}");
        }
    }

    #[test]
    fn preparse_publishes_only_complete_sorted_missing_batches() {
        let st = PragStudent::default();
        let texts = vec![
            ("Zebra sleeps.".into(), false),
            ("Alice walks.".into(), false),
            ("Zebra sleeps.".into(), false),
        ];
        st.trees
            .borrow_mut()
            .insert("Already cached.".into(), vec![]);
        let before = st.trees.borrow().clone();
        assert!(
            st.preparse_with(&texts, |_| Err(anyhow::anyhow!("synthetic parser failure")))
                .is_err()
        );
        assert_eq!(
            st.trees.borrow().keys().collect::<Vec<_>>(),
            before.keys().collect::<Vec<_>>()
        );
        assert!(st.preparse_with(&texts, |_| Ok(vec![])).is_err());
        assert_eq!(
            st.trees.borrow().keys().collect::<Vec<_>>(),
            before.keys().collect::<Vec<_>>()
        );
        st.preparse_with(&texts, |units| {
            assert_eq!(units, &["Alice walks.", "Zebra sleeps."]);
            Ok(vec![vec![]; units.len()])
        })
        .unwrap();
        assert_eq!(
            st.trees
                .borrow()
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            vec!["Alice walks.", "Already cached.", "Zebra sleeps."]
        );
        st.preparse_with(&texts, |_| panic!("warm cache must not call parser"))
            .unwrap();
    }
}
