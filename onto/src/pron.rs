//! v3: API for pronouns — «gender / animate / human» for a lemma in context. Interface and tests only; the consumer is
//! the next version of coreference (`coref/`): he/she/it/they to the antecedent.
//!
//! The lemma's sense comes from the sentence (WSD v1), otherwise the core concept or the first sense. Then:
//! - **human / animate** — seeds `Human` and `Animate` on the world layer (real, fable, fairy tale);
//! - **gender** — cues from the gloss and OEWN ancestors: female/woman/girl/mother… → fem., male/man/boy/king… → masc.;
//! - **persona** — on a layer where an animal speaks (`CanSpeak`), it may be he/she (the fox in a fable is «he»).
//!
//! Every answer comes with a reason.

use crate::kb::*;
use crate::model::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gender {
    Masc,
    Fem,
    /// inanimate, or an animal without a gender cue — «it»
    Neut,
    /// a human or persona without a gender cue — he or she
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tri {
    Yes,
    No,
    Unknown,
}

/// Lemma context: sentence (for WSD) and world layer.
#[derive(Clone, Copy, Debug)]
pub struct Ctx<'a> {
    pub sentence: Option<&'a str>,
    pub layer: LayerId,
}

#[derive(Clone, Debug)]
pub struct PronInfo {
    pub concept: Cid,
    pub gender: Gender,
    pub animate: Tri,
    pub human: Tri,
    /// may be he/she as a persona (speaks on this layer)
    pub persona: bool,
    pub why: Vec<String>,
}

impl PronInfo {
    /// Allowed 3rd person singular pronouns.
    pub fn pronouns(&self) -> &'static [&'static str] {
        match (self.human, self.persona, self.gender) {
            (_, _, Gender::Masc) if self.human == Tri::Yes || self.persona => &["he"],
            (_, _, Gender::Fem) if self.human == Tri::Yes || self.persona => &["she"],
            (Tri::Yes, _, _) => &["he", "she"],
            (_, true, _) => &["he", "she", "it"],
            (_, _, Gender::Masc) => &["it", "he"],
            (_, _, Gender::Fem) => &["it", "she"],
            _ => &["it"],
        }
    }
}

const FEM: &[&str] = &["female", "woman", "women", "girl", "mother", "wife", "daughter", "sister", "queen", "princess", "lady", "she", "her", "widow", "hen", "cow", "mare", "ewe", "sow", "doe", "vixen", "witch", "nun", "maid", "maiden"];
const MASC: &[&str] = &["male", "man", "men", "boy", "father", "husband", "son", "brother", "king", "prince", "gentleman", "he", "his", "bull", "stallion", "ram", "boar", "stag", "cock", "rooster", "monk", "sir", "lord"];

/// Gender from gloss words (and ancestor names): the first cue wins; «female» and «male» together — unknown.
pub fn gender_cue(text: &str) -> Option<(Gender, String)> {
    let words: Vec<String> = text.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(|w| w.to_lowercase()).collect();
    let f = words.iter().find(|w| FEM.contains(&w.as_str()));
    let m = words.iter().find(|w| MASC.contains(&w.as_str()));
    match (f, m) {
        (Some(f), None) => Some((Gender::Fem, f.clone())),
        (None, Some(m)) => Some((Gender::Masc, m.clone())),
        _ => None,
    }
}

fn tri(kb: &Kb, c: Cid, p: Prop, layer: LayerId) -> (Tri, String) {
    match kb.prop(c, p, layer) {
        PropAnswer::Known { value, win, .. } => (if value { Tri::Yes } else { Tri::No }, format!("{}={} — seed {}", p.name(), if value { "yes" } else { "no" }, kb.asserts[win].id)),
        PropAnswer::Undetermined { .. } => (Tri::Unknown, format!("{}: conflicting inheritance", p.name())),
        PropAnswer::Unknown => (Tri::Unknown, format!("{}: no seed", p.name())),
    }
}

/// Lemma sense for a pronoun: WSD by sentence, otherwise the core, otherwise the first sense.
pub fn sense(kb: &Kb, lemma: &str, ctx: &Ctx) -> Option<(Cid, String)> {
    let g = &kb.g;
    let f = crate::query::lookup(g, lemma, Pos::Noun)?;
    if let Some(s) = ctx.sentence {
        let w = crate::wsd::Wsd { kb, idf: crate::wsd::Idf::new(g), prior: crate::wsd::Prior::default(), mix: crate::wsd::Mix::ALL, perm: None };
        if let Some(ch) = w.choose(lemma, s) {
            return Some((ch.sense(), "WSD by sentence".into()));
        }
    }
    if let Some(&c) = f.senses.iter().find(|&&c| crate::core::of_concept(g, c).is_some()) {
        return Some((c, "core concept".into()));
    }
    Some((f.senses[0], "first sense".into()))
}

/// Gender, animacy, humanness for a lemma in context.
pub fn info(kb: &Kb, lemma: &str, ctx: &Ctx) -> Option<PronInfo> {
    let g = &kb.g;
    let (c, how) = sense(kb, lemma, ctx)?;
    let mut why = vec![format!("sense: {} ({how})", g.show(c))];
    let (human, wh) = tri(kb, c, Prop::Human, ctx.layer);
    let (animate, wa) = tri(kb, c, Prop::Animate, ctx.layer);
    let (speak, _) = tri(kb, c, Prop::CanSpeak, ctx.layer);
    why.push(wh);
    why.push(wa);
    let persona = speak == Tri::Yes && human != Tri::Yes;
    if persona {
        why.push(format!("speaks on layer {} — persona", kb.layers[ctx.layer as usize].name));
    }
    // gender: the concept's own gloss, then ancestor names (up to 3 steps)
    let mut text = g.gloss_of(c).to_string();
    let mut cue = gender_cue(&format!("{} {text}", g.label(c)));
    if cue.is_none() {
        let mut cur = c;
        for _ in 0..3 {
            let Some(st) = kb.up(cur).first().copied() else { break };
            cur = st.to;
            text = g.label(cur);
            if let Some(x) = gender_cue(&text) {
                cue = Some((x.0, format!("{} (ancestor {})", x.1, g.label(cur))));
                break;
            }
        }
    }
    let gender = match cue {
        Some((gd, w)) => {
            why.push(format!("gender: {gd:?} — cue «{w}»"));
            gd
        }
        None if human == Tri::Yes || persona => {
            why.push("gender: no cue — he or she".into());
            Gender::Unknown
        }
        None => Gender::Neut,
    };
    Some(PronInfo { concept: c, gender, animate, human, persona, why })
}

/// Short wrappers for coreference.
pub fn gender(kb: &Kb, lemma: &str, ctx: &Ctx) -> Option<Gender> {
    info(kb, lemma, ctx).map(|i| i.gender)
}
pub fn animate(kb: &Kb, lemma: &str, ctx: &Ctx) -> Option<Tri> {
    info(kb, lemma, ctx).map(|i| i.animate)
}
pub fn human(kb: &Kb, lemma: &str, ctx: &Ctx) -> Option<Tri> {
    info(kb, lemma, ctx).map(|i| i.human)
}
