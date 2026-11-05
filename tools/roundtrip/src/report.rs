//! The report: per set, the normal-mode divergence from every start and the exact-mode residual of every layer;
//! across sets, the residual classes ranked with the change to the system that would remove them.

use std::collections::BTreeMap;
use std::fmt::Write;

use crate::class::{CLASSES, Class, LAYERS, Layer};
use crate::tally::{STARTS, Tally};

/// What would have to change in Mova for a class of residual at a layer to go away.
pub fn proposal(l: Layer, c: Class) -> Option<&'static str> {
    Some(match (l, c) {
        (_, Class::Meta) => return None,
        (Layer::Tokens, Class::Layout) => "en/tok: a document layout record next to the tokens (line breaks, blank lines as offsets); one space bit cannot say where a book wraps its lines — layout, not language",
        (Layer::Tokens, Class::Spacing) => "en/tok: keep the literal whitespace (double spaces, tabs, characters the tokenizer does not keep) or normalize it when a text is read",
        (Layer::Tree, Class::Order) => "en/lin.rs: order features the generator lacks (fronted adverbials and obliques, «said X» and inverted speech frames, coordination, vocatives, parentheticals), trained on the agreed school and tales trees too; a swap step for non-projective trees",
        (Layer::Tree, Class::Spacing) => "en/text.rs spacing rules: quotation marks (open/close by position), typographic clitics (’s, n’t), dashes and hyphens; SpaceAfter only where the rules miss",
        (Layer::Tree, Class::Case) => "en/morph.rs case: the capital after an opening quote and at a speech start, titles used as names (the Queen, the Mock Turtle), all-caps words; a case bit learned per lemma",
        (Layer::Tree, Class::Choice) => "en tagger + lemmatizer: a form that is another word than its lemma and tag say (the tag or the lemma contradicts the form); fix the tag or lemma where inflection does not give the form back — a free self-check of the annotation",
        (Layer::Tree, Class::Form) => "en/morph.rs: contractions as a bit on the word ('s, n't, ca, 'm, 're, 'll), a variant bit for doublets (learnt/learned); inflection from lemma, tag and the tree's FEATS is done (`tree_feats`)",
        (Layer::Tree, Class::Typography) => "en: the source's typography (’ “ ” — vs ' \") as one bit per document (a style record), not a per-word escape",
        (Layer::Tree, Class::Spelling) => "en/morph.rs + emode.rs: a spelling layer for the source's forms (archaic -eth/-est, thee/thou, British spellings, misspellings): form ↔ lemma+tag+variant",
        (Layer::Frames, Class::Function) => "prag Ent/Frame: keep determiners (a/the/this/his), the preposition of a place and the clause markers (that, if, when, because); a frame keeps the head only",
        (Layer::Frames, Class::Punct) => "en/lin.rs: a punctuation generator from clause structure (commas around clauses and appositions, quotes around speech) so punctuation is decoded, not stored",
        (Layer::Frames, Class::Tense) => "prag Frame `tam` is read: what is left is the realizer's agreement (VBP for VBZ), get-passives, and the copula or auxiliaries of clauses the frames do not make",
        (Layer::Frames, Class::Dropped) => "prag frames: every content word in some frame or entity — adverbials, nmod of nouns, relative clauses, xcomp chains, vocatives, interjections",
        (Layer::Frames, Class::Quantity) => "prag Ent: numbers and quantifiers (nummod, all/some/every), the number of each mention",
        (Layer::Frames, Class::Neg) => "prag Frame: negation beyond the verb (no, never, nothing, nobody, nor) with its scope",
        (Layer::Frames, Class::Attachment) => "prag frame graph: clause links as the tree has them (conj vs a new clause, advcl type, xcomp/ccomp), apposition, the head of a copular clause",
        (Layer::Frames, Class::Lexical) => "prag frames: lemma choices the realizer cannot make (copula vs verb, particle verbs, pronoun resolution that replaces a word)",
        (Layer::Frames, Class::PronName) => "prag frames: a pronoun resolved to an entity loses its form in some slots; keep the pronoun in every slot",
        (Layer::Frames, Class::Case) => "prag frames: entity ids lower-cased or capitalized against the text; keep the lemma's case",
        (Layer::Frames, Class::Form) => "prag Ent: degree and other inflection of modifiers",
        (Layer::World, Class::PronName) => "world → frames: the referring-expression generator (`world::refer`) misses possessives (his, its, their), pronouns beyond the sentence before, and the author's choice of a name where a pronoun would resolve",
        (Layer::World, Class::Dropped) => "world: modifiers with a time (which one was new at which mention), as world v1 `set X key=value` states, not a set on the entity",
        (Layer::World, Class::Sentence) => "world → frames: a sentence planner (which events go into one sentence), with the sentence and paragraph boundaries kept as discourse structure",
        (Layer::World, Class::Links) => "world: the explicit sequence links (then, after, before) as edges of the story time axis (story.rs TimeGraph)",
        (Layer::World, Class::Repetition) => "world: a statement said again as a mention of an existing fact (world v1 `mention`), not dropped",
        (Layer::World, Class::Quote) => "world: direct versus reported speech on the say event",
        (Layer::World, Class::Quantity) => "world: the number of a mention (one of them, both) next to the entity's",
        (Layer::World, Class::Function) => "world: whether a mention is a name or a description",
        (Layer::World, Class::Lexical) => "world: frame fields the event line does not keep",
        (Layer::Tokens, Class::Markup) => "site pages: the markup from a template learned per section (front matter, crumbs, the HTML link, markers) and from the page id, instead of copies of the markup before it in the block",
        (Layer::Tree, Class::Conllu) => "site pages: the Hebrew trees (right to left, MACULA's) from the Hebrew route and MACULA's MISC from the verse id and a lexicon; the English trees' patches are what `en` gets wrong against the agreed trees",
        (Layer::Frames, Class::FrTable) => "site pages: the frames reader's state over the pages before the block (the chapter, the document); the Hebrew verbs' frames from MACULA's Frame= on the rebuilt tree",
        (Layer::Tokens, Class::Unparsed) => "site pages: the Hebrew lines through the Hebrew route, the Aramaic ones need an Aramaic parser",
        _ => "—",
    })
}

fn f1(x: f64) -> String {
    format!("{x:.1}")
}

fn per(x: f64, n: u64) -> f64 {
    // + 0.0: an empty f64 sum is -0.0
    if n == 0 { 0.0 } else { x / n as f64 + 0.0 }
}

/// The report of one set.
pub fn set(name: &str, t: &Tally, exact: bool, normal: bool) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "## {name}\n");
    let _ = writeln!(
        s,
        "{} volumes, {} bytes, {} sentences, {} tokens, {} frames, {} world events{}.\n",
        t.volumes,
        t.bytes,
        t.sents,
        t.tokens,
        t.frames,
        t.events,
        if t.gold_used + t.gold_skipped > 0 { format!("; gold trees used for {} sentences ({} skipped: other tokens)", t.gold_used, t.gold_skipped) } else { String::new() }
    );
    if exact {
        let _ = writeln!(s, "**Exact mode:** {} of {} volumes came back byte for byte.{}\n", t.exact_ok, t.volumes, if t.exact_fail.is_empty() { String::new() } else { format!(" First failures: {}", t.exact_fail.join("; ")) });
        let _ = writeln!(s, "Residual per sentence — bits (items) — by layer and class:\n");
        let _ = writeln!(s, "| class | {} |", LAYERS.map(|l| l.boundary()).join(" | "));
        let _ = writeln!(s, "|---|{}", "---:|".repeat(LAYERS.len()));
        for c in CLASSES {
            if !LAYERS.iter().any(|l| t.residual.cells.contains_key(&(*l, c))) {
                continue;
            }
            let row: Vec<String> = LAYERS
                .iter()
                .map(|l| t.residual.cells.get(&(*l, c)).map_or("".into(), |x| format!("{} ({:.2})", f1(per(x.bits, t.sents)), per(x.items as f64, t.sents))))
                .collect();
            let _ = writeln!(s, "| {} | {} |", c.name(), row.join(" | "));
        }
        let tot: Vec<String> = LAYERS
            .iter()
            .map(|l| {
                let (b, n) = t.residual.cells.iter().filter(|(k, _)| k.0 == *l && k.1 != Class::Meta).fold((0.0, 0u64), |a, (_, v)| (a.0 + v.bits, a.1 + v.items));
                format!("**{}** ({:.2})", f1(per(b, t.sents)), per(n as f64, t.sents))
            })
            .collect();
        let _ = writeln!(s, "| **all but meta** | {} |", tot.join(" | "));
        let lang: Vec<String> = LAYERS
            .iter()
            .map(|l| {
                let b: f64 = t.residual.cells.iter().filter(|(k, _)| k.0 == *l && !matches!(k.1, Class::Meta | Class::Layout)).map(|(_, v)| v.bits).sum();
                format!("{:.2}", per(b, t.tokens))
            })
            .collect();
        let _ = writeln!(s, "| bits per token, without meta and layout | {} |\n", lang.join(" | "));
    }
    if normal {
        let _ = writeln!(s, "**Normal mode** (no residual): the text decoded from each start, against the original sentence.\n");
        let _ = writeln!(s, "| start | sentences | exact % | chrF | divergent words per 100 tokens | top classes |");
        let _ = writeln!(s, "|---|---:|---:|---:|---:|---|");
        for st in STARTS {
            let Some(d) = t.normal.get(&st) else { continue };
            if d.sents == 0 {
                continue;
            }
            let items: u64 = d.cells.values().map(|c| c.items).sum();
            let mut top: Vec<(&Class, u64)> = d.cells.iter().map(|(k, v)| (k, v.items)).collect();
            top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
            let top: Vec<String> = top.iter().take(4).map(|(c, n)| format!("{} {:.1}", c.name(), 100.0 * *n as f64 / d.tokens.max(1) as f64)).collect();
            let _ = writeln!(s, "| {} | {} | {:.1} | {:.1} | {:.1} | {} |", st.name(), d.sents, 100.0 * per(d.exact as f64, d.sents), 100.0 * per(d.chrf, d.sents), 100.0 * items as f64 / d.tokens.max(1) as f64, top.join(", "));
        }
        let _ = writeln!(s, "\nDivergent words per 100 tokens, by class and start:\n");
        let starts: Vec<_> = STARTS.iter().filter(|st| t.normal.get(st).is_some_and(|d| d.sents > 0)).collect();
        let _ = writeln!(s, "| class | {} |", starts.iter().map(|x| x.name()).collect::<Vec<_>>().join(" | "));
        let _ = writeln!(s, "|---|{}", "---:|".repeat(starts.len()));
        for c in CLASSES {
            if !starts.iter().any(|st| t.normal[st].cells.contains_key(&c)) {
                continue;
            }
            let row: Vec<String> = starts.iter().map(|st| t.normal[st].cells.get(&c).map_or("".into(), |x| format!("{:.1}", 100.0 * x.items as f64 / t.normal[st].tokens.max(1) as f64))).collect();
            let _ = writeln!(s, "| {} | {} |", c.name(), row.join(" | "));
        }
        let _ = writeln!(s);
    }
    s
}

/// Examples of a set: residual items per layer and class, divergences per start and class.
pub fn examples(name: &str, t: &Tally) -> String {
    let mut s = format!("## {name}\n\n### Residual (exact mode)\n\n");
    for ((l, c), cell) in &t.residual.cells {
        let _ = writeln!(s, "- **{} / {}** ({} items): {}", l.boundary(), c.name(), cell.items, cell.examples.iter().map(|e| format!("`{}`", e.replace('`', "'"))).collect::<Vec<_>>().join(" · "));
        if !cell.kinds.is_empty() {
            let mut k: Vec<(&String, &u64)> = cell.kinds.iter().collect();
            k.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            let _ = writeln!(s, "  - by kind: {}", k.iter().take(12).map(|(x, n)| format!("{} {:.1}%", x.replace('`', "'"), 100.0 * **n as f64 / cell.items.max(1) as f64)).collect::<Vec<_>>().join(", "));
        }
    }
    let _ = writeln!(s, "\n### Divergence (normal mode)\n");
    for (st, d) in &t.normal {
        for (c, cell) in &d.cells {
            let _ = writeln!(s, "- **{} / {}** ({}): {}", st.name(), c.name(), cell.items, cell.examples.iter().take(2).map(|e| format!("`{}`", e.replace('`', "'"))).collect::<Vec<_>>().join(" · "));
        }
    }
    s + "\n"
}

/// The residual classes of all sets ranked by bits per sentence (the mean over the sets, each set weighing the
/// same), with the change that would remove them.
pub fn ranked(sets: &[(String, Tally)]) -> String {
    let mut mean: BTreeMap<(Layer, Class), (f64, f64)> = BTreeMap::new();
    let mut per_set: BTreeMap<(Layer, Class), Vec<String>> = BTreeMap::new();
    let k = sets.len().max(1) as f64;
    for (n, t) in sets {
        for (key, v) in &t.residual.cells {
            let e = mean.entry(*key).or_default();
            e.0 += per(v.bits, t.sents) / k;
            e.1 += per(v.items as f64, t.sents) / k;
            per_set.entry(*key).or_default().push(format!("{n} {:.1}", per(v.bits, t.sents)));
        }
    }
    let mut rows: Vec<_> = mean.into_iter().filter(|(key, _)| proposal(key.0, key.1).is_some()).collect();
    let total: f64 = rows.iter().map(|(_, v)| v.0).sum();
    rows.sort_by(|a, b| b.1.0.total_cmp(&a.1.0).then(a.0.cmp(&b.0)));
    let mut s = String::from("| rank | layer | class | bits / sentence | items / sentence | share | per set (bits / sentence) | change |\n|---:|---|---|---:|---:|---:|---|---|\n");
    for (i, ((l, c), (bits, items))) in rows.iter().enumerate() {
        let _ = writeln!(s, "| {} | {} | {} | {:.1} | {:.2} | {:.1}% | {} | {} |", i + 1, l.boundary(), c.name(), bits, items, 100.0 * bits / total.max(1e-9), per_set[&(*l, *c)].join(", "), proposal(*l, *c).unwrap_or("—"));
    }
    s
}

/// Machine-readable rows: set, layer, class, items, bits, sentences, tokens.
pub fn tsv(sets: &[(String, Tally)]) -> (String, String) {
    let mut r = String::from("set\tlayer\tclass\titems\tbits\tsentences\ttokens\n");
    let mut n = String::from("set\tstart\tsentences\texact\tchrf\ttokens\tclass\titems\n");
    for (name, t) in sets {
        for ((l, c), v) in &t.residual.cells {
            let _ = writeln!(r, "{name}\t{}\t{}\t{}\t{:.1}\t{}\t{}", l.name(), c.name(), v.items, v.bits, t.sents, t.tokens);
        }
        for (st, d) in &t.normal {
            for (c, v) in &d.cells {
                let _ = writeln!(n, "{name}\t{}\t{}\t{}\t{:.4}\t{}\t{}\t{}", st.name(), d.sents, d.exact, d.chrf, d.tokens, c.name(), v.items);
            }
        }
    }
    (r, n)
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn zero_normalization_and_rows() {
        assert_eq!(per(12.0, 0), 0.0);
        assert_eq!(per(12.0, 3), 4.0);
        assert!(proposal(Layer::World, Class::Meta).is_none());
        let empty = set("empty", &Tally::default(), true, true);
        assert!(!empty.contains("NaN") && !empty.contains("inf"));
        let mut t = Tally {
            sents: 2,
            tokens: 4,
            ..Default::default()
        };
        t.residual
            .add(Layer::Tree, Class::Case, 8.0, || "`case`".into());
        let sets = vec![("sample".into(), t)];
        let (r, n) = tsv(&sets);
        assert!(r.contains("sample\ttree\tcase\t1\t8.0\t2\t4\n"));
        assert_eq!(n.lines().count(), 1);
        assert!(ranked(&sets).contains("4.0 | 0.50 | 100.0%"));
        assert!(examples("sample", &sets[0].1).contains("'case'"));
    }
}
