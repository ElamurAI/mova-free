//! One Sea Spider per host: with the lease held, `serve`, `fetch` and `robots` refuse to start,
//! name the running holder and exit 5, before loading the deputy or touching the network.
use std::path::PathBuf;
use std::process::Command;

fn tmp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("seaspider-single-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn second_start_reports_the_running_one() {
    let d = tmp("cli");
    let lock = d.join("seaspider.lock");
    let config = d.join("seaspider.json");
    std::fs::write(&config, serde_json::json!({"robot":{"contact_url":"https://example.invalid/movabot"},"store":"store",
        "service":{"lock":lock}}).to_string()).unwrap();
    let queue = d.join("q.txt");
    std::fs::write(&queue, "https://example.invalid/\n").unwrap();
    let held = seaspider::singleton::acquire(&lock, "serve /run/first/seaspider.sock").unwrap();
    let bin = env!("CARGO_BIN_EXE_seaspider");
    let sock = d.join("sock").join("seaspider.sock");
    // The deputy configuration does not exist: a refused start never gets that far.
    for args in [
        vec!["serve", config.to_str().unwrap(), "/nonexistent/deputy.json", sock.to_str().unwrap()],
        vec!["fetch", config.to_str().unwrap(), queue.to_str().unwrap()],
        vec!["robots", config.to_str().unwrap(), "https://example.invalid/"],
    ] {
        let out = Command::new(bin).args(&args).output().unwrap();
        let err = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(5), "{args:?}: {err}");
        assert!(err.contains(&format!("Sea Spider is already running (pid {}, serve /run/first/seaspider.sock since ", std::process::id())), "{err}");
        assert!(err.contains("I did not start a second one."), "{err}");
    }
    assert!(!sock.exists() && !d.join("store").exists(), "a refused start creates no socket and no store");
    // Released: the same start proceeds past the lease (and fails later, on the missing deputy).
    drop(held);
    let out = Command::new(bin).args(["serve", config.to_str().unwrap(), "/nonexistent/deputy.json", sock.to_str().unwrap()]).output().unwrap();
    assert_eq!(out.status.code(), Some(1), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(!String::from_utf8_lossy(&out.stderr).contains("already running"));
    let _ = std::fs::remove_dir_all(&d);
}
