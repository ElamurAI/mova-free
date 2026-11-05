//! Generic Crab CLI. Commands perform only their explicitly selected operation.
use mova_crab::{
    angel, collect_fish_reports, dolphin, native_mova, recall_fish_state, store, supervisor, Result,
};
use serde_json::Value;
use std::{fs::File, io::Read, path::Path, process::ExitCode};
fn json(path: &str, limit: u64) -> Result<Value> {
    let mut b = Vec::new();
    File::open(path)?.take(limit + 1).read_to_end(&mut b)?;
    if b.len() as u64 > limit {
        return Err("input exceeds declared byte limit".into());
    }
    Ok(serde_json::from_slice(&b)?)
}
fn dispatch(a: &[String]) -> Result<()> {
    // Native capabilities have explicit paths, not deployment CONFIG or shell commands.
    if a.first().is_some_and(|s| s == "native-collect") {
        if a.len() != 4 {
            return Err("usage: mova-crab native-collect TRUSTED_ROOT INDEX NEW_DIR".into());
        }
        let index: native_mova::ReportIndex =
            serde_json::from_slice(&native_mova::read_bounded(Path::new(&a[2]))?)?;
        let report = collect_fish_reports(Path::new(&a[1]), &index, Path::new(&a[3]))?;
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }
    if a.first().is_some_and(|s| s == "native-recall") {
        if a.len() != 5 {
            return Err("usage: mova-crab native-recall TASK STATE TRACE_REPORT NEW_REPORT".into());
        }
        let task: native_mova::RecallTask =
            serde_json::from_slice(&native_mova::read_bounded(Path::new(&a[1]))?)?;
        let report = recall_fish_state(&task, Path::new(&a[2]), Path::new(&a[3]))?;
        native_mova::json_new(Path::new(&a[4]), &report)?;
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }
    if a.len() == 3 && a[0] == "ctl" {
        let request = json(&a[2], 16_384)?;
        let reply = angel::call(Path::new(&a[1]), &request)?;
        println!("{}", serde_json::to_string_pretty(&reply)?);
        if reply["ok"] != true {
            return Err("control request rejected".into());
        }
        return Ok(());
    }
    if a.len() < 2 {
        return Err("usage: mova-crab serve|prepare|annotate|ingest|status|dolphin-status|check|flow-status CONFIG [--once] | export CONFIG NEW_DIR | dolphin-finish CONFIG ID REPORT | ctl SOCKET REQUEST".into());
    }
    let c = json(&a[1], 256 * 1024)?;
    let once = a.get(2).is_some_and(|s| s == "--once");
    let value = match a[0].as_str() {
        "serve" if a.len() == 2 => {
            angel::serve(&c)?;
            Value::Null
        }
        "prepare" if a.len() == 2 => supervisor::prepare(&c)?,
        "annotate" if a.len() == 2 || (a.len() == 3 && once) => {
            supervisor::run(&c, once)?;
            Value::Null
        }
        "ingest" if a.len() == 2 => store::ingest(&c)?,
        "status" if a.len() == 2 => store::status(&c)?,
        "export" if a.len() == 3 => store::export(&c, Path::new(&a[2]))?,
        "dolphin-status" if a.len() == 2 => dolphin::status(&c)?,
        "check" if a.len() == 2 => angel::check(&c)?,
        "flow-status" if a.len() == 2 => angel::flow_observe(&c)?,
        "dolphin-finish" if a.len() == 4 => dolphin::finish(&c, &a[2], &json(&a[3], 128_000)?)?,
        _ => return Err("unknown command or invalid bounded arguments".into()),
    };
    if !value.is_null() {
        println!("{}", serde_json::to_string_pretty(&value)?);
    }
    Ok(())
}
fn main() -> ExitCode {
    match dispatch(&std::env::args().skip(1).collect::<Vec<_>>()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn absent_or_unknown_cli_does_not_launch_anything() {
        assert!(dispatch(&[]).is_err());
        assert!(dispatch(&["unknown".into()]).is_err());
    }
    #[test]
    fn native_wrong_arity_is_refused_before_deployment_config_read() {
        for command in ["native-collect", "native-recall"] {
            let error = dispatch(&[command.into(), "absent-config".into()]).unwrap_err();
            assert!(error.to_string().starts_with("usage: mova-crab native-"));
        }
    }
    #[test]
    fn native_collection_cli_keeps_external_evidence_declared() {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("crab-native-cli-{}-{stamp}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let evidence = b"bounded external review";
        std::fs::write(root.join("review.txt"), evidence).unwrap();
        let index = serde_json::json!({
            "schema":1,"id":"cli-collection","reports":[{
                "id":"external0","actor":"fish","kind":"external_review","outcome":"rejected",
                "source":null,"findings":[],"artifacts":[{
                    "path":"review.txt","sha256":native_mova::digest(evidence),"format":"external_review_evidence"
                }]
            }]
        });
        let input = root.join("index.json");
        std::fs::write(&input, serde_json::to_vec(&index).unwrap()).unwrap();
        let output = root.join("collection");
        dispatch(&[
            "native-collect".into(),
            root.display().to_string(),
            input.display().to_string(),
            output.display().to_string(),
        ])
        .unwrap();
        let report: Value =
            serde_json::from_slice(&std::fs::read(output.join("report.json")).unwrap()).unwrap();
        assert_eq!(report["semantic_admission"], false);
        assert_eq!(report["groups"][0]["declared_outcome"], "rejected");
        assert_eq!(
            report["review_queue"][0]["artifacts"][0]["format"],
            "external_review_evidence"
        );
        assert!(dispatch(&[
            "native-collect".into(),
            root.display().to_string(),
            input.display().to_string(),
            output.display().to_string()
        ])
        .is_err());
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn native_input_fifo_is_rejected_without_a_writer() {
        use std::{ffi::CString, os::unix::ffi::OsStrExt};
        let path = std::env::temp_dir().join(format!("crab-native-fifo-{}", std::process::id()));
        let name = CString::new(path.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
        let start = std::time::Instant::now();
        let error = dispatch(&[
            "native-collect".into(),
            "/".into(),
            path.display().to_string(),
            "unused-output".into(),
        ])
        .unwrap_err();
        assert!(error.to_string().contains("bounded regular file"));
        assert!(start.elapsed() < std::time::Duration::from_secs(1));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn bounded_json_refuses_oversized_and_invalid_inputs() {
        let p = std::env::temp_dir().join(format!("crab-cli-{}", std::process::id()));
        std::fs::write(&p, b"null").unwrap();
        assert!(json(p.to_str().unwrap(), 3).is_err());
        assert!(json(p.to_str().unwrap(), 4).unwrap().is_null());
        std::fs::write(&p, b"{").unwrap();
        assert!(json(p.to_str().unwrap(), 4).is_err());
        std::fs::remove_file(p).unwrap();
    }
}
