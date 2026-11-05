//! Asking another animal in English: relay one English request to that animal's private socket
//! and bring back its English answer.
//!
//! Two entry points share one client path (`ask_animal`):
//! - `AnimalRelay`, the deputy capability handler `animal_ask`. A deputy (Crab) is configured with
//!   a capability whose trusted parameters name the animal, its socket and the caller name this
//!   deputy declares; the prompt supplies only the quoted English request (one `task` slot of kind
//!   `text`): "Ask Sea Spider: «Are you busy?»". The socket is never taken from a prompt.
//! - `ask_animal`, a library function for animals that hold a configured socket of their own
//!   (the Whales).
//!
//! No new command language: the relayed text is ordinary English and the answer is the other
//! animal's English. The caller name is a declaration by a same-owner process, checked by the
//! asked animal against its own allowlist; it is not an authentication.
use crate::socket::{ask_from, caller_name};
use crate::{Action, CapabilityHandler, SlotKind, SlotRule, digest};
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;

/// The handler name a deputy configuration uses for relayed requests.
pub const HANDLER: &str = "animal_ask";
/// How long a relay waits for the other animal's answer. Animals answer from their journals and
/// queue long work, so a longer wait means the other side is stuck.
pub const ANSWER_WAIT: Duration = Duration::from_secs(30);

/// The other animal's reply: its English answer and the whole reply line.
#[derive(Clone, Debug)]
pub struct Reply {
    pub ok: bool,
    /// The English answer, or the transport-level error when `ok` is false.
    pub answer: String,
    pub raw: Value,
}

/// Send one English request to the animal listening on `socket`, declaring `caller`.
/// The request id makes the call idempotent on the other side (a repeated id returns the
/// recorded answer).
pub fn ask_animal(socket: &Path, caller: &str, id: &str, english: &str, urgent: bool) -> Result<Reply> {
    ensure!(socket.is_absolute(), "the animal's socket must be an absolute configured path");
    ensure!(caller_name(caller), "invalid caller name {caller:?}");
    ensure!(!english.trim().is_empty(), "nothing to ask");
    let raw = ask_from(socket, id, true, english, urgent, Some(caller), Some(ANSWER_WAIT))?;
    let ok = raw["ok"] == true;
    let answer = if ok {
        raw["report"]["answer"].as_str().context("the animal's reply has no English answer")?.to_owned()
    } else {
        raw["error"].as_str().unwrap_or("the animal refused the request without a reason").to_owned()
    };
    Ok(Reply { ok, answer, raw })
}

/// Check an `animal_ask` capability: parameters `animal` (a display name), `socket` (absolute)
/// and `caller` (a caller name), nothing else; exactly one slot `task` of kind text, 1..1.
pub fn validate(p: &Value, slots: &[SlotRule]) -> Result<()> {
    let o = p.as_object().context("animal_ask parameters must be an object")?;
    ensure!(
        o.keys().all(|k| ["animal", "socket", "caller"].contains(&k.as_str())),
        "animal_ask parameters are animal, socket and caller only"
    );
    let animal = p["animal"].as_str().context("animal_ask needs the animal's name")?;
    ensure!(
        !animal.is_empty() && animal.len() <= 64 && animal.bytes().all(|b| b.is_ascii_alphanumeric() || b == b' ' || b == b'-'),
        "invalid animal name"
    );
    let socket = p["socket"].as_str().context("animal_ask needs the configured socket of the animal")?;
    ensure!(Path::new(socket).is_absolute() && socket.len() <= 1024 && !socket.contains("/../"), "the socket must be an absolute path");
    ensure!(p["caller"].as_str().is_some_and(caller_name), "animal_ask needs a caller name");
    ensure!(
        slots.len() == 1 && slots[0].name == "task" && slots[0].kinds == [SlotKind::Text] && slots[0].min == 1 && slots[0].max == 1,
        "animal_ask needs exactly one slot: task, kind text, 1..1"
    );
    Ok(())
}

/// The deputy capability handler `animal_ask`.
pub struct AnimalRelay;

impl CapabilityHandler for AnimalRelay {
    fn execute(&self, p: &Value, a: &Action, invocation_id: &str) -> Result<Value> {
        // Parameters were validated with the configuration; check the trusted ones again here so
        // a handler installed under another configuration cannot be pointed elsewhere.
        let socket = p["socket"].as_str().filter(|s| Path::new(s).is_absolute()).context("no configured socket for this animal")?;
        let caller = p["caller"].as_str().context("no configured caller name")?;
        let animal = p["animal"].as_str().unwrap_or("the animal");
        let [task] = a.arguments.get("task").map(Vec::as_slice).unwrap_or_default() else {
            anyhow::bail!("asking {animal} needs exactly one quoted request bound by the parse");
        };
        let r = ask_animal(Path::new(socket), caller, invocation_id, task, false)
            .with_context(|| format!("I could not reach {animal}"))?;
        ensure!(r.ok, "{animal} did not answer: {}", r.answer);
        Ok(json!({"animal":animal,"socket":socket,"caller":caller,"request_id":invocation_id,"request":task,
            "answer":r.answer,"failure":r.raw["report"]["report"]["failure"],
            "reply_sha256":digest(&serde_json::to_vec(&r.raw)?),"transport":"animal_socket"}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::socket::{Request, serve_requests};
    use std::collections::BTreeMap;

    fn rule(name: &str, kinds: Vec<SlotKind>) -> SlotRule {
        SlotRule { name: name.into(), kinds, min: 1, max: 1 }
    }
    fn dir(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("octopus-relay-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn configuration_requires_a_configured_socket() {
        let task = vec![rule("task", vec![SlotKind::Text])];
        let good = json!({"animal":"Sea Spider","socket":"/run/x/seaspider.sock","caller":"crab"});
        assert!(validate(&good, &task).is_ok());
        // Red controls: no socket, a relative socket, an extra parameter, a bad caller, wrong slots.
        for bad in [
            json!({"animal":"Sea Spider","caller":"crab"}),
            json!({"animal":"Sea Spider","socket":"seaspider.sock","caller":"crab"}),
            json!({"animal":"Sea Spider","socket":"/a/../b.sock","caller":"crab"}),
            json!({"animal":"Sea Spider","socket":"/a/s.sock","caller":"crab","argv":["sh"]}),
            json!({"animal":"Sea Spider","socket":"/a/s.sock","caller":"Crab!"}),
            json!({"animal":"Sea Spider","socket":"/a/s.sock"}),
        ] {
            assert!(validate(&bad, &task).is_err(), "{bad}");
        }
        assert!(validate(&good, &[]).is_err());
        assert!(validate(&good, &[rule("task", vec![SlotKind::Url])]).is_err());
        // The handler itself refuses parameters without a socket.
        let a = Action {
            capability: "ask".into(),
            matched_entities: vec![],
            matched_observations: vec![],
            native_request_heads: vec![],
            native_observation_nodes: vec![],
            arguments: BTreeMap::from([("task".into(), vec!["Are you busy?".into()])]),
            native_argument_nodes: vec![],
        };
        assert!(AnimalRelay.execute(&json!({"animal":"Sea Spider","caller":"crab"}), &a, "x-1").is_err());
    }

    #[test]
    fn relays_english_with_the_declared_caller() {
        let d = dir("serve");
        let sock = d.join("animal.sock");
        let s2 = sock.clone();
        let server = std::thread::spawn(move || {
            serve_requests(&s2, &|r: &Request| {
                Ok(json!({"answer":format!("{} asked me: {}", r.from.as_deref().unwrap_or("nobody"), r.prompt),"report":{"failure":null}}))
            })
            .unwrap()
        });
        while !sock.exists() {
            std::thread::sleep(Duration::from_millis(20));
        }
        let a = Action {
            capability: "ask".into(),
            matched_entities: vec!["spider".into()],
            matched_observations: vec![],
            native_request_heads: vec![],
            native_observation_nodes: vec![],
            arguments: BTreeMap::from([("task".into(), vec!["Are you busy?".into()])]),
            native_argument_nodes: vec![],
        };
        let p = json!({"animal":"Sea Spider","socket":sock,"caller":"crab"});
        let out = AnimalRelay.execute(&p, &a, "octopus-r1-0").unwrap();
        assert_eq!(out["answer"], "crab asked me: Are you busy?");
        assert_eq!(out["request_id"], "octopus-r1-0");
        let r = ask_animal(&sock, "whale", "w-1", "How many pages have you stored?", false).unwrap();
        assert!(r.ok);
        assert_eq!(r.answer, "whale asked me: How many pages have you stored?");
        // An unreachable configured socket is an error, not a silent empty answer.
        let p = json!({"animal":"Sea Spider","socket":d.join("missing.sock"),"caller":"crab"});
        assert!(AnimalRelay.execute(&p, &a, "octopus-r2-0").is_err());
        std::fs::write(d.join("STOP"), b"").unwrap();
        server.join().unwrap();
        let _ = std::fs::remove_dir_all(&d);
    }
}
