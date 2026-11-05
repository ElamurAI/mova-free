//! Jurisdictions as data: a folder per jurisdiction under `jurisdictions/`, loaded without a code change.
//!
//! ```text
//! jurisdiction.tsv   key<TAB>value…  id, name, place, era, body, status (filled|sample|slot), places, persons,
//!                    principles (the precedence between conflicting rules, the plain one first), inherits, suspend,
//!                    source (where `extract` reads the rules from), note
//! rules.tsv          the rules (rule::HEADER)
//! terms.tsv          head, narrow, broad, plain (narrow|broad), opposite, note: what a word of the law covers there
//! penalties.tsv      pattern, rank (0–7 on the shared scale), gloss: how heavy each sanction is
//! authorities.tsv    pattern, class, rank: who speaks, and whose word prevails (lower rank wins)
//! ```
//! `_common/terms.tsv` and `_common/penalties.tsv` are read first; a jurisdiction's own rows override them by head
//! or pattern.

use anyhow::{Context, Result, bail};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::rule::{Rule, read_rules};

/// How two conflicting rules are settled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Principle {
    /// lex specialis: the rule with more conditions and exceptions wins
    Specific,
    /// the authority with the lower rank wins (Torah over Mishnah over a named sage)
    Rank,
    /// lex posterior: the rule later in the body of law wins
    Later,
    /// the harshest verdict wins
    Strict,
    /// the mildest verdict wins (in dubio pro reo; the rule of lenity)
    Lenient,
}

impl Principle {
    pub const ALL: [Principle; 5] = [Principle::Specific, Principle::Rank, Principle::Later, Principle::Strict, Principle::Lenient];
    pub fn name(self) -> &'static str {
        match self {
            Principle::Specific => "specific",
            Principle::Rank => "rank",
            Principle::Later => "later",
            Principle::Strict => "strict",
            Principle::Lenient => "lenient",
        }
    }
    pub fn parse(s: &str) -> Option<Principle> {
        Principle::ALL.into_iter().find(|p| p.name() == s)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Term {
    pub head: String,
    pub narrow: Vec<String>,
    pub broad: Vec<String>,
    /// the plain reading of the term in this jurisdiction: broad (true) or narrow
    pub plain_broad: bool,
    pub opposite: Vec<String>,
    pub note: String,
}

#[derive(Clone, Debug)]
pub struct Penalty {
    pub pattern: String,
    pub rank: u8,
    pub gloss: String,
}

/// A word that flags a rule (status) or a penalty (one the human-rights instruments forbid): `_common/flags.tsv`.
#[derive(Clone, Debug)]
pub struct Flag {
    pub pattern: String,
    pub kind: String,
    /// `text` (the rule's sentence) or `penalty`
    pub field: String,
    pub reason: String,
}

/// A legal device (ha'aramah): it changes the classification of a place, a thing, a transaction or an agent without
/// changing the facts (`devices.tsv`).
#[derive(Clone, Debug)]
pub struct Device {
    pub id: String,
    pub name: String,
    /// words that must all be in the case for the device to be there
    pub trigger: Vec<String>,
    /// place, thing, transaction or agent
    pub field: String,
    /// the words as the facts are
    pub substance: Vec<String>,
    /// what the device makes of them
    pub form: Vec<String>,
    pub source: String,
    pub note: String,
}

#[derive(Clone, Debug)]
pub struct Authority {
    pub pattern: String,
    pub class: String,
    pub rank: u8,
}

/// How strongly a word of a case is covered by a word of a rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cover {
    No,
    /// only on the broad reading: a choice point
    Broad,
    /// the narrow reading covers it (or the same lemma)
    Sure,
}

#[derive(Clone, Debug, Default)]
pub struct Jurisdiction {
    pub id: String,
    pub dir: PathBuf,
    pub meta: BTreeMap<String, Vec<Vec<String>>>,
    pub places: Vec<String>,
    pub persons: Vec<String>,
    pub principles: Vec<Principle>,
    /// verdicts up to this severity may be suspended (probation), with the provision that allows it
    pub suspend: Option<(u8, String)>,
    pub terms: Vec<Term>,
    pub penalties: Vec<Penalty>,
    pub authorities: Vec<Authority>,
    pub flags: Vec<Flag>,
    pub devices: Vec<Device>,
    pub rules: Vec<Rule>,
}

fn tsv(path: &Path) -> Result<Vec<Vec<String>>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    Ok(text.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).map(|l| l.split('\t').map(|c| c.trim().to_string()).collect()).collect())
}

fn list(s: Option<&String>) -> Vec<String> {
    s.map(|s| s.split(',').map(|x| x.trim().to_lowercase()).filter(|x| !x.is_empty() && x != "-").collect()).unwrap_or_default()
}

fn read_terms(path: &Path) -> Result<Vec<Term>> {
    let mut out = Vec::new();
    for c in tsv(path)? {
        if c.first().is_some_and(|h| h == "head") {
            continue;
        }
        let plain = c.get(3).map(String::as_str).unwrap_or("narrow");
        if !matches!(plain, "narrow" | "broad") {
            bail!("{}: plain must be narrow or broad: {:?}", path.display(), c);
        }
        out.push(Term { head: c[0].to_lowercase(), narrow: list(c.get(1)), broad: list(c.get(2)), plain_broad: plain == "broad", opposite: list(c.get(4)), note: c.get(5).cloned().unwrap_or_default() });
    }
    Ok(out)
}

fn read_penalties(path: &Path) -> Result<Vec<Penalty>> {
    let mut out = Vec::new();
    for c in tsv(path)? {
        if c.first().is_some_and(|h| h == "pattern") {
            continue;
        }
        let rank: u8 = c.get(1).and_then(|r| r.parse().ok()).with_context(|| format!("{}: bad rank {:?}", path.display(), c))?;
        out.push(Penalty { pattern: c[0].to_lowercase(), rank, gloss: c.get(2).cloned().unwrap_or_default() });
    }
    Ok(out)
}

fn read_devices(path: &Path) -> Result<Vec<Device>> {
    let mut out = Vec::new();
    for c in tsv(path)? {
        if c.first().is_some_and(|h| h == "id") {
            continue;
        }
        if c.len() < 6 {
            bail!("{}: a device needs id, name, trigger, field, substance, form: {:?}", path.display(), c);
        }
        if !matches!(c[3].as_str(), "place" | "thing" | "transaction" | "agent") {
            bail!("{}: unknown field {}", path.display(), c[3]);
        }
        out.push(Device { id: c[0].clone(), name: c[1].clone(), trigger: list(c.get(2)), field: c[3].clone(), substance: list(c.get(4)), form: list(c.get(5)), source: c.get(6).cloned().unwrap_or_default(), note: c.get(7).cloned().unwrap_or_default() });
    }
    Ok(out)
}

fn read_flags(path: &Path) -> Result<Vec<Flag>> {
    Ok(tsv(path)?.into_iter().filter(|c| c.first().is_some_and(|h| h != "pattern")).map(|c| Flag { pattern: c[0].to_lowercase(), kind: c.get(1).cloned().unwrap_or_default(), field: c.get(2).cloned().unwrap_or_else(|| "text".into()), reason: c.get(3).cloned().unwrap_or_default() }).collect())
}

/// Words of a text, lower case, as one string with single spaces around each («a b c» → « a b c »).
fn words(t: &str) -> String {
    let w: Vec<String> = t.split(|c: char| !c.is_alphanumeric() && c != '-').filter(|x| !x.is_empty()).map(|x| x.to_lowercase()).collect();
    format!(" {} ", w.join(" "))
}

fn read_authorities(path: &Path) -> Result<Vec<Authority>> {
    let mut out = Vec::new();
    for c in tsv(path)? {
        if c.first().is_some_and(|h| h == "pattern") {
            continue;
        }
        let rank: u8 = c.get(2).and_then(|r| r.parse().ok()).with_context(|| format!("{}: bad rank {:?}", path.display(), c))?;
        out.push(Authority { pattern: c[0].to_lowercase(), class: c.get(1).cloned().unwrap_or_default(), rank });
    }
    Ok(out)
}

/// The folder of the jurisdictions: `YOMHADIN_DIR`, else the crate's own `jurisdictions/`.
pub fn root() -> PathBuf {
    std::env::var("YOMHADIN_DIR").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/jurisdictions")))
}

impl Jurisdiction {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.meta.get(key).and_then(|v| v.first()).and_then(|r| r.first())
    }
    pub fn name(&self) -> String {
        self.get("name").cloned().unwrap_or_else(|| self.id.clone())
    }
    pub fn status(&self) -> &str {
        self.get("status").map(String::as_str).unwrap_or("slot")
    }
    /// `modern` (law in force) or `historical`.
    pub fn kind(&self) -> &str {
        self.get("kind").map(String::as_str).unwrap_or("historical")
    }
    /// Whether the jurisdiction admits legal devices (`devices yes|no|moot`; yes when it lists any and says nothing).
    pub fn admits_devices(&self) -> bool {
        match self.get("devices").map(String::as_str) {
            Some("yes") => true,
            Some(_) => false,
            None => !self.devices.is_empty(),
        }
    }
    /// The provenance: (derived|hand|none, the description).
    pub fn provenance(&self) -> (String, String) {
        self.meta.get("provenance").and_then(|v| v.first()).map(|r| (r.first().cloned().unwrap_or_default(), r.get(1).cloned().unwrap_or_default())).unwrap_or_else(|| ("none".into(), String::new()))
    }
    /// Why a rule is flagged (empty when it is not): its words of status, its penalty.
    pub fn flags_of(&self, r: &Rule) -> Vec<String> {
        let (text, pen) = (words(&r.text), words(&r.penalty));
        let mut out: Vec<String> = Vec::new();
        for f in &self.flags {
            let hay = if f.field == "penalty" { &pen } else { &text };
            if hay.contains(&format!(" {} ", f.pattern)) {
                let w = format!("{}: {} («{}»)", f.kind, f.reason, f.pattern);
                if !out.contains(&w) {
                    out.push(w);
                }
            }
        }
        out
    }
    /// The same jurisdiction without its flagged rules (for the reference).
    pub fn unflagged(&self) -> Jurisdiction {
        let mut j = self.clone();
        j.rules.retain(|r| self.flags_of(r).is_empty());
        j
    }

    /// The `source` lines: (kind, spec).
    pub fn sources(&self) -> Vec<(String, String)> {
        self.meta.get("source").map(|v| v.iter().map(|r| (r.first().cloned().unwrap_or_default(), r.get(1).cloned().unwrap_or_default())).collect()).unwrap_or_default()
    }

    /// Load one folder (without the rules of the jurisdictions it inherits: `load_all` adds them).
    pub fn load(dir: &Path, common: &Path) -> Result<Jurisdiction> {
        let mut meta: BTreeMap<String, Vec<Vec<String>>> = BTreeMap::new();
        for c in tsv(&dir.join("jurisdiction.tsv"))? {
            if let Some((k, rest)) = c.split_first() {
                meta.entry(k.clone()).or_default().push(rest.to_vec());
            }
        }
        let id = meta.get("id").and_then(|v| v.first()).and_then(|r| r.first()).cloned().with_context(|| format!("{}: no id", dir.display()))?;
        let one = |k: &str| meta.get(k).and_then(|v| v.first()).and_then(|r| r.first()).cloned();
        let principles: Vec<Principle> = match one("principles") {
            Some(p) => p.split(',').map(|x| Principle::parse(x.trim()).with_context(|| format!("{}: unknown principle {x}", dir.display()))).collect::<Result<_>>()?,
            None => vec![Principle::Specific, Principle::Later],
        };
        let suspend = meta.get("suspend").and_then(|v| v.first()).and_then(|r| Some((r.first()?.parse().ok()?, r.get(1).cloned().unwrap_or_default())));
        // terms and penalties: the common rows, overridden by the jurisdiction's own
        let mut terms: BTreeMap<String, Term> = BTreeMap::new();
        let mut order: Vec<String> = Vec::new();
        for t in read_terms(&common.join("terms.tsv"))?.into_iter().chain(read_terms(&dir.join("terms.tsv"))?) {
            if !terms.contains_key(&t.head) {
                order.push(t.head.clone());
            }
            terms.insert(t.head.clone(), t);
        }
        let terms = order.into_iter().map(|h| terms.remove(&h).unwrap()).collect();
        let own = read_penalties(&dir.join("penalties.tsv"))?;
        let mut penalties: Vec<Penalty> = read_penalties(&common.join("penalties.tsv"))?.into_iter().filter(|p| !own.iter().any(|o| o.pattern == p.pattern)).collect();
        penalties.extend(own);
        let rules_path = dir.join("rules.tsv");
        let rules = if rules_path.exists() { read_rules(&rules_path)? } else { Vec::new() };
        Ok(Jurisdiction {
            places: list(one("places").as_ref()),
            persons: list(one("persons").as_ref()),
            id,
            dir: dir.to_path_buf(),
            principles,
            suspend,
            terms,
            penalties,
            authorities: read_authorities(&dir.join("authorities.tsv"))?,
            flags: read_flags(&common.join("flags.tsv"))?.into_iter().chain(read_flags(&dir.join("flags.tsv"))?).collect(),
            devices: read_devices(&dir.join("devices.tsv"))?,
            rules,
            meta,
        })
    }

    /// Does `rule_w` (a word of a rule) cover `case_w` (a word of a case) here?
    pub fn cover(&self, rule_w: &str, case_w: &str) -> Cover {
        if rule_w == case_w {
            return Cover::Sure;
        }
        let mut best = Cover::No;
        for t in &self.terms {
            let in_n = |w: &str| t.head == w || t.narrow.iter().any(|x| x == w);
            let in_b = |w: &str| in_n(w) || t.broad.iter().any(|x| x == w);
            if in_n(rule_w) && in_n(case_w) {
                return Cover::Sure;
            }
            if in_b(rule_w) && in_b(case_w) {
                best = Cover::Broad;
            }
        }
        best
    }

    /// The term that makes `rule_w` cover `case_w` broadly, for the why and for its plain reading.
    pub fn term_of(&self, rule_w: &str, case_w: &str) -> Option<&Term> {
        self.terms.iter().find(|t| {
            let in_b = |w: &str| t.head == w || t.narrow.iter().any(|x| x == w) || t.broad.iter().any(|x| x == w);
            in_b(rule_w) && in_b(case_w)
        })
    }

    /// Is `case_w` the opposite of `rule_w` here («night» against «rise» of the sun)?
    pub fn opposite(&self, rule_w: &str, case_w: &str) -> bool {
        self.terms.iter().any(|t| {
            let in_n = |w: &str| t.head == w || t.narrow.iter().any(|x| x == w);
            in_n(rule_w) && t.opposite.iter().any(|x| x == case_w)
        })
    }

    /// The severity of a sanction on the shared scale (0 free … 7 death): the heaviest pattern found in it.
    pub fn severity(&self, penalty: &str) -> Option<(u8, String)> {
        let p = penalty.to_lowercase();
        self.penalties.iter().filter(|x| p.contains(&x.pattern)).max_by_key(|x| x.rank).map(|x| (x.rank, x.gloss.clone()))
    }

    /// The class and rank of an authority (the first pattern found in its name); a name nobody listed is `named`, 5.
    pub fn authority(&self, who: &str) -> (String, u8) {
        let w = who.to_lowercase();
        self.authorities.iter().find(|a| w.contains(&a.pattern)).map(|a| (a.class.clone(), a.rank)).unwrap_or_else(|| ("named".into(), 5))
    }
}

/// Every jurisdiction under the root, sorted by id; `inherits` adds the rules of another (marked with its id).
pub fn load_all(root: &Path) -> Result<Vec<Jurisdiction>> {
    let common = root.join("_common");
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(root).with_context(|| format!("{}", root.display()))?.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.is_dir() && p.join("jurisdiction.tsv").exists()).collect();
    dirs.sort();
    let mut js: Vec<Jurisdiction> = dirs.iter().map(|d| Jurisdiction::load(d, &common)).collect::<Result<_>>()?;
    let own: BTreeMap<String, Vec<Rule>> = js.iter().map(|j| (j.id.clone(), j.rules.clone())).collect();
    for j in &mut js {
        if let Some(rows) = j.meta.get("inherits") {
            let mut inherited = Vec::new();
            for r in rows {
                let from = r.first().cloned().unwrap_or_default();
                let rules = own.get(&from).with_context(|| format!("{} inherits unknown {from}", j.id))?;
                inherited.extend(rules.iter().cloned());
            }
            inherited.extend(std::mem::take(&mut j.rules));
            j.rules = inherited;
        }
    }
    Ok(js)
}
