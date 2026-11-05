//! End-to-end checks against a local HTTP server on 127.0.0.1 (no external network).
//! Host names a.test, b.test, c.test are pinned to 127.0.0.1 through `network.resolve`, so they
//! share one IP and therefore one 2-second limit.

use seaspider::config::Config;
use seaspider::crawl::Crawler;
use seaspider::store::{Reader, Record};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
struct Req {
    host: String,
    path: String,
    ua: String,
    at: Instant,
}

#[derive(Default)]
struct Resp {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
    stall: Option<Duration>,
}

fn ok(body: &str) -> Resp {
    Resp { status: 200, headers: vec![("Content-Type".into(), "text/html; charset=utf-8".into())], body: body.as_bytes().to_vec(), stall: None }
}

fn text(status: u16, body: &str) -> Resp {
    Resp { status, headers: vec![("Content-Type".into(), "text/plain".into())], body: body.as_bytes().to_vec(), stall: None }
}

fn redirect(status: u16, to: &str) -> Resp {
    Resp { status, headers: vec![("Location".into(), to.into())], body: Vec::new(), stall: None }
}

type Handler = Arc<dyn Fn(&Req, u16) -> Resp + Send + Sync>;

struct Server {
    port: u16,
    log: Arc<Mutex<Vec<Req>>>,
}

impl Server {
    fn start(h: Handler) -> Server {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = l.local_addr().unwrap().port();
        let log: Arc<Mutex<Vec<Req>>> = Arc::default();
        let lg = log.clone();
        std::thread::spawn(move || {
            for s in l.incoming() {
                let Ok(s) = s else { continue };
                let (h, lg) = (h.clone(), lg.clone());
                std::thread::spawn(move || handle(s, h, lg, port));
            }
        });
        Server { port, log }
    }

    fn url(&self, host: &str, path: &str) -> String {
        format!("http://{host}:{}{path}", self.port)
    }

    fn seen(&self) -> Vec<Req> {
        self.log.lock().unwrap().clone()
    }

    fn paths(&self) -> Vec<String> {
        self.seen().iter().map(|r| format!("{}{}", r.host, r.path)).collect()
    }
}

fn handle(mut s: TcpStream, h: Handler, log: Arc<Mutex<Vec<Req>>>, port: u16) {
    let at = Instant::now();
    let mut buf = Vec::new();
    let mut b = [0u8; 4096];
    while !buf.windows(4).any(|w| w == b"\r\n\r\n") && buf.len() < 65536 {
        match s.read(&mut b) {
            Ok(0) | Err(_) => return,
            Ok(n) => buf.extend_from_slice(&b[..n]),
        }
    }
    let head = String::from_utf8_lossy(&buf).to_string();
    let mut lines = head.split("\r\n");
    let path = lines.next().unwrap_or("").split(' ').nth(1).unwrap_or("/").to_string();
    let (mut host, mut ua) = (String::new(), String::new());
    for l in lines {
        if let Some((k, v)) = l.split_once(':') {
            match k.trim().to_ascii_lowercase().as_str() {
                "host" => host = v.trim().split(':').next().unwrap_or("").to_string(),
                "user-agent" => ua = v.trim().to_string(),
                _ => {}
            }
        }
    }
    let req = Req { host, path, ua, at };
    log.lock().unwrap().push(req.clone());
    let r = h(&req, port);
    let reason = match r.status {
        200 => "OK",
        301 => "Moved Permanently",
        302 => "Found",
        404 => "Not Found",
        503 => "Service Unavailable",
        _ => "Status",
    };
    let mut out = format!("HTTP/1.1 {} {reason}\r\nConnection: close\r\n", r.status);
    for (k, v) in &r.headers {
        out.push_str(&format!("{k}: {v}\r\n"));
    }
    if let Some(d) = r.stall {
        out.push_str(&format!("Content-Length: {}\r\n\r\n", r.body.len() + 100));
        let _ = s.write_all(out.as_bytes());
        let _ = s.write_all(&r.body);
        let _ = s.flush();
        std::thread::sleep(d);
        return;
    }
    out.push_str(&format!("Content-Length: {}\r\n\r\n", r.body.len()));
    let _ = s.write_all(out.as_bytes());
    let _ = s.write_all(&r.body);
    let _ = s.flush();
}

fn tmp(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("seaspider-it-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    p
}

fn merge(a: &mut serde_json::Value, b: serde_json::Value) {
    match (a, b) {
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k).or_insert(serde_json::Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

fn config(store: &Path, extra: serde_json::Value) -> Config {
    let mut v = serde_json::json!({
        "robot": {"contact_url": "https://example.invalid/movabot"},
        "store": store.to_str().unwrap(),
        "fetch": {"connect_timeout_ms": 2000, "timeout_ms": 4000},
        "network": {"allow_private_addresses": true, "resolve": {"a.test": "127.0.0.1", "b.test": "127.0.0.1", "c.test": "127.0.0.1"}},
    });
    merge(&mut v, extra);
    serde_json::from_value(v).unwrap()
}

async fn crawl(store: &Path, extra: serde_json::Value, urls: &[String]) -> (seaspider::crawl::Summary, Crawler) {
    let mut c = Crawler::new(config(store, extra), store).unwrap();
    let s = c.run(urls).await.unwrap();
    (s, c)
}

fn pages(store: &Path) -> Vec<Record> {
    Reader::open(store).unwrap().index.recs.into_iter().filter(|r| r.kind == "page").collect()
}

fn outcome(store: &Path, url: &str) -> Record {
    pages(store).into_iter().rev().find(|r| r.url == url).unwrap_or_else(|| panic!("no record for {url}"))
}

/// Every consecutive pair of arrivals at the one IP is at least `gap` apart.
fn assert_spacing(reqs: &[Req], gap: Duration) {
    let mut t: Vec<Instant> = reqs.iter().map(|r| r.at).collect();
    t.sort();
    for w in t.windows(2) {
        let d = w[1] - w[0];
        assert!(d >= gap, "two requests to one IP only {d:?} apart (need {gap:?})");
    }
}

#[tokio::test]
async fn robots_disallow_longest_match_and_wildcards() {
    let srv = Server::start(Arc::new(|r: &Req, _| match r.path.as_str() {
        "/robots.txt" => text(200, "User-agent: *\nDisallow: /private\nAllow: /private/open\nDisallow: /*.pdf$\n\nUser-agent: OtherBot\nDisallow: /\n"),
        _ => ok("<html>page</html>"),
    }));
    let d = tmp("robots");
    let urls: Vec<String> = ["/public", "/private/x", "/private/open/y", "/doc.pdf"].iter().map(|p| srv.url("a.test", p)).collect();
    let (s, _) = crawl(&d, serde_json::json!({}), &urls).await;
    assert_eq!((s.fetched, s.disallowed), (2, 2));
    let seen = srv.paths();
    assert!(!seen.iter().any(|p| p.contains("/private/x") || p.contains(".pdf")), "a disallowed path reached the server: {seen:?}");
    assert!(seen.iter().any(|p| p == "a.test/private/open/y"), "the longer Allow wins");
    let r = outcome(&d, &urls[1]);
    assert_eq!(r.outcome, "disallowed");
    assert_eq!(r.robots.as_ref().unwrap().rule.as_deref(), Some("Disallow: /private"));
    assert_eq!(r.robots.as_ref().unwrap().group, "*");
    let ua = format!("MovaBot/{} (+https://example.invalid/movabot)", env!("CARGO_PKG_VERSION"));
    assert!(srv.seen().iter().all(|q| q.ua == ua), "User-Agent of the abstract robot");
    assert_spacing(&srv.seen(), Duration::from_secs(2));
}

#[tokio::test]
async fn own_group_and_crawl_delay() {
    let srv = Server::start(Arc::new(|r: &Req, _| match (r.host.as_str(), r.path.as_str()) {
        ("a.test", "/robots.txt") => text(200, "User-agent: *\nDisallow: /\n\nUser-agent: MovaBot\nAllow: /\nCrawl-delay: 3\n"),
        ("b.test", "/robots.txt") => text(200, "User-agent: *\nCrawl-delay: 100\n"),
        _ => ok("<html>p</html>"),
    }));
    let d = tmp("delay");
    let urls = vec![srv.url("a.test", "/1"), srv.url("a.test", "/2"), srv.url("b.test", "/x")];
    let (s, _) = crawl(&d, serde_json::json!({}), &urls).await;
    assert_eq!((s.fetched, s.deferred), (2, 1));
    assert_eq!(outcome(&d, &urls[0]).robots.unwrap().group, "own");
    let a: Vec<Req> = srv.seen().into_iter().filter(|r| r.host == "a.test" && r.path != "/robots.txt").collect();
    assert_eq!(a.len(), 2);
    let gap = a[1].at - a[0].at;
    assert!(gap >= Duration::from_secs(3), "Crawl-delay 3 honoured: {gap:?}");
    assert!(!srv.paths().contains(&"b.test/x".to_string()), "Crawl-delay above the cap is deferred, not fetched");
}

#[tokio::test]
async fn robots_unavailable_unreachable_and_network_error() {
    let srv = Server::start(Arc::new(|r: &Req, _| match (r.host.as_str(), r.path.as_str()) {
        ("a.test", "/robots.txt") => text(404, "no"),
        ("b.test", "/robots.txt") => text(503, "busy"),
        _ => ok("<html>p</html>"),
    }));
    // a port with nothing listening
    let closed = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let d = tmp("unreach");
    let urls = vec![srv.url("a.test", "/x"), srv.url("b.test", "/y"), format!("http://c.test:{closed}/z")];
    let (s, _) = crawl(&d, serde_json::json!({}), &urls).await;
    assert_eq!((s.fetched, s.disallowed), (1, 2));
    assert!(outcome(&d, &urls[0]).robots.unwrap().source.starts_with("allow_all (HTTP 404"));
    let b = outcome(&d, &urls[1]);
    assert!(!b.robots.as_ref().unwrap().allowed && b.robots.unwrap().source.contains("HTTP 503"));
    assert!(!srv.paths().contains(&"b.test/y".to_string()), "5xx robots.txt closes the origin");
    let c = outcome(&d, &urls[2]);
    assert_eq!(c.outcome, "disallowed");
    assert!(c.robots.unwrap().source.starts_with("unreachable"));
}

#[tokio::test]
async fn per_ip_two_second_spacing_across_hosts() {
    let srv = Server::start(Arc::new(|r: &Req, _| match r.path.as_str() {
        "/robots.txt" => text(404, ""),
        _ => ok("<html>p</html>"),
    }));
    let d = tmp("spacing");
    let urls = vec![srv.url("a.test", "/1"), srv.url("b.test", "/1"), srv.url("a.test", "/2")];
    let (s, c) = crawl(&d, serde_json::json!({}), &urls).await;
    assert_eq!(s.fetched, 3);
    let seen = srv.seen();
    assert_eq!(seen.len(), 5, "two robots.txt and three pages");
    assert_spacing(&seen, Duration::from_secs(2));
    // the client side: start of each request >= end of the previous + 2 s
    for w in c.requests.windows(2) {
        assert!(w[1].start >= w[0].end + Duration::from_secs(2), "{:?} -> {:?}", w[0].url, w[1].url);
    }
    // deterministic round-robin order: a/1, b/1, a/2
    let order: Vec<String> = pages(&d).iter().map(|r| r.url.clone()).collect();
    assert_eq!(order, urls[..].iter().map(|u| u.to_string()).collect::<Vec<_>>());
}

#[tokio::test]
async fn redirects_are_rechecked() {
    let srv = Server::start(Arc::new(|r: &Req, port| match (r.host.as_str(), r.path.as_str()) {
        ("a.test", "/robots.txt") => text(404, ""),
        ("b.test", "/robots.txt") => text(200, "User-agent: *\nDisallow: /secret\n"),
        ("a.test", "/r") => redirect(302, &format!("http://b.test:{port}/secret")),
        ("a.test", "/r2") => redirect(301, &format!("http://b.test:{port}/open#frag")),
        _ => ok("<html>p</html>"),
    }));
    let d = tmp("redirect");
    let urls = vec![srv.url("a.test", "/r"), srv.url("a.test", "/r2")];
    let (s, _) = crawl(&d, serde_json::json!({}), &urls).await;
    assert_eq!((s.fetched, s.disallowed), (1, 1));
    assert!(!srv.paths().contains(&"b.test/secret".to_string()), "a disallowed redirect target is not requested");
    let r = outcome(&d, &urls[0]);
    assert_eq!(r.chain.len(), 1);
    assert_eq!(r.chain[0].status, 302);
    assert!(r.error.unwrap().contains("redirect target"));
    let r2 = outcome(&d, &urls[1]);
    assert_eq!(r2.outcome, "fetched");
    assert_eq!(r2.final_url.as_deref(), Some(srv.url("b.test", "/open").as_str()));
    assert_eq!(r2.chain.iter().map(|h| h.status).collect::<Vec<_>>(), vec![301, 200]);
    assert_spacing(&srv.seen(), Duration::from_secs(2));
}

#[tokio::test]
async fn body_size_timeout_gzip_and_redirect_bounds() {
    let srv = Server::start(Arc::new(|r: &Req, _| match r.path.as_str() {
        "/robots.txt" => text(404, ""),
        "/big" => ok(&"x".repeat(100_000)),
        "/slow" => Resp { status: 200, headers: vec![], body: b"partial".to_vec(), stall: Some(Duration::from_secs(6)) },
        "/gz" => {
            let mut e = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
            e.write_all(b"<html>decoded body</html>").unwrap();
            Resp { status: 200, headers: vec![("Content-Type".into(), "text/html".into()), ("Content-Encoding".into(), "gzip".into())], body: e.finish().unwrap(), stall: None }
        }
        "/l1" => redirect(302, "/l2"),
        "/l2" => redirect(302, "/l3"),
        _ => ok("<html>p</html>"),
    }));
    let d = tmp("bounds");
    let urls: Vec<String> = ["/big", "/slow", "/gz", "/l1"].iter().map(|p| srv.url("a.test", p)).collect();
    let extra = serde_json::json!({"fetch": {"max_body_bytes": 10000, "timeout_ms": 1500, "max_redirects": 1}});
    let (_, _) = crawl(&d, extra, &urls).await;
    let big = outcome(&d, &urls[0]);
    assert!(big.truncated);
    assert_eq!(big.body_bytes, Some(10000));
    let slow = outcome(&d, &urls[1]);
    assert_eq!(slow.outcome, "error");
    assert!(slow.error.unwrap().contains("timed out"));
    let r = Reader::open(&d).unwrap();
    assert_eq!(r.get(&urls[2]).unwrap().1, b"<html>decoded body</html>", "stored decoded");
    let l = outcome(&d, &urls[3]);
    assert_eq!(l.chain.len(), 2);
    assert!(l.error.unwrap().contains("more than 1 redirects"));
}

fn projection(d: &Path) -> Vec<String> {
    Reader::open(d)
        .unwrap()
        .index
        .recs
        .iter()
        .map(|r| format!("{} {} {} {:?} {:?} {:?} {:?} {:?} {:?}", r.kind, r.url, r.outcome, r.status, r.final_url, r.sha256, (r.seg, r.off, r.len), r.robots.as_ref().map(|n| (n.allowed, n.rule.clone())), r.chain.iter().map(|h| h.status).collect::<Vec<_>>()))
        .collect()
}

#[tokio::test]
async fn resume_skips_and_runs_are_identical() {
    let srv = Server::start(Arc::new(|r: &Req, _| match r.path.as_str() {
        "/robots.txt" => text(200, "User-agent: *\nDisallow: /no\n"),
        p => ok(&format!("<html><a href=\"/next\">n</a>{p}</html>")),
    }));
    let urls: Vec<String> = ["/p1", "/no", "/p1", "/p2"].iter().map(|p| srv.url("a.test", p)).collect();
    let (d1, d2) = (tmp("det1"), tmp("det2"));
    let (s1, _) = crawl(&d1, serde_json::json!({}), &urls).await;
    let (s2, _) = crawl(&d2, serde_json::json!({}), &urls).await;
    assert_eq!((s1.fetched, s1.disallowed, s1.duplicates), (2, 1, 1));
    assert_eq!((s2.fetched, s2.disallowed, s2.duplicates), (2, 1, 1));
    assert_eq!(projection(&d1), projection(&d2), "two runs give identical journals (timestamps aside)");
    assert_eq!(std::fs::read(d1.join("links.jsonl")).unwrap(), std::fs::read(d2.join("links.jsonl")).unwrap());
    let before = srv.seen().len();
    // an interrupted fetch: an intent with no outcome
    let intent = Record { v: 1, kind: "intent".into(), url: srv.url("a.test", "/p3"), outcome: "intent".into(), fetched_unix: 1, ..Default::default() };
    let mut j = std::fs::OpenOptions::new().append(true).open(d1.join("pages.jsonl")).unwrap();
    writeln!(j, "{}", serde_json::to_string(&intent).unwrap()).unwrap();
    let mut again = urls.clone();
    again.push(srv.url("a.test", "/p3"));
    let (s3, _) = crawl(&d1, serde_json::json!({}), &again).await;
    assert_eq!(s3.skipped_done, 3, "p1, no, p2 are done");
    assert_eq!(s3.uncertain_recorded, 1);
    assert_eq!(s3.fetched, 1, "the uncertain URL is fetched again");
    let new: Vec<String> = srv.seen()[before..].iter().map(|r| r.path.clone()).collect();
    assert_eq!(new, vec!["/robots.txt", "/p3"]);
    // refetch age: everything is old after 0 s
    let (s4, _) = crawl(&d1, serde_json::json!({"resume": {"refetch_after_secs": 0}}), &urls[..1]).await;
    assert_eq!((s4.skipped_done, s4.fetched), (0, 1));
}

#[tokio::test]
async fn links_file_follow_mode_and_cli() {
    let srv = Server::start(Arc::new(|r: &Req, port| match (r.host.as_str(), r.path.as_str()) {
        (_, "/robots.txt") => text(404, ""),
        ("a.test", "/") => ok(&format!("<a href=\"/a\">a</a><a rel=nofollow href=\"/b\">b</a><a href=\"http://b.test:{port}/c\">c</a><a href=\"/img.png\">i</a>")),
        ("a.test", "/a") => ok("<a href=\"/deep\">d</a>"),
        _ => ok("<html>leaf</html>"),
    }));
    let start = vec![srv.url("a.test", "/")];
    let d0 = tmp("nofollow");
    let (s0, _) = crawl(&d0, serde_json::json!({}), &start).await;
    assert_eq!((s0.fetched, s0.followed), (1, 0), "following is off by default");
    let links = std::fs::read_to_string(d0.join("links.jsonl")).unwrap();
    let v: serde_json::Value = serde_json::from_str(links.lines().next().unwrap()).unwrap();
    let got: Vec<(String, bool)> = serde_json::from_value(v["links"].clone()).unwrap();
    assert_eq!(got, vec![(srv.url("a.test", "/a"), false), (srv.url("a.test", "/b"), true), (srv.url("b.test", "/c"), false)]);
    let d1 = tmp("follow");
    let (s1, _) = crawl(&d1, serde_json::json!({"follow": {"enabled": true, "max_depth": 1}}), &start).await;
    assert_eq!((s1.fetched, s1.followed), (2, 1));
    let paths = srv.paths();
    assert!(paths.contains(&"a.test/a".to_string()));
    for p in ["a.test/b", "b.test/c", "a.test/deep", "a.test/img.png"] {
        assert!(!paths.contains(&p.to_string()), "{p} must not be followed");
    }
    assert_eq!(outcome(&d1, &srv.url("a.test", "/a")).depth, 1);
    // CLI: get and stats over the store
    let bin = env!("CARGO_BIN_EXE_seaspider");
    let out = std::process::Command::new(bin).args(["get", d1.to_str().unwrap(), &srv.url("a.test", "/a")]).output().unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, b"<a href=\"/deep\">d</a>");
    let out = std::process::Command::new(bin).args(["stats", d1.to_str().unwrap()]).output().unwrap();
    let st: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(st["page_outcomes"]["fetched"], 2);
    let bad = std::process::Command::new(bin).args(["fetch"]).output().unwrap();
    assert!(!bad.status.success(), "usage error exits non-zero");
}

#[tokio::test]
async fn private_addresses_refused_by_default() {
    let srv = Server::start(Arc::new(|_: &Req, _| ok("x")));
    let d = tmp("private");
    let urls = vec![srv.url("127.0.0.1", "/x")];
    let (s, _) = crawl(&d, serde_json::json!({"network": {"allow_private_addresses": false}}), &urls).await;
    assert_eq!(s.errors, 1);
    assert!(outcome(&d, &urls[0]).error.unwrap().contains("non-public"));
    assert!(srv.seen().is_empty(), "no request left the crawler");
}
