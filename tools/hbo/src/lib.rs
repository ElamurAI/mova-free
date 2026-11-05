//! The parts of `hbo` other crates read: CoNLL-U with multiword tokens (`conllu`), the segmenter (`seg`), Hebrew text
//! and its levels of pointing (`text`), several segmentations for the parser to choose from (`lattice`), and raw
//! text → segmented words → `gparse annotate` → trees (below). The commands are the binary (`main.rs`).

pub mod conllu;
pub mod lattice;
pub mod seg;
pub mod text;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use conllu::{Sent, Token, Word};
use seg::{Model, Piece};
use text::{Level, norm};

pub type R<T> = Result<T, String>;

pub fn load_model(path: &str) -> R<Model> {
    Ok(Model::load(&fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?))
}

/// The segmenter of a model dir (`<dir>/seg.tsv`). A pointed segmenter with `partial` gets the unpointed one beside
/// it (`<dir>/../unpointed/seg.tsv`, when there is one) as its fallback for partly pointed words it does not know.
pub fn load_seg(dir: &Path) -> R<Model> {
    let mut m = load_model(&dir.join("seg.tsv").to_string_lossy())?;
    if m.opts.partial && m.level == Some(Level::Pointed) {
        if let Some(u) = dir.parent().map(|p| p.join("unpointed").join("seg.tsv")).filter(|u| u.is_file()) {
            m.fallback = Some(Box::new(load_model(&u.to_string_lossy())?));
        }
    }
    Ok(m)
}

/// The i-th written word of a sentence (`keys`: the sentence's words at the model's level, `Model::keys`) cut by the
/// segmenter in its context: its words with the forms as written, `Seg=` in the first word's MISC.
pub fn cut_token(m: &Model, form: &str, keys: &[String], i: usize, space_after: bool) -> Token {
    let (ps, why) = m.split_in(keys, i);
    cut_as(form, &ps, &why.text, space_after)
}

/// A written word cut as given (a candidate of the lattice): `Seg=` says which candidate's reasons.
pub fn cut_with(m: &Model, form: &str, ps: &[Piece]) -> Token {
    let why = m.candidates(form).into_iter().find(|c| c.ps == ps).map(|c| c.why).unwrap_or_else(|| "given".into());
    cut_as(form, ps, &why, true)
}

/// A written word cut into the given pieces, `why` in `Seg=`.
pub fn cut_as(form: &str, ps: &[Piece], why: &str, space_after: bool) -> Token {
    let parts = Model::cut(form, ps);
    let mut words: Vec<Word> = parts.iter().map(|(f, _)| Word::new(f)).collect();
    let mut misc = vec![format!("Seg={}", why.replace(' ', ","))];
    if ps.iter().any(|p| *p == Piece::H) {
        misc.push("Swallowed=ה".into());
    }
    if words.len() == 1 && !space_after {
        misc.push("SpaceAfter=No".into());
    }
    words[0].cols[9] = misc.join("|");
    Token { form: form.to_string(), misc: if words.len() > 1 && !space_after { "SpaceAfter=No".into() } else { String::new() }, words }
}

pub fn mark_token(form: &str) -> Token {
    let mut w = Word::new(form);
    w.cols[3] = "PUNCT".into();
    Token { form: form.to_string(), misc: String::new(), words: vec![w] }
}

/// One raw line into sentences of segmented written words (sentence ids from `first`).
pub fn segment_line(m: &Model, line: &str, first: usize) -> Vec<Sent> {
    let mut out = Vec::new();
    for toks in text::tokenize(line) {
        // no space after a maqaf or before a mark
        let space: Vec<bool> = (0..toks.len()).map(|i| !toks[i].ends_with(text::MAQAF) && toks.get(i + 1).is_some_and(|n| !text::is_mark(n))).collect();
        let keys = m.keys(&toks.iter().map(String::as_str).collect::<Vec<_>>());
        let tokens = toks.iter().zip(&space).enumerate().map(|(i, (t, &sp))| if text::is_mark(t) { mark_token(t) } else { cut_token(m, t, &keys, i, sp) }).collect();
        let mut s = Sent { comments: Vec::new(), tokens };
        let txt: String = toks.iter().zip(&space).map(|(t, &sp)| if sp { format!("{t} ") } else { t.clone() }).collect();
        s.comments = vec![format!("# sent_id = {}", first + out.len()), format!("# text = {txt}")];
        out.push(s);
    }
    out
}

/// Raw text (one or more sentences a line) into sentences of segmented written words.
pub fn segment(m: &Model, src: &str) -> Vec<Sent> {
    let mut out = Vec::new();
    for line in src.lines() {
        let n = out.len();
        out.extend(segment_line(m, line, n + 1));
    }
    out
}

/// Gold sentences with their written words cut again by the segmenter, with no annotation (the input of an
/// end-to-end measure).
pub fn resegment(m: &Model, ss: &mut [Sent]) {
    for s in ss.iter_mut() {
        s.comments.retain(|c| c.starts_with("# sent_id") || c.starts_with("# text"));
        let keys = m.keys(&s.tokens.iter().map(|t| t.form.as_str()).collect::<Vec<_>>());
        for (i, t) in s.tokens.iter_mut().enumerate() {
            *t = if t.words.iter().all(|w| w.upos() == "PUNCT") { mark_token(&t.form) } else { cut_token(m, &t.form, &keys, i, true) };
        }
    }
}

/// The forms the parser was trained on: normalized to its level; ASCII punctuation as the Hebrew marks.
pub fn parser_view(ss: &[Sent], level: Level) -> Vec<Sent> {
    let mut v = ss.to_vec();
    for s in v.iter_mut() {
        for t in s.tokens.iter_mut() {
            for w in t.words.iter_mut() {
                w.cols[1] = match w.cols[1].as_str() {
                    ":" | "." | "?" | "!" => "׃".into(),
                    f if w.cols[3] == "PUNCT" && !matches!(f, "׃" | "׀" | "פ" | "ס") => "׀".into(),
                    f => norm(f, level),
                };
                w.cols[3] = "_".into();
            }
        }
    }
    v
}

/// A parser's LEMMA … DEPREL onto the base sentences' words (their multiword tokens and MISC kept).
pub fn merge(base: &mut [Sent], parsed: &[Sent]) -> R<()> {
    if base.len() != parsed.len() {
        return Err(format!("{} sentences in the base, {} parsed", base.len(), parsed.len()));
    }
    for (b, p) in base.iter_mut().zip(parsed) {
        let pw: Vec<&Word> = p.words().collect();
        if pw.len() != b.n_words() {
            return Err(format!("{}: {} words in the base, {} parsed", b.id(), b.n_words(), pw.len()));
        }
        for (w, q) in b.tokens.iter_mut().flat_map(|t| t.words.iter_mut()).zip(pw) {
            for k in 2..=7 {
                w.cols[k] = q.cols[k].clone();
            }
        }
    }
    Ok(())
}

/// The model of a dir: a dir with `pointed/` and `unpointed/` picks by the text.
pub fn model_dir(dir: &Path, src: &str) -> PathBuf {
    if dir.join("pointed").is_dir() { dir.join(if text::is_pointed(src) { "pointed" } else { "unpointed" }) } else { dir.to_path_buf() }
}

/// Parse segmented sentences in place with `gparse annotate` (the binary `gparse`) and the model of `d`
/// (`seg.tsv`'s level, `gparse/`, `lex.conllu`); `tmp` is the prefix of the parser's files, removed after.
pub fn annotate(d: &Path, m: &Model, ss: &mut [Sent], gparse: &str, tmp: &str) -> R<()> {
    let level = m.level.unwrap_or(Level::Pointed);
    let tin = PathBuf::from(format!("{tmp}.parser-in.conllu"));
    let tout = PathBuf::from(format!("{tmp}.parser-out.conllu"));
    fs::write(&tin, conllu::write(&parser_view(ss, level))).map_err(|e| e.to_string())?;
    let st = Command::new(gparse).arg("annotate").arg(d.join("gparse")).arg(&tin).arg(&tout).env("GP_LEX", d.join("lex.conllu")).status().map_err(|e| format!("{gparse}: {e}"))?;
    if !st.success() {
        return Err(format!("{gparse} annotate failed: {st}"));
    }
    let parsed = conllu::read_files(&[tout.to_string_lossy().to_string()])?;
    merge(ss, &parsed)?;
    let _ = fs::remove_file(&tin);
    let _ = fs::remove_file(&tout);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_gets_the_unpointed_fallback() {
        let root = std::env::temp_dir().join(format!("hbo-load-seg-{}", std::process::id()));
        for (l, with) in [("pointed", "\twith=partial"), ("unpointed", "")] {
            fs::create_dir_all(root.join(l)).unwrap();
            fs::write(root.join(l).join("seg.tsv"), format!("# hbo segmenter\tlevel={l}\tmax_k=4\tmin=10\trare=2{with}\n")).unwrap();
        }
        assert!(load_seg(&root.join("pointed")).unwrap().fallback.is_some());
        // negative control: without `partial`, or alone in its dir, no fallback
        fs::write(root.join("pointed").join("seg.tsv"), "# hbo segmenter\tlevel=pointed\tmax_k=4\tmin=10\trare=2\n").unwrap();
        assert!(load_seg(&root.join("pointed")).unwrap().fallback.is_none());
        assert!(load_seg(&root.join("unpointed")).unwrap().fallback.is_none());
        let _ = fs::remove_dir_all(&root);
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn cuts_views_and_partial_merge_effects() {
        let t = cut_as("אב", &seg::parse_label("1.H.1"), "two reasons", false);
        assert_eq!(t.misc, "SpaceAfter=No");
        assert_eq!(t.words[0].cols[9], "Seg=two,reasons|Swallowed=ה");
        let one = cut_as("אב", &seg::parse_label("2"), "whole", false);
        assert_eq!(one.misc, "");
        assert!(one.words[0].cols[9].contains("SpaceAfter=No"));
        let m = Model::default();
        let ss = segment(&m, "בָּא.\nאב:");
        assert_eq!(ss.len(), 2);
        assert_eq!(ss[1].id(), "2");
        assert_eq!(ss[0].comment("text"), Some("בָּא."));
        let pv = parser_view(&ss, Level::Unpointed);
        assert_eq!(pv[0].tokens[0].words[0].form(), "בא");
        assert_eq!(pv[0].tokens[1].words[0].form(), "׃");
        assert_eq!(pv[0].tokens[1].words[0].upos(), "_");
        let mut base = ss.clone();
        let mut parsed = pv;
        parsed[0].tokens[0].words[0].cols[2] = "changed".into();
        parsed[1].tokens.pop();
        assert!(merge(&mut base, &parsed).is_err());
        assert_eq!(base[0].tokens[0].words[0].cols[2], "changed");
        assert_eq!(base[0].tokens[0].words[0].form(), "בָּא");
        assert_eq!(base[1], ss[1]);
        assert!(std::panic::catch_unwind(|| cut_as("אב", &[], "", true)).is_err());
    }
}
