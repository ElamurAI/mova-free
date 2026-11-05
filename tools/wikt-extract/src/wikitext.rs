//! Wikitext of one English Wiktionary definition, example or list line → plain text, with what its templates say
//! kept apart: labels, form-of links, nyms (synonyms, antonyms …), examples. Template calls are parsed with their
//! nesting (`{{l|en|{{…}}}}`, `[[a|b]]` inside parameters); each known template has its own handler; a template
//! with no handler is never silently dropped: its name is counted as a gap (`Out::gaps`) and its text left out.
//!
//! The handlers reproduce what the common English templates print (`{{l|en|word|alt}}` → alt or word, `{{w|A|b}}`
//! → b, `{{taxlink|Felis catus|species}}` → Felis catus, `{{gloss|x}}` → (x), `{{plural of|en|cat}}` → plural of
//! cat), not the Lua modules behind them: categories, links and formatting are not reproduced.

use std::collections::BTreeMap;

/// A parsed template call: its name (trimmed, as written) and parameters, positional (1-based in Wiktionary's
/// terms, 0-based here) and named.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Call {
    pub name: String,
    pub pos: Vec<String>,
    pub named: BTreeMap<String, String>,
}

impl Call {
    /// Positional parameter `i` (0-based), trimmed; empty when absent.
    pub fn p(&self, i: usize) -> &str {
        self.pos.get(i).map(|s| s.trim()).unwrap_or("")
    }
    pub fn n(&self, k: &str) -> &str {
        self.named.get(k).map(|s| s.trim()).unwrap_or("")
    }
}

/// What rendering a piece of wikitext found besides its text.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Out {
    pub labels: Vec<String>,
    /// (kind, lemma): «plural of» cat.
    pub form_of: Option<(String, String)>,
    /// (relation, term): ("syn", "buddy").
    pub nyms: Vec<(String, String)>,
    pub examples: Vec<String>,
    /// Templates with no handler, by name.
    pub gaps: BTreeMap<String, u32>,
    /// Templates known and deliberately printed as nothing (categories, maintenance, boxes), by name.
    pub dropped: BTreeMap<String, u32>,
}

/// Index of the `}}` closing the template that opens at `s[i..]` (`{{`), with nesting; None when unclosed.
fn close_braces(b: &[u8], i: usize) -> Option<usize> {
    let mut d = 0usize;
    let mut j = i;
    while j + 1 < b.len() {
        if b[j] == b'{' && b[j + 1] == b'{' {
            d += 1;
            j += 2;
        } else if b[j] == b'}' && b[j + 1] == b'}' {
            d -= 1;
            if d == 0 {
                return Some(j);
            }
            j += 2;
        } else {
            j += 1;
        }
    }
    None
}

/// Split a template body at top-level `|` (not inside nested `{{…}}` or `[[…]]`).
pub fn split_params(body: &str) -> Vec<&str> {
    let b = body.as_bytes();
    let (mut out, mut start, mut db, mut dl, mut j) = (Vec::new(), 0usize, 0i32, 0i32, 0usize);
    while j < b.len() {
        match b[j] {
            b'{' if b.get(j + 1) == Some(&b'{') => {
                db += 1;
                j += 2;
                continue;
            }
            b'}' if b.get(j + 1) == Some(&b'}') && db > 0 => {
                db -= 1;
                j += 2;
                continue;
            }
            b'[' if b.get(j + 1) == Some(&b'[') => {
                dl += 1;
                j += 2;
                continue;
            }
            b']' if b.get(j + 1) == Some(&b']') && dl > 0 => {
                dl -= 1;
                j += 2;
                continue;
            }
            b'|' if db == 0 && dl == 0 => {
                out.push(&body[start..j]);
                start = j + 1;
            }
            _ => {}
        }
        j += 1;
    }
    out.push(&body[start..]);
    out
}

/// Parse a template body (between `{{` and `}}`) into a call. A parameter `k=v` is named when `k` has no `[[`/`{{`.
pub fn parse_call(body: &str) -> Call {
    let parts = split_params(body);
    let mut c = Call { name: parts[0].trim().to_string(), ..Call::default() };
    for p in &parts[1..] {
        match p.split_once('=') {
            Some((k, v)) if !k.contains("[[") && !k.contains("{{") && !k.contains('<') => {
                c.named.insert(k.trim().to_string(), v.to_string());
            }
            _ => c.pos.push(p.to_string()),
        }
    }
    c
}

/// Every top-level template call in `s`, in order.
pub fn calls(s: &str) -> Vec<Call> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < b.len() {
        if b[i] == b'{' && b[i + 1] == b'{' {
            if let Some(j) = close_braces(b, i) {
                out.push(parse_call(&s[i + 2..j]));
                i = j + 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Templates printed as nothing on purpose: categories, sense ids, maintenance requests, sidebars, references.
const DROP: &[&str] = &[
    "c", "C", "top", "topics", "cln", "catlangname", "categorize", "cat", "senseid", "sid", "rfdef", "rfex", "rfquote", "rfquotek", "rfc-sense",
    "rfv-sense", "rfd-sense", "rfclarify", "rfm-sense", "rft-sense", "rfi", "rfe", "rfc", "rfv", "rfd", "rfm", "rfgender", "defdate", "defdt", "attn", "wikipedia", "wp", "pedia",
    "slim-wikipedia", "wikispecies", "specieslite", "comcatlite", "commonscat", "commonslite", "also", "minitoc", "picdic", "anchor", "tea room sense", "tea room", "refn", "wikidata lexeme", "swp", "tcl", "senseno", "list:places in the Philippines/en",
    "ref", "color panel", "colour panel", "R:Webster 1913", "nbsp", "audio", "IPA", "enPR", "rhymes", "hyphenation", "hyph", "cite-book", "cite-web", "wikiquote", "examples",
    "attention", "maintenance line", "checksense", "rfscript", "box", "projectlink", "wikibooks", "wikisource", "wikiversity", "jump", "redirect", "was wotd", "wotd", "lena",
    "t-needed", "trans-top", "trans-bottom", "trans-mid", "trans-see", "checktrans-top", "checktrans-bottom", "rfdate", "rfdatek", "rfref", "unsupported", "ISBN", "rfex-rel",
    "dercat", "en-plural noun", "rel-top", "rel-bottom", "rel-mid", "der-top", "der-bottom", "der-mid", "small caps", "pos_n", "qualifier-lite", "sup-ref", "inline", "ws",
];

const LABELS: &[&str] = &["lb", "lbl", "label", "term-label", "tlb", "lbs"];
const QUALIFIERS: &[&str] = &["q", "qual", "qualifier", "i", "qf", "q-lite", "qualifier-lite", "sense", "s"];
const GLOSSES: &[&str] = &["gloss", "gl"];
const LINKS: &[&str] = &["l", "l-self", "ll", "m", "m-self", "link", "mention", "m+", "l+", "langname-lite", "lang", "cog", "noncog", "ncog", "der", "inh", "bor", "lbor", "slbor", "calque", "cal", "psm", "bor+", "inh+", "der+", "uder", "obor", "lg"];
const TEXT1: &[&str] = &["U", "nowrap", "smallcaps", "sc", "IPAchar", "IPAfont", "sub", "sup", "small", "big", "nobr", "upright", "vern", "w2", "unicode", "Unicode", "angle bracket", "angbr", "mono", "monospace", "keyword", "code", "font", "Latn", "Grek", "Cyrl", "Hebr", "lang-lite", "small-caps"];
const NONGLOSS: &[&str] = &["n-g", "ngd", "non-gloss definition", "non-gloss", "ng", "n-g-lite", "ngl", "def", "defn"];
const EXAMPLES: &[&str] = &["ux", "uxa", "suffixusex", "prefixusex", "affixusex", "compound-usex", "uxi", "usex", "eg", "afex", "coi", "collocation", "co"];
const TAXON: &[&str] = &["taxlink", "taxfmt", "taxlinknew", "taxlinkwiki", "taxlink2", "spelink", "mtax"];
const NYMS: &[(&str, &str)] = &[
    ("syn", "syn"), ("synonyms", "syn"), ("syn-lite", "syn"), ("ant", "ant"), ("antonyms", "ant"), ("hyper", "hyper"), ("hypernyms", "hyper"),
    ("hypo", "hypo"), ("hyponyms", "hypo"), ("coord", "coord"), ("cot", "coord"), ("mero", "mero"), ("holo", "holo"), ("troponyms", "tropo"),
    ("perfect synonyms", "syn"), ("synonym", "syn"), ("nearsyn", "syn"), ("nsyn", "syn"), ("alti", "alt"), ("coordinate terms", "coord"), ("hol", "holo"), ("mer", "mero"), ("meronyms", "mero"), ("holonyms", "holo"), ("comeronyms", "comero"), ("antonym", "ant"), ("hypernym", "hyper"), ("hyponym", "hypo"), ("near-syn", "syn"), ("near-synonyms", "syn"), ("imperfect synonyms", "syn"),
];

/// A form-of template's display name, its lemma parameter index (0-based; language first for most), or None.
/// The generic rule: a name ending in « of» (`plural of`, `alternative form of`, `en-past of`) is a form-of template;
/// `en-…` ones take no language parameter.
pub fn form_of(c: &Call) -> Option<(String, String)> {
    let name = c.name.as_str();
    let (kind, idx) = match name {
        "alt form" | "alt form of" => ("alternative form of", 1),
        "alt sp" | "alt spelling" => ("alternative spelling of", 1),
        "abbr of" => ("abbreviation of", 1),
        "syn of" => ("synonym of", 1),
        "alt case" | "altcase" | "alt case form" | "alt case form of" => ("alternative letter-case form of", 1),
        "altform" => ("alternative form of", 1),
        "altsp" | "alt spell" | "altspell" => ("alternative spelling of", 1),
        "missp" => ("misspelling of", 1),
        "obs form" => ("obsolete form of", 1),
        "obs sp" => ("obsolete spelling of", 1),
        "cens sp" => ("censored spelling of", 1),
        "stand sp" => ("standard spelling of", 1),
        "comb form" => ("combining form of", 1),
        "dim of" => ("diminutive of", 1),
        "init of" => ("initialism of", 1),
        "ellip of" => ("ellipsis of", 1),
        "contr of" => ("contraction of", 1),
        "obs form of" | "obs sp of" => ("obsolete form of", 1),
        "pronunciation spelling of" | "pron sp of" => ("pronunciation spelling of", 1),
        "infl of" | "inflection of" => ("inflection of", 1),
        "short for" => ("short for", 1),
        "clip of" | "clipping of" => ("clipping of", 1),
        "only used in" | "only in" => ("only used in", 1),
        "standard spelling of" | "standard form of" => (name, 1),
        n if n.starts_with("en-") && n.ends_with(" of") => (n.trim_start_matches("en-"), 0),
        n if n.ends_with(" of") => (n, 1),
        _ => return None,
    };
    // an empty term slot shows the display text that follows it ({{initialism of|en||Third Generation Partnership Project}})
    let lemma = if c.p(idx).is_empty() { c.p(idx + 1) } else { c.p(idx) }.trim_start_matches(':').to_string();
    (!lemma.is_empty()).then(|| (kind.to_string(), lemma))
}

/// Strip inline modifiers (`word<q:rare>`, `word<t:gloss>`) and a `#Section` from a term.
pub fn term(s: &str) -> String {
    let s = s.split('<').next().unwrap_or("");
    let s = s.split('#').next().unwrap_or("").trim();
    // a Wikipedia or interwiki prefix on a term (`w:acute porphyria`) is not part of the word
    let s = s.strip_prefix("w:").or_else(|| s.strip_prefix(":w:")).unwrap_or(s);
    s.trim().to_string()
}

/// The terms of a nym template or column list (`{{syn|en|a|b<q:rare>|Thesaurus:x}}`, `{{col|en|a|b}}`): lang first,
/// `Thesaurus:` pages, `;`/`,` separators and empty slots left out.
pub fn nym_terms(c: &Call) -> Vec<String> {
    c.pos.iter().skip(1).flat_map(|p| p.split(',')).map(|p| plain(&term(p))).filter(|t| !t.is_empty() && !t.starts_with("Thesaurus:") && !matches!(t.as_str(), ";" | "," | "_" | "-")).collect()
}

/// Wikitext to plain text, its templates handled (see the module comment); `out` collects labels and gaps.
pub fn render(s: &str, out: &mut Out) -> String {
    let mut r = String::new();
    let b = s.as_bytes();
    let mut i = 0;
    let mut last = 0;
    while i < b.len() {
        if b[i] == b'{' && b.get(i + 1) == Some(&b'{') {
            if let Some(j) = close_braces(b, i) {
                r.push_str(&s[last..i]);
                let c = parse_call(&s[i + 2..j]);
                r.push_str(&template(&c, out));
                i = j + 2;
                last = i;
                continue;
            }
        }
        // an external link [https://… text] → text
        if b[i] == b'[' && (s[i + 1..].starts_with("http://") || s[i + 1..].starts_with("https://")) {
            if let Some(e) = s[i + 1..].find(']') {
                r.push_str(&s[last..i]);
                let body = &s[i + 1..i + 1 + e];
                r.push_str(body.split_once(' ').map(|x| x.1).unwrap_or(""));
                i = i + 1 + e + 1;
                last = i;
                continue;
            }
        }
        if b[i] == b'[' && b.get(i + 1) == Some(&b'[') {
            if let Some(e) = s[i + 2..].find("]]") {
                // a link may hold a template ([[w:{{…}}]]) rarely; the text is what follows the last top-level |
                r.push_str(&s[last..i]);
                r.push_str(&link(&s[i + 2..i + 2 + e], out));
                i = i + 2 + e + 2;
                last = i;
                continue;
            }
        }
        i += 1;
    }
    r.push_str(&s[last..]);
    tidy(&r)
}

/// The text of a wiki link body: `a|b` → b, `a#English` → a, `w:Article` → Article; files and categories → nothing.
fn link(body: &str, out: &mut Out) -> String {
    let parts = split_params(body);
    let target = parts[0].trim();
    let low = target.to_lowercase();
    if low.starts_with("category:") || low.starts_with("file:") || low.starts_with("image:") || low.starts_with(":category:") {
        return String::new();
    }
    let text = if parts.len() > 1 { parts[parts.len() - 1].to_string() } else {
        let t = target.trim_start_matches(':');
        let t = t.strip_prefix("w:").or_else(|| t.strip_prefix("wikipedia:")).unwrap_or(t);
        t.split('#').next().unwrap_or("").to_string()
    };
    render(&text, out)
}

/// Bold/italic quotes, HTML comments and tags, references and the common entities out; spaces collapsed.
pub fn tidy(s: &str) -> String {
    let mut t = s.to_string();
    for (a, z) in [("<!--", "-->"), ("<ref", "</ref>")] {
        while let Some(p) = t.find(a) {
            let rest = &t[p..];
            // a self-closing <ref …/> ends at its own />
            let end = if a == "<ref" && rest.find("/>").is_some_and(|q| rest.find('>').is_some_and(|g| g == q + 1)) {
                rest.find("/>").map(|q| q + 2)
            } else {
                rest.find(z).map(|q| q + z.len())
            };
            t.replace_range(p..p + end.unwrap_or(rest.len()), "");
        }
    }
    let mut o = String::with_capacity(t.len());
    let mut in_tag = false;
    let cs: Vec<char> = t.chars().collect();
    for (k, &ch) in cs.iter().enumerate() {
        match ch {
            '<' if cs.get(k + 1).is_some_and(|n| n.is_ascii_alphabetic() || *n == '/') => {
                in_tag = true;
                o.push(' ');
            }
            '>' if in_tag => in_tag = false,
            _ if in_tag => {}
            _ => o.push(ch),
        }
    }
    let o = o.replace("'''", "").replace("''", "").replace("&nbsp;", " ").replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&ndash;", "–").replace("&mdash;", "—").replace("&hellip;", "…");
    let o: String = o.chars().map(|c| if c == '\t' || c == '\n' || c == '\r' { ' ' } else { c }).collect();
    let mut w = o.split_whitespace().collect::<Vec<_>>().join(" ");
    for (a, z) in [(" ,", ","), (" .", "."), ("( ", "("), (" )", ")"), (" ;", ";"), (" :", ":")] {
        w = w.replace(a, z);
    }
    w.replace("()", "").split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Plain text of a parameter with no side effects kept (labels, gaps of nested templates are discarded).
pub fn plain(s: &str) -> String {
    render(s, &mut Out::default())
}

fn cap(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// The labels of a label template (lang first): `_`, `and`, `or` joiners left out; links rendered.
pub fn label_values(c: &Call) -> Vec<String> {
    c.pos.iter().skip(1).map(|p| plain(p)).filter(|l| !l.is_empty() && !matches!(l.as_str(), "_" | "and" | "or" | "&")).collect()
}

/// What one template prints, with its side effects on `out`.
pub fn template(c: &Call, out: &mut Out) -> String {
    let n = c.name.as_str();
    let rp = |s: &str, out: &mut Out| render(s, out);
    if LABELS.contains(&n) {
        out.labels.extend(label_values(c));
        return String::new();
    }
    if DROP.contains(&n) || n.starts_with("R:") || n.starts_with("RQ:") || n.starts_with("quote-") || n.starts_with("cite-") || n.starts_with("rf") {
        *out.dropped.entry(n.to_string()).or_default() += 1;
        return String::new();
    }
    if QUALIFIERS.contains(&n) {
        let v: Vec<String> = c.pos.iter().map(|p| rp(p, out)).filter(|x| !x.is_empty()).collect();
        return if v.is_empty() { String::new() } else { format!("({})", v.join(", ")) };
    }
    if GLOSSES.contains(&n) {
        return format!("({})", rp(c.p(0), out));
    }
    if LINKS.contains(&n) {
        // lang, term, alt, gloss (cog/der/inh/bor: target lang, source lang, term, alt)
        let (t, a) = if matches!(n, "l" | "l-self" | "ll" | "m" | "m-self" | "link" | "mention" | "m+" | "l+") { (1, 2) } else if matches!(n, "cog" | "noncog" | "ncog" | "langname-lite" | "lang" | "lg") { (1, 2) } else { (2, 3) };
        let alt = if !c.n("alt").is_empty() { c.n("alt").to_string() } else { c.p(a).to_string() };
        let shown = if !alt.is_empty() { alt } else { c.p(t).to_string() };
        return rp(&term(&shown), out);
    }
    if TEXT1.contains(&n) {
        return rp(c.p(0), out);
    }
    if NONGLOSS.contains(&n) {
        return rp(c.p(0), out);
    }
    if TAXON.contains(&n) {
        let alt = c.n("alt");
        return rp(if alt.is_empty() { c.p(0) } else { alt }, out);
    }
    if EXAMPLES.contains(&n) {
        let ex = rp(c.p(1), out);
        if !ex.is_empty() {
            out.examples.push(ex.clone());
        }
        return if n == "uxi" { format!("({ex})") } else { String::new() };
    }
    if let Some((_, rel)) = NYMS.iter().find(|x| x.0 == n) {
        for t in nym_terms(c) {
            out.nyms.push((rel.to_string(), t));
        }
        return String::new();
    }
    if let Some((kind, lemma)) = form_of(c) {
        let shown = term(&lemma);
        let text = format!("{} {}", kind, plain(&shown));
        if out.form_of.is_none() {
            out.form_of = Some((kind.clone(), plain(&shown)));
        }
        let gloss = if !c.n("t").is_empty() { format!(" (“{}”)", plain(c.n("t"))) } else { String::new() };
        return cap(&text) + &gloss;
    }
    match n {
        "w" | "W" | "wikipedia link" | "wlink" | "pedlink" | "w+" => {
            let shown = if !c.p(1).is_empty() { c.p(1) } else { c.p(0) };
            rp(shown, out).split('#').next().unwrap_or("").to_string()
        }
        "..." | "…" | "nb..." | "ellipsis" => "[…]".into(),
        "," => ",".into(),
        "!" => "|".into(),
        "=" => "=".into(),
        "'" => "'".into(),
        "1" | "cap" | "Cap" | "ucfirst" => cap(&rp(c.p(0), out)),
        "lc" | "lcfirst" => rp(c.p(0), out).to_lowercase(),
        "surname" => {
            let g = c.p(1);
            if g.is_empty() || g.contains('=') { "A surname".into() } else { format!("A {} surname", plain(g)) }
        }
        "given name" => {
            let g = if c.p(1).is_empty() { c.n("gender").to_string() } else { c.p(1).to_string() };
            let g = plain(&g).replace("unisex", "unisex");
            if g.is_empty() { "A given name".into() } else { format!("A {g} given name") }
        }
        "place" => place(c),
        "taxon" => {
            // taxon|rank|higher rank|higher name|description
            let mut t = format!("A taxonomic {}", plain(c.p(0)));
            if !c.p(1).is_empty() && !c.p(2).is_empty() {
                t.push_str(&format!(" within the {} {}", plain(c.p(1)), plain(c.p(2))));
            }
            if !c.p(3).is_empty() {
                t.push_str(&format!(" – {}", plain(c.p(3))));
            }
            t
        }
        "&lit" => {
            let ts: Vec<String> = c.pos.iter().skip(1).map(|p| plain(&term(p))).filter(|t| !t.is_empty()).collect();
            format!("Used other than figuratively or idiomatically: see {}", ts.join(", "))
        }
        "demonym-noun" => format!("A native or inhabitant of {}.", plain(c.p(1))),
        "demonym-adj" => format!("Of, from, or relating to {}.", plain(c.p(1))),
        "abbreviation-old" | "abbreviation" => String::new(),
        "unit" | "SI-unit-abb" => String::new(),
        "en-noun" | "en-verb" | "en-adj" | "en-adv" | "head" | "en-proper noun" | "en-prop" => String::new(),
        "col" | "col1" | "col2" | "col3" | "col4" | "col5" | "col-auto" | "col-u" | "der2" | "der3" | "der4" | "rel2" | "rel3" | "rel4" | "der3-u" | "der4-u" => {
            // a list in a nyms section: the caller reads the terms (`nym_terms`); in running text, the terms joined
            nym_terms(c).join(", ")
        }
        "IPAlink" => rp(c.p(0), out),
        "glossary" => rp(if c.p(1).is_empty() { c.p(0) } else { c.p(1) }, out),
        "+obj" => "[+ object]".into(),
        "BC" => "BC".into(),
        "AD" | "CE" => n.into(),
        "smc" | "sm" => rp(c.p(0), out),
        "zh-l" | "ja-l" | "ko-l" => rp(c.p(0), out),
        "special" | "lit" => rp(c.p(0), out),
        "nuclide" => format!("{}{}", plain(c.p(0)), plain(c.p(2))),
        "frac" => c.pos.iter().map(|p| plain(p)).collect::<Vec<_>>().join("/"),
        "chem" => c.pos.iter().map(|p| plain(p)).collect::<String>(),
        "sense-lite" => format!("({})", rp(c.p(0), out)),
        _ => {
            *out.gaps.entry(n.to_string()).or_default() += 1;
            String::new()
        }
    }
}

/// `{{place|en|town|p/Kent|c/England}}` → «A town in Kent, England»; the new style with `<<…>>` placeholders and
/// free text (`{{place|en|A <<town>> in <<p/Kent>>}}`) is printed with its markers and prefixes removed.
fn place(c: &Call) -> String {
    let first = c.p(1);
    if first.contains("<<") || first.contains(' ') && !first.contains('/') {
        let mut t = first.replace("<<", "").replace(">>", "");
        for pre in ["c/", "p/", "s/", "r/", "co/", "cc/", "dist/", "city/", "town/", "state/", "prov/", "cont/", "country/", "isl/", "par/"] {
            t = t.replace(pre, "");
        }
        return cap(&tidy(&t));
    }
    let kind = first.split('/').next().unwrap_or("place").to_string();
    let kind = if kind.is_empty() { "place".to_string() } else { kind };
    // holonyms (`co/Cumbria`); a bare short word among them is the connector (`in`, `near`, `on`)
    let mut conn = "in".to_string();
    let mut holo: Vec<String> = Vec::new();
    for p in c.pos.iter().skip(2) {
        let v = p.trim();
        if !v.contains('/') && !v.is_empty() && v.len() <= 6 && v.chars().all(|ch| ch.is_ascii_lowercase()) {
            if holo.is_empty() {
                conn = v.to_string();
            }
            continue;
        }
        let v = v.split_once('/').map(|x| x.1).unwrap_or(v);
        let h = plain(&term(v.split(':').next_back().unwrap_or(v)));
        if !h.is_empty() && h != ";" {
            holo.push(h);
        }
    }
    let art = if kind.starts_with(['a', 'e', 'i', 'o', 'u']) { "An" } else { "A" };
    let mut t = format!("{art} {kind}");
    if !holo.is_empty() {
        t.push_str(&format!(" {conn} {}", holo.join(", ")));
    }
    let tail = c.n("t");
    if !tail.is_empty() {
        t.push_str(&format!(" ({})", plain(tail)));
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(s: &str) -> (String, Out) {
        let mut o = Out::default();
        (render(s, &mut o), o)
    }

    #[test]
    fn links_bold_and_templates() {
        assert_eq!(r("A [[fellow]], [[comrade|Comrade]] or '''mate'''.").0, "A fellow, Comrade or mate.");
        assert_eq!(r("{{l|en|tube}} (US), {{l|en|TV}}").0, "tube (US), TV");
        assert_eq!(r("{{m|en|hotel|(ho)tel}}").0, "(ho)tel");
        assert_eq!(r("A cat ({{taxlink|Felis catus|species}}).").0, "A cat (Felis catus).");
        assert_eq!(r("In {{w|The IT Crowd|the show}}.").0, "In the show.");
        assert_eq!(r("[[television#English|Television]].").0, "Television.");
        assert_eq!(r("x [[Category:en:Cats]] y<ref>a source</ref> z<!-- c -->").0, "x y z");
    }

    #[test]
    fn labels_examples_nyms_and_form_of() {
        let (t, o) = r("{{lb|en|colloquial|British|_|and|[[Australia]]}} A [[friend]], usually of the same sex.");
        assert_eq!(t, "A friend, usually of the same sex.");
        assert_eq!(o.labels, vec!["colloquial", "British", "Australia"]);
        let (t, o) = r("{{ux|en|He's my best '''mate'''.}}");
        assert_eq!((t.as_str(), o.examples), ("", vec!["He's my best mate.".to_string()]));
        let (_, o) = r("{{syn|en|friend|buddy<q:informal>|Thesaurus:friend}}");
        assert_eq!(o.nyms, vec![("syn".into(), "friend".into()), ("syn".into(), "buddy".into())]);
        let (t, o) = r("{{plural of|en|cat}}");
        assert_eq!((t.as_str(), o.form_of), ("Plural of cat", Some(("plural of".into(), "cat".into()))));
        let (t, o) = r("{{en-past of|run}}");
        assert_eq!((t.as_str(), o.form_of), ("Past of run", Some(("past of".into(), "run".into()))));
    }

    /// The gap rule's negative control: an unknown template is counted, never silently dropped; a known
    /// category template is counted as dropped, not as a gap.
    #[test]
    fn unknown_templates_are_counted_as_gaps() {
        let (t, o) = r("A {{zz-unknown|x}} thing{{C|en|Cats}}.");
        assert_eq!(t, "A thing.");
        assert_eq!(o.gaps.get("zz-unknown"), Some(&1));
        assert_eq!(o.dropped.get("C"), Some(&1));
        assert!(o.gaps.get("C").is_none());
    }

    #[test]
    fn nested_parameters_and_place() {
        let c = parse_call("l|en|{{w|a|b}}|t=x|[[a|b]]");
        assert_eq!(c.pos, vec!["en", "{{w|a|b}}", "[[a|b]]"]);
        assert_eq!(c.n("t"), "x");
        assert_eq!(r("{{place|en|town|p/Kent|c/England}}").0, "A town in Kent, England");
        assert_eq!(r("{{place|en|A <<town>> in <<p/Kent>>}}").0, "A town in Kent");
        assert_eq!(r("{{surname|en}}.").0, "A surname.");
        assert_eq!(r("{{place|en|hamlet|in|co/Cumbria|cc/England}}").0, "A hamlet in Cumbria, England");
        assert_eq!(r("{{abbreviation of|en|w:acute porphyria}}").0, "Abbreviation of acute porphyria");
        assert_eq!(r("See [https://example.org the site] and {{U|goodbye}}.").0, "See the site and goodbye.");
        assert_eq!(r("{{initialism of|en||Third Generation Partnership Project}}").0, "Initialism of Third Generation Partnership Project");
    }
}
