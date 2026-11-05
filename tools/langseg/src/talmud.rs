//! The Jerusalem Talmud in Guggenheimer's edition (Sefaria JSON, CC BY): the original and the English translation
//! by address, `<Tractate>.<chapter>.<halakhah>.<segment>` (1-based, as Sefaria numbers them).
//!
//! - The original: tags, Sefaria's page marks and Guggenheimer's note anchors are taken off (`corpus::clean`), the
//!   heading `משנה:` / `הלכה:` tells the part (Mishnah or the Halakhah, the gemara) and is dropped; the rest is cut
//!   into language segments by the `Segmenter`.
//! - The English: the translation only. Guggenheimer's footnotes (`<i class="footnote">`, his commentary) and their
//!   markers are left out, other tags are taken off with their text kept, the heading `MISHNAH:` / `HALAKHAH:` is
//!   dropped.

use serde_json::Value;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Mishnah,
    Halakhah,
}

impl Part {
    pub fn name(self) -> &'static str {
        match self {
            Part::Mishnah => "mishnah",
            Part::Halakhah => "halakhah",
        }
    }
}

/// One address with its text as given: (chapter, halakhah, segment), 1-based.
pub type Addressed = Vec<((usize, usize, usize), String)>;

/// The `text` of a Sefaria JSON with three levels (chapter, halakhah, segment).
pub fn read(path: &Path) -> Result<Addressed, String> {
    let s = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let v: Value = serde_json::from_str(&s).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = Vec::new();
    let empty = Vec::new();
    for (c, ch) in v.get("text").and_then(Value::as_array).unwrap_or(&empty).iter().enumerate() {
        for (h, hal) in ch.as_array().unwrap_or(&empty).iter().enumerate() {
            for (g, seg) in hal.as_array().unwrap_or(&empty).iter().enumerate() {
                if let Some(t) = seg.as_str() {
                    out.push(((c + 1, h + 1, g + 1), t.to_string()));
                }
            }
        }
    }
    Ok(out)
}

/// The heading word that opens a part, taken off the text: (part, rest).
fn heading<'a>(text: &'a str, marks: [(&str, Part); 2]) -> (Option<Part>, &'a str) {
    let t = text.trim_start();
    for (m, p) in marks {
        if let Some(rest) = t.strip_prefix(m) {
            return (Some(p), rest.trim_start());
        }
    }
    (None, t)
}

/// The original text of one segment: clean, with its part heading taken off.
pub fn original(raw: &str) -> (Option<Part>, String) {
    let c = crate::corpus::clean(raw);
    let (p, rest) = heading(&c, [("משנה:", Part::Mishnah), ("הלכה:", Part::Halakhah)]);
    (p, rest.to_string())
}

/// The English translation of one segment without the footnotes: every tag is taken off, and the content of a
/// footnote (`class="footnote"`) or a footnote marker (`class="footnote-marker"`), nested tags included, is dropped.
pub fn english(raw: &str) -> (Option<Part>, String) {
    let mut out = String::with_capacity(raw.len());
    // one entry per open element: whether its content is dropped
    let mut stack: Vec<bool> = Vec::new();
    let mut rest = raw;
    while let Some(i) = rest.find('<') {
        if !stack.last().copied().unwrap_or(false) {
            out.push_str(&rest[..i]);
        }
        let Some(j) = rest[i..].find('>') else {
            rest = "";
            break;
        };
        let tag = &rest[i + 1..i + j];
        rest = &rest[i + j + 1..];
        let name: String = tag.trim_start_matches('/').chars().take_while(|c| c.is_ascii_alphanumeric()).collect::<String>().to_ascii_lowercase();
        if tag.starts_with('/') {
            stack.pop();
        } else if tag.ends_with('/') || matches!(name.as_str(), "br" | "img" | "hr" | "wbr") {
            if !stack.last().copied().unwrap_or(false) {
                out.push(' ');
            }
        } else {
            let note = tag.contains("class=\"footnote\"") || tag.contains("class=\"footnote-marker\"");
            stack.push(note || stack.last().copied().unwrap_or(false));
        }
    }
    if !stack.last().copied().unwrap_or(false) {
        out.push_str(rest);
    }
    let out = out
        .replace("&nbsp;", " ")
        .replace("&thinsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&");
    let out = out.split_whitespace().collect::<Vec<_>>().join(" ");
    let (p, rest) = heading(&out, [("MISHNAH:", Part::Mishnah), ("HALAKHAH:", Part::Halakhah)]);
    (p, rest.to_string())
}

/// The language segments of a text: (lang, text); a segment's text runs to the start of the next one, so the
/// punctuation between segments stays with the segment before it.
pub fn segments(sg: &crate::Segmenter, text: &str) -> Vec<(crate::Lang, String)> {
    let ss = sg.segment(text, 0);
    let mut out = Vec::new();
    for (i, s) in ss.iter().enumerate() {
        let end = ss.get(i + 1).map_or(text.len(), |n| n.start);
        let t = text[s.start..end].replace(['\t', '\n', '\r'], " ");
        let t = t.trim();
        if !t.is_empty() {
            out.push((s.lang, t.to_string()));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_drops_footnotes() {
        let raw = "<strong>MISHNAH:</strong> When does one read the <i>Shema</i> in the evening<sup class=\"footnote-marker\">1</sup><i class=\"footnote\">The verses <a class=\"refLink\" href=\"/x\"><i>Deut.</i> 6:4</a> say so.</i>? From the time&nbsp;the priests enter.";
        let (p, t) = english(raw);
        assert_eq!(p, Some(Part::Mishnah));
        assert_eq!(t, "When does one read the Shema in the evening? From the time the priests enter.");
        // negative control: a plain italic is kept, only a footnote is dropped
        let (p, t) = english("<i>Peah</i> is given<br/>at the end.");
        assert_eq!(p, None);
        assert_eq!(t, "Peah is given at the end.");
    }

    #[test]
    fn original_heading() {
        let (p, t) = original("<strong><big>הלכה:</big></strong> <i data-overlay=\"Vilna Pages\" data-value=\"1b\"></i>אִיתָא חָמֵי.");
        assert_eq!(p, Some(Part::Halakhah));
        assert_eq!(t, "אִיתָא חָמֵי.");
        let (p, _) = original("אִיתָא חָמֵי.");
        assert_eq!(p, None);
    }
}
