//! Bounded supplied-input joins; no live parser, corpus, teacher or held-out evaluation.
use prag::{
    agree,
    data::{Item, Src},
    eval::{self, Set},
    feats::{self, Dict, Row, SLOTS, UNSEEN},
    model::{self, Classifier, Kind, Pred, Train},
    schema::{self, Act, Field, Labels, Voice},
};
fn row(first: &str) -> Row {
    let mut r = vec!["-".to_string(); SLOTS.len()];
    r[0] = first.into();
    r[feats::prev_slot()] = "start".into();
    r
}
fn item(src: Src, doc: &str, ord: i64) -> Item {
    Item {
        key: format!("{}:{doc}:{ord}", src.name()),
        src,
        doc: doc.into(),
        title: String::new(),
        para: 1,
        ord,
        pos: 0,
        plen: 1,
        text: String::new(),
        batch: 0,
        ctx: vec![],
    }
}

#[test]
fn actual_models_transport_six_field_classes_through_schema() {
    let labels =
        schema::parse_labels(&["q", "request", "1", "0", "neg", "character", "-"]).unwrap();
    let expected = [1, 3, 1, 0, 1, 1];
    assert_eq!(Field::ALL.map(|f| f.get(&labels)), expected);
    let rows = vec![row("can"); 4];
    let dict = Dict::build(&rows);
    let x: Vec<_> = rows.iter().map(|r| dict.encode(r)).collect();
    let ys: Vec<_> = expected.into_iter().map(|c| vec![c; rows.len()]).collect();
    let keys: Vec<_> = (0..rows.len()).map(|i| format!("fixture-{i}")).collect();
    for kind in Kind::ALL {
        let set = eval::fit_set(kind, &x, &ys, &keys, &dict, 17);
        let out = eval::predict_seq(&set, &rows, &[None; 4], &[0, 1, 2, 3], &dict);
        for i in 0..rows.len() {
            assert_eq!(out[&i].class, expected, "{kind:?}, row {i}");
            assert_eq!(out[&i].preds.len(), 6);
            let mut reconstructed = Labels::default();
            for (f, c) in Field::ALL.into_iter().zip(out[&i].class) {
                f.set(&mut reconstructed, c);
            }
            assert_eq!(reconstructed, labels);
            let text = schema::labels_tsv(&reconstructed);
            assert_eq!(
                schema::parse_labels(&text.split('\t').collect::<Vec<_>>()).unwrap(),
                labels
            );
        }
    }
}
#[test]
fn dictionary_reindex_preserves_encoded_rows_and_real_predictions() {
    let rows = vec![row("left"), row("right"), row("left"), row("right")];
    let dict = Dict::build(&rows);
    let text = serde_json::to_string(&dict).unwrap();
    let mut restored: Dict = serde_json::from_str(&text).unwrap();
    restored.reindex();
    for act in Act::ALL {
        assert_ne!(restored.id(feats::prev_slot(), act.name()), UNSEEN);
    }
    let probe = row("never-observed");
    let p = restored.encode(&probe);
    assert_eq!(p[0], UNSEEN);
    assert_eq!(dict.encode(&probe), p);
    let x: Vec<_> = rows.iter().map(|r| dict.encode(r)).collect();
    let rx: Vec<_> = rows.iter().map(|r| restored.encode(r)).collect();
    assert_eq!(x, rx);
    let y = [0, 1, 0, 1];
    let names = ["left", "right"];
    let keys: Vec<_> = (0..4).map(|i| format!("row-{i}")).collect();
    for kind in Kind::ALL {
        let a = model::fit(
            kind,
            &Train {
                x: &x,
                y: &y,
                names: &names,
                keys: &keys,
                dict: &dict,
                seed: 31,
            },
        );
        let b = model::fit(
            kind,
            &Train {
                x: &rx,
                y: &y,
                names: &names,
                keys: &keys,
                dict: &restored,
                seed: 31,
            },
        );
        for r in x.iter().chain(std::iter::once(&p)) {
            let (a, b) = (a.predict(r), b.predict(r));
            assert_eq!(
                (a.class, a.conf.to_bits(), a.why),
                (b.class, b.conf.to_bits(), b.why),
                "{kind:?}"
            );
        }
    }
}
// This observer isolates eval's real context adapter; it is not a trained model.
struct Observer {
    act: bool,
    start: u32,
    ask: u32,
    request: u32,
}
impl Classifier for Observer {
    fn kind(&self) -> Kind {
        Kind::IgTree
    }
    fn predict(&self, x: &[u32]) -> Pred {
        let p = x[feats::prev_slot()];
        let class = if !self.act {
            0
        } else if p == self.start {
            Act::Ask.idx()
        } else if p == self.ask {
            Act::Request.idx()
        } else if p == self.request {
            Act::Thank.idx()
        } else {
            Act::Other.idx()
        };
        Pred {
            class,
            conf: 1.0,
            why: vec![(format!("observed-prev={p}"), 1.0)],
        }
    }
    fn bytes(&self) -> usize {
        0
    }
    fn info(&self) -> String {
        "test context observer".into()
    }
}
fn observers(dict: &Dict) -> Set {
    Set {
        kind: Kind::IgTree,
        train_secs: 0.0,
        models: (0..6)
            .map(|i| {
                Box::new(Observer {
                    act: i == 1,
                    start: dict.id(feats::prev_slot(), "start"),
                    ask: dict.id(feats::prev_slot(), "ask"),
                    request: dict.id(feats::prev_slot(), "request"),
                }) as Box<dyn Classifier>
            })
            .collect(),
    }
}
#[test]
fn predecessor_identity_is_preserved_through_prediction_context() {
    let items = [
        item(Src::Tale, "same", 1),
        item(Src::Tale, "same", 2),
        item(Src::Gum, "same", 1),
        item(Src::Gum, "same", 2),
        item(Src::Tatoeba, "same", 1),
        item(Src::Tatoeba, "same", 2),
        item(Src::Tale, "other", 2),
    ];
    let prev = feats::prev_of(&items);
    assert_eq!(prev, [None, Some(0), None, Some(2), None, None, None]);
    let mut rows = vec![row("fixture"); items.len()];
    for r in &mut rows {
        r[feats::prev_slot()] = "warn".into();
    }
    let dict = Dict::build(&rows);
    let out = eval::predict_seq(
        &observers(&dict),
        &rows,
        &prev,
        &[0, 1, 2, 3, 4, 5, 6],
        &dict,
    );
    let acts: Vec<_> = (0..items.len()).map(|i| out[&i].class[1]).collect();
    assert_eq!(acts, [2, 3, 2, 3, 2, 2, 2]);
    assert!(rows.iter().all(|r| r[feats::prev_slot()] == "warn"));
}
#[test]
fn predecessor_schedule_and_subset_have_explicit_start_fallback() {
    let items = [
        item(Src::Tale, "chain", 1),
        item(Src::Tale, "chain", 2),
        item(Src::Tale, "chain", 3),
    ];
    let prev = feats::prev_of(&items);
    let rows = vec![row("fixture"); 3];
    let dict = Dict::build(&rows);
    let set = observers(&dict);
    let normal = eval::predict_seq(&set, &rows, &prev, &[0, 1, 2], &dict);
    assert_eq!(
        (
            normal[&0].class[1],
            normal[&1].class[1],
            normal[&2].class[1]
        ),
        (2, 3, 7)
    );
    let reverse = eval::predict_seq(&set, &rows, &prev, &[2, 1, 0], &dict);
    assert!((0..3).all(|i| reverse[&i].class[1] == 2));
    let subset = eval::predict_seq(&set, &rows, &prev, &[1, 2], &dict);
    assert_eq!(
        (subset.len(), subset[&1].class[1], subset[&2].class[1]),
        (2, 2, 3)
    );
    assert!(!subset.contains_key(&0));
}
#[test]
fn schema_fields_reach_agreement_with_character_voice_preserved() {
    let mut labels = Labels::default();
    let ordinary = schema::labels_tsv(&labels);
    labels.voice = Voice::Character;
    let character = schema::labels_tsv(&labels);
    let text = format!(
        "sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tmeans\nplain\t{ordinary}\nspeech\t{character}\n"
    );
    let trees = "# sent_id = plain\n1\tGo\tgo\tVERB\tVB\t_\t0\troot\t_\t_\n\n# sent_id = speech\n1\tGo\tgo\tVERB\tVB\t_\t0\troot\t_\t_\n";
    let (agreed, _) = agree::run(&text, "sent_id\n", trees);
    let table = agree::table(&agreed);
    assert_eq!(table["plain"]["form"], "imp");
    assert_eq!(table["plain"]["act"], "request");
    assert_eq!(table["speech"]["form"], "decl");
    assert_eq!(table["speech"]["act"], "assert");
    assert_eq!(table["speech"]["voice"], "character");
    assert_eq!(table["plain"]["voice"], "none");
    assert!(!table["speech"].contains_key("means"));
    for id in ["plain", "speech"] {
        let r = &table[id];
        let cols = [
            r["form"].as_str(),
            r["act"].as_str(),
            r["indirect"].as_str(),
            r["hedge"].as_str(),
            r["polarity"].as_str(),
            r["voice"].as_str(),
            "-",
        ];
        assert!(schema::parse_labels(&cols).is_ok());
    }
}
#[test]
fn normalized_cue_values_retain_slot_local_identity() {
    let words: Vec<_> = ["I", "think", "thank", "you", "let", "us", "go"]
        .into_iter()
        .map(feats::norm_word)
        .collect();
    let cues = feats::Cues::load();
    assert_eq!(
        cues.find(&words),
        [(0, "hedge"), (2, "thank"), (4, "suggest")]
    );
    let mut r = row(&words[0]);
    r[feats::slot("cue")] = cues.find(&words)[1].1.into();
    r[feats::slot("hedge")] = words[0].clone();
    let d = Dict::build(&[r.clone()]);
    let encoded = d.encode(&r);
    assert_eq!(
        d.show(feats::slot("cue"), encoded[feats::slot("cue")]),
        "cue=thank"
    );
    assert_eq!(
        d.show(feats::slot("hedge"), encoded[feats::slot("hedge")]),
        "hedge=i"
    );
    assert_eq!(d.id(feats::slot("cue"), "i"), UNSEEN);
}
#[test]
fn score_observation_preserves_denominator_and_present_class_union() {
    let gold = [0, 0, 1, 2];
    let (acc, f1) = eval::scores(&gold, &[0, 1, 1, 2], 4);
    assert_eq!(acc, 0.75);
    assert!((f1 - 7.0 / 9.0).abs() < 1e-12);
    let (acc, f1) = eval::scores(&gold, &[3, 1, 1, 2], 4);
    assert_eq!(acc, 0.5);
    assert!((f1 - 5.0 / 12.0).abs() < 1e-12);
    assert_eq!(eval::scores(&[], &[], 4), (0.0, 0.0));
}
