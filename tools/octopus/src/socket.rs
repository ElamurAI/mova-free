//! Private Unix-socket transport for the deputy: one bounded NDJSON request per connection,
//! answered sequentially; waiting requests are ordered by an optional `priority` ("urgent" before
//! "normal", arrival order within a class), and a running request is never interrupted, so a new
//! prompt cannot destroy unfinished earlier work. Only the owning user may connect (SO_PEERCRED); the socket lives in
//! an owned 0700 directory and is created 0600. The transport adds no capability: every request
//! goes through the same `Deputy::run` path, ledger reservation and refusals as the CLI.
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    os::unix::{
        fs::{DirBuilderExt, FileTypeExt, MetadataExt, PermissionsExt},
        io::AsRawFd,
        net::{UnixListener, UnixStream},
    },
    path::Path,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Request line bound: the 8192-byte prompt plus a small JSON envelope.
pub const REQUEST_MAX: usize = 16 * 1024;
/// Reply line bound; a larger report is replaced by an explicit refusal.
pub const RESPONSE_MAX: usize = 4 * 1024 * 1024;

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// The socket's directory must exist or be creatable as an owned 0700 directory.
pub fn private_dir(dir: &Path) -> Result<()> {
    ensure!(dir.is_absolute(), "socket directory must be absolute");
    if !dir.exists() {
        fs::DirBuilder::new().recursive(true).mode(0o700).create(dir)?;
    }
    let m = fs::symlink_metadata(dir)?;
    ensure!(
        m.is_dir() && m.uid() == unsafe { libc::geteuid() } && m.mode() & 0o077 == 0,
        "deputy socket directory must be owned and private (0700)"
    );
    Ok(())
}

fn peer_cred(stream: &UnixStream) -> Result<(u32, i32)> {
    let mut cred = libc::ucred { pid: 0, uid: 0, gid: 0 };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    let r = unsafe {
        libc::getsockopt(
            stream.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            &mut cred as *mut _ as *mut libc::c_void,
            &mut len,
        )
    };
    ensure!(r == 0, "SO_PEERCRED failed");
    Ok((cred.uid, cred.pid))
}

fn frame(stream: &UnixStream, max: usize) -> Result<Vec<u8>> {
    let mut line = Vec::new();
    BufReader::new(stream.try_clone()?)
        .take(max as u64 + 1)
        .read_until(b'\n', &mut line)?;
    ensure!(line.ends_with(b"\n"), "request must be one newline-terminated line within bound");
    ensure!(line.len() <= max, "request exceeds {max} bytes");
    line.pop();
    Ok(line)
}

/// Validate the request envelope:
/// `{"schema":1,"id":ID,"mode":"plan"|"run","prompt":TEXT,"priority"?:"urgent"|"normal","from"?:CALLER}`.
/// The prompt is arbitrary ordinary text; understanding it is the deputy's job, not the envelope's.
/// `from` names the asking animal (`caller_name`); it is a declaration by a same-owner process,
/// not an authentication, and servers that do not read it ignore it.
pub fn parse_request(v: &Value) -> Result<(String, bool, String)> {
    let o = v.as_object().context("request must be a JSON object")?;
    ensure!(
        o.keys().all(|k| ["schema", "id", "mode", "prompt", "priority", "from"].contains(&k.as_str())),
        "unknown request field"
    );
    ensure!(
        v.get("from").is_none_or(|f| f.as_str().is_some_and(caller_name)),
        "from must be a caller name (1..64 lowercase ASCII letters, digits, '-')"
    );
    ensure!(
        matches!(v.get("priority").and_then(Value::as_str), None | Some("urgent" | "normal"))
            && v.get("priority").is_none_or(Value::is_string),
        "priority must be urgent or normal"
    );
    ensure!(v["schema"] == 1, "schema must be 1");
    let id = v["id"].as_str().context("id string required")?;
    let mode = match v["mode"].as_str() {
        Some("plan") => false,
        Some("run") => true,
        _ => bail!("mode must be plan or run"),
    };
    let prompt = v["prompt"].as_str().context("prompt string required")?;
    Ok((id.to_owned(), mode, prompt.to_owned()))
}

/// A caller name: 1..64 lowercase ASCII letters, digits and `-`, starting with a letter
/// ("crab", "whale", "octopus").
pub fn caller_name(s: &str) -> bool {
    s.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && s.len() <= 64
        && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// One admitted request as a server handler sees it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub id: String,
    pub execute: bool,
    pub prompt: String,
    pub urgent: bool,
    /// The declared caller (`from`), if any.
    pub from: Option<String>,
    /// The connecting process (SO_PEERCRED), recorded as evidence of who asked.
    pub peer_pid: i32,
}

/// Serve until `dir/STOP` exists. `handle` receives `(id, execute, prompt)`.
pub fn serve(socket: &Path, handle: &dyn Fn(&str, bool, &str) -> Result<Value>) -> Result<()> {
    serve_prioritized(socket, &|id, execute, prompt, _| handle(id, execute, prompt))
}

/// As `serve`, and the handler also learns whether the request was urgent (to order work it
/// queues itself); the transport order is the same.
pub fn serve_prioritized(
    socket: &Path,
    handle: &dyn Fn(&str, bool, &str, bool) -> Result<Value>,
) -> Result<()> {
    serve_requests(socket, &|r: &Request| handle(&r.id, r.execute, &r.prompt, r.urgent))
}

/// As `serve_prioritized`, and the handler sees the whole admitted request, including the
/// declared caller and the connecting process.
pub fn serve_requests(socket: &Path, handle: &dyn Fn(&Request) -> Result<Value>) -> Result<()> {
    let dir = socket.parent().context("socket parent")?;
    private_dir(dir)?;
    if socket.exists() {
        let m = fs::symlink_metadata(socket)?;
        ensure!(
            m.file_type().is_socket() && m.uid() == unsafe { libc::geteuid() },
            "refusing foreign or non-socket endpoint"
        );
        ensure!(UnixStream::connect(socket).is_err(), "socket already has a listener");
        fs::remove_file(socket)?;
    }
    let listener = UnixListener::bind(socket)?;
    fs::set_permissions(socket, fs::Permissions::from_mode(0o600))?;
    listener.set_nonblocking(true)?;
    let me = unsafe { libc::geteuid() };
    let status = dir.join("status.json");
    let mut last = 0;
    let mut seq = 0u64;
    let mut queue: Vec<(bool, u64, UnixStream, Result<Request>)> = vec![];
    loop {
        if dir.join("STOP").exists() {
            break;
        }
        if now() >= last + 10 {
            last = now();
            let s = json!({"kind":"mova_octopus_deputy","state":"listening","pid":std::process::id(),"heartbeat_unix":last});
            let tmp = dir.join("status.part");
            fs::write(&tmp, serde_json::to_vec(&s)?)?;
            fs::rename(&tmp, &status)?;
        }
        // Drain every waiting connection, then answer them urgent-first; never preempt.
        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    stream.set_nonblocking(false)?;
                    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
                    stream.set_write_timeout(Some(Duration::from_secs(10)))?;
                    let request = admit(&stream, me);
                    let urgent = request.as_ref().is_ok_and(|r| r.urgent);
                    queue.push((!urgent, seq, stream, request));
                    seq += 1;
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e.into()),
            }
        }
        if queue.is_empty() {
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        queue.sort_by_key(|q| (q.0, q.1));
        let (_, _, mut stream, request) = queue.remove(0);
        let reply = match request {
            // A panicking handler fails only its own request; the server keeps serving.
            Ok(req) => match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| handle(&req)),
            ) {
                Ok(Ok(report)) => json!({"schema":1,"id":req.id,"ok":true,"urgent":req.urgent,"report":report}),
                Ok(Err(e)) => json!({"schema":1,"id":req.id,"ok":false,"error":format!("{e:#}")}),
                Err(_) => json!({"schema":1,"id":req.id,"ok":false,
                    "error":"handler panicked; request not retried, inspect the deputy ledger"}),
            },
            Err(e) => json!({"schema":1,"id":"","ok":false,"error":format!("{e:#}")}),
        };
        let mut bytes = serde_json::to_vec(&reply)?;
        if bytes.len() + 1 > RESPONSE_MAX {
            bytes = serde_json::to_vec(&json!({"schema":1,"id":reply["id"],"ok":false,
                "error":"report exceeds response budget; see the deputy ledger"}))?;
        }
        bytes.push(b'\n');
        let _ = stream.write_all(&bytes);
    }
    let _ = fs::remove_file(socket);
    let s = json!({"kind":"mova_octopus_deputy","state":"stopped","pid":std::process::id(),"heartbeat_unix":now()});
    fs::write(&status, serde_json::to_vec(&s)?)?;
    Ok(())
}

/// Owner check and bounded request read at accept time, before queueing.
fn admit(stream: &UnixStream, me: u32) -> Result<Request> {
    let (uid, peer_pid) = peer_cred(stream)?;
    ensure!(uid == me, "peer uid {uid} is not the owner");
    let v: Value = serde_json::from_slice(&frame(stream, REQUEST_MAX)?)?;
    let (id, execute, prompt) = parse_request(&v)?;
    let from = v.get("from").and_then(Value::as_str).map(str::to_owned);
    Ok(Request { id, execute, prompt, urgent: v["priority"] == "urgent", from, peer_pid })
}

fn write_status(dir: &Path, v: &Value) -> Result<()> {
    let tmp = dir.join("angel-status.part");
    fs::write(&tmp, serde_json::to_vec(v)?)?;
    fs::rename(&tmp, dir.join("angel-status.json"))?;
    Ok(())
}

/// Angel: keep the deputy server alive. Holds `angel.lock` (one angel per directory), builds the
/// handler with `make` and runs `serve`; a load failure, a serve error or a panic is recorded in
/// `angel-status.json` and retried after a doubling backoff (1 s .. `max_backoff`). Only `STOP`
/// ends it. In-flight work is never replayed: the deputy ledger keeps uncertain reservations.
pub fn angel<H: Fn(&str, bool, &str) -> Result<Value>>(
    socket: &Path,
    make: &dyn Fn() -> Result<H>,
    max_backoff: Duration,
) -> Result<()> {
    angel_prioritized(
        socket,
        &|| {
            let h = make()?;
            Ok(move |id: &str, execute: bool, prompt: &str, _: bool| h(id, execute, prompt))
        },
        max_backoff,
    )
}

/// As `angel`, serving with `serve_prioritized`.
pub fn angel_prioritized<H: Fn(&str, bool, &str, bool) -> Result<Value>>(
    socket: &Path,
    make: &dyn Fn() -> Result<H>,
    max_backoff: Duration,
) -> Result<()> {
    angel_requests(
        socket,
        &|| {
            let h = make()?;
            Ok(move |r: &Request| h(&r.id, r.execute, &r.prompt, r.urgent))
        },
        max_backoff,
    )
}

/// As `angel`, serving with `serve_requests`.
pub fn angel_requests<H: Fn(&Request) -> Result<Value>>(
    socket: &Path,
    make: &dyn Fn() -> Result<H>,
    max_backoff: Duration,
) -> Result<()> {
    let dir = socket.parent().context("socket parent")?;
    private_dir(dir)?;
    let lock = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(dir.join("angel.lock"))?;
    ensure!(
        unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0,
        "another deputy angel holds the lease"
    );
    let mut restarts = 0u64;
    let mut backoff = Duration::from_secs(1);
    loop {
        if dir.join("STOP").exists() {
            write_status(dir, &json!({"kind":"mova_octopus_angel","state":"stopped","restarts":restarts,"heartbeat_unix":now()}))?;
            return Ok(());
        }
        write_status(dir, &json!({"kind":"mova_octopus_angel","state":"serving","pid":std::process::id(),"restarts":restarts,"heartbeat_unix":now()}))?;
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let handler = make()?;
            serve_requests(socket, &handler)
        }));
        let error = match outcome {
            Ok(Ok(())) => continue, // STOP observed by serve; the loop head records it
            Ok(Err(e)) => format!("{e:#}"),
            Err(_) => "deputy server panicked".to_owned(),
        };
        restarts += 1;
        write_status(dir, &json!({"kind":"mova_octopus_angel","state":"restarting","error":error,"restarts":restarts,"backoff_seconds":backoff.as_secs_f64(),"heartbeat_unix":now()}))?;
        let until = std::time::Instant::now() + backoff;
        while std::time::Instant::now() < until && !dir.join("STOP").exists() {
            thread::sleep(Duration::from_millis(50));
        }
        backoff = (backoff * 2).min(max_backoff);
    }
}

/// Client: send one request and return the reply line.
pub fn ask(socket: &Path, id: &str, execute: bool, prompt: &str) -> Result<Value> {
    ask_with(socket, id, execute, prompt, false)
}
/// Client with an explicit priority; urgent requests are answered before waiting normal ones.
pub fn ask_with(socket: &Path, id: &str, execute: bool, prompt: &str, urgent: bool) -> Result<Value> {
    ask_from(socket, id, execute, prompt, urgent, None, None)
}
/// Client that names its caller (`from`) and may bound the wait for the reply.
pub fn ask_from(
    socket: &Path,
    id: &str,
    execute: bool,
    prompt: &str,
    urgent: bool,
    from: Option<&str>,
    read_timeout: Option<Duration>,
) -> Result<Value> {
    ensure!(from.is_none_or(caller_name), "invalid caller name");
    let mut s = UnixStream::connect(socket)
        .with_context(|| format!("connect {}", socket.display()))?;
    s.set_write_timeout(Some(Duration::from_secs(5)))?;
    s.set_read_timeout(read_timeout)?;
    let mut req = json!({"schema":1,"id":id,"mode":if execute {"run"} else {"plan"},"prompt":prompt,
        "priority":if urgent {"urgent"} else {"normal"}});
    if let Some(f) = from {
        req["from"] = json!(f);
    }
    let mut line = serde_json::to_vec(&req)?;
    ensure!(line.len() < REQUEST_MAX, "request exceeds {REQUEST_MAX} bytes");
    line.push(b'\n');
    s.write_all(&line)?;
    let mut reply = Vec::new();
    BufReader::new(s)
        .take(RESPONSE_MAX as u64 + 1)
        .read_until(b'\n', &mut reply)?;
    ensure!(reply.ends_with(b"\n"), "reply missing or over bound");
    Ok(serde_json::from_slice(&reply)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(0);
    fn dir() -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!(
            "octopus-socket-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&d);
        d
    }
    fn echo(id: &str, execute: bool, prompt: &str) -> Result<Value> {
        ensure!(prompt != "fail", "handler refused");
        Ok(json!({"id":id,"execute":execute,"prompt":prompt}))
    }
    #[test]
    fn private_socket_round_trip_bounds_and_stop() {
        let d = dir();
        let sock = d.join("deputy.sock");
        let s2 = sock.clone();
        let server = thread::spawn(move || serve(&s2, &echo).unwrap());
        for _ in 0..100 {
            if sock.exists() {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(fs::metadata(&d).unwrap().mode() & 0o777, 0o700);
        assert_eq!(fs::metadata(&sock).unwrap().mode() & 0o777, 0o600);
        let r = ask(&sock, "inv-1", false, "Ask Crab.").unwrap();
        assert_eq!(r["ok"], true);
        assert_eq!(r["report"], json!({"id":"inv-1","execute":false,"prompt":"Ask Crab."}));
        assert_eq!(ask(&sock, "inv-2", true, "x").unwrap()["report"]["execute"], true);
        let r = ask(&sock, "inv-3", true, "fail").unwrap();
        assert_eq!((r["ok"].clone(), r["id"].clone()), (json!(false), json!("inv-3")));
        // Unknown field, wrong mode and an over-bound line are refused without reaching the handler.
        for bad in [
            json!({"schema":1,"id":"a","mode":"plan","prompt":"x","shell":"rm"}),
            json!({"schema":1,"id":"a","mode":"exec","prompt":"x"}),
        ] {
            let mut s = UnixStream::connect(&sock).unwrap();
            s.write_all(format!("{bad}\n").as_bytes()).unwrap();
            let mut reply = String::new();
            BufReader::new(s).read_line(&mut reply).unwrap();
            let v: Value = serde_json::from_str(&reply).unwrap();
            assert_eq!(v["ok"], false, "{bad}");
        }
        assert!(ask(&sock, "big", false, &"x".repeat(REQUEST_MAX)).is_err());
        let mut s = UnixStream::connect(&sock).unwrap();
        let _ = s.write_all(&vec![b'a'; REQUEST_MAX + 10]);
        let mut reply = String::new();
        let _ = BufReader::new(s).read_line(&mut reply);
        assert!(reply.contains("\"ok\":false"));
        fs::write(d.join("STOP"), b"").unwrap();
        server.join().unwrap();
        assert!(!sock.exists());
        let _ = fs::remove_dir_all(&d);
    }
    #[test]
    fn urgent_waits_for_running_work_then_precedes_normal() {
        use std::sync::{Arc, Mutex};
        let d = dir();
        let sock = d.join("deputy.sock");
        let order = Arc::new(Mutex::new(Vec::<String>::new()));
        let (s2, o2) = (sock.clone(), order.clone());
        let server = thread::spawn(move || {
            serve(&s2, &move |id, _, _| {
                if id == "first" {
                    thread::sleep(Duration::from_millis(800));
                }
                o2.lock().unwrap().push(id.to_owned());
                Ok(json!({}))
            })
            .unwrap()
        });
        while !sock.exists() {
            thread::sleep(Duration::from_millis(20));
        }
        let s = sock.clone();
        let first = thread::spawn(move || ask(&s, "first", false, "a").unwrap());
        thread::sleep(Duration::from_millis(250));
        let s = sock.clone();
        let normal = thread::spawn(move || ask(&s, "normal", false, "b").unwrap());
        thread::sleep(Duration::from_millis(100));
        let s = sock.clone();
        let urgent = thread::spawn(move || ask_with(&s, "urgent", false, "c", true).unwrap());
        assert_eq!(first.join().unwrap()["ok"], true);
        assert_eq!(urgent.join().unwrap()["urgent"], true);
        assert_eq!(normal.join().unwrap()["urgent"], false);
        assert_eq!(*order.lock().unwrap(), ["first", "urgent", "normal"]);
        fs::write(d.join("STOP"), b"").unwrap();
        server.join().unwrap();
        let _ = fs::remove_dir_all(&d);
    }
    #[test]
    fn angel_survives_load_failures_and_handler_panics() {
        use std::sync::atomic::AtomicU32;
        let d = dir();
        let sock = d.join("deputy.sock");
        let s2 = sock.clone();
        static LOADS: AtomicU32 = AtomicU32::new(0);
        let angel_thread = thread::spawn(move || {
            angel(
                &s2,
                &|| {
                    ensure!(LOADS.fetch_add(1, Ordering::SeqCst) >= 2, "model not ready");
                    Ok(|id: &str, _: bool, p: &str| {
                        if p == "panic" {
                            panic!("boom");
                        }
                        Ok(json!({"id":id}))
                    })
                },
                Duration::from_millis(200),
            )
            .unwrap()
        });
        while !sock.exists() {
            thread::sleep(Duration::from_millis(20));
        }
        let status: Value = serde_json::from_slice(&fs::read(d.join("angel-status.json")).unwrap()).unwrap();
        assert_eq!(status["restarts"], 2);
        let r = ask(&sock, "p", true, "panic").unwrap();
        assert_eq!(r["ok"], false);
        assert_eq!(ask(&sock, "after", true, "x").unwrap()["report"]["id"], "after");
        // A second angel on the same directory is refused by the lease.
        assert!(angel(&sock, &|| Ok(echo), Duration::from_millis(10)).is_err());
        fs::write(d.join("STOP"), b"").unwrap();
        angel_thread.join().unwrap();
        let status: Value = serde_json::from_slice(&fs::read(d.join("angel-status.json")).unwrap()).unwrap();
        assert_eq!(status["state"], "stopped");
        let _ = fs::remove_dir_all(&d);
    }
    #[test]
    fn non_private_directory_is_refused() {
        let d = dir();
        fs::create_dir_all(&d).unwrap();
        fs::set_permissions(&d, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(serve(&d.join("deputy.sock"), &echo).is_err());
        assert!(!d.join("deputy.sock").exists());
        let _ = fs::remove_dir_all(&d);
    }
    #[test]
    fn declared_caller_and_peer_reach_the_handler() {
        let d = dir();
        let sock = d.join("deputy.sock");
        let s2 = sock.clone();
        let server = thread::spawn(move || {
            serve_requests(&s2, &|r: &Request| Ok(json!({"from":r.from,"pid":r.peer_pid,"urgent":r.urgent}))).unwrap()
        });
        while !sock.exists() {
            thread::sleep(Duration::from_millis(20));
        }
        let r = ask_from(&sock, "a", true, "x", true, Some("crab"), Some(Duration::from_secs(5))).unwrap();
        assert_eq!(r["report"]["from"], "crab");
        assert_eq!(r["report"]["pid"], std::process::id());
        assert_eq!(r["report"]["urgent"], true);
        assert_eq!(ask(&sock, "b", true, "x").unwrap()["report"]["from"], Value::Null);
        // A malformed caller name is refused by the envelope, before the handler.
        assert!(ask_from(&sock, "c", true, "x", false, Some("Crab Bot"), None).is_err());
        let mut s = UnixStream::connect(&sock).unwrap();
        s.write_all(b"{\"schema\":1,\"id\":\"c\",\"mode\":\"run\",\"prompt\":\"x\",\"from\":\"../crab\"}\n").unwrap();
        let mut reply = String::new();
        BufReader::new(s).read_line(&mut reply).unwrap();
        assert!(reply.contains("\"ok\":false") && reply.contains("caller name"), "{reply}");
        fs::write(d.join("STOP"), b"").unwrap();
        server.join().unwrap();
        let _ = fs::remove_dir_all(&d);
    }
}
