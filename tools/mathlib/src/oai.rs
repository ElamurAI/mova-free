//! Harvesting OAI-PMH `ListRecords` page by page with `resumptionToken`.
//!
//! Each response is stored as is in `<dir>/pNNNNN.xml.gz` (raw XML, compressed). State is `<dir>/state.tsv`
//! (one line per page: number, records, token, time); rerunning the same command resumes from
//! the last token; an empty token means the harvest is finished. The file `<dir>/STOP` pauses before the next
//! request (state is saved; to resume, remove STOP and run again).
//!
//! arXiv etiquette (API ToU): one connection, ≥3 s between requests (our `delay`, 4 s by default);
//! 503 with `Retry-After` — wait as long as told (protocol, not a retry); 403/429 — stop.
//! Other failures — stop with state saved, no blind retries.

use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use flate2::Compression;
use flate2::write::GzEncoder;

use crate::http::{Polite, encode, utc_now};

pub struct Harvest<'a> {
    pub base: &'a str,
    pub set: Option<&'a str>,
    pub prefix: &'a str,
    pub from: Option<&'a str>,
    pub delay: Duration,
    pub max_pages: usize,
}

/// The last state line: (pages, total records, token). `None` — no state.
fn last_state(state: &Path) -> Result<Option<(usize, u64, String)>> {
    if !state.exists() {
        return Ok(None);
    }
    let mut last = None;
    let mut total = 0u64;
    for line in BufReader::new(File::open(state)?).lines() {
        let line = line?;
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 3 || c[0] == "page" {
            continue;
        }
        let page: usize = c[0].parse().with_context(|| format!("state: {line}"))?;
        total += c[1].parse::<u64>().unwrap_or(0);
        last = Some((page, total, c[2].to_string()));
    }
    Ok(last)
}

pub fn run(h: &Harvest, dir: &Path) -> Result<()> {
    fs::create_dir_all(dir)?;
    let state = dir.join("state.tsv");
    let (mut page, mut total, mut token) = match last_state(&state)? {
        Some((p, t, tok)) if tok.is_empty() => {
            println!("{} DONE {} pages, {t} records — harvest already finished", utc_now(), p);
            return Ok(());
        }
        Some((p, t, tok)) => {
            println!("{} RESUME from page {} ({t} records), token {tok}", utc_now(), p + 1);
            (p, t, Some(tok))
        }
        None => {
            fs::write(&state, "page\trecords\ttoken\tutc\n")?;
            (0, 0, None)
        }
    };
    let mut http = Polite::new(h.delay);
    let mut busy = 0u32;
    let mut done_here = 0usize;
    loop {
        if dir.join("STOP").exists() {
            println!("{} PAUSE STOP file; state: {page} pages, {total} records. To resume: remove STOP and run again", utc_now());
            return Ok(());
        }
        if h.max_pages > 0 && done_here >= h.max_pages {
            println!("{} LIMIT {} pages per run; to resume, run again", utc_now(), h.max_pages);
            return Ok(());
        }
        let url = match &token {
            Some(t) => format!("{}?verb=ListRecords&resumptionToken={}", h.base, encode(t)),
            None => {
                let mut u = format!("{}?verb=ListRecords&metadataPrefix={}", h.base, encode(h.prefix));
                if let Some(s) = h.set {
                    u.push_str(&format!("&set={}", encode(s)));
                }
                if let Some(f) = h.from {
                    u.push_str(&format!("&from={}", encode(f)));
                }
                u
            }
        };
        let t0 = Instant::now();
        let body = match http.text(&url) {
            Ok(Ok(b)) => b,
            Ok(Err(st)) if st.code == 503 => {
                busy += 1;
                let wait = st.retry_after.unwrap_or(30).clamp(5, 600);
                println!("{} 503 Retry-After {:?} — waiting {wait} s (consecutive {busy})", utc_now(), st.retry_after);
                if busy > 20 {
                    bail!("503 more than 20 times in a row — stopping; state saved");
                }
                std::thread::sleep(Duration::from_secs(wait));
                continue;
            }
            Ok(Err(st)) => bail!(
                "HTTP {} on {url} — stopping the route (403/429 — forbidden or rate limit; other — failure), state saved. Body: {}",
                st.code,
                st.body.chars().take(300).collect::<String>()
            ),
            Err(e) => return Err(e.context("transport — stopping, state saved; to resume, run again")),
        };
        busy = 0;
        if let Some(code) = oai_error(&body) {
            if code == "noRecordsMatch" {
                append(&state, page + 1, 0, "")?;
                println!("{} DONE noRecordsMatch", utc_now());
                return Ok(());
            }
            bail!("OAI error {code} on {url} — stopping (for badResumptionToken: the token has expired, see docs/math-library.md)");
        }
        let n = body.matches("<record>").count() as u64;
        let next = resumption_token(&body);
        page += 1;
        done_here += 1;
        total += n;
        let out = dir.join(format!("p{page:05}.xml.gz"));
        let mut gz = GzEncoder::new(File::create(&out)?, Compression::default());
        gz.write_all(body.as_bytes())?;
        gz.finish()?.sync_all()?;
        append(&state, page, n, next.as_deref().unwrap_or(""))?;
        println!(
            "{} page {page} +{n} = {total} ({:.1} s, {} KB) next={}",
            utc_now(),
            t0.elapsed().as_secs_f64(),
            body.len() / 1024,
            next.as_deref().unwrap_or("—")
        );
        match next {
            Some(t) if !t.is_empty() => token = Some(t),
            _ => {
                println!("{} DONE {page} pages, {total} records", utc_now());
                return Ok(());
            }
        }
    }
}

fn append(state: &Path, page: usize, n: u64, token: &str) -> Result<()> {
    let mut f = OpenOptions::new().append(true).open(state)?;
    writeln!(f, "{page}\t{n}\t{token}\t{}", utc_now())?;
    f.sync_all()?;
    Ok(())
}

/// `<error code="…">` of an OAI-PMH response.
fn oai_error(body: &str) -> Option<String> {
    let i = body.find("<error ")?;
    let rest = &body[i..];
    let j = rest.find("code=")? + 5;
    let q = rest[j..].chars().next()?;
    let end = rest[j + 1..].find(q)?;
    Some(rest[j + 1..j + 1 + end].to_string())
}

/// Contents of the last `<resumptionToken …>…</resumptionToken>`; an empty element means end of list.
fn resumption_token(body: &str) -> Option<String> {
    let i = body.rfind("<resumptionToken")?;
    let rest = &body[i..];
    if rest[..rest.find('>')?].ends_with('/') {
        return Some(String::new());
    }
    let open = rest.find('>')? + 1;
    let close = rest.find("</resumptionToken>")?;
    let raw = rest[open..close].trim();
    Some(quick_xml::escape::unescape(raw).map(|c| c.into_owned()).unwrap_or_else(|_| raw.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_and_errors() {
        let b = "<ListRecords><record></record><resumptionToken expirationDate='x'>verb%3DListRecords%26skip%3D1300</resumptionToken></ListRecords>";
        assert_eq!(resumption_token(b).as_deref(), Some("verb%3DListRecords%26skip%3D1300"));
        assert_eq!(resumption_token("<resumptionToken completeListSize='5'/>").as_deref(), Some(""));
        assert_eq!(resumption_token("<resumptionToken></resumptionToken>").as_deref(), Some(""));
        assert_eq!(resumption_token("<ListRecords></ListRecords>"), None);
        assert_eq!(oai_error("<error code=\"badResumptionToken\">x</error>").as_deref(), Some("badResumptionToken"));
        assert_eq!(oai_error("<ListRecords/>"), None);
    }
}
