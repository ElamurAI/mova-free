//! Four finite interaction witnesses over supplied synthetic data.
//! The TSV is supplied independently: prompt generation does not execute a teacher.
//! These tests exercise pure joins, not acquisition or a single live pipeline.

use std::collections::{BTreeMap, HashSet};

use qagen::gate::{self, Level};
use qagen::prompt;
use qagen::schema::{ExIm, QType, Rule};
use qagen::select::{self, Para, Pick, Sent};

fn para(book: &str, doc: &str, n: i64, title: &str, texts: &[(&str, &str)]) -> Para {
    Para {
        doc: doc.into(),
        book: book.into(),
        title: title.into(),
        author: String::new(),
        para: n,
        block: "p".into(),
        sents: texts
            .iter()
            .map(|&(id, text)| Sent {
                sent_id: id.into(),
                text: text.into(),
            })
            .collect(),
        dialogue: false,
        words: texts
            .iter()
            .map(|(_, t)| t.split_whitespace().count())
            .sum(),
        ctx: Vec::new(),
        batch: 99,
    }
}

fn pick(n: usize, rule: u8) -> Pick {
    Pick {
        paras: n,
        min_words: 1,
        max_words: 100,
        rule,
        ..Pick::default()
    }
}

fn body(text: &str) -> &str {
    text.split_once("\nParagraphs:\n")
        .expect("prompt task delimiter")
        .1
}

fn pair_reasons(g: &gate::Gated) -> Vec<&str> {
    g.rej
        .iter()
        .filter(|r| r.level == Level::Pair)
        .map(|r| r.reason.as_str())
        .collect()
}

#[test]
fn selected_identity_exclusions_join_prompt_and_gate_coordinates() {
    let mut heading = para(
        "b:1",
        "b:1:tale",
        18,
        "Heading",
        &[
            ("heading:801", "Forbidden heading."),
            ("heading:802", "Another heading."),
        ],
    );
    heading.block = "head".into();
    let all = vec![
        para(
            "b:1",
            "b:1:tale",
            16,
            "Excluded",
            &[
                ("excluded:601", "Excluded payload."),
                ("excluded:602", "Never selected."),
            ],
        ),
        para(
            "b:1",
            "b:1:tale",
            17,
            "Bell",
            &[
                ("source:701", "A bell rang."),
                ("source:702", "Mira opened the gate."),
            ],
        ),
        heading,
        para(
            "b:1",
            "b:1:tale",
            19,
            "Key",
            &[
                ("source:901", "Oren held a silver key."),
                ("source:902", "Rain stopped."),
            ],
        ),
    ];
    let skip = HashSet::from(["b:1:tale#16".to_string()]);
    let config = pick(2, 2);
    let selected = select::select(&all, &config, &skip);
    assert_eq!(selected, [1, 3], "W17-J1-SELECTION");
    let batch: Vec<&Para> = selected.iter().map(|&i| &all[i]).collect();
    assert_eq!(
        batch.iter().map(|p| p.key()).collect::<Vec<_>>(),
        ["b:1:tale#17", "b:1:tale#19"]
    );
    let rendered = prompt::prompt(&batch, config.rule().unwrap());
    assert_eq!(
        body(&rendered),
        "\n=== P1 - \"Bell\", paragraph 17\n[1] A bell rang.\n[2] Mira opened the gate.\n\n=== P2 - \"Key\", paragraph 19\n[1] Oren held a silver key.\n[2] Rain stopped.\n",
        "W17-J1-PROMPT-COORDINATES"
    );
    let g = gate::gate(
        "1\taction\texplicit\t2\tWhat did Mira do?\topened the gate\n2\tcharacter\texplicit\t1\tWho held a key?\tOren",
        &batch,
        config.rule().unwrap(),
    );
    assert_eq!(g.format, None);
    assert_eq!(pair_reasons(&g), Vec::<&str>::new());
    assert_eq!(
        g.ok.iter()
            .map(|p| (p.p, p.k, p.anchors.clone()))
            .collect::<Vec<_>>(),
        [(1, 1, vec![2]), (2, 1, vec![1])],
        "W17-J1-GATE-COORDINATES"
    );
    let identities: Vec<(&str, i64, &str)> =
        g.ok.iter()
            .map(|p| {
                let source = batch[p.p - 1];
                (
                    source.doc.as_str(),
                    source.para,
                    source.sents[p.anchors[0] - 1].sent_id.as_str(),
                )
            })
            .collect();
    assert_eq!(
        identities,
        [
            ("b:1:tale", 17, "source:702"),
            ("b:1:tale", 19, "source:901")
        ],
        "W17-J1-PROVENANCE"
    );
}

#[test]
fn closed_schema_and_rule_join_prompt_and_span_gate() {
    let p = para(
        "b:2",
        "b:2:tale",
        41,
        "Garden",
        &[
            ("source:80", "Red, blue flowers bloomed."),
            ("source:81", "Mira waited."),
        ],
    );
    let names = [
        "character",
        "setting",
        "action",
        "feeling",
        "causal relationship",
        "outcome resolution",
        "prediction",
    ];
    let response = "1\tsetting\texplicit\t1\tWhat bloomed?\tRed, blue flowers\n\
1\taction\texplicit\t1\tWhich colors appeared?\tred blue flowers\n\
1\tcausal\texplicit\t1\tWhy did they bloom?\tRed, blue flowers\n\
1\tfeeling\tinferred\t2\tHow did Mira feel?\tcalm";
    for version in [1, 2] {
        let config = pick(1, version);
        let rule = config.rule().unwrap();
        let rendered = prompt::prompt(&[&p], rule);
        let types: Vec<&str> = prompt::rules(rule)
            .split_once("type - exactly one of these seven names:\n")
            .unwrap()
            .1
            .split_once("ex - explicit or implicit:\n")
            .unwrap()
            .0
            .lines()
            .map(|line| line.trim().split_once(" - ").unwrap().0)
            .collect();
        assert_eq!(types, names, "W17-J2-PROMPT-CLOSED-TYPES");
        assert_eq!(QType::ALL.map(QType::name), names);
        assert_eq!(ExIm::ALL.map(ExIm::name), ["explicit", "implicit"]);
        assert_eq!(
            rendered.contains("VERBATIM SPAN"),
            version == 2,
            "W17-J2-RULE-PROMPT"
        );
        assert!(rendered.contains("ex - explicit or implicit:"));
        let g = gate::gate(response, &[&p], rule);
        assert_eq!(g.format, None);
        if version == 1 {
            assert_eq!(
                g.ok.iter().map(|p| p.answer.as_str()).collect::<Vec<_>>(),
                ["Red, blue flowers", "red blue flowers"]
            );
            assert_eq!(
                pair_reasons(&g),
                [
                    "type «causal» not in the set",
                    "explicitness «inferred» not in the set"
                ]
            );
        } else {
            assert_eq!(
                g.ok.iter().map(|p| p.answer.as_str()).collect::<Vec<_>>(),
                ["Red, blue flowers"]
            );
            assert_eq!(
                pair_reasons(&g),
                [
                    "explicit: consecutive words in sentence 1, but different punctuation",
                    "type «causal» not in the set",
                    "explicitness «inferred» not in the set",
                ],
                "W17-J2-RULE-GATE"
            );
        }
    }
    assert_eq!(
        body(&prompt::prompt(&[&p], Rule::V1)),
        body(&prompt::prompt(&[&p], Rule::V2))
    );
}

#[test]
fn sentence_provenance_survives_normalization_ambiguity() {
    let mut a = para(
        "b:3",
        "b:3:first",
        50,
        "First",
        &[
            ("original:120", "“KING’S   bell rang.”"),
            ("original:121", "Mira slept."),
        ],
    );
    a.ctx = vec!["Hidden treasure sparkled.".into()];
    let mut b = para(
        "b:3",
        "b:3:second",
        90,
        "Second",
        &[
            ("alternate:800", "king's bell rang."),
            ("alternate:801", "Mira slept."),
        ],
    );
    b.ctx = a.ctx.clone();
    let batch = [&a, &b];
    let rendered = prompt::prompt(&batch, Rule::V2);
    assert!(body(&rendered).contains("[1] “KING’S   bell rang.”\n"));
    assert!(body(&rendered).contains("[1] king's bell rang.\n"));
    assert_eq!(
        body(&rendered)
            .matches("context (before this paragraph")
            .count(),
        2
    );
    assert_eq!(
        gate::Sents::of(&a).norm,
        ["king's bell rang.", "mira slept."]
    );
    assert_eq!(
        gate::Sents::of(&b).norm,
        ["king's bell rang.", "mira slept."]
    );
    let response = "1\taction\texplicit\t2\tWhat rang at the wrong anchor?\tking's bell\n\
1\taction\texplicit\t1\tWhat rang?\tKING’S bell\n\
1\tsetting\texplicit\t1\tWhat sparkled before?\tHidden treasure\n\
2\taction\texplicit\t2\tWhat rang at the wrong anchor?\tking's bell\n\
2\taction\texplicit\t1\tWhat rang?\tKING’S bell\n\
2\tsetting\texplicit\t1\tWhat sparkled before?\tHidden treasure";
    let g = gate::gate(response, &batch, Rule::V2);
    assert_eq!(g.format, None);
    assert_eq!(
        pair_reasons(&g),
        [
            "explicit: span of sentence 1, which is not an anchor",
            "explicit: not a span of an anchor sentence",
            "explicit: span of sentence 1, which is not an anchor",
            "explicit: not a span of an anchor sentence",
        ],
        "W17-J3-SOURCE-SCOPE"
    );
    assert_eq!(
        g.ok.iter()
            .map(|p| (p.p, p.k, p.anchors.clone(), p.answer.as_str()))
            .collect::<Vec<_>>(),
        [
            (1, 1, vec![1], "KING’S bell"),
            (2, 1, vec![1], "KING’S bell")
        ],
        "W17-J3-AMBIGUOUS-OBSERVATION"
    );
    let sources: Vec<&str> =
        g.ok.iter()
            .map(|p| batch[p.p - 1].sents[p.anchors[0] - 1].sent_id.as_str())
            .collect();
    assert_eq!(
        sources,
        ["original:120", "alternate:800"],
        "W17-J3-PROVENANCE"
    );
}

#[test]
fn quota_caps_join_book_selection_and_prompt_cardinality() {
    let mut all = Vec::new();
    for n in 1..=5 {
        let mut p = para(
            "book:a",
            "book:a:tale",
            n,
            "A",
            &[("a:1", "A bird sang."), ("a:2", "The sky cleared.")],
        );
        if n > 1 {
            p.block = "head".into();
        }
        all.push(p);
    }
    for n in 1..=3 {
        let mut p = para(
            "book:b",
            "book:b:tale",
            n,
            "B",
            &[("b:1", "A bell rang."), ("b:2", "The sun rose.")],
        );
        p.dialogue = n == 1;
        all.push(p);
    }
    let config = pick(4, 2);
    let skip = HashSet::from(["book:b:tale#3".to_string()]);
    let size = BTreeMap::from([("book:a".to_string(), 10), ("book:b".to_string(), 6)]);
    let cap = BTreeMap::from([("book:a".to_string(), 1), ("book:b".to_string(), 2)]);
    let expected = BTreeMap::from([("book:a".to_string(), 1), ("book:b".to_string(), 2)]);
    let q = select::quotas(&size, &cap, config.paras);
    assert_eq!(q, expected, "W17-J4-CAPPED-QUOTAS");
    let selected = select::select(&all, &config, &skip);
    assert_eq!(selected, [0, 5, 6], "W17-J4-CAPPED-SELECTION");
    let batch: Vec<&Para> = selected.iter().map(|&i| &all[i]).collect();
    let mut observed = BTreeMap::new();
    for p in &batch {
        *observed.entry(p.book.clone()).or_insert(0) += 1;
    }
    assert_eq!(observed, expected, "W17-J4-BOOK-COUNTS");
    assert_eq!(observed, q, "W17-J4-QUOTA-SELECTION-JOIN");
    let rendered = prompt::prompt(&batch, config.rule().unwrap());
    let headings: Vec<&str> = body(&rendered)
        .lines()
        .filter(|line| line.starts_with("=== P"))
        .collect();
    assert_eq!(
        headings,
        [
            "=== P1 - \"A\", paragraph 1",
            "=== P2 - \"B\", paragraph 1",
            "=== P3 - \"B\", paragraph 2",
        ],
        "W17-J4-PROMPT-CARDINALITY"
    );
}
