//! Typed prompt arguments for configured capabilities: absolute http(s) URLs, host names,
//! catalog identifiers (hyphen-joined slugs such as `smith-2024-tree-parsing`, recognized only
//! when some capability declares an `id` slot) and quoted free text (a span in «...», “...” or
//! "...", for example a task handed to a consultant).
//!
//! Before parsing, every URL- or host-shaped word of the prompt is validated strictly and replaced
//! by a reserved proper-noun placeholder (`Zurla`, `Zhostb`, ...), so the native parser sees an
//! ordinary name in the argument's place and sentence splitting never cuts a URL at its dots. A
//! quoted span is masked first and as a whole (`Ztexta`, ...): its words are never read as
//! instructions, and URLs inside it stay part of the text. An
//! argument reaches a capability only through the parse: its placeholder node must hang from the
//! action's request head through argument relations (object, oblique, nominal modifier,
//! apposition, conjunct, flat, compound, list, dep, subject) without crossing a clause boundary
//! (relative, adverbial or complement clause, parataxis) or an exclusion marker ("except",
//! "instead", ...). A word that is not bound that way, a missing or surplus argument, and an
//! invalid or non-public address refuse the whole prompt with a review candidate; nothing is ever
//! taken from a free substring of the text. One linear rule complements the parse: an argument of
//! any kind (quoted text, identifier, URL, host) alone right after a colon that closes a request
//! clause belongs to the nearest request before the colon (`colon_head`).
use en::annotate::Word;
use en::gram::Rel;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

/// Most arguments in one prompt.
pub const MAX_ARGUMENTS: usize = 16;
const URL_MAX: usize = 2048;
/// Longest quoted text argument in UTF-8 bytes (the whole prompt is at most 8192).
pub const TEXT_MAX: usize = 6000;
/// Quotation delimiters admitted around a text argument: (opening, closing).
const QUOTES: [(char, char); 3] = [('«', '»'), ('“', '”'), ('"', '"')];
const RESERVED: [&str; 4] = ["zurl", "zhost", "ztext", "zslug"];
/// Longest catalog identifier in bytes.
pub const ID_MAX: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotKind {
    /// An absolute http:// or https:// URL without user information.
    Url,
    /// A DNS host name (or IPv4 literal) without scheme, port or path.
    Host,
    /// Free text quoted in the prompt, opaque to the planner.
    Text,
    /// A catalog identifier: lowercase ASCII letters and digits in at least three hyphen-joined
    /// parts, with at least one letter and one digit (`smith-2024-tree-parsing`). Masked only when
    /// some capability declares a slot of this kind.
    Id,
}

/// A capability's typed argument: `min..=max` prompt words of the given kinds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SlotRule {
    pub name: String,
    pub kinds: Vec<SlotKind>,
    pub min: usize,
    pub max: usize,
}

/// Address policy for prompt arguments. Non-public hosts (loopback, private, link-local and
/// reserved names such as `localhost`, `.local`, `.test`) are refused unless allowed here; tests
/// allow them for 127.0.0.1 fixtures only.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SlotPolicy {
    #[serde(default)]
    pub allow_private_hosts: bool,
}
impl SlotPolicy {
    pub fn is_default(&self) -> bool {
        !self.allow_private_hosts
    }
}

/// One validated argument and the placeholder that stands for it in the parsed text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Argument {
    pub kind: SlotKind,
    pub value: String,
    pub placeholder: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Masked {
    pub text: String,
    pub arguments: Vec<Argument>,
}

pub fn validate_rules(rules: &[SlotRule]) -> Result<(), String> {
    if rules.len() > 4 {
        return Err("at most four slots per capability".into());
    }
    let mut seen = std::collections::BTreeSet::new();
    for r in rules {
        let name_ok = !r.name.is_empty()
            && r.name.len() <= 32
            && r.name.bytes().all(|b| b.is_ascii_lowercase() || b == b'_');
        if !name_ok || r.kinds.is_empty() || r.max == 0 || r.min > r.max || r.max > MAX_ARGUMENTS {
            return Err(format!("invalid slot {:?}", r.name));
        }
        // Kinds are disjoint across a capability's slots: an argument's kind names its slot.
        for k in &r.kinds {
            if !seen.insert(*k) {
                return Err("slot kinds must be disjoint within a capability".into());
            }
        }
    }
    Ok(())
}

fn placeholder(kind: SlotKind, i: usize) -> String {
    let letter = char::from(b'a' + i as u8);
    match kind {
        SlotKind::Url => format!("Zurl{letter}"),
        SlotKind::Host => format!("Zhost{letter}"),
        SlotKind::Text => format!("Ztext{letter}"),
        SlotKind::Id => format!("Zslug{letter}"),
    }
}

/// The shape of a catalog identifier (see `SlotKind::Id`).
pub fn id_shape(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    (5..=ID_MAX).contains(&s.len())
        && parts.len() >= 3
        && parts.iter().all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit()))
        && s.bytes().any(|b| b.is_ascii_digit())
        && s.bytes().any(|b| b.is_ascii_lowercase())
}

fn host_syntax(h: &str) -> bool {
    let labels: Vec<&str> = h.split('.').collect();
    h.len() <= 253
        && labels.len() >= 2
        && labels.iter().all(|l| {
            !l.is_empty()
                && l.len() <= 63
                && l.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
                && !l.starts_with('-')
                && !l.ends_with('-')
        })
        && labels.last().is_some_and(|t| t.len() >= 2 && !t.bytes().all(|b| b.is_ascii_digit()))
}

fn ipv4(h: &str) -> Option<Ipv4Addr> {
    let parts: Vec<&str> = h.split('.').collect();
    if parts.len() != 4 || parts.iter().any(|p| p.is_empty() || p.len() > 3 || !p.bytes().all(|b| b.is_ascii_digit())) {
        return None;
    }
    h.parse().ok()
}

fn private_ipv4(ip: Ipv4Addr) -> bool {
    let o = ip.octets();
    ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_unspecified()
        || ip.is_broadcast()
        || ip.is_multicast()
        || ip.is_documentation()
        || o[0] == 0
        || (o[0] == 100 && (64..128).contains(&o[1]))
        || o[0] >= 240
}

fn private_name(h: &str) -> bool {
    h == "localhost"
        || ["localhost", "local", "internal", "lan", "home.arpa", "test", "invalid"]
            .iter()
            .any(|s| h.ends_with(&format!(".{s}")))
}

/// Validate a host (lowercase): a DNS name or an IPv4 literal, public unless the policy allows.
pub fn validate_host(h: &str, policy: &SlotPolicy) -> Result<String, String> {
    let h = h.to_ascii_lowercase();
    let private = match ipv4(&h) {
        Some(ip) => private_ipv4(ip),
        None if host_syntax(&h) => private_name(&h),
        None if h == "localhost" => true,
        None => return Err(format!("{h:?} is not a valid host name")),
    };
    if private && !policy.allow_private_hosts {
        return Err(format!("{h} is a loopback, private or reserved address"));
    }
    Ok(h)
}

/// Validate an absolute URL: http or https, no user information, a valid public host, an
/// optional port in 1..=65535, printable ASCII without spaces, quotes or angle brackets.
pub fn validate_url(s: &str, policy: &SlotPolicy) -> Result<String, String> {
    if s.len() > URL_MAX || !s.is_ascii() || s.bytes().any(|b| b.is_ascii_control() || b" \"<>\\^`{|}".contains(&b)) {
        return Err("a URL must be printable ASCII of at most 2048 bytes".into());
    }
    let (scheme, rest) = s.split_once("://").ok_or("not an absolute URL")?;
    let scheme = scheme.to_ascii_lowercase();
    if scheme != "http" && scheme != "https" {
        return Err(format!("only http and https URLs are fetched, not {scheme}"));
    }
    let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let (authority, tail) = rest.split_at(end);
    if authority.contains('@') {
        return Err("a URL with user information (credentials) is refused".into());
    }
    if authority.starts_with('[') {
        return Err("IPv6 literal hosts are not admitted".into());
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) => {
            let n: u32 = p.parse().map_err(|_| format!("invalid port {p:?}"))?;
            if !(1..=65535).contains(&n) || !p.bytes().all(|b| b.is_ascii_digit()) {
                return Err(format!("invalid port {p:?}"));
            }
            (h, format!(":{n}"))
        }
        None => (authority, String::new()),
    };
    let host = validate_host(host, policy)?;
    let tail = if tail.is_empty() { "/" } else { tail };
    Ok(format!("{scheme}://{host}{port}{tail}"))
}

/// Split a whitespace word into leading punctuation, core and trailing punctuation. A closing
/// parenthesis stays in the core while it balances an opening one (`.../Foo_(bar)`).
fn trim_word(word: &str) -> (&str, &str, &str) {
    let start = word.len() - word.trim_start_matches(['(', '<', '[']).len();
    let mut end = word.len();
    loop {
        let core = &word[start..end];
        let Some(c) = core.chars().last() else { break };
        let unbalanced = c == ')' && core.matches(')').count() > core.matches('(').count();
        if ".,;:!?>]".contains(c) || unbalanced {
            end -= c.len_utf8();
        } else {
            break;
        }
    }
    (&word[..start], &word[start..end], &word[end..])
}

fn reserved_word(text: &str) -> Option<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .find(|w| RESERVED.iter().any(|r| w.to_ascii_lowercase().starts_with(r)))
        .map(str::to_string)
}

/// Validate one quoted text: non-empty, bounded, no control characters except newline and tab.
pub fn validate_text(t: &str) -> Result<String, String> {
    let t = t.trim();
    if t.is_empty() {
        return Err("a quoted text argument is empty".into());
    }
    if t.len() > TEXT_MAX {
        return Err(format!("a quoted text argument exceeds {TEXT_MAX} bytes"));
    }
    if t.chars().any(|c| c.is_control() && c != '\n' && c != '\t') {
        return Err("a quoted text argument contains control characters".into());
    }
    Ok(t.to_string())
}

/// Mask quoted spans as whole text arguments. A quote that is not closed, an opening quote inside
/// a span, and a reserved placeholder word outside the spans refuse the prompt.
fn mask_quotes(prompt: &str, arguments: &mut Vec<Argument>) -> Result<String, String> {
    let mut out = String::with_capacity(prompt.len());
    let mut rest = prompt;
    loop {
        let next = rest
            .char_indices()
            .find_map(|(i, c)| QUOTES.iter().find(|q| q.0 == c || q.1 == c).map(|q| (i, c, *q)));
        let Some((i, c, (open, close))) = next else {
            if let Some(w) = reserved_word(rest) {
                return Err(format!("{w:?} is a reserved argument placeholder"));
            }
            out.push_str(rest);
            return Ok(out);
        };
        if c != open {
            return Err(format!("closing quote {c} without its opening quote"));
        }
        if let Some(w) = reserved_word(&rest[..i]) {
            return Err(format!("{w:?} is a reserved argument placeholder"));
        }
        out.push_str(&rest[..i]);
        let after = &rest[i + open.len_utf8()..];
        let end = after.find(close).ok_or_else(|| format!("quote {open} is not closed"))?;
        let inner = &after[..end];
        if open != close && inner.contains(open) {
            return Err(format!("nested quote {open} inside a quoted text"));
        }
        if arguments.len() == MAX_ARGUMENTS {
            return Err(format!("at most {MAX_ARGUMENTS} arguments in one request"));
        }
        let p = placeholder(SlotKind::Text, arguments.len());
        out.push_str(&p);
        arguments.push(Argument { kind: SlotKind::Text, value: validate_text(inner)?, placeholder: p });
        rest = &after[end + close.len_utf8()..];
    }
}

/// Replace argument words by placeholders, validating each. Quoted spans become text arguments
/// first. Words that merely contain a dot stay text unless they have URL or host shape; a host
/// followed by a path but no scheme is refused. Catalog identifiers stay text.
pub fn mask(prompt: &str, policy: &SlotPolicy) -> Result<Masked, String> {
    mask_with(prompt, policy, false)
}

/// As `mask`; with `ids` true, words of catalog-identifier shape also become `id` arguments.
pub fn mask_with(prompt: &str, policy: &SlotPolicy, ids: bool) -> Result<Masked, String> {
    let mut arguments = vec![];
    let unquoted = mask_quotes(prompt, &mut arguments)?;
    let texts: Vec<String> = arguments.iter().map(|a| a.placeholder.clone()).collect();
    let mut text = String::with_capacity(unquoted.len());
    for raw in unquoted.split_inclusive(char::is_whitespace) {
        let word = raw.trim_end_matches(char::is_whitespace);
        let space = &raw[word.len()..];
        let (lead, core, trail) = trim_word(word);
        let lower = core.to_ascii_lowercase();
        if texts.iter().any(|t| t == core) {
            text.push_str(raw);
            continue;
        }
        if RESERVED.iter().any(|r| lower.starts_with(r)) {
            return Err(format!("{core:?} is a reserved argument placeholder"));
        }
        let found = if core.contains("://") {
            Some((SlotKind::Url, validate_url(core, policy)?))
        } else if ipv4(&lower).is_some() || (host_syntax(&lower) && !lower.contains('@')) {
            Some((SlotKind::Host, validate_host(&lower, policy)?))
        } else if ids && id_shape(core) {
            Some((SlotKind::Id, core.to_string()))
        } else if let Some((h, _)) = lower.split_once('/') {
            if host_syntax(h) {
                return Err(format!("{core:?} needs its http:// or https:// scheme"));
            }
            None
        } else {
            None
        };
        match found {
            Some((kind, value)) => {
                if arguments.len() == MAX_ARGUMENTS {
                    return Err(format!("at most {MAX_ARGUMENTS} URLs, hosts or identifiers in one request"));
                }
                let p = placeholder(kind, arguments.len());
                text.push_str(&format!("{lead}{p}{trail}{space}"));
                arguments.push(Argument { kind, value, placeholder: p });
            }
            None => text.push_str(raw),
        }
    }
    Ok(Masked { text, arguments })
}

/// Relations an argument may hang by on its way to the request head; anything else (clauses,
/// modifiers of other kinds) ends the path unbound.
fn argument_relation(r: Rel) -> bool {
    matches!(
        r.base(),
        Rel::Obj | Rel::Iobj | Rel::Obl | Rel::Nmod | Rel::Appos | Rel::Conj | Rel::Compound
            | Rel::Flat | Rel::List | Rel::Dep | Rel::Nsubj
    )
}

/// Dependents that exclude rather than name: "everything except X", "X instead of Y".
const EXCLUDERS: [&str; 8] = ["except", "excluding", "instead", "besides", "but", "unlike", "rather", "other"];

/// The nearest request head reached from `node` through argument relations, or None when the
/// path crosses a clause boundary, reaches the root without a head, or carries an exclusion.
pub fn bound_head(w: &[Word], node: usize, heads: &[usize]) -> Option<usize> {
    let mut at = node;
    for _ in 0..w.len() {
        let excluded = w.iter().any(|d| d.head == at + 1 && EXCLUDERS.contains(&d.lemma.to_lowercase().as_str()));
        if excluded {
            return None;
        }
        if heads.contains(&at) {
            return Some(at);
        }
        if w[at].head == 0 || !argument_relation(w[at].rel) {
            return None;
        }
        at = w[at].head - 1;
    }
    None
}

/// An argument introduced by a colon after the request ("Ask the Opus Dolphin: «...»", "Survey
/// this paper: paper-2023-gamma.") is the content of that request, however the parser attached the
/// placeholder (the native parses read it as `vocative`, `parataxis`, `appos`, `obl` or `obj`
/// depending on the model and the verb). The rule is linear, not tree-based and the same for every
/// slot kind: `node` is an argument placeholder, the word right before it is a colon,
/// only punctuation follows it, nothing excluding hangs from it, and a request head precedes the
/// colon. The nearest such head (the request whose clause the colon closes) is returned.
/// Not covered: a span before the request, a colon with words after the span, a colon with no
/// request head before it; those stay with `bound_head` alone.
pub fn colon_head(w: &[Word], node: usize, heads: &[usize]) -> Option<usize> {
    let colon = node.checked_sub(1)?;
    let is_punct = |i: usize| w[i].upos == en::gram::UPos::PUNCT;
    if w[colon].form != ":" || !is_punct(colon) || !(node + 1..w.len()).all(is_punct) {
        return None;
    }
    if w.iter().any(|d| d.head == node + 1 && EXCLUDERS.contains(&d.lemma.to_lowercase().as_str())) {
        return None;
    }
    heads.iter().copied().filter(|&h| h < colon).max()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn open() -> SlotPolicy {
        SlotPolicy { allow_private_hosts: true }
    }
    #[test]
    fn urls_are_validated_strictly() {
        let p = SlotPolicy::default();
        assert_eq!(validate_url("https://Mova.Elamur.AI", &p).unwrap(), "https://mova.elamur.ai/");
        assert_eq!(validate_url("HTTP://example.org:8080/a?b=1#c", &p).unwrap(), "http://example.org:8080/a?b=1#c");
        for bad in [
            "ftp://example.org/",
            "file:///etc/passwd",
            "https://user:pw@example.org/",
            "https://example.org@evil.example/",
            "http://127.0.0.1/",
            "http://localhost:8080/",
            "http://10.1.2.3/",
            "http://192.168.0.1/",
            "http://169.254.1.1/",
            "http://a.test/",
            "http://[::1]/",
            "https://example.org:0/",
            "https://example.org:99999/",
            "https://exa mple.org/",
            "https://example/",
            "https://-bad.example.org/",
        ] {
            assert!(validate_url(bad, &p).is_err(), "{bad}");
        }
        assert_eq!(validate_url("http://127.0.0.1:9/x", &open()).unwrap(), "http://127.0.0.1:9/x");
        assert_eq!(validate_url("http://a.test:9/x", &open()).unwrap(), "http://a.test:9/x");
    }
    #[test]
    fn masking_keeps_text_and_numbers_arguments() {
        let m = mask("Fetch https://mova.elamur.ai/ and (https://example.org/Foo_(bar)), please.", &SlotPolicy::default()).unwrap();
        assert_eq!(m.text, "Fetch Zurla and (Zurlb), please.");
        assert_eq!(m.arguments[1].value, "https://example.org/Foo_(bar)");
        let m = mask("Are we allowed to fetch Mova.Elamur.AI?", &SlotPolicy::default()).unwrap();
        assert_eq!(m.text, "Are we allowed to fetch Zhosta?");
        assert_eq!(m.arguments[0].value, "mova.elamur.ai");
        // Dotted words that are not hosts stay text; a schemeless path, a private host, a
        // reserved placeholder and too many arguments are refused.
        assert_eq!(mask("Read e.g. this, i.e. 0.4 now.", &SlotPolicy::default()).unwrap().arguments.len(), 0);
        for bad in ["Fetch mova.elamur.ai/about.", "Fetch http://localhost/.", "Fetch Zurla.", "Get ftp://example.org/x"] {
            assert!(mask(bad, &SlotPolicy::default()).is_err(), "{bad}");
        }
        let many = (0..17).map(|i| format!("https://h{i}.example.org/")).collect::<Vec<_>>().join(" ");
        assert!(mask(&format!("Fetch {many}"), &SlotPolicy::default()).is_err());
    }
    #[test]
    fn slot_rules_are_bounded_and_disjoint() {
        let r = |name: &str, kinds: Vec<SlotKind>, min, max| SlotRule { name: name.into(), kinds, min, max };
        assert!(validate_rules(&[r("urls", vec![SlotKind::Url], 1, 16)]).is_ok());
        assert!(validate_rules(&[r("urls", vec![SlotKind::Url], 2, 1)]).is_err());
        assert!(validate_rules(&[r("urls", vec![SlotKind::Url], 0, 17)]).is_err());
        assert!(validate_rules(&[r("a", vec![SlotKind::Url], 0, 1), r("b", vec![SlotKind::Url, SlotKind::Host], 0, 1)]).is_err());
        assert!(validate_rules(&[r("Bad-Name", vec![SlotKind::Url], 0, 1)]).is_err());
        assert!(validate_rules(&[r("task", vec![SlotKind::Text], 1, 1), r("urls", vec![SlotKind::Url], 0, 1)]).is_ok());
    }
    #[test]
    fn catalog_identifiers_are_masked_only_when_enabled() {
        let p = SlotPolicy::default();
        let prompt = "Describe this paper: smith-2024-tree-parsing, and a-2005-brief-survey-text-mining.";
        let m = mask_with(prompt, &p, true).unwrap();
        assert_eq!(m.text, "Describe this paper: Zsluga, and Zslugb.");
        assert_eq!((m.arguments[0].kind, m.arguments[0].value.as_str()), (SlotKind::Id, "smith-2024-tree-parsing"));
        // Negative control: without an id slot the same words stay text.
        assert_eq!(mask(prompt, &p).unwrap().arguments.len(), 0);
        for not_id in ["state-of-the-art", "2024-11-04", "well-known", "Smith-2024-tree", "a--2005-x", "-a-1-b", "a-1-b-"] {
            assert!(!id_shape(not_id), "{not_id}");
        }
        assert!(id_shape("a-1-b"));
        assert!(!id_shape(&format!("a-1-{}", "b".repeat(ID_MAX))));
        assert!(mask_with("Describe Zslugb.", &p, true).is_err());
    }
    /// Native parses (en.bin 2483516a and the one before it, 2026-11-05) of colon-introduced tasks.
    fn words(spec: &[(&str, en::gram::UPos, usize, Rel)]) -> Vec<Word> {
        spec.iter()
            .map(|(form, upos, head, rel)| Word {
                form: (*form).into(),
                lemma: (*form).into(),
                upos: *upos,
                tag: en::gram::Tag::NN,
                feats: en::gram::Feats(0),
                head: *head,
                rel: *rel,
            })
            .collect()
    }
    #[test]
    fn a_colon_hands_the_quotation_to_the_preceding_request() {
        use en::gram::UPos::*;
        // «ask the copilot dolphin : ztexta»: the placeholder is a vocative of «ask».
        let ask = |rel| {
            words(&[
                ("ask", VERB, 0, Rel::Root),
                ("the", DET, 4, Rel::Det),
                ("copilot", PROPN, 4, Rel::Compound),
                ("dolphin", PROPN, 1, Rel::Obj),
                (":", PUNCT, 6, Rel::Punct),
                ("ztexta", PROPN, 1, rel),
            ])
        };
        for rel in [Rel::Vocative, Rel::Parataxis, Rel::Obj] {
            assert_eq!(colon_head(&ask(rel), 5, &[0]), Some(0), "{rel:?}");
        }
        // The parse alone refuses the vocative reading: the colon rule is what binds it.
        assert_eq!(bound_head(&ask(Rel::Vocative), 5, &[0]), None);
        // Two requests: the nearest one before the colon («check crab and ask the dolphin: ...»).
        assert_eq!(colon_head(&ask(Rel::Vocative), 5, &[0, 3]), Some(3));
        // Negative controls: no colon, no request head before the colon, words after the span,
        // a span before the request, an excluding dependent.
        let mut no_colon = ask(Rel::Vocative);
        no_colon[4].form = ",".into();
        assert_eq!(colon_head(&no_colon, 5, &[0]), None);
        let note = words(&[("note", NOUN, 0, Rel::Root), (":", PUNCT, 3, Rel::Punct), ("ztexta", PROPN, 1, Rel::Appos)]);
        assert_eq!(colon_head(&note, 2, &[]), None);
        let mut trailing = ask(Rel::Vocative);
        trailing.push(Word { form: "now".into(), lemma: "now".into(), upos: ADV, tag: en::gram::Tag::RB, feats: en::gram::Feats(0), head: 1, rel: Rel::Advmod });
        assert_eq!(colon_head(&trailing, 5, &[0]), None);
        let before = words(&[("ztexta", PROPN, 2, Rel::Nsubj), ("ask", VERB, 0, Rel::Root), ("dolphin", PROPN, 2, Rel::Obj)]);
        assert_eq!(colon_head(&before, 0, &[1]), None);
        let except = words(&[
            ("ask", VERB, 0, Rel::Root),
            ("dolphin", PROPN, 1, Rel::Obj),
            ("except", ADP, 5, Rel::Case),
            (":", PUNCT, 5, Rel::Punct),
            ("ztexta", PROPN, 1, Rel::Vocative),
        ]);
        assert_eq!(colon_head(&except, 4, &[0]), None);
        let mut kept = except.clone();
        kept[2] = Word { form: "please".into(), lemma: "please".into(), ..kept[2].clone() };
        assert_eq!(colon_head(&kept, 4, &[0]), Some(0));
    }
    #[test]
    fn quoted_text_is_one_opaque_argument() {
        let p = SlotPolicy::default();
        let m = mask("Ask the consultant «Is https://x.example.org/ fine? Do not stop Eel.», please.", &p).unwrap();
        assert_eq!(m.text, "Ask the consultant Ztexta, please.");
        assert_eq!(m.arguments.len(), 1);
        assert_eq!(m.arguments[0].kind, SlotKind::Text);
        assert_eq!(m.arguments[0].value, "Is https://x.example.org/ fine? Do not stop Eel.");
        let m = mask("Ask “one” and \"two\" at https://a.example.org/", &p).unwrap();
        assert_eq!(m.text, "Ask Ztexta and Ztextb at Zurlc");
        assert_eq!(m.arguments[1].value, "two");
        for bad in [
            "Ask «unclosed.",
            "Ask » stray.",
            "Ask «a «nested» b».",
            "Ask «   ».",
            "Ask \"open only.",
            "Ask Ztexta now.",
            "Ask «fine» and Ztextb.",
        ] {
            assert!(mask(bad, &p).is_err(), "{bad}");
        }
        assert!(mask(&format!("Ask «{}».", "x".repeat(TEXT_MAX + 1)), &p).is_err());
        assert!(mask("Ask «a\u{7}b».", &p).is_err());
    }
}
