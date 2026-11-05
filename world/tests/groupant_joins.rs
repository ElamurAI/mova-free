//! Actual API joins on supplied inputs; no parser or teacher acquisition.
use en::annotate::Word;
use math::big::q;
use std::collections::{BTreeMap, BTreeSet};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
use world::{
    mentions::Mention,
    opchoice::{self, Op},
    qframe::{Clause, Focus, QFrame, QKind, Wh},
    tree::{PTag, Rel, Tree, UPos},
    types::{Field, Id, Reg},
};
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        static SERIAL: AtomicUsize = AtomicUsize::new(0);
        let p = std::env::temp_dir().join(format!(
            "mova-groupant-joins-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn mention(value: i64) -> Mention {
    Mention {
        value,
        thing: "apple".into(),
        adj: BTreeSet::new(),
        verb: "have".into(),
        ctx: BTreeSet::new(),
        sentence: 0,
        in_q: false,
        per: None,
        verb_word: "have".into(),
        thing_word: "apple".into(),
        holder_asked: None,
        tree: Vec::new(),
    }
}
fn expression(left: &str, op: Op, right: &str) -> String {
    let symbol = match op {
        Op::Add => '+',
        Op::Sub => '-',
        Op::Mul => '*',
        Op::Div => '/',
    };
    format!("({left} {symbol} {right})")
}
fn oracle(op: Op, x: i64, y: i64) -> Option<i64> {
    // Widened arithmetic is independent of the small-domain implementation.
    let (x, y) = (i128::from(x), i128::from(y));
    let v = match op {
        Op::Add => x + y,
        Op::Mul => x * y,
        Op::Sub if x >= y => x - y,
        Op::Div if y != 0 && x % y == 0 => x / y,
        _ => return None,
    };
    i64::try_from(v).ok()
}
#[test]
fn operation_candidates_join_exact_quantity_scripts() {
    let ops = [Op::Add, Op::Sub, Op::Mul, Op::Div];
    let mut evaluated = 0;
    for a in 0..=3 {
        for b in 0..=3 {
            for c in 0..=3 {
                let values = [a, b, c];
                let ms = values.iter().map(|v| mention(*v)).collect::<Vec<_>>();
                let candidates = opchoice::candidates(&ms);
                let mut expected = BTreeSet::new();
                for x in 0..3 {
                    for y in 0..3 {
                        if x == y {
                            continue;
                        }
                        for op in ops {
                            if matches!(op, Op::Add | Op::Mul) && x > y {
                                continue;
                            }
                            if let Some(v) = oracle(op, values[x], values[y]) {
                                expected.insert((x, op, y, None, v));
                                for z in 0..3 {
                                    if z == x || z == y {
                                        continue;
                                    }
                                    for op2 in ops {
                                        if let Some(v2) = oracle(op2, v, values[z]) {
                                            expected.insert((x, op, y, Some((op2, z)), v2));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                let actual = candidates
                    .iter()
                    .map(|c| (c.x, c.op, c.y, c.then, c.value))
                    .collect::<BTreeSet<_>>();
                assert_eq!(actual, expected);
                assert_eq!(actual.len(), candidates.len());
                let sents = vec![format!("There are {a} apples, {b} pears, and {c} plums.")];
                for candidate in candidates {
                    let mut expr = expression(
                        &values[candidate.x].to_string(),
                        candidate.op,
                        &values[candidate.y].to_string(),
                    );
                    if let Some((op, z)) = candidate.then {
                        expr = expression(&expr, op, &values[z].to_string());
                    }
                    let (cmds, errs) = world::quant::parse_script(&format!("@1 ask {expr}"));
                    assert!(errs.is_empty());
                    let run = world::quant::run(&cmds, &sents);
                    assert!(run.errors.is_empty(), "{expr}: {:?}", run.errors);
                    assert_eq!(run.answer, Some(q(candidate.value)), "{expr}");
                    assert!(run.state.is_empty());
                    evaluated += 1;
                }
            }
        }
    }
    assert!(evaluated > 1000);
}

#[test]
fn parsed_world_history_joins_markdown_observations() {
    let program = "loc L1 type=room name=Hall ^s1\nloc L2 type=room name=Garden here=0 ^s1\nchar C1 type=human name=Alice at=L1 ^s1\nmove C1 to=L2 ^s2\nmove C1 to=L9 ^s3";
    let (cmds, errs) = world::lang::parse_all(program);
    assert!(errs.is_empty(), "{errs:?}");
    let text = world::world::Text {
        doc: 0,
        title: "projection fixture".into(),
        sents: vec![
            (1, "Alice starts in the hall.".into()),
            (1, "Alice goes to the garden.".into()),
            (2, "The destination is absent.".into()),
        ],
    };
    let run = world::world::run(text, &cmds);
    assert_eq!((run.accepted, run.rejects.len()), (4, 1));
    let who = Id::new(Reg::C, 1);
    let e = run.world.ent(who).unwrap();
    assert_eq!(e.snap(1).id(Field::At), Some(Id::new(Reg::L, 1)));
    assert_eq!(e.now().id(Field::At), Some(Id::new(Reg::L, 2)));
    let old = world::md::snap_str(&run.world, who, 1);
    let now = world::md::snap_str(&run.world, who, e.latest());
    assert!(old.contains("Hall"), "{old}");
    assert!(!old.contains("Garden"), "{old}");
    assert!(now.contains("Garden"), "{now}");
    let rendered = world::md::world_md(&run, &errs, 5);
    assert!(rendered.contains("accepted 4, rejected by format 0, by gates 1"));
    for anchor in ["^s1", "^s2", "^s3"] {
        assert!(rendered.contains(anchor));
    }
    assert!(rendered.contains("projection fixture"));
    assert!(run.uncovered.is_empty());
}
fn frame(content: Vec<String>) -> QFrame {
    QFrame {
        q: "synthetic".into(),
        wh: Wh::What,
        kind: QKind::Generic,
        focus: Focus::None,
        main: Clause {
            content,
            ..Clause::default()
        },
        mark: None,
        sub: None,
        fut: false,
        passive: false,
        prep: None,
        about: Vec::new(),
        target: Vec::new(),
        whnp: Vec::new(),
    }
}
fn word(form: &str) -> Word {
    Word {
        form: form.into(),
        lemma: form.into(),
        upos: UPos::NOUN,
        tag: PTag::NN,
        head: 0,
        rel: Rel::Root,
        feats: Default::default(),
    }
}
#[test]
fn question_frame_reader_join_preserves_multiset_coverage() {
    let alphabet = ["zzqa", "zzqb", "zzqc"];
    for mask in 0..8 {
        let tree = Tree::of_words(
            (0..3)
                .filter(|i| mask & (1 << i) != 0)
                .map(|i| word(alphabet[i]))
                .collect(),
        );
        for code in 0..64 {
            let counts = [code % 4, code / 4 % 4, code / 16 % 4];
            let terms = (0..3)
                .flat_map(|i| std::iter::repeat_n(alphabet[i].to_owned(), counts[i]))
                .collect();
            let fr = frame(terms);
            let n: usize = counts.iter().sum();
            let hits: usize = (0..3)
                .filter(|i| mask & (1 << i) != 0)
                .map(|i| counts[i])
                .sum();
            let expected = if n == 0 { 0.0 } else { hits as f64 / n as f64 };
            assert_eq!(world::reader::cover(&fr, &tree), expected);
            let labels = (0..3)
                .filter(|i| counts[*i] > 0)
                .map(|i| alphabet[i].to_string())
                .collect::<BTreeSet<_>>();
            assert_eq!(
                world::reader::qlemmas(&fr)
                    .into_iter()
                    .collect::<BTreeSet<_>>(),
                labels
            );
        }
    }
}
#[test]
fn selected_book_body_is_the_copy_gate_input() {
    let story = "amber birch cedar dahlia elm fir ginkgo hazel iris juniper";
    let prefix = "one two three four five six seven eight nine";
    let suffix = "red blue green yellow orange purple white black brown";
    let raw = format!(
        "{prefix}\n*** START OF THE PROJECT GUTENBERG EBOOK FIXTURE ***\n{story}\n*** END OF THE PROJECT GUTENBERG EBOOK FIXTURE ***\n{suffix}"
    );
    let body = world::events::book_body(&raw);
    assert!(body.contains(story));
    assert!(!body.contains(prefix));
    assert!(!body.contains(suffix));
    let gate = world::bookans::Gate::new(body);
    for length in 0..=10 {
        let answer = story
            .split_whitespace()
            .take(length)
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(
            !gate.check(&answer).is_empty(),
            length >= world::bookans::COPY_RUN
        );
    }
    assert!(gate.check(prefix).is_empty());
    assert!(gate.check(suffix).is_empty());
    assert!(!gate.check("\"novel words outside corpus\"").is_empty());
}

#[test]
fn sense_animacy_joins_who_question_eligibility() {
    use world::qud::{Answer, Attach, Tok};
    for lemma in ["person", "dog", "stone", "zzunclassified"] {
        let question = vec![
            Tok::new("Who", "who", UPos::PRON, Rel::Nsubj, 2),
            Tok::new("runs", "run", UPos::VERB, Rel::Root, 0),
        ];
        let answer = vec![
            Tok::new(lemma, lemma, UPos::NOUN, Rel::Nsubj, 2),
            Tok::new("runs", "run", UPos::VERB, Rel::Root, 0),
        ];
        let steps = world::qud::track_toks(&[
            ("Who runs?".into(), question),
            (format!("{lemma} runs."), answer),
        ]);
        let admissible = matches!(world::sense::animacy(lemma), world::sense::Anim::Animate(_));
        assert_eq!(
            steps[1].attach == Attach::Explicit(1),
            admissible,
            "{lemma}"
        );
        assert_eq!(steps[1].stack.is_empty(), admissible, "{lemma}");
        if admissible {
            assert_eq!(steps[1].answer, Answer::Complete);
        }
    }
}
fn csv_field(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}
#[test]
fn tabular_and_fairytale_csv_agree_on_single_line_domain() {
    let scratch = Scratch::new();
    let path = scratch.0.join("fixture.csv");
    for a in ["", "plain", "a,b", "a\"b", "é"] {
        for b in ["", "plain", "a,b", "a\"b", "é"] {
            let serialized = format!("left,right\n{},{}\n", csv_field(a), csv_field(b));
            std::fs::write(&path, &serialized).unwrap();
            let table = world::tabular::read_csv(&path).unwrap();
            let full = world::ftqa::csv(&serialized).unwrap();
            assert_eq!(table.head, full[0]);
            assert_eq!(table.rows, full[1..]);
            assert_eq!(table.rows, vec![vec![a.to_string(), b.to_string()]]);
            let refs = table.rows.iter().collect::<Vec<_>>();
            let records = world::tabular::read_world(&table.head, &refs, &[]);
            let expected = BTreeMap::from([
                (
                    "left".to_string(),
                    if a.is_empty() { "unknown" } else { a }.to_string(),
                ),
                (
                    "right".to_string(),
                    if b.is_empty() { "unknown" } else { b }.to_string(),
                ),
            ]);
            assert_eq!(records.attrs, vec![expected]);
            assert_eq!(records.links, vec![Vec::<usize>::new()]);
        }
    }
}
