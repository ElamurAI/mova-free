//! The link downloader: queue -> robots.txt -> polite fetch with re-checked redirects -> store.
//!
//! Order is deterministic: queue URLs keep their file order within a host, hosts are visited
//! round-robin in `BTreeMap` order, and follow mode appends links in page order. Timing never
//! changes the order; it only decides how long the crawler sleeps.
//!
//! Every request (robots.txt, page, redirect hop) goes through one reservation: the IP must have
//! been idle for the configured interval (at least 2 s) and the host for its delay
//! `max(interval, Crawl-delay)`. Every redirect target is checked against its own origin's
//! robots.txt and reserved again. Adapted from `visit` and `robots` in the family Spider's
//! `spider/src/main.rs`, without its consent levels, AIMD pacing and conditional requests.

use crate::config::Config;
use crate::fetch::{self, Got};
use crate::links;
use crate::polite::{Dns, Politeness};
use crate::robots::{self, Robots, Rules};
use crate::store::{Hop, LinksLine, Record, RobotsNote, Store, VERSION};
use reqwest::{Client, Url};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::net::IpAddr;
use std::path::Path;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// One request as it happened (for measurements and tests).
#[derive(Clone, Debug)]
pub struct RequestLog {
    pub kind: &'static str,
    pub url: String,
    pub host: String,
    pub ip: IpAddr,
    pub start: Instant,
    pub end: Instant,
}

enum RobotsState {
    Parsed(Rules),
    AllowAll(String),
    Unreachable(String),
}

struct RobotsEntry {
    at: Instant,
    ttl: Duration,
    url: String,
    state: RobotsState,
}

#[derive(Clone, Debug)]
struct Item {
    url: String,
    depth: u32,
    parent: Option<String>,
}

/// Counts of one run.
#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct Summary {
    pub queued: u64,
    pub duplicates: u64,
    pub skipped_done: u64,
    pub fetched: u64,
    pub disallowed: u64,
    pub deferred: u64,
    pub errors: u64,
    pub followed: u64,
    pub requests: u64,
    pub uncertain_recorded: u64,
}

pub struct Crawler {
    cfg: Config,
    pub store: Store,
    client: Client,
    dns: Dns,
    polite: Politeness,
    robots: BTreeMap<String, RobotsEntry>,
    pub requests: Vec<RequestLog>,
    ua: String,
}

pub fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// Queue file: one URL per line — a JSON object with "url", or TSV whose first field is the URL.
/// Empty lines and lines starting with '#' are skipped.
pub fn load_queue(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        if l.starts_with('{') {
            let v: serde_json::Value = serde_json::from_str(l).map_err(|e| format!("{}:{}: {e}", path.display(), n + 1))?;
            let u = v["url"].as_str().ok_or_else(|| format!("{}:{}: no \"url\" string", path.display(), n + 1))?;
            out.push(u.to_string());
        } else {
            out.push(l.split('\t').next().unwrap_or("").trim().to_string());
        }
    }
    Ok(out)
}

/// Absolute http(s) URL without fragment, in reqwest's canonical form.
pub fn normalize_url(u: &str) -> Result<Url, String> {
    let mut url = Url::parse(u.trim()).map_err(|e| format!("invalid URL: {e}"))?;
    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err("only http(s) URLs with a host".into());
    }
    url.set_fragment(None);
    Ok(url)
}

fn host_of(u: &Url) -> String {
    u.host_str().unwrap_or("").to_ascii_lowercase()
}

fn origin_of(u: &Url) -> String {
    format!("{}://{}:{}", u.scheme(), host_of(u), u.port_or_known_default().unwrap_or(0))
}

fn path_of(u: &Url) -> String {
    match u.query() {
        Some(q) => format!("{}?{q}", u.path()),
        None => u.path().to_string(),
    }
}

fn is_html(g: &Got) -> bool {
    match g.headers.get("content-type") {
        Some(ct) => ct.to_ascii_lowercase().contains("html"),
        None => g.body.iter().find(|b| !b.is_ascii_whitespace()) == Some(&b'<'),
    }
}

/// The decision for a path under one robots.txt state.
fn note(state: &RobotsState, robots_url: &str, path: &str) -> RobotsNote {
    match state {
        RobotsState::Parsed(rules) => {
            let v = rules.decide(path);
            RobotsNote {
                robots_url: robots_url.to_string(),
                source: "parsed".into(),
                group: rules.group.as_str().into(),
                allowed: v.allowed,
                rule: v.rule.map(|(a, p)| format!("{}: {p}", if a { "Allow" } else { "Disallow" })),
                crawl_delay: rules.crawl_delay,
            }
        }
        RobotsState::AllowAll(why) => RobotsNote { robots_url: robots_url.to_string(), source: why.clone(), group: "none".into(), allowed: true, rule: None, crawl_delay: None },
        RobotsState::Unreachable(why) => RobotsNote { robots_url: robots_url.to_string(), source: why.clone(), group: "none".into(), allowed: false, rule: None, crawl_delay: None },
    }
}

/// The explanation of one decision (shared by `explain` and `stored_explanation`). `basis` says
/// whether robots.txt was fetched for it or read back from the store.
pub fn explanation(cfg: &Config, u: &Url, n: RobotsNote, basis: &str) -> serde_json::Value {
    let cd = n.crawl_delay.unwrap_or(0.0);
    let interval = cfg.ip_interval().as_secs_f64();
    let deferred = cd > cfg.politeness.max_crawl_delay_secs as f64;
    serde_json::json!({
        "url": u.as_str(),
        "token": cfg.robot.token,
        "user_agent": cfg.robot.user_agent(),
        "robots": n,
        "host_delay_secs": interval.max(cd),
        "max_crawl_delay_secs": cfg.politeness.max_crawl_delay_secs,
        "deferred": deferred,
        "decision": if !n.allowed { "disallowed" } else if deferred { "deferred" } else { "allowed" },
        "basis": basis,
    })
}

/// The decision for a URL from the latest robots.txt record of its origin in a store, without any
/// request: None when there is no such record or it is older than its TTL (`robots.ttl_secs`, or
/// `robots.unreachable_retry_secs` for an unreachable one), or a 2xx record has no stored body.
/// The status handling is the crawler's: 2xx parse, 3xx and 4xx except 429 allow all, else closed.
pub fn stored_explanation(cfg: &Config, reader: &crate::store::Reader, url: &str, now_unix: u64) -> Result<Option<serde_json::Value>, String> {
    let u = normalize_url(url)?;
    let robots_url = u.join("/robots.txt").map_err(|e| e.to_string())?.to_string();
    let Some(rec) = reader.index.recs.iter().rev().find(|r| r.kind == "robots" && r.url == robots_url) else { return Ok(None) };
    let age = now_unix.saturating_sub(rec.fetched_unix);
    let state = match rec.status {
        _ if rec.outcome == "error" => RobotsState::Unreachable(rec.error.clone().unwrap_or_else(|| "unreachable".into())),
        Some(s) if (200..300).contains(&s) => {
            let Ok(body) = crate::store::read_body(&reader.dir, rec) else { return Ok(None) };
            RobotsState::Parsed(Robots::parse(&String::from_utf8_lossy(&body)).rules_for(&cfg.robot.token))
        }
        Some(s) if (300..400).contains(&s) => RobotsState::AllowAll(format!("allow_all (HTTP {s} without a usable Location)")),
        Some(s) if (400..500).contains(&s) && s != 429 => RobotsState::AllowAll(format!("allow_all (HTTP {s})")),
        Some(s) => RobotsState::Unreachable(format!("unreachable (HTTP {s})")),
        None => return Ok(None),
    };
    let ttl = if matches!(state, RobotsState::Unreachable(_)) { cfg.robots.unreachable_retry_secs } else { cfg.robots.ttl_secs };
    if age >= ttl {
        return Ok(None);
    }
    let mut v = explanation(cfg, &u, note(&state, &robots_url, &path_of(&u)), "stored");
    v["robots_fetched_unix"] = rec.fetched_unix.into();
    Ok(Some(v))
}

impl Crawler {
    pub fn new(cfg: Config, store_dir: &Path) -> Result<Crawler, String> {
        cfg.validate()?;
        let store = Store::open(store_dir, cfg.storage.segment_max_bytes, cfg.storage.zstd_level)?;
        let dns = Dns::new(Duration::from_secs(cfg.network.dns_ttl_secs), cfg.network.allow_private_addresses, cfg.network.resolve.clone());
        let ua = cfg.robot.user_agent();
        let client = fetch::client(&cfg.fetch, &ua, dns.resolver())?;
        let polite = Politeness::new(cfg.ip_interval());
        Ok(Crawler { cfg, store, client, dns, polite, robots: BTreeMap::new(), requests: Vec::new(), ua })
    }

    pub fn user_agent(&self) -> &str {
        &self.ua
    }

    /// One request through the reservation: sleep until the IP and host are free, pin the IP.
    async fn polite_get(&mut self, kind: &'static str, host: &str, ips: &[IpAddr], host_delay: Duration, url: &Url, limit: u64) -> (IpAddr, Result<Got, String>) {
        let (ip, start) = self.polite.reserve(host, ips, host_delay, Instant::now());
        tokio::time::sleep_until(tokio::time::Instant::from_std(start)).await;
        self.dns.pin(host, ip);
        let t0 = Instant::now();
        let r = fetch::get(&self.client, url.as_str(), limit).await;
        let end = Instant::now();
        self.polite.finish(host, ip, end, host_delay);
        self.requests.push(RequestLog { kind, url: url.to_string(), host: host.to_string(), ip, start: t0, end });
        (ip, r)
    }

    /// Fetch robots.txt of an origin (following up to `robots.max_redirects` redirects, each one
    /// reserved), record it, and cache the state.
    async fn fetch_robots(&mut self, page: &Url) -> RobotsEntry {
        let mut cur = page.join("/robots.txt").expect("robots URL");
        let robots_url = cur.to_string();
        let ttl = Duration::from_secs(self.cfg.robots.ttl_secs);
        let retry = Duration::from_secs(self.cfg.robots.unreachable_retry_secs);
        let at = Instant::now();
        let mut chain: Vec<Hop> = Vec::new();
        let mut redirects = 0;
        let (state, got) = loop {
            let host = host_of(&cur);
            let ips = match self.dns.resolve(&host).await {
                Ok(ips) => ips,
                Err(e) => break (RobotsState::Unreachable(format!("unreachable ({e})")), None),
            };
            let interval = self.polite.ip_interval();
            let (ip, r) = self.polite_get("robots", &host, &ips, interval, &cur, robots::MAX_BYTES as u64).await;
            match r {
                Err(e) => {
                    chain.push(Hop { url: cur.to_string(), status: 0, ip: Some(ip.to_string()) });
                    break (RobotsState::Unreachable(format!("unreachable ({e})")), None);
                }
                Ok(g) => {
                    chain.push(Hop { url: cur.to_string(), status: g.status, ip: g.remote_ip.clone().or(Some(ip.to_string())) });
                    let s = g.status;
                    if (300..400).contains(&s) {
                        if let Some(n) = g.headers.get("location").and_then(|l| cur.join(l).ok()) {
                            redirects += 1;
                            if redirects > self.cfg.robots.max_redirects {
                                break (RobotsState::AllowAll(format!("allow_all (more than {} redirects: unavailable)", self.cfg.robots.max_redirects)), Some(g));
                            }
                            if matches!(n.scheme(), "http" | "https") {
                                cur = n;
                                continue;
                            }
                        }
                        break (RobotsState::AllowAll(format!("allow_all (HTTP {s} without a usable Location)")), Some(g));
                    }
                    if (200..300).contains(&s) {
                        let rules = Robots::parse(&String::from_utf8_lossy(&g.body)).rules_for(&self.cfg.robot.token);
                        break (RobotsState::Parsed(rules), Some(g));
                    }
                    // RFC 9309 §2.3.1.3/4: 4xx unavailable -> allow; 5xx unreachable -> disallow.
                    // 429 is treated as unreachable: the server asks for rest.
                    if (400..500).contains(&s) && s != 429 {
                        break (RobotsState::AllowAll(format!("allow_all (HTTP {s})")), Some(g));
                    }
                    break (RobotsState::Unreachable(format!("unreachable (HTTP {s})")), Some(g));
                }
            }
        };
        // the robots.txt fetch is a journal record too (body stored when 2xx)
        let mut rec = Record { v: VERSION, kind: "robots".into(), url: robots_url.clone(), fetched_unix: unix_now(), chain: chain.clone(), user_agent: self.ua.clone(), ..Default::default() };
        rec.final_url = chain.last().map(|h| h.url.clone());
        rec.status = chain.last().map(|h| h.status).filter(|s| *s > 0);
        match (&state, &got) {
            (RobotsState::Unreachable(why), None) => {
                rec.outcome = "error".into();
                rec.error = Some(why.clone());
            }
            _ => rec.outcome = "fetched".into(),
        }
        if let Some(g) = &got {
            rec.headers = g.headers.clone();
            rec.content_type = g.headers.get("content-type").cloned();
            rec.truncated = g.truncated;
            if (200..300).contains(&g.status) {
                if let Ok((loc, sha)) = self.store.put_body(&g.body) {
                    rec.seg = Some(loc.seg);
                    rec.off = Some(loc.off);
                    rec.len = Some(loc.len);
                    rec.body_bytes = Some(loc.body_bytes);
                    rec.sha256 = Some(sha);
                }
            }
        }
        if let Err(e) = self.store.append(&rec) {
            eprintln!("seaspider: journal: {e}");
        }
        let ttl = if matches!(state, RobotsState::Unreachable(_)) { retry } else { ttl };
        RobotsEntry { at, ttl, url: robots_url, state }
    }

    /// The robots.txt decision for a URL (fetching robots.txt when not cached or stale).
    pub async fn robots_decision(&mut self, url: &Url) -> RobotsNote {
        let origin = origin_of(url);
        let fresh = self.robots.get(&origin).is_some_and(|e| e.at.elapsed() < e.ttl);
        if !fresh {
            let e = self.fetch_robots(url).await;
            self.robots.insert(origin.clone(), e);
        }
        let e = &self.robots[&origin];
        note(&e.state, &e.url, &path_of(url))
    }

    /// Explain the decision for one URL (fetches robots.txt politely; no page request).
    pub async fn explain(&mut self, url: &str) -> Result<serde_json::Value, String> {
        let u = normalize_url(url)?;
        self.dns.resolve(&host_of(&u)).await?;
        let n = self.robots_decision(&u).await;
        Ok(explanation(&self.cfg, &u, n, "fetched_now"))
    }

    /// One queue item: robots, intent, fetch, redirects (each re-checked), record.
    async fn visit(&mut self, item: &Item) -> (Record, Option<(Url, Got)>) {
        let t0 = Instant::now();
        let mut rec = Record { v: VERSION, kind: "page".into(), url: item.url.clone(), depth: item.depth, parent: item.parent.clone(), user_agent: self.ua.clone(), ..Default::default() };
        let mut cur = match normalize_url(&item.url) {
            Ok(u) => u,
            Err(e) => {
                rec.outcome = "error".into();
                rec.error = Some(e);
                rec.fetched_unix = unix_now();
                return (rec, None);
            }
        };
        let max_redirects = self.cfg.fetch.max_redirects;
        let mut intent = false;
        let mut last: Option<Got> = None;
        let mut hop = 0;
        loop {
            let host = host_of(&cur);
            let ips = match self.dns.resolve(&host).await {
                Ok(ips) => ips,
                Err(e) => {
                    rec.outcome = "error".into();
                    rec.error = Some(e);
                    break;
                }
            };
            let note = self.robots_decision(&cur).await;
            let allowed = note.allowed;
            let cd = note.crawl_delay.unwrap_or(0.0);
            rec.robots = Some(note);
            if !allowed {
                rec.outcome = "disallowed".into();
                if hop > 0 {
                    rec.error = Some(format!("redirect target {cur} is disallowed"));
                }
                last = None;
                break;
            }
            if cd > self.cfg.politeness.max_crawl_delay_secs as f64 {
                rec.outcome = "deferred".into();
                rec.error = Some(format!("crawl-delay {cd}s exceeds politeness.max_crawl_delay_secs"));
                last = None;
                break;
            }
            let host_delay = self.polite.ip_interval().max(Duration::from_secs_f64(cd));
            if !intent {
                let i = Record { v: VERSION, kind: "intent".into(), url: item.url.clone(), outcome: "intent".into(), fetched_unix: unix_now(), depth: item.depth, parent: item.parent.clone(), ..Default::default() };
                if let Err(e) = self.store.append(&i) {
                    rec.outcome = "error".into();
                    rec.error = Some(format!("journal: {e}"));
                    break;
                }
                intent = true;
            }
            let (ip, r) = self.polite_get("page", &host, &ips, host_delay, &cur, self.cfg.fetch.max_body_bytes).await;
            match r {
                Err(e) => {
                    rec.chain.push(Hop { url: cur.to_string(), status: 0, ip: Some(ip.to_string()) });
                    rec.outcome = "error".into();
                    rec.error = Some(e);
                    last = None;
                    break;
                }
                Ok(g) => {
                    rec.chain.push(Hop { url: cur.to_string(), status: g.status, ip: g.remote_ip.clone().or(Some(ip.to_string())) });
                    let next = if (300..400).contains(&g.status) {
                        g.headers.get("location").and_then(|l| cur.join(l).ok()).filter(|n| matches!(n.scheme(), "http" | "https")).map(|mut n| {
                            n.set_fragment(None);
                            n
                        })
                    } else {
                        None
                    };
                    rec.outcome = "fetched".into();
                    match next {
                        Some(n) if hop < max_redirects => {
                            hop += 1;
                            cur = n;
                            last = Some(g);
                        }
                        Some(_) => {
                            rec.error = Some(format!("more than {max_redirects} redirects"));
                            last = Some(g);
                            break;
                        }
                        None => {
                            last = Some(g);
                            break;
                        }
                    }
                }
            }
        }
        rec.fetched_unix = unix_now();
        rec.ms = Some(t0.elapsed().as_millis() as u64);
        if let Some(h) = rec.chain.last() {
            rec.final_url = Some(h.url.clone());
            rec.status = Some(h.status).filter(|s| *s > 0);
        }
        let mut out = None;
        if let Some(g) = last {
            rec.headers = g.headers.clone();
            rec.content_type = g.headers.get("content-type").cloned();
            rec.truncated = g.truncated;
            match self.store.put_body(&g.body) {
                Ok((loc, sha)) => {
                    rec.seg = Some(loc.seg);
                    rec.off = Some(loc.off);
                    rec.len = Some(loc.len);
                    rec.body_bytes = Some(loc.body_bytes);
                    rec.sha256 = Some(sha);
                }
                Err(e) => {
                    rec.outcome = "error".into();
                    rec.error = Some(format!("store: {e}"));
                }
            }
            if rec.outcome == "fetched" {
                out = Some((cur, g));
            }
        }
        (rec, out)
    }

    /// Fetch a queue. Already stored URLs (by URL, within the refetch age) are skipped.
    pub async fn run(&mut self, urls: &[String]) -> Result<Summary, String> {
        let mut sum = Summary::default();
        let req0 = self.requests.len();
        sum.uncertain_recorded = self.store.uncertain_on_open;
        let now = unix_now();
        let refetch = self.cfg.resume.refetch_after_secs;
        let mut queues: BTreeMap<String, VecDeque<Item>> = BTreeMap::new();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for raw in urls {
            sum.queued += 1;
            let (key, host) = match normalize_url(raw) {
                Ok(u) => (u.to_string(), host_of(&u)),
                // invalid URLs still produce an error record, in their own pseudo-host queue
                Err(_) => (raw.clone(), String::new()),
            };
            if !seen.insert(key.clone()) {
                sum.duplicates += 1;
                continue;
            }
            if self.store.done(&key, now, refetch) {
                sum.skipped_done += 1;
                continue;
            }
            queues.entry(host).or_default().push_back(Item { url: key, depth: 0, parent: None });
        }
        // round-robin over hosts in BTreeMap order; FIFO within a host
        while queues.values().any(|q| !q.is_empty()) {
            let hosts: Vec<String> = queues.iter().filter(|(_, q)| !q.is_empty()).map(|(h, _)| h.clone()).collect();
            for host in hosts {
                let Some(item) = queues.get_mut(&host).and_then(|q| q.pop_front()) else { continue };
                let (rec, got) = self.visit(&item).await;
                match rec.outcome.as_str() {
                    "fetched" => sum.fetched += 1,
                    "disallowed" => sum.disallowed += 1,
                    "deferred" => sum.deferred += 1,
                    _ => sum.errors += 1,
                }
                self.store.append(&rec)?;
                let Some((final_url, g)) = got else { continue };
                if !is_html(&g) || rec.sha256.is_none() {
                    continue;
                }
                let header_nofollow = g.headers.get("x-robots-tag").is_some_and(|v| v.to_ascii_lowercase().contains("nofollow"));
                let out = links::outgoing(&final_url, &g.body);
                let line = LinksLine { url: rec.url.clone(), final_url: final_url.to_string(), sha256: rec.sha256.clone().unwrap_or_default(), links: out.iter().map(|(u, nf)| (u.to_string(), *nf || header_nofollow)).collect() };
                self.store.append_links(&line)?;
                let f = &self.cfg.follow;
                if !f.enabled || item.depth >= f.max_depth || header_nofollow {
                    continue;
                }
                let item_host = Url::parse(&item.url).map(|u| host_of(&u)).unwrap_or_default();
                for (u, nf) in out {
                    if nf || host_of(&u) != item_host || sum.followed >= f.max_pages {
                        continue;
                    }
                    let key = u.to_string();
                    if !seen.insert(key.clone()) || self.store.done(&key, now, refetch) {
                        continue;
                    }
                    sum.followed += 1;
                    queues.entry(item_host.clone()).or_default().push_back(Item { url: key, depth: item.depth + 1, parent: Some(final_url.to_string()) });
                }
            }
        }
        sum.requests = (self.requests.len() - req0) as u64;
        Ok(sum)
    }
}
