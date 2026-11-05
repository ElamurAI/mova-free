//! Sea Spider's English: short, correct sentences from structured results, realized
//! deterministically with Mova's realizer (`mova_octopus::realize`: number agreement, plurals and
//! past forms from Mova's morphology, lists). Every function states only what its input says;
//! the structured JSON travels alongside the sentence.
use mova_octopus::realize::{be, could_not, list, noun, noun_count, paragraph, past, sentence};
use mova_octopus::Report;
use serde_json::Value;

/// 1234567 -> "1,234,567".
pub fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// Unix seconds -> "2026-11-04 06:07 UTC" (proleptic Gregorian, Hinnant's civil-from-days).
pub fn utc(unix: u64) -> String {
    let days = (unix / 86400) as i64;
    let secs = unix % 86400;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02} {:02}:{:02} UTC", secs / 3600, secs % 3600 / 60)
}

/// The parenthesised detail of a robots source such as "unreachable (HTTP 503)".
fn detail(source: &str) -> Option<&str> {
    let a = source.find('(')?;
    source[a + 1..].strip_suffix(')')
}

/// What one decision means in English.
pub fn robots(e: &Value) -> String {
    let url = e["url"].as_str().unwrap_or("?");
    let token = e["token"].as_str().unwrap_or("MovaBot");
    let n = &e["robots"];
    let robots_url = n["robots_url"].as_str().unwrap_or("robots.txt");
    let source = n["source"].as_str().unwrap_or("");
    let rule = n["rule"].as_str().map(|r| format!(" (rule {r})")).unwrap_or_default();
    let first = match e["decision"].as_str() {
        Some("allowed") if source == "parsed" => {
            let rule = if rule.is_empty() { " (no rule matches it)".to_string() } else { rule };
            format!("yes, {robots_url} allows {token} to fetch {url}{rule}")
        }
        Some("allowed") => format!(
            "yes, {robots_url} is unavailable ({}), so {token} may fetch {url}",
            detail(source).unwrap_or(source)
        ),
        Some("deferred") => format!(
            "yes, {robots_url} allows {url}{rule}, but its Crawl-delay of {} seconds is longer than my limit of {} seconds, so I would defer it",
            n["crawl_delay"].as_f64().unwrap_or(0.0),
            e["max_crawl_delay_secs"].as_u64().unwrap_or(0)
        ),
        _ if source == "parsed" => format!("no, {robots_url} disallows {url} for {token}{rule}"),
        _ => format!(
            "no, I could not read {robots_url} ({}), so I keep away from that site for now",
            detail(source).unwrap_or(source)
        ),
    };
    let basis = match e["basis"].as_str() {
        Some("stored") => e["robots_fetched_unix"]
            .as_u64()
            .map(|t| format!("I read robots.txt at {}.", utc(t)))
            .unwrap_or_default(),
        _ => "I have just read robots.txt.".into(),
    };
    paragraph(&[sentence(&first), basis])
}

pub fn robots_queued(target: &str, job: &str) -> String {
    paragraph(&[
        sentence(&format!("I have not read robots.txt for {target} recently, so I have queued a check ({job})")),
        sentence("Ask me again in a moment"),
    ])
}

/// Where a newly queued job stands.
pub fn fetch_queued(job: &str, urls: &[String], ahead: usize, busy: bool) -> String {
    let what = format!("I have queued {} for fetching ({job}): {}", noun_count(urls.len() as u64, "page"), list(urls));
    let when = match (ahead, busy) {
        (0, false) => "I am starting on it now".to_string(),
        (0, true) => "It starts when the current job is done".to_string(),
        (n, _) => format!("{} {} ahead of it", noun_count(n as u64, "job"), be(n as u64)),
    };
    paragraph(&[sentence(&what), sentence(&when)])
}

/// One URL's outcome in a job, from the job journal (`animal::UrlOutcome` as JSON).
pub fn outcome(o: &Value) -> String {
    let url = o["url"].as_str().unwrap_or("?");
    let status = o["status"].as_u64();
    match o["outcome"].as_str() {
        Some("already_stored") => sentence(&format!("I already had {url}, so I did not fetch it again")),
        Some("fetched") if status.is_some_and(|s| s >= 400) => {
            could_not("get", url, &format!("the server answered HTTP {}", status.unwrap_or(0)))
        }
        Some("fetched") => sentence(&format!(
            "I fetched {url} (HTTP {}, {} bytes)",
            status.unwrap_or(0),
            thousands(o["bytes"].as_u64().unwrap_or(0))
        )),
        Some("disallowed") => could_not(
            "fetch",
            url,
            &match o["rule"].as_str() {
                Some(r) => format!("robots.txt disallows it (rule {r})"),
                None => "robots.txt could not be read, so the site is closed for now".into(),
            },
        ),
        Some("deferred") => could_not("fetch", url, "its Crawl-delay is longer than my limit"),
        _ => could_not(
            "fetch",
            url,
            &format!("the request failed: {}", o["error"].as_str().unwrap_or("unknown error").trim_end_matches('.')),
        ),
    }
}

/// The status question: what runs now, what waits, how the last job ended.
pub fn status(s: &Value) -> String {
    let mut out = vec![];
    let waiting = s["waiting"].as_u64().unwrap_or(0);
    match s.get("running").filter(|r| !r.is_null()) {
        Some(r) => {
            let total = r["urls"].as_u64().unwrap_or(0);
            let done = r["done"].as_u64().unwrap_or(0);
            let now = r["current"].as_str().map(|u| format!(", now {u}")).unwrap_or_default();
            out.push(sentence(&format!(
                "I am working on {} (started {}): {done} of {} {} done{now}",
                r["job"].as_str().unwrap_or("a job"),
                utc(r["started_unix"].as_u64().unwrap_or(0)),
                noun_count(total, "page"),
                be(total)
            )));
        }
        None => out.push(sentence("I am not fetching anything now")),
    }
    out.push(sentence(&if waiting == 0 {
        "no job is waiting".to_string()
    } else {
        format!("{} {} waiting", noun_count(waiting, "job"), be(waiting))
    }));
    if let Some(f) = s.get("last").filter(|f| !f.is_null()) {
        out.push(finished(f));
    }
    paragraph(&out)
}

/// A finished job in the past tense, with each URL that did not end as fetched.
pub fn finished(f: &Value) -> String {
    let job = f["job"].as_str().unwrap_or("a job");
    let when = utc(f["finished_unix"].as_u64().unwrap_or(0));
    let empty = vec![];
    let outcomes = f["outcomes"].as_array().unwrap_or(&empty);
    let fetched = outcomes
        .iter()
        .filter(|o| o["outcome"] == "fetched" && o["status"].as_u64().is_some_and(|s| s < 400))
        .count() as u64;
    let mut out = vec![];
    match f["state"].as_str() {
        Some("done") if f["kind"] == "robots" => out.push(sentence(&format!("my last job, {job}, checked robots.txt at {when}"))),
        Some("done") => out.push(sentence(&format!(
            "my last job, {job}, finished at {when}: I {} {} of {}",
            past("fetch"),
            fetched,
            noun_count(outcomes.len() as u64 + f["not_started"].as_u64().unwrap_or(0), "page")
        ))),
        Some(state) => out.push(sentence(&format!(
            "my last job, {job}, {} at {when} after {} of {}",
            match state {
                "stopped" => "stopped on request",
                "interrupted" => "was interrupted by a restart",
                _ => "failed",
            },
            outcomes.len(),
            noun_count(outcomes.len() as u64 + f["not_started"].as_u64().unwrap_or(0), "page")
        ))),
        None => {}
    }
    for o in outcomes {
        let good = o["outcome"] == "fetched" && o["status"].as_u64().is_some_and(|s| s < 400);
        if !good {
            out.push(outcome(o));
        }
    }
    paragraph(&out)
}

/// Store counts, optionally for one host.
pub fn stats(s: &Value) -> String {
    let pages = s["pages"].as_u64().unwrap_or(0);
    let host = s["host"].as_str();
    let mut out = vec![match host {
        Some(h) => sentence(&format!("I have stored {} from {h}", noun_count(pages, "page"))),
        None => sentence(&format!(
            "I have stored {} from {} ({} bytes); the store keeps {} compressed bytes in all",
            noun_count(pages, "page"),
            noun_count(s["hosts"].as_u64().unwrap_or(0), "host"),
            thousands(s["page_bytes"].as_u64().unwrap_or(0)),
            thousands(s["frame_bytes"].as_u64().unwrap_or(0))
        )),
    }];
    let (dis, err) = (s["disallowed"].as_u64().unwrap_or(0), s["failed"].as_u64().unwrap_or(0));
    if dis + err > 0 {
        let mut why = vec![];
        if dis > 0 {
            why.push(format!("robots.txt disallowed {dis}"));
        }
        if err > 0 {
            why.push(format!("{err} failed"));
        }
        out.push(sentence(&format!(
            "I could not fetch {} more {}: {}",
            dis + err,
            noun(dis + err, "page"),
            list(&why)
        )));
    }
    paragraph(&out)
}

/// A stored page summary, or why there is none.
pub fn page(p: &Value) -> String {
    let url = p["url"].as_str().unwrap_or("?");
    match p["state"].as_str() {
        Some("running") => sentence(&format!("I am fetching {url} right now; ask me again in a moment")),
        Some("queued") => sentence(&format!("I have not fetched {url} yet; it is waiting in {}", p["job"].as_str().unwrap_or("the queue"))),
        Some("absent") => sentence(&format!("I have not fetched {url}")),
        Some("record") => {
            let r = &p["record"];
            if r["outcome"] != "fetched" || r["status"].as_u64().is_some_and(|s| s >= 400) {
                return outcome(&serde_json::json!({"url":url,"outcome":r["outcome"],"status":r["status"],
                    "rule":r["robots"]["rule"],"error":r["error"]}));
            }
            let mut out = vec![sentence(&format!(
                "I fetched {url} at {}: HTTP {}, {}, {} bytes",
                utc(r["fetched_unix"].as_u64().unwrap_or(0)),
                r["status"].as_u64().unwrap_or(0),
                r["content_type"].as_str().unwrap_or("no content type"),
                thousands(r["body_bytes"].as_u64().unwrap_or(0))
            ))];
            if let Some(f) = r["final_url"].as_str().filter(|f| *f != url) {
                out.push(sentence(&format!("It redirected to {f}")));
            }
            if let Some(t) = p["title"].as_str().filter(|t| !t.is_empty()) {
                out.push(sentence(&format!("Its title is “{t}”")));
            }
            if let Some(n) = p["links"].as_u64() {
                out.push(sentence(&format!("It links to {}", noun_count(n, "page"))));
            }
            if r["truncated"] == true {
                out.push(sentence("I kept only the first part of it, as my size limit says"));
            }
            paragraph(&out)
        }
        _ => sentence(&format!("I know nothing about {url}")),
    }
}

/// The whole answer to a prompt: each action's sentence, or why nothing was done.
pub fn report(r: &Report) -> String {
    if let Some(f) = &r.failure {
        let mut reason = f.reason.trim_end_matches('.').to_string();
        if let Some(c) = reason.get(..1) {
            if reason.get(1..2).is_some_and(|n| n.chars().all(|ch| ch.is_lowercase() || ch == ' ')) {
                reason = c.to_lowercase() + &reason[1..];
            }
        }
        let done: Vec<String> = r.results.iter().filter_map(|x| x["output"]["answer"].as_str().map(str::to_string)).collect();
        let head = if f.stage == "execution_failure" {
            sentence(&format!("I could not finish this request: {reason}"))
        } else {
            sentence(&format!("I did not act on this request because I could not read it as one I can do: {reason}"))
        };
        let review = r.followups.first().and_then(|c| c["candidate_id"].as_str()).map(|id| {
            sentence(&format!("I kept it for review as candidate {}", &id[..id.len().min(12)]))
        });
        let mut out = done;
        out.push(head);
        out.extend(review);
        return paragraph(&out);
    }
    let parts: Vec<String> = r.results.iter().filter_map(|x| x["output"]["answer"].as_str().map(str::to_string)).collect();
    if parts.is_empty() {
        return sentence("I understood the request, but nothing was executed");
    }
    paragraph(&parts)
}

/// A caller that is not on the allowlist.
pub fn caller_refused(caller: &str, callers: &[String]) -> String {
    let list: Vec<&str> = callers.iter().map(String::as_str).collect();
    format!("I did not act on this request because {caller} is not among the callers I answer ({}).", list.join(", "))
}

/// The text of `<title>` in an HTML body: entities for &, <, >, quotes decoded, whitespace
/// collapsed, control characters dropped, at most 160 characters.
pub fn title(html: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(&html[..html.len().min(256 * 1024)]);
    let lower = text.to_ascii_lowercase();
    let a = lower.find("<title")?;
    let b = a + lower[a..].find('>')? + 1;
    let e = b + lower[b..].find("</title")?;
    let raw = text[b..e]
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'");
    let t: String = raw.split_whitespace().collect::<Vec<_>>().join(" ").chars().filter(|c| !c.is_control()).take(160).collect();
    Some(t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn numbers_dates_and_titles() {
        assert_eq!(thousands(0), "0");
        assert_eq!(thousands(999), "999");
        assert_eq!(thousands(1000), "1,000");
        assert_eq!(thousands(1_234_567), "1,234,567");
        assert_eq!(utc(0), "1970-01-01 00:00 UTC");
        assert_eq!(utc(1_793_772_420), "2026-11-04 06:07 UTC");
        assert_eq!(utc(951_782_400), "2000-02-29 00:00 UTC");
        assert_eq!(title(b"<html><TITLE>\n Mova &amp; friends </title>").as_deref(), Some("Mova & friends"));
        assert_eq!(title(b"<p>no title</p>"), None);
    }
    #[test]
    fn robots_answers() {
        let base = json!({"url":"https://a.example/x","token":"MovaBot","max_crawl_delay_secs":60,"basis":"fetched_now",
            "robots":{"robots_url":"https://a.example/robots.txt","source":"parsed","group":"*","allowed":true,"rule":"Allow: /","crawl_delay":null},
            "decision":"allowed"});
        assert_eq!(robots(&base), "Yes, https://a.example/robots.txt allows MovaBot to fetch https://a.example/x (rule Allow: /). I have just read robots.txt.");
        let mut no = base.clone();
        no["decision"] = json!("disallowed");
        no["robots"]["rule"] = json!("Disallow: /x");
        assert!(robots(&no).starts_with("No, https://a.example/robots.txt disallows https://a.example/x for MovaBot (rule Disallow: /x)."));
        let mut down = base.clone();
        down["decision"] = json!("disallowed");
        down["robots"] = json!({"robots_url":"https://a.example/robots.txt","source":"unreachable (HTTP 503)","allowed":false,"rule":null});
        assert!(robots(&down).starts_with("No, I could not read https://a.example/robots.txt (HTTP 503), so I keep away"));
        let mut gone = base.clone();
        gone["robots"] = json!({"robots_url":"https://a.example/robots.txt","source":"allow_all (HTTP 404)","allowed":true,"rule":null});
        assert!(robots(&gone).starts_with("Yes, https://a.example/robots.txt is unavailable (HTTP 404), so MovaBot may fetch"));
        let mut slow = base.clone();
        slow["decision"] = json!("deferred");
        slow["robots"]["crawl_delay"] = json!(120.0);
        assert!(robots(&slow).contains("Crawl-delay of 120 seconds is longer than my limit of 60 seconds"));
        let mut stored = base;
        stored["basis"] = json!("stored");
        stored["robots_fetched_unix"] = json!(1_793_772_420u64);
        assert!(robots(&stored).ends_with("I read robots.txt at 2026-11-04 06:07 UTC."));
    }
    #[test]
    fn queue_status_stats_and_pages() {
        let one = vec!["https://a.example/".to_string()];
        assert_eq!(fetch_queued("job-1", &one, 0, false), "I have queued 1 page for fetching (job-1): https://a.example/. I am starting on it now.");
        let two = vec!["https://a.example/".to_string(), "https://b.example/".to_string()];
        assert_eq!(
            fetch_queued("job-2", &two, 2, true),
            "I have queued 2 pages for fetching (job-2): https://a.example/ and https://b.example/. 2 jobs are ahead of it."
        );
        assert!(fetch_queued("j", &one, 1, true).ends_with("1 job is ahead of it."));
        assert!(fetch_queued("j", &one, 0, true).ends_with("It starts when the current job is done."));
        let idle = json!({"waiting":0,"running":null,"last":{"job":"j1","kind":"fetch","state":"done","finished_unix":0,"not_started":0,
            "outcomes":[{"url":"https://a.example/","outcome":"fetched","status":200,"bytes":1200},
                        {"url":"https://a.example/p","outcome":"disallowed","rule":"Disallow: /p"},
                        {"url":"https://a.example/m","outcome":"fetched","status":404}]}});
        assert_eq!(
            status(&idle),
            "I am not fetching anything now. No job is waiting. My last job, j1, finished at 1970-01-01 00:00 UTC: I fetched 1 of 3 pages. \
I could not fetch https://a.example/p because robots.txt disallows it (rule Disallow: /p). \
I could not get https://a.example/m because the server answered HTTP 404."
        );
        let busy = json!({"waiting":1,"running":{"job":"j2","started_unix":60,"urls":3,"done":1,"current":"https://a.example/2"}});
        assert_eq!(
            status(&busy),
            "I am working on j2 (started 1970-01-01 00:01 UTC): 1 of 3 pages are done, now https://a.example/2. 1 job is waiting."
        );
        assert_eq!(
            stats(&json!({"pages":1,"hosts":1,"page_bytes":5548,"frame_bytes":2151,"disallowed":2,"failed":1})),
            "I have stored 1 page from 1 host (5,548 bytes); the store keeps 2,151 compressed bytes in all. I could not fetch 3 more pages: robots.txt disallowed 2 and 1 failed."
        );
        assert_eq!(stats(&json!({"pages":2,"host":"a.example","disallowed":0,"failed":0})), "I have stored 2 pages from a.example.");
        assert_eq!(page(&json!({"url":"https://a.example/","state":"absent"})), "I have not fetched https://a.example/.");
        let rec = json!({"url":"https://a.example/","state":"record","title":"Hello","links":1,
            "record":{"outcome":"fetched","status":200,"content_type":"text/html","body_bytes":1234,"fetched_unix":0,"final_url":"https://a.example/","truncated":false}});
        assert_eq!(
            page(&rec),
            "I fetched https://a.example/ at 1970-01-01 00:00 UTC: HTTP 200, text/html, 1,234 bytes. Its title is “Hello”. It links to 1 page."
        );
    }
}
