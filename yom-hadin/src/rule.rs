//! A rule as data: its deontic force, the act, the conditions, the exceptions, the penalty and the authority, with
//! the address of the text it was read from. One rule per line of a jurisdiction's `rules.tsv`.

use anyhow::{Context, Result, bail};
use std::fmt::Write as _;
use std::path::Path;

/// What a rule does to the act.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Force {
    /// «thou shalt», «one must», «is obligated»
    Obligation,
    /// «thou shalt not», «may not», «is forbidden»
    Prohibition,
    /// «may», «is permitted»
    Permission,
    /// «shall pay», «shall be put to death», «is liable», «is guilty»
    Liability,
    /// «is exempt», «there shall be no bloodguiltiness for him», «no claim can be made»
    Exemption,
}

impl Force {
    pub const ALL: [Force; 5] = [Force::Obligation, Force::Prohibition, Force::Permission, Force::Liability, Force::Exemption];
    pub fn name(self) -> &'static str {
        match self {
            Force::Obligation => "obligation",
            Force::Prohibition => "prohibition",
            Force::Permission => "permission",
            Force::Liability => "liability",
            Force::Exemption => "exemption",
        }
    }
    pub fn parse(s: &str) -> Option<Force> {
        Force::ALL.into_iter().find(|f| f.name() == s)
    }
}

/// One clause of a rule, read from a frame: the verb (lemma, with its particle), negation, the patient's head lemma
/// and the other role fillers as `key:lemma` (`from:corner`, `when:night`, `place:field`).
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Clause {
    pub verb: String,
    pub neg: bool,
    pub patient: Option<String>,
    pub args: Vec<String>,
}

impl Clause {
    /// `[!]verb/patient/arg,arg` — `-` for an empty patient.
    pub fn enc(&self) -> String {
        let mut s = String::new();
        if self.neg {
            s.push('!');
        }
        s += &self.verb.replace(' ', "_");
        s.push('/');
        s += self.patient.as_deref().unwrap_or("-");
        if !self.args.is_empty() {
            s.push('/');
            s += &self.args.join(",");
        }
        s
    }
    pub fn dec(s: &str) -> Result<Clause> {
        let (neg, s) = match s.strip_prefix('!') {
            Some(r) => (true, r),
            None => (false, s),
        };
        let mut it = s.split('/');
        let verb = it.next().filter(|v| !v.is_empty()).with_context(|| format!("clause without a verb: {s}"))?.replace('_', " ");
        let patient = it.next().filter(|p| *p != "-" && !p.is_empty()).map(str::to_string);
        let args = it.next().map(|a| a.split(',').filter(|x| !x.is_empty()).map(str::to_string).collect()).unwrap_or_default();
        Ok(Clause { verb, neg, patient, args })
    }
    /// For showing: «steal ox», «not pay it», «take sheaf from:corner».
    pub fn show(&self) -> String {
        let mut s = String::new();
        if self.neg {
            s += "not ";
        }
        s += &self.verb;
        if let Some(p) = &self.patient {
            s += " ";
            s += p;
        }
        for a in &self.args {
            s += " ";
            s += a;
        }
        s
    }
}

/// A rule. `conds` is a conjunction of alternatives (`a|b;c` = (a or b) and c); `excs` is a disjunction of
/// conjunctions (`a+b;c` = (a and b) or c: any exception that holds sets the rule aside).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    pub id: String,
    pub jur: String,
    /// the address of the text: `EXO.22.3`, `Peah.1.1.2`, `CC-UA.185.1`
    pub source: String,
    /// the sentence it was read from (pool id or the text file's address)
    pub sent: String,
    pub authority: String,
    pub force: Force,
    /// the sanction as read («pay five ox for ox», «put to death»), empty when none is said
    pub penalty: String,
    pub acts: Vec<Clause>,
    pub conds: Vec<Vec<Clause>>,
    pub excs: Vec<Vec<Clause>>,
    /// `agent=man`, `recipient=poor`
    pub parties: Vec<String>,
    /// the dispute this rule is one side of (`dispute:Peah.1.1:obligate`), empty when none
    pub dispute: String,
    /// where its tree came from: `db` (the agreed parse database), `slm` (the en parser now), `hand` (a written rule)
    pub tree: String,
    /// the rule's own stated reason, when the source gives one («for: ye be stranger», «that: ox have rest»)
    pub reason: String,
    pub text: String,
}

pub const HEADER: &str = "id\tjur\tsource\tsent\tauthority\tforce\tpenalty\tacts\tconds\texcs\tparties\tdispute\ttree\treason\ttext";

fn dash(s: &str) -> &str {
    if s.is_empty() { "-" } else { s }
}
fn undash(s: &str) -> String {
    if s == "-" { String::new() } else { s.to_string() }
}
fn clean(s: &str) -> String {
    s.replace(['\t', '\n', '\r'], " ")
}

impl Rule {
    /// The verb the rule is about: its first act, else its first condition's first clause.
    pub fn act_verb(&self) -> Option<&str> {
        self.acts.first().or_else(|| self.conds.first().and_then(|g| g.first())).map(|c| c.verb.as_str())
    }

    pub fn to_line(&self) -> String {
        let acts: Vec<String> = self.acts.iter().map(Clause::enc).collect();
        let conds: Vec<String> = self.conds.iter().map(|g| g.iter().map(Clause::enc).collect::<Vec<_>>().join("|")).collect();
        let excs: Vec<String> = self.excs.iter().map(|g| g.iter().map(Clause::enc).collect::<Vec<_>>().join("+")).collect();
        let mut s = String::new();
        let _ = write!(
            s,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            self.jur,
            self.source,
            dash(&self.sent),
            dash(&clean(&self.authority)),
            self.force.name(),
            dash(&clean(&self.penalty)),
            dash(&acts.join(";")),
            dash(&conds.join(";")),
            dash(&excs.join(";")),
            dash(&self.parties.join(",")),
            dash(&self.dispute),
            dash(&self.tree),
            dash(&clean(&self.reason)),
            clean(&self.text)
        );
        s
    }

    pub fn from_line(l: &str) -> Result<Rule> {
        let c: Vec<&str> = l.split('\t').collect();
        // 14 columns: a file written before the reason column (read with no reason)
        let c: Vec<&str> = match c.len() {
            15 => c,
            14 => c[..13].iter().copied().chain(["-", c[13]]).collect(),
            n => bail!("a rule line needs 15 columns, has {n}: {l}"),
        };
        let clauses = |s: &str, sep: char| -> Result<Vec<Vec<Clause>>> {
            if s == "-" {
                return Ok(Vec::new());
            }
            s.split(';').map(|g| g.split(sep).map(Clause::dec).collect()).collect()
        };
        Ok(Rule {
            id: c[0].to_string(),
            jur: c[1].to_string(),
            source: c[2].to_string(),
            sent: undash(c[3]),
            authority: undash(c[4]),
            force: Force::parse(c[5]).with_context(|| format!("unknown force {}", c[5]))?,
            penalty: undash(c[6]),
            acts: clauses(c[7], '|')?.into_iter().flatten().collect(),
            conds: clauses(c[8], '|')?,
            excs: clauses(c[9], '+')?,
            parties: if c[10] == "-" { Vec::new() } else { c[10].split(',').map(str::to_string).collect() },
            dispute: undash(c[11]),
            tree: undash(c[12]),
            reason: undash(c[13]),
            text: c[14].to_string(),
        })
    }
}

pub fn write_rules(path: &Path, rules: &[Rule]) -> Result<()> {
    let mut s = String::from(HEADER);
    s.push('\n');
    for r in rules {
        s += &r.to_line();
        s.push('\n');
    }
    std::fs::write(path, s).with_context(|| format!("{}", path.display()))
}

pub fn read_rules(path: &Path) -> Result<Vec<Rule>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    text.lines().filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with("id\t")).map(Rule::from_line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_round_trip() {
        let r = Rule {
            id: "t:1".into(),
            jur: "torah".into(),
            source: "EXO.21.37".into(),
            sent: "jps:EXO:620".into(),
            authority: "Torah".into(),
            force: Force::Liability,
            penalty: "pay five ox for ox".into(),
            acts: vec![],
            conds: vec![vec![Clause { verb: "steal".into(), neg: false, patient: Some("ox".into()), args: vec![] }], vec![Clause { verb: "kill".into(), ..Default::default() }, Clause { verb: "sell".into(), ..Default::default() }]],
            excs: vec![vec![Clause { verb: "find".into(), neg: true, patient: None, args: vec!["when:night".into()] }]],
            parties: vec!["agent=man".into()],
            dispute: String::new(),
            tree: "db".into(),
            reason: "for: ye be stranger".into(),
            text: "If a man steal an ox …".into(),
        };
        let back = Rule::from_line(&r.to_line()).unwrap();
        assert_eq!(back, r);
    }
}
