//! Synthetic checkpoint transport probe; no training or answer-quality claim.
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant},
};
use world::knowledge::{Lesson, retention::Retained};

const COURSE: &str = "synthetic-checkpoint";
const TEXT: &str = "A copper token rests in a box.";
static SERIAL: AtomicUsize = AtomicUsize::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mova-checkpoint-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn lesson() -> Lesson {
    Lesson {
        course: COURSE.into(),
        n: 1,
        explain: vec![TEXT.into()],
        ..Default::default()
    }
}
// The public wire contract has struct field order, unlike Value's sorted maps.
// This independent encoder names that contract explicitly; no private types.
fn ordered(object: &Value, fields: &[&str]) -> String {
    format!(
        "{{{}}}",
        fields
            .iter()
            .map(|key| format!("{}:{}", serde_json::to_string(key).unwrap(), object[*key]))
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn payload_bytes(v: &Value) -> Vec<u8> {
    let p = &v["payload"];
    let graph = ordered(
        &p["graph"],
        &["frames", "same", "texts", "weights", "parts", "isa", "per"],
    );
    // `dictionary` is always written (empty in states saved before it was recorded);
    // `licence` only when the dictionary held share-alike or non-commercial add-ons.
    let dictionary = p.get("dictionary").cloned().unwrap_or_else(|| json!(""));
    let licence = match p.get("licence").and_then(Value::as_str) {
        Some(l) if !l.is_empty() => format!(",\"licence\":{}", json!(l)),
        _ => String::new(),
    };
    format!(
        "{{\"schema\":{},\"course\":{},\"lessons\":{},\"graph\":{},\"dictionary\":{}{}}}",
        p["schema"], p["course"], p["lessons"], graph, dictionary, licence
    )
    .into_bytes()
}
fn reseal(v: &mut Value) {
    v["sha256"] = hash(&payload_bytes(v)).into();
}
fn write(path: &Path, v: &Value) {
    fs::write(path, serde_json::to_vec(v).unwrap()).unwrap();
}
fn fixture(dir: &Scratch) -> Value {
    let empty = dir.path("empty.json");
    Retained::new(COURSE).save(&empty).unwrap();
    let mut v: Value = serde_json::from_slice(&fs::read(empty).unwrap()).unwrap();
    let digest = hash(
        &serde_json::to_vec(&(
            COURSE,
            7usize,
            vec![TEXT],
            Vec::<String>::new(),
            Vec::<(String, String, Vec<String>)>::new(),
        ))
        .unwrap(),
    );
    v["payload"]["lessons"] = json!({"7": digest});
    v["payload"]["graph"]["texts"] =
        json!([[[7, 0], TEXT], [[7, 1], "A second synthetic source."]]);
    let weight = v["payload"]["graph"]["weights"][0].as_i64().unwrap();
    v["payload"]["graph"]["weights"][0] = (weight + 3).into();
    assert!(
        v["payload"]["graph"]["frames"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    reseal(&mut v);
    v
}
fn child(dir: &Scratch, input: &Path, label: &str, mode: &str) -> Value {
    let report = dir.path(&format!("{label}.report"));
    let output = dir.path(&format!("{label}.json"));
    let log = fs::File::create(dir.path(&format!("{label}.log"))).unwrap();
    let mut process = Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "child_checkpoint_probe",
            "--nocapture",
        ])
        .env("MOVA_CHECKPOINT_INPUT", input)
        .env("MOVA_CHECKPOINT_OUTPUT", output)
        .env("MOVA_CHECKPOINT_REPORT", &report)
        .env("MOVA_CHECKPOINT_MODE", mode)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    let status = loop {
        if let Some(status) = process.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = process.kill();
            let _ = process.wait();
            panic!("checkpoint child timed out");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    assert!(
        status.success(),
        "child failed: {}",
        fs::read_to_string(dir.path(&format!("{label}.log"))).unwrap()
    );
    let receipt: Value = serde_json::from_slice(&fs::read(report).unwrap()).unwrap();
    assert_ne!(
        receipt["pid"].as_u64().unwrap(),
        u64::from(std::process::id())
    );
    receipt["logical"].clone()
}
#[test]
#[ignore = "subprocess helper; requires explicit checkpoint paths and mode"]
fn child_checkpoint_probe() {
    let input =
        PathBuf::from(std::env::var_os("MOVA_CHECKPOINT_INPUT").expect("explicit child input"));
    let output =
        PathBuf::from(std::env::var_os("MOVA_CHECKPOINT_OUTPUT").expect("explicit child output"));
    let mode = std::env::var("MOVA_CHECKPOINT_MODE").expect("explicit child mode");
    let before = fs::read(&input).unwrap();
    let logical = if mode == "hash-error" {
        let err = Retained::load(&input)
            .err()
            .expect("altered checksum must fail")
            .to_string();
        assert_eq!(err, "retained state hash mismatch");
        assert!(!output.exists());
        json!({"error":err})
    } else {
        assert_eq!(mode, "restore");
        let retained = Retained::load(&input).unwrap();
        let fingerprint = retained.fingerprint().unwrap();
        let matching = lesson();
        let found = retained.retained_lesson(&matching).unwrap();
        let mut presentation = matching.clone();
        presentation.n = 999;
        presentation.title = "Alternative presentation".into();
        presentation
            .tests
            .push(("Unused question?".into(), "One".into(), vec!["Two".into()]));
        let metadata_match = retained.retained_lesson(&presentation).unwrap();
        let mut changed = matching.clone();
        changed.explain.push("Different evidence.".into());
        let changed_match = retained.retained_lesson(&changed).unwrap();
        let mut foreign = matching;
        foreign.course = "other-course".into();
        let foreign_error = retained.retained_lesson(&foreign).unwrap_err().to_string();
        let next = retained.next_lesson_number().unwrap();
        retained.save(&output).unwrap();
        let output_before = fs::read(&output).unwrap();
        let err = retained.save(&output).unwrap_err();
        assert_eq!(
            err.downcast_ref::<std::io::Error>().unwrap().kind(),
            std::io::ErrorKind::AlreadyExists
        );
        assert_eq!(fs::read(&output).unwrap(), output_before);
        assert_eq!(retained.fingerprint().unwrap(), fingerprint);
        json!({"fingerprint":fingerprint,"found":found,"metadata_match":metadata_match,"changed_match":changed_match,"next":next,"foreign_error":foreign_error,"overwrite":"AlreadyExists"})
    };
    assert_eq!(fs::read(input).unwrap(), before);
    fs::write(
        std::env::var_os("MOVA_CHECKPOINT_REPORT").expect("explicit child report"),
        serde_json::to_vec(&json!({"pid":std::process::id(),"logical":logical})).unwrap(),
    )
    .unwrap();
}
#[test]
fn synthetic_checkpoint_survives_fresh_process_and_canonicalizes() {
    let dir = Scratch::new();
    let mut v = fixture(&dir);
    let canonical_hash = hash(&payload_bytes(&v));
    let input = dir.path("input.json");
    // Vector order is accepted but is not retained after map reconstruction.
    v["payload"]["graph"]["texts"]
        .as_array_mut()
        .unwrap()
        .reverse();
    reseal(&mut v);
    assert_ne!(v["sha256"], canonical_hash);
    write(&input, &v);
    let original = fs::read(&input).unwrap();
    let a = child(&dir, &input, "first", "restore");
    let b = child(&dir, &input, "second", "restore");
    assert_eq!(a, b);
    assert_eq!(
        a,
        json!({"fingerprint":canonical_hash,"found":7,"metadata_match":7,"changed_match":null,"next":8,"foreign_error":"different course needs a separate retained state","overwrite":"AlreadyExists"})
    );
    assert_eq!(fs::read(&input).unwrap(), original);
    let saved = fs::read(dir.path("first.json")).unwrap();
    assert_eq!(saved, fs::read(dir.path("second.json")).unwrap());
    let canonical: Value = serde_json::from_slice(&saved).unwrap();
    assert_eq!(
        canonical["payload"]["graph"]["texts"][0],
        json!([[7, 0], TEXT])
    );
    assert_eq!(
        canonical["payload"]["graph"]["weights"],
        v["payload"]["graph"]["weights"]
    );
    let c = child(&dir, &dir.path("first.json"), "third", "restore");
    assert_eq!(a, c);
    assert_eq!(saved, fs::read(dir.path("third.json")).unwrap());
    println!("logical checkpoint restore: {a}");
}
#[test]
fn checksum_rejects_unsealed_changes_but_does_not_authenticate_resealed_state() {
    let dir = Scratch::new();
    let mut v = fixture(&dir);
    let original_hash = v["sha256"].clone();
    let weight = v["payload"]["graph"]["weights"][0].as_i64().unwrap();
    v["payload"]["graph"]["weights"][0] = (weight + 1).into();
    let input = dir.path("tampered.json");
    write(&input, &v);
    let original = fs::read(&input).unwrap();
    assert_eq!(
        child(&dir, &input, "reject", "hash-error"),
        json!({"error":"retained state hash mismatch"})
    );
    assert_eq!(fs::read(&input).unwrap(), original);
    reseal(&mut v);
    let resealed = dir.path("resealed.json");
    write(&resealed, &v);
    let accepted = child(&dir, &resealed, "accept", "restore");
    assert_eq!(accepted["fingerprint"], v["sha256"]);
    assert_ne!(accepted["fingerprint"], original_hash);
    assert_eq!(accepted["found"], 7);
    assert_eq!(accepted["next"], 8);
    println!(
        "logical checkpoint integrity: rejected stale checksum; accepted distinct resealed fingerprint"
    );
}
