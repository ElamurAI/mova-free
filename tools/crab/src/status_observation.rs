//! Bounded read-only observations of explicitly configured worker status files.
//! A matching, fresh artifact is not process health or permission to control a worker.
use crate::Result;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::OpenOptions,
    io::Read,
    os::unix::fs::OpenOptionsExt,
    path::{Component, PathBuf},
};

pub const MAX_ATTACHMENTS: usize = 8;
pub const MAX_STATUS_BYTES: usize = 256 * 1024;
const MAX_COUNTERS: usize = 16;
const STALE_SECONDS: u64 = 120;

/// Trusted immutable deployment input, not a client-selected path or executable.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatusAttachment {
    pub id: String,
    pub path: PathBuf,
    pub expected_kind: String,
    pub expected_config_sha256: String,
}

fn identifier(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
}

/// Validate once when constructing the owning Crab context; no artifact is opened here.
pub fn validate(attachments: &[StatusAttachment]) -> Result<()> {
    if attachments.len() > MAX_ATTACHMENTS {
        return Err("too many status attachments".into());
    }
    let mut ids = BTreeSet::new();
    for a in attachments {
        if !identifier(&a.id) || !ids.insert(&a.id) || !identifier(&a.expected_kind) {
            return Err("status attachments require distinct bounded IDs and kinds".into());
        }
        if !a.path.is_absolute()
            || a.path.to_str().is_none_or(|s| s.len() > 1024)
            || a.path
                .components()
                .any(|c| matches!(c, Component::ParentDir))
        {
            return Err("status attachment paths must be absolute without parent traversal".into());
        }
        if a.expected_config_sha256.len() != 64
            || !a
                .expected_config_sha256
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("status attachment config identity must be lowercase SHA256".into());
        }
    }
    Ok(())
}

// Fixed projection excludes prompts, artifact paths, arbitrary objects and model prose.
fn projection(v: &Value) -> Value {
    let mut out = Map::new();
    for key in [
        "schema",
        "pid",
        "cursor",
        "not_after_unix",
        "pages_total",
        "pages_materialized",
        "targets_total",
        "heartbeat_unix",
    ] {
        if let Some(n) = v[key].as_u64() {
            out.insert(key.into(), json!(n));
        }
    }
    for key in ["model_invoked", "training_authorized", "active_model_call"] {
        if let Some(b) = v[key].as_bool() {
            out.insert(key.into(), json!(b));
        }
    }
    for key in ["state", "current_page"] {
        if let Some(s) = v[key]
            .as_str()
            .filter(|s| !s.is_empty() && s.len() <= 128 && !s.chars().any(char::is_control))
        {
            out.insert(key.into(), json!(s));
        }
    }
    for key in ["counts", "source_counts"] {
        if let Some(m) = v[key].as_object() {
            let numeric: Vec<_> = m
                .iter()
                .filter(|(k, v)| identifier(k) && v.as_u64().is_some())
                .collect();
            let mut counters = Map::new();
            for (name, value) in numeric.iter().take(MAX_COUNTERS) {
                counters.insert((*name).clone(), (*value).clone());
            }
            out.insert(key.into(), Value::Object(counters));
            if numeric.len() > MAX_COUNTERS {
                out.insert(format!("{key}_truncated"), json!(true));
            }
        }
    }
    Value::Object(out)
}

/// Read one configured artifact at a supplied observation time. No process probe or write.
/// Identity failures withhold the projection. Future/missing heartbeat remains explicit.
pub fn observe(a: &StatusAttachment, read_unix: u64) -> Value {
    if validate(std::slice::from_ref(a)).is_err() {
        return json!({"status":"invalid_configuration","health_inferred":false,"control_authority":false,"training_admission":false});
    }
    let mut result = json!({"id":a.id,"path":a.path,"read_unix":read_unix,
        "health_inferred":false,"control_authority":false,"training_admission":false});
    // O_NONBLOCK prevents special-file opens from blocking; O_NOFOLLOW rejects a final symlink.
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW)
        .open(&a.path);
    let mut file = match file {
        Ok(f) => f,
        Err(e) => {
            result["status"] = json!(if e.kind() == std::io::ErrorKind::NotFound {
                "missing"
            } else {
                "read_error"
            });
            return result;
        }
    };
    match file.metadata() {
        Ok(m) if !m.is_file() => {
            result["status"] = json!("not_regular");
            return result;
        }
        Ok(m) if m.len() > MAX_STATUS_BYTES as u64 => {
            result["status"] = json!("oversized");
            return result;
        }
        Err(_) => {
            result["status"] = json!("read_error");
            return result;
        }
        _ => {}
    }
    let mut bytes = Vec::new();
    if (&mut file)
        .take(MAX_STATUS_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .is_err()
    {
        result["status"] = json!("read_error");
        return result;
    }
    if bytes.len() > MAX_STATUS_BYTES {
        result["status"] = json!("oversized");
        return result;
    }
    result["artifact_sha256"] = json!(format!("{:x}", Sha256::digest(&bytes)));
    let v: Value = match serde_json::from_slice(&bytes) {
        Ok(Value::Object(m)) => Value::Object(m),
        _ => {
            result["status"] = json!("malformed");
            return result;
        }
    };
    if v["kind"].as_str() != Some(a.expected_kind.as_str())
        || v["config_sha256"].as_str() != Some(a.expected_config_sha256.as_str())
    {
        result["status"] = json!("identity_mismatch");
        return result;
    }
    result["status"] = json!("observed");
    result["kind"] = json!(a.expected_kind);
    let heartbeat = v["heartbeat_unix"].as_u64();
    let freshness = match heartbeat {
        None => "unknown",
        Some(t) if t > read_unix => "future",
        Some(t) if read_unix - t > STALE_SECONDS => "stale",
        Some(_) => "fresh",
    };
    result["freshness"] = json!(freshness);
    result["heartbeat_age_seconds"] =
        json!(heartbeat.filter(|t| *t <= read_unix).map(|t| read_unix - t));
    result["value"] = projection(&v);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        root: PathBuf,
        attachment: StatusAttachment,
    }
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "crab-status-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            let attachment = StatusAttachment {
                id: "worker-0".into(),
                path: root.join("status.json"),
                expected_kind: "native_worker".into(),
                expected_config_sha256: "a".repeat(64),
            };
            Self { root, attachment }
        }
        fn write(&self, v: &Value) {
            fs::write(&self.attachment.path, serde_json::to_vec(v).unwrap()).unwrap();
        }
        fn status(&self) -> Value {
            json!({"kind":"native_worker","config_sha256":"a".repeat(64),"heartbeat_unix":1000,"state":"active","counts":{"validated_targets":12,"quarantined_targets":2},"source_counts":{"prepared_targets":3,"deferred":5},"prompt":"private untrusted prose","details":{"secret":"excluded"}})
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    #[test]
    fn exact_identity_keeps_source_counters_without_leaking_arbitrary_json() {
        let f = Fixture::new();
        f.write(&f.status());
        validate(std::slice::from_ref(&f.attachment)).unwrap();
        let got = observe(&f.attachment, 1010);
        assert_eq!(got["status"], "observed");
        assert_eq!(got["freshness"], "fresh");
        assert_eq!(got["value"]["counts"]["validated_targets"], 12);
        assert_eq!(got["value"]["source_counts"]["deferred"], 5);
        assert!(got["value"].get("prompt").is_none());
        assert!(got["value"].get("details").is_none());
        assert_eq!(got, observe(&f.attachment, 1010));
        assert_eq!(got["health_inferred"], false);
        assert_eq!(got["control_authority"], false);
    }
    #[test]
    fn identity_and_heartbeat_partitions_keep_unknown_and_future_explicit() {
        let f = Fixture::new();
        let mut v = f.status();
        for key in ["kind", "config_sha256"] {
            let mut wrong = v.clone();
            wrong[key] = json!("wrong");
            f.write(&wrong);
            let got = observe(&f.attachment, 1000);
            assert_eq!(got["status"], "identity_mismatch");
            assert!(got.get("value").is_none());
        }
        for (time, state, age) in [
            (Some(1001), "future", None),
            (Some(880), "fresh", Some(120)),
            (Some(879), "stale", Some(121)),
            (None, "unknown", None),
        ] {
            v["heartbeat_unix"] = json!(time);
            f.write(&v);
            let got = observe(&f.attachment, 1000);
            assert_eq!(got["freshness"], state);
            assert_eq!(got["heartbeat_age_seconds"], json!(age));
        }
    }
    #[test]
    fn missing_malformed_special_and_oversized_artifacts_are_not_observations() {
        let f = Fixture::new();
        assert_eq!(observe(&f.attachment, 1000)["status"], "missing");
        for b in [b"broken".as_slice(), b"[]", b"null"] {
            fs::write(&f.attachment.path, b).unwrap();
            assert_eq!(observe(&f.attachment, 1000)["status"], "malformed");
        }
        fs::write(&f.attachment.path, vec![b' '; MAX_STATUS_BYTES + 1]).unwrap();
        assert_eq!(observe(&f.attachment, 1000)["status"], "oversized");
        fs::remove_file(&f.attachment.path).unwrap();
        fs::create_dir(&f.attachment.path).unwrap();
        assert_eq!(observe(&f.attachment, 1000)["status"], "not_regular");
        fs::remove_dir(&f.attachment.path).unwrap();
        std::os::unix::fs::symlink("missing-target", &f.attachment.path).unwrap();
        assert_eq!(observe(&f.attachment, 1000)["status"], "read_error");
    }
    #[test]
    fn invalid_or_excessive_catalogs_are_rejected_before_reads() {
        let f = Fixture::new();
        validate(&[]).unwrap();
        assert!(validate(&vec![f.attachment.clone(); MAX_ATTACHMENTS + 1]).is_err());
        assert!(validate(&[f.attachment.clone(), f.attachment.clone()]).is_err());
        for mode in 0..5 {
            let mut a = f.attachment.clone();
            match mode {
                0 => a.id = "bad\n".into(),
                1 => a.expected_kind = "".into(),
                2 => a.path = "relative".into(),
                3 => a.path = "/absolute/../escape".into(),
                _ => a.expected_config_sha256 = "A".repeat(64),
            };
            assert!(validate(&[a]).is_err());
        }
        let mut invalid = f.attachment.clone();
        invalid.path = "relative".into();
        assert_eq!(observe(&invalid, 1000)["status"], "invalid_configuration");
        assert!(!f.attachment.path.exists());
    }
    #[test]
    fn fifo_is_rejected_without_waiting_for_a_writer() {
        use std::os::unix::ffi::OsStrExt;
        let f = Fixture::new();
        let name = std::ffi::CString::new(f.attachment.path.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
        assert_eq!(observe(&f.attachment, 1000)["status"], "not_regular");
    }
    #[test]
    fn counter_and_string_projection_budgets_are_explicit() {
        let f = Fixture::new();
        let mut v = f.status();
        v["state"] = json!("s".repeat(129));
        v["counts"] = json!((0..20)
            .map(|i| (format!("c{i:02}"), json!(i)))
            .collect::<Map<_, _>>());
        v["counts"]["private"] = json!("secret");
        f.write(&v);
        let got = observe(&f.attachment, 1000);
        assert!(got["value"].get("state").is_none());
        assert_eq!(got["value"]["counts"].as_object().unwrap().len(), 16);
        assert_eq!(got["value"]["counts_truncated"], true);
        assert!(got["value"]["counts"].get("private").is_none());
        assert!(serde_json::to_vec(&got).unwrap().len() < 8192);
    }
}
