//! Math-mode lexer and a token stream with push-back (for macro expansion).
//!
//! TeX rules that matter for formulas: a control word `\alpha` is ASCII letters, spaces after it
//! are eaten; a control symbol `\,` is one character; `%` is a comment to the end of the line; spaces
//! merge into one `Space` token (it splits a number but does not go into the tree); `~` is a space.
//! The lexer does not fail: doubtful input (`#` without a digit, `\` at the end) is returned as a character, and the error with a position
//! comes from the parser.

/// Token kind. Command names are borrowed from the source (formula or preamble with macros).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T<'a> {
    /// Control sequence without `\`: `alpha`, `,`, `\` (for `\\`).
    Cmd(&'a str),
    /// The same, but without macro expansion: a value saved by `\let` or `\DeclareMathOperator`.
    CmdRaw(&'a str),
    Char(char),
    Open,
    Close,
    Sup,
    Sub,
    Amp,
    /// Macro parameter `#1`…`#9`.
    Param(u8),
    Space,
    /// End of a substream (an argument parsed separately).
    End,
}

#[derive(Clone, Copy, Debug)]
pub struct Tok<'a> {
    pub t: T<'a>,
    /// Byte offset in the formula source (for expanded macros, the position of the call).
    pub pos: u32,
}

/// Split `src` into tokens; positions start from `base`.
pub fn lex<'a>(src: &'a str, base: u32, out: &mut Vec<Tok<'a>>) {
    let b = src.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        let pos = base + i as u32;
        let c = b[i];
        match c {
            b'\\' => {
                if i + 1 >= b.len() {
                    out.push(Tok { t: T::Char('\\'), pos });
                    i += 1;
                    continue;
                }
                let n = b[i + 1];
                if n.is_ascii_alphabetic() {
                    let mut j = i + 1;
                    while j < b.len() && b[j].is_ascii_alphabetic() {
                        j += 1;
                    }
                    out.push(Tok { t: T::Cmd(&src[i + 1..j]), pos });
                    while j < b.len() && matches!(b[j], b' ' | b'\t' | b'\n' | b'\r') {
                        j += 1;
                    }
                    i = j;
                } else if matches!(n, b' ' | b'\t' | b'\n' | b'\r') {
                    // control space `\ `: a space that splits a number (adjacent spaces merge)
                    if !matches!(out.last(), Some(Tok { t: T::Space, .. })) {
                        out.push(Tok { t: T::Space, pos });
                    }
                    i += 2;
                } else {
                    let len = utf8_len(n);
                    out.push(Tok { t: T::Cmd(&src[i + 1..i + 1 + len]), pos });
                    i += 1 + len;
                }
            }
            b'{' => {
                out.push(Tok { t: T::Open, pos });
                i += 1;
            }
            b'}' => {
                out.push(Tok { t: T::Close, pos });
                i += 1;
            }
            b'^' => {
                out.push(Tok { t: T::Sup, pos });
                i += 1;
            }
            b'_' => {
                out.push(Tok { t: T::Sub, pos });
                i += 1;
            }
            b'&' => {
                out.push(Tok { t: T::Amp, pos });
                i += 1;
            }
            b'#' => {
                if i + 1 < b.len() && (b'1'..=b'9').contains(&b[i + 1]) {
                    out.push(Tok { t: T::Param(b[i + 1] - b'0'), pos });
                    i += 2;
                } else {
                    out.push(Tok { t: T::Char('#'), pos });
                    i += 1;
                }
            }
            b'%' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b' ' | b'\t' | b'\n' | b'\r' | b'~' => {
                while i < b.len() && matches!(b[i], b' ' | b'\t' | b'\n' | b'\r' | b'~') {
                    i += 1;
                }
                if !matches!(out.last(), Some(Tok { t: T::Space, .. })) {
                    out.push(Tok { t: T::Space, pos });
                }
            }
            _ => {
                let len = utf8_len(c);
                let ch = src[i..i + len].chars().next().unwrap();
                out.push(Tok { t: T::Char(ch), pos });
                i += len;
            }
        }
    }
}

fn utf8_len(first: u8) -> usize {
    match first {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        _ => 4,
    }
}

/// Token stream: a stack (top = next token), so macro expansion simply pushes the body on top.
#[derive(Default, Debug)]
pub struct Stream<'a> {
    stack: Vec<Tok<'a>>,
}

impl<'a> Stream<'a> {
    pub fn new(src: &'a str) -> Stream<'a> {
        let mut v = Vec::with_capacity(src.len() / 2 + 4);
        lex(src, 0, &mut v);
        v.reverse();
        Stream { stack: v }
    }

    pub fn from_tokens(mut toks: Vec<Tok<'a>>) -> Stream<'a> {
        toks.reverse();
        Stream { stack: toks }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn next(&mut self) -> Option<Tok<'a>> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<T<'a>> {
        self.stack.last().map(|t| t.t)
    }

    /// The token after the next one.
    pub fn peek2(&self) -> Option<T<'a>> {
        let n = self.stack.len();
        if n >= 2 { Some(self.stack[n - 2].t) } else { None }
    }

    /// Push tokens to the front of the stream (in direct order).
    pub fn push_front(&mut self, toks: &[Tok<'a>]) {
        self.stack.extend(toks.iter().rev());
    }

    pub fn push_one(&mut self, t: Tok<'a>) {
        self.stack.push(t);
    }

    pub fn skip_spaces(&mut self) {
        while let Some(T::Space) = self.peek() {
            self.stack.pop();
        }
    }

    /// After an opened `{` (already eaten): tokens up to the matching `}`; None — the brace is not closed.
    pub fn group_tokens(&mut self) -> Option<Vec<Tok<'a>>> {
        let mut depth = 0usize;
        let mut out = Vec::new();
        loop {
            let t = self.stack.pop()?;
            match t.t {
                T::Open => depth += 1,
                T::Close => {
                    if depth == 0 {
                        return Some(out);
                    }
                    depth -= 1;
                }
                T::End => {
                    // substream boundary: the brace is not closed inside
                    self.stack.push(t);
                    return None;
                }
                _ => {}
            }
            out.push(t);
        }
    }

    /// Macro argument without expansion: a group `{…}` (without braces) or a single token.
    /// `Ok(None)` — no argument (end or `}`); `Err(pos)` — unclosed `{` at `pos`.
    pub fn arg_tokens(&mut self) -> Result<Option<Vec<Tok<'a>>>, u32> {
        self.skip_spaces();
        let Some(t) = self.stack.pop() else { return Ok(None) };
        match t.t {
            T::Open => self.group_tokens().map(Some).ok_or(t.pos),
            T::Close | T::End => {
                self.stack.push(t);
                Ok(None)
            }
            _ => Ok(Some(vec![t])),
        }
    }

    /// Optional argument `[…]` (a `]` inside a `{…}` group does not count).
    /// `Ok(None)` — no argument; `Err(pos)` — `[` without `]`.
    pub fn opt_tokens(&mut self) -> Result<Option<Vec<Tok<'a>>>, u32> {
        self.skip_spaces();
        if self.peek() != Some(T::Char('[')) {
            return Ok(None);
        }
        let open = self.stack.pop().unwrap();
        let mut depth = 0usize;
        let mut out = Vec::new();
        loop {
            let Some(t) = self.stack.pop() else { return Err(open.pos) };
            match t.t {
                T::Open => depth += 1,
                T::Close => {
                    if depth == 0 {
                        self.stack.push(t);
                        return Err(open.pos);
                    }
                    depth -= 1;
                }
                T::Char(']') if depth == 0 => return Ok(Some(out)),
                T::End => {
                    self.stack.push(t);
                    return Err(open.pos);
                }
                _ => {}
            }
            out.push(t);
        }
    }

    /// Eat `*` right after a command (`\operatorname*`, `\tag*`).
    /// Rewind the stream to length `len` (drop tokens pushed on top after it).
    pub fn truncate(&mut self, len: usize) {
        if self.stack.len() > len {
            self.stack.truncate(len);
        }
    }

    pub fn eat_star(&mut self) -> bool {
        if self.peek() == Some(T::Char('*')) {
            self.stack.pop();
            true
        } else {
            false
        }
    }
}

/// Tokens → canonical text (for `\text{…}`, environment names, column specifications).
/// Re-lexing the result gives the same tokens; the text round trip relies on this.
pub fn render(toks: &[Tok]) -> String {
    let mut s = String::new();
    for (i, t) in toks.iter().enumerate() {
        match t.t {
            T::Cmd(n) | T::CmdRaw(n) => {
                s.push('\\');
                s.push_str(n);
                let word = n.bytes().all(|c| c.is_ascii_alphabetic());
                if word {
                    if let Some(T::Char(c)) = toks.get(i + 1).map(|t| t.t) {
                        if c.is_ascii_alphabetic() {
                            s.push(' ');
                        }
                    }
                }
            }
            T::Char(c) => s.push(c),
            T::Open => s.push('{'),
            T::Close => s.push('}'),
            T::Sup => s.push('^'),
            T::Sub => s.push('_'),
            T::Amp => s.push('&'),
            T::Param(k) => {
                s.push('#');
                s.push((b'0' + k) as char);
            }
            T::Space => s.push(' '),
            T::End => {}
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<T<'_>> {
        let mut v = Vec::new();
        lex(src, 0, &mut v);
        v.into_iter().map(|t| t.t).collect()
    }

    #[test]
    fn lexes_commands_and_spaces() {
        assert_eq!(kinds("\\alpha x"), vec![T::Cmd("alpha"), T::Char('x')]);
        assert_eq!(kinds("\\,x"), vec![T::Cmd(","), T::Char('x')]);
        assert_eq!(kinds("a \\\\ b"), vec![T::Char('a'), T::Space, T::Cmd("\\"), T::Space, T::Char('b')]);
        assert_eq!(kinds("x^{2}_1"), vec![T::Char('x'), T::Sup, T::Open, T::Char('2'), T::Close, T::Sub, T::Char('1')]);
        assert_eq!(kinds("a % comment\nb"), vec![T::Char('a'), T::Space, T::Char('b')]);
        assert_eq!(kinds("#1#x"), vec![T::Param(1), T::Char('#'), T::Char('x')]);
        assert_eq!(kinds("≤α"), vec![T::Char('≤'), T::Char('α')]);
        assert_eq!(kinds("\\ x~y"), vec![T::Space, T::Char('x'), T::Space, T::Char('y')]);
    }

    #[test]
    fn positions_are_bytes() {
        let mut v = Vec::new();
        lex("α+\\beta", 10, &mut v);
        let p: Vec<u32> = v.iter().map(|t| t.pos).collect();
        assert_eq!(p, vec![10, 12, 13]);
    }

    #[test]
    fn render_is_stable() {
        for s in ["if \\alpha x>0", "a\\,b", "\\foo1", "50\\%", "x $y$ z", "{a}b"] {
            let mut v = Vec::new();
            lex(s, 0, &mut v);
            let r = render(&v);
            let mut w = Vec::new();
            lex(&r, 0, &mut w);
            assert_eq!(render(&w), r, "{s}");
        }
    }
}
