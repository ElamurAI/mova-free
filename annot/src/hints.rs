//! Expert-system hints for patch mode: violations of seed rules (`en/seeds`) on the Rust draft.
//! Design note: an expert system that tests the annotator's output and helps it. The rules are
//! precise but only see the ungrammatical, so these are hints, not a verdict: Opus decides. On Rust
//! drafts (300 sentences of EWT test, 26.09): error — 52 of 58 genuine, warn — 70 of 73; together
//! they cover ~12% of the wrong tokens.

use std::path::Path;

use anyhow::{Result, bail};
use en::conllu::{Sentence, Token};
use en::expert::Rule;

use crate::ann::Ann;

pub struct Hints {
    rules: Vec<Rule>,
}

impl Hints {
    /// Rules from the seeds folder: only `error`; with `warn` — warnings too. An unparsed rule is a
    /// stop, not a silent skip.
    pub fn load(dir: &Path, warn: bool) -> Result<Hints> {
        let (rules, errs) = en::expert::load(dir);
        if let Some(e) = errs.first() {
            bail!("seed rules failed to parse ({}), first: {e}", errs.len());
        }
        Ok(Hints::from_rules(rules, warn))
    }

    pub fn from_rules(rules: Vec<Rule>, warn: bool) -> Hints {
        Hints { rules: rules.into_iter().filter(|r| warn || r.severity == "error").collect() }
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// Hint lines for a sentence annotation: "tokens 3 «is», 5 «dogs»: <what the rule says> [id]".
    pub fn for_ann(&self, id: &str, text: &str, a: &Ann) -> Vec<String> {
        let s = Sentence {
            id: id.to_string(),
            text: text.to_string(),
            tokens: a
                .iter()
                .map(|t| Token { form: t.form.clone(), lemma: t.lemma.clone(), tag: Some(t.tag), upos: Some(t.upos), feats: t.feats, head: t.head, rel: t.rel, space_after: true })
                .collect(),
            has_feats: true,
            has_lemmas: true,
            has_forms: true,
        };
        let mut out = Vec::new();
        for r in &self.rules {
            for b in en::expert::check(r, &s).1 {
                let toks: Vec<String> = b.iter().map(|&i| format!("{i} «{}»", a[i - 1].form)).collect();
                out.push(format!("tokens {}: {} [{}]", toks.join(", "), r.what, r.id));
            }
        }
        out.sort();
        out.dedup();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ann::Tok;
    use en::gram::{Feats, Rel, Tag, UPos};

    const SEED: &str = "# t\n\n```rule\nrule: t.vbz-sing\nwhat: VBZ — singular\nmatch: v[xpos=VBZ]\nrequire: v[feats.Number=Sing]\nseverity: error\nsource: test\n```\n\n```rule\nrule: t.warn-only\nwhat: warning\nmatch: v[xpos=VBZ]\nrequire: not v[xpos=VBZ]\nseverity: warn\nsource: test\n```\n";

    fn tok(form: &str, upos: UPos, tag: Tag, feats: &str, head: usize, rel: Rel) -> Tok {
        Tok { form: form.into(), lemma: form.to_lowercase(), upos, tag, feats: Feats::parse(feats).0, head, rel }
    }

    #[test]
    fn hints_fire_only_on_violations() {
        let (rules, errs) = en::expert::parse_file(SEED, "t.md");
        assert!(errs.is_empty());
        let h = Hints::from_rules(rules, false);
        assert_eq!(h.len(), 1, "warn rules are not taken without the flag");
        let good = vec![tok("It", UPos::PRON, Tag::PRP, "Case=Nom|Number=Sing|Person=3|PronType=Prs", 2, Rel::Nsubj), tok("rains", UPos::VERB, Tag::VBZ, "Mood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin", 0, Rel::Root)];
        // negative control: a correct annotation gets no hints
        assert!(h.for_ann("s", "It rains", &good).is_empty());
        let mut bad = good.clone();
        bad[1].feats = Feats::parse("Mood=Ind|Number=Plur|Person=3|Tense=Pres|VerbForm=Fin").0;
        let v = h.for_ann("s", "It rains", &bad);
        assert_eq!(v, vec!["tokens 2 «rains»: VBZ — singular [t.vbz-sing]".to_string()]);
    }
}
