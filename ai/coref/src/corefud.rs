//! CorefUD format: mentions in MISC `Entity=`. The bracket `(e-…` opens a mention of entity `e` at a word,
//! `e)` closes it, `(e-…)` is a single-word mention. The fields after the number are defined by `# global.Entity`
//! (in GUM: `GRP-etype-infstat-salience-centering-minspan-link-identity`). A mention stays within one sentence;
//! discontinuous mentions (`[1/2]`) do not occur in GUM 2.18 — they raise an error (fail-fast).

use anyhow::{Result, bail};
use en::conllu::{Col, Doc as UdDoc};

use crate::doc::{Document, Sent};

/// Mention span: document sentence and words [start, end] (0-based, inclusive).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Span {
    pub sent: usize,
    pub start: usize,
    pub end: usize,
}

/// A mention from the file: span, entity number and the raw fields after the number.
#[derive(Clone, Debug)]
pub struct FileMention {
    pub span: Span,
    pub eid: String,
    /// The etype field (second in `global.Entity`), if present.
    pub etype: String,
}

/// Document mentions from `Entity=`, in opening order.
pub fn read_mentions(doc: &Document) -> Result<Vec<FileMention>> {
    let mut out: Vec<FileMention> = Vec::new();
    // open ones: (eid, index into out)
    let mut open: Vec<(String, usize)> = Vec::new();
    for (si, s) in doc.sents.iter().enumerate() {
        for (w, e) in s.entity.iter().enumerate() {
            let Some(e) = e else { continue };
            parse_value(e, |ev| {
                match ev {
                    Ev::Open(eid, rest) => {
                        let etype = rest.split('-').next().unwrap_or("").to_string();
                        out.push(FileMention { span: Span { sent: si, start: w, end: usize::MAX }, eid: eid.to_string(), etype });
                        open.push((eid.to_string(), out.len() - 1));
                    }
                    Ev::Close(eid) => {
                        let Some(pos) = open.iter().rposition(|(x, _)| x == eid) else {
                            bail!("{} {}: word {}: closed \"{eid}\" but no open mention", doc.id, s.id, w + 1);
                        };
                        let (_, mi) = open.remove(pos);
                        if out[mi].span.sent != si {
                            bail!("{} {}: mention \"{eid}\" crosses a sentence boundary — not supported", doc.id, s.id);
                        }
                        out[mi].span.end = w;
                    }
                }
                Ok(())
            })
            .map_err(|err| anyhow::anyhow!("{} {} word {}: {err}", doc.id, s.id, w + 1))?;
        }
    }
    if let Some((eid, _)) = open.first() {
        bail!("{}: mention \"{eid}\" not closed by the end of the document", doc.id);
    }
    Ok(out)
}

enum Ev<'a> {
    Open(&'a str, &'a str),
    Close(&'a str),
}

fn parse_value<'a>(v: &'a str, mut f: impl FnMut(Ev<'a>) -> Result<()>) -> Result<()> {
    let b = v.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'(' {
            let mut j = i + 1;
            while j < b.len() && b[j] != b'(' && b[j] != b')' {
                j += 1;
            }
            let content = &v[i + 1..j];
            let (eid, rest) = content.split_once('-').unwrap_or((content, ""));
            if eid.is_empty() || eid.contains('[') {
                bail!("mention \"{content}\": empty or discontinuous number — not supported");
            }
            f(Ev::Open(eid, rest))?;
            if j < b.len() && b[j] == b')' {
                f(Ev::Close(eid))?;
                i = j + 1;
            } else {
                i = j;
            }
        } else {
            let mut j = i;
            while j < b.len() && b[j] != b')' {
                j += 1;
            }
            if j == b.len() {
                bail!("\"{v}\": closing without \")\"");
            }
            let eid = &v[i..j];
            if eid.contains('[') {
                bail!("discontinuous mention \"{eid}\" — not supported");
            }
            f(Ev::Close(eid))?;
            i = j + 1;
        }
    }
    Ok(())
}

/// A mention for writing: span, head (sentence word, 0-based), entity number, type and note (sieve id).
#[derive(Clone, Debug)]
pub struct OutMention {
    pub span: Span,
    pub head: usize,
    pub eid: usize,
    pub etype: String,
    pub note: String,
}

/// The `Entity=` value for word `w`: openings first (longer first; a single-word mention last and closed
/// immediately), then closings of mentions that started earlier (inner first).
fn entity_value(ms: &[&OutMention], w: usize) -> String {
    let mut opens: Vec<&&OutMention> = ms.iter().filter(|m| m.span.start == w).collect();
    opens.sort_by_key(|m| (std::cmp::Reverse(m.span.end), m.eid));
    let mut closes: Vec<&&OutMention> = ms.iter().filter(|m| m.span.end == w && m.span.start < w).collect();
    closes.sort_by_key(|m| (std::cmp::Reverse(m.span.start), m.eid));
    let mut s = String::new();
    for m in opens {
        let head1 = m.head - m.span.start + 1;
        s.push_str(&format!("(e{}-{}-{}-{}", m.eid, m.etype, head1, m.note));
        if m.span.end == w {
            s.push(')');
        }
    }
    for m in closes {
        s.push_str(&format!("e{})", m.eid));
    }
    s
}

/// Write mentions into MISC: remove old `Entity`, `Bridge`, `SplitAnte`, add the new ones; the
/// `# global.Entity` header of the document's first sentence is ours (`eid-etype-head-other`).
pub fn write(ud: &mut UdDoc, docs: &[Document], per_doc: &[Vec<OutMention>]) -> Result<()> {
    for (d, ms) in docs.iter().zip(per_doc) {
        for (si, s) in d.sents.iter().enumerate() {
            let fs = &mut ud.sents[s.file_idx];
            if si == 0 {
                fs.comments.retain(|c| !c.starts_with("# global.Entity"));
                let at = fs.comments.iter().position(|c| c.starts_with("# newdoc")).map_or(0, |p| p + 1);
                fs.comments.insert(at, "# global.Entity = eid-etype-head-other".to_string());
            }
            let here: Vec<&OutMention> = ms.iter().filter(|m| m.span.sent == si).collect();
            for w in 0..s.len() {
                let val = entity_value(&here, w);
                let row = fs.word_mut(w);
                let old = row.get(Col::Misc).to_string();
                let mut parts: Vec<String> = if old == "_" { Vec::new() } else { old.split('|').filter(|p| !(p.starts_with("Entity=") || p.starts_with("Bridge=") || p.starts_with("SplitAnte="))).map(str::to_string).collect() };
                if !val.is_empty() {
                    parts.push(format!("Entity={val}"));
                }
                parts.sort_by(|a, b| a.split('=').next().unwrap_or("").to_lowercase().cmp(&b.split('=').next().unwrap_or("").to_lowercase()));
                row.set(Col::Misc, if parts.is_empty() { "_".to_string() } else { parts.join("|") });
            }
        }
    }
    Ok(())
}

/// Remove `Entity`, `Bridge`, `SplitAnte` from the MISC of the whole file (to check that the resolver does not see gold).
pub fn strip(ud: &mut UdDoc) {
    for fs in &mut ud.sents {
        for r in &mut fs.rows {
            let old = r.get(Col::Misc).to_string();
            let parts: Vec<&str> = old.split('|').filter(|p| *p != "_" && !(p.starts_with("Entity=") || p.starts_with("Bridge=") || p.starts_with("SplitAnte="))).collect();
            r.set(Col::Misc, if parts.is_empty() { "_".to_string() } else { parts.join("|") });
        }
    }
}

/// Head of a span by the tree (like udapi corefud.MoveHead): words whose head is outside the span;
/// among several — the highest in the tree, then non-punctuation, then the first.
pub fn span_head(s: &Sent, sp: Span) -> usize {
    let (a, b) = (sp.start, sp.end);
    let mut c: Vec<usize> = (a..=b).filter(|&i| s.parent(i).is_none_or(|p| p < a || p > b)).collect();
    if c.is_empty() {
        return a;
    }
    if c.len() > 1 {
        let md = c.iter().map(|&i| s.depth[i]).min().unwrap();
        c.retain(|&i| s.depth[i] == md);
    }
    if c.len() > 1 {
        let np: Vec<usize> = c.iter().copied().filter(|&i| s.upos(i) != Some(en::gram::UPos::PUNCT)).collect();
        if !np.is_empty() {
            c = np;
        }
    }
    c[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn events(v: &str) -> Vec<String> {
        let mut out = Vec::new();
        parse_value(v, |e| {
            out.push(match e {
                Ev::Open(a, r) => format!("({a}|{r}"),
                Ev::Close(a) => format!("{a})"),
            });
            Ok(())
        })
        .unwrap();
        out
    }

    #[test]
    fn parse_gum_values() {
        assert_eq!(events("(7-person-new-ssnns-cf2-1-coref)6)5)"), ["(7|person-new-ssnns-cf2-1-coref", "7)", "6)", "5)"]);
        assert_eq!(events("(1-abstract-new-snnnn-cf1-2-sgl"), ["(1|abstract-new-snnnn-cf1-2-sgl"]);
        assert_eq!(events("4)1)"), ["4)", "1)"]);
        assert_eq!(events("(4-place-new-nnnnn-cf2-2-coref-Coron%2C_Palawan"), ["(4|place-new-nnnnn-cf2-2-coref-Coron%2C_Palawan"]);
        // negative control: a discontinuous mention and a closing without a bracket are errors
        assert!(parse_value("(3[1/2]-person", |_| Ok(())).is_err());
        assert!(parse_value("12", |_| Ok(())).is_err());
    }
}
