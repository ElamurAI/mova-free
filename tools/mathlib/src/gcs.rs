//! Listing of a public Google Cloud Storage bucket without keys (JSON API `storage/v1/b/<bucket>/o`):
//! the arXiv mirror (Kaggle) `arxiv-dataset` — to measure the sizes of math PDFs without downloading them.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Duration;

use anyhow::{Result, bail};
use serde_json::Value;

use crate::http::{Polite, encode, utc_now};

pub fn list(bucket: &str, prefix: &str, out: &Path) -> Result<()> {
    let mut http = Polite::new(Duration::from_secs(1));
    let mut w = BufWriter::new(File::create(out)?);
    writeln!(w, "name\tsize\tupdated\tmd5")?;
    let (mut token, mut n, mut bytes, mut pages) = (String::new(), 0u64, 0u64, 0u32);
    loop {
        let mut url = format!(
            "https://storage.googleapis.com/storage/v1/b/{bucket}/o?prefix={}&maxResults=1000&fields=nextPageToken,items(name,size,updated,md5Hash)",
            encode(prefix)
        );
        if !token.is_empty() {
            url.push_str(&format!("&pageToken={}", encode(&token)));
        }
        let body = match http.text(&url)? {
            Ok(b) => b,
            Err(st) => bail!("HTTP {} on listing {bucket}/{prefix} — stopping: {}", st.code, st.body.chars().take(200).collect::<String>()),
        };
        pages += 1;
        let v: Value = serde_json::from_str(&body)?;
        for it in v["items"].as_array().into_iter().flatten() {
            let size: u64 = it["size"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0);
            n += 1;
            bytes += size;
            writeln!(
                w,
                "{}\t{size}\t{}\t{}",
                it["name"].as_str().unwrap_or(""),
                it["updated"].as_str().unwrap_or(""),
                it["md5Hash"].as_str().unwrap_or("")
            )?;
        }
        match v["nextPageToken"].as_str() {
            Some(t) if !t.is_empty() => token = t.to_string(),
            _ => break,
        }
    }
    w.flush()?;
    println!("{} GCS {bucket}/{prefix}: {n} objects, {:.2} GB, {pages} requests → {}", utc_now(), bytes as f64 / 1e9, out.display());
    Ok(())
}
