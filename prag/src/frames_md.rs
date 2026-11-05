//! The frames of an English sentence as a Markdown table: the table the site's corpora pages give under every tree
//! (verb, tense and mood, who, what or whom, to whom, where, when, the other phrases, and what assigned the roles).
//! It lives here so that the page builder and `roundtrip`'s site blocks, which rebuild the table from the tree
//! instead of storing it, write the same bytes.

use std::fmt::Write as _;

use crate::frames::{By, Ent, Frame, Kind, Slot};

/// Markdown text: the characters Markdown would read as markup are escaped.
pub fn t(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        if matches!(c, '\\' | '`' | '*' | '_' | '[' | ']' | '<' | '>' | '|' | '#') {
            o.push('\\');
        }
        o.push(c);
    }
    o
}

/// Prepositions and nouns that put a phrase in time rather than space.
const TIME_PREPS: [&str; 8] = ["after", "before", "until", "till", "during", "since", "ere", "throughout"];
const TIME_NOUNS: [&str; 24] = ["day", "night", "morning", "evening", "year", "month", "week", "time", "hour", "generation", "sabbath", "season", "beginning", "end", "while", "today", "tomorrow", "yesterday", "age", "moment", "noon", "dawn", "spring", "eternity"];
const TIME_WORDS: [&str; 10] = ["then", "now", "again", "ever", "never", "always", "still", "yet", "afterward", "henceforth"];
const PLACE_WORDS: [&str; 6] = ["there", "here", "thither", "hither", "thence", "hence"];

/// An entity as a phrase: its determiner or possessor, the head as written (with its adjectives and compounds), its prepositional
/// modifiers and the phrases coordinated with it — the words the reader kept, in the order English says them.
fn phrase(e: &Ent) -> String {
    let mut w: Vec<String> = Vec::new();
    if let Some(p) = &e.poss {
        let p = phrase(p);
        w.push(if matches!(p.to_lowercase().as_str(), "his" | "her" | "its" | "their" | "my" | "our" | "your" | "thy" | "thine" | "mine") { p } else { format!("{p}'s") });
    } else if let Some((d, _)) = &e.det {
        w.push(d.clone());
    }
    w.push(if e.text.is_empty() { e.id.clone() } else { e.text.clone() });
    for (p, m) in &e.mods {
        w.push(format!("{p} {}", phrase(m)));
    }
    for c in &e.conj {
        w.push(format!("{} {}", c.cc.clone().unwrap_or_else(|| "and".into()), phrase(c)));
    }
    w.join(" ")
}

fn ent(e: &Ent) -> String {
    let p = phrase(e);
    match &e.pron {
        Some(pr) if !e.id.eq_ignore_ascii_case(pr) && !p.eq_ignore_ascii_case(&e.id) => format!("{} *= {}*", t(&p), t(&e.id)),
        _ => t(&p),
    }
}

fn is_time(prep: &str, e: &Ent) -> bool {
    TIME_PREPS.contains(&prep.to_lowercase().as_str()) || TIME_NOUNS.contains(&e.id.to_lowercase().as_str())
}

/// Tense, aspect, voice and modality in words, from the reader's short tag.
fn tam_words(f: &Frame) -> String {
    if !f.tam.read {
        return String::new();
    }
    f.tam
        .show()
        .split('.')
        .filter(|x| !x.is_empty())
        .map(|x| match x {
            "pres" => "present",
            "fut" => "future",
            "imp" => "imperative",
            "inf" => "infinitive",
            "to-inf" => "to-infinitive",
            "ger" => "gerund",
            "part" => "participle",
            "perf" => "perfect",
            "prog" => "progressive",
            "pass" => "passive",
            "do" => "do-support",
            other => other,
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

/// The frames of an English sentence as a table: verb, tense and mood, who, what or whom, to whom, where, when,
/// the other phrases, and what assigned the roles.
pub fn table(fs: &[Frame]) -> String {
    let mut rows = String::new();
    for (i, f) in fs.iter().enumerate() {
        if f.kind == Kind::Bare {
            continue;
        }
        let verb = match f.kind {
            Kind::Fact(l) if f.verb.is_empty() || f.verb == "be" => l.name().to_string(),
            _ => f.verb.clone(),
        };
        let mut v = format!("{}**{}**", if f.neg { "**not** " } else { "" }, t(&verb));
        if let Kind::Say(a) = f.kind {
            let _ = write!(v, " *speech: {}*", a.name());
        }
        let mut notes: Vec<&str> = Vec::new();
        if f.cond {
            notes.push("a condition");
        }
        if f.irrealis {
            notes.push("not said to happen");
        }
        if f.quoted {
            notes.push("in a quote");
        }
        if !notes.is_empty() {
            let _ = write!(v, " *{}*", notes.join(", "));
        }
        let role = |s: Slot, e: &Option<Ent>| -> String {
            match e {
                Some(e) => format!("{}{}", ent(e), if f.unsaid.contains(&s) { " *not said here*" } else { "" }),
                None => String::new(),
            }
        };
        // the reader's place slot, unless its head is a word of time («in the beginning»)
        let (in_time, in_space): (Vec<&Ent>, Vec<&Ent>) = f.place.iter().partition(|e| TIME_NOUNS.contains(&e.id.to_lowercase().as_str()));
        let mut place: Vec<String> = Vec::new();
        let mut other: Vec<String> = Vec::new();
        for e in in_space {
            // where something ends up, not a place: «into paper and plastic and food» *goal*
            match f.goal() {
                Some(r) => other.push(format!("{} {} *{r}*", t(e.case.as_deref().unwrap_or_default()), ent(e))),
                None => place.push(ent(e)),
            }
        }
        let mut when: Vec<String> = in_time.into_iter().map(|e| format!("{}{}", e.case.as_ref().map(|c| format!("{} ", t(c))).unwrap_or_default(), ent(e))).collect();
        for (p, e) in &f.obls {
            let s = format!("{} {}", t(p), ent(e));
            // a phrase VerbNet calls a Goal or a Result is where something ends up, not a place: «into paper» *goal*
            let role = f.how.roles.iter().find(|(k, r)| *k == format!("{p} {}", e.id) && matches!(r.as_str(), "Goal" | "Result")).map(|(_, r)| r.to_lowercase());
            if is_time(p, e) {
                when.push(s)
            } else if let Some(r) = role {
                other.push(format!("{s} *{r}*"))
            } else {
                other.push(s)
            }
        }
        for (r, e) in &f.nps {
            if TIME_NOUNS.contains(&e.id.to_lowercase().as_str()) { when.push(ent(e)) } else { other.push(format!("{} *{}*", ent(e), r)) }
        }
        for (_, w, _) in &f.words {
            let lw = w.to_lowercase();
            if TIME_WORDS.contains(&lw.as_str()) {
                when.push(t(w));
            } else if PLACE_WORDS.contains(&lw.as_str()) {
                place.push(t(w));
            }
        }
        // the reader's times without a role («when:week» for «a week», «on Tuesdays») go to «when»
        let (times, attrs): (Vec<&String>, Vec<&String>) = f.attrs.iter().partition(|a| a.starts_with("when:"));
        when.extend(times.iter().map(|a| t(&a["when:".len()..])));
        if !attrs.is_empty() {
            other.push(format!("{} *property*", t(&attrs.iter().map(|a| a.as_str()).collect::<Vec<_>>().join(", "))));
        }
        let how = if f.how.by == By::Heuristic { "tree rules".to_string() } else { f.how.show() };
        let _ = writeln!(rows, "| {} | {v} | {} | {} | {} | {} | {} | {} | {} | {} |", i + 1, t(&tam_words(f)), role(Slot::Agent, &f.agent), role(Slot::Patient, &f.patient), role(Slot::Recipient, &f.recipient), place.join("; "), when.join("; "), other.join("; "), t(&how));
    }
    if rows.is_empty() {
        return "<!-- .note -->\nThe frames reader finds no event or fact in this sentence.\n\n".into();
    }
    format!("<!-- .fr -->\n| # | verb | tense, mood | who | what, whom | to whom | where | when | other | roles by |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n{rows}\n<!-- .note -->\nOne row per clause the reader finds. «not said here»: a role filled from the clause above. «roles by»: a VerbNet class and frame when one fits the verb, else the reader's rules on the tree.\n\n")
}

/// A word of a Hebrew tree as MACULA writes it (the CoNLL-U columns the Hebrew frames table reads).
#[derive(Clone, Debug, Default)]
pub struct HWord {
    pub id: usize,
    pub form: String,
    pub lemma: String,
    pub head: usize,
    pub feats: String,
    pub misc: String,
}

fn kv<'a>(s: &'a str, key: &str) -> Option<&'a str> {
    s.split('|').find_map(|x| x.strip_prefix(key).and_then(|r| r.strip_prefix('=')))
}

/// A form without its cantillation accents.
pub fn bare(s: &str) -> String {
    s.chars().filter(|c| !matches!(*c as u32, 0x0591..=0x05AF)).collect()
}

/// The words of a Hebrew CoNLL-U (ranges and comments left out).
pub fn hwords(conllu: &str) -> Vec<HWord> {
    conllu
        .lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("```"))
        .filter_map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() < 10 || c[0].contains('-') || c[0].contains('.') {
                return None;
            }
            Some(HWord { id: c[0].parse().ok()?, form: c[1].into(), lemma: c[2].into(), head: c[6].parse().unwrap_or(0), feats: c[5].into(), misc: c[9].into() })
        })
        .collect()
}

/// MACULA's verb frames of a Hebrew verse as the site's table: each verb with its form and stem, and the head word of
/// each participant (A0, A1, A2, AA) with its gloss.
pub fn hebrew_table(ws: &[HWord]) -> String {
    use std::collections::BTreeMap;
    let by_mid: BTreeMap<&str, usize> = ws.iter().enumerate().filter_map(|(i, w)| kv(&w.misc, "MId").map(|m| (m.trim_start_matches(|c: char| !c.is_ascii_digit()), i))).collect();
    let word = |i: usize| -> String {
        let w = &ws[i];
        let g = kv(&w.misc, "Gloss").unwrap_or("").replace('.', " ");
        if g.is_empty() { t(&bare(&w.form)) } else { format!("{} *{}*", t(&bare(&w.form)), t(&g)) }
    };
    let mut rows = String::new();
    for (i, w) in ws.iter().enumerate() {
        let Some(f) = kv(&w.misc, "Frame").filter(|f| !f.is_empty()) else { continue };
        let mut roles: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        let mut cur = "";
        for part in f.split(';').filter(|p| !p.is_empty()) {
            let (r, id) = match part.split_once(':') {
                Some((r, id)) => (r, id),
                None => (cur, part),
            };
            cur = r;
            if let Some(&k) = by_mid.get(id) {
                roles.entry(r).or_default().push(k);
            }
        }
        let cell = |r: &str| roles.get(r).map(|ks| ks.iter().map(|&k| word(k)).collect::<Vec<_>>().join(" + ")).unwrap_or_default();
        let neg = ws.iter().any(|x| x.head == w.id && matches!(x.lemma.as_str(), "לֹא" | "אַל" | "אַיִן" | "בַּל"));
        let form = [kv(&w.misc, "Type").map(|x| x.replace('_', " ")), kv(&w.feats, "HebBinyan").map(|x| x.to_lowercase())].into_iter().flatten().collect::<Vec<_>>().join(" · ");
        let _ = writeln!(rows, "| {}{} | {} | {} | {} | {} | {} |", if neg { "**not** " } else { "" }, word(i), t(&form), cell("A0"), cell("A1"), cell("A2"), cell("AA"));
    }
    if rows.is_empty() {
        return "<!-- .note -->\nMACULA marks no verb frame in this verse.\n\n".into();
    }
    format!("<!-- .fr -->\n| verb | form · stem | who (A0) | what, whom (A1) | to whom, further (A2) | who makes it happen (AA) |\n| --- | --- | --- | --- | --- | --- |\n{rows}\n<!-- .note -->\nMACULA's semantic frames: for each verb the head word of each participant. A0 is the one who acts, A1 what the act is done to or is about, A2 a further participant; AA, with a causative verb, the one who makes it happen.\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frames::{Ctx, Tree, frames_of_tree};
    use en::annotate::Word;
    use en::gram::{Feats, Rel, Tag, UPos};

    fn w(form: &str, lemma: &str, upos: UPos, tag: Tag, head: usize, rel: Rel) -> Word {
        Word { form: form.into(), lemma: lemma.into(), upos, tag, feats: Feats::default(), head, rel }
    }

    #[test]
    fn a_table_from_a_tree_and_none_without_frames() {
        let ws = vec![w("God", "god", UPos::PROPN, Tag::NNP, 2, Rel::Nsubj), w("created", "create", UPos::VERB, Tag::VBD, 0, Rel::Root), w("the", "the", UPos::DET, Tag::DT, 4, Rel::Det), w("heaven", "heaven", UPos::NOUN, Tag::NN, 2, Rel::Obj), w(".", ".", UPos::PUNCT, Tag::Stop, 2, Rel::Punct)];
        let (fs, _, _) = frames_of_tree(&Tree::of_words(ws), &mut Ctx::default(), false);
        let s = table(&fs);
        assert!(s.starts_with("<!-- .fr -->\n| # | verb |") && s.contains("**create**") && s.contains("the heaven"), "{s}");
        assert!(table(&[]).contains("no event"));
        assert_eq!(t("a|b*c"), "a\\|b\\*c");
    }
}


#[cfg(test)]
mod wave14_models {
    use super::*;
    use en::gram::{Rel, Tag};

    fn entity(id: &str) -> Ent {
        Ent {
            id: id.into(),
            ..Ent::default()
        }
    }

    #[test]
    fn text_escape_and_accent_projection_have_distinct_fibers() {
        let chars = [
            '\\', '\u{0060}', '*', '_', '[', ']', '<', '>', '|', '#', 'a', 'é', 'א',
        ];
        for a in chars {
            for b in chars {
                let input = format!("{a}{b}");
                let expected: String = [a, b]
                    .into_iter()
                    .map(|c| {
                        if "\\\u{0060}*_[]<>|#".contains(c) {
                            format!("\\{c}")
                        } else {
                            c.to_string()
                        }
                    })
                    .collect();
                assert_eq!(t(&input), expected, "escaped character transport");
            }
        }
        assert_eq!(
            bare("\u{0590}\u{0591}א\u{05af}\u{05b0}"),
            "\u{0590}א\u{05b0}"
        );
        assert_eq!(
            bare("א\u{0591}"),
            bare("א\u{05af}"),
            "accent projection loses identity"
        );
        assert_eq!(
            t("line\nline"),
            "line\nline",
            "line breaks are outside the escape alphabet"
        );
    }

    #[test]
    fn phrase_possession_coordination_and_pronoun_annotation() {
        let mut e = entity("cat");
        e.text = "small cat".into();
        e.poss = Some(Box::new(entity("Alice")));
        e.mods.push(("with".into(), entity("hat")));
        let mut dog = entity("dog");
        dog.cc = Some("or".into());
        e.conj.push(dog);
        assert_eq!(phrase(&e), "Alice's small cat with hat or dog");
        e.poss = Some(Box::new(entity("her")));
        e.pron = Some("it".into());
        assert_eq!(ent(&e), "her small cat with hat or dog *= cat*");
        e.pron = Some("CAT".into());
        assert_eq!(ent(&e), "her small cat with hat or dog");
    }

    #[test]
    fn english_columns_preserve_source_numbers_flags_and_time_routes() {
        let mut f = Frame::new(Kind::Event, "a|b".into());
        f.agent = Some(entity("who"));
        f.patient = Some(entity("what"));
        f.recipient = Some(entity("whom"));
        f.neg = true;
        f.cond = true;
        f.irrealis = true;
        f.quoted = true;
        f.unsaid.push(Slot::Agent);
        let mut night = entity("night");
        night.case = Some("in".into());
        f.place = Some(night);
        f.obls.push(("after".into(), entity("meal")));
        f.attrs = vec!["when:today".into(), "blue".into()];
        f.words = vec![(Rel::Advmod, "there".into(), Tag::RB)];
        let out = table(&[Frame::new(Kind::Bare, "ignored".into()), f]);
        let row = out.lines().find(|l| l.starts_with("| 2 |")).unwrap();
        let cells: Vec<_> = row.split(" | ").collect();
        assert_eq!(cells.len(), 10);
        assert!(cells[1].contains("**not** **a\\|b**"));
        assert!(cells[1].contains("a condition, not said to happen, in a quote"));
        assert_eq!(cells[3], "who *not said here*");
        assert_eq!(cells[4], "what");
        assert_eq!(cells[5], "whom");
        assert_eq!(cells[6], "there");
        assert_eq!(cells[7], "in night; after meal; today");
        assert_eq!(cells[8], "blue *property*");
        assert!(!out.contains("ignored"));
        assert!(table(&[]).contains("no event or fact"));
        assert!(table(&[Frame::new(Kind::Bare, "x".into())]).contains("no event or fact"));
    }

    #[test]
    fn conllu_field_transport_is_tolerant_and_ordered() {
        let text = "# comment\nFENCE\n1-2\tx\t_\t_\t_\t_\t0\t_\t_\t_\n1.1\tx\t_\t_\t_\t_\t0\t_\t_\t_\nshort\n2\tב\tlemma\tV\t_\tHebBinyan=QAL\tbad\t_\t_\tMId=m20|Gloss=word\n1\tא\troot\tV\t_\t_\t2\t_\t_\tMId=m10\textra";
        let ws = hwords(text);
        assert_eq!(ws.len(), 2);
        assert_eq!((ws[0].id, ws[0].head), (2, 0));
        assert_eq!(ws[0].lemma, "lemma");
        assert_eq!(ws[0].feats, "HebBinyan=QAL");
        assert_eq!(ws[0].misc, "MId=m20|Gloss=word");
        assert_eq!((ws[1].id, ws[1].head), (1, 2));
        assert_eq!(kv("Glossary=x|Gloss=a|Gloss=b", "Gloss"), Some("a"));
    }

    #[test]
    fn hebrew_roles_share_ids_and_last_duplicate_wins() {
        let ws = vec![
            HWord {
                id: 1,
                form: "א\u{0591}".into(),
                feats: "HebBinyan=QAL".into(),
                misc: "MId=m1|Gloss=act.now|Type=finite_verb|Frame=A0:2;3;A1:missing;AA:2".into(),
                ..HWord::default()
            },
            HWord {
                id: 2,
                form: "old".into(),
                misc: "MId=m2".into(),
                ..HWord::default()
            },
            HWord {
                id: 3,
                form: "other".into(),
                misc: "MId=m3".into(),
                ..HWord::default()
            },
            HWord {
                id: 4,
                form: "new|x".into(),
                misc: "MId=ref2".into(),
                ..HWord::default()
            },
            HWord {
                id: 5,
                head: 1,
                lemma: "לֹא".into(),
                ..HWord::default()
            },
        ];
        let out = hebrew_table(&ws);
        let row = out.lines().find(|l| l.starts_with("| **not**")).unwrap();
        assert_eq!(
            row,
            "| **not** א *act now* | finite verb · qal | new\\|x + other |  |  | new\\|x |"
        );
        assert!(!out.contains("old"));
        assert!(hebrew_table(&[]).contains("no verb frame"));
    }

    #[test]
    fn english_hidden_provenance_has_ambiguous_reverse() {
        let mut a = Frame::new(Kind::Event, "run".into());
        let b = a.clone();
        a.src = (9, 7);
        a.cause = Some(11);
        a.next = Some(12);
        a.quote = vec![1, 2];
        assert_eq!(
            table(&[a]),
            table(&[b]),
            "table cannot reconstruct causal/source links"
        );
    }
}
