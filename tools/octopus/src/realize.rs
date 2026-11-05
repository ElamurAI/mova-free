//! A small deterministic English realizer for animals' answers: counted noun phrases with number
//! agreement, verb agreement, past tense, lists and sentence punctuation. Plurals and past forms
//! come from Mova's own morphology (`en::morph::Morph`, the built-in dictionary and its suffix
//! rules); no language model is involved. It realizes what the caller states, nothing more.
use en::gram::Tag;
use en::morph::Morph;
use crate::Report;

/// `noun_count(1, "page")` = "1 page"; `noun_count(3, "page")` = "3 pages"; 0 takes the plural.
pub fn noun_count(n: u64, lemma: &str) -> String {
    format!("{n} {}", noun(n, lemma))
}

/// The noun form agreeing with `n`.
pub fn noun(n: u64, lemma: &str) -> String {
    if n == 1 { lemma.to_string() } else { Morph.inflect(lemma, Tag::NNS) }
}

/// "is" for one, "are" otherwise.
pub fn be(n: u64) -> &'static str {
    if n == 1 { "is" } else { "are" }
}

/// "was" for one, "were" otherwise.
pub fn was(n: u64) -> &'static str {
    if n == 1 { "was" } else { "were" }
}

/// "has" for one, "have" otherwise.
pub fn has(n: u64) -> &'static str {
    if n == 1 { "has" } else { "have" }
}

/// Simple past of a verb lemma: "fetch" -> "fetched", "get" -> "got".
pub fn past(lemma: &str) -> String {
    Morph.inflect(lemma, Tag::VBD)
}

/// Past participle: "store" -> "stored", "write" -> "written".
pub fn participle(lemma: &str) -> String {
    Morph.inflect(lemma, Tag::VBN)
}

/// "A", "A and B", "A, B and C".
pub fn list<S: AsRef<str>>(items: &[S]) -> String {
    match items {
        [] => String::new(),
        [a] => a.as_ref().to_string(),
        [rest @ .., last] => format!(
            "{} and {}",
            rest.iter().map(AsRef::as_ref).collect::<Vec<_>>().join(", "),
            last.as_ref()
        ),
    }
}

/// Capitalize the first letter and end with a full stop unless the text already ends a sentence.
pub fn sentence(text: &str) -> String {
    let t = text.trim();
    let mut c = t.chars();
    let mut out = match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => return String::new(),
    };
    if !out.ends_with(['.', '!', '?']) {
        out.push('.');
    }
    out
}

/// "I could not fetch X because robots.txt disallows it." The reason is a clause, not a label.
pub fn could_not(verb: &str, object: &str, because: &str) -> String {
    sentence(&format!("I could not {verb} {object} because {because}"))
}

/// Join sentences with single spaces.
pub fn paragraph<S: AsRef<str>>(sentences: &[S]) -> String {
    sentences.iter().map(AsRef::as_ref).filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")
}

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

/// The whole answer of an animal to one prompt: each executed action's `output.answer`
/// sentence, or why nothing (more) was done, with the review candidate that keeps the prompt.
pub fn report(r: &Report) -> String {
    let done: Vec<String> = r.results.iter().filter_map(|x| x["output"]["answer"].as_str().map(str::to_string)).collect();
    let Some(f) = &r.failure else {
        if done.is_empty() {
            return sentence("I understood the request, but nothing was executed");
        }
        return paragraph(&done);
    };
    let mut reason = f.reason.trim_end_matches('.').to_string();
    if let Some(c) = reason.get(..1) {
        if reason.get(1..2).is_some_and(|n| n.chars().all(|ch| ch.is_lowercase() || ch == ' ')) {
            reason = c.to_lowercase() + &reason[1..];
        }
    }
    let head = if f.stage == "execution_failure" {
        sentence(&format!("I could not finish this request: {reason}"))
    } else {
        sentence(&format!("I did not act on this request because I could not read it as one I can do: {reason}"))
    };
    let review = r
        .followups
        .first()
        .and_then(|c| c["candidate_id"].as_str())
        .map(|id| sentence(&format!("I kept it for review as candidate {}", &id[..id.len().min(12)])));
    let mut out = done;
    out.push(head);
    out.extend(review);
    paragraph(&out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn agreement_lists_and_tense() {
        assert_eq!(noun_count(1, "page"), "1 page");
        assert_eq!(noun_count(0, "page"), "0 pages");
        assert_eq!(noun_count(3, "host"), "3 hosts");
        assert_eq!(noun_count(2, "segment"), "2 segments");
        assert_eq!(noun_count(2, "job"), "2 jobs");
        assert_eq!(noun_count(2, "byte"), "2 bytes");
        assert_eq!((be(1), be(2), was(1), was(0), has(1), has(5)), ("is", "are", "was", "were", "has", "have"));
        assert_eq!(past("fetch"), "fetched");
        assert_eq!(past("get"), "got");
        assert_eq!(past("queue"), "queued");
        assert_eq!(participle("store"), "stored");
        assert_eq!(list::<&str>(&[]), "");
        assert_eq!(list(&["a"]), "a");
        assert_eq!(list(&["a", "b"]), "a and b");
        assert_eq!(list(&["a", "b", "c"]), "a, b and c");
        assert_eq!(sentence("yes, it is allowed"), "Yes, it is allowed.");
        assert_eq!(sentence("is it?"), "Is it?");
        assert_eq!((thousands(0), thousands(999), thousands(1000), thousands(1_234_567)), ("0".into(), "999".into(), "1,000".into(), "1,234,567".into()));
        assert_eq!(utc(0), "1970-01-01 00:00 UTC");
        assert_eq!(utc(1_793_772_420), "2026-11-04 06:07 UTC");
        assert_eq!(utc(951_782_400), "2000-02-29 00:00 UTC");
        assert_eq!(
            could_not("fetch", "https://a.example/x", "robots.txt disallows it (Disallow: /x)"),
            "I could not fetch https://a.example/x because robots.txt disallows it (Disallow: /x)."
        );
    }
}
