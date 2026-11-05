//! Synthetic public run/report joins. Rule V1; no teacher, model, database or corpus.
use qagen::{
    report, run,
    schema::Rule,
    select::{Para, Pick, Sent},
};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        let p = std::env::temp_dir().join(format!(
            "mova-wave18-join-{}-{}",
            std::process::id(),
            ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        fs::create_dir(p.join("raw")).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn para() -> Para {
    Para {
        doc: "fixture:1".into(),
        book: "fixture".into(),
        title: "Original fixture".into(),
        author: "Fixture".into(),
        para: 1,
        block: "p".into(),
        sents: vec![Sent {
            sent_id: "s1".into(),
            text: "Mira carried a blue box.".into(),
        }],
        dialogue: false,
        words: 6,
        ctx: vec![],
        batch: 1,
    }
}
fn raw(answer: &str) -> String {
    format!("1\tcharacter\texplicit\t1\tWho carried a box?\t{answer}\n")
}
fn call(tokens: u64) -> serde_json::Value {
    serde_json::json!({"step":"fixture","batch":1,"sentences":1,"secs":1.25,"cost_usd":0.125,"input_tokens":tokens,"output_tokens":2,"cache_read_tokens":3,"cache_write_tokens":4})
}
fn write_calls(dir: &Path, values: &[serde_json::Value]) {
    fs::write(
        dir.join("calls.jsonl"),
        values.iter().map(|v| format!("{v}\n")).collect::<String>(),
    )
    .unwrap();
}
#[test]
fn retry_resolution_joins_gather_and_collect_accounting() {
    let t = Temp::new();
    let paras = [para()];
    fs::write(run::raw_path(&t.0, 1), raw("primary")).unwrap();
    fs::write(run::retry_path(&t.0, 1), raw("retry")).unwrap();
    write_calls(&t.0, &[call(7), call(11)]);
    assert_eq!(
        run::try_final_raw(&t.0, 1).unwrap(),
        Some(run::retry_path(&t.0, 1)),
        "W18-QJ1-RETRY-PATH"
    );
    let gathered = report::gather(&t.0, &paras, Rule::V1).unwrap();
    assert_eq!(gathered.pairs.len(), 1);
    assert_eq!(gathered.pairs[0].1.answer, "retry");
    let params = report::collect(
        &t.0,
        &paras,
        &Pick {
            rule: 1,
            ..Pick::default()
        },
    )
    .unwrap();
    assert_eq!(params["pairs"]["n"], 1);
    assert_eq!(params["calls"]["n"], 2);
    assert_eq!(params["calls"]["retries"], 1);
    assert_eq!(params["calls"]["input_tokens"], 18);
    assert_eq!(params["calls"]["secs_sum"], 2.5);
    assert_eq!(params["calls"]["cost_usd_estimate"], 0.25);
    let qa = fs::read_to_string(t.0.join("qa.tsv")).unwrap();
    assert!(qa.contains("\ts1\tWho carried a box?\tretry\n"));
    assert!(!qa.contains("primary"));
    let disk: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(t.0.join("params.json")).unwrap()).unwrap();
    assert_eq!(disk, params);
}
#[test]
fn accounting_failure_preflight_preserves_all_report_artifacts() {
    let t = Temp::new();
    let paras = [para()];
    fs::write(run::raw_path(&t.0, 1), raw("Mira")).unwrap();
    for failure in 0..2 {
        for name in ["qa.tsv", "rejects.tsv", "params.json"] {
            fs::write(t.0.join(name), format!("sentinel:{name}")).unwrap();
        }
        if failure == 0 {
            fs::write(t.0.join("calls.jsonl"), "{broken\n").unwrap();
        } else {
            write_calls(&t.0, &[call(u64::MAX), call(1)]);
        }
        assert!(
            report::collect(
                &t.0,
                &paras,
                &Pick {
                    rule: 1,
                    ..Pick::default()
                }
            )
            .is_err(),
            "accounting_preflight_rejects"
        );
        for name in ["qa.tsv", "rejects.tsv", "params.json"] {
            assert_eq!(
                fs::read_to_string(t.0.join(name)).unwrap(),
                format!("sentinel:{name}"),
                "accounting_preflight_preserves_artifacts"
            );
        }
    }
}
#[cfg(unix)]
#[test]
fn retry_metadata_error_propagates_through_gather() {
    let t = Temp::new();
    let paras = [para()];
    fs::write(run::raw_path(&t.0, 1), raw("Mira")).unwrap();
    let retry = run::retry_path(&t.0, 1);
    std::os::unix::fs::symlink(retry.file_name().unwrap(), &retry).unwrap();
    assert!(run::try_final_raw(&t.0, 1).is_err());
    assert!(
        report::gather(&t.0, &paras, Rule::V1).is_err(),
        "retry_error_is_not_missing_or_primary_fallback"
    );
}
