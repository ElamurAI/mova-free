use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let p = std::env::temp_dir().join(format!("mova-twin-test-{}-{stamp}", std::process::id()));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn spec(dir: &Path, script: &str) -> Value {
    json!({"schema":1,"family":"control","context":{"hardware":"test"},"executable":"/bin/sh",
        "cwd":dir,"args":["-c",script],"env":{},"axes":[],"inputs":{},
        "feature_source":"constant test workload","features":{"intercept":1.},
        "prior":{"intercept":1.},"ridge":1.,"timeout_seconds":1,
        "outputs":[{"path":"stdout.txt","contains":["complete"],"sha256":null}]})
}
fn run(root: &Path, s: &Value, name: &str, parent: Option<&str>) -> std::process::Output {
    let p = root.join(format!("{name}.json"));
    fs::write(&p, serde_json::to_vec(s).unwrap()).unwrap();
    let mut c = Command::new(env!("CARGO_BIN_EXE_mova-twin"));
    c.arg("run").arg(&p).arg(root.join(name));
    if let Some(parent) = parent {
        c.arg(root.join(parent));
    }
    c.output().unwrap()
}
fn value(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
#[test]
fn forecast_precedes_execution_and_parent_chain_is_prospective() {
    let t = Scratch::new();
    let s = spec(
        &t.0,
        "test -f {run}/forecast.json && test ! -e {run}/observation.json && echo complete",
    );
    let r = run(&t.0, &s, "one", None);
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    let before = fs::read(t.0.join("one/forecast.json")).unwrap();
    assert!(run(&t.0, &s, "two", Some("one")).status.success());
    assert_eq!(fs::read(t.0.join("one/forecast.json")).unwrap(), before);
    assert_eq!(
        value(&t.0.join("two/forecast.json"))["prediction"]["observations"],
        1
    );
    assert_eq!(value(&t.0.join("two/updated.json"))["model"]["n"], 2);
    assert!(!run(&t.0, &s, "two", Some("one")).status.success());
    let mut changed = s.clone();
    changed["context"]["hardware"] = json!("different");
    assert!(
        !run(&t.0, &changed, "wrong-domain", Some("two"))
            .status
            .success()
    );
    let report = Command::new(env!("CARGO_BIN_EXE_mova-twin"))
        .arg("report")
        .arg(t.0.join("one"))
        .arg(t.0.join("two"))
        .output()
        .unwrap();
    assert!(report.status.success());
    let rows: Value = serde_json::from_slice(&report.stdout).unwrap();
    assert_eq!(rows.as_array().unwrap().len(), 2);
    assert_eq!(rows[1]["observations_before"], 1);
    let mut checkpoint = value(&t.0.join("two/updated.json"));
    checkpoint["model"]["theta"][0] = json!(12345.);
    fs::write(
        t.0.join("two/updated.json"),
        serde_json::to_vec(&checkpoint).unwrap(),
    )
    .unwrap();
    assert!(
        !run(&t.0, &s, "model-tampered", Some("two"))
            .status
            .success()
    );
    fs::write(t.0.join("one/observation.json"), b"{}").unwrap();
    assert!(!run(&t.0, &s, "tampered", Some("one")).status.success());
}
#[test]
fn failed_timeout_invalid_output_and_changed_input_do_not_train() {
    let t = Scratch::new();
    for (name, script) in [
        ("exit", "echo complete; exit 7"),
        ("timeout", "sleep 10"),
        ("missing", "echo wrong"),
    ] {
        let s = spec(&t.0, script);
        assert!(!run(&t.0, &s, name, None).status.success());
        assert!(t.0.join(name).join("forecast.json").is_file());
        assert_eq!(
            value(&t.0.join(name).join("observation.json"))["accepted"],
            false
        );
        assert!(!t.0.join(name).join("updated.json").exists());
    }
    fs::write(t.0.join("input"), "old").unwrap();
    let mut s = spec(&t.0, "echo changed > input; echo complete");
    s["inputs"] = json!({"fixture":t.0.join("input")});
    assert!(!run(&t.0, &s, "mutated", None).status.success());
    assert!(!t.0.join("mutated/updated.json").exists());
    assert!(
        value(&t.0.join("mutated/observation.json"))["errors"]
            .to_string()
            .contains("changed during")
    );
}
#[test]
fn prediction_does_not_launch_the_command() {
    let t = Scratch::new();
    let s = spec(&t.0, "touch launched; echo complete");
    let p = t.0.join("spec.json");
    fs::write(&p, serde_json::to_vec(&s).unwrap()).unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_mova-twin"))
        .arg("predict")
        .arg(p)
        .output()
        .unwrap();
    assert!(o.status.success());
    assert!(!t.0.join("launched").exists());
}
