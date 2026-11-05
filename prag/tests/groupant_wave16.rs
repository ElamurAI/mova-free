//! Supplied scorer and annotation-gate joins; no corpus or teacher calls.
use prag::{
    data::{Item, Src},
    eval, fbench,
    frames::{Ent, Frame, How, Kind, Tam},
    schema::{self, Act, Field},
    silver,
};
fn ent(id: &str) -> Ent {
    Ent {
        id: id.into(),
        text: id.into(),
        ..Default::default()
    }
}
fn frame(kind: Kind, verb: &str) -> Frame {
    Frame {
        kind,
        agent: None,
        verb: verb.into(),
        neg: false,
        patient: None,
        recipient: None,
        place: None,
        attrs: vec![],
        manner: vec![],
        obls: vec![],
        quote: vec![],
        quoted: false,
        cause: None,
        next: None,
        cond: false,
        irrealis: false,
        src: (0, 0),
        how: How::default(),
        unsaid: vec![],
        words: vec![],
        nps: vec![],
        mark: vec![],
        cc: None,
        link: None,
        tam: Tam::default(),
    }
}
#[test]
fn notation_alignment_preserves_permuted_links_and_occurrence_bounds() {
    let mut speech = frame(Kind::Say(Act::Assert), "say");
    speech.agent = Some(ent("ada"));
    speech.quote = vec![1];
    speech.next = Some(1);
    let mut event = frame(Kind::Event, "carry");
    event.agent = Some(ent("bob"));
    event.patient = Some(ent("box"));
    event.cause = Some(0);
    let gold: Vec<_> = [speech.clone(), event.clone()]
        .iter()
        .map(|f| fbench::parse_frame(&f.show(0)).unwrap())
        .collect();
    event.cause = Some(1);
    speech.quote = vec![0];
    speech.next = Some(0);
    let sys: Vec<_> = [event, speech]
        .iter()
        .map(|f| fbench::parse_frame(&f.show(0)).unwrap())
        .collect();
    assert_eq!(
        fbench::align(&gold, &sys).into_iter().collect::<Vec<_>>(),
        [(0, 1), (1, 0)]
    );
    let counts = fbench::score_line(&gold, &sys);
    for r in [6, 7, 8] {
        assert_eq!(
            counts[r],
            fbench::Prf {
                tp: 1,
                sys: 1,
                gold: 1
            }
        );
    }
    assert_eq!(
        fbench::overall(&counts),
        fbench::Prf {
            tp: 9,
            sys: 9,
            gold: 9
        }
    );
    let mut duplicated = sys.clone();
    duplicated[1].says.push(0);
    let counts = fbench::score_line(&gold, &duplicated);
    assert_eq!(
        counts[6],
        fbench::Prf {
            tp: 1,
            sys: 2,
            gold: 1
        }
    );
    assert_eq!(counts[6].p(), 0.5);
    assert_eq!(counts[6].r(), 1.0);
    for c in counts {
        assert!(c.tp <= c.sys && c.tp <= c.gold);
    }
}
fn item(key: &str, src: Src, para: i64, batch: usize) -> Item {
    Item {
        key: key.into(),
        src,
        doc: "shared".into(),
        title: "fixture".into(),
        para,
        ord: 1,
        pos: 0,
        plen: 1,
        text: format!("sentence {key}"),
        batch,
        ctx: vec![],
    }
}
#[test]
fn numbered_silver_rows_keep_item_and_closed_class_identity() {
    let items = [
        item("tale", Src::Tale, 1, 4),
        item("standalone", Src::Tatoeba, 0, 4),
    ];
    let refs: Vec<_> = items.iter().collect();
    let answer =
        "2\tdecl\tassert\t0\t0\tnone\tnone\t-\n1\tdecl\tnarrate\t0\t0\tnone\tnarrator\t-\n";
    let gated = silver::gate(answer, &refs);
    assert!(gated.bad.is_empty());
    assert_eq!(gated.junk, 0);
    assert_eq!(gated.ok.keys().copied().collect::<Vec<_>>(), [1, 2]);
    let expected = [Act::Narrate, Act::Assert];
    let mut actual = vec![];
    for (i, act) in expected.into_iter().enumerate() {
        let labels = &gated.ok[&(i + 1)];
        assert_eq!(labels.act, act);
        let encoded = schema::labels_tsv(labels);
        let decoded = schema::parse_labels(&encoded.split('\t').collect::<Vec<_>>()).unwrap();
        assert_eq!(decoded.act, act);
        actual.push(decoded.act.idx());
    }
    assert_eq!(
        eval::scores(
            &[Act::Narrate.idx(), Act::Assert.idx()],
            &actual,
            Field::Act.classes().len()
        ),
        (1.0, 1.0)
    );
    let missing = silver::gate(answer.lines().nth(1).unwrap(), &refs);
    assert_eq!(missing.ok.keys().copied().collect::<Vec<_>>(), [1]);
    assert_eq!(missing.bad.len(), 1);
    assert_eq!(missing.bad[0].0, Some(2));
    let duplicate = silver::gate(
        &format!("{answer}{}\n", answer.lines().next().unwrap()),
        &refs,
    );
    assert!(duplicate.ok.is_empty());
    assert_eq!(duplicate.bad.len(), 1);
    assert_eq!(duplicate.bad[0].0, None);
}
#[test]
fn batching_and_prompt_context_share_source_safe_paragraph_identity() {
    let a = item("a", Src::Tale, 7, 9);
    let mut b = item("b", Src::Gum, 8, 9);
    b.ctx = vec!["retained context".into()];
    let items = [item("outside", Src::Tatoeba, 0, 3), a.clone(), b.clone()];
    let batches = silver::batches(&items);
    assert_eq!(batches.keys().copied().collect::<Vec<_>>(), [3, 9]);
    assert_eq!(
        batches[&9]
            .iter()
            .map(|x| x.key.as_str())
            .collect::<Vec<_>>(),
        ["a", "b"]
    );
    let prompt = silver::prompt(&batches[&9]);
    assert!(prompt.contains("context (previous sentences, do not label): retained context"));
    assert!(prompt.find("1\tsentence a").unwrap() < prompt.find("2\tsentence b").unwrap());
    b.src = Src::Tale;
    assert!(!silver::prompt(&[&a, &b]).contains("retained context"));
    let high = item("high", Src::Tale, i64::MAX, 0);
    let mut low = item("low", Src::Tale, i64::MIN, 0);
    low.ctx = vec!["boundary context".into()];
    assert!(silver::prompt(&[&high, &low]).contains("boundary context"));
}
