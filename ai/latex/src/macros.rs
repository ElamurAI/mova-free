//! Macros: `\newcommand`/`\renewcommand`/`\providecommand` (with `[n]` and an optional first
//! argument), `\def` with `#1…#9` (also with delimiters, as in TeX), `\let`, `\DeclareMathOperator`.
//! Definitions are stored as tokens borrowed from the source; the parser (`parse.rs`) expands them.

use std::collections::HashMap;

use crate::lex::{Stream, T, Tok, lex};
use crate::table::{Entry, lookup_cmd};

#[derive(Clone, Debug)]
pub struct Macro<'a> {
    /// Number of arguments.
    pub nargs: u8,
    /// Default value of the first (optional) argument: `\newcommand{\f}[2][x]{…}`.
    pub default: Option<Vec<Tok<'a>>>,
    /// Parameter text of a delimited `\def` (`#1.#2`); None — simple parameters (`#1#2`).
    pub pattern: Option<Vec<Tok<'a>>>,
    pub body: Vec<Tok<'a>>,
}

/// Macro table. Names and bodies are borrowed from the source (the document preamble or the formula itself).
#[derive(Clone, Debug, Default)]
pub struct Macros<'a> {
    map: HashMap<&'a str, Macro<'a>>,
}

/// Summary of collecting definitions from a document.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScanStats {
    pub defined: usize,
    /// Malformed definitions that were skipped.
    pub skipped: usize,
}

/// Definition error: what is wrong and where.
pub type DefErr = (&'static str, u32);

impl<'a> Macros<'a> {
    pub fn new() -> Macros<'a> {
        Macros::default()
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn get(&self, name: &str) -> Option<&Macro<'a>> {
        self.map.get(name)
    }
    pub fn contains(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }
    pub fn insert(&mut self, name: &'a str, m: Macro<'a>) {
        self.map.insert(name, m);
    }
    pub fn names(&self) -> impl Iterator<Item = &&'a str> {
        self.map.keys()
    }

    /// Collect definitions from `.tex` text (preamble and body) in order of appearance: a later one overrides
    /// an earlier one, `\providecommand` only adds new ones. Malformed definitions are skipped and counted.
    pub fn scan(&mut self, tex: &'a str) -> ScanStats {
        let mut toks = Vec::with_capacity(tex.len() / 3);
        lex(tex, 0, &mut toks);
        let mut s = Stream::from_tokens(toks);
        let mut st = ScanStats::default();
        while let Some(t) = s.next() {
            let T::Cmd(name) = t.t else { continue };
            let r = match lookup_cmd(name) {
                Some(Entry::NewCommand) => read_newcommand(&mut s, t.pos).map(|d| (d, true)),
                Some(Entry::ProvideCommand) => read_newcommand(&mut s, t.pos).map(|d| (d, false)),
                Some(Entry::Def) => read_def(&mut s, t.pos).map(|d| (d, true)),
                Some(Entry::DeclareOp) => read_declareop(&mut s, t.pos).map(|d| (d, true)),
                Some(Entry::Let) => read_let(&mut s, t.pos).map(|(n, target)| ((n, self.let_meaning(target)), true)),
                _ => continue,
            };
            match r {
                Ok(((n, m), overwrite)) => {
                    if overwrite || !self.contains(n) {
                        self.insert(n, m);
                    }
                    st.defined += 1;
                }
                Err(_) => st.skipped += 1,
            }
        }
        st
    }

    /// Define macros from a string (handy for tests and the CLI): `\newcommand{\R}{\mathbb{R}}`.
    pub fn define(&mut self, src: &'a str) -> ScanStats {
        self.scan(src)
    }

    /// Value for `\let\a\b`: a copy of macro `\b` if it exists, otherwise `\b` itself, unexpanded.
    pub fn let_meaning(&self, target: Tok<'a>) -> Macro<'a> {
        if let T::Cmd(b) = target.t {
            if let Some(m) = self.get(b) {
                return m.clone();
            }
            return simple(vec![Tok { t: T::CmdRaw(b), pos: target.pos }]);
        }
        simple(vec![target])
    }
}

fn simple<'a>(body: Vec<Tok<'a>>) -> Macro<'a> {
    Macro { nargs: 0, default: None, pattern: None, body }
}

/// The name being defined: `\foo` or `{\foo}`.
fn read_name<'a>(s: &mut Stream<'a>) -> Option<&'a str> {
    s.skip_spaces();
    match s.next()?.t {
        T::Cmd(n) | T::CmdRaw(n) => Some(n),
        T::Open => {
            s.skip_spaces();
            let T::Cmd(n) = s.next()?.t else { return None };
            s.skip_spaces();
            match s.next()?.t {
                T::Close => Some(n),
                _ => None,
            }
        }
        _ => None,
    }
}

fn count(toks: &[Tok]) -> Option<u8> {
    let digits: String = toks
        .iter()
        .filter_map(|t| match t.t {
            T::Char(c) => Some(c),
            T::Space => None,
            _ => Some('?'),
        })
        .collect();
    match digits.parse::<u8>() {
        Ok(n) if n <= 9 => Some(n),
        _ => None,
    }
}

/// After `\newcommand` (already consumed): `*`? name `[n]`? `[default]`? `{body}`.
pub fn read_newcommand<'a>(s: &mut Stream<'a>, pos: u32) -> Result<(&'a str, Macro<'a>), DefErr> {
    s.eat_star();
    let name = read_name(s).ok_or(("\\newcommand: missing macro name", pos))?;
    let nargs = match s.opt_tokens() {
        Ok(Some(t)) => count(&t).ok_or(("\\newcommand: argument count is not a number 0…9", pos))?,
        Ok(None) => 0,
        Err(p) => return Err(("\\newcommand: [ without ]", p)),
    };
    let default = if nargs > 0 {
        s.opt_tokens().map_err(|p| ("\\newcommand: [ without ]", p))?
    } else {
        None
    };
    let body = s.arg_tokens().map_err(|p| ("\\newcommand: unclosed {", p))?.ok_or(("\\newcommand: missing body", pos))?;
    Ok((name, Macro { nargs, default, pattern: None, body }))
}

/// After `\def` (already consumed): name, parameter text up to `{`, body.
pub fn read_def<'a>(s: &mut Stream<'a>, pos: u32) -> Result<(&'a str, Macro<'a>), DefErr> {
    s.skip_spaces();
    let name = match s.next().map(|t| t.t) {
        Some(T::Cmd(n)) | Some(T::CmdRaw(n)) => n,
        _ => return Err(("\\def: missing macro name", pos)),
    };
    let mut params = Vec::new();
    loop {
        let t = s.next().ok_or(("\\def: missing body", pos))?;
        match t.t {
            T::Open => break,
            T::Close | T::End => return Err(("\\def: missing body", t.pos)),
            _ => params.push(t),
        }
    }
    let body = s.group_tokens().ok_or(("\\def: unclosed {", pos))?;
    let mut k = 0u8;
    for t in &params {
        if let T::Param(n) = t.t {
            k += 1;
            if n != k {
                return Err(("\\def: parameters out of order", t.pos));
            }
        }
    }
    let plain = params.iter().all(|t| matches!(t.t, T::Param(_)));
    let pattern = if plain { None } else { Some(params) };
    Ok((name, Macro { nargs: k, default: None, pattern, body }))
}

/// After `\let`: name, `=`?, target (a single token).
pub fn read_let<'a>(s: &mut Stream<'a>, pos: u32) -> Result<(&'a str, Tok<'a>), DefErr> {
    s.skip_spaces();
    let name = match s.next().map(|t| t.t) {
        Some(T::Cmd(n)) | Some(T::CmdRaw(n)) => n,
        _ => return Err(("\\let: missing name", pos)),
    };
    s.skip_spaces();
    if s.peek() == Some(T::Char('=')) {
        s.next();
        s.skip_spaces();
    }
    match s.next() {
        Some(t) if !matches!(t.t, T::End | T::Close) => Ok((name, t)),
        _ => Err(("\\let: missing target", pos)),
    }
}

/// After `\DeclareMathOperator`: `*`? name `{text}` → macro `\operatorname{text}`.
pub fn read_declareop<'a>(s: &mut Stream<'a>, pos: u32) -> Result<(&'a str, Macro<'a>), DefErr> {
    let star = s.eat_star();
    let name = read_name(s).ok_or(("\\DeclareMathOperator: missing name", pos))?;
    let text = s
        .arg_tokens()
        .map_err(|p| ("\\DeclareMathOperator: unclosed {", p))?
        .ok_or(("\\DeclareMathOperator: missing text", pos))?;
    let mut body = vec![Tok { t: T::CmdRaw("operatorname"), pos }];
    if star {
        body.push(Tok { t: T::Char('*'), pos });
    }
    body.push(Tok { t: T::Open, pos });
    body.extend(text);
    body.push(Tok { t: T::Close, pos });
    Ok((name, simple(body)))
}

/// Arguments by the parameter text of a delimited `\def` (`#1.#2`), as in TeX: literals must
/// match; a parameter followed by a literal collects tokens (whole groups) up to that literal;
/// an argument that is exactly one group loses its outer braces. None — the call did not match the pattern.
pub fn match_pattern<'a>(s: &mut Stream<'a>, pattern: &[Tok<'a>]) -> Option<Vec<Vec<Tok<'a>>>> {
    let mut args = Vec::new();
    let mut i = 0;
    while i < pattern.len() {
        match pattern[i].t {
            T::Param(_) => {
                // delimiter — literals up to the next parameter or the end of the pattern
                let mut j = i + 1;
                while j < pattern.len() && !matches!(pattern[j].t, T::Param(_)) {
                    j += 1;
                }
                let delim: Vec<T> = pattern[i + 1..j].iter().map(|t| t.t).collect();
                if delim.is_empty() {
                    args.push(s.arg_tokens().ok()??);
                } else {
                    args.push(collect_until(s, &delim)?);
                }
                i = j;
            }
            lit => {
                if lit != T::Space {
                    s.skip_spaces();
                }
                let t = s.next()?;
                if t.t != lit {
                    return None;
                }
                i += 1;
            }
        }
    }
    Some(args)
}

fn collect_until<'a>(s: &mut Stream<'a>, delim: &[T<'a>]) -> Option<Vec<Tok<'a>>> {
    let mut out: Vec<Tok<'a>> = Vec::new();
    let mut depth = 0usize;
    loop {
        if depth == 0 && out_ends_with(&out, delim) {
            out.truncate(out.len() - delim.len());
            // exactly one group — outer braces are removed
            if single_group(&out) {
                out.remove(0);
                out.pop();
            }
            return Some(out);
        }
        let t = s.next()?;
        match t.t {
            T::Open => depth += 1,
            T::Close => {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
            }
            T::End => {
                s.push_one(t);
                return None;
            }
            _ => {}
        }
        out.push(t);
    }
}

/// The tokens are exactly one group `{…}`.
fn single_group(toks: &[Tok]) -> bool {
    if !matches!(toks.first().map(|t| t.t), Some(T::Open)) {
        return false;
    }
    let mut depth = 0usize;
    for (i, t) in toks.iter().enumerate() {
        match t.t {
            T::Open => depth += 1,
            T::Close => {
                depth -= 1;
                if depth == 0 {
                    return i == toks.len() - 1;
                }
            }
            _ => {}
        }
    }
    false
}

fn out_ends_with(out: &[Tok], delim: &[T]) -> bool {
    out.len() >= delim.len() && out[out.len() - delim.len()..].iter().zip(delim).all(|(a, b)| a.t == *b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_definitions() {
        let tex = r"\newcommand{\R}{\mathbb{R}} \newcommand\norm[1]{\left\|#1\right\|}
            \renewcommand{\epsilon}{\varepsilon} \def\ip#1#2{\langle #1,#2\rangle}
            \providecommand{\R}{X} \DeclareMathOperator*{\argmax}{arg\,max} \let\oldsqrt\sqrt
            % \newcommand{\hidden}{h}
            \newcommand{\bad}[x]{y}";
        let mut m = Macros::new();
        let st = m.scan(tex);
        assert_eq!(st.defined, 7);
        assert_eq!(st.skipped, 1);
        assert!(m.contains("R") && m.contains("norm") && m.contains("ip") && m.contains("argmax"));
        assert!(!m.contains("hidden"));
        assert_eq!(m.get("norm").unwrap().nargs, 1);
        assert_eq!(m.get("ip").unwrap().nargs, 2);
        // \providecommand did not override
        assert_eq!(crate::lex::render(&m.get("R").unwrap().body), "\\mathbb{R}");
        assert_eq!(crate::lex::render(&m.get("oldsqrt").unwrap().body), "\\sqrt");
        assert!(matches!(m.get("oldsqrt").unwrap().body[0].t, T::CmdRaw("sqrt")));
    }

    #[test]
    fn delimited_def() {
        let mut m = Macros::new();
        m.scan(r"\def\pair(#1,#2){\langle #1|#2\rangle}");
        let mac = m.get("pair").unwrap();
        assert_eq!(mac.nargs, 2);
        let mut s = Stream::new("(a+b,{c,d})x");
        let args = match_pattern(&mut s, mac.pattern.as_ref().unwrap()).unwrap();
        assert_eq!(crate::lex::render(&args[0]), "a+b");
        assert_eq!(crate::lex::render(&args[1]), "c,d");
        assert_eq!(s.peek(), Some(T::Char('x')));
    }
}
