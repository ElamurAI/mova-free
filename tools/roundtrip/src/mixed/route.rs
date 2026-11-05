//! The meta layer, text ↔ pieces: a line cut into language pieces, each with the route that reads it.
//!
//! `langseg` gives segments over the letters (byte spans of words). Every byte of the line goes to one piece: a gap
//! between two segments is cut at its last run of whitespace — what comes before it (closing quotes, punctuation)
//! ends the piece before, the whitespace is the joint, what comes after it (opening quotes) starts the next piece.
//! Neighbouring segments of one route are one piece. The decoder joins the pieces with one space; a joint that is
//! not one space is residual.

use langseg::{Lang, Rng, Segmenter};

use super::Route;
use super::sample::Span;

#[derive(Clone, Debug)]
pub struct Piece {
    pub start: usize,
    pub end: usize,
    pub route: Route,
}

/// A line as pieces: piece k is `line[start..end]`; the joint before piece k (k ≥ 1) is `line[pieces[k-1].end..start]`.
#[derive(Clone, Debug)]
pub struct Split {
    pub pieces: Vec<Piece>,
}

/// How the pieces are routed: by `langseg` (the system), by the sampler's gold (an oracle), or the negative controls —
/// everything to English, or `langseg`'s languages shuffled over the segments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Langseg,
    Oracle,
    AllEn,
    Shuffle,
}

impl Mode {
    pub fn name(self) -> &'static str {
        match self {
            Mode::Langseg => "langseg",
            Mode::Oracle => "oracle",
            Mode::AllEn => "all-en",
            Mode::Shuffle => "shuffle",
        }
    }
}

pub fn route_of(l: Lang) -> Route {
    match l {
        Lang::En | Lang::Translit => Route::En,
        Lang::Hbo | Lang::HboMishnaic | Lang::He => Route::Heb,
        _ => Route::Text,
    }
}

/// Segments of a line (byte spans and languages) in the mode.
pub fn segments(sg: &Segmenter, line: &str, gold: &[Span], items_lang: &dyn Fn(usize) -> Lang, mode: Mode) -> Vec<(usize, usize, Lang)> {
    match mode {
        // the shuffle permutes the languages over all the segments of a volume (`shuffle`), after this
        Mode::Langseg | Mode::Shuffle => sg.segment(line, 0).into_iter().map(|s| (s.start, s.end, s.lang)).collect(),
        Mode::AllEn => {
            let t = line.trim();
            if t.is_empty() {
                Vec::new()
            } else {
                let a = line.len() - line.trim_start().len();
                vec![(a, a + t.len(), Lang::En)]
            }
        }
        Mode::Oracle => {
            // the letters of each gold item span, by the tokenizer of langseg
            let toks = langseg::tokenize(line);
            let mut out: Vec<(usize, usize, Lang)> = Vec::new();
            for sp in gold {
                let Some(k) = sp.item else { continue };
                let inside: Vec<&langseg::Token> = toks.iter().filter(|t| t.start >= sp.start && t.end <= sp.end).collect();
                if let (Some(a), Some(b)) = (inside.first(), inside.last()) {
                    out.push((a.start, b.end, items_lang(k)));
                }
            }
            out
        }
    }
}

const QUOTES: [char; 8] = ['«', '»', '“', '”', '(', ')', '[', ']'];

/// Segments cut where a document's quote marks or brackets stand between two of their words: langseg reads a
/// Hebrew-script run whole, so two quotes side by side («…» (…)) come out as one segment.
pub fn cut_at_quotes(line: &str, segs: &[(usize, usize, Lang)]) -> Vec<(usize, usize, Lang)> {
    let toks = langseg::tokenize(line);
    let mut out = Vec::new();
    for &(s, e, l) in segs {
        let inside: Vec<&langseg::Token> = toks.iter().filter(|t| t.start >= s && t.end <= e).collect();
        if inside.is_empty() {
            out.push((s, e, l));
            continue;
        }
        let mut a = inside[0].start;
        for w in inside.windows(2) {
            if line[w[0].end..w[1].start].contains(QUOTES) {
                out.push((a, w[0].end, l));
                a = w[1].start;
            }
        }
        out.push((a, e, l));
    }
    out
}

/// The negative control: the languages of all the segments of a volume permuted.
pub fn shuffle(segs: &mut [Vec<(usize, usize, Lang)>], rng: &mut Rng) {
    let mut langs: Vec<Lang> = segs.iter().flatten().map(|x| x.2).collect();
    rng.shuffle(&mut langs);
    let mut it = langs.into_iter();
    for x in segs.iter_mut().flatten() {
        x.2 = it.next().unwrap();
    }
}

fn is_open(c: char) -> bool {
    matches!(c, '«' | '“' | '‘' | '„' | '(' | '[' | '"' | '\'')
}

/// The pieces of a line from its segments.
pub fn split(line: &str, segs: &[(usize, usize, Lang)]) -> Split {
    // neighbouring segments of one route are one piece: English always (its tokenizer reads the punctuation between
    // an English and a transliterated word); Hebrew and Aramaic unless quote marks or brackets of a document part them
    // (« » “ ” ( ) [ ]) — one sentence that langseg cut in two varieties stays whole for its parser, two quotes stay two
    let mut runs: Vec<(usize, usize, Route)> = Vec::new();
    for &(s, e, l) in segs {
        let r = route_of(l);
        match runs.last_mut() {
            Some(x) if x.2 == r && (r == Route::En || !line[x.1..s].contains(QUOTES)) => x.1 = e,
            _ => runs.push((s, e, r)),
        }
    }
    if runs.is_empty() {
        return Split { pieces: if line.is_empty() { Vec::new() } else { vec![Piece { start: 0, end: line.len(), route: Route::Text }] } };
    }
    let mut pieces: Vec<Piece> = runs.iter().map(|x| Piece { start: x.0, end: x.1, route: x.2 }).collect();
    pieces[0].start = 0;
    let n = pieces.len();
    pieces[n - 1].end = line.len();
    for k in 0..n - 1 {
        let (a, b) = (pieces[k].end, pieces[k + 1].start);
        let g = &line[a..b];
        // the last run of whitespace in the gap
        let ws: Vec<(usize, char)> = g.char_indices().collect();
        let last_ws = ws.iter().rposition(|x| x.1.is_whitespace());
        let (ea, sb) = match last_ws {
            Some(j) => {
                let mut i = j;
                while i > 0 && ws[i - 1].1.is_whitespace() {
                    i -= 1;
                }
                (ws[i].0, ws[j].0 + ws[j].1.len_utf8())
            }
            None => {
                // no whitespace: the opening marks go to the next piece
                let o = ws.iter().position(|x| is_open(x.1)).map(|i| ws[i].0).unwrap_or(g.len());
                (o, o)
            }
        };
        pieces[k].end = a + ea;
        pieces[k + 1].start = a + sb;
    }
    Split { pieces }
}

/// The canonical joint and the residual one.
pub fn joint<'a>(line: &'a str, sp: &Split, k: usize) -> &'a str {
    &line[sp.pieces[k - 1].end..sp.pieces[k].start]
}

/// Decode a line from its pieces' texts and the residual joints (`None` = one space).
pub fn join(texts: &[String], joints: &[Option<String>]) -> String {
    let mut s = String::new();
    for (k, t) in texts.iter().enumerate() {
        if k > 0 {
            s.push_str(joints[k].as_deref().unwrap_or(" "));
        }
        s.push_str(t);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pieces_cover_the_line() {
        let line = "As it says, «בְּרֵאשִׁית בָּרָא» then God made it.";
        let a = line.find('ב').unwrap();
        let b = line.find('»').unwrap();
        let segs = vec![(0, 10, Lang::En), (a, b, Lang::Hbo), (b + 3, line.len() - 1, Lang::En)];
        let sp = split(line, &segs);
        assert_eq!(sp.pieces.len(), 3);
        assert_eq!(&line[sp.pieces[0].start..sp.pieces[0].end], "As it says,");
        assert_eq!(&line[sp.pieces[1].start..sp.pieces[1].end], "«בְּרֵאשִׁית בָּרָא»");
        assert_eq!(&line[sp.pieces[2].start..sp.pieces[2].end], "then God made it.");
        let texts: Vec<String> = sp.pieces.iter().map(|p| line[p.start..p.end].to_string()).collect();
        let joints: Vec<Option<String>> = (0..3).map(|k| if k == 0 { None } else { Some(joint(line, &sp, k).to_string()).filter(|j| j != " ") }).collect();
        assert_eq!(join(&texts, &joints), line);
        // negative control: a joint left out of the residual gives the line back wrong
        let line2 = "a  «ב»";
        let sp2 = split(line2, &[(0, 1, Lang::En), (5, 7, Lang::Hbo)]);
        let t2: Vec<String> = sp2.pieces.iter().map(|p| line2[p.start..p.end].to_string()).collect();
        assert_ne!(join(&t2, &[None, None]), line2);
    }
}
