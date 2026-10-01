//! The compressor explained for humans: every layer on one sentence. From this data `en explain` prints
//! and the the project site page is built (`en explain-json`). Everything is real model output.

use serde::Serialize;

use crate::dict;
use crate::gram::{Case, Rel, Tag};
use crate::model::Model;
use crate::parse::Act;
use crate::tok;

/// Tokenizer automaton table: names and descriptions of classes, states, commands; cells are numbers.
#[derive(Serialize)]
pub struct Fsa {
    pub classes: Vec<[&'static str; 2]>,
    pub states: Vec<[&'static str; 2]>,
    pub cmds: Vec<[&'static str; 2]>,
    pub table: Vec<Vec<[usize; 2]>>,
}

pub fn fsa() -> Fsa {
    Fsa {
        classes: tok::CLASS_INFO.iter().map(|&(a, b)| [a, b]).collect(),
        states: tok::STATE_INFO.iter().map(|&(a, b)| [a, b]).collect(),
        cmds: tok::CMD_INFO.iter().map(|&(a, b)| [a, b]).collect(),
        table: tok::fsa_table().into_iter().map(|r| r.into_iter().map(|(k, s)| [k, s]).collect()).collect(),
    }
}

#[derive(Serialize)]
pub struct FsaStep {
    pub ch: String,
    pub class: usize,
    pub state: usize,
    pub cmd: usize,
    pub next: usize,
}

/// A full-run form record: tag, lemma, UD frequency, sources (u — UD, a — AGID paradigm,
/// r — reverse lookup, b — stem).
#[derive(Serialize)]
pub struct Analysis {
    pub tag: &'static str,
    pub lemma: String,
    pub count: u32,
    pub src: String,
}

/// Word: tag, classes with probabilities, lemma, case, heap number, constant, dictionary records.
#[derive(Serialize)]
pub struct Word {
    pub form: String,
    pub space: bool,
    pub tag: &'static str,
    pub known: bool,
    pub classes: Vec<(&'static str, f64)>,
    pub lemma: String,
    pub case: &'static str,
    pub id: Option<u32>,
    pub constant: Option<&'static str>,
    pub analyses: Vec<Analysis>,
}

#[derive(Serialize)]
pub struct Step {
    pub stack: Vec<usize>,
    pub buffer: usize,
    pub scores: [f32; 3],
    pub act: &'static str,
    pub arc: Option<(usize, usize, &'static str)>,
}

#[derive(Serialize)]
pub struct Node {
    pub lemma: String,
    pub id: u32,
    pub local: bool,
    pub tag: &'static str,
    pub case: &'static str,
    pub rel: &'static str,
    pub head: u16,
    pub space: bool,
    pub bytes: String,
}

#[derive(Serialize)]
pub struct Side {
    pub dep: usize,
    pub head: usize,
    pub left: bool,
    pub level: Option<u8>,
    pub counts: (u32, u32),
}

/// All steps for a sentence.
#[derive(Serialize)]
pub struct Explained {
    pub text: String,
    pub fsa: Vec<FsaStep>,
    pub words: Vec<Word>,
    pub steps: Vec<Step>,
    pub tree: Vec<(usize, &'static str)>,
    pub graph: Vec<Node>,
    pub question: bool,
    pub cap_first: bool,
    pub local: Vec<String>,
    pub sides: Vec<Side>,
    pub order: Vec<usize>,
    pub lossless: String,
    pub generative: String,
}

fn case_name(c: Case) -> &'static str {
    match c {
        Case::AsIs => "AsIs",
        Case::Title => "Title",
        Case::Upper => "Upper",
        Case::Raw => "Raw",
    }
}

fn src_letters(s: u8) -> String {
    [(dict::UD, 'u'), (dict::AGID, 'a'), (dict::REV, 'r'), (dict::BASE, 'b')].iter().filter(|(b, _)| s & b != 0).map(|x| x.1).collect()
}

fn act_name(a: Act) -> &'static str {
    match a {
        Act::Shift => "SHIFT",
        Act::Left => "LEFT",
        Act::Right => "RIGHT",
    }
}

pub fn explain(model: &Model, text: &str) -> Explained {
    let toks = tok::tokenize(text, &model.lx);
    let forms: Vec<&str> = toks.iter().map(|t| t.form.as_str()).collect();
    let tags = model.tagger.tag(&forms);
    let (tree, psteps) = model.parser.parse_steps(&forms, &tags);
    let g = model.encode(text);
    let words = forms
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let (known, classes) = model.tagger.tnt().word_classes(f);
            let n = g.nodes[i];
            let analyses = dict::form(&f.to_lowercase())
                .map(|x| {
                    dict::analyses(x)
                        .map(|r| Analysis { tag: r.tag.name(), lemma: dict::text(r.lemma()).to_string(), count: r.count, src: src_letters(r.src) })
                        .collect()
                })
                .unwrap_or_default();
            Word {
                form: f.to_string(),
                space: toks[i].space_after,
                tag: tags[i].name(),
                known,
                classes: classes.into_iter().take(5).map(|(t, p)| (t.name(), p)).collect(),
                lemma: g.str(n.lemma).to_string(),
                case: case_name(n.case),
                id: (!n.lemma.is_local()).then(|| n.lemma.raw()),
                constant: dict::const_name(dict::lower(n.lemma)).or_else(|| dict::const_name(n.lemma)),
                analyses,
            }
        })
        .collect();
    let steps = psteps
        .into_iter()
        .map(|s| Step { stack: s.stack, buffer: s.buffer, scores: s.scores, act: act_name(s.act), arc: s.arc.map(|(h, d, r)| (h, d, r.name())) })
        .collect();
    let graph = g
        .nodes
        .iter()
        .map(|n| Node {
            lemma: g.str(n.lemma).to_string(),
            id: n.lemma.index() as u32,
            local: n.lemma.is_local(),
            tag: n.tag.name(),
            case: case_name(n.case),
            rel: n.rel.name(),
            head: n.head,
            space: n.space_after,
            bytes: n.bytes().iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" "),
        })
        .collect();
    let sides = model
        .lin
        .explain(&model.lin_nodes(&g), g.question)
        .into_iter()
        .map(|s| Side { dep: s.dep, head: s.head, left: s.left, level: s.level, counts: s.counts })
        .collect();
    Explained {
        text: text.to_string(),
        fsa: tok::trace(text).into_iter().map(|s| FsaStep { ch: s.ch.to_string(), class: s.class, state: s.state, cmd: s.cmd, next: s.next }).collect(),
        words,
        steps,
        tree: tree.iter().map(|&(h, r): &(usize, Rel)| (h, r.name())).collect(),
        graph,
        question: g.question,
        cap_first: g.cap_first,
        local: g.local.clone(),
        sides,
        order: model.order(&g),
        lossless: model.decode(&g, true),
        generative: model.decode(&g, false),
    }
}

/// Tags that occur in the explanation — names for the legend.
pub fn tag_names() -> Vec<&'static str> {
    Tag::ALL.iter().map(|t| t.name()).collect()
}
