//! The report of the meta layer test.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

use super::link::LinkMode;
use super::route::Mode;
use super::sample::{Stream, VARIETIES};
use super::{Ctx, Run};
use crate::class::{Class, Layer};
use crate::tally::Start;


fn find<'a>(runs: &'a [(String, Mode, LinkMode, Run)], stream: &str, m: Mode, l: LinkMode) -> Option<&'a Run> {
    runs.iter().find(|r| r.0 == stream && r.1 == m && r.2 == l).map(|r| &r.3)
}

/// All the mono streams of a mode as one run.
fn mono(runs: &[(String, Mode, LinkMode, Run)], m: Mode) -> Run {
    let mut out = Run::default();
    for r in runs.iter().filter(|r| r.0.starts_with("mono-") && r.1 == m) {
        out.merge(&r.3);
    }
    out
}

fn bits(r: &Run, v: &str, l: Option<Layer>) -> f64 {
    let n = r.items.get(v).copied().unwrap_or(0);
    if n == 0 {
        return f64::NAN;
    }
    let b: f64 = r.by.get(v).map(|t| t.residual.cells.iter().filter(|(k, _)| k.1 != Class::Meta && l.is_none_or(|l| k.0 == l)).map(|(_, c)| c.bits).sum()).unwrap_or(0.0);
    b / n as f64 + 0.0
}


fn chrf(r: &Run, v: &str, s: Start) -> f64 {
    r.by.get(v).and_then(|t| t.normal.get(&s)).filter(|d| d.sents > 0).map_or(f64::NAN, |d| 100.0 * d.chrf / d.sents as f64)
}

fn f(x: f64) -> String {
    if x.is_nan() { "—".into() } else { format!("{x:.1}") }
}

fn acc(r: &Run, v: &str, code: &str) -> (f64, f64, u64) {
    let tot: u64 = r.conf.iter().filter(|(k, _)| k.0 == v).map(|(_, n)| n).sum();
    let ok: u64 = r.conf.get(&(v.to_string(), code.to_string())).copied().unwrap_or(0);
    let (ro, rt) = r.route_ok.get(v).copied().unwrap_or((0, 0));
    if tot == 0 {
        return (f64::NAN, f64::NAN, 0);
    }
    (100.0 * ok as f64 / tot as f64, 100.0 * ro as f64 / rt.max(1) as f64, tot)
}

pub fn report(ctx: &Ctx, streams: &[Stream], runs: &[(String, Mode, LinkMode, Run)], n: usize, seed: u64, vol: usize) -> String {
    let mut s = String::new();
    let al = LinkMode::Align;
    let m_l = mono(runs, Mode::Langseg);
    let m_o = mono(runs, Mode::Oracle);
    let empty = Run::default();
    let get = |st: &str, m: Mode| find(runs, st, m, al).unwrap_or(&empty);
    let (l_l, l_o, l_e, l_s) = (get("lines", Mode::Langseg), get("lines", Mode::Oracle), get("lines", Mode::AllEn), get("lines", Mode::Shuffle));
    let (d_l, d_o, d_e, d_s) = (get("docs", Mode::Langseg), get("docs", Mode::Oracle), get("docs", Mode::AllEn), get("docs", Mode::Shuffle));
    let vs: Vec<&str> = VARIETIES.iter().map(|v| v.name).filter(|v| m_l.items.contains_key(*v)).collect();
    let _ = writeln!(s, "# The meta layer test: mixed languages and styles\n");
    let _ = writeln!(s, "Seed {seed}; at most {n} sentences a variety; volumes (one world each) of {vol} lines. Streams: {}.\n", streams.iter().map(|x| format!("`{}` ({} lines)", x.name, x.lines.len())).collect::<Vec<_>>().join(", "));
    // the sample
    let _ = writeln!(s, "## The sample\n\n| variety | source | route | sentences | pieces' sentences | tokens |\n|---|---|---|---:|---:|---:|");
    for v in &vs {
        let var = &VARIETIES[super::sample::variety(v).unwrap()];
        let t = m_o.by.get(*v).cloned().unwrap_or_default();
        let _ = writeln!(s, "| {v} | {} | {} | {} | {} | {} |", var.source, var.route.name(), m_o.items.get(*v).unwrap_or(&0), t.sents, t.tokens);
    }
    // the gate
    let _ = writeln!(s, "\n## Exact mode: the gate\n\nVolumes that came back byte for byte (every line, and on the way down every piece's frames, tree and tokens).\n\n| stream | langseg | oracle | all-en | shuffle | langseg, link-any |\n|---|---:|---:|---:|---:|---:|");
    let g = |r: &Run| if r.volumes == 0 { "—".to_string() } else { format!("{}/{}", r.exact_ok, r.volumes) };
    let _ = writeln!(s, "| mono (all) | {} | {} | — | — | — |", g(&m_l), g(&m_o));
    for st in ["lines", "docs"] {
        let _ = writeln!(s, "| {st} | {} | {} | {} | {} | {} |", g(get(st, Mode::Langseg)), g(get(st, Mode::Oracle)), g(get(st, Mode::AllEn)), g(get(st, Mode::Shuffle)), g(find(runs, st, Mode::Langseg, LinkMode::Any).unwrap_or(&empty)));
    }
    let fails: Vec<&String> = runs.iter().flat_map(|r| r.3.fails.iter()).take(6).collect();
    if !fails.is_empty() {
        let _ = writeln!(s, "\nFirst failures: {}", fails.iter().map(|x| format!("`{x}`")).collect::<Vec<_>>().join("; "));
    }
    // routing
    let _ = writeln!(s, "\n## Routing\n\nShare of langseg's tokens (words) given the variety's language / sent to the variety's route, by gold variety.\n\n| variety | tokens (lines) | mono lang | mono route | lines lang | lines route | docs lang | docs route | all-en route | shuffle route |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    let mut macro_: BTreeMap<&str, (f64, usize)> = BTreeMap::new();
    for v in &vs {
        let code = VARIETIES[super::sample::variety(v).unwrap()].lang.code();
        let (a1, r1, _) = acc(&m_l, v, code);
        let (a2, r2, n2) = acc(l_l, v, code);
        let (a3, r3, _) = acc(d_l, v, code);
        let (_, r4, _) = acc(l_e, v, code);
        let (_, r5, _) = acc(l_s, v, code);
        for (k, x) in [("ml", a1), ("mr", r1), ("ll", a2), ("lr", r2), ("dl", a3), ("dr", r3), ("er", r4), ("sr", r5)] {
            if !x.is_nan() {
                let e = macro_.entry(k).or_default();
                e.0 += x;
                e.1 += 1;
            }
        }
        let _ = writeln!(s, "| {v} | {n2} | {} | {} | {} | {} | {} | {} | {} | {} |", f(a1), f(r1), f(a2), f(r2), f(a3), f(r3), f(r4), f(r5));
    }
    let mac = |k: &str| macro_.get(k).map_or(f64::NAN, |x| x.0 / x.1 as f64);
    let _ = writeln!(s, "| **macro** | | **{}** | **{}** | **{}** | **{}** | **{}** | **{}** | **{}** | **{}** |", f(mac("ml")), f(mac("mr")), f(mac("ll")), f(mac("lr")), f(mac("dl")), f(mac("dr")), f(mac("er")), f(mac("sr")));
    // confusion
    let mut conf: BTreeMap<(String, String), u64> = BTreeMap::new();
    for r in [l_l, d_l] {
        for (k, n) in &r.conf {
            *conf.entry(k.clone()).or_default() += n;
        }
    }
    let codes: BTreeSet<String> = conf.keys().map(|k| k.1.clone()).collect();
    let _ = writeln!(s, "\nConfusion, lines and docs with langseg (tokens; rows gold, columns predicted):\n\n| variety | {} |\n|---|{}", codes.iter().cloned().collect::<Vec<_>>().join(" | "), "---:|".repeat(codes.len()));
    for v in &vs {
        let row: Vec<String> = codes.iter().map(|c| conf.get(&(v.to_string(), c.clone())).map_or(String::new(), |n| n.to_string())).collect();
        let _ = writeln!(s, "| {v} | {} |", row.join(" | "));
    }
    // residual
    let _ = writeln!(s, "\n## Residual per sentence (bits), by layer — `lines` with langseg\n\nA sentence is one sampled item; a piece counts for the item that holds most of its bytes. The joints between pieces (text ↔ pieces) are below the table.\n\n| variety | route | text ↔ tokens | tokens ↔ tree | tree ↔ frames | frames ↔ world | all, lines | all, mono |\n|---|---|---:|---:|---:|---:|---:|---:|");
    for v in &vs {
        let r = VARIETIES[super::sample::variety(v).unwrap()].route.name();
        let _ = writeln!(s, "| {v} | {r} | {} | {} | {} | {} | **{}** | {} |", f(bits(l_l, v, Some(Layer::Tokens))), f(bits(l_l, v, Some(Layer::Tree))), f(bits(l_l, v, Some(Layer::Frames))), f(bits(l_l, v, Some(Layer::World))), f(bits(l_l, v, None)), f(bits(&m_l, v, None)));
    }
    for (name, r) in [("lines", l_l), ("docs", d_l)] {
        let gl = r.by.get("glue").map(|t| t.residual.cells.iter().filter(|(k, _)| k.0 == Layer::Route).map(|(_, c)| c.bits).sum::<f64>()).unwrap_or(0.0);
        let _ = writeln!(s, "\nJoints of `{name}` (text ↔ pieces): {:.2} bits a line over {} lines.", gl / r.lines.max(1) as f64 + 0.0, r.lines);
    }
    // normal mode
    let _ = writeln!(s, "\n## Normal mode: chrF of the text decoded from each start\n\nOnly the sentences a parser read (English and Hebrew routes): an Aramaic sentence routed to Hebrew counts here, the rest of Aramaic has no tree.\n\n| variety | sentences read (lines) | tree, mono | tree, lines | frames, mono | frames, lines | world, mono | world, lines | world, docs |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|");
    for v in &vs {
        let nread = l_l.by.get(*v).and_then(|t| t.normal.get(&Start::Tree)).map_or(0, |d| d.sents);
        let _ = writeln!(s, "| {v} | {nread} | {} | {} | {} | {} | {} | {} | {} |", f(chrf(&m_l, v, Start::Tree)), f(chrf(l_l, v, Start::Tree)), f(chrf(&m_l, v, Start::Frames)), f(chrf(l_l, v, Start::Frames)), f(chrf(&m_l, v, Start::World)), f(chrf(l_l, v, Start::World)), f(chrf(d_l, v, Start::World)));
    }
    // the world
    let _ = writeln!(s, "\n## One world: entities joined across languages\n\n| run | aligned pairs | verse links | name links | right | wrong | not verified | missed |\n|---|---:|---:|---:|---:|---:|---:|---:|");
    for (name, r) in [("lines, langseg", l_l), ("docs, langseg", d_l), ("docs, oracle", d_o), ("docs, langseg, link-any (control)", find(runs, "docs", Mode::Langseg, LinkMode::Any).unwrap_or(&empty)), ("lines, langseg, link-any (control)", find(runs, "lines", Mode::Langseg, LinkMode::Any).unwrap_or(&empty))] {
        let k = &r.links;
        let _ = writeln!(s, "| {name} | {} | {} | {} | {} | {} | {} | {} |", k.pairs, k.verse_links, k.name_links, k.right, k.wrong, k.unverified, k.missed);
    }
    if !d_l.links.examples.is_empty() {
        let _ = writeln!(s, "\nExamples (docs, langseg; all in `links.tsv`): {}", d_l.links.examples.iter().take(12).map(|x| format!("`{x}`")).collect::<Vec<_>>().join(" · "));
    }
    // the cost of mixing
    // the cost of mixing, item by item
    let _ = writeln!(s, "\n## The cost of mixing\n\nAll residual bits per sentence (item) of a variety, routed by the gold (oracle) or by langseg: alone (`mono`), shuffled with every other variety (`lines`, the same items), and in documents (`docs`: a quote is a whole item; an English carrier is cut by its quote), where each item is compared with itself in `mono`. Δ context = lines − mono with the oracle (the discourse and the world shared with other languages); Δ routing = what langseg adds in `lines` over the oracle, less what it adds in `mono`.\n\n| variety | mono, oracle | mono, langseg | lines, oracle | lines, langseg | Δ context | Δ routing | docs items | docs, langseg | the same in mono | Δ docs | Δ docs, oracle |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for v in &vs {
        let (a, b, c, d) = (bits(&m_o, v, None), bits(&m_l, v, None), bits(l_o, v, None), bits(l_l, v, None));
        let (ds, dm, dn) = paired(ctx, d_l, &m_l, v, |_| true);
        let (os, om, _) = paired(ctx, d_o, &m_o, v, |_| true);
        let _ = writeln!(s, "| {v} | {} | {} | {} | {} | {} | {} | {dn} | {} | {} | {} | {} |", f(a), f(b), f(c), f(d), f(c - a), f((d - c) - (b - a)), f(ds), f(dm), f(ds - dm), f(os - om));
    }
    // controls
    let _ = writeln!(s, "\n## Negative controls\n\nIn `lines` (one item a line, so every bit is its item's): all residual bits per sentence, and chrF from the tree and from the frames — langseg against everything sent to English (`all-en`) and against langseg's languages shuffled over the segments of a volume (`shuffle`). A control must lose.\n\n| variety | bits langseg | bits all-en | bits shuffle | tree chrF langseg | all-en | shuffle | frames chrF langseg | all-en | shuffle |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for v in &vs {
        let _ = writeln!(s, "| {v} | {} | {} | {} | {} | {} | {} | {} | {} | {} |", f(bits(l_l, v, None)), f(bits(l_e, v, None)), f(bits(l_s, v, None)), f(chrf(l_l, v, Start::Tree)), f(chrf(l_e, v, Start::Tree)), f(chrf(l_s, v, Start::Tree)), f(chrf(l_l, v, Start::Frames)), f(chrf(l_e, v, Start::Frames)), f(chrf(l_s, v, Start::Frames)));
    }
    let per_line = |r: &Run| -> f64 { r.by.values().flat_map(|t| t.residual.cells.iter()).filter(|(k, _)| k.1 != Class::Meta).map(|(_, c)| c.bits).sum::<f64>() / r.lines.max(1) as f64 };
    let _ = writeln!(s, "\nAll residual bits per line, every language together: `lines` langseg {} / oracle {} / all-en {} / shuffle {}; `docs` langseg {} / oracle {} / all-en {} / shuffle {}.", f(per_line(l_l)), f(per_line(l_o)), f(per_line(l_e)), f(per_line(l_s)), f(per_line(d_l)), f(per_line(d_o)), f(per_line(d_e)), f(per_line(d_s)));
    // the ablation of the cut at quote marks
    if let Some(nc) = find(runs, "docs:no-quote-cut", Mode::Langseg, al) {
        let _ = writeln!(s, "\n## Ablation: segments cut at a document's quote marks\n\nlangseg reads a Hebrew-script run whole, so two quotes side by side («…» (…)) can come out as one segment; the meta layer cuts segments at « » “ ” ( ) [ ]. Bits per sentence in `docs` (langseg), each item against itself in `mono`; all bits per document: with the cut {}, without {}.\n\n| variety | docs items | Δ docs, cut | Δ docs, not cut |\n|---|---:|---:|---:|", f(per_line(d_l)), f(per_line(nc)));
        for v in &vs {
            let (a, b, n1) = paired(ctx, d_l, &m_l, v, |_| true);
            let (c, d, _) = paired(ctx, nc, &m_l, v, |_| true);
            let _ = writeln!(s, "| {v} | {n1} | {} | {} |", f(a - b), f(c - d));
        }
    }
    // where the meta layer leaks
    let _ = writeln!(s, "\n## Where the meta layer leaks\n\nThe (variety, layer, class) cells whose bits per sentence grow most from `mono` to `lines` and to `docs` (langseg), each item against itself; the classes are those of the single-language round trip.\n");
    let mut leaks: Vec<(f64, String)> = Vec::new();
    for v in &vs {
        for (name, r) in [("lines", l_l), ("docs", d_l)] {
            let mut keys: BTreeSet<(Layer, Class)> = BTreeSet::new();
            for (k, (_, cells)) in r.item.iter().chain(m_l.item.iter()) {
                if ctx.var_of(*k) == *v {
                    keys.extend(cells.keys().copied());
                }
            }
            for key in keys {
                if key.1 == Class::Meta {
                    continue;
                }
                let (a, b, n) = paired(ctx, r, &m_l, v, |k| k == key);
                let dlt = a - b;
                if n > 0 && dlt.abs() >= 0.5 {
                    leaks.push((dlt, format!("{v} | {name} | {n} | {} | {} | {:+.1}", key.0.boundary(), key.1.name(), dlt)));
                }
            }
        }
    }
    leaks.sort_by(|a, b| b.0.abs().total_cmp(&a.0.abs()).then(a.1.cmp(&b.1)));
    let _ = writeln!(s, "| variety | stream | items | layer | class | Δ bits / sentence |\n|---|---|---:|---|---|---:|");
    for (_, l) in leaks.iter().take(30) {
        let _ = writeln!(s, "| {l} |");
    }
    s
}

/// The mean bits per item of the items of a variety in a run, and of the same items in a base run (the cells chosen).
fn paired(ctx: &Ctx, r: &Run, base: &Run, v: &str, keep: impl Fn((Layer, Class)) -> bool) -> (f64, f64, usize) {
    let sum = |cells: &BTreeMap<(Layer, Class), f64>| -> f64 { cells.iter().filter(|(k, _)| k.1 != Class::Meta && keep(**k)).map(|(_, b)| b).sum() };
    let (mut a, mut b, mut n) = (0.0, 0.0, 0usize);
    for (k, (occ, cells)) in &r.item {
        if ctx.var_of(*k) != v {
            continue;
        }
        let Some((bo, bc)) = base.item.get(k) else { continue };
        a += sum(cells) / (*occ).max(1) as f64;
        b += sum(bc) / (*bo).max(1) as f64;
        n += 1;
    }
    if n == 0 { (f64::NAN, f64::NAN, 0) } else { (a / n as f64 + 0.0, b / n as f64 + 0.0, n) }
}

/// Machine-readable: run, mode, link mode, variety, items, layer, class, residual items, bits.
pub fn tsv(runs: &[(String, Mode, LinkMode, Run)]) -> String {
    let mut s = String::from("stream\tmode\tlink\tvariety\titems\tlayer\tclass\tresidual_items\tbits\n");
    for (st, m, l, r) in runs {
        for (v, t) in &r.by {
            let n = r.items.get(v).copied().unwrap_or(0);
            for ((ly, c), cell) in &t.residual.cells {
                let _ = writeln!(s, "{st}\t{}\t{:?}\t{v}\t{n}\t{}\t{}\t{}\t{:.1}", m.name(), l, ly.name(), c.name(), cell.items, cell.bits);
            }
        }
    }
    s
}

/// Per item: run, mode, link mode, item id, variety, times read, bits per layer (route, tokens, tree, frames, world).
pub fn items_tsv(ctx: &Ctx, runs: &[(String, Mode, LinkMode, Run)]) -> String {
    let mut s = String::from("stream\tmode\tlink\titem\tvariety\tread\troute\ttokens\ttree\tframes\tworld\n");
    let layers = [Layer::Route, Layer::Tokens, Layer::Tree, Layer::Frames, Layer::World];
    for (st, m, l, r) in runs {
        for (k, (occ, cells)) in &r.item {
            let per: Vec<String> = layers.iter().map(|ly| format!("{:.1}", cells.iter().filter(|(key, _)| key.0 == *ly && key.1 != Class::Meta).map(|(_, b)| b).sum::<f64>() / (*occ).max(1) as f64 + 0.0)).collect();
            let _ = writeln!(s, "{st}\t{}\t{l:?}\t{}\t{}\t{occ}\t{}", m.name(), ctx.items[*k].id, ctx.var_of(*k), per.join("\t"));
        }
    }
    s
}
#[cfg(test)]
mod wave32_tests {
    use super::*;
    #[test]
    fn denominators_and_meta() {
        let mut r = Run::default();
        assert!(bits(&r, "v", None).is_nan());
        r.items.insert("v".into(), 2);
        let t = r.by.entry("v".into()).or_default();
        t.residual
            .add(Layer::Tokens, Class::Spacing, 8.0, || "gap".into());
        t.residual
            .add(Layer::Tokens, Class::Meta, 100.0, || "meta".into());
        assert_eq!(bits(&r, "v", None), 4.0);
        assert_eq!(bits(&r, "v", Some(Layer::Tree)), 0.0);
        assert_eq!(f(f64::NAN), "—");
        r.conf.insert(("v".into(), "en".into()), 3);
        r.route_ok.insert("v".into(), (2, 4));
        assert_eq!(acc(&r, "v", "en"), (100.0, 50.0, 3));
        let out = tsv(&[("lines".into(), Mode::Oracle, LinkMode::Align, r)]);
        assert!(out.contains("meta"));
        assert!(out.contains("100.0"));
    }
}
