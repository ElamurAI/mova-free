//! The nerve graph: words, triggers, events of the self and the sentences of the worlds in one graph (`docs/nerves.md`).
//!
//! Words met in a message, a parse or a text activate their nodes; the activation spreads along the edges and reaches
//! the matching places — the sentences of the worlds, the triggers of the code, the events the small ones lived
//! through — and from them other words. The graph of a state is a fixed determinant: built from the skeleton's
//! manuals and triggers, ordered maps only, integer weights, a fixed number of steps — the same input gives the same
//! activation at any hour, under any load, on any system.
//!
//!   world nerves "<English text>"   — what the text activates
//!   world nerves --stats            — the size of the graph and its hash

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;

/// Fixed-point one: activations and weights are integers.
const ONE: u64 = 1_000_000;
/// Nodes kept after each step of spreading.
const KEEP: usize = 4000;
const STEPS: usize = 3;

/// Words too common to carry meaning between sentences.
const STOP: [&str; 64] = [
    "a", "an", "the", "and", "or", "but", "if", "of", "to", "in", "on", "at", "by", "for", "with", "from", "into", "as", "is", "are",
    "was", "were", "be", "been", "being", "it", "its", "this", "that", "these", "those", "there", "their", "they", "them", "he", "she",
    "his", "her", "we", "you", "i", "not", "no", "can", "may", "will", "would", "should", "do", "does", "did", "has", "have", "had",
    "than", "then", "so", "such", "which", "who", "what", "when", "one",
];

/// Question words and fillers: they ask, they do not mean.
const ASKING: [&str; 6] = ["how", "where", "why", "whose", "whom", "very"];

/// Number words: arguments of triggers, never nodes.
const NUMBERS: [&str; 34] = [
    "zero", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
    "sixteen", "seventeen", "eighteen", "nineteen", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    "hundred", "thousand", "million", "billion", "half", "many", "much",
];

/// A word as a node: lower case, letters and digits, a plain plural made singular (the same on every system).
pub fn norm(w: &str) -> Option<String> {
    let w: String = w.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect();
    // numbers are the arguments of triggers, not concepts: they do not spread
    if w.is_empty() || STOP.contains(&w.as_str()) || NUMBERS.contains(&w.as_str()) || ASKING.contains(&w.as_str()) || w.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let w = if w.len() > 4 && w.ends_with("ies") {
        format!("{}y", &w[..w.len() - 3])
    } else if w.len() > 3 && w.ends_with('s') && !w.ends_with("ss") && !w.ends_with("us") && !w.ends_with("is") {
        w[..w.len() - 1].to_string()
    } else {
        w
    };
    Some(w)
}

/// The sentences of a manual: one a line, a line wrapped onto the next joined back (a line that does not end a
/// sentence goes on in the next).
pub fn sentences_of(manual: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut open = false;
    let mut code = false;
    for l in manual.lines().map(str::trim) {
        // a fenced example of code is for the reader of the page, not a statement for the hands
        if l.starts_with("```") {
            code = !code;
            open = false;
            continue;
        }
        if code {
            continue;
        }
        if l.is_empty() || l.starts_with('#') {
            open = false;
            continue;
        }
        if open {
            let last = out.last_mut().unwrap();
            last.push(' ');
            last.push_str(l);
        } else {
            out.push(l.to_string());
        }
        open = !l.ends_with(['.', '!', '?', '»', '"', ')', ']', ':']);
    }
    out
}

fn content_words(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !c.is_alphanumeric()).filter_map(norm).collect()
}

/// What a node is.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    Word(String),
    Trigger(&'static str),
    Event(&'static str),
    /// a sentence of a world: (world id, its line)
    Sentence(&'static str, String),
}

pub struct Nerves {
    pub nodes: Vec<Node>,
    index: BTreeMap<Node, u32>,
    /// edges of each node: (target, weight in thousandths), sorted by target
    adj: Vec<Vec<(u32, u32)>>,
}

impl Nerves {
    fn id(&mut self, n: Node, edges: &mut Vec<BTreeMap<u32, u32>>) -> u32 {
        if let Some(&i) = self.index.get(&n) {
            return i;
        }
        let i = self.nodes.len() as u32;
        self.nodes.push(n.clone());
        self.index.insert(n, i);
        edges.push(BTreeMap::new());
        i
    }

    /// The graph of the skeleton: every central manual, every trigger, every event of the self.
    pub fn build() -> Nerves {
        let mut g = Nerves { nodes: Vec::new(), index: BTreeMap::new(), adj: Vec::new() };
        let mut edges: Vec<BTreeMap<u32, u32>> = Vec::new();
        let mut link = |g: &mut Nerves, edges: &mut Vec<BTreeMap<u32, u32>>, a: Node, b: Node, w: u32| {
            let (x, y) = (g.id(a, edges), g.id(b, edges));
            edges[x as usize].insert(y, w);
            edges[y as usize].insert(x, w);
        };
        for t in &crate::triggers::TRIGGERS {
            for w in t.words.iter().filter_map(|w| norm(w)) {
                link(&mut g, &mut edges, Node::Trigger(t.name), Node::Word(w), 1000);
            }
        }
        for (e, ws) in crate::triggers::EVENTS {
            for w in ws.iter().filter_map(|w| norm(w)) {
                link(&mut g, &mut edges, Node::Event(e), Node::Word(w), 1000);
            }
        }
        for (world, _, manual) in crate::state::CENTRAL_MANUALS.iter() {
            for line in sentences_of(manual) {
                let line = line.as_str();
                let ws = content_words(line);
                if ws.is_empty() {
                    continue;
                }
                for w in ws {
                    link(&mut g, &mut edges, Node::Sentence(world, line.to_string()), Node::Word(w), 1000);
                }
            }
        }
        g.adj = edges.into_iter().map(|m| m.into_iter().collect()).collect();
        g
    }

    pub fn edges(&self) -> usize {
        self.adj.iter().map(Vec::len).sum::<usize>() / 2
    }

    /// A hash of the whole graph (its nodes in order and their edges): the identity of the determinant.
    pub fn hash(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut eat = |b: &[u8]| {
            for x in b {
                h ^= u64::from(*x);
                h = h.wrapping_mul(0x0100_0000_01b3);
            }
        };
        for (i, n) in self.nodes.iter().enumerate() {
            eat(format!("{n:?}").as_bytes());
            for (t, w) in &self.adj[i] {
                eat(&t.to_le_bytes());
                eat(&w.to_le_bytes());
            }
        }
        h
    }

    /// Spread from the words of a text: every node with its total activation over the steps, strongest first (ties by
    /// node order). A word passes its activation to its neighbours divided by its degree, so rare words weigh more.
    pub fn spread(&self, text: &str) -> Vec<(u32, u64)> {
        let mut act: BTreeMap<u32, u64> = BTreeMap::new();
        for w in content_words(text) {
            if let Some(&i) = self.index.get(&Node::Word(w)) {
                act.insert(i, ONE);
            }
        }
        let mut total = act.clone();
        for _ in 0..STEPS {
            let mut next: BTreeMap<u32, u64> = BTreeMap::new();
            for (&u, &a) in &act {
                let d = self.adj[u as usize].len().max(1) as u64;
                for &(v, w) in &self.adj[u as usize] {
                    *next.entry(v).or_default() += a * u64::from(w) / 1000 / d;
                }
            }
            let mut v: Vec<(u32, u64)> = next.into_iter().filter(|x| x.1 > 0).collect();
            v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            v.truncate(KEEP);
            for &(i, a) in &v {
                *total.entry(i).or_default() += a;
            }
            act = v.into_iter().collect();
        }
        let mut out: Vec<(u32, u64)> = total.into_iter().collect();
        out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        out
    }

    /// The sentences of one world that answer a text best: those that hold most of the text's words first, then the
    /// strongest activation, then the shorter.
    pub fn sentences(&self, text: &str, world: &str, k: usize) -> Vec<String> {
        let asked = content_words(text);
        let mut v: Vec<(usize, usize, u64, String)> = self
            .spread(text)
            .into_iter()
            .take(KEEP)
            .filter_map(|(i, a)| match &self.nodes[i as usize] {
                Node::Sentence(w, s) if *w == world => {
                    let ws = content_words(s);
                    let cover = ws.intersection(&asked).count();
                    // how much of the sentence is about what was asked (a sentence that is mostly the asked words is
                    // the plain statement; a long one that touches them is a story about something else)
                    Some((cover, cover * 1000 / ws.len().max(1), a, s.clone()))
                }
                _ => None,
            })
            .collect();
        v.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)).then(b.2.cmp(&a.2)).then(a.3.cmp(&b.3)));
        v.into_iter().take(k).map(|x| x.3).collect()
    }
}

/// The graph of the skeleton, built once per process (it depends only on the compiled skeleton).
pub fn graph() -> &'static Nerves {
    static G: std::sync::OnceLock<Nerves> = std::sync::OnceLock::new();
    G.get_or_init(Nerves::build)
}

/// `world nerves "<text>" | --stats`.
pub fn main_args(args: &[String]) -> Result<()> {
    let t0 = std::time::Instant::now();
    let g = graph();
    if args.get(1).map(String::as_str) == Some("--stats") {
        let kinds = |f: fn(&Node) -> bool| g.nodes.iter().filter(|n| f(n)).count();
        println!(
            "nerves: {} nodes ({} words, {} sentences, {} triggers, {} events), {} edges, hash {:016x}, built in {:.1} s",
            g.nodes.len(),
            kinds(|n| matches!(n, Node::Word(_))),
            kinds(|n| matches!(n, Node::Sentence(..))),
            kinds(|n| matches!(n, Node::Trigger(_))),
            kinds(|n| matches!(n, Node::Event(_))),
            g.edges(),
            g.hash(),
            t0.elapsed().as_secs_f64()
        );
        return Ok(());
    }
    let text = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
    let act = g.spread(&text);
    let show = |f: &dyn Fn(&Node) -> Option<String>, k: usize| act.iter().filter_map(|(i, a)| f(&g.nodes[*i as usize]).map(|s| format!("{s} ({})", a / 1000))).take(k).collect::<Vec<_>>();
    println!("triggers: {}", show(&|n| if let Node::Trigger(t) = n { Some(t.to_string()) } else { None }, 5).join(", "));
    println!("events: {}", show(&|n| if let Node::Event(e) = n { Some(e.to_string()) } else { None }, 5).join(", "));
    println!("words: {}", show(&|n| if let Node::Word(w) = n { Some(w.clone()) } else { None }, 15).join(", "));
    println!("sentences:");
    for s in show(&|n| if let Node::Sentence(w, s) = n { Some(format!("[{w}] {s}")) } else { None }, 8) {
        println!("  · {s}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(v: &[(u32, u64)]) -> u64 {
        v.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, (i, a)| (h ^ u64::from(*i) ^ a.rotate_left(17)).wrapping_mul(0x0100_0000_01b3))
    }

    /// The determinant gate: the same text gives the same activation twice, and under load from other threads.
    #[test]
    fn spreading_is_deterministic_under_load() {
        let g = graph();
        let text = "How many seconds does a training round take on a processor at three gigahertz?";
        let first = digest(&g.spread(text));
        assert_eq!(first, digest(&g.spread(text)));
        let hashes: Vec<u64> = std::thread::scope(|s| {
            let hs: Vec<_> = (0..8).map(|_| s.spawn(|| digest(&graph().spread(text)))).collect();
            hs.into_iter().map(|h| h.join().unwrap()).collect()
        });
        assert!(hashes.iter().all(|&h| h == first));
        assert_eq!(g.hash(), Nerves::build().hash(), "building the graph twice gives the same graph");
    }

    /// Negative control: an order taken from a HashMap (random per instance) does change the digest — the gate can
    /// show red.
    #[test]
    fn planted_hash_order_turns_the_gate_red() {
        let v: Vec<(u32, u64)> = (0..200).map(|i| (i, 1)).collect();
        let seen: BTreeSet<u64> = (0..8)
            .map(|_| {
                let m: std::collections::HashMap<u32, u64> = v.iter().copied().collect();
                digest(&m.into_iter().collect::<Vec<_>>())
            })
            .collect();
        assert!(seen.len() > 1, "a HashMap order must differ between instances, or this control proves nothing");
    }

    #[test]
    #[cfg(feature = "worlds")]
    fn time_words_reach_the_world_of_time() {
        let s = graph().sentences("How many minutes are in an hour?", "time", 5);
        assert!(s.iter().any(|x| x.contains("minute") && x.contains("hour")), "{s:?}");
    }
}
