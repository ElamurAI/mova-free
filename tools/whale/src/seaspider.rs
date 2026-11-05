//! The Whales asking Sea Spider in English. Off unless the Whale animal configuration sets
//! `seaspider_socket`; then a Whale sends one English request ("Fetch https://….", "What did you
//! get from https://…?") over Sea Spider's private socket as caller `whale` and gets Sea Spider's
//! English answer back. Sea Spider decides by its own allowlist whether it answers the Whales and
//! journals who asked. The transport is the shared `mova_octopus::relay::ask_animal`.
use crate::animal::AnimalConfig;
use crate::Result;
pub use mova_octopus::relay::Reply;

/// The caller name the Whales declare.
pub const CALLER: &str = "whale";

/// Ask Sea Spider one English request; refused when no Sea Spider socket is configured.
pub fn ask_sea_spider(cfg: &AnimalConfig, id: &str, english: &str, urgent: bool) -> Result<Reply> {
    let socket = cfg.seaspider_socket.as_ref().ok_or("Sea Spider is not configured for the Whales (seaspider_socket is unset)")?;
    mova_octopus::relay::ask_animal(socket, CALLER, id, english, urgent).map_err(|e| format!("{e:#}").into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mova_octopus::socket::{serve_requests, Request};
    use serde_json::json;
    use std::time::Duration;

    fn cfg(socket: Option<&std::path::Path>) -> AnimalConfig {
        let mut v = json!({"run_root":"/r","executable":{"path":"/r/x","sha256":"0".repeat(64)},"survey":{},
            "catalog":{"path":"/r/c.jsonl","sha256":"0".repeat(64),"kind":"paper","path_field":"text_path","formats":{}}});
        if let Some(s) = socket {
            v["seaspider_socket"] = json!(s);
        }
        serde_json::from_value(v).unwrap()
    }

    #[test]
    fn off_unless_configured_and_asks_as_whale() {
        // Off by default: no socket, no request.
        let e = ask_sea_spider(&cfg(None), "s-0", "Are you busy?", false).unwrap_err().to_string();
        assert!(e.contains("not configured"), "{e}");
        let d = std::env::temp_dir().join(format!("whale-seaspider-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        let sock = d.join("seaspider.sock");
        let s2 = sock.clone();
        let server = std::thread::spawn(move || {
            serve_requests(&s2, &|r: &Request| Ok(json!({"answer":format!("{} asked: {}", r.from.as_deref().unwrap_or("-"), r.prompt)}))).unwrap()
        });
        while !sock.exists() {
            std::thread::sleep(Duration::from_millis(20));
        }
        let r = ask_sea_spider(&cfg(Some(&sock)), "s-1", "Are you busy?", false).unwrap();
        assert!(r.ok);
        assert_eq!(r.answer, "whale asked: Are you busy?");
        std::fs::write(d.join("STOP"), b"").unwrap();
        server.join().unwrap();
        let _ = std::fs::remove_dir_all(&d);
    }
}
