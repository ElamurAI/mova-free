//! First cut of the pragmatics schema (docs/context.md, "Pragmatics: what was meant", 26.09.2026) —
//! closed value sets as enums. A value outside the set is a parse error, not a silent string.

use serde::{Deserialize, Serialize};

/// Enum with a closed list of names: `ALL`, `name()`, `parse()`, `idx()`.
macro_rules! closed {
    ($(#[$m:meta])* $name:ident { $($v:ident = $s:literal),+ $(,)? }) => {
        $(#[$m])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
        pub enum $name { $($v),+ }

        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$v),+];
            pub fn name(self) -> &'static str {
                match self { $($name::$v => $s),+ }
            }
            pub fn parse(s: &str) -> Option<$name> {
                match s { $($s => Some($name::$v),)+ _ => None }
            }
            pub fn idx(self) -> usize {
                self as usize
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(self.name())
            }
        }
    };
}

closed! {
    /// Form (sentence type).
    Form { Decl = "decl", Q = "q", Wh = "wh", Imp = "imp", Excl = "excl", Frag = "frag", Intj = "intj", Other = "other" }
}

closed! {
    /// Speech act.
    Act {
        Assert = "assert", Narrate = "narrate", Ask = "ask", Request = "request", Offer = "offer", Suggest = "suggest",
        Promise = "promise", Thank = "thank", Apologize = "apologize", Greet = "greet", Agree = "agree",
        Disagree = "disagree", Evaluate = "evaluate", Express = "express", Warn = "warn", Other = "other",
    }
}

closed! {
    /// Speaker's evaluation.
    Polarity { Pos = "pos", Neg = "neg", None = "none" }
}

closed! {
    /// Who is speaking in the text.
    Voice { Narrator = "narrator", Character = "character", None = "none" }
}

/// Sentence labels per the first cut.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Labels {
    pub form: Form,
    pub act: Act,
    pub indirect: bool,
    pub hedge: bool,
    pub polarity: Polarity,
    pub voice: Voice,
    /// what was meant (written only by the LLM); "-" — nothing beyond the literal
    pub means: String,
}

/// A field the SLM learns (all except `means`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Field {
    Form,
    Act,
    Indirect,
    Hedge,
    Polarity,
    Voice,
}

impl Field {
    pub const ALL: [Field; 6] = [Field::Form, Field::Act, Field::Indirect, Field::Hedge, Field::Polarity, Field::Voice];

    pub fn name(self) -> &'static str {
        match self {
            Field::Form => "form",
            Field::Act => "act",
            Field::Indirect => "indirect",
            Field::Hedge => "hedge",
            Field::Polarity => "polarity",
            Field::Voice => "voice",
        }
    }

    /// Class names of the field (class number = position in the list).
    pub fn classes(self) -> Vec<&'static str> {
        match self {
            Field::Form => Form::ALL.iter().map(|x| x.name()).collect(),
            Field::Act => Act::ALL.iter().map(|x| x.name()).collect(),
            Field::Indirect | Field::Hedge => vec!["0", "1"],
            Field::Polarity => Polarity::ALL.iter().map(|x| x.name()).collect(),
            Field::Voice => Voice::ALL.iter().map(|x| x.name()).collect(),
        }
    }

    /// Class of the field in the labels.
    pub fn get(self, l: &Labels) -> usize {
        match self {
            Field::Form => l.form.idx(),
            Field::Act => l.act.idx(),
            Field::Indirect => l.indirect as usize,
            Field::Hedge => l.hedge as usize,
            Field::Polarity => l.polarity.idx(),
            Field::Voice => l.voice.idx(),
        }
    }

    /// Write the class into the labels. Panics before mutation for an invalid class.
    pub fn set(self, l: &mut Labels, c: usize) {
        if matches!(self, Field::Indirect | Field::Hedge) {
            assert!(c < 2, "Boolean class outside the set 0|1");
        }
        match self {
            Field::Form => l.form = Form::ALL[c],
            Field::Act => l.act = Act::ALL[c],
            Field::Indirect => l.indirect = c == 1,
            Field::Hedge => l.hedge = c == 1,
            Field::Polarity => l.polarity = Polarity::ALL[c],
            Field::Voice => l.voice = Voice::ALL[c],
        }
    }
}

impl Default for Labels {
    fn default() -> Labels {
        Labels { form: Form::Decl, act: Act::Assert, indirect: false, hedge: false, polarity: Polarity::None, voice: Voice::None, means: "-".into() }
    }
}

/// Maximum words in `means` (brief: "up to ~15 words"; the gate leaves headroom).
pub const MEANS_MAX_WORDS: usize = 30;

/// Parse the seven columns after the number: `form act indirect hedge polarity voice means`. An error carries
/// the reason, which goes to the rejects log.
pub fn parse_labels(cols: &[&str]) -> Result<Labels, String> {
    if cols.len() != 7 {
        return Err(format!("{} columns instead of 8", cols.len() + 1));
    }
    let bit = |s: &str, what: &str| match s.trim() {
        "0" => Ok(false),
        "1" => Ok(true),
        x => Err(format!("{what}={x:?} outside the set 0|1")),
    };
    let form = Form::parse(cols[0].trim()).ok_or_else(|| format!("form={:?} outside the set", cols[0]))?;
    let act = Act::parse(cols[1].trim()).ok_or_else(|| format!("act={:?} outside the set", cols[1]))?;
    let indirect = bit(cols[2], "indirect")?;
    let hedge = bit(cols[3], "hedge")?;
    let polarity = Polarity::parse(cols[4].trim()).ok_or_else(|| format!("polarity={:?} outside the set", cols[4]))?;
    let voice = Voice::parse(cols[5].trim()).ok_or_else(|| format!("voice={:?} outside the set", cols[5]))?;
    let means = cols[6].trim().to_string();
    if means.is_empty() {
        return Err("means is empty".into());
    }
    let words = means.split_whitespace().count();
    if words > MEANS_MAX_WORDS {
        return Err(format!("means: {words} words > {MEANS_MAX_WORDS}"));
    }
    Ok(Labels { form, act, indirect, hedge, polarity, voice, means })
}

/// Labels TSV row without the key: `form act indirect hedge polarity voice means`.
pub fn labels_tsv(l: &Labels) -> String {
    format!("{}\t{}\t{}\t{}\t{}\t{}\t{}", l.form, l.act, l.indirect as u8, l.hedge as u8, l.polarity, l.voice, l.means)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_sets_parse_and_reject() {
        let ok = parse_labels(&["q", "request", "1", "0", "none", "character", "asks Hans to open the window"]).unwrap();
        assert_eq!((ok.form, ok.act, ok.indirect, ok.voice), (Form::Q, Act::Request, true, Voice::Character));
        assert_eq!(labels_tsv(&ok), "q\trequest\t1\t0\tnone\tcharacter\tasks Hans to open the window");
        // negative controls: out-of-set value, extra or missing column, empty and too long means
        assert!(parse_labels(&["question", "request", "1", "0", "none", "character", "x"]).is_err());
        assert!(parse_labels(&["q", "threat", "1", "0", "none", "character", "x"]).is_err());
        assert!(parse_labels(&["q", "request", "yes", "0", "none", "character", "x"]).is_err());
        assert!(parse_labels(&["q", "request", "1", "0", "neutral", "character", "x"]).is_err());
        assert!(parse_labels(&["q", "request", "1", "0", "none", "speaker", "x"]).is_err());
        assert!(parse_labels(&["q", "request", "1", "0", "none", "character"]).is_err());
        assert!(parse_labels(&["q", "request", "1", "0", "none", "character", " "]).is_err());
        assert!(parse_labels(&["q", "request", "1", "0", "none", "character", &"w ".repeat(40)]).is_err());
        for f in Field::ALL {
            let mut l = Labels::default();
            let n = f.classes().len();
            f.set(&mut l, n - 1);
            assert_eq!(f.get(&l), n - 1, "{f:?}");
        }
    }
}

#[cfg(test)]
mod wave13_models {
    use super::*;

    const NAMES: [&[&str]; 6] = [
        &["decl", "q", "wh", "imp", "excl", "frag", "intj", "other"],
        &["assert", "narrate", "ask", "request", "offer", "suggest", "promise", "thank", "apologize", "greet", "agree", "disagree", "evaluate", "express", "warn", "other"],
        &["0", "1"], &["0", "1"], &["pos", "neg", "none"], &["narrator", "character", "none"],
    ];

    #[test]
    fn finite_class_tables_and_local_updates() {
        for (i, f) in Field::ALL.into_iter().enumerate() {
            assert_eq!(f.classes(), NAMES[i]);
            assert_eq!(f.name(), ["form", "act", "indirect", "hedge", "polarity", "voice"][i]);
            for c in 0..NAMES[i].len() {
                let mut l = Labels::default();
                l.means = "preserved provenance".into();
                let before = l.clone();
                f.set(&mut l, c);
                for other in Field::ALL {
                    assert_eq!(other.get(&l), if other == f { c } else { other.get(&before) });
                }
                assert_eq!(l.means, before.means);
                assert_eq!(labels_tsv(&l).split('\t').nth(i), Some(NAMES[i][c]));
            }
        }
    }

    #[test]
    fn invalid_class_panics_before_mutation() {
        for (i, f) in Field::ALL.into_iter().enumerate() {
            for c in [NAMES[i].len(), usize::MAX] {
                let mut l = Labels::default();
                l.indirect = true;
                l.hedge = true;
                let before = l.clone();
                let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f.set(&mut l, c)));
                assert!(rejected.is_err(), "invalid class must reject: {f:?} {c}");
                assert_eq!(l, before, "invalid class must not mutate");
            }
        }
    }

    #[test]
    fn exhaustive_categorical_transport() {
        let mut count = 0;
        for a in NAMES[0] { for b in NAMES[1] { for c in NAMES[2] {
            for d in NAMES[3] { for e in NAMES[4] { for f in NAMES[5] {
                let cols = [*a, *b, *c, *d, *e, *f, "literal meaning"];
                let l = parse_labels(&cols).unwrap();
                assert_eq!(labels_tsv(&l), cols.join("\t"));
                for (i, field) in Field::ALL.into_iter().enumerate() {
                    assert_eq!(field.get(&l), NAMES[i].iter().position(|s| *s == cols[i]).unwrap());
                }
                count += 1;
            }}}
        }}}
        assert_eq!(count, 4608);
    }

    #[test]
    fn whitespace_fibers_and_transport_domain() {
        let base = ["decl", "assert", "0", "0", "none", "none", "a  meaning"];
        let expected = parse_labels(&base).unwrap();
        for pad in [" ", "\r\n", "\u{2003}"] {
            let padded: Vec<String> = base.iter().map(|s| format!("{pad}{s}{pad}")).collect();
            assert_eq!(parse_labels(&padded.iter().map(String::as_str).collect::<Vec<_>>()).unwrap(), expected);
        }
        // These public values are representable but outside the lossless row domain.
        let mut l = expected;
        l.means = " a meaning ".into();
        assert_ne!(parse_labels(&labels_tsv(&l).split('\t').collect::<Vec<_>>()).unwrap(), l);
        l.means = "a\tmeaning".into();
        assert!(parse_labels(&labels_tsv(&l).split('\t').collect::<Vec<_>>()).is_err());
    }

    #[test]
    fn rejection_order_and_word_boundary() {
        let valid = ["decl", "assert", "0", "0", "none", "none", "-"];
        for mask in 1usize..128 {
            let mut cols = valid;
            for (i, col) in cols.iter_mut().enumerate() {
                if mask & (1 << i) != 0 { *col = if i == 6 { "" } else { "invalid" }; }
            }
            let error = parse_labels(&cols).unwrap_err();
            let first = mask.trailing_zeros() as usize;
            assert!(error.starts_with(["form=", "act=", "indirect=", "hedge=", "polarity=", "voice=", "means is empty"][first]), "{error}");
        }
        for n in [0, 1, 29, 30, 31] {
            let words = vec!["word"; n].join("\u{2003}");
            let mut cols = valid;
            cols[6] = &words;
            assert_eq!(parse_labels(&cols).is_ok(), (1..=30).contains(&n));
        }
        for n in [0, 6, 8] { assert!(parse_labels(&vec!["-"; n]).is_err()); }
    }

    #[test]
    fn json_and_tsv_have_distinct_closed_names() {
        for form in Form::ALL {
            let json = serde_json::to_string(form).unwrap();
            assert_eq!(serde_json::from_str::<Form>(&json).unwrap(), *form);
            assert!(serde_json::from_str::<Form>(&format!("{:?}", form.name())).is_err());
            assert_eq!(Form::parse(form.name()), Some(*form));
            assert_eq!(Form::parse(&format!(" {} ", form.name())), None);
        }
        let l = Labels::default();
        assert_eq!(serde_json::from_str::<Labels>(&serde_json::to_string(&l).unwrap()).unwrap(), l);
        assert_eq!(labels_tsv(&l), "decl\tassert\t0\t0\tnone\tnone\t-");
    }
}
