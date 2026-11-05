//! Narrow Markdown section grammar shared by groupant references and consolidation.
//! Plain column-zero `## Name` lines outside backtick/tilde fences delimit sections;
//! this is not a Markdown renderer (no setext, HTML, list or container parsing).

/// One line of a Markdown text: bytes without terminator, byte span with terminator,
/// and whether it is a fence delimiter or lies inside a fence.
pub struct Line<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
    pub fenced: bool,
}

/// Splits text into lines and marks fenced ones. A fence opens with at least three
/// backticks or tildes indented by zero to three spaces (backtick info strings cannot
/// contain backticks) and closes with the same marker, at least as long, followed only
/// by spaces or tabs. An unclosed fence marks every remaining line.
pub fn lines(text: &str) -> Vec<Line<'_>> {
    let mut out = Vec::new();
    let mut fence: Option<(u8, usize)> = None;
    let mut start = 0;
    for raw in text.split_inclusive('\n') {
        let end = start + raw.len();
        let line = raw
            .strip_suffix('\n')
            .map(|l| l.strip_suffix('\r').unwrap_or(l))
            .unwrap_or(raw);
        let indent = line.bytes().take_while(|b| *b == b' ').count();
        let candidate = &line[indent..];
        let marker = candidate.as_bytes().first().copied().unwrap_or(0);
        let run = candidate.bytes().take_while(|b| *b == marker).count();
        let fenced = if let Some((opening, length)) = fence {
            if indent <= 3
                && marker == opening
                && run >= length
                && candidate[run..].trim_matches([' ', '\t']).is_empty()
            {
                fence = None;
            }
            true
        } else if indent <= 3
            && matches!(marker, b'`' | b'~')
            && run >= 3
            && (marker != b'`' || !candidate[run..].contains('`'))
        {
            fence = Some((marker, run));
            true
        } else {
            false
        };
        out.push(Line {
            text: line,
            start,
            end,
            fenced,
        });
        start = end;
    }
    out
}

/// True when the text ends inside an open fence.
pub fn unclosed_fence(text: &str) -> bool {
    let mut fence: Option<(u8, usize)> = None;
    for raw in text.lines() {
        let indent = raw.bytes().take_while(|b| *b == b' ').count();
        let candidate = &raw[indent..];
        let marker = candidate.as_bytes().first().copied().unwrap_or(0);
        let run = candidate.bytes().take_while(|b| *b == marker).count();
        if let Some((opening, length)) = fence {
            if indent <= 3
                && marker == opening
                && run >= length
                && candidate[run..].trim_matches([' ', '\t']).is_empty()
            {
                fence = None;
            }
        } else if indent <= 3
            && matches!(marker, b'`' | b'~')
            && run >= 3
            && (marker != b'`' || !candidate[run..].contains('`'))
        {
            fence = Some((marker, run));
        }
    }
    fence.is_some()
}

/// Names of all plain H2 headings (`## Name` outside fences), in order.
pub fn h2_names(text: &str) -> Vec<&str> {
    lines(text)
        .into_iter()
        .filter(|l| !l.fenced)
        .filter_map(|l| l.text.strip_prefix("## "))
        .collect()
}

/// Number of exact `## {section}` lines outside fences.
pub fn heading_count(text: &str, section: &str) -> usize {
    let heading = format!("## {section}");
    lines(text)
        .iter()
        .filter(|l| !l.fenced && l.text == heading)
        .count()
}

/// Bytes of the unique section `## {section}`: from the start of its heading line up to
/// the next plain H2 line or the end of the text, without trailing blank lines and
/// without the final line terminator. None unless exactly one such heading exists.
pub fn section_bytes<'a>(text: &'a str, section: &str) -> Option<&'a str> {
    if heading_count(text, section) != 1 {
        return None;
    }
    let heading = format!("## {section}");
    let all = lines(text);
    let first = all.iter().position(|l| !l.fenced && l.text == heading)?;
    let mut last = first;
    for (i, l) in all.iter().enumerate().skip(first + 1) {
        if !l.fenced && l.text.starts_with("## ") {
            break;
        }
        if !l.text.trim().is_empty() {
            last = i;
        }
    }
    let start = all[first].start;
    let end = all[last].start + all[last].text.len();
    Some(&text[start..end])
}

/// Whether the unique section `## {section}` holds anything beyond headings and blank
/// lines (text or fenced lines). None unless exactly one such heading exists.
pub fn section_has_body(text: &str, section: &str) -> Option<bool> {
    let bytes = section_bytes(text, section)?;
    Some(
        lines(bytes)
            .iter()
            .skip(1)
            .any(|l| l.fenced || (!l.text.trim().is_empty() && heading_level(l.text).is_none())),
    )
}

/// Splits a reference `file#Section` into the file and an optional section name.
/// The section is everything after the first `#`; it must be nonempty.
pub fn split_reference(reference: &str) -> (&str, Option<&str>) {
    match reference.split_once('#') {
        Some((file, section)) => (file, Some(section)),
        None => (reference, None),
    }
}

/// GitHub-style heading anchor: lowercase, keep alphanumerics, `-` and `_`, spaces to `-`.
pub fn anchor(name: &str) -> String {
    name.trim()
        .chars()
        .flat_map(char::to_lowercase)
        .filter_map(|c| match c {
            ' ' => Some('-'),
            c if c.is_alphanumeric() || c == '-' || c == '_' => Some(c),
            _ => None,
        })
        .collect()
}

/// ATX heading level of a line (zero to three leading spaces, one to six `#`,
/// then end of line, space or tab), or None.
pub fn heading_level(line: &str) -> Option<usize> {
    let indent = line.bytes().take_while(|b| *b == b' ').count();
    if indent > 3 {
        return None;
    }
    let rest = &line[indent..];
    let level = rest.bytes().take_while(|b| *b == b'#').count();
    if !(1..=6).contains(&level) {
        return None;
    }
    match rest.as_bytes().get(level) {
        None | Some(b' ') | Some(b'\t') => Some(level),
        _ => None,
    }
}

/// Smallest ATX heading level outside fences.
pub fn min_heading_level(text: &str) -> Option<usize> {
    lines(text)
        .iter()
        .filter(|l| !l.fenced)
        .filter_map(|l| heading_level(l.text))
        .min()
}

/// Adds `shift` hashes to every ATX heading outside fences. Lines are otherwise
/// unchanged, so the transformation is reversed by removing the same hashes.
/// Headings pushed past level six stop being headings; `overflow` counts them.
pub fn demote(text: &str, shift: usize) -> (String, usize) {
    let mut out = String::with_capacity(text.len() + 16);
    let mut overflow = 0;
    for l in lines(text) {
        let raw = &text[l.start..l.end];
        match (!l.fenced).then(|| heading_level(l.text)).flatten() {
            Some(level) if shift > 0 => {
                let indent = l.text.bytes().take_while(|b| *b == b' ').count();
                out.push_str(&raw[..indent]);
                out.push_str(&"#".repeat(shift));
                out.push_str(&raw[indent..]);
                if level + shift > 6 {
                    overflow += 1;
                }
            }
            _ => out.push_str(raw),
        }
    }
    (out, overflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sections_follow_the_plain_h2_grammar() {
        let text = "# T\n\n## A\nx\n```\n## B\n```\n\n\n## B\ny\n\n";
        assert_eq!(h2_names(text), ["A", "B"]);
        assert_eq!(section_bytes(text, "A"), Some("## A\nx\n```\n## B\n```"));
        assert_eq!(section_bytes(text, "B"), Some("## B\ny"));
        assert_eq!(section_bytes(text, "C"), None);
        let twice = "## A\n## A\n";
        assert_eq!(heading_count(twice, "A"), 2);
        assert_eq!(section_bytes(twice, "A"), None); // negative control: duplicates fail
        assert!(unclosed_fence("~~~\n## A\n"));
        assert!(!unclosed_fence("~~~\n## A\n~~~~\n"));
        assert_eq!(
            split_reference("model.md#Forward model"),
            ("model.md", Some("Forward model"))
        );
        assert_eq!(
            anchor("Prospective model — 2026"),
            "prospective-model--2026"
        );
    }

    #[test]
    fn a_body_is_more_than_headings_and_blank_lines() {
        let text = "## A\n\n#### Sub\n\n## B\nx\n## C\n```\n```\n## D\n";
        assert_eq!(section_has_body(text, "A"), Some(false)); // negative control
        assert_eq!(section_has_body(text, "B"), Some(true));
        assert_eq!(section_has_body(text, "C"), Some(true));
        assert_eq!(section_has_body(text, "D"), Some(false));
        assert_eq!(section_has_body(text, "E"), None);
    }

    #[test]
    fn demotion_is_reversible_and_skips_fences() {
        let text = "# Top\n## Sub\n```\n# code\n```\n####### seven\n#tag\n";
        let (out, overflow) = demote(text, 2);
        assert_eq!(
            out,
            "### Top\n#### Sub\n```\n# code\n```\n####### seven\n#tag\n"
        );
        assert_eq!(overflow, 0);
        assert_eq!(min_heading_level(&out), Some(3));
        let (deep, overflow) = demote("##### five\n", 2);
        assert_eq!((deep.as_str(), overflow), ("####### five\n", 1));
    }
}
