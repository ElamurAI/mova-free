//! The gap maker on real data, with Mova's annotator (set MOVA_OCTOPUS_TEST_MODEL to an en.bin; without it these
//! tests report a skip) and Open English WordNet under `$MOVA_DATA/raw/en-oewn/json`: the red controls from the
//! command words (only/just, mum/parent, now/out/just, go/say must never be wrong options of each other), and the
//! commands dictionary layer of the repository made into lessons that keep the school's rule.
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use turtle::fit::{Lex, slot_of};

fn model() -> Option<PathBuf> {
    match std::env::var_os("MOVA_OCTOPUS_TEST_MODEL") {
        Some(p) => Some(PathBuf::from(p)),
        None => {
            eprintln!("SKIPPED: MOVA_OCTOPUS_TEST_MODEL not configured");
            None
        }
    }
}

/// The commands dictionary of the school data (outside the repository; `mova fetch 0.5 school`).
fn layer() -> PathBuf {
    world::knowledge::dictionaries_dir().join("commands.tsv")
}

fn rows() -> Vec<nautilus::layer::Row> {
    nautilus::layer::parse_rows(&std::fs::read_to_string(layer()).expect("the school data (commands.tsv): mova fetch 0.5 school, or set MOVA_SCHOOL_DATA"))
}

/// The lexicons of the layer's words and of the red controls' words (the layer changes; the controls stay).
fn lex(rows: &[nautilus::layer::Row]) -> Lex {
    let mut words: BTreeSet<String> = rows.iter().map(|r| r.word.to_lowercase()).collect();
    for (_, answer, never) in FITTED {
        words.insert(answer.to_string());
        words.extend(never.iter().map(|w| w.to_string()));
    }
    let meanings: BTreeMap<String, String> = rows.iter().map(|r| (r.word.to_lowercase(), r.meaning.clone())).collect();
    Lex::load(&words, &meanings).unwrap()
}

fn words(s: &str) -> BTreeSet<String> {
    s.split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '-').filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
}

/// The questions of 2026-11-04 whose wrong options also fitted: (sentence, answer, the candidates that must never be
/// shown in their base form).
const FITTED: [(&str, &str, &[&str]); 6] = [
    ("Real people also chop commands down to a word or two. People who take only the exact words, like a robot, get it wrong.", "only", &["just"]),
    ("Leo just points and says \"it's there\". What happens next?", "just", &["only"]),
    ("Mum at the front door with her coat on asks \"did you feed the cat?\"", "mum", &["parent", "dad"]),
    ("\"I'd like you to…\" from a teacher or parent is an order said kindly, not a wish you may ignore.", "parent", &["mum", "dad"]),
    ("The door opened for her now", "now", &["out", "just", "still"]),
    ("If you are holding something hot or sharp, you make it safe or pause first, then go.", "go", &["say", "stay", "stop"]),
];

#[test]
fn red_controls_from_the_command_words() {
    let Some(model) = model() else { return };
    let a = en::annotate::Annotator::from_bytes(&std::fs::read(&model).unwrap()).unwrap();
    let rows = rows();
    let l = lex(&rows);
    // the near-synonyms, by WordNet or the meanings, in either direction
    for (x, y) in [("only", "just"), ("mum", "parent")] {
        assert!(l.kin(x, y).is_some() && l.kin(y, x).is_some(), "{x}/{y} must be near-synonyms");
    }
    // negative control: an unrelated pair is not
    assert!(l.kin("mum", "robot").is_none());
    for (s, answer, never) in FITTED {
        let slot = slot_of(&a, s, answer).unwrap_or_else(|| panic!("no slot for {answer} in {s}"));
        for c in never {
            match l.judge(answer, c, slot, &words(s)) {
                Ok((form, why)) => assert_ne!(form, *c, "«{c}» fits the gap of «{answer}» ({why}): {s}"),
                Err(_) => {}
            }
        }
        // the gap still gets three wrong options from the layer, none of them a refused base form
        let class = rows.iter().find(|r| r.word == answer).map(turtle::make::pos_class).unwrap_or_default();
        let (wrong, _) = turtle::make::wrong_options(&rows, answer, &class, s, slot, &l, &BTreeMap::new());
        assert_eq!(wrong.len(), 3, "{answer}: {wrong:?}");
        assert!(wrong.iter().all(|w| !never.contains(&w.as_str())), "{answer}: {wrong:?}");
    }
}

#[test]
fn the_commands_layer_makes_lessons_that_keep_the_rule() {
    let Some(model) = model() else { return };
    let a = en::annotate::Annotator::from_bytes(&std::fs::read(&model).unwrap()).unwrap();
    let rows = rows();
    let l = lex(&rows);
    let slot = |s: &str, w: &str| slot_of(&a, s, w);
    let m = turtle::make::gap_lessons(&rows, "commands-words", "the command course", &turtle::make::Fit { lex: &l, slot: &slot });
    assert!(!m.lessons.is_empty() && m.questions > 100, "{} lessons, {} questions", m.lessons.len(), m.questions);
    for (f, t) in &m.lessons {
        let r = turtle::check::lesson(f, t);
        assert!(r.problems.is_empty(), "{:?}", r.problems);
        let lesson = world::knowledge::parse(t, f).unwrap();
        for (q, right, wrong) in lesson.exercises.iter().chain(&lesson.tests) {
            for (x, y) in [("only", "just"), ("just", "only"), ("mum", "parent"), ("parent", "mum"), ("mum", "dad"), ("go", "say"), ("now", "out"), ("now", "just")] {
                assert!(!(right == x && wrong.iter().any(|w| w == y)), "«{y}» is a wrong option of «{x}»: {q}");
            }
        }
    }
    // deterministic
    let again = turtle::make::gap_lessons(&rows, "commands-words", "the command course", &turtle::make::Fit { lex: &l, slot: &slot });
    assert_eq!(again, m);
}
