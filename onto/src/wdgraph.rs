//! Wikidata hierarchy in Rust: «is a kind of» edges — P279 (subclass of) and P171 (parent taxon), cycles, closure.
//!
//! - **Edges** come from the derived `claims-*.tsv.zst` (rank not deprecated). Taxa in Wikidata go via
//!   P171, not P279: without P171 the whale will not reach mammals.
//! - **Cycles** — strongly connected components (Tarjan, iterative) and self-loops. The gate lists them rather than
//!   hiding them: edges inside the components are removed from the closure and written out; after breaking, the graph must be
//!   acyclic (Kahn's check). Negative control — an injected edge that closes a new cycle.
//! - **Closure** — BFS upward from each node in parallel; the shortest depth.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed};
use std::time::Instant;

use anyhow::{bail, Context, Result};

use crate::wdx::{thousands, Tsv};

/// Kind of an «is a kind of» edge.
pub const P279: u16 = 279;
pub const P171: u16 = 171;

/// «Is a kind of» graph: nodes are sorted Q numbers, edges are CSR upward (subclass → class).
#[derive(Default)]
pub struct IsaGraph {
    pub nodes: Vec<u32>,
    pub off: Vec<u32>,
    pub up: Vec<u32>,
    pub kind: Vec<u16>,
}

impl IsaGraph {
    /// From edges (s, o, p) in Q numbers; duplicates of identical (s, o, p) are merged.
    pub fn from_edges(mut edges: Vec<(u32, u32, u16)>) -> IsaGraph {
        edges.sort_unstable();
        edges.dedup();
        let mut nodes: Vec<u32> = edges.iter().flat_map(|e| [e.0, e.1]).collect();
        nodes.sort_unstable();
        nodes.dedup();
        let n = nodes.len();
        let idx = |q: u32| nodes.binary_search(&q).expect("node") as u32;
        let mut off = vec![0u32; n + 1];
        for e in &edges {
            off[idx(e.0) as usize + 1] += 1;
        }
        for i in 0..n {
            off[i + 1] += off[i];
        }
        let mut up = vec![0u32; edges.len()];
        let mut kind = vec![0u16; edges.len()];
        let mut fill = off.clone();
        for e in &edges {
            let s = idx(e.0) as usize;
            let k = fill[s] as usize;
            up[k] = idx(e.1);
            kind[k] = e.2;
            fill[s] += 1;
        }
        IsaGraph { nodes, off, up, kind }
    }

    pub fn n(&self) -> usize {
        self.nodes.len()
    }

    pub fn idx(&self, q: u32) -> Option<u32> {
        self.nodes.binary_search(&q).ok().map(|i| i as u32)
    }

    pub fn ups(&self, v: u32) -> std::ops::Range<usize> {
        self.off[v as usize] as usize..self.off[v as usize + 1] as usize
    }

    pub fn n_edges(&self) -> usize {
        self.up.len()
    }

    /// Edges back in Q numbers.
    pub fn edges(&self) -> Vec<(u32, u32, u16)> {
        let mut v = Vec::with_capacity(self.up.len());
        for s in 0..self.n() as u32 {
            for k in self.ups(s) {
                v.push((self.nodes[s as usize], self.nodes[self.up[k] as usize], self.kind[k]));
            }
        }
        v
    }

    /// Ancestors of a node (BFS upward): (node, shortest depth), without the node itself; `max_depth` — limit.
    pub fn ancestors(&self, v: u32, max_depth: u16) -> Vec<(u32, u16)> {
        let mut seen: HashMap<u32, u16> = HashMap::new();
        seen.insert(v, 0);
        let mut q = std::collections::VecDeque::from([v]);
        let mut out = Vec::new();
        while let Some(x) = q.pop_front() {
            let d = seen[&x];
            if d >= max_depth {
                continue;
            }
            for k in self.ups(x) {
                let w = self.up[k];
                if let std::collections::hash_map::Entry::Vacant(e) = seen.entry(w) {
                    e.insert(d + 1);
                    out.push((w, d + 1));
                    q.push_back(w);
                }
            }
        }
        out
    }
}

/// Strongly connected components (Tarjan, no recursion): component number of each node.
pub fn scc(g: &IsaGraph) -> (Vec<u32>, u32) {
    let n = g.n();
    const NONE: u32 = u32::MAX;
    let mut index = vec![NONE; n];
    let mut low = vec![0u32; n];
    let mut on_stack = vec![false; n];
    let mut stack: Vec<u32> = Vec::new();
    let mut comp = vec![NONE; n];
    let mut ncomp = 0u32;
    let mut next = 0u32;
    let mut call: Vec<(u32, u32)> = Vec::new();
    for s in 0..n as u32 {
        if index[s as usize] != NONE {
            continue;
        }
        index[s as usize] = next;
        low[s as usize] = next;
        next += 1;
        stack.push(s);
        on_stack[s as usize] = true;
        call.push((s, g.off[s as usize]));
        while let Some(top) = call.last_mut() {
            let v = top.0;
            if top.1 < g.off[v as usize + 1] {
                let w = g.up[top.1 as usize];
                top.1 += 1;
                if index[w as usize] == NONE {
                    index[w as usize] = next;
                    low[w as usize] = next;
                    next += 1;
                    stack.push(w);
                    on_stack[w as usize] = true;
                    call.push((w, g.off[w as usize]));
                } else if on_stack[w as usize] {
                    low[v as usize] = low[v as usize].min(index[w as usize]);
                }
            } else {
                call.pop();
                if let Some(&(u, _)) = call.last() {
                    low[u as usize] = low[u as usize].min(low[v as usize]);
                }
                if low[v as usize] == index[v as usize] {
                    loop {
                        let w = stack.pop().expect("Tarjan stack");
                        on_stack[w as usize] = false;
                        comp[w as usize] = ncomp;
                        if w == v {
                            break;
                        }
                    }
                    ncomp += 1;
                }
            }
        }
    }
    (comp, ncomp)
}

/// A cycle found by the gate: component members (Q numbers) and the edges inside it.
#[derive(Clone, Debug)]
pub struct Cycle {
    pub members: Vec<u32>,
    pub edges: Vec<(u32, u32, u16)>,
}

/// Cycle gate: components with more than one node, and self-loops. Returns the cycles and the graph without their internal edges.
pub fn break_cycles(g: &IsaGraph) -> (Vec<Cycle>, IsaGraph) {
    let (comp, ncomp) = scc(g);
    let mut size = vec![0u32; ncomp as usize];
    for &c in &comp {
        size[c as usize] += 1;
    }
    let mut cyc: HashMap<u32, Cycle> = HashMap::new();
    let mut keep = Vec::with_capacity(g.n_edges());
    for s in 0..g.n() as u32 {
        for k in g.ups(s) {
            let o = g.up[k];
            let e = (g.nodes[s as usize], g.nodes[o as usize], g.kind[k]);
            let (cs, co) = (comp[s as usize], comp[o as usize]);
            if cs == co && (size[cs as usize] > 1 || s == o) {
                let c = cyc.entry(cs).or_insert_with(|| Cycle { members: Vec::new(), edges: Vec::new() });
                c.edges.push(e);
            } else {
                keep.push(e);
            }
        }
    }
    for s in 0..g.n() {
        if let Some(c) = cyc.get_mut(&comp[s]) {
            c.members.push(g.nodes[s]);
        }
    }
    let mut v: Vec<Cycle> = cyc.into_values().collect();
    for c in &mut v {
        c.members.sort_unstable();
        c.edges.sort_unstable();
    }
    v.sort_by(|a, b| b.members.len().cmp(&a.members.len()).then(a.members.cmp(&b.members)));
    // nodes that had only internal cycle edges disappear from the new graph — they have no ancestors after breaking;
    // the new graph has its own indices, the link between graphs is only via Q numbers
    (v, IsaGraph::from_edges(keep))
}

/// Kahn's check: `None` — acyclic, otherwise — how many nodes remain in cycles.
pub fn acyclic(g: &IsaGraph) -> Option<usize> {
    let n = g.n();
    let mut indeg = vec![0u32; n];
    for &w in &g.up {
        indeg[w as usize] += 1;
    }
    let mut q: Vec<u32> = (0..n as u32).filter(|&v| indeg[v as usize] == 0).collect();
    let mut done = 0usize;
    while let Some(v) = q.pop() {
        done += 1;
        for k in g.ups(v) {
            let w = g.up[k] as usize;
            indeg[w] -= 1;
            if indeg[w] == 0 {
                q.push(w as u32);
            }
        }
    }
    if done == n { None } else { Some(n - done) }
}

/// «Is a kind of» edges from the derived `claims-*.tsv.zst`: P279 and P171, rank ≥ 1. In parallel over files.
pub fn read_isa_edges(dir: &Path) -> Result<(Vec<(u32, u32, u16)>, u64)> {
    let files = parts(dir, "claims")?;
    let results: Vec<Result<(Vec<(u32, u32, u16)>, u64)>> = std::thread::scope(|sc| {
        let hs: Vec<_> = files
            .iter()
            .map(|p| {
                sc.spawn(move || -> Result<(Vec<(u32, u32, u16)>, u64)> {
                    let mut v = Vec::new();
                    let mut dep = 0u64;
                    for_each_row(p, |f| {
                        if f.len() < 4 {
                            return Ok(());
                        }
                        let pp: u16 = f[1].parse()?;
                        if pp != P279 && pp != P171 {
                            return Ok(());
                        }
                        if f[3] == "0" {
                            dep += 1;
                            return Ok(());
                        }
                        v.push((f[0].parse()?, f[2].parse()?, pp));
                        Ok(())
                    })?;
                    Ok((v, dep))
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap_or_else(|_| Err(anyhow::anyhow!("reader thread panicked")))).collect()
    });
    let mut all = Vec::new();
    let mut dep = 0;
    for r in results {
        let (v, d) = r?;
        all.extend(v);
        dep += d;
    }
    Ok((all, dep))
}

/// Table part files: `<table>-w*.tsv.zst`, sorted.
pub fn parts(dir: &Path, table: &str) -> Result<Vec<PathBuf>> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("no folder {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            let n = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            n.starts_with(&format!("{table}-w")) && n.ends_with(".tsv.zst")
        })
        .collect();
    v.sort();
    if v.is_empty() {
        bail!("no files {table}-w*.tsv.zst in {}", dir.display());
    }
    Ok(v)
}

/// Each line of a compressed TSV — fields.
pub fn for_each_row(p: &Path, mut f: impl FnMut(&[&str]) -> Result<()>) -> Result<()> {
    let dec = zstd::stream::read::Decoder::new(File::open(p).with_context(|| format!("cannot open {}", p.display()))?)?;
    let mut rd = BufReader::with_capacity(1 << 20, dec);
    let mut line = String::new();
    loop {
        line.clear();
        if rd.read_line(&mut line)? == 0 {
            break;
        }
        let l = line.trim_end_matches('\n');
        let fl: Vec<&str> = l.split('\t').collect();
        f(&fl).with_context(|| format!("line «{}» in {}", l.chars().take(80).collect::<String>(), p.display()))?;
    }
    Ok(())
}

/// Labels and popularity of items from `items-*.tsv.zst` — only for the needed Qs.
pub struct ItemInfo {
    pub label: HashMap<u32, String>,
    pub desc: HashMap<u32, String>,
    pub n_wiki: HashMap<u32, u16>,
    pub flags: HashMap<u32, u32>,
}

pub fn read_items(dir: &Path, want: &(dyn Fn(u32) -> bool + Sync), with_text: &(dyn Fn(u32) -> bool + Sync)) -> Result<ItemInfo> {
    let files = parts(dir, "items")?;
    let res: Vec<Result<ItemInfo>> = std::thread::scope(|sc| {
        let hs: Vec<_> = files
            .iter()
            .map(|p| {
                sc.spawn(move || -> Result<ItemInfo> {
                    let mut ii = ItemInfo { label: HashMap::new(), desc: HashMap::new(), n_wiki: HashMap::new(), flags: HashMap::new() };
                    for_each_row(p, |f| {
                        if f.len() < 10 {
                            return Ok(());
                        }
                        let q: u32 = f[0].parse()?;
                        if !want(q) {
                            return Ok(());
                        }
                        ii.n_wiki.insert(q, f[7].parse()?);
                        ii.flags.insert(q, f[9].parse()?);
                        if with_text(q) {
                            let lab = if !f[1].is_empty() { f[1] } else { f[2] };
                            ii.label.insert(q, lab.to_string());
                            ii.desc.insert(q, f[4].to_string());
                        }
                        Ok(())
                    })?;
                    Ok(ii)
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap_or_else(|_| Err(anyhow::anyhow!("thread panicked")))).collect()
    });
    let mut out = ItemInfo { label: HashMap::new(), desc: HashMap::new(), n_wiki: HashMap::new(), flags: HashMap::new() };
    for r in res {
        let r = r?;
        out.label.extend(r.label);
        out.desc.extend(r.desc);
        out.n_wiki.extend(r.n_wiki);
        out.flags.extend(r.flags);
    }
    Ok(out)
}

/// Closure statistics: (sum of ancestors, largest count, greatest depth, distribution by depth).
pub struct ClosureStats {
    pub pairs: u64,
    pub max_anc: u32,
    pub max_anc_q: u32,
    pub max_depth: u16,
    pub by_depth: Vec<u64>,
    pub counts: Vec<u32>,
    pub secs: f64,
}

/// Closure in parallel: BFS upward for each node. If `write` is given, writes rows (q, ancestor, depth)
/// only for nodes where `keep(node)`, into `closure-t<k>.tsv.zst`.
pub fn closure(g: &IsaGraph, threads: usize, write: Option<(&Path, i32)>, keep: &(dyn Fn(u32) -> bool + Sync)) -> Result<(ClosureStats, Vec<(String, PathBuf, u64, u64, u64)>)> {
    let t0 = Instant::now();
    let n = g.n();
    let next = AtomicUsize::new(0);
    let pairs = AtomicU64::new(0);
    const BLOCK: usize = 4096;
    let res: Vec<Result<(Vec<(u32, u32)>, u32, u32, u16, Vec<u64>, Option<(String, PathBuf, u64, u64, u64)>)>> = std::thread::scope(|sc| {
        let hs: Vec<_> = (0..threads)
            .map(|t| {
                let (next, pairs) = (&next, &pairs);
                sc.spawn(move || -> Result<(Vec<(u32, u32)>, u32, u32, u16, Vec<u64>, Option<(String, PathBuf, u64, u64, u64)>)> {
                    let mut stamp = vec![u32::MAX; n];
                    let mut depth = vec![0u16; n];
                    let mut queue: Vec<u32> = Vec::new();
                    let mut counts = Vec::new();
                    let (mut max_anc, mut max_q, mut max_d) = (0u32, 0u32, 0u16);
                    let mut by_depth = vec![0u64; 64];
                    let mut out = match write {
                        Some((dir, level)) => Some(Tsv::create(dir.join(format!("closure-t{t}.tsv.zst")), level)?),
                        None => None,
                    };
                    loop {
                        let a = next.fetch_add(BLOCK, Relaxed);
                        if a >= n {
                            break;
                        }
                        for v in a..(a + BLOCK).min(n) {
                            queue.clear();
                            queue.push(v as u32);
                            stamp[v] = v as u32;
                            depth[v] = 0;
                            let mut h = 0;
                            let w_row = out.is_some() && keep(v as u32);
                            while h < queue.len() {
                                let x = queue[h];
                                h += 1;
                                let d = depth[x as usize];
                                for k in g.ups(x) {
                                    let w = g.up[k];
                                    if stamp[w as usize] != v as u32 {
                                        stamp[w as usize] = v as u32;
                                        depth[w as usize] = d + 1;
                                        queue.push(w);
                                        by_depth[((d + 1) as usize).min(63)] += 1;
                                        if w_row {
                                            if let Some(o) = out.as_mut() {
                                                o.n(g.nodes[v] as u64).t().n(g.nodes[w as usize] as u64).t().n((d + 1) as u64).end()?;
                                            }
                                        }
                                    }
                                }
                            }
                            let c = (queue.len() - 1) as u32;
                            counts.push((v as u32, c));
                            if c > max_anc {
                                max_anc = c;
                                max_q = g.nodes[v];
                            }
                            if let Some(&last) = queue.last() {
                                max_d = max_d.max(depth[last as usize]);
                            }
                            pairs.fetch_add(c as u64, Relaxed);
                        }
                    }
                    let f = match out {
                        Some(o) => {
                            let (p, r, raw, z) = o.finish()?;
                            Some((format!("closure-t{t}"), p, r, raw, z))
                        }
                        None => None,
                    };
                    Ok((counts, max_anc, max_q, max_d, by_depth, f))
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap_or_else(|_| Err(anyhow::anyhow!("closure thread panicked")))).collect()
    });
    let mut st = ClosureStats { pairs: pairs.load(Relaxed), max_anc: 0, max_anc_q: 0, max_depth: 0, by_depth: vec![0; 64], counts: vec![0; n], secs: 0.0 };
    let mut files = Vec::new();
    for r in res {
        let (counts, ma, mq, md, bd, f) = r?;
        for (v, c) in counts {
            st.counts[v as usize] = c;
        }
        if ma > st.max_anc {
            st.max_anc = ma;
            st.max_anc_q = mq;
        }
        st.max_depth = st.max_depth.max(md);
        for i in 0..64 {
            st.by_depth[i] += bd[i];
        }
        if let Some(f) = f {
            files.push(f);
        }
    }
    st.secs = t0.elapsed().as_secs_f64();
    Ok((st, files))
}

/// Write the graph edges into `isa_edges.tsv.zst` (s, o, p).
pub fn write_edges(g: &IsaGraph, path: &Path, level: i32) -> Result<(String, PathBuf, u64, u64, u64)> {
    let mut t = Tsv::create(path.to_path_buf(), level)?;
    for s in 0..g.n() as u32 {
        for k in g.ups(s) {
            t.n(g.nodes[s as usize] as u64).t().n(g.nodes[g.up[k] as usize] as u64).t().n(g.kind[k] as u64).end()?;
        }
    }
    let (p, r, raw, z) = t.finish()?;
    Ok(("isa_edges".into(), p, r, raw, z))
}

/// Short node description: `Q89 «apple»`.
pub fn show(info: &ItemInfo, q: u32) -> String {
    match info.label.get(&q) {
        Some(l) if !l.is_empty() => format!("Q{q} «{l}»"),
        _ => format!("Q{q}"),
    }
}

#[cfg(test)]
mod t {
    use super::*;

    /// Synthetic graph: A⊂B⊂C, D⊂E⊂D (cycle), F⊂F (self-loop), G⊂C. The gate finds both cycles, after breaking —
    /// a DAG; the injected edge C⊂A closes a new cycle, and the gate shows it (turns red).
    #[test]
    fn cycles_gate() {
        let e = vec![(1, 2, P279), (2, 3, P279), (4, 5, P279), (5, 4, P279), (6, 6, P279), (7, 3, P171)];
        let g = IsaGraph::from_edges(e.clone());
        assert!(acyclic(&g).is_some());
        let (cyc, g2) = break_cycles(&g);
        assert_eq!(cyc.len(), 2);
        assert_eq!(cyc[0].members, vec![4, 5]);
        assert_eq!(cyc[1].members, vec![6]);
        assert_eq!(acyclic(&g2), None, "after breaking — a DAG");
        assert!(g2.idx(4).is_none() && g2.idx(6).is_none(), "nodes with only cycle edges are gone");
        // closure: 1 → {2, 3}, 7 → {3}
        let i1 = g2.idx(1).unwrap();
        let mut a: Vec<(u32, u16)> = g2.ancestors(i1, 10).into_iter().map(|(v, d)| (g2.nodes[v as usize], d)).collect();
        a.sort();
        assert_eq!(a, vec![(2, 1), (3, 2)]);
        let (st, _) = closure(&g2, 2, None, &|_| true).unwrap();
        assert_eq!(st.pairs, 2 + 1 + 1); // 1→2,3; 2→3; 7→3
        // negative control: injected edge 3⊂1 (class ⊂ its own subclass)
        let mut e2 = g2.edges();
        e2.push((3, 1, P279));
        let (cyc2, g3) = break_cycles(&IsaGraph::from_edges(e2));
        assert_eq!(cyc2.len(), 1);
        assert!(cyc2[0].edges.contains(&(3, 1, P279)));
        assert_eq!(cyc2[0].members, vec![1, 2, 3]);
        assert_eq!(acyclic(&g3), None);
    }

    /// Tarjan on a long chain does not hit the stack limit (iterative).
    #[test]
    fn deep_chain() {
        let e: Vec<(u32, u32, u16)> = (1..200_000u32).map(|i| (i, i + 1, P171)).collect();
        let g = IsaGraph::from_edges(e);
        let (_, n) = scc(&g);
        assert_eq!(n as usize, g.n());
        assert_eq!(acyclic(&g), None);
    }
}

/// What to materialize from the closure.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Materialize {
    /// all nodes if the pairs do not exceed the limit, otherwise — only nodes with Wikipedia
    Auto,
    All,
    /// only nodes with an article in at least one Wikipedia (popular classes and taxa)
    Wiki,
    None,
}

impl Materialize {
    pub fn parse(s: &str) -> Result<Materialize> {
        Ok(match s {
            "auto" => Materialize::Auto,
            "all" => Materialize::All,
            "wiki" => Materialize::Wiki,
            "none" => Materialize::None,
            _ => bail!("--materialize auto|all|wiki|none"),
        })
    }
}

/// «Is a kind of» pass: edges → cycle gate → control with an injected cycle → closure → files and report.
pub fn run_closure(dir: &Path, runs: &Path, threads: usize, mat: Materialize, limit: u64, inject: Option<(u32, u32)>, level: i32) -> Result<String> {
    use std::fmt::Write as _;
    let t0 = Instant::now();
    let (edges, dep) = read_isa_edges(dir)?;
    let n279 = edges.iter().filter(|e| e.2 == P279).count();
    let n171 = edges.len() - n279;
    let g = IsaGraph::from_edges(edges);
    let t_read = t0.elapsed().as_secs_f64();
    // cycle gate
    let (cycles, g2) = break_cycles(&g);
    let removed: usize = cycles.iter().map(|c| c.edges.len()).sum();
    let dag = acyclic(&g2);
    let t_cyc = t0.elapsed().as_secs_f64() - t_read;
    let before = acyclic(&g);
    // negative control: an edge that closes a cycle (class ⊂ its own subclass)
    let (a, b) = match inject {
        Some(x) => x,
        None => {
            // apple: fruit ⊂ apple; if Q89 has no ancestors (incomplete graph) — the first node with ancestors
            let starts = [89u32, 5113, 144, 146, 729];
            let mut pick = None;
            for q in starts.iter().copied().chain(g2.nodes.iter().copied()) {
                let Some(v) = g2.idx(q) else { continue };
                let anc = g2.ancestors(v, 64);
                let fruit = g2.idx(1364).filter(|f| q == 89 && anc.iter().any(|x| x.0 == *f));
                if let Some(top) = fruit.or_else(|| anc.iter().max_by_key(|x| x.1).map(|x| x.0)) {
                    pick = Some((g2.nodes[top as usize], q));
                    break;
                }
            }
            pick.context("the graph has no edges")?
        }
    };
    let mut e3 = g2.edges();
    e3.push((a, b, P279));
    let g3 = IsaGraph::from_edges(e3);
    let raw_red = acyclic(&g3).is_some();
    let (cyc3, g3b) = break_cycles(&g3);
    let caught = cyc3.iter().find(|c| c.edges.contains(&(a, b, P279)));
    let control_ok = raw_red && caught.is_some() && acyclic(&g3b).is_none() && dag.is_none();
    // closure: count first, then decide on materialization
    let (st, _) = closure(&g2, threads, None, &|_| true)?;
    // labels for the report: cycle members, control nodes
    let mut text_q: HashSet<u32> = cycles.iter().flat_map(|c| c.members.iter().copied()).collect();
    for q in [89u32, 1364, 3314483, 1865281, 7377, 152, 5113, 729, 35120, 223557, 488383, a, b, st.max_anc_q] {
        text_q.insert(q);
    }
    let info = read_items(dir, &|q| g.idx(q).is_some(), &|q| text_q.contains(&q))?;
    let wiki = |v: u32| info.n_wiki.get(&g2.nodes[v as usize]).copied().unwrap_or(0) > 0;
    let wiki_pairs: u64 = (0..g2.n() as u32).filter(|&v| wiki(v)).map(|v| st.counts[v as usize] as u64).sum();
    let wiki_nodes = (0..g2.n() as u32).filter(|&v| wiki(v)).count();
    let choice = match mat {
        Materialize::Auto if st.pairs <= limit => Materialize::All,
        Materialize::Auto => Materialize::Wiki,
        m => m,
    };
    let mut files = Vec::new();
    files.push(write_edges(&g2, &dir.join("isa_edges.tsv.zst"), level)?);
    let t_cl = Instant::now();
    match choice {
        Materialize::All => files.extend(closure(&g2, threads, Some((dir, level)), &|_| true)?.1),
        Materialize::Wiki => files.extend(closure(&g2, threads, Some((dir, level)), &wiki)?.1),
        _ => {}
    }
    let t_write = t_cl.elapsed().as_secs_f64();
    // cycles — as separate TSVs (into the derived data for the database and into runs for people)
    let mut ct = String::from("# cycle\tsize\tq\tlabel\n");
    let mut ce = String::from("# cycle\ts\to\tp\n");
    for (i, c) in cycles.iter().enumerate() {
        for q in &c.members {
            let _ = writeln!(ct, "{}\t{}\t{}\t{}", i + 1, c.members.len(), q, info.label.get(q).map(|s| s.as_str()).unwrap_or(""));
        }
        for e in &c.edges {
            let _ = writeln!(ce, "{}\t{}\t{}\t{}", i + 1, e.0, e.1, e.2);
        }
    }
    std::fs::write(dir.join("isa_cycles.tsv"), &ct)?;
    std::fs::write(dir.join("isa_cycle_edges.tsv"), &ce)?;
    std::fs::write(runs.join("isa-cycles.tsv"), &ct)?;
    // report
    let mut s = String::from("# Wikidata: «is a kind of» hierarchy (P279 + P171), cycles, closure\n\n");
    let _ = writeln!(s, "- edges: P279 {} and P171 {} (live, rank ≥ normal); deprecated skipped {}; nodes {}", thousands(n279 as u64), thousands(n171 as u64), thousands(dep), thousands(g.n() as u64));
    let _ = writeln!(s, "- time: reading {t_read:.0} s, cycles {t_cyc:.1} s, closure count {:.0} s ({threads} threads), writing {t_write:.0} s; total {:.0} s\n", st.secs, t0.elapsed().as_secs_f64());
    let _ = writeln!(s, "## Cycle gate\n");
    let _ = writeln!(s, "- before breaking: {}", match before { None => "no cycles".into(), Some(k) => format!("{k} nodes in or under cycles — **red**, list below") });
    let _ = writeln!(s, "- cycles found (strongly connected components > 1 node and self-loops): **{}**, nodes in them {}, internal edges {} — removed from the closure and written (`isa_cycles.tsv`, `isa_cycle_edges.tsv`)", cycles.len(), cycles.iter().map(|c| c.members.len()).sum::<usize>(), removed);
    let _ = writeln!(s, "- after breaking: {}", match dag { None => "acyclic (Kahn's check) — **green**".to_string(), Some(k) => format!("**RED**: {k} nodes remain in cycles") });
    let _ = writeln!(s, "- cycle sizes: {}", {
        let mut h: std::collections::BTreeMap<usize, usize> = Default::default();
        for c in &cycles {
            *h.entry(c.members.len()).or_default() += 1;
        }
        h.iter().map(|(k, v)| format!("{k} nodes — {v}")).collect::<Vec<_>>().join(", ")
    });
    let _ = writeln!(s, "\n**Negative control:** injected edge {} ⊂ {} (class ⊂ its own subclass). Raw graph — {}; the gate {} (cycle of {} nodes: {}). After breaking — {}. Control: **{}**.\n",
        show(&info, a), show(&info, b),
        if raw_red { "with a cycle" } else { "without a cycle (?)" },
        if caught.is_some() { "turns red: the edge is in the cycle list" } else { "does NOT see the edge" },
        caught.map(|c| c.members.len()).unwrap_or(0),
        caught.map(|c| c.members.iter().take(8).map(|q| show(&info, *q)).collect::<Vec<_>>().join(", ")).unwrap_or_default(),
        if acyclic(&g3b).is_none() { "acyclic" } else { "with a cycle" },
        if control_ok { "yes" } else { "NO" });
    let _ = writeln!(s, "### Cycles (first 60 of {})\n", cycles.len());
    for (i, c) in cycles.iter().take(60).enumerate() {
        let _ = writeln!(s, "{}. {} — edges: {}", i + 1, c.members.iter().take(10).map(|q| show(&info, *q)).collect::<Vec<_>>().join(", "), c.edges.iter().take(6).map(|e| format!("Q{} ⊂{} Q{}", e.0, if e.2 == P171 { "t" } else { "" }, e.1)).collect::<Vec<_>>().join("; "));
    }
    let _ = writeln!(s, "\n## Closure\n");
    let _ = writeln!(s, "- pairs (node, ancestor): **{}** over {} nodes, {:.1} on average; the most — {} at {}; greatest depth {}", thousands(st.pairs), thousands(g2.n() as u64), st.pairs as f64 / g2.n().max(1) as f64, st.max_anc, show(&info, st.max_anc_q), st.max_depth);
    let _ = writeln!(s, "- of them at nodes with a Wikipedia article ({} nodes): {}", thousands(wiki_nodes as u64), thousands(wiki_pairs));
    let _ = writeln!(s, "- by depth: {}", st.by_depth.iter().enumerate().skip(1).filter(|x| *x.1 > 0).map(|(d, n)| format!("{d}: {}", thousands(*n))).collect::<Vec<_>>().join(" · "));
    let _ = writeln!(s, "- decision: **{:?}** (requested {:?}, limit {} pairs)\n", choice, mat, thousands(limit));
    let _ = writeln!(s, "| file | rows | TSV, MB | zstd, MB |\n|---|---:|---:|---:|");
    for (t, _, r, raw, z) in &files {
        let _ = writeln!(s, "| {t} | {} | {:.0} | {:.0} |", thousands(*r), *raw as f64 / 1e6, *z as f64 / 1e6);
    }
    if !control_ok || dag.is_some() {
        std::fs::write(runs.join("closure.md"), &s)?;
        bail!("cycle gate or negative control failed — see {}", runs.join("closure.md").display());
    }
    std::fs::write(runs.join("closure.md"), &s)?;
    Ok(s)
}
