//! Stage 6: a consultant's draft articles → claims → sources. An Owl asks its own private terminal
//! (Sol) for a long draft article on its paper's topics with as many citations as possible; the
//! draft is internal and low quality by design — its value is the sources it names. Mova splits the
//! draft into claims, follows each claim's citations to originals (the library first; then a download
//! request for an open copy, or a page for Sea Spider), checks a claim against the cited text when we
//! hold it, and keeps every claim whose source is closed or not found as a claim «from Sol» with that
//! provenance. The draft never enters a description.
use crate::corpus::{Corpus, Doc};
use crate::graph::Neighbourhood;
use crate::library::{words, Library, RefEntry};
use crate::licence::Origin;
use crate::missing;
use crate::web::Target;
use anyhow::{anyhow, Result};
use mova_whale::terminal::{PrivateTerminal, Question};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub const SCHEMA: &str = "mova-sol-claims-v1";

/// The request for a draft article on the paper's topics.
pub fn prompt(g: &Neighbourhood, words: usize) -> String {
    let topics: Vec<String> = g.topics.iter().filter(|t| !t.name.is_empty()).map(|t| t.name.clone()).chain(g.concepts.iter().map(|c| c.replace('-', " "))).collect();
    let refs: Vec<String> = g.references.iter().filter_map(|r| r.entry.title.clone()).take(12).collect();
    let mut s = format!(
        "Write a long draft article (about {words} words, plain English, Markdown) on the topics of the paper «{}»{}{}. Topics: {}.\n\n",
        g.paper.title,
        if g.paper.authors.is_empty() { String::new() } else { format!(" by {}", g.paper.authors.join(", ")) },
        g.paper.year.map(|y| format!(" ({y})")).unwrap_or_default(),
        if topics.is_empty() { "the paper's own subject".into() } else { topics.join("; ") }
    );
    s.push_str("Cite as many original sources as you can: end every factual sentence with numbered citations like [1] or [2, 5]. Prefer primary sources (the papers that introduced a method, dataset or result) over surveys and blogs. ");
    s.push_str("Finish with a section «## References» listing every number once as «[n] Authors (Year). Title. Venue. DOI or arXiv id or ACL Anthology id or URL». Do not invent identifiers: leave one out when you are not sure. This is an internal working draft; quality of style does not matter, sources do.\n");
    if !refs.is_empty() {
        s.push_str(&format!("\nThe paper itself cites, among others: {}.\n", refs.join("; ")));
    }
    s
}

/// Ask the terminal for one draft; the answer is registered in the corpus as `sol:<id>` (internal,
/// low quality) and its text returned.
pub fn draft(c: &mut Corpus, t: &mut dyn PrivateTerminal, g: &Neighbourhood, id: &str, words: usize) -> Result<String> {
    let q = Question { id: id.into(), kind: "article".into(), subject: format!("article:{}", g.paper.id), text: prompt(g, words), context: vec![], items: vec![] };
    let a = t.ask(&q).map_err(|e| anyhow!("the terminal gave no draft ({}): {}", e.kind(), e.reason()))?;
    store(c, id, &format!("Draft article on the topics of «{}»", g.paper.title), &a.text, json!({"terminal": t.animal(), "terminal_question": a.terminal_id, "elapsed_ms": a.elapsed_ms}))?;
    Ok(a.text)
}

/// Register a draft text in the corpus (stored compressed; internal, low quality).
pub fn store(c: &mut Corpus, id: &str, title: &str, text: &str, how: Value) -> Result<()> {
    let mut d = Doc::new(&format!("sol:{id}"), "sol", Origin::Sol);
    d.title = title.into();
    d.roles = vec!["draft".into()];
    d.licence = "consultant draft (internal)".into();
    d.licence_from = "consultant".into();
    d.quality = Some("low".into());
    d.cite = json!({"draft": id, "how": how});
    // The reader needs a title and a part heading.
    let md = if text.trim_start().starts_with('#') { text.to_string() } else { format!("# {title}\n\n## Draft\n\n{text}") };
    c.register(d, &md)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Citation {
    pub n: usize,
    pub entry: RefEntry,
    /// `library_text`, `library_no_text`, `download_requested`, `web_requested`, `unresolved`.
    pub resolution: String,
    pub source: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Claim {
    pub id: String,
    pub text: String,
    pub citations: Vec<Citation>,
    /// `sourced` (a cited original we hold), `pending` (an original requested), `from_sol` (no
    /// available original: the claim is kept with the consultant as its only source).
    pub status: String,
    pub provenance: Value,
    /// The best matching sentence of a cited original we hold (content-word overlap).
    pub support: Option<Value>,
}

fn find_from(s: &str, pat: &str) -> Option<String> {
    let i = s.find(pat)? + pat.len();
    let t: String = s[i..].chars().take_while(|c| !c.is_whitespace() && !matches!(c, ',' | ';' | ')' | ']' | '>' | '"' | '«' | '»')).collect();
    let t = t.trim_end_matches(['.', '/']).to_string();
    (!t.is_empty()).then_some(t)
}

/// A reference-list line as a bibliography entry: ids (DOI, arXiv, ACL Anthology), URL, year, title.
pub fn parse_entry(raw: &str) -> RefEntry {
    let l = raw.trim();
    let mut e = RefEntry { raw: l.into(), ..Default::default() };
    let low = l.to_lowercase();
    if let Some(i) = low.find("10.") {
        let d: String = l[i..].chars().take_while(|c| !c.is_whitespace() && !matches!(c, ',' | ';' | ')' | ']' | '>' | '"')).collect();
        if d.contains('/') {
            e.doi = Some(d.trim_end_matches('.').to_lowercase());
        }
    }
    e.arxiv = find_from(&low, "arxiv.org/abs/").or_else(|| find_from(&low, "arxiv.org/pdf/")).or_else(|| find_from(&low, "arxiv:")).map(|a| a.trim_end_matches(".pdf").to_string()).filter(|a| a.chars().next().is_some_and(|c| c.is_ascii_digit() || c.is_ascii_alphabetic()));
    e.acl = find_from(l, "aclanthology.org/").or_else(|| find_from(l, "ACL Anthology ")).filter(|a| a.contains('-') || a.contains('.'));
    e.url = l.split_whitespace().find(|w| w.starts_with("http://") || w.starts_with("https://")).map(|w| w.trim_end_matches(['.', ',', ';', ')', '>']).to_string());
    e.year = l.split(|c: char| !c.is_ascii_digit()).filter(|x| x.len() == 4).filter_map(|x| x.parse::<i64>().ok()).find(|y| (1900..=2100).contains(y));
    // Title: a quoted title, else the sentence after «(Year).».
    if let Some(a) = l.find(['"', '“', '«']) {
        let rest = &l[a + l[a..].chars().next().unwrap().len_utf8()..];
        if let Some(b) = rest.find(['"', '”', '»']) {
            e.title = Some(rest[..b].trim().trim_end_matches(['.', ',']).to_string());
        }
    }
    if e.title.is_none() {
        if let Some(y) = e.year {
            let pat = format!("{y}).");
            let after = l.find(&pat).map(|i| &l[i + pat.len()..]).or_else(|| l.find(&format!("{y}.")).map(|i| &l[i + 5..]));
            if let Some(t) = after.map(str::trim) {
                // Up to the first sentence end; a question mark stays in the title.
                let end = t.find(". ").map(|i| i).into_iter().chain(t.find("? ").map(|i| i + 1)).min().unwrap_or(t.len());
                let t = t[..end].trim().trim_end_matches('.');
                if t.split_whitespace().count() >= 3 {
                    e.title = Some(t.to_string());
                }
            }
        }
    }
    e.author = l.split(|c: char| c == ',' || c == '(' || c == ' ').find(|w| w.len() > 1 && w.chars().next().is_some_and(char::is_uppercase)).map(str::to_string);
    e
}

/// Split a draft into body sentences with their citation numbers, and its numbered reference list
/// (`mova_whale::claims::split`, shared with the Whale epoch's citation check).
pub use mova_whale::claims::split;

/// What the claims ask for next: works to download (stage 2) and pages for Sea Spider (stage 4).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Requests {
    pub downloads: BTreeMap<String, RefEntry>,
    pub pages: Vec<Target>,
}

/// Claims of a registered draft, their citations resolved.
pub fn claims(c: &mut Corpus, lib: &Library, draft: &str) -> Result<(Vec<Claim>, Requests)> {
    let did = format!("sol:{draft}");
    let text = c.text(&did)?;
    let (body, refs) = split(&text);
    let mut req = Requests::default();
    let mut resolved: BTreeMap<usize, Citation> = BTreeMap::new();
    let mut seen_pages = BTreeSet::new();
    for (n, raw) in &refs {
        let e = parse_entry(raw);
        let (resolution, source) = if let Some(w) = lib.resolve(&e) {
            (if lib.text(w).is_some() { "library_text" } else { "library_no_text" }, Some(format!("lib:{}", w.id)))
        } else if let Some(k) = missing::key_of(&e) {
            req.downloads.insert(k.clone(), e.clone());
            ("download_requested", Some(k))
        } else if let Some(u) = e.url.clone().filter(|u| !missing::shadow(u)) {
            if seen_pages.insert(u.clone()) {
                req.pages.push(Target { url: u.clone(), why: "claim".into(), about: format!("{did} [{n}]") });
            }
            ("web_requested", Some(u))
        } else if let Some(k) = missing::title_lookup_key(&e) {
            // Only a title: looked up in the snapshot by title at the next download request.
            req.downloads.insert(k.clone(), e.clone());
            ("download_requested", Some(k))
        } else {
            ("unresolved", None)
        };
        resolved.insert(*n, Citation { n: *n, entry: e, resolution: resolution.into(), source });
    }
    let mut out = vec![];
    for (i, (s, nums)) in body.into_iter().enumerate() {
        let cits: Vec<Citation> = nums.iter().map(|n| resolved.get(n).cloned().unwrap_or(Citation { n: *n, entry: RefEntry::default(), resolution: "unresolved".into(), source: None })).collect();
        let held: Vec<&Citation> = cits.iter().filter(|c| c.resolution == "library_text").collect();
        let status = if !held.is_empty() {
            "sourced"
        } else if cits.iter().any(|c| matches!(c.resolution.as_str(), "download_requested" | "web_requested")) {
            "pending"
        } else {
            "from_sol"
        };
        let mut support = None;
        let cw: BTreeSet<String> = words(&s).into_iter().collect();
        for h in &held {
            let src = h.source.clone().unwrap();
            if c.doc(&src).is_none() {
                continue;
            }
            let t = c.text(&src)?;
            let best = world::quant::sentences(&t.replace('\n', " ")).into_iter().map(|x| {
                let xw: BTreeSet<String> = words(&x).into_iter().collect();
                let inter = cw.intersection(&xw).count() as f64;
                let uni = cw.union(&xw).count().max(1) as f64;
                ((inter / uni * 1000.0).round() / 1000.0, x)
            }).max_by(|a, b| a.0.total_cmp(&b.0).then_with(|| b.1.cmp(&a.1)));
            if let Some((score, x)) = best {
                if support.as_ref().is_none_or(|v: &Value| v["overlap"].as_f64().unwrap_or(0.0) < score) {
                    support = Some(json!({"doc": src, "sentence": x, "overlap": score}));
                }
            }
        }
        let provenance = match status {
            "from_sol" => json!({"source": "sol", "draft": did, "why": if cits.is_empty() { "no citation" } else { "cited sources closed or not found" }}),
            _ => json!({"source": "citations", "draft": did}),
        };
        out.push(Claim { id: format!("{draft}-c{:03}", i + 1), text: s, citations: cits, status: status.into(), provenance, support });
    }
    Ok((out, req))
}

/// Write the claims and requests of a draft under `<corpus>/sol/`.
pub fn save(c: &Corpus, draft: &str, cl: &[Claim], req: &Requests) -> Result<Value> {
    let d = c.dir.join("sol");
    mova_whale::util::private_dir(&d).map_err(|e| anyhow!("{e}"))?;
    crate::corpus::write_private(&d.join(format!("{draft}-claims.jsonl")), cl.iter().map(|x| serde_json::to_string(x).unwrap() + "\n").collect::<String>().as_bytes())?;
    crate::corpus::write_private(&d.join(format!("{draft}-requests.json")), &serde_json::to_vec_pretty(&json!({"schema": SCHEMA, "downloads": req.downloads, "pages": req.pages}))?)?;
    let mut by: BTreeMap<&str, usize> = BTreeMap::new();
    for x in cl {
        *by.entry(x.status.as_str()).or_default() += 1;
    }
    Ok(json!({"claims": cl.len(), "by_status": by, "downloads": req.downloads.len(), "pages": req.pages.len(), "dir": d}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Limits;
    use crate::corpus::tests::{built, model};
    use crate::graph;
    use crate::library::tests::fixture;
    use mova_whale::terminal::FakeTerminal;

    const DRAFT: &str = "# Error types in GEC\n\n## Background\n\nGrammatical error correction systems are evaluated with edit-based metrics [1]. \
The CoNLL-2014 shared task defined the standard test set for the field [2]. Learner essays were graded automatically in early work [3]. \
Many teachers prefer feedback by error type [4]. Error types help learners more than corrections alone.\n\n## References\n\n\
[1] Ray A (2015). Evaluation of grammatical error correction. ACL Anthology P15-1068.\n\
[2] Ng, H. T. et al. (2014). The CoNLL-2014 Shared Task on Grammatical Error Correction. doi:10.3115/v1/W14-1701\n\
[3] Yannakoudakis, H. (2011). A New Dataset and Method for Automatically Grading ESOL Texts. arXiv:1101.0001\n\
[4] Some Blog (2020). Why error types matter. https://blog.example.org/error-types\n";

    #[test]
    fn entries_and_split() {
        let e = parse_entry("[2] Ng, H. T. et al. (2014). The CoNLL-2014 Shared Task on Grammatical Error Correction. doi:10.3115/v1/W14-1701");
        assert_eq!(e.doi.as_deref(), Some("10.3115/v1/w14-1701"));
        assert_eq!(e.year, Some(2014));
        assert_eq!(e.title.as_deref(), Some("The CoNLL-2014 Shared Task on Grammatical Error Correction"));
        assert_eq!(parse_entry("Yo (2011). A title here. arXiv:1101.0001").arxiv.as_deref(), Some("1101.0001"));
        assert_eq!(parse_entry("Ray (2015). Evaluation. https://aclanthology.org/P15-1068/").acl.as_deref(), Some("P15-1068"));
        let (b, r) = split(DRAFT);
        assert_eq!(r.len(), 4);
        assert_eq!(b[0], ("Grammatical error correction systems are evaluated with edit-based metrics.".to_string(), vec![1]));
        assert_eq!(b.last().unwrap().1, Vec::<usize>::new());
        assert_eq!(parse_entry("[5] Ferris, D. (2006). Does error feedback help student writers? Cambridge University Press.").title.as_deref(), Some("Does error feedback help student writers?"));
        let (b2, _) = split("A claim cites a range of works here [1-3, 5].");
        assert_eq!(b2[0].1, vec![1, 2, 3, 5]);
    }

    #[test]
    fn a_fake_sol_draft_becomes_claims_with_sources() {
        if !model() {
            return;
        }
        let (d, mut c) = built("sol");
        let lib = Library::load(&fixture(&d.join("lib2"))).unwrap();
        let g = graph::build(&lib, "pa-2017-error-types", &Limits::default()).unwrap();
        let mut t = FakeTerminal { animal: "owl-1".into(), answers: [(format!("article:{}", g.paper.id), DRAFT.to_string())].into(), limit_after: None, received: vec![] };
        draft(&mut c, &mut t, &g, "d1", 3000).unwrap();
        assert!(t.received[0].text.contains("Cite as many original sources"));
        let s = c.doc("sol:d1").unwrap().clone();
        assert!(s.internal_only && !s.public_admissible && s.quality.as_deref() == Some("low"));
        assert!(crate::corpus::assert_public(&s).is_err());
        let (cl, req) = claims(&mut c, &lib, "d1").unwrap();
        let st: Vec<&str> = cl.iter().map(|x| x.status.as_str()).collect();
        assert_eq!(st, vec!["sourced", "pending", "pending", "pending", "from_sol"]);
        assert_eq!(cl[0].citations[0].source.as_deref(), Some("lib:ra-2015-gec-evaluation"));
        assert!(cl[0].support.as_ref().unwrap()["overlap"].as_f64().unwrap() > 0.0);
        assert_eq!(cl[4].provenance["source"], "sol");
        assert_eq!(req.downloads.keys().cloned().collect::<Vec<_>>(), vec!["arxiv:1101.0001", "doi:10.3115/v1/w14-1701"]);
        assert_eq!(req.pages[0].url, "https://blog.example.org/error-types");
        save(&c, "d1", &cl, &req).unwrap();
        // A terminal that hits its limit gives no draft and registers nothing.
        let mut lim = FakeTerminal { animal: "owl-1".into(), answers: BTreeMap::new(), limit_after: Some(0), received: vec![] };
        assert!(draft(&mut c, &mut lim, &g, "d2", 3000).is_err());
        assert!(c.doc("sol:d2").is_none());
        let _ = std::fs::remove_dir_all(d);
    }
}
