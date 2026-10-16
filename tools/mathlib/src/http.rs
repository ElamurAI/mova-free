//! Polite HTTP: one connection, a pause between requests, a User-Agent without personal data.
//!
//! The pause counts from the end of the previous response (body fully read), so the rate is at most one request
//! per `delay`. Statuses are not retried here: the decision (503 with Retry-After — wait, 403/429 — stop
//! the route) is made by the caller.

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use md5::Md5;
use sha2::{Digest, Sha256};

/// The library's User-Agent: name and purpose, no e-mail or names (project rule).
pub const UA: &str = "MovaLibrary/0.1 (research)";

pub struct Polite {
    agent: ureq::Agent,
    delay: Duration,
    last: Option<Instant>,
}

/// A non-2xx response: code, Retry-After (seconds), start of the body.
pub struct Status {
    pub code: u16,
    pub retry_after: Option<u64>,
    pub body: String,
}

/// A downloaded file: hashes are computed on the fly.
pub struct Fetched {
    pub bytes: u64,
    pub sha256: String,
    pub md5: String,
    pub content_type: String,
    pub disposition: String,
    pub final_url: String,
}

impl Polite {
    pub fn new(delay: Duration) -> Self {
        let agent = ureq::AgentBuilder::new()
            .user_agent(UA)
            .timeout_connect(Duration::from_secs(30))
            .timeout_read(Duration::from_secs(300))
            .redirects(5)
            .build();
        Self { agent, delay, last: None }
    }

    fn wait(&mut self) {
        if let Some(t) = self.last {
            let el = t.elapsed();
            if el < self.delay {
                std::thread::sleep(self.delay - el);
            }
        }
    }

    fn call(&mut self, url: &str) -> Result<std::result::Result<ureq::Response, Status>> {
        self.wait();
        let r = self.agent.get(url).call();
        let out = match r {
            Ok(resp) => Ok(resp),
            Err(ureq::Error::Status(code, resp)) => {
                let retry_after = resp.header("Retry-After").and_then(|s| s.trim().parse().ok());
                let mut body = String::new();
                let _ = resp.into_reader().take(4096).read_to_string(&mut body);
                self.last = Some(Instant::now());
                Err(Status { code, retry_after, body })
            }
            Err(e) => {
                self.last = Some(Instant::now());
                return Err(e).with_context(|| format!("GET {url}"));
            }
        };
        Ok(out)
    }

    /// A text response (OAI-PMH). ureq decodes gzip transport encoding itself.
    pub fn text(&mut self, url: &str) -> Result<std::result::Result<String, Status>> {
        Ok(self.text_url(url)?.map(|(s, _)| s))
    }

    /// Text and the final URL after redirects (the base for relative links).
    pub fn text_url(&mut self, url: &str) -> Result<std::result::Result<(String, String), Status>> {
        match self.call(url)? {
            Ok(resp) => {
                let fin = resp.get_url().to_string();
                let mut b = Vec::new();
                let rd = resp.into_reader().read_to_end(&mut b);
                self.last = Some(Instant::now());
                rd.with_context(|| format!("body {url}"))?;
                // some pages are Latin-1 — do not fail, decode with replacement
                Ok(Ok((String::from_utf8_lossy(&b).into_owned(), fin)))
            }
            Err(st) => Ok(Err(st)),
        }
    }

    /// A file into `dest` (via `.part`), sha256 and md5 on the fly. `limit` is the size cap for one file.
    pub fn download(&mut self, url: &str, dest: &Path, limit: u64) -> Result<std::result::Result<Fetched, Status>> {
        let resp = match self.call(url)? {
            Ok(r) => r,
            Err(st) => return Ok(Err(st)),
        };
        let content_type = resp.content_type().to_string();
        let disposition = resp.header("Content-Disposition").unwrap_or("").to_string();
        let final_url = resp.get_url().to_string();
        if let Some(len) = resp.header("Content-Length").and_then(|s| s.parse::<u64>().ok()) {
            if len > limit {
                self.last = Some(Instant::now());
                return Ok(Err(Status { code: 0, retry_after: None, body: format!("Content-Length {len} > cap {limit}") }));
            }
        }
        let part = dest.with_extension("part");
        let mut out = File::create(&part).with_context(|| format!("{}", part.display()))?;
        let (mut sha, mut md5) = (Sha256::new(), Md5::new());
        let mut rd = resp.into_reader();
        let mut buf = vec![0u8; 1 << 16];
        let mut bytes = 0u64;
        loop {
            let n = match rd.read(&mut buf) {
                Ok(n) => n,
                Err(e) => {
                    self.last = Some(Instant::now());
                    let _ = std::fs::remove_file(&part);
                    return Err(e).with_context(|| format!("body {url}"));
                }
            };
            if n == 0 {
                break;
            }
            bytes += n as u64;
            if bytes > limit {
                self.last = Some(Instant::now());
                drop(out);
                let _ = std::fs::remove_file(&part);
                return Ok(Err(Status { code: 0, retry_after: None, body: format!("body > cap {limit}") }));
            }
            sha.update(&buf[..n]);
            md5.update(&buf[..n]);
            out.write_all(&buf[..n])?;
        }
        out.sync_all()?;
        drop(out);
        self.last = Some(Instant::now());
        std::fs::rename(&part, dest)?;
        Ok(Ok(Fetched {
            bytes,
            sha256: hex(&sha.finalize()),
            md5: hex(&md5.finalize()),
            content_type,
            disposition,
            final_url,
        }))
    }
}

pub fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

/// Percent-encoding of a query parameter value (everything except RFC 3986 unreserved).
pub fn encode(v: &str) -> String {
    let mut s = String::with_capacity(v.len() * 3);
    for b in v.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            s.push(b as char);
        } else {
            s.push_str(&format!("%{b:02X}"));
        }
    }
    s
}

/// UTC time `YYYY-MM-DDTHH:MM:SSZ` without external crates.
pub fn utc_now() -> String {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let (days, rem) = (secs / 86_400, secs % 86_400);
    let (y, m, d) = civil(days as i64);
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z", rem / 3600, rem % 3600 / 60, rem % 60)
}

/// Days since 1970-01-01 → (year, month, day), Howard Hinnant's algorithm.
fn civil(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

/// Free space on the partition holding `path`, in bytes (`df -B1 --output=avail`).
pub fn free_bytes(path: &Path) -> Result<u64> {
    let out = std::process::Command::new("df").arg("-B1").arg("--output=avail").arg(path).output().context("df")?;
    let s = String::from_utf8_lossy(&out.stdout);
    s.lines().nth(1).and_then(|l| l.trim().parse().ok()).context("df: not a number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_and_dates() {
        assert_eq!(encode("verb%3DListRecords%26skip%3D1300"), "verb%253DListRecords%2526skip%253D1300");
        assert_eq!(encode("a b/c"), "a%20b%2Fc");
        assert_eq!(civil(0), (1970, 1, 1));
        assert_eq!(civil(20_722), (2026, 9, 26));
        assert_eq!(civil(11_016), (2000, 2, 29));
    }
}
