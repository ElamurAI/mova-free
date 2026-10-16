//! The converter of the linguists' open role lexicons into compact tables (read by `crate::vn`). What the reader is
//! built from must be permissive (licence/DATA.md); share-alike and research-only data serve tests only:
//!
//! - VerbNet 3.4 (cu-clear/verbnet, VerbNet 3.0 License — permissive; built in as `prag/data/verbnet.tsv`): classes
//!   with members, thematic roles with selectional restrictions, frames (a syntactic pattern → roles) with their
//!   semantic predicates;
//! - Princeton WordNet 3.1 (WordNet 3.0 License — permissive; built in as `prag/data/nominals.tsv`): the nouns whose
//!   first sense names a verb's act or event, by WordNet's derivational links («arrival» → arrive), and whether the
//!   linked verb sense takes an object;
//! - TESTS ONLY, written outside the repo ($MOVA_DATA/raw-derived/prag-lexicon): PropBank frames 3.1 (CC BY-SA 4.0 —
//!   share-alike): rolesets, ARG0..ARG5 with function tags, descriptions, VerbNet links, the prepositions the examples
//!   use for each argument; NomBank 1.0 (NYU, research-only; frames and NOMLEX-plus): nominalisations and how their
//!   arguments are written (of / by / possessive / noun modifier), and NomBank's noun rolesets with their source verb.
//!
//! `prag lexicon convert <verbnet3.4 dir> <propbank frames dir> <nombank.1.0 dir> <out dir> [<test-only dir>]`,
//! `prag lexicon nominals <wordnet dict dir> <out dir>`: deterministic (files in name order, rows in document order);
//! the header of each table records the source and a checksum of what was read. No XML crate: the files are plain, a
//! small tag reader does.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result};

// ---- a small XML reader ----------------------------------------------------------------------------------------

#[derive(Debug)]
enum Ev {
    /// name, attributes, self-closing
    Open(String, Vec<(String, String)>, bool),
    Close(String),
    Text(String),
}

fn unescape(s: &str) -> String {
    s.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'").replace("&amp;", "&")
}

fn events(s: &str) -> Vec<Ev> {
    let mut out = Vec::new();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'<' {
            let j = s[i..].find('<').map(|k| i + k).unwrap_or(b.len());
            let t = unescape(s[i..j].trim());
            if !t.is_empty() {
                out.push(Ev::Text(t));
            }
            i = j;
            continue;
        }
        if s[i..].starts_with("<!--") {
            i = s[i..].find("-->").map(|k| i + k + 3).unwrap_or(b.len());
            continue;
        }
        if s[i..].starts_with("<!") || s[i..].starts_with("<?") {
            i = s[i..].find('>').map(|k| i + k + 1).unwrap_or(b.len());
            continue;
        }
        // a tag: find its end outside quotes
        let mut j = i + 1;
        let mut q: Option<u8> = None;
        while j < b.len() {
            match (q, b[j]) {
                (None, b'"') | (None, b'\'') => q = Some(b[j]),
                (Some(c), x) if x == c => q = None,
                (None, b'>') => break,
                _ => {}
            }
            j += 1;
        }
        let inner = &s[i + 1..j.min(b.len())];
        i = j + 1;
        if let Some(name) = inner.strip_prefix('/') {
            out.push(Ev::Close(name.trim().to_string()));
            continue;
        }
        let selfc = inner.trim_end().ends_with('/');
        let inner = inner.trim_end().trim_end_matches('/');
        let name_end = inner.find(|c: char| c.is_whitespace()).unwrap_or(inner.len());
        let name = inner[..name_end].to_string();
        let mut attrs = Vec::new();
        let rest = &inner[name_end..];
        let rb = rest.as_bytes();
        let mut k = 0;
        while k < rb.len() {
            while k < rb.len() && rb[k].is_ascii_whitespace() {
                k += 1;
            }
            let ks = k;
            while k < rb.len() && rb[k] != b'=' && !rb[k].is_ascii_whitespace() {
                k += 1;
            }
            let key = rest[ks..k].to_string();
            while k < rb.len() && (rb[k] == b'=' || rb[k].is_ascii_whitespace()) {
                k += 1;
            }
            if k < rb.len() && (rb[k] == b'"' || rb[k] == b'\'') {
                let qc = rb[k];
                let vs = k + 1;
                let ve = rest[vs..].find(qc as char).map(|x| vs + x).unwrap_or(rb.len());
                attrs.push((key, unescape(&rest[vs..ve])));
                k = ve + 1;
            } else if !key.is_empty() {
                attrs.push((key, String::new()));
            }
        }
        out.push(Ev::Open(name, attrs, selfc));
    }
    out
}

fn attr<'a>(a: &'a [(String, String)], k: &str) -> &'a str {
    a.iter().find(|(x, _)| x == k).map(|(_, v)| v.as_str()).unwrap_or("")
}

/// A table cell: no tabs, newlines or the separators the tables use inside cells.
fn cell(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ").replace(['\t', '|'], "/")
}

/// FNV-1a over everything read: the tables' header records it, so a different input shows.
#[derive(Default)]
struct Sum(u64);

impl Sum {
    fn add(&mut self, s: &str) {
        if self.0 == 0 {
            self.0 = 0xcbf29ce484222325;
        }
        for b in s.bytes() {
            self.0 ^= b as u64;
            self.0 = self.0.wrapping_mul(0x100000001b3);
        }
    }
}

fn xml_files(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut v: Vec<_> = std::fs::read_dir(dir).with_context(|| format!("{}", dir.display()))?.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.extension().is_some_and(|x| x == "xml")).collect();
    v.sort();
    Ok(v)
}

/// Particles and prepositions: a member «bring-up» / «back_down» is the phrasal verb «bring up».
const PRT: [&str; 30] = ["up", "down", "off", "out", "away", "back", "over", "around", "round", "on", "in", "about", "through", "along", "together", "apart", "by", "forth", "for", "to", "into", "of", "with", "from", "at", "upon", "after", "across", "behind", "like"];

/// A member or alias as the reader writes the verb: «give_up» → «give up», «bring-out» → «bring out».
pub fn lemma_of(name: &str) -> String {
    let s = name.trim().to_lowercase().replace('_', " ");
    if let Some((a, b)) = s.rsplit_once('-') {
        if PRT.contains(&b) && !a.is_empty() {
            return format!("{} {b}", a.replace('-', " "));
        }
    }
    s
}

// ---- VerbNet ---------------------------------------------------------------------------------------------------

/// Restrictions being read: nested groups of `SELRESTR`/`SYNRESTR` with their logic.
#[derive(Default)]
struct Restr {
    groups: Vec<(bool, Vec<String>)>,
    done: Option<String>,
}

impl Restr {
    fn open(&mut self, a: &[(String, String)]) {
        self.groups.push((attr(a, "logic") == "or", Vec::new()));
    }
    fn item(&mut self, a: &[(String, String)]) {
        let v = format!("{}{}", attr(a, "Value"), attr(a, "type"));
        if let Some(g) = self.groups.last_mut() {
            g.1.push(v);
        } else {
            self.done = Some(v);
        }
    }
    fn close(&mut self) {
        let Some((or, items)) = self.groups.pop() else { return };
        if items.is_empty() {
            return;
        }
        let s = items.join(if or { "|" } else { "&" });
        match self.groups.last_mut() {
            Some(g) => g.1.push(if items.len() > 1 { format!("({s})") } else { s }),
            None => self.done = Some(match self.done.take() { Some(d) => format!("{d}&{s}"), None => s }),
        }
    }
    fn take(&mut self) -> String {
        self.groups.clear();
        self.done.take().unwrap_or_default()
    }
}

struct VClass {
    id: String,
    parent: String,
    members: Vec<String>,
    roles: Vec<String>,
    frames: Vec<String>,
}

fn verbnet(dir: &Path, sum: &mut Sum) -> Result<(String, usize, usize, usize)> {
    let mut classes: Vec<VClass> = Vec::new();
    for f in xml_files(dir)? {
        let s = std::fs::read_to_string(&f)?;
        sum.add(&s);
        let mut stack: Vec<usize> = Vec::new();
        let mut restr = Restr::default();
        // what the restrictions being read belong to
        #[derive(PartialEq)]
        enum In {
            None,
            Role(String),
            Np(String),
            Prep(String),
        }
        let mut target = In::None;
        let mut in_syntax = false;
        let mut in_sem = false;
        let mut in_example = false;
        let (mut primary, mut syntax, mut sem, mut example): (String, Vec<String>, Vec<String>, String) = Default::default();
        let finish = |target: &mut In, restr: &mut Restr, classes: &mut Vec<VClass>, stack: &[usize], syntax: &mut Vec<String>| {
            let r = restr.take();
            match std::mem::replace(target, In::None) {
                In::Role(t) => {
                    if let Some(&c) = stack.last() {
                        classes[c].roles.push(if r.is_empty() { t } else { format!("{t}[{r}]") });
                    }
                }
                In::Np(v) => syntax.push(if r.is_empty() { v } else { format!("{v}[{r}]") }),
                In::Prep(v) => syntax.push(if r.is_empty() { format!("{{{v}}}") } else if v.is_empty() { format!("{{{r}}}") } else { format!("{{{v}}}[{r}]") }),
                In::None => {}
            }
        };
        for ev in events(&s) {
            match ev {
                Ev::Open(n, a, selfc) => match n.as_str() {
                    "VNCLASS" | "VNSUBCLASS" => {
                        let parent = stack.last().map(|&c| classes[c].id.clone()).unwrap_or_else(|| "-".into());
                        classes.push(VClass { id: attr(&a, "ID").to_string(), parent, members: Vec::new(), roles: Vec::new(), frames: Vec::new() });
                        stack.push(classes.len() - 1);
                    }
                    "MEMBER" => {
                        if let Some(&c) = stack.last() {
                            classes[c].members.push(lemma_of(attr(&a, "name")));
                        }
                    }
                    "THEMROLE" => {
                        target = In::Role(attr(&a, "type").to_string());
                        if selfc {
                            finish(&mut target, &mut restr, &mut classes, &stack, &mut syntax);
                        }
                    }
                    "SELRESTRS" | "SYNRESTRS" if !selfc => restr.open(&a),
                    "SELRESTR" | "SYNRESTR" => restr.item(&a),
                    "DESCRIPTION" => primary = attr(&a, "primary").to_string(),
                    "EXAMPLE" => in_example = example.is_empty(),
                    "SYNTAX" => in_syntax = true,
                    "SEMANTICS" => in_sem = true,
                    "NP" if in_syntax => {
                        target = In::Np(attr(&a, "value").to_string());
                        if selfc {
                            finish(&mut target, &mut restr, &mut classes, &stack, &mut syntax);
                        }
                    }
                    "PREP" if in_syntax => {
                        target = In::Prep(attr(&a, "value").split_whitespace().collect::<Vec<_>>().join("|"));
                        if selfc {
                            finish(&mut target, &mut restr, &mut classes, &stack, &mut syntax);
                        }
                    }
                    "VERB" if in_syntax => syntax.push("V".into()),
                    "LEX" if in_syntax => syntax.push(format!("'{}'", attr(&a, "value").trim())),
                    "ADJ" if in_syntax => syntax.push("ADJ".into()),
                    "ADV" if in_syntax => syntax.push("ADV".into()),
                    "PRED" if in_sem => sem.push(format!("{}{}", if attr(&a, "bool") == "!" { "!" } else { "" }, attr(&a, "value"))),
                    _ => {}
                },
                Ev::Close(n) => match n.as_str() {
                    "VNCLASS" | "VNSUBCLASS" => {
                        stack.pop();
                    }
                    "SELRESTRS" | "SYNRESTRS" => restr.close(),
                    "THEMROLE" | "NP" | "PREP" => finish(&mut target, &mut restr, &mut classes, &stack, &mut syntax),
                    "EXAMPLE" => in_example = false,
                    "SYNTAX" => in_syntax = false,
                    "SEMANTICS" => in_sem = false,
                    "FRAME" => {
                        if let Some(&c) = stack.last() {
                            sem.dedup();
                            classes[c].frames.push(format!("{}\t{}\t{}\t{}", cell(&primary), syntax.join(" ").replace('\t', " "), cell(&sem.join(" ")), cell(&example)));
                        }
                        primary.clear();
                        syntax.clear();
                        sem.clear();
                        example.clear();
                    }
                    _ => {}
                },
                Ev::Text(t) => {
                    if in_example {
                        example = t;
                        in_example = false;
                    }
                }
            }
        }
    }
    let mut out = String::new();
    let (mut nm, mut nf) = (0, 0);
    for c in &classes {
        writeln!(out, "class\t{}\t{}\t{}\t{}", c.id, c.parent, c.members.join(","), c.roles.join(" "))?;
        nm += c.members.len();
        for f in &c.frames {
            writeln!(out, "frame\t{}\t{f}", c.id)?;
            nf += 1;
        }
    }
    Ok((out, classes.len(), nm, nf))
}

// ---- PropBank --------------------------------------------------------------------------------------------------

/// Words an argument of an example can start with that say how it is written.
const PREPS: [&str; 40] = ["to", "for", "from", "with", "about", "at", "in", "on", "into", "onto", "of", "by", "as", "against", "over", "under", "through", "toward", "towards", "out", "off", "up", "down", "around", "across", "between", "among", "upon", "after", "before", "behind", "like", "near", "inside", "within", "without", "beyond", "past", "along", "than"];

fn propbank(dir: &Path, sum: &mut Sum) -> Result<(String, usize)> {
    let mut out = String::new();
    let mut n = 0;
    for f in xml_files(dir)? {
        let s = std::fs::read_to_string(&f)?;
        sum.add(&s);
        let mut lemma = String::new();
        // the roleset being read
        let mut id = String::new();
        let mut name = String::new();
        let mut vncls: Vec<String> = Vec::new();
        let mut args: Vec<(String, String, String, String, Vec<String>)> = Vec::new();
        let mut aliases: [Vec<String>; 3] = Default::default();
        let mut alias_pos = String::new();
        let mut in_alias = false;
        let mut in_role = false;
        let mut arg_n: Option<String> = None;
        let mut in_example = false;
        for ev in events(&s) {
            match ev {
                Ev::Open(t, a, selfc) => match t.as_str() {
                    "predicate" => lemma = lemma_of(attr(&a, "lemma")),
                    "roleset" => {
                        id = attr(&a, "id").to_string();
                        name = attr(&a, "name").to_string();
                        vncls.clear();
                        args.clear();
                        aliases = Default::default();
                        for c in attr(&a, "vncls").split_whitespace() {
                            if c != "-" && !vncls.contains(&c.to_string()) {
                                vncls.push(c.to_string());
                            }
                        }
                    }
                    "alias" if !selfc => {
                        in_alias = true;
                        alias_pos = attr(&a, "pos").to_string();
                        for c in attr(&a, "verbnet").split_whitespace() {
                            if c != "-" && !vncls.contains(&c.to_string()) {
                                vncls.push(c.to_string());
                            }
                        }
                    }
                    "role" => {
                        in_role = !selfc;
                        args.push((attr(&a, "n").to_string(), attr(&a, "f").to_lowercase(), String::new(), cell(attr(&a, "descr")), Vec::new()));
                    }
                    "vnrole" if in_role => {
                        let c = attr(&a, "vncls").to_string();
                        if !c.is_empty() && c != "-" && !vncls.contains(&c) {
                            vncls.push(c);
                        }
                        if let Some(x) = args.last_mut() {
                            let th = attr(&a, "vntheta").to_lowercase();
                            if !th.is_empty() && !x.2.split(',').any(|y| y == th) {
                                x.2 = if x.2.is_empty() { th } else { format!("{},{th}", x.2) };
                            }
                        }
                    }
                    "example" => in_example = true,
                    "arg" if in_example && !selfc => arg_n = Some(attr(&a, "n").to_string()),
                    _ => {}
                },
                Ev::Close(t) => match t.as_str() {
                    "alias" => in_alias = false,
                    "role" => in_role = false,
                    "example" => in_example = false,
                    "arg" => arg_n = None,
                    "roleset" => {
                        let a: Vec<String> = args.iter().map(|(n, f, th, d, ps)| format!("{n}:{f}:{th}:{}:{d}", ps.join(","))).collect();
                        writeln!(out, "roleset\t{id}\t{lemma}\t{}\t{}\t{}\t{}\t{}\t{}", cell(&name), vncls.join(","), a.join("|"), aliases[0].join(","), aliases[1].join(","), aliases[2].join(","))?;
                        n += 1;
                    }
                    _ => {}
                },
                Ev::Text(x) => {
                    if in_alias {
                        let k = match alias_pos.as_str() {
                            "v" => 0,
                            "n" => 1,
                            _ => 2,
                        };
                        let l = lemma_of(&x);
                        if !aliases[k].contains(&l) {
                            aliases[k].push(l);
                        }
                    } else if let Some(an) = &arg_n {
                        let first = x.split_whitespace().next().unwrap_or("").to_lowercase();
                        if PREPS.contains(&first.as_str()) {
                            if let Some(r) = args.iter_mut().find(|r| &r.0 == an) {
                                if !r.4.contains(&first) {
                                    r.4.push(first);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok((out, n))
}

// ---- NomBank and NOMLEX-plus -----------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Sx {
    Atom(String),
    Str(String),
    List(Vec<Sx>),
}

fn sexps(s: &str) -> Vec<Sx> {
    let cs: Vec<char> = s.chars().collect();
    let mut stack: Vec<Vec<Sx>> = vec![Vec::new()];
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c == ';' {
            while i < cs.len() && cs[i] != '\n' {
                i += 1;
            }
        } else if c == '(' {
            stack.push(Vec::new());
            i += 1;
        } else if c == ')' {
            let l = stack.pop().unwrap_or_default();
            if let Some(top) = stack.last_mut() {
                top.push(Sx::List(l));
            } else {
                stack.push(vec![Sx::List(l)]);
            }
            i += 1;
        } else if c == '"' {
            let mut j = i + 1;
            let mut t = String::new();
            while j < cs.len() && cs[j] != '"' {
                if cs[j] == '\\' && j + 1 < cs.len() {
                    j += 1;
                }
                t.push(cs[j]);
                j += 1;
            }
            stack.last_mut().unwrap().push(Sx::Str(t));
            i = j + 1;
        } else if c.is_whitespace() {
            i += 1;
        } else {
            let mut j = i;
            while j < cs.len() && !cs[j].is_whitespace() && cs[j] != '(' && cs[j] != ')' && cs[j] != '"' {
                j += 1;
            }
            stack.last_mut().unwrap().push(Sx::Atom(cs[i..j].iter().collect()));
            i = j;
        }
    }
    stack.into_iter().next().unwrap_or_default()
}

impl Sx {
    fn list(&self) -> &[Sx] {
        match self {
            Sx::List(l) => l,
            _ => &[],
        }
    }
    fn atom(&self) -> &str {
        match self {
            Sx::Atom(a) | Sx::Str(a) => a,
            _ => "",
        }
    }
    /// The value after `:KEY` in a property list.
    fn get(&self, key: &str) -> Option<&Sx> {
        let l = self.list();
        l.iter().position(|x| matches!(x, Sx::Atom(a) if a == key)).and_then(|i| l.get(i + 1))
    }
}

/// The positions an argument can take in a noun phrase: `poss` (the city's), `nn` (a noun modifier), a preposition.
fn positions(x: &Sx, into: &mut Vec<String>) {
    for p in x.list() {
        let l = p.list();
        let Some(head) = l.first() else { continue };
        let v: Vec<String> = match head.atom() {
            "DET-POSS" => vec!["poss".into()],
            "N-N-MOD" => vec!["nn".into()],
            "PP" => p.get(":PVAL").map(|v| v.list().iter().map(|s| s.atom().to_lowercase()).collect()).unwrap_or_default(),
            _ => Vec::new(),
        };
        for s in v {
            if !into.contains(&s) {
                into.push(s);
            }
        }
    }
}

fn nombank(dir: &Path, sum: &mut Sum) -> Result<(String, usize, usize)> {
    let mut out = String::new();
    // NOMLEX-plus: the nominalisations of verbs
    let s = std::fs::read_to_string(dir.join("NOMLEX-plus-clean.1.0"))?;
    sum.add(&s);
    let mut nn = 0;
    for e in sexps(&s) {
        let l = e.list();
        let Some(kind) = l.first().map(Sx::atom) else { continue };
        if !matches!(kind, "NOM" | "NOMING" | "NOMLIKE") {
            continue;
        }
        let orth = e.get(":ORTH").map(Sx::atom).unwrap_or("").to_lowercase();
        let verb = e.get(":VERB").map(Sx::atom).unwrap_or("").to_lowercase();
        if orth.is_empty() || verb.is_empty() {
            continue;
        }
        let nomtype: Vec<String> = e.get(":NOM-TYPE").map(|t| t.list().iter().filter_map(|x| x.list().first().map(|a| a.atom().to_lowercase())).collect()).unwrap_or_default();
        let (mut subj, mut obj, mut iobj, mut pval, mut subcs) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        if let Some(v) = e.get(":VERB-SUBJ") {
            positions(v, &mut subj);
        }
        if let Some(sc) = e.get(":VERB-SUBC") {
            for c in sc.list() {
                let Some(name) = c.list().first().map(|a| a.atom().to_lowercase()) else { continue };
                if !subcs.contains(&name) {
                    subcs.push(name);
                }
                if let Some(v) = c.get(":SUBJECT") {
                    positions(v, &mut subj);
                }
                if let Some(v) = c.get(":OBJECT") {
                    positions(v, &mut obj);
                }
                if let Some(v) = c.get(":IND-OBJ") {
                    positions(v, &mut iobj);
                }
                if let Some(v) = c.get(":PVAL") {
                    for p in v.list() {
                        let p = p.atom().to_lowercase();
                        if !p.is_empty() && !pval.contains(&p) {
                            pval.push(p);
                        }
                    }
                }
            }
        }
        writeln!(out, "nomlex\t{orth}\t{verb}\t{}\t{}\t{}\t{}\t{}\t{}", kind.to_lowercase(), nomtype.join(","), subj.join(","), obj.join(","), iobj.join(","), pval.join(","))?;
        let _ = &subcs;
        nn += 1;
    }
    // NomBank's frames: the noun rolesets and their source verbs
    let mut nr = 0;
    for f in xml_files(&dir.join("frames"))? {
        let s = std::fs::read_to_string(&f)?;
        sum.add(&s);
        let mut lemma = String::new();
        let (mut id, mut name, mut source, mut vncls) = (String::new(), String::new(), String::new(), String::new());
        let mut args: Vec<String> = Vec::new();
        for ev in events(&s) {
            match ev {
                Ev::Open(t, a, _) => match t.as_str() {
                    "predicate" => lemma = attr(&a, "lemma").to_lowercase(),
                    "roleset" => {
                        id = attr(&a, "id").to_string();
                        name = cell(attr(&a, "name"));
                        source = attr(&a, "source").to_string();
                        vncls = attr(&a, "vncls").to_string();
                        args.clear();
                    }
                    "role" => args.push(format!("{}::{}", attr(&a, "n"), cell(attr(&a, "descr")))),
                    "vnrole" => {
                        if let Some(x) = args.last_mut() {
                            let th = attr(&a, "vntheta").to_lowercase();
                            if let Some((n, d)) = x.split_once("::") {
                                *x = format!("{n}:{th}:{d}");
                            }
                        }
                    }
                    _ => {}
                },
                Ev::Close(t) if t == "roleset" => {
                    writeln!(out, "noun\t{id}\t{lemma}\t{name}\t{source}\t{vncls}\t{}", args.join("|"))?;
                    nr += 1;
                }
                _ => {}
            }
        }
    }
    Ok((out, nn, nr))
}

/// Convert the three lexicons: VerbNet (permissive) into `out` (verbnet.tsv, built into the reader); PropBank (CC BY-SA)
/// and NomBank (research-only) into `test` (propbank.tsv, nombank.tsv — outside the repo, for tests and measurement
/// only: share-alike and research-only data never build MMM). A report of what was made.
pub fn convert(vn: &Path, pb: &Path, nb: &Path, out: &Path, test: &Path) -> Result<String> {
    std::fs::create_dir_all(test)?;
    let mut rep = String::new();
    let mut s1 = Sum::default();
    let (t, nc, nm, nf) = verbnet(vn, &mut s1)?;
    let head = format!(
        "# VerbNet 3.4 (github.com/cu-clear/verbnet, master at commit ae8e9cfd, archive md5 9a23657f9bd062d6d799d59aebe36468); \
         VerbNet 3.0 License: free to use, copy, modify and distribute, commercial use included, with the copyright kept — \
         Copyright 2009 by the Regents of the University of Colorado / Martha Palmer. Converted by `prag lexicon convert`, \
         input checksum {:016x}.\n# rows: class<TAB>id<TAB>parent<TAB>members<TAB>roles[selrestr]; frame<TAB>class<TAB>primary<TAB>syntax<TAB>semantics<TAB>example. \
         Syntax: Role[selrestr / synrestr] for an NP, V, {{prep|prep}} or {{+type}} for a PREP, 'word' for a LEX, ADJ, ADV.\n",
        s1.0
    );
    std::fs::write(out.join("verbnet.tsv"), format!("{head}{t}"))?;
    writeln!(rep, "verbnet.tsv: {nc} classes (with subclasses), {nm} members, {nf} frames, {} bytes", head.len() + t.len())?;
    let mut s2 = Sum::default();
    let (t, nr) = propbank(pb, &mut s2)?;
    let head = format!(
        "# PropBank frame files 3.1 (github.com/propbank/propbank-frames, tag v3.1, archive md5 0f0e84041be07956963b4bc2b60ea2af); \
         CC BY-SA 4.0 — TESTS ONLY (never built into the reader); this derived table is shared alike under CC BY-SA 4.0, with attribution to the PropBank project \
         (Palmer, Gildea, Kingsbury 2005; Pradhan et al. 2022). Converted by `prag lexicon convert`, input checksum {:016x}.\n\
         # rows: roleset<TAB>id<TAB>lemma<TAB>name<TAB>vn classes<TAB>args n:f:vntheta:preps:descr|…<TAB>verb aliases<TAB>noun aliases<TAB>adjective aliases \
         (preps: the words the examples' arguments start with)\n",
        s2.0
    );
    std::fs::write(test.join("propbank.tsv"), format!("{head}{t}"))?;
    writeln!(rep, "{}/propbank.tsv (tests only): {nr} rolesets, {} bytes", test.display(), head.len() + t.len())?;
    let mut s3 = Sum::default();
    let (t, nn, nr) = nombank(nb, &mut s3)?;
    let head = format!(
        "# NomBank 1.0 (nlp.cs.nyu.edu/meyers/NomBank.html; nltk_data nombank.1.0.zip, md5 57afdc46230ea33208e4e277de24765b): \
         NOMLEX-plus-clean and the noun frames; distributed by NYU with permission, free for research — commercial terms \
         not stated (TESTS ONLY, never built into the reader; see docs/pragmatics.md). Converted by `prag lexicon convert`, input checksum {:016x}.\n\
         # rows: nomlex<TAB>noun<TAB>verb<TAB>class<TAB>nom-type<TAB>subject positions<TAB>object positions<TAB>indirect object positions<TAB>other preps \
         (poss = a possessive, nn = a noun modifier); noun<TAB>roleset<TAB>noun<TAB>name<TAB>source verb roleset<TAB>vn class<TAB>args n:vntheta:descr|…\n",
        s3.0
    );
    std::fs::write(test.join("nombank.tsv"), format!("{head}{t}"))?;
    writeln!(rep, "{}/nombank.tsv (tests only): {nn} NOMLEX nominalisations of verbs, {nr} NomBank noun rolesets, {} bytes", test.display(), head.len() + t.len())?;
    Ok(rep)
}

// ---- WordNet's derivational links: the nouns that name a verb's event ------------------------------------------

/// The lexicographer files of nouns that name an act, an event or a state (WordNet's numbers): the first sense of
/// the noun must be one of them («arrival» noun.act, not «building» noun.artifact, not «teacher» noun.person).
const EVENT_FILES: [(u32, &str); 9] = [(4, "act"), (9, "cognition"), (10, "communication"), (11, "event"), (12, "feeling"), (19, "phenomenon"), (21, "possession"), (22, "process"), (26, "state")];

/// WordNet's verb frames with an object (verb.Framestext: «Somebody ----s something», «… somebody PP» …).
const TRANSITIVE_FRAMES: [u32; 17] = [5, 8, 9, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 24, 25, 30, 31];

/// A synset of data.noun / data.verb: lexicographer file, words, pointers (symbol, offset, pos, source, target),
/// verb frames (frame, word number; 0 = every word).
struct Syn {
    lexfile: u32,
    words: Vec<String>,
    ptrs: Vec<(String, u64, char, usize, usize)>,
    frames: Vec<(u32, usize)>,
}

fn wn_data(s: &str) -> std::collections::BTreeMap<u64, Syn> {
    let mut m = std::collections::BTreeMap::new();
    for l in s.lines().filter(|l| !l.starts_with(' ')) {
        let body = l.split(" | ").next().unwrap_or(l);
        let t: Vec<&str> = body.split_whitespace().collect();
        if t.len() < 4 {
            continue;
        }
        let Ok(off) = t[0].parse::<u64>() else { continue };
        let lexfile = t[1].parse().unwrap_or(0);
        let Ok(wc) = usize::from_str_radix(t[3], 16) else { continue };
        let mut i = 4;
        let mut words = Vec::new();
        for _ in 0..wc {
            // adjectives carry a marker «(a)»; nouns and verbs do not
            words.push(t.get(i).map(|w| w.split('(').next().unwrap_or(w).to_lowercase()).unwrap_or_default());
            i += 2;
        }
        let pc: usize = t.get(i).and_then(|x| x.parse().ok()).unwrap_or(0);
        i += 1;
        let mut ptrs = Vec::new();
        for _ in 0..pc {
            if i + 3 >= t.len() {
                break;
            }
            let st = t[i + 3];
            let (src, tgt) = (usize::from_str_radix(&st[..2.min(st.len())], 16).unwrap_or(0), usize::from_str_radix(st.get(2..4).unwrap_or("0"), 16).unwrap_or(0));
            ptrs.push((t[i].to_string(), t[i + 1].parse().unwrap_or(0), t[i + 2].chars().next().unwrap_or('?'), src, tgt));
            i += 4;
        }
        let mut frames = Vec::new();
        if t.get(2) == Some(&"v") {
            let fc: usize = t.get(i).and_then(|x| x.parse().ok()).unwrap_or(0);
            i += 1;
            for _ in 0..fc {
                if i + 2 >= t.len() {
                    break;
                }
                let f = t.get(i + 1).and_then(|x| x.parse().ok()).unwrap_or(0);
                let w = t.get(i + 2).and_then(|x| usize::from_str_radix(x, 16).ok()).unwrap_or(0);
                frames.push((f, w));
                i += 3;
            }
        }
        m.insert(off, Syn { lexfile, words, ptrs, frames });
    }
    m
}

fn common_prefix(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count()
}

/// The noun names the verb's event, not a thing the verb is formed from: «arrival», «destruction», «growth», «walk»
/// yes; «memory» → memorize, «germ» → germinate, «fright» → frighten, «idea» → ideate no (the verb is the noun with a
/// verb-making suffix). A stricter variant that also took a same-word noun («name», «light») only in act / event /
/// process / feeling senses lost 14 of the exam answers it gains (docs/pragmatics.md), so same-word nouns stay.
fn deverbal(noun: &str, verb: &str) -> bool {
    let cp = common_prefix(noun, verb);
    let verbalised = verb.len() >= noun.len() && verb != noun && ["ize", "ise", "ify", "fy", "ate", "en"].iter().any(|s| verb.ends_with(s)) && cp >= 3 && cp > noun.len() / 2;
    !verbalised
}

/// `prag lexicon nominals <wordnet dict dir> <out dir>`: Princeton WordNet 3.1 (WordNet 3.0 License, BSD-like) →
/// `nominals.tsv`: every single-word noun whose first sense (WordNet's frequency order) lies in an event file (act,
/// event, process, state, communication, cognition, feeling, phenomenon, possession — not artifact, person, object …)
/// → the verb of the first such sense with a derivational link from this word (the most similar spelling when several,
/// `deverbal` holding), and whether the linked verb sense takes an object (its frames). A report of what was made.
pub fn nominals(dict: &Path, out: &Path) -> Result<String> {
    let mut sum = Sum::default();
    let rd = |f: &str| std::fs::read_to_string(dict.join(f)).with_context(|| format!("{}", dict.join(f).display()));
    let (idx, dn, dv) = (rd("index.noun")?, rd("data.noun")?, rd("data.verb")?);
    for s in [&idx, &dn, &dv] {
        sum.add(s);
    }
    let (nouns, verbs) = (wn_data(&dn), wn_data(&dv));
    let mut t = String::new();
    let (mut n, mut nt) = (0usize, 0usize);
    let mut by_file: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    for l in idx.lines().filter(|l| !l.starts_with(' ')) {
        let c: Vec<&str> = l.split_whitespace().collect();
        if c.len() < 4 || c[0].contains(['_', '-', '.', '\'']) || c[0].chars().any(|x| x.is_ascii_digit()) {
            continue;
        }
        let lemma = c[0];
        let pcnt: usize = c[3].parse().unwrap_or(0);
        // lemma pos synset_cnt p_cnt [ptr_symbol…] sense_cnt tagsense_cnt synset_offset…
        let offs: Vec<u64> = c.iter().skip(4 + pcnt + 2).filter_map(|x| x.parse::<u64>().ok()).collect();
        // the first sense names an act or event; the verb comes from the first sense of an event file linked from this word
        let Some(file) = offs.first().and_then(|o| nouns.get(o)).and_then(|s| EVENT_FILES.iter().find(|(k, _)| *k == s.lexfile)).map(|x| x.1) else { continue };
        let mut best: Option<(usize, String, u64, usize)> = None;
        for o in &offs {
            let Some(s) = nouns.get(o) else { continue };
            if !EVENT_FILES.iter().any(|(k, _)| *k == s.lexfile) {
                continue;
            }
            let Some(w) = s.words.iter().position(|x| x == lemma).map(|p| p + 1) else { continue };
            for (sym, off, pos, src, tgt) in &s.ptrs {
                if sym != "+" || *pos != 'v' || *src != w && *src != 0 {
                    continue;
                }
                let Some(v) = verbs.get(off) else { continue };
                let Some(verb) = v.words.get(tgt.saturating_sub(1)) else { continue };
                if verb.contains('_') || !deverbal(lemma, verb) {
                    continue;
                }
                let cp = common_prefix(lemma, verb);
                if best.as_ref().is_none_or(|b| cp > b.0) {
                    best = Some((cp, verb.clone(), *off, *tgt));
                }
            }
            if best.is_some() {
                break;
            }
        }
        let Some((_, verb, off, tgt)) = best else { continue };
        let v = &verbs[&off];
        let fr: Vec<u32> = v.frames.iter().filter(|(_, k)| *k == 0 || *k == tgt).map(|(f, _)| *f).collect();
        let tr = fr.iter().any(|f| TRANSITIVE_FRAMES.contains(f));
        let mut fs = fr.clone();
        fs.sort_unstable();
        fs.dedup();
        writeln!(t, "{lemma}\t{verb}\t{file}\t{}\t{}\t{off:08}", if tr { "t" } else { "i" }, fs.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","))?;
        n += 1;
        nt += tr as usize;
        *by_file.entry(file).or_default() += 1;
    }
    let head = format!(
        "# Princeton WordNet 3.1 (wordnetcode.princeton.edu/wn3.1.dict.tar.gz), WordNet 3.0 License: permission to use, copy, \
         modify and distribute for any purpose and without fee, with the notice kept — WordNet 3.1 Copyright 2011 by Princeton \
         University. All rights reserved. Converted by `prag lexicon nominals`, input checksum {:016x}.\n\
         # rows: noun<TAB>verb (a derivational link of the noun's first sense)<TAB>the noun's lexicographer file<TAB>t = the linked \
         verb sense takes an object, i = it does not<TAB>its WordNet frames<TAB>the verb synset\n",
        sum.0
    );
    std::fs::write(out.join("nominals.tsv"), format!("{head}{t}"))?;
    Ok(format!(
        "nominals.tsv: {n} nouns naming a verb's event ({nt} transitive), by file: {}, {} bytes\n",
        by_file.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "),
        head.len() + t.len()
    ))
}
