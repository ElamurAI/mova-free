//! langseg: split a text into language segments.
//!
//! 1. **Script runs.** The text is cut into tokens (Hebrew script with its points and accents, Latin, other
//!    scripts); neutral characters (spaces, digits, punctuation) join tokens of one script and split tokens of two.
//!    Inside a Latin run, scholarly transliteration of Hebrew and Aramaic (ʾ ʿ ḥ ṭ ṣ š ḇ ḵ ā ə …, and every Latin
//!    word inside an italic span that holds such a word) is cut out as `translit`.
//! 2. **Varieties.** Inside a Hebrew-script run every word fires transparent features (manual cues from
//!    [`cues`], pointing, generic prefixes and suffixes, the word itself). Each feature has a log-probability
//!    per variety (naive Bayes, learned from the pool); a Viterbi pass over the words, with a fixed cost for
//!    every switch, cuts the run into segments, and each segment says which features decided it.
//!
//! Everything is deterministic: `BTreeMap` everywhere, no randomness outside the seeded shuffles of the tests.

pub mod corpus;
pub mod cues;
pub mod talmud;

use std::collections::BTreeMap;
use unicode_normalization::UnicodeNormalization;

/// The language codes the segmenter emits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {
    En,
    Translit,
    Other,
    Hbo,
    He,
    Arc,
    ArcTargum,
    Tmr,
    Jpa,
    ArcZohar,
    HboMishnaic,
}

/// The Hebrew-script varieties the classifier decides between, in a fixed order (the model's columns).
pub const HEB: [Lang; 8] = [
    Lang::Hbo,
    Lang::HboMishnaic,
    Lang::He,
    Lang::Arc,
    Lang::ArcTargum,
    Lang::Tmr,
    Lang::Jpa,
    Lang::ArcZohar,
];
pub const K: usize = HEB.len();

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Translit => "translit",
            Lang::Other => "other",
            Lang::Hbo => "hbo",
            Lang::He => "he",
            Lang::Arc => "arc",
            Lang::ArcTargum => "arc-x-targum",
            Lang::Tmr => "tmr",
            Lang::Jpa => "jpa",
            Lang::ArcZohar => "arc-x-zohar",
            Lang::HboMishnaic => "hbo-x-mishnaic",
        }
    }
    pub fn parse(s: &str) -> Option<Lang> {
        [
            Lang::En,
            Lang::Translit,
            Lang::Other,
            Lang::Hbo,
            Lang::He,
            Lang::Arc,
            Lang::ArcTargum,
            Lang::Tmr,
            Lang::Jpa,
            Lang::ArcZohar,
            Lang::HboMishnaic,
        ]
        .into_iter()
        .find(|l| l.code() == s)
    }
    /// The column of a Hebrew-script variety in the model.
    pub fn col(self) -> Option<usize> {
        HEB.iter().position(|&l| l == self)
    }
}

// ---------------------------------------------------------------- characters and tokens

pub fn is_heb_letter(c: char) -> bool {
    matches!(c, '\u{05D0}'..='\u{05EA}' | '\u{05EF}'..='\u{05F2}' | '\u{FB1D}'..='\u{FB4F}')
}
/// Vowel points, dagesh, shin/sin dots, rafe, meteg.
pub fn is_heb_point(c: char) -> bool {
    matches!(c, '\u{05B0}'..='\u{05BD}' | '\u{05BF}' | '\u{05C1}' | '\u{05C2}' | '\u{05C4}' | '\u{05C5}' | '\u{05C7}')
}
/// Cantillation accents.
pub fn is_heb_accent(c: char) -> bool {
    matches!(c, '\u{0591}'..='\u{05AF}')
}
fn is_heb_mark(c: char) -> bool {
    is_heb_point(c) || is_heb_accent(c)
}
fn is_geresh(c: char) -> bool {
    matches!(c, '\u{05F3}' | '\u{05F4}')
}
fn is_latin_letter(c: char) -> bool {
    c.is_ascii_alphabetic()
        || matches!(c, '\u{00C0}'..='\u{024F}' | '\u{0250}'..='\u{02AF}' | '\u{1E00}'..='\u{1EFF}')
            && c != '\u{00D7}'
            && c != '\u{00F7}'
        || matches!(c, '\u{02B0}'..='\u{02FF}') && c != '\u{02C6}' && c != '\u{02DC}'
}
fn is_combining(c: char) -> bool {
    matches!(c, '\u{0300}'..='\u{036F}')
}

/// Strong signs of scholarly transliteration of Hebrew or Aramaic in Latin script.
pub fn is_translit_char(c: char) -> bool {
    matches!(
        c,
        'ʾ' | 'ʿ' | 'ʼ' | 'ʻ' | 'ḥ' | 'Ḥ' | 'ṭ' | 'Ṭ' | 'ṣ' | 'Ṣ' | 'ś' | 'Ś' | 'š' | 'Š' | 'ḇ' | 'ḡ' | 'ḏ' | 'ḵ' | 'ṯ'
            | 'ā' | 'ē' | 'ī' | 'ō' | 'ū' | 'Ā' | 'Ē' | 'Ī' | 'Ō' | 'Ū' | 'ă' | 'ĕ' | 'ŏ' | 'ə' | '\u{0304}'
            | '\u{0306}'
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Script {
    Hebr,
    Latn,
    Other,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub script: Script,
    /// italic nesting at this token (markdown `*…*`/`_…_` or `<i>`/`<em>`), with the span number
    pub italic: Option<usize>,
}

/// Cut a text into tokens with byte offsets. Hebrew tokens keep their points, accents, geresh and an inner `"`
/// or a final `'` (ת"ח, ר'); Latin tokens keep combining marks and an inner hyphen or apostrophe.
pub fn tokenize(text: &str) -> Vec<Token> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let n = chars.len();
    let mut out = Vec::new();
    let mut italic: Option<usize> = None;
    let mut spans = 0usize;
    let mut i = 0;
    let lower = text.to_ascii_lowercase();
    while i < n {
        let (b, c) = chars[i];
        // italic markers
        if c == '*' || c == '_' {
            // a run of the same marker toggles italics once
            let mut j = i;
            while j < n && chars[j].1 == c {
                j += 1;
            }
            italic = if italic.is_some() { None } else { spans += 1; Some(spans) };
            i = j;
            continue;
        }
        if c == '<' {
            let rest = &lower[b..];
            if rest.starts_with("<i>") || rest.starts_with("<em>") {
                spans += 1;
                italic = Some(spans);
            } else if rest.starts_with("</i>") || rest.starts_with("</em>") {
                italic = None;
            }
            if let Some(k) = rest.find('>') {
                if k < 12 {
                    let stop = b + k + 1;
                    while i < n && chars[i].0 < stop {
                        i += 1;
                    }
                    continue;
                }
            }
        }
        let script = if is_heb_letter(c) {
            Script::Hebr
        } else if is_latin_letter(c) {
            Script::Latn
        } else if c.is_alphabetic() {
            Script::Other
        } else {
            i += 1;
            continue;
        };
        let mut j = i + 1;
        while j < n {
            let d = chars[j].1;
            let next = chars.get(j + 1).map(|x| x.1);
            let ok = match script {
                Script::Hebr => {
                    is_heb_letter(d)
                        || is_heb_mark(d)
                        || is_geresh(d)
                        || d == '"' && next.is_some_and(is_heb_letter)
                        || d == '\'' && !next.is_some_and(is_heb_letter)
                }
                Script::Latn => {
                    is_latin_letter(d)
                        || is_combining(d)
                        || matches!(d, '-' | '\'' | '’') && next.is_some_and(|x| is_latin_letter(x) || x == 'ʾ' || x == 'ʿ')
                }
                Script::Other => d.is_alphabetic() && !is_heb_letter(d) && !is_latin_letter(d) || is_combining(d),
            };
            if !ok {
                break;
            }
            j += 1;
        }
        let end = if j < n { chars[j].0 } else { text.len() };
        out.push(Token { start: b, end, script, italic });
        i = j;
    }
    out
}

/// A run of tokens of one script (token index range).
#[derive(Clone, Debug)]
pub struct Run {
    pub script: Script,
    pub from: usize,
    pub to: usize,
}

pub fn runs(tokens: &[Token]) -> Vec<Run> {
    let mut out: Vec<Run> = Vec::new();
    for (i, t) in tokens.iter().enumerate() {
        match out.last_mut() {
            Some(r) if r.script == t.script => r.to = i + 1,
            _ => out.push(Run { script: t.script, from: i, to: i + 1 }),
        }
    }
    out
}

/// Inside a Latin run: is each token transliteration? A token with a strong sign is; so is every token in an
/// italic span that holds one, and a token joined to one by a hyphen.
pub fn translit_mask(text: &str, tokens: &[Token]) -> Vec<bool> {
    let mut mask: Vec<bool> = tokens
        .iter()
        .map(|t| t.script == Script::Latn && text[t.start..t.end].chars().any(is_translit_char))
        .collect();
    let spans: std::collections::BTreeSet<usize> =
        tokens.iter().zip(&mask).filter(|(t, &m)| m && t.italic.is_some()).map(|(t, _)| t.italic.unwrap()).collect();
    for (i, t) in tokens.iter().enumerate() {
        if t.script == Script::Latn && t.italic.is_some_and(|s| spans.contains(&s)) {
            mask[i] = true;
        }
    }
    mask
}

// ---------------------------------------------------------------- features

/// The consonantal skeleton: letters (presentation forms decomposed), with `"` for gershayim and `'` for geresh.
pub fn skeleton(word: &str) -> String {
    let mut s = String::new();
    for c in word.nfd() {
        if is_heb_letter(c) {
            s.push(c);
        } else if c == '\u{05F4}' || c == '"' {
            s.push('"');
        } else if c == '\u{05F3}' || c == '\'' {
            s.push('\'');
        }
    }
    s
}

/// Remove points and accents (for the unpointed condition of the measurements).
pub fn strip_points(text: &str) -> String {
    text.nfd().filter(|&c| !is_heb_mark(c)).nfc().collect()
}

/// Feature groups: `P` pointing and orthography, `C` manual cue words, `A` manual cue morphemes,
/// `M` generic prefixes and suffixes, `W` the word itself.
#[derive(Clone, Debug)]
pub struct Groups(pub String);
impl Groups {
    pub fn all() -> Groups {
        Groups("PCAMW".into())
    }
    pub fn has(&self, g: char) -> bool {
        self.0.contains(g)
    }
}

pub struct Featurizer {
    cues: BTreeMap<&'static str, &'static cues::Cue>,
    pub groups: Groups,
}

impl Featurizer {
    pub fn new(groups: Groups) -> Featurizer {
        Featurizer { cues: cues::word_cues(), groups }
    }

    /// The features of one Hebrew-script word.
    pub fn features(&self, word: &str) -> Vec<String> {
        let mut f = Vec::new();
        let sk = skeleton(word);
        let letters: Vec<char> = sk.chars().filter(|&c| is_heb_letter(c)).collect();
        let len = letters.len();
        if self.groups.has('P') {
            let d: Vec<char> = word.nfd().collect();
            if d.iter().any(|&c| is_heb_point(c)) {
                f.push("P:niqqud".to_string());
            }
            if d.iter().any(|&c| is_heb_accent(c)) {
                f.push("P:accent".to_string());
            }
            // וַ + dagesh in the next letter, which is one of the imperfect prefixes
            // (the marks of a letter come in canonical order, so the dagesh is anywhere among them; accents too)
            let lpos: Vec<usize> = (0..d.len()).filter(|&i| is_heb_letter(d[i])).collect();
            let marks = |i: usize| -> &[char] { &d[lpos[i] + 1..lpos.get(i + 1).copied().unwrap_or(d.len())] };
            if lpos.len() >= 2
                && d[0] == 'ו'
                && marks(0).contains(&'\u{05B7}')
                && matches!(d[lpos[1]], 'י' | 'ת' | 'נ' | 'א')
                && (marks(1).contains(&'\u{05BC}') || d[lpos[1]] == 'א')
            {
                f.push("P:wayyiqtol".to_string());
            }
            if sk.contains('"') {
                f.push("P:abbr".to_string());
            } else if sk.contains('\'') {
                f.push("P:geresh".to_string());
            }
        }
        let plain: String = letters.iter().collect();
        if self.groups.has('C') {
            let hit = self.cues.get(sk.as_str()).or_else(|| self.cues.get(plain.as_str())).or_else(|| {
                plain.strip_prefix('ו').filter(|r| r.chars().count() >= 2).and_then(|r| self.cues.get(r))
            });
            if let Some(q) = hit {
                f.push(format!("C:{}", q.name));
            }
        }
        if self.groups.has('A') {
            let mut seen = std::collections::BTreeSet::new();
            for a in cues::AFFIXES {
                if len >= a.min_len && (if a.pre { plain.starts_with(a.form) } else { plain.ends_with(a.form) }) && seen.insert(a.name) {
                    f.push(format!("A:{}", a.name));
                }
            }
        }
        if self.groups.has('M') && len >= 2 {
            let pre2: String = letters.iter().take(2).collect();
            let suf2: String = letters[len - 2..].iter().collect();
            f.push(format!("M:p1{}", letters[0]));
            f.push(format!("M:s1{}", letters[len - 1]));
            f.push(format!("M:p2{pre2}"));
            f.push(format!("M:s2{suf2}"));
            if len >= 4 {
                let suf3: String = letters[len - 3..].iter().collect();
                f.push(format!("M:s3{suf3}"));
            }
        }
        if self.groups.has('W') && !plain.is_empty() {
            f.push(format!("W:{sk}"));
        }
        f
    }
}

// ---------------------------------------------------------------- the model

/// Naive Bayes over word features, one multinomial per feature group g:
/// `logp[f][c] = ln((n(f,c) + α) / (N_g(c) + α·V_g))`.
pub struct Model {
    pub logp: BTreeMap<String, [f32; K]>,
    pub groups: Groups,
    pub alpha: f64,
}

/// Counts gathered from labelled segments.
#[derive(Default)]
pub struct Counts {
    pub n: BTreeMap<String, [u32; K]>,
}

impl Counts {
    pub fn add(&mut self, fz: &Featurizer, text: &str, col: usize) {
        for t in tokenize(text) {
            if t.script != Script::Hebr {
                continue;
            }
            for f in fz.features(&text[t.start..t.end]) {
                self.n.entry(f).or_insert([0; K])[col] += 1;
            }
        }
    }

    pub fn model(&self, groups: Groups, alpha: f64, min_count: u32) -> Model {
        let kept: Vec<(&String, &[u32; K])> =
            self.n.iter().filter(|(f, c)| c.iter().sum::<u32>() >= min_count || !f.starts_with('W') && !f.starts_with('M')).collect();
        // one multinomial per feature group: the mass of a group that is absent from a text (points, say)
        // moves no probability between varieties
        let mut tot: BTreeMap<char, ([f64; K], f64)> = BTreeMap::new();
        for (f, c) in &kept {
            let g = tot.entry(f.chars().next().unwrap_or('?')).or_insert(([0.0; K], 0.0));
            for k in 0..K {
                g.0[k] += c[k] as f64;
            }
            g.1 += 1.0;
        }
        let mut logp = BTreeMap::new();
        for (f, c) in kept {
            let (t, v) = &tot[&f.chars().next().unwrap_or('?')];
            let mut row = [0f32; K];
            for k in 0..K {
                row[k] = ((c[k] as f64 + alpha) / (t[k] + alpha * v)).ln() as f32;
            }
            logp.insert(f.clone(), row);
        }
        Model { logp, groups, alpha }
    }
}

impl Model {
    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        use std::io::Write;
        let mut w = std::io::BufWriter::new(std::fs::File::create(path)?);
        writeln!(w, "#langseg\tgroups={}\talpha={}\t{}", self.groups.0, self.alpha, HEB.map(|l| l.code()).join("\t"))?;
        for (f, row) in &self.logp {
            let cols: Vec<String> = row.iter().map(|x| format!("{x:.4}")).collect();
            writeln!(w, "{f}\t{}", cols.join("\t"))?;
        }
        Ok(())
    }

    pub fn load(path: &std::path::Path) -> Result<Model, String> {
        let s = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut lines = s.lines();
        let head = lines.next().ok_or("empty model")?;
        let mut groups = Groups::all();
        let mut alpha = 0.5;
        for part in head.split('\t') {
            if let Some(g) = part.strip_prefix("groups=") {
                groups = Groups(g.into());
            }
            if let Some(a) = part.strip_prefix("alpha=") {
                alpha = a.parse().unwrap_or(0.5);
            }
        }
        let mut logp = BTreeMap::new();
        for l in lines {
            let mut it = l.split('\t');
            let f = it.next().unwrap_or_default().to_string();
            let mut row = [0f32; K];
            for x in row.iter_mut() {
                *x = it.next().and_then(|v| v.parse().ok()).ok_or("bad model row")?;
            }
            logp.insert(f, row);
        }
        Ok(Model { logp, groups, alpha })
    }

    /// The per-variety score of one word, and the features that fired (known to the model).
    pub fn word_score<'a>(&'a self, fz: &Featurizer, word: &str) -> ([f64; K], Vec<&'a str>) {
        let mut s = [0f64; K];
        let mut fired = Vec::new();
        for f in fz.features(word) {
            if let Some((name, row)) = self.logp.get_key_value(&f) {
                for k in 0..K {
                    s[k] += row[k] as f64;
                }
                fired.push(name.as_str());
            }
        }
        (s, fired)
    }

    /// The per-feature log-odds of variety `a` over variety `b`.
    pub fn log_odds(&self, f: &str, a: usize, b: usize) -> f64 {
        self.logp.get(f).map(|r| (r[a] - r[b]) as f64).unwrap_or(0.0)
    }
}

// ---------------------------------------------------------------- segmentation

/// One output segment: byte span, language, and (for Hebrew-script segments) the deciding features.
#[derive(Clone, Debug)]
pub struct Segment {
    pub start: usize,
    pub end: usize,
    pub lang: Lang,
    /// runner-up variety, total margin in nats, and the top features with their summed log-odds
    pub why: Option<(Lang, f64, Vec<(String, f64)>)>,
}

pub struct Segmenter {
    pub model: Model,
    pub fz: Featurizer,
    /// the cost of a switch between varieties inside a Hebrew-script run, in nats
    pub switch: f64,
    /// strip points and accents before scoring (the unpointed condition)
    pub strip: bool,
}

impl Segmenter {
    pub fn new(model: Model, switch: f64) -> Segmenter {
        let fz = Featurizer::new(model.groups.clone());
        Segmenter { model, fz, switch, strip: false }
    }

    fn word<'a>(&self, w: &'a str) -> std::borrow::Cow<'a, str> {
        if self.strip { std::borrow::Cow::Owned(strip_points(w)) } else { std::borrow::Cow::Borrowed(w) }
    }

    /// Decide one Hebrew-script span as a whole: the argmax of the summed word scores.
    pub fn classify(&self, text: &str) -> (Lang, [f64; K]) {
        let mut tot = [0f64; K];
        for t in tokenize(text) {
            if t.script == Script::Hebr {
                let (s, _) = self.model.word_score(&self.fz, &self.word(&text[t.start..t.end]));
                for k in 0..K {
                    tot[k] += s[k];
                }
            }
        }
        (HEB[argmax(&tot)], tot)
    }

    /// Viterbi over the words of one Hebrew-script run: the best variety per word with a cost per switch.
    /// `cost[i]` is the switch cost between word i-1 and word i (cost[0] is unused).
    pub fn viterbi(&self, scores: &[[f64; K]], cost: &[f64]) -> Vec<usize> {
        let n = scores.len();
        if n == 0 {
            return Vec::new();
        }
        let mut best = scores[0];
        let mut back = vec![[0usize; K]; n];
        for i in 1..n {
            let top = argmax(&best);
            let mut next = [0f64; K];
            for k in 0..K {
                let (stay, jump) = (best[k], best[top] - cost[i]);
                if stay >= jump {
                    next[k] = stay + scores[i][k];
                    back[i][k] = k;
                } else {
                    next[k] = jump + scores[i][k];
                    back[i][k] = top;
                }
            }
            best = next;
        }
        let mut path = vec![0usize; n];
        path[n - 1] = argmax(&best);
        for i in (1..n).rev() {
            path[i - 1] = back[i][path[i]];
        }
        path
    }

    /// Segment a text.
    pub fn segment(&self, text: &str, explain: usize) -> Vec<Segment> {
        let toks = tokenize(text);
        let mask = translit_mask(text, &toks);
        let mut out = Vec::new();
        for r in runs(&toks) {
            match r.script {
                Script::Other => out.push(Segment { start: toks[r.from].start, end: toks[r.to - 1].end, lang: Lang::Other, why: None }),
                Script::Latn => {
                    let mut i = r.from;
                    while i < r.to {
                        let m = mask[i];
                        let mut j = i + 1;
                        while j < r.to && mask[j] == m {
                            j += 1;
                        }
                        let lang = if m { Lang::Translit } else { Lang::En };
                        out.push(Segment { start: toks[i].start, end: toks[j - 1].end, lang, why: None });
                        i = j;
                    }
                }
                Script::Hebr => {
                    let words: Vec<_> = toks[r.from..r.to].iter().map(|t| self.word(&text[t.start..t.end]).into_owned()).collect();
                    let scored: Vec<([f64; K], Vec<&str>)> = words.iter().map(|w| self.model.word_score(&self.fz, w)).collect();
                    let scores: Vec<[f64; K]> = scored.iter().map(|x| x.0).collect();
                    // a sentence end between two words (. : ? ! ׃ ;) makes a switch four times cheaper
                    let cost: Vec<f64> = (r.from..r.to)
                        .map(|i| {
                            let gap = if i > r.from { &text[toks[i - 1].end..toks[i].start] } else { "" };
                            if gap.contains(['.', ':', '?', '!', '׃', ';']) { self.switch / 4.0 } else { self.switch }
                        })
                        .collect();
                    let path = self.viterbi(&scores, &cost);
                    let mut i = 0;
                    while i < path.len() {
                        let mut j = i + 1;
                        while j < path.len() && path[j] == path[i] {
                            j += 1;
                        }
                        let win = path[i];
                        let mut tot = [0f64; K];
                        for s in &scores[i..j] {
                            for k in 0..K {
                                tot[k] += s[k];
                            }
                        }
                        let ru = (0..K).filter(|&k| k != win).max_by(|&a, &b| tot[a].total_cmp(&tot[b]).then(b.cmp(&a))).unwrap();
                        let why = (explain > 0).then(|| {
                            let mut agg: BTreeMap<&str, f64> = BTreeMap::new();
                            for (_, fired) in &scored[i..j] {
                                for f in fired {
                                    *agg.entry(f).or_default() += self.model.log_odds(f, win, ru);
                                }
                            }
                            let mut v: Vec<(String, f64)> = agg.into_iter().map(|(f, x)| (f.to_string(), x)).collect();
                            v.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
                            v.truncate(explain);
                            (HEB[ru], tot[win] - tot[ru], v)
                        });
                        let (a, b) = (r.from + i, r.from + j - 1);
                        out.push(Segment { start: toks[a].start, end: toks[b].end, lang: HEB[win], why });
                        i = j;
                    }
                }
            }
        }
        out
    }
}

/// Index of the maximum; ties go to the lower index (deterministic).
pub fn argmax(x: &[f64; K]) -> usize {
    let mut b = 0;
    for k in 1..K {
        if x[k] > x[b] {
            b = k;
        }
    }
    b
}

/// FNV-1a, for deterministic splits and samples.
pub fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    // splitmix64 finaliser: FNV alone leaves ids that differ in the last character next to each other
    h ^= h >> 30;
    h = h.wrapping_mul(0xbf58476d1ce4e5b9);
    h ^= h >> 27;
    h = h.wrapping_mul(0x94d049bb133111eb);
    h ^ (h >> 31)
}

/// A small deterministic generator (xorshift64*) for the shuffles of the controls and the mixed documents.
pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
}

/// Format the explanation of a segment: `runner-up margin | feature(+x) …`, with cue glosses.
pub fn explain(why: &(Lang, f64, Vec<(String, f64)>)) -> String {
    let (ru, margin, feats) = why;
    let fs: Vec<String> = feats
        .iter()
        .map(|(f, x)| match cues::gloss(f) {
            Some(g) => format!("{f}({x:+.1}; {g})"),
            None => format!("{f}({x:+.1})"),
        })
        .collect();
    format!("vs {} by {margin:.1}: {}", ru.code(), fs.join(", "))
}

#[cfg(test)]
mod wave31 {
    use super::*;
    #[test]
    fn empty_ties_and_switch_objective() {
        let sg = Segmenter::new(
            Model {
                logp: BTreeMap::new(),
                groups: Groups::all(),
                alpha: 0.5,
            },
            4.0,
        );
        assert_eq!(sg.classify("").0, Lang::Hbo);
        assert!(sg.segment("123 !", 2).is_empty());
        assert_eq!(argmax(&[0.0; K]), 0);
        assert!(sg.viterbi(&[], &[]).is_empty());
        let mut a = [-100.0; K];
        a[0] = 3.0;
        a[1] = 0.0;
        let mut b = [-100.0; K];
        b[0] = 0.0;
        b[1] = 4.0;
        assert_eq!(sg.viterbi(&[a, b], &[0.0, 1.0]), [0, 1]);
        assert_eq!(sg.viterbi(&[a, b], &[0.0, 10.0]), [1, 1]);
        let ts = tokenize("a אב *malkā test*");
        assert!(ts.iter().all(|t| t.start < t.end
            && "a אב *malkā test*".is_char_boundary(t.start)
            && "a אב *malkā test*".is_char_boundary(t.end)));
        assert_eq!(
            translit_mask("a אב *malkā test*", &ts),
            [false, false, true, true]
        );
        assert_eq!(Rng(0).next(), 0);
    }
}
