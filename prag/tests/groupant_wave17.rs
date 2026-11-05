//! Supplied event/accounting/label joins, without live teacher or corpus calls.
//! The client adapter below is explicit test wiring, not process/run evidence.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use prag::{
    data::{Item, Src},
    opus::{append_call, interpret_reply, read_calls},
    schema::Act,
    silver,
};
use serde_json::json;

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mova-wave17-prag-joins-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).expect("exclusive scratch directory");
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn item(key: &str, src: Src, para: i64) -> Item {
    Item {
        key: key.into(),
        src,
        doc: "synthetic-doc".into(),
        title: "supplied fixture".into(),
        para,
        ord: 700,
        pos: 0,
        plen: 1,
        text: format!("Synthetic sentence {key}."),
        batch: 9,
        ctx: vec![],
    }
}

fn events(rows: &str) -> String {
    format!(
        "{}\n{}\n",
        json!({"type":"item.completed","item":{"type":"agent_message","text":rows}}),
        json!({"type":"turn.completed","usage":{
            "input_tokens":13,"output_tokens":5,"cached_input_tokens":7
        }})
    )
}

// Success here describes this supplied adapter only. It does not invoke Opus::ask.
fn supplied_client(events: &str, ledger: &Path, items: &[&Item]) -> anyhow::Result<silver::Gated> {
    let value = mova_teacher::decode_events(events)?;
    let (rows, call) = interpret_reply("prag-silver", 9, items.len(), 0.25, &value);
    append_call(ledger, &call)?;
    Ok(silver::gate(&rows, items))
}

// Keep this indexing visible: the proposed compiled adapter mutant reverses it.
fn accepted_sources<'a>(g: &silver::Gated, items: &[&'a Item]) -> Vec<(&'a str, Act)> {
    g.ok.iter()
        .map(|(&k, label)| (items[k - 1].key.as_str(), label.act))
        .collect()
}

fn assert_accounting(ledger: &Path, attempted: usize) {
    let calls = read_calls(ledger).unwrap();
    assert_eq!(
        calls.len(),
        1,
        "one attempted call per supplied successful turn"
    );
    assert_eq!(
        serde_json::to_value(&calls[0]).unwrap(),
        json!({
            "step":"prag-silver",
            "batch":9,
            "sentences":attempted,
            "secs":0.25,
            "cost_usd":0.0,
            "input_tokens":13,
            "output_tokens":5,
            "cache_read_tokens":7,
            "cache_write_tokens":0
        }),
        "W17-PJ-ACCOUNTING-OBSERVATION"
    );
}

#[test]
fn turn_rows_accounting_and_labels_keep_batch_coordinates() {
    let tmp = Scratch::new();
    let ledger = tmp.0.join("calls.jsonl");
    let items = [
        item("tale-source", Src::Tale, 41),
        item("standalone-source", Src::Tatoeba, 0),
    ];
    let refs: Vec<_> = items.iter().collect();
    let rows = "2\tdecl\tassert\t0\t0\tnone\tnone\t-\n\
1\tdecl\tnarrate\t0\t0\tnone\tnarrator\t-\n";
    let g = supplied_client(&events(rows), &ledger, &refs).unwrap();
    assert_accounting(&ledger, 2);
    assert_eq!(g.junk, 0);
    assert_eq!(g.bad, []);
    assert_eq!(g.ok.keys().copied().collect::<Vec<_>>(), [1, 2]);
    assert_eq!(
        accepted_sources(&g, &refs),
        [
            ("tale-source", Act::Narrate),
            ("standalone-source", Act::Assert)
        ],
        "W17-PJ1-SOURCE-LABEL-JOIN"
    );
}

#[test]
fn attempted_counts_do_not_turn_partial_labels_into_acceptance() {
    let tmp = Scratch::new();
    let items = [
        item("tale-first", Src::Tale, 17),
        item("missing-middle", Src::Tatoeba, 0),
        item("standalone-third", Src::Tatoeba, 0),
    ];
    let refs: Vec<_> = items.iter().collect();
    let rows = "3\tdecl\tassert\t0\t0\tnone\tnone\t-\n\
1\tdecl\tnarrate\t0\t0\tnone\tnarrator\t-\n";
    let partial_ledger = tmp.0.join("partial.jsonl");
    let partial = supplied_client(&events(rows), &partial_ledger, &refs).unwrap();
    assert_accounting(&partial_ledger, 3);
    assert_eq!(partial.junk, 0);
    assert_eq!(partial.ok.keys().copied().collect::<Vec<_>>(), [1, 3]);
    assert_eq!(
        partial.bad,
        [(Some(2), "no line".into())],
        "W17-PJ2-MISSING-ROW"
    );
    assert_eq!(
        accepted_sources(&partial, &refs),
        [
            ("tale-first", Act::Narrate),
            ("standalone-third", Act::Assert)
        ]
    );
    assert_eq!(
        (
            read_calls(&partial_ledger).unwrap()[0].sentences,
            partial.ok.len()
        ),
        (3, 2),
        "W17-PJ2-ATTEMPTED-NOT-ACCEPTED"
    );

    let duplicate_ledger = tmp.0.join("duplicate.jsonl");
    let duplicate_rows = format!("{rows}1\tdecl\tnarrate\t0\t0\tnone\tnarrator\t-\n");
    let duplicate = supplied_client(&events(&duplicate_rows), &duplicate_ledger, &refs).unwrap();
    assert_accounting(&duplicate_ledger, 3);
    assert!(duplicate.ok.is_empty(), "W17-PJ2-DUPLICATE-REJECTS-LABELS");
    assert_eq!(
        duplicate.bad,
        [(
            None,
            "batch: extra numbers [], repeated [1] — alignment in doubt".into()
        )]
    );
    assert_eq!(read_calls(&duplicate_ledger).unwrap()[0].sentences, 3);
}

#[test]
fn failed_turn_stops_before_accounting_and_successful_suffix() {
    let tmp = Scratch::new();
    let ledger = tmp.0.join("calls.jsonl");
    let items = [item("prior-success", Src::Tatoeba, 0)];
    let refs: Vec<_> = items.iter().collect();
    let good = events("1\tdecl\tassert\t0\t0\tnone\tnone\t-\n");
    let prior = supplied_client(&good, &ledger, &refs).unwrap();
    assert_eq!(
        accepted_sources(&prior, &refs),
        [("prior-success", Act::Assert)]
    );
    assert_accounting(&ledger, 1);
    let before = std::fs::read(&ledger).unwrap();
    let failure = "{\"type\":\"turn.failed\"}\n";
    let error = "{\"type\":\"error\"}\n";
    let rejected = [
        ("failed suffix", format!("{good}{failure}")),
        (
            "failed prefix with valid suffix",
            format!("{failure}{good}"),
        ),
        ("repeated successful turn", format!("{good}{good}")),
        ("error prefix with valid suffix", format!("{error}{good}")),
    ];
    for (name, trace) in rejected {
        assert!(
            supplied_client(&trace, &ledger, &refs).is_err(),
            "W17-PJ3-DECODE-SHORT-CIRCUIT: {name}"
        );
        assert_eq!(
            std::fs::read(&ledger).unwrap(),
            before,
            "W17-PJ3-LEDGER-UNCHANGED: {name}"
        );
        assert_accounting(&ledger, 1);
    }
}
