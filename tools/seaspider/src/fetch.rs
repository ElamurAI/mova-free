//! One bounded HTTP(S) GET: rustls, gzip/brotli/deflate decoding, no cookies, no redirects
//! (the crawler follows them itself so that every hop is re-checked), bounded time and body.
//!
//! Adapted from the family Spider (`fetch` in `spider/src/main.rs`): the body is read in chunks
//! and cut at the limit, and a fixed subset of headers is kept.

use crate::config::FetchCfg;
use crate::polite::PinResolver;
use reqwest::{redirect::Policy, Client};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

/// Headers kept in the journal.
pub const KEPT_HEADERS: &[&str] = &[
    "content-type",
    "content-length",
    "content-encoding",
    "content-language",
    "last-modified",
    "etag",
    "location",
    "retry-after",
    "x-robots-tag",
    "date",
];

/// A response, body decoded and possibly truncated.
pub struct Got {
    pub status: u16,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
    pub truncated: bool,
    pub remote_ip: Option<String>,
}

/// The client: User-Agent of the robot, pinned resolver, no redirects, no idle connections
/// (a pooled connection could otherwise bypass the IP the scheduler reserved).
pub fn client(cfg: &FetchCfg, user_agent: &str, resolver: PinResolver) -> Result<Client, String> {
    let mut h = reqwest::header::HeaderMap::new();
    h.insert(reqwest::header::ACCEPT, cfg.accept.parse().map_err(|_| "bad accept header".to_string())?);
    Client::builder()
        .user_agent(user_agent)
        .default_headers(h)
        .connect_timeout(Duration::from_millis(cfg.connect_timeout_ms))
        .timeout(Duration::from_millis(cfg.timeout_ms))
        .redirect(Policy::none())
        .pool_max_idle_per_host(0)
        .dns_resolver(Arc::new(resolver))
        .build()
        .map_err(|e| format!("http client: {e}"))
}

/// GET `url`, keep at most `limit` decoded body bytes.
pub async fn get(client: &Client, url: &str, limit: u64) -> Result<Got, String> {
    let mut resp = client.get(url).send().await.map_err(|e| short(&e))?;
    let status = resp.status().as_u16();
    let remote_ip = resp.remote_addr().map(|a| a.ip().to_string());
    let mut headers = BTreeMap::new();
    for name in KEPT_HEADERS {
        if let Some(v) = resp.headers().get(*name).and_then(|v| v.to_str().ok()) {
            headers.insert(name.to_string(), v.chars().take(1000).collect());
        }
    }
    let limit = limit as usize;
    let mut body = Vec::new();
    let mut truncated = false;
    while let Some(chunk) = resp.chunk().await.map_err(|e| short(&e))? {
        if body.len() + chunk.len() > limit {
            body.extend_from_slice(&chunk[..limit - body.len()]);
            truncated = true;
            break;
        }
        body.extend_from_slice(&chunk);
    }
    Ok(Got { status, headers, body, truncated, remote_ip })
}

/// The error with its causes, on one line, bounded.
fn short(e: &reqwest::Error) -> String {
    let mut s = e.to_string();
    let mut src = std::error::Error::source(e);
    while let Some(c) = src {
        s.push_str(": ");
        s.push_str(&c.to_string());
        src = c.source();
    }
    if e.is_timeout() && !s.contains("timed out") {
        s.push_str(" (timed out)");
    }
    s.replace('\n', " ").chars().take(400).collect()
}
