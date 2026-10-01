//! First cut of the pragmatics schema —
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

/// A field the MMM learns (all except `means`).
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

    /// Write the class into the labels.
    pub fn set(self, l: &mut Labels, c: usize) {
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
