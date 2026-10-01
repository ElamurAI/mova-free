//! Closed sets: FairytaleQA question types, answer explicitness and the run rule. Names are verbatim as in FairytaleQA
//! (`attribute`, `ex-or-im`) and in the CHECK of table `qa` (migration v3), so comparison with the reference needs no
//! translation of names.

/// Question type — a FairytaleQA narrative element or relation (Xu et al., ACL 2022).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum QType {
    Character,
    Setting,
    Action,
    Feeling,
    Causal,
    Outcome,
    Prediction,
}

impl QType {
    pub const ALL: [QType; 7] = [QType::Character, QType::Setting, QType::Action, QType::Feeling, QType::Causal, QType::Outcome, QType::Prediction];

    pub fn name(self) -> &'static str {
        match self {
            QType::Character => "character",
            QType::Setting => "setting",
            QType::Action => "action",
            QType::Feeling => "feeling",
            QType::Causal => "causal relationship",
            QType::Outcome => "outcome resolution",
            QType::Prediction => "prediction",
        }
    }

    /// Only full names of the set; case and surrounding whitespace do not matter.
    pub fn parse(s: &str) -> Option<QType> {
        let s = s.trim().to_lowercase();
        QType::ALL.into_iter().find(|t| t.name() == s)
    }
}

/// Explicitness: the answer is stated in the text or is inferred.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ExIm {
    Explicit,
    Implicit,
}

impl ExIm {
    pub const ALL: [ExIm; 2] = [ExIm::Explicit, ExIm::Implicit];

    pub fn name(self) -> &'static str {
        match self {
            ExIm::Explicit => "explicit",
            ExIm::Implicit => "implicit",
        }
    }

    pub fn parse(s: &str) -> Option<ExIm> {
        let s = s.trim().to_lowercase();
        ExIm::ALL.into_iter().find(|t| t.name() == s)
    }
}

/// Run rule. V1 — the 26.09 pilot: an explicit answer may paraphrase the text. V2 — an explicit
/// answer is a verbatim span of a single anchor sentence, as in FairytaleQA; an implicit one is free text.
/// The number is stored in the run's `pick.json` (absent — V1), so the pilot is reproduced with its own rule.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Rule {
    V1,
    V2,
}

impl Rule {
    pub fn of(n: u8) -> Option<Rule> {
        match n {
            1 => Some(Rule::V1),
            2 => Some(Rule::V2),
            _ => None,
        }
    }

    pub fn num(self) -> u8 {
        match self {
            Rule::V1 => 1,
            Rule::V2 => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_round_trip() {
        assert_eq!(Rule::of(1), Some(Rule::V1));
        assert_eq!(Rule::of(Rule::V2.num()), Some(Rule::V2));
        assert_eq!(Rule::of(0), None);
        assert_eq!(Rule::of(3), None);
    }

    #[test]
    fn closed_sets_round_trip() {
        for t in QType::ALL {
            assert_eq!(QType::parse(t.name()), Some(t));
        }
        assert_eq!(QType::parse(" Causal Relationship "), Some(QType::Causal));
        // negative controls: abbreviations and foreign names are outside the set
        assert_eq!(QType::parse("causal"), None);
        assert_eq!(QType::parse("outcome"), None);
        assert_eq!(QType::parse("who"), None);
        assert_eq!(ExIm::parse("implicit"), Some(ExIm::Implicit));
        assert_eq!(ExIm::parse("inferred"), None);
    }
}
