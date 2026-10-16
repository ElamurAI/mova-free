//! Compressor as a single API: `encode(text) → Graph`, `decode(Graph) → text` (lossless or
//! generative), `adapt(domain texts)` — further training of the generator on the domain's own parses
//! (self-training: the domain has no trees, the trees come from the general parser).

use crate::conllu::{Sentence, Token};
use crate::dict::{self, w};
use crate::gram::{Case, Rel, Sym, Tag};
use crate::lin::{Linearizer, Node};
use crate::morph::Morph;
use crate::parse::Parser;
use crate::ptag::PTagger;
use crate::tok::{self, Lexicon};

/// Sentence graph vertex: 12 bytes, no strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GNode {
    /// lemma (or, for `Case::Raw`, the form itself): shared dictionary or the graph's local heap
    pub lemma: Sym,
    pub tag: Tag,
    pub case: Case,
    /// relation to head and head (0 — root)
    pub rel: Rel,
    pub head: u16,
    pub space_after: bool,
}

impl GNode {
    /// Vertex as 12 bytes (little-endian): lemma 4, tag 1, case 1, relation 1, space 1, head 2,
    /// reserved 2 — this is how it lies in memory and on disk.
    pub fn bytes(&self) -> [u8; 12] {
        let mut b = [0u8; 12];
        b[..4].copy_from_slice(&self.lemma.raw().to_le_bytes());
        b[4] = self.tag as u8;
        b[5] = self.case as u8;
        b[6] = self.rel as u8;
        b[7] = self.space_after as u8;
        b[8..10].copy_from_slice(&self.head.to_le_bytes());
        b
    }
}

/// Sentence graph: vertices in text order (lossless mode uses this order).
#[derive(Clone, Debug, Default)]
pub struct Graph {
    pub nodes: Vec<GNode>,
    /// local heap: strings not in the model's dictionary (number with the high bit set)
    pub local: Vec<String>,
    /// sentence bit: interrogative (set by the encoder from the original order)
    pub question: bool,
    /// sentence bit: first word capitalized — a property of the position, not the word (after reordering
    /// the capital letter goes to whichever word became first)
    pub cap_first: bool,
}

impl Graph {
    /// String number: from the dictionary's lemma heap, otherwise from the graph's local heap (added if needed).
    fn sym(&mut self, s: &str) -> Sym {
        if let Some(x) = dict::lemma(s) {
            return x;
        }
        let i = self.local.iter().position(|l| l == s).unwrap_or_else(|| {
            self.local.push(s.to_string());
            self.local.len() - 1
        });
        Sym::local(i)
    }

    /// String of a number.
    pub fn str(&self, s: Sym) -> &str {
        if s.is_local() { &self.local[s.index()] } else { dict::text(s) }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Model {
    pub lx: Lexicon,
    pub tagger: PTagger,
    pub morph: Morph,
    pub parser: Parser,
    pub lin: Linearizer,
    base: Vec<Sentence>,
}

impl Model {
    /// Training all layers on UD sentences with PTB tags; the parser uses beam 8 (`EN_BEAM` changes it).
    pub fn train(ud: Vec<Sentence>) -> Model {
        let beam: usize = std::env::var("EN_BEAM").ok().and_then(|x| x.parse().ok()).unwrap_or(8);
        Self::train_with(ud, beam)
    }

    /// Training with document context (`crate::ctx`) — only for `en ud-eval` experiments: `ctx` is
    /// the context of each sentence of `ud` (same order). Tagger and parser learn with context
    /// features; the rest — as in `train`.
    pub fn train_ctx(ud: Vec<Sentence>, ctx: &[crate::ctx::Ctx]) -> Model {
        assert_eq!(ud.len(), ctx.len(), "context — one per sentence");
        let beam: usize = std::env::var("EN_BEAM").ok().and_then(|x| x.parse().ok()).unwrap_or(8);
        let (ud, ctx): (Vec<Sentence>, Vec<crate::ctx::Ctx>) = ud.into_iter().zip(ctx.iter().cloned()).filter(|(s, _)| s.tagged()).unzip();
        let lx = Lexicon::learn(&ud);
        let tagger = PTagger::train_ctx(&ud, Some(&ctx), 6);
        let parser = if beam > 1 { Parser::train_beam_ctx(&ud, Some(&ctx), 12, beam) } else { Parser::train(&ud, 10) };
        let lin = Linearizer::train(&ud);
        Model { lx, tagger, morph: Morph, parser, lin, base: ud }
    }

    /// The same with an explicit parser beam width (1 — greedy parsing).
    pub fn train_with(ud: Vec<Sentence>, beam: usize) -> Model {
        let ud: Vec<Sentence> = ud.into_iter().filter(Sentence::tagged).collect();
        let lx = Lexicon::learn(&ud);
        let tagger = PTagger::train(&ud, 6);
        let parser = if beam > 1 { Parser::train_beam(&ud, 12, beam) } else { Parser::train(&ud, 10) };
        let lin = Linearizer::train(&ud);
        Model { lx, tagger, morph: Morph, parser, lin, base: ud }
    }

    pub fn encode(&self, text: &str) -> Graph {
        let toks = tok::tokenize(text, &self.lx);
        let words: Vec<&str> = toks.iter().map(|t| t.form.as_str()).collect();
        let tags = self.tagger.tag(&words);
        let tree = self.parser.parse(&words, &tags);
        let mut g = Graph::default();
        for (i, w) in words.iter().enumerate() {
            let (lemma, case) = self.morph.encode(w, tags[i]);
            let lemma = g.sym(&lemma);
            g.nodes.push(GNode { lemma, tag: tags[i], case, rel: tree[i].1, head: tree[i].0 as u16, space_after: toks[i].space_after });
        }
        // a/an — a rule by the next word: lemma "a", the rule gives the form; if the rule does not reproduce
        // the original ("an hour", "a university") — escape
        for i in 0..words.len() {
            let lw = words[i].to_lowercase();
            if tags[i] == Tag::DT && (lw == "a" || lw == "an") {
                let case = if words[i] == lw { Some(Case::AsIs) } else if words[i] == crate::text::title(&lw) { Some(Case::Title) } else { None };
                if let (true, Some(case)) = (lw == article(words.get(i + 1).copied()), case) {
                    g.nodes[i].lemma = w::A;
                    g.nodes[i].case = case;
                } else {
                    g.nodes[i].lemma = g.sym(words[i]);
                    g.nodes[i].case = Case::Raw;
                }
            }
        }
        // capital letter of the first word is positional, unless the word is capitalized on its own
        if let Some(first) = g.nodes.first() {
            let own = first.tag.is_proper() || dict::lower(first.lemma) == w::I || g.str(first.lemma).starts_with(char::is_uppercase);
            if first.case == Case::Title && !own {
                g.nodes[0].case = Case::AsIs;
                g.cap_first = true;
            }
        }
        let qmark = words.iter().zip(&tree).any(|(w, (_, r))| *r == Rel::Punct && w.contains('?'));
        g.question = crate::lin::mood(&self.lin_nodes(&g), qmark);
        g
    }

    /// Nodes for the order generator: lemma key — lowercase number.
    pub fn lin_nodes(&self, g: &Graph) -> Vec<Node> {
        g.nodes
            .iter()
            .map(|n| {
                let lex = if n.lemma.is_local() { dict::lex(&g.local[n.lemma.index()]) } else { dict::lower(n.lemma) };
                Node { tag: n.tag, rel: n.rel, head: n.head as usize, lex }
            })
            .collect()
    }

    /// Word forms of the graph (inflection from lemma, tag and case).
    pub fn forms(&self, g: &Graph) -> Vec<String> {
        g.nodes.iter().map(|n| self.morph.decode(g.str(n.lemma), n.tag, n.case)).collect()
    }

    /// Word order for generative mode.
    pub fn order(&self, g: &Graph) -> Vec<usize> {
        self.lin.order(&self.lin_nodes(g), g.question)
    }

    /// Text from the graph: `lossless` — vertex order as stored; otherwise the generator gives the order.
    pub fn decode(&self, g: &Graph, lossless: bool) -> String {
        let mut forms = self.forms(g);
        let space: Vec<bool> = g.nodes.iter().map(|n| n.space_after).collect();
        let order: Vec<usize> = if lossless { (1..=g.nodes.len()).collect() } else { self.order(g) };
        // position rules: a/an by the next output word, capital letter of the first word
        for k in 0..order.len() {
            let i = order[k] - 1;
            let n = g.nodes[i];
            if n.lemma == w::A && n.tag == Tag::DT && n.case != Case::Raw {
                let next = order.get(k + 1).map(|&j| forms[j - 1].clone());
                let a = article(next.as_deref());
                forms[i] = if n.case == Case::AsIs { a.to_string() } else { crate::text::title(a) };
            }
        }
        if g.cap_first {
            if let Some(&f) = order.first() {
                forms[f - 1] = crate::text::title(&forms[f - 1]);
            }
        }
        crate::text::detok(&forms, &order, &space)
    }

    /// Domain adaptation of the generator: domain texts → own trees → training the linearizer on
    /// UD + domain (domain repeated `weight` times — style weight).
    pub fn adapt(&mut self, domain: &[String], weight: usize) {
        let mut pseudo: Vec<Sentence> = Vec::new();
        for text in domain {
            let g = self.encode(text);
            let forms = self.forms(&g);
            let tokens = g
                .nodes
                .iter()
                .zip(&forms)
                .map(|(n, f)| Token {
                    form: f.clone(),
                    lemma: g.str(n.lemma).to_string(),
                    tag: Some(n.tag),
                    upos: None,
                    feats: Default::default(),
                    head: n.head as usize,
                    rel: n.rel,
                    space_after: n.space_after,
                })
                .collect();
            pseudo.push(Sentence { id: String::new(), text: text.clone(), tokens, has_feats: false, has_lemmas: true, has_forms: true });
        }
        let mut all = self.base.clone();
        for _ in 0..weight {
            all.extend(pseudo.iter().cloned());
        }
        self.lin = Linearizer::train(&all);
    }
}

/// The a/an rule: "an" before a vowel letter.
fn article(next: Option<&str>) -> &'static str {
    match next.and_then(|w| w.chars().next()) {
        Some(c) if matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') => "an",
        _ => "a",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_is_12_bytes() {
        assert_eq!(std::mem::size_of::<GNode>(), 12);
        let n = GNode { lemma: w::THE, tag: Tag::DT, case: Case::Title, rel: Rel::Det, head: 2, space_after: true };
        let b = n.bytes();
        assert_eq!(u32::from_le_bytes([b[0], b[1], b[2], b[3]]), w::THE.raw());
        assert_eq!((b[4], b[5], b[6], b[7], b[8]), (Tag::DT as u8, Case::Title as u8, Rel::Det as u8, 1, 2));
    }
}
