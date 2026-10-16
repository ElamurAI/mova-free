//! LLM scenarios (independent author — Opus, did not see the code): 400 in 10 areas, `tests/data/vmm/`. Frozen
//! by hash before the first run (commit 1f4581225; hashes re-computed 01.10 after translating only the three header comment lines of each file to English). Red ones — only analysed and classified (README, «Independent
//! tests»): a test error, a documented difference, or a defect in «What's next». A fixed defect or a new red one
//! changes the set — the test catches this in both directions.

use mlab::suite;

const FROZEN: &[(&str, &str)] = &[
    ("ctl.txt", "c04230118c3d8852e4ff42ae148e1a097e530d7ab75f8011eb40bdc2062611ab"),
    ("edge.txt", "a2f055d432ff9b0be4e680af690d922df35e6055a2cca9a62ac183dea662f221"),
    ("idx.txt", "45a7448f260c3048a412480bcd920a3a455aad136ba9214c02a6ea9fab0e0154"),
    ("la.txt", "16185d05de3a80466df21482e2295af659b9ecd8021eec50af34bd42caa681dc"),
    ("ops.txt", "68404a7712f93e3894aead63287b0984e270a04d9f45ff94dab9f8bf27914685"),
    ("poly.txt", "01299493168994d3a185c293c77bc70e821166939954b208d514a3c9fb86ea0f"),
    ("red.txt", "4618f1e4dcce298ab05731546559ba7f0df19d8767e77fe3a8ae2ccedcf7af8d"),
    ("stat.txt", "e96e4e5431684074008bbc14a46f0dd52c5fb160997bd2bee25fe43391f19bc3"),
    ("str.txt", "69669e40ec592ff1d53207b64bb84fcc6fad045d0c499370dae53c7f797b9377"),
    ("tab.txt", "bfd5c979a053e179f4a13d45ffb4e317dabf66bb6ea24e6b53ff1a66659ca41b"),
];

/// (id, class, why) — red after analysis.
const KNOWN_RED: &[(&str, &str, &str)] = &[
    ("vmm-la-015", "test error", "trace(A*B) = 28 (A*B = [5 11; 14 23]), the LLM expected 29"),
    ("vmm-stat-039", "test error", "anova1(y, g, 'Display', 'off'): in MATLAB the third argument is 'on'/'off'; the mistake was suggested by the prompt"),
    ("vmm-str-012", "difference", "strrep with overlap: MATLAB 'bb', Octave 'ba'; mlab — as Octave"),
    ("vmm-stat-040", "difference", "fitlm returns a coefficient table, not a LinearModel object (mdl.Coefficients) — documented"),
    ("vmm-ops-007", "defect, later", "A.^2' — transpose and power have the same precedence, left to right: (A.^2)'; mlab takes A.^(2')"),
    ("vmm-ctl-003", "defect, later", "after for k = 1:0 the variable k is an empty 1×0; in mlab it is undefined"),
    ("vmm-tab-008", "defect, later", "T.x(2) = 10 — assignment to an element of a table column is not parsed"),
    ("vmm-edge-033", "defect, later", "command syntax for any function: disp hello"),
];

fn sha256_hex(data: &[u8]) -> String {
    use std::process::Command;
    // the same sha256 as in tests/suite.rs — via the system sha256sum, so as not to duplicate the implementation
    // tests run in parallel in one process — the file name is unique for each call
    static SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let k = SEQ.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let tmp = std::env::temp_dir().join(format!("mlab-vmm-{}-{k}.bin", std::process::id()));
    std::fs::write(&tmp, data).unwrap();
    let out = Command::new("sha256sum").arg(&tmp).output().expect("sha256sum");
    let _ = std::fs::remove_file(&tmp);
    String::from_utf8_lossy(&out.stdout).split_whitespace().next().unwrap_or("").to_string()
}

fn load() -> Vec<suite::Scenario> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/vmm");
    let mut all = Vec::new();
    for (file, hash) in FROZEN {
        let text = std::fs::read_to_string(format!("{dir}/{file}")).unwrap();
        assert_eq!(&sha256_hex(text.as_bytes()), hash, "{file} changed after freezing");
        all.extend(suite::parse(&text));
    }
    all
}

fn check(vm: bool) {
    let sc = load();
    assert_eq!(sc.len(), 400);
    let res = suite::run_opts(&sc, None, vm);
    let mut red: Vec<&str> = res.iter().filter(|r| !r.pass).map(|r| r.id.as_str()).collect();
    red.sort();
    let mut want: Vec<&str> = KNOWN_RED.iter().map(|k| k.0).collect();
    want.sort();
    for r in res.iter().filter(|r| !r.pass && !want.contains(&r.id.as_str())) {
        eprintln!("✗ {}: {:?}", r.id, r.first_diff);
    }
    assert_eq!(red, want, "the set of red ones changed (vm = {vm})");
}

#[test]
fn vmm_scenarios_tree() {
    check(false);
}

#[test]
fn vmm_scenarios_vm() {
    check(true);
}

#[test]
fn vmm_scenarios_vm_gate_byte_identical() {
    let rep = suite::vm_gate(&load(), None);
    assert!(rep.mismatch.is_none(), "{:?}", rep.mismatch);
    assert_eq!(rep.same, 400);
}

/// Negative control: a broken `sum` must add red ones.
#[test]
fn vmm_negative_control_broken_sum() {
    let res = suite::run(&load(), Some("sum"));
    let red = res.iter().filter(|r| !r.pass).count();
    assert!(red > KNOWN_RED.len() + 10, "broken sum gave only {red} red");
}
