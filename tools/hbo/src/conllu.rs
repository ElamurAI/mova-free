//! CoNLL-U with multiword tokens: a sentence is a list of tokens, a token the written word and its words.

use std::fmt::Write as _;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Word {
    /// the ten columns; ID is renumbered on writing, HEAD is a word number in the sentence
    pub cols: [String; 10],
}

impl Word {
    pub fn new(form: &str) -> Word {
        let mut cols: [String; 10] = Default::default();
        for c in cols.iter_mut() {
            *c = "_".into();
        }
        cols[1] = form.into();
        Word { cols }
    }
    pub fn form(&self) -> &str {
        &self.cols[1]
    }
    pub fn upos(&self) -> &str {
        &self.cols[3]
    }
    pub fn head(&self) -> usize {
        self.cols[6].parse().unwrap_or(0)
    }
    pub fn rel(&self) -> &str {
        &self.cols[7]
    }
    pub fn misc(&self, key: &str) -> Option<&str> {
        self.cols[9].split('|').find_map(|kv| kv.strip_prefix(key).and_then(|v| v.strip_prefix('=')))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Token {
    /// the written form (the MWT line's FORM, or the word's own)
    pub form: String,
    /// MISC of the MWT line (SpaceAfter …); empty for a token of one word
    pub misc: String,
    pub words: Vec<Word>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sent {
    pub comments: Vec<String>,
    pub tokens: Vec<Token>,
}

impl Sent {
    pub fn id(&self) -> &str {
        self.comment("sent_id").unwrap_or("")
    }
    pub fn comment(&self, key: &str) -> Option<&str> {
        self.comments.iter().find_map(|c| c.strip_prefix("# ").and_then(|c| c.strip_prefix(key)).and_then(|c| c.strip_prefix(" = ")))
    }
    pub fn words(&self) -> impl Iterator<Item = &Word> {
        self.tokens.iter().flat_map(|t| &t.words)
    }
    pub fn n_words(&self) -> usize {
        self.tokens.iter().map(|t| t.words.len()).sum()
    }
}

pub fn read(src: &str) -> Vec<Sent> {
    let mut out = Vec::new();
    let mut cur = Sent::default();
    // words still owed to the open multiword token
    let mut owed = 0usize;
    for l in src.lines().chain(std::iter::once("")) {
        if l.is_empty() {
            if !cur.tokens.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            cur.comments.clear();
            owed = 0;
            continue;
        }
        if l.starts_with('#') {
            cur.comments.push(l.to_string());
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 10 || c[0].contains('.') {
            continue;
        }
        if let Some((a, b)) = c[0].split_once('-') {
            let (a, b): (usize, usize) = (a.parse().unwrap_or(0), b.parse().unwrap_or(0));
            cur.tokens.push(Token { form: c[1].into(), misc: if c[9] == "_" { String::new() } else { c[9].into() }, words: Vec::new() });
            owed = b + 1 - a.min(b + 1);
            continue;
        }
        let mut w = Word::default();
        for (k, v) in c.iter().enumerate() {
            w.cols[k] = v.to_string();
        }
        if owed > 0 {
            cur.tokens.last_mut().unwrap().words.push(w);
            owed -= 1;
        } else {
            cur.tokens.push(Token { form: c[1].into(), misc: String::new(), words: vec![w] });
        }
    }
    out
}

pub fn write(sents: &[Sent]) -> String {
    let mut s = String::new();
    for x in sents {
        for c in &x.comments {
            writeln!(s, "{c}").unwrap();
        }
        let mut id = 1;
        for t in &x.tokens {
            if t.words.len() > 1 {
                writeln!(s, "{}-{}\t{}\t_\t_\t_\t_\t_\t_\t_\t{}", id, id + t.words.len() - 1, t.form, if t.misc.is_empty() { "_" } else { &t.misc }).unwrap();
            }
            for w in &t.words {
                let mut cols = w.cols.clone();
                cols[0] = id.to_string();
                writeln!(s, "{}", cols.join("\t")).unwrap();
                id += 1;
            }
        }
        s.push('\n');
    }
    s
}

pub fn read_files(paths: &[String]) -> Result<Vec<Sent>, String> {
    let mut out = Vec::new();
    for p in paths {
        out.extend(read(&std::fs::read_to_string(p).map_err(|e| format!("{p}: {e}"))?));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let src = "# sent_id = x\n1-2\tוַיְהִי\t_\t_\t_\t_\t_\t_\t_\t_\n1\tוַ\tוְ\tCCONJ\t_\t_\t2\tcc\t_\tMId=o1\n2\tיְהִי\tהָיָה\tVERB\t_\t_\t0\troot\t_\tMId=o2\n3\tאוֹר\tאוֹר\tNOUN\t_\t_\t2\tnsubj\t_\t_\n\n";
        let s = read(src);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].tokens.len(), 2);
        assert_eq!(s[0].tokens[0].words.len(), 2);
        assert_eq!(s[0].tokens[0].words[1].misc("MId"), Some("o2"));
        assert_eq!(write(&s), src);
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn lossy_rows_ids_and_empty_tokens() {
        let src = "# only comment\n\n# sent_id = x\n8-9\tab\t_\t_\t_\t_\t_\t_\t_\tSpaceAfter=No\n8.1\tignored\t_\t_\t_\t_\t_\t_\t_\t_\nbad\n8\ta\t_\tNOUN\t_\t_\t9\tdep\t_\tKey=x|KeyLong=y\n9\tb\t_\tNOUN\t_\t_\t0\troot\t_\t_";
        let s = read(src);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].n_words(), 2);
        assert_eq!(s[0].tokens[0].words[0].misc("Key"), Some("x"));
        assert_eq!(s[0].tokens[0].words[0].misc("Ke"), None);
        let encoded = write(&s);
        assert!(encoded.contains("1-2\tab"));
        assert!(encoded.contains("1\ta\t_\tNOUN\t_\t_\t9\tdep"));
        assert_eq!(read(&encoded)[0].n_words(), 2);
        let orphan = read("4-5\tx\t_\t_\t_\t_\t_\t_\t_\t_");
        assert_eq!(orphan[0].tokens.len(), 1);
        assert_eq!(write(&orphan), "\n");
        assert_eq!(Word::new("x").head(), 0);
    }
}
