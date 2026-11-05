//! The report: systems, the exact residual by stage and class, meta-world agreement, the deltas, the lexicon,
//! examples with the reasons of every choice.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::Result;
use roundtrip::class::{CLASSES, Class};
use roundtrip::tally::Residual;

use crate::measure::STAGES;
use crate::meta::Lexicon;
use crate::profile::{self, Delta, Standard};
use crate::{Agg, BAgg, WorldAgree};

pub struct Info<'a> {
    pub split: String,
    pub lex: &'a Lexicon,
    pub std: &'a Standard,
    pub deltas: &'a [Delta],
    pub deltas_shuf: &'a [Delta],
    /// style: training units, measured units
    pub units: Vec<(String, usize, usize)>,
    /// Hebrew reader: verses, words with a chunk, words without
    pub hreader: (usize, u64, u64),
    /// the false-friends table rows
    pub false_friends: Vec<String>,
}

/// The classes of the task: lexical choice, construction, order, function words, punctuation and case, morphology.
pub const GROUPS: [&str; 6] = ["lexical", "construction", "order", "function", "punct/case", "morphology"];

pub fn group(c: Class) -> usize {
    match c {
        Class::Lexical | Class::Dropped | Class::PronName | Class::Quantity | Class::Neg => 0,
        Class::Attachment | Class::Tense | Class::Sentence | Class::Links | Class::Quote | Class::Repetition => 1,
        Class::Order => 2,
        Class::Function => 3,
        Class::Punct | Class::Case | Class::Spacing | Class::Typography | Class::Layout | Class::Meta | Class::Markup => 4,
        Class::Form | Class::Choice | Class::Spelling | Class::Accent => 5,
        // a piece kept as text (no parser for its language): not one of the task's classes, counted with lexical choice
        Class::Unparsed => 0,
        // a page's tree and frames table beyond what the layers give: construction
        Class::Conllu | Class::FrTable => 1,
        Class::Given | Class::Repeat => 4,
    }
}

fn bits(r: &Residual) -> f64 {
    r.cells.values().map(|c| c.bits).sum()
}

fn by_class(r: &Residual) -> BTreeMap<Class, (f64, u64)> {
    let mut m: BTreeMap<Class, (f64, u64)> = BTreeMap::new();
    for ((_, c), cell) in &r.cells {
        let e = m.entry(*c).or_default();
        e.0 += cell.bits;
        e.1 += cell.items;
    }
    m
}

fn groups_of(rs: &[&Residual]) -> [f64; 6] {
    let mut g = [0.0; 6];
    for r in rs {
        for ((_, c), cell) in &r.cells {
            g[group(*c)] += cell.bits;
        }
    }
    g
}

fn per(x: f64, n: u64) -> f64 {
    x / n.max(1) as f64
}

pub fn write(out: &Path, info: &Info, aggs: &[Agg], worlds: &[WorldAgree], bagg: &[BAgg]) -> Result<()> {
    let mut md = String::new();
    let _ = writeln!(md, "# Translation through the meta-world — {} split\n", info.split);
    let _ = writeln!(md, "Units (training / measured):");
    for (n, tr, ev) in &info.units {
        let _ = writeln!(md, "- {n}: {tr} / {ev}");
    }
    let _ = writeln!(
        md,
        "\nLexicon: {} words and {} phrases in {} concepts. Standard encoder: {} meta tokens seen. Hebrew reader: {} training verses, {} Hebrew words with a chunk, {} without.\n",
        info.lex.single.len(),
        info.lex.phrases.len(),
        info.lex.clusters.len(),
        info.std.tokens,
        info.hreader.0,
        info.hreader.1,
        info.hreader.2
    );
    // blocks
    if !bagg.is_empty() {
        let _ = writeln!(md, "## Blocks\n\nWhole chapters up to 2 500 words, cut at chapter boundaries; the block's text is its chapter lines, each verse's layout (paragraph and poetry marks, headings) and verse lines. Bits per block of the code that gives the target block back byte for byte (reader over the whole block, the verses' encoder and stack, the layout), per 1 000 target words; the same with the reader verse by verse (positions within a verse, nothing moves between verses); normal mode: the translation's block (the source's layout, the verses as written) against the target's.\n");
        let _ = writeln!(md, "| system | blocks | gate | words | exact blocks | chrF | reader | encoder + stack | layout | bits per block | bits per 1 000 words | verse-anchored reader: bits per block | per 1 000 words |");
        let _ = writeln!(md, "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
        for b in bagg {
            let n = b.blocks.max(1) as f64;
            let r = bits(&b.reader);
            let t = bits(&b.tail);
            let l = bits(&b.layout);
            let an = bits(&b.anchored) + l;
            let _ = writeln!(md, "| {} | {} | {} | {} | {} | {:.1} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} |", b.name, b.blocks, b.gate_ok, b.words, b.exact, 100.0 * b.chrf / n, r / n, t / n, l / n, (r + t + l) / n, 1000.0 * (r + t + l) / b.words.max(1) as f64, an / n, 1000.0 * an / b.words.max(1) as f64);
        }
        let _ = writeln!(md, "\nBlocks by class (bits per block, all parts):\n");
        let _ = writeln!(md, "| system | {} |", GROUPS.join(" | "));
        let _ = writeln!(md, "|---|{}", "---:|".repeat(GROUPS.len()));
        for b in bagg {
            let gr = groups_of(&[&b.reader, &b.tail, &b.layout]);
            let _ = writeln!(md, "| {} | {} |", b.name, gr.iter().map(|x| format!("{:.0}", per(*x, b.blocks))).collect::<Vec<_>>().join(" | "));
        }
        for b in bagg.iter().filter(|b| !b.fails.is_empty()) {
            let _ = writeln!(md, "\nBlock gate failures, {}: {}", b.name, b.fails.join(", "));
        }
        let _ = writeln!(md);
    }
    // systems
    let _ = writeln!(md, "## Systems\n\nBits per verse of the exact residual by stage: reader (world → meta-world), encoder (meta-world → the author's words: the standard with the author's delta), stack (the standard stack: tree → tokens → text); their sum is the split code, gated byte for byte. The direct code sends the target from the translation's own output (tree script + stack, as the baselines are measured), also gated.\n");
    let _ = writeln!(md, "| system | group | encoder / note | verses | gate | exact % | chrF | reader | encoder | stack | total | direct | direct gate |");
    let _ = writeln!(md, "|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for g in aggs {
        let b: Vec<f64> = (0..3).map(|s| per(bits(&g.res[s]), g.verses)).collect();
        let _ = writeln!(
            md,
            "| {} | {} | {} | {} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {} | {} |",
            g.name,
            g.group,
            g.note,
            g.verses,
            g.gate_ok,
            100.0 * per(g.exact as f64, g.verses),
            100.0 * per(g.chrf, g.verses),
            b[0],
            b[1],
            b[2],
            b.iter().sum::<f64>(),
            if g.direct.cells.is_empty() { "—".to_string() } else { format!("{:.1}", per(bits(&g.direct), g.verses)) },
            if g.direct.cells.is_empty() { "—".to_string() } else { g.direct_ok.to_string() }
        );
    }
    // classes
    let _ = writeln!(md, "\n## The residual by class (bits per verse, all stages)\n");
    let _ = writeln!(md, "| system | {} |", GROUPS.join(" | "));
    let _ = writeln!(md, "|---|{}", "---:|".repeat(GROUPS.len()));
    for g in aggs {
        let gr = groups_of(&[&g.res[0], &g.res[1], &g.res[2]]);
        let _ = writeln!(md, "| {} | {} |", g.name, gr.iter().map(|x| format!("{:.1}", per(*x, g.verses))).collect::<Vec<_>>().join(" | "));
    }
    // meta-world agreement
    let _ = writeln!(md, "\n## Meta-world agreement\n\nThe reader stage alone: bits per verse to turn the meta-world read from the source into the one read from the target, by class (zero for an autoencoder).\n");
    let _ = writeln!(md, "| pair | {} | total |", GROUPS.join(" | "));
    let _ = writeln!(md, "|---|{}---:|", "---:|".repeat(GROUPS.len()));
    for g in aggs.iter().filter(|g| g.group == "any→any" || g.group == "hebrew" || g.name.contains("shuffled lexicon") || g.name.contains("misaligned")) {
        let gr = groups_of(&[&g.res[0]]);
        let _ = writeln!(md, "| {} | {} | {:.1} |", g.name, gr.iter().map(|x| format!("{:.1}", per(*x, g.verses))).collect::<Vec<_>>().join(" | "), per(bits(&g.res[0]), g.verses));
    }
    let _ = writeln!(md, "\nThe world level: events and entities of each verse as meta concepts (English: the book's world from `prag` frames; Hebrew: MACULA's verbs and frame arguments A0–A2 read by the Hebrew reader), matched as multisets; negation: whether the verse has a negated event (Hebrew: `macula prag`'s negation).\n");
    let _ = writeln!(md, "| readings | verses | events P | events R | entities P | entities R | negation same % |");
    let _ = writeln!(md, "|---|---:|---:|---:|---:|---:|---:|");
    for w in worlds {
        let pr = |m: u64, n: u64| 100.0 * m as f64 / n.max(1) as f64;
        let _ = writeln!(md, "| {} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} |", w.name, w.verses, pr(w.ev.0, w.ev.1), pr(w.ev.0, w.ev.2), pr(w.en.0, w.en.1), pr(w.en.0, w.en.2), pr(w.neg_same, w.verses));
    }
    // deltas
    let _ = writeln!(md, "\n## Authors as deltas over the standard\n\nSize of each delta and the encoder stage on the author's held-out text (bits per unit), with the standard alone, with the author's delta, with the wrong author's delta and with a delta learned on shuffled units.\n");
    let _ = writeln!(md, "| author | overrides | delta bits | shuffled-delta overrides | standard | + delta | + wrong delta | + shuffled delta | bare forms | delta gain % |");
    let _ = writeln!(md, "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for (i, d) in info.deltas.iter().enumerate() {
        let find = |tag: &str| aggs.iter().find(|g| g.group == "autoencoder" && g.name == format!("{} → meta → {}: {tag}", d.name, d.name)).map(|g| per(bits(&g.res[1]), g.verses)).unwrap_or(f64::NAN);
        let (s, w, x, sh, b) = (find("standard"), find("standard + delta"), find("standard + wrong delta"), find("standard + shuffled delta"), find("bare forms"));
        let _ = writeln!(md, "| {} | {} | {:.0} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} |", d.name, d.size(), d.bits(), info.deltas_shuf[i].size(), s, w, x, sh, b, 100.0 * (s - w) / s.max(1e-9));
    }
    for d in info.deltas {
        let _ = writeln!(md, "\n**{}** — the overrides that fix most (level, context → chunk, what was written there before the override (mostly), tokens it fixes, tokens it holds):\n", d.name);
        for (l, key, o) in profile::listing(d, 10) {
            let _ = writeln!(md, "- [{}] «{}» → «{}» (before: «{}»), +{}, {} of {}", profile::LEVEL_NAMES[l], key, o.chunk, o.was, o.gain, o.n, o.total);
        }
    }
    // lexicon
    let _ = writeln!(md, "\n## The interlingua lexicon (largest concepts)\n");
    let mut cl: Vec<(&String, &Vec<(String, u32)>)> = info.lex.clusters.iter().collect();
    cl.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(b.0)));
    for (c, ms) in cl.iter().take(30) {
        let _ = writeln!(md, "- **{c}**: {}", ms.iter().map(|(m, n)| format!("{m} ({n})")).collect::<Vec<_>>().join(", "));
    }
    // false friends
    let _ = writeln!(md, "\n## False friends across centuries (KJV's word on the training verses)\n\nAnchored to itself by the longest common subsequence, or standing where the two differ, and what the other writes there; the concept the lexicon reads it as.\n");
    let _ = writeln!(md, "| word | against | concept | anchored to itself | where they differ | the other writes |");
    let _ = writeln!(md, "|---|---|---|---:|---:|---|");
    for r in &info.false_friends {
        let _ = writeln!(md, "{r}");
    }
    // largest classes for the main systems
    let _ = writeln!(md, "\n## Largest residual classes\n");
    for g in aggs.iter().filter(|g| matches!(g.name.as_str(), "jps → kjv" | "kjv → web" | "jps → web" | "wlc → jps")) {
        let _ = writeln!(md, "**{}** (bits per verse; stage/class: examples)\n", g.name);
        let mut v: Vec<(f64, usize, Class, u64)> = Vec::new();
        for s in 0..3 {
            for (c, (b, n)) in by_class(&g.res[s]) {
                v.push((b, s, c, n));
            }
        }
        v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap().then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
        for (b, s, c, n) in v.iter().take(8) {
            let ex: Vec<String> = g.res[*s].cells.iter().filter(|(k, _)| k.1 == *c).flat_map(|(_, cell)| cell.examples.iter().take(2).cloned()).take(2).collect();
            let _ = writeln!(md, "- {} / {}: {:.1} bits, {:.2} items per verse — {}", STAGES[*s], c.name(), per(*b, g.verses), per(*n as f64, g.verses), ex.join(" · "));
        }
        let _ = writeln!(md);
    }
    // normal mode divergence
    let _ = writeln!(md, "## Normal mode: divergence from the target, words per 100 target tokens\n");
    let shown: Vec<Class> = CLASSES.iter().copied().filter(|c| aggs.iter().any(|g| g.div.cells.get(c).is_some_and(|x| x.items > 0))).collect();
    let _ = writeln!(md, "| system | {} |", shown.iter().map(|c| c.name()).collect::<Vec<_>>().join(" | "));
    let _ = writeln!(md, "|---|{}", "---:|".repeat(shown.len()));
    for g in aggs.iter().filter(|g| g.group != "autoencoder") {
        let _ = writeln!(md, "| {} | {} |", g.name, shown.iter().map(|c| format!("{:.1}", 100.0 * g.div.cells.get(c).map_or(0, |x| x.items) as f64 / g.tokens.max(1) as f64)).collect::<Vec<_>>().join(" | "));
    }
    // gate failures
    let _ = writeln!(md, "\n## Gate failures\n");
    for g in aggs {
        for f in &g.fails {
            let _ = writeln!(md, "- {}: {}", g.name, f);
        }
    }
    std::fs::write(out.join("report.md"), &md)?;
    // examples
    let mut ex = String::from("# Examples\n\n");
    for g in aggs {
        let _ = writeln!(ex, "## {}\n", g.name);
        for (k, src, o, t, why) in &g.examples {
            let _ = writeln!(ex, "- **{k}**\n  - source: {src}\n  - output: {o}\n  - target: {t}");
            for w in why.iter().take(14) {
                let _ = writeln!(ex, "    - {w}");
            }
        }
        let _ = writeln!(ex);
    }
    std::fs::write(out.join("examples.md"), ex)?;
    // numbers
    let mut tsv = String::from("system\tgroup\tverses\tgate\texact\tchrf\tstage\tclass\tbits\titems\n");
    for g in aggs {
        for s in 0..3 {
            for (c, (b, n)) in by_class(&g.res[s]) {
                let _ = writeln!(tsv, "{}\t{}\t{}\t{}\t{}\t{:.3}\t{}\t{}\t{:.1}\t{}", g.name, g.group, g.verses, g.gate_ok, g.exact, per(g.chrf, g.verses), STAGES[s], c.name(), b, n);
            }
        }
    }
    std::fs::write(out.join("numbers.tsv"), tsv)?;
    std::fs::write(out.join("lexicon.tsv"), info.lex.to_tsv())?;
    let mut dl = String::from("author\tlevel\tcontext\tchunk\tstandard\tgain\tn\ttotal\n");
    for d in info.deltas {
        for (l, m) in d.levels.iter().enumerate() {
            for (k, o) in m {
                let _ = writeln!(dl, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", d.name, profile::LEVEL_NAMES[l], k, o.chunk, o.was, o.gain, o.n, o.total);
            }
        }
    }
    std::fs::write(out.join("deltas.tsv"), dl)?;
    Ok(())
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn class_conservation_and_zero_denominator() {
        use roundtrip::class::Layer;
        let mut r = Residual::default();
        r.add(Layer::World, Class::Lexical, 3.0, String::new);
        r.add(Layer::Tokens, Class::Spacing, 2.0, String::new);
        assert_eq!(groups_of(&[&r]), [3.0, 0.0, 0.0, 0.0, 2.0, 0.0]);
        assert_eq!(bits(&r), 5.0);
        assert_eq!(by_class(&r)[&Class::Lexical], (3.0, 1));
        assert_eq!((per(5.0, 0), per(5.0, 2)), (5.0, 2.5));
    }
}
