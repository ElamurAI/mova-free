//! Numbered citations in a consultant's text: the split into cited sentences and a reference list
//! (shared with `mova-topic-corpus`, whose draft claims use it), the check that every citation
//! names one of the sources the text was given, and claims followed to those sources. A claim is a
//! body sentence with its citation numbers; it is `sourced` when a cited source holds a sentence
//! whose content words overlap it at least `min_support` (Jaccard), `weak` when it cites sources
//! but none holds such a sentence, `uncited` without a citation. Overlap is lexical support, not
//! entailment.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

/// Split a draft into body sentences with their citation numbers, and its numbered reference list.
pub fn split(text: &str) -> (Vec<(String, Vec<usize>)>, BTreeMap<usize, String>) {
    let mut body = String::new();
    let mut refs: BTreeMap<usize, String> = BTreeMap::new();
    let mut in_refs = false;
    for line in text.lines() {
        let t = line.trim();
        let head = t.trim_start_matches('#').trim().trim_end_matches(':').to_lowercase();
        if (t.starts_with('#') || t.len() < 30) && matches!(head.as_str(), "references" | "bibliography" | "sources" | "works cited") {
            in_refs = true;
            continue;
        }
        if in_refs {
            if t.starts_with('#') {
                in_refs = false;
                continue;
            }
            let t = t.trim_start_matches(['-', '*', ' ']);
            let num: String = t.trim_start_matches('[').chars().take_while(char::is_ascii_digit).collect();
            if !num.is_empty() {
                let rest = t.trim_start_matches('[')[num.len()..].trim_start_matches([']', '.', ')', ' ']).to_string();
                if let Ok(n) = num.parse() {
                    refs.insert(n, rest);
                }
            }
            continue;
        }
        if t.starts_with('#') {
            body.push_str("\n\n");
            continue;
        }
        body.push_str(t);
        body.push(if t.is_empty() { '\n' } else { ' ' });
    }
    let mut out = vec![];
    for para in body.split("\n\n") {
        for s in world::quant::sentences(para.trim()) {
            let mut nums = vec![];
            let mut rest = s.as_str();
            while let Some(i) = rest.find('[') {
                let Some(j) = rest[i..].find(']') else { break };
                let inner = &rest[i + 1..i + j];
                if inner.chars().all(|c| c.is_ascii_digit() || matches!(c, ',' | ' ' | '-' | '–')) {
                    for part in inner.split(',') {
                        let p = part.trim();
                        if let Some((a, b)) = p.split_once(['-', '–']) {
                            if let (Ok(a), Ok(b)) = (a.trim().parse::<usize>(), b.trim().parse::<usize>()) {
                                if a <= b && b - a < 20 {
                                    nums.extend(a..=b);
                                }
                            }
                        } else if let Ok(n) = p.parse() {
                            nums.push(n);
                        }
                    }
                }
                rest = &rest[i + j + 1..];
            }
            let clean: String = {
                let mut c = String::new();
                let mut depth = 0;
                for ch in s.chars() {
                    match ch {
                        '[' => depth += 1,
                        ']' if depth > 0 => depth -= 1,
                        _ if depth == 0 => c.push(ch),
                        _ => {}
                    }
                }
                c.split_whitespace().collect::<Vec<_>>().join(" ").replace(" .", ".").replace(" ,", ",")
            };
            if clean.split_whitespace().count() >= 4 {
                nums.sort_unstable();
                nums.dedup();
                out.push((clean, nums));
            }
        }
    }
    (out, refs)
}

/// One source a text was given, under its number.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Given {
    pub n: usize,
    /// The citation id (a dossier text id or a library id).
    pub id: String,
    pub kind: String,
    pub path: std::path::PathBuf,
}

/// Why a consultant's text is refused (it is kept in quarantine, never registered).
#[derive(Clone, Debug, PartialEq)]
pub enum Refusal {
    /// A body citation or a reference-list entry names a number that was not given.
    Unknown(Vec<usize>),
    /// No sentence carries a citation.
    Uncited,
}

impl Refusal {
    pub fn reason(&self) -> String {
        match self {
            Refusal::Unknown(v) => format!("citations to sources that were not given: {v:?}"),
            Refusal::Uncited => "no sentence cites a given source".into(),
        }
    }
}

/// The text's cited sentences, checked against the given sources.
pub fn check(text: &str, given: &[Given]) -> std::result::Result<(Vec<(String, Vec<usize>)>, BTreeMap<usize, String>), Refusal> {
    let (body, refs) = split(text);
    let known: BTreeSet<usize> = given.iter().map(|g| g.n).collect();
    let unknown: BTreeSet<usize> = body.iter().flat_map(|(_, ns)| ns.iter().copied()).chain(refs.keys().copied()).filter(|n| !known.contains(n)).collect();
    if !unknown.is_empty() {
        return Err(Refusal::Unknown(unknown.into_iter().collect()));
    }
    if !body.iter().any(|(_, ns)| !ns.is_empty()) {
        return Err(Refusal::Uncited);
    }
    Ok((body, refs))
}

/// Content words for support: alphanumeric words of three or more letters, lowercased, without
/// stop words.
pub fn words(s: &str) -> BTreeSet<String> {
    s.split(|c: char| !c.is_alphanumeric()).filter(|w| w.chars().count() >= 3).map(str::to_lowercase).filter(|w| !crate::reader::STOP.contains(&w.as_str())).collect()
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Claim {
    pub id: String,
    pub text: String,
    /// Citation ids of the given sources it cites, in number order.
    pub cites: Vec<String>,
    /// `sourced`, `weak` or `uncited`.
    pub status: String,
    /// The best supporting sentence of a cited source: {source, sentence, overlap}.
    pub support: Option<Value>,
}

/// Claims of a checked text, each followed to the sentences of the sources it cites
/// (`texts`: number -> the source's text).
pub fn claims(prefix: &str, body: &[(String, Vec<usize>)], given: &[Given], texts: &BTreeMap<usize, String>, min_support: f64) -> Vec<Claim> {
    let by_n: BTreeMap<usize, &Given> = given.iter().map(|g| (g.n, g)).collect();
    let sentences: BTreeMap<usize, Vec<(String, BTreeSet<String>)>> = texts.iter().map(|(n, t)| (*n, world::quant::sentences(&t.replace('\n', " ")).into_iter().map(|s| { let w = words(&s); (s, w) }).collect())).collect();
    let mut out = vec![];
    for (i, (s, ns)) in body.iter().enumerate() {
        let cw = words(s);
        let mut best: Option<(f64, usize, String)> = None;
        for n in ns {
            for (x, xw) in sentences.get(n).map(Vec::as_slice).unwrap_or(&[]) {
                let inter = cw.intersection(xw).count() as f64;
                let uni = cw.union(xw).count().max(1) as f64;
                let score = (inter / uni * 1000.0).round() / 1000.0;
                if best.as_ref().is_none_or(|b| score > b.0) {
                    best = Some((score, *n, x.clone()));
                }
            }
        }
        let status = if ns.is_empty() {
            "uncited"
        } else if best.as_ref().is_some_and(|b| b.0 >= min_support) {
            "sourced"
        } else {
            "weak"
        };
        let cites = ns.iter().filter_map(|n| by_n.get(n).map(|g| g.id.clone())).collect();
        let support = best.map(|(o, n, x)| json!({"source": by_n.get(&n).map(|g| g.id.clone()), "n": n, "sentence": x, "overlap": o}));
        out.push(Claim { id: format!("{prefix}-c{:04}", i + 1), text: s.clone(), cites, status: status.into(), support });
    }
    out
}

/// A claim's identity across epochs: its content words, sorted.
pub fn claim_key(text: &str) -> String {
    words(text).into_iter().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn given() -> Vec<Given> {
        (1..=3).map(|n| Given { n, id: format!("doc/0-original-{n}"), kind: "original".into(), path: std::path::PathBuf::from("/x") }).collect()
    }

    #[test]
    fn citations_to_unknown_sources_are_refused() {
        let ok = "# A\n\nThe parser reads every word once [1]. It attaches heads [2, 3].\n\n## References\n\n[1] a\n[2] b\n[3] c\n";
        assert!(check(ok, &given()).is_ok());
        // Negative controls: a body citation and a reference entry beyond the given sources.
        assert_eq!(check("The parser reads every word once [4].\n", &given()), Err(Refusal::Unknown(vec![4])));
        assert_eq!(check("The parser reads every word once [1].\n\n## References\n\n[1] a\n[7] invented\n", &given()), Err(Refusal::Unknown(vec![7])));
        assert_eq!(check("The parser reads every word once.\n", &given()), Err(Refusal::Uncited));
    }

    #[test]
    fn claims_follow_citations_to_their_sources() {
        let texts: BTreeMap<usize, String> = [(1, "The parser reads each word of the sentence once. Nothing else.".to_string()), (2, "Bananas grow in warm places.".to_string())].into();
        let body = vec![("The parser reads each word once in a sentence.".to_string(), vec![1]), ("The parser reads each word once in a sentence too.".to_string(), vec![2]), ("Plain words here without any source.".to_string(), vec![])];
        let c = claims("e1", &body, &given(), &texts, 0.3);
        assert_eq!(c.iter().map(|x| x.status.as_str()).collect::<Vec<_>>(), vec!["sourced", "weak", "uncited"]);
        assert_eq!(c[0].cites, vec!["doc/0-original-1".to_string()]);
        assert_eq!(c[0].support.as_ref().unwrap()["n"], 1);
    }
}
