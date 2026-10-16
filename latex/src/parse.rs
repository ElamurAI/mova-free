//! Math-mode parser: tokens → `Node` tree.
//!
//! As in TeX: a script attaches to the preceding atom (if none, to an empty group `{}`); an argument is
//! a group `{…}` or a single token; `\over` splits the group in two; `{\bf …}` applies to the end of the group;
//! `&` and `\\` split a table. Macros are expanded at the token level. Numbers are consecutive digits with one
//! decimal point; a space breaks them. An unknown command becomes an `Unknown` node with its position and
//! group arguments. Unbalanced input (`{`, `\left`, `\begin`) is an error with a position.

use std::fmt;

use crate::ast::{Env, EnvName, Node, Scripts, Span, Unknown, empty, norm};
use crate::lex::{Stream, T, Tok, render};
use crate::macros::{Macro, Macros, match_pattern, read_declareop, read_def, read_let, read_newcommand};
use crate::table::{Entry, EnvKind, Sym, TextStyle, font_letter, lookup_char, lookup_cmd, lookup_env, negated};

/// Delimiter from raw tokens (`(`, `\\{`; empty or `.` means no delimiter); None means not a delimiter.
fn delim_of(toks: &[Tok]) -> Option<Option<Sym>> {
    let toks: Vec<&Tok> = toks.iter().filter(|t| t.t != T::Space).collect();
    match toks.as_slice() {
        [] => Some(None),
        [t] => match t.t {
            T::Char('.') => Some(None),
            T::Char('<') => Some(Some(Sym::Langle)),
            T::Char('>') => Some(Some(Sym::Rangle)),
            T::Char(c) => lookup_char(c).filter(|s| s.is_delim()).map(Some),
            T::Cmd(n) | T::CmdRaw(n) => match lookup_cmd(n) {
                Some(Entry::Sym(s)) if s.is_delim() => Some(Some(s)),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

/// How many macro expansions are allowed per formula (protection against `\def\a{\a}`).
pub const MAX_EXPANSIONS: u32 = 10_000;
/// How many tokens may accumulate in the stream (protection against exponential macros).
pub const MAX_TOKENS: usize = 1_000_000;
/// Maximum group nesting depth.
pub const MAX_DEPTH: u32 = 400;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    /// Byte offset in the formula source.
    pub pos: u32,
    pub kind: ErrorKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    UnclosedGroup,
    ExtraClose,
    MissingRight,
    ExtraRight,
    BadDelim,
    UnclosedEnv(Box<str>),
    EnvMismatch(Box<str>, Box<str>),
    ExtraEnd(Box<str>),
    MissingArg(&'static str),
    DoubleSup,
    DoubleSub,
    Dollar,
    Param,
    TrailingBackslash,
    MacroLoop,
    BadDefinition(&'static str),
    UnclosedOpt,
    MisplacedAmp,
    MisplacedRowSep,
    TooDeep,
}

impl ErrorKind {
    /// Short code for statistics.
    pub fn code(&self) -> &'static str {
        match self {
            ErrorKind::UnclosedGroup => "unclosed-group",
            ErrorKind::ExtraClose => "extra-close",
            ErrorKind::MissingRight => "missing-right",
            ErrorKind::ExtraRight => "extra-right",
            ErrorKind::BadDelim => "bad-delim",
            ErrorKind::UnclosedEnv(_) => "unclosed-env",
            ErrorKind::EnvMismatch(..) => "env-mismatch",
            ErrorKind::ExtraEnd(_) => "extra-end",
            ErrorKind::MissingArg(_) => "missing-arg",
            ErrorKind::DoubleSup => "double-sup",
            ErrorKind::DoubleSub => "double-sub",
            ErrorKind::Dollar => "dollar",
            ErrorKind::Param => "param",
            ErrorKind::TrailingBackslash => "trailing-backslash",
            ErrorKind::MacroLoop => "macro-loop",
            ErrorKind::BadDefinition(_) => "bad-definition",
            ErrorKind::UnclosedOpt => "unclosed-opt",
            ErrorKind::MisplacedAmp => "misplaced-amp",
            ErrorKind::MisplacedRowSep => "misplaced-rowsep",
            ErrorKind::TooDeep => "too-deep",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::UnclosedGroup => f.write_str("unclosed brace {"),
            ErrorKind::ExtraClose => f.write_str("extra brace }"),
            ErrorKind::MissingRight => f.write_str("\\left without \\right"),
            ErrorKind::ExtraRight => f.write_str("\\right without \\left"),
            ErrorKind::BadDelim => f.write_str("not a delimiter after \\left, \\right or \\middle"),
            ErrorKind::UnclosedEnv(n) => write!(f, "\\begin{{{n}}} without \\end{{{n}}}"),
            ErrorKind::EnvMismatch(a, b) => write!(f, "\\begin{{{a}}} closed by \\end{{{b}}}"),
            ErrorKind::ExtraEnd(n) => write!(f, "\\end{{{n}}} without \\begin"),
            ErrorKind::MissingArg(w) => write!(f, "missing argument: {w}"),
            ErrorKind::DoubleSup => f.write_str("double superscript"),
            ErrorKind::DoubleSub => f.write_str("double subscript"),
            ErrorKind::Dollar => f.write_str("$ inside a formula"),
            ErrorKind::Param => f.write_str("# outside a macro"),
            ErrorKind::TrailingBackslash => f.write_str("\\ at the end of the formula"),
            ErrorKind::MacroLoop => write!(f, "macros expand endlessly (over {MAX_EXPANSIONS})"),
            ErrorKind::BadDefinition(w) => f.write_str(w),
            ErrorKind::UnclosedOpt => f.write_str("[ without ]"),
            ErrorKind::MisplacedAmp => f.write_str("& outside a table"),
            ErrorKind::MisplacedRowSep => f.write_str("\\\\ outside a table"),
            ErrorKind::TooDeep => write!(f, "group nesting over {MAX_DEPTH}"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "position {}: {}", self.pos, self.kind)
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Character (not byte) index in the source, for humans.
    pub fn column(&self, src: &str) -> usize {
        let p = (self.pos as usize).min(src.len());
        src.char_indices().take_while(|(i, _)| *i < p).count()
    }

    /// Source with the error location marked: `\frac{a}{b⟦…`.
    pub fn context(&self, src: &str) -> String {
        let mut p = (self.pos as usize).min(src.len());
        while !src.is_char_boundary(p) {
            p -= 1;
        }
        format!("{}⟦{}", &src[..p], &src[p..])
    }
}

fn err<X>(pos: u32, kind: ErrorKind) -> Result<X, Error> {
    Err(Error { pos, kind })
}

type R<X> = Result<X, Error>;

/// Parse a formula (math-mode content) without document macros.
pub fn parse(src: &str) -> R<Node> {
    parse_with(src, &Macros::new())
}

/// Parse a formula with document macros (`Macros::scan`).
pub fn parse_with<'a>(src: &'a str, macros: &Macros<'a>) -> R<Node> {
    let mut p = Parser { s: Stream::new(src), macros, local: Macros::new(), expansions: 0, depth: 0 };
    p.top()
}

/// Where a list of atoms ends.
enum Stop {
    /// End of the formula; a table is allowed (implicit `aligned`).
    Eof,
    Brace(u32),
    /// `\pmatrix{…}`, `\substack{…}`: a table group.
    TableBrace(u32),
    Right(u32),
    Env(Box<str>, u32),
    /// AMS-TeX: `\aligned … \endaligned`: ends at the command with this name.
    Word(Box<str>, u32),
    /// End of a substream (optional argument, macro argument).
    Marker(u32),
}

impl Stop {
    fn table(&self) -> bool {
        matches!(self, Stop::Eof | Stop::TableBrace(_) | Stop::Env(..) | Stop::Word(..))
    }
}

/// How the list ended.
enum Ended {
    Eof,
    Close,
    Right(Option<Sym>),
    End,
    Amp,
    RowSep,
    Marker,
}

/// What a command produced.
enum Out {
    Atom(Node),
    Many(Vec<Node>),
    Nothing,
    FontSwitch(crate::table::Font),
    Infix(crate::table::FracKind),
    Right(Option<Sym>),
    End(Box<str>, u32),
    RowSep,
    Sup,
    Sub,
}

struct Parser<'a, 'm> {
    s: Stream<'a>,
    macros: &'m Macros<'a>,
    /// Definitions from the formula itself.
    local: Macros<'a>,
    expansions: u32,
    depth: u32,
}

fn bx(n: Node) -> Box<Node> {
    Box::new(n)
}

fn into_items(n: Node) -> Vec<Node> {
    match n {
        Node::Row(v) => v,
        other => vec![other],
    }
}

impl<'a, 'm> Parser<'a, 'm> {
    fn top(&mut self) -> R<Node> {
        let mut rows = self.table(&Stop::Eof)?;
        if rows.len() == 1 && rows[0].len() == 1 {
            return Ok(rows.pop().unwrap().pop().unwrap());
        }
        let kind = if rows.iter().any(|r| r.len() > 1) { EnvKind::Aligned } else { EnvKind::Gathered };
        Ok(Node::Env(Box::new(Env { name: EnvName::Known(kind), arg: None, rows })))
    }

    /// Table: cells separated by `&`, rows by `\\`; an empty last row is dropped.
    fn table(&mut self, stop: &Stop) -> R<Vec<Vec<Node>>> {
        let mut rows = Vec::new();
        let mut row = Vec::new();
        loop {
            let (items, ended) = self.list(stop)?;
            row.push(norm(items));
            match ended {
                Ended::Amp => {}
                Ended::RowSep => rows.push(std::mem::take(&mut row)),
                _ => {
                    rows.push(row);
                    break;
                }
            }
        }
        if rows.len() > 1 && rows.last().is_some_and(|r| r.len() == 1 && r[0].is_empty_row()) {
            rows.pop();
        }
        Ok(rows)
    }

    fn list(&mut self, stop: &Stop) -> R<(Vec<Node>, Ended)> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            let pos = self.s.next().map_or(0, |t| t.pos);
            return err(pos, ErrorKind::TooDeep);
        }
        let r = self.list_inner(stop);
        self.depth -= 1;
        r
    }

    fn list_inner(&mut self, stop: &Stop) -> R<(Vec<Node>, Ended)> {
        let mut items: Vec<Node> = Vec::new();
        loop {
            let Some(tok) = self.s.next() else { return self.eof(stop, items) };
            match tok.t {
                T::Space => {}
                T::End => {
                    if let Stop::Marker(_) = stop {
                        return Ok((items, Ended::Marker));
                    }
                    self.s.push_one(tok);
                    return self.eof(stop, items);
                }
                T::Close => {
                    return match stop {
                        Stop::Brace(_) | Stop::TableBrace(_) => Ok((items, Ended::Close)),
                        _ => err(tok.pos, ErrorKind::ExtraClose),
                    };
                }
                T::Open => {
                    let n = self.group(tok.pos)?;
                    items.push(Node::Group(bx(n)));
                }
                T::Sup => self.script(&mut items, true, tok.pos)?,
                T::Sub => self.script(&mut items, false, tok.pos)?,
                T::Amp => {
                    return if stop.table() { Ok((items, Ended::Amp)) } else { err(tok.pos, ErrorKind::MisplacedAmp) };
                }
                T::Param(k) => {
                    // `CP^2\#2\bar{CP^2}` without `\`: in a formula (not in a macro) `#2` is a symbol and a number
                    items.push(Node::Sym(Sym::Hash));
                    items.push(self.number((b'0' + k) as char));
                }
                T::Char(c) if c.is_ascii_digit() => items.push(self.number(c)),
                T::Char('\'') => self.primes(&mut items),
                T::Char(c) => items.push(self.char_atom(c, tok.pos)?),
                T::Cmd(name) | T::CmdRaw(name) => {
                    if let Stop::Word(w, _) = stop {
                        if **w == *name {
                            return Ok((items, Ended::End));
                        }
                    }
                    let raw = matches!(tok.t, T::CmdRaw(_));
                    match self.command(name, raw, tok.pos)? {
                        Out::Atom(n) => items.push(n),
                        Out::Many(v) => items.extend(v),
                        Out::Nothing => {}
                        Out::FontSwitch(f) => {
                            let (rest, ended) = self.list(stop)?;
                            items.push(Node::Font(f, bx(norm(rest))));
                            return Ok((items, ended));
                        }
                        Out::Infix(k) => {
                            let (rest, ended) = self.list(stop)?;
                            let num = norm(items);
                            let den = norm(rest);
                            return Ok((vec![Node::Frac(k, bx(num), bx(den))], ended));
                        }
                        Out::Right(d) => {
                            return match stop {
                                Stop::Right(_) => Ok((items, Ended::Right(d))),
                                _ => err(tok.pos, ErrorKind::ExtraRight),
                            };
                        }
                        Out::End(name, p) => {
                            return match stop {
                                Stop::Env(expected, _) if **expected == *name => Ok((items, Ended::End)),
                                Stop::Env(expected, _) => err(p, ErrorKind::EnvMismatch(expected.clone(), name)),
                                _ => err(p, ErrorKind::ExtraEnd(name)),
                            };
                        }
                        Out::RowSep => {
                            if !stop.table() {
                                return err(tok.pos, ErrorKind::MisplacedRowSep);
                            }
                            self.after_rowsep(tok.pos)?;
                            return Ok((items, Ended::RowSep));
                        }
                        Out::Sup => self.script(&mut items, true, tok.pos)?,
                        Out::Sub => self.script(&mut items, false, tok.pos)?,
                    }
                }
            }
        }
    }

    fn eof(&self, stop: &Stop, items: Vec<Node>) -> R<(Vec<Node>, Ended)> {
        match stop {
            Stop::Eof => Ok((items, Ended::Eof)),
            Stop::Brace(p) | Stop::TableBrace(p) | Stop::Marker(p) => err(*p, ErrorKind::UnclosedGroup),
            Stop::Right(p) => err(*p, ErrorKind::MissingRight),
            Stop::Env(n, p) | Stop::Word(n, p) => err(*p, ErrorKind::UnclosedEnv(n.clone())),
        }
    }

    fn group(&mut self, pos: u32) -> R<Node> {
        let (items, _) = self.list(&Stop::Brace(pos))?;
        Ok(norm(items))
    }

    fn number(&mut self, first: char) -> Node {
        let mut s = String::with_capacity(4);
        s.push(first);
        let mut dot = false;
        loop {
            match self.s.peek() {
                Some(T::Char(d)) if d.is_ascii_digit() => {
                    s.push(d);
                    self.s.next();
                }
                Some(T::Char('.')) if !dot && matches!(self.s.peek2(), Some(T::Char(d)) if d.is_ascii_digit()) => {
                    dot = true;
                    s.push('.');
                    self.s.next();
                }
                _ => break,
            }
        }
        Node::Num(s.into())
    }

    /// A single character as an atom (no digit merging; for arguments).
    fn char_atom(&mut self, c: char, pos: u32) -> R<Node> {
        match c {
            '$' => err(pos, ErrorKind::Dollar),
            // `$#P$` is an error in TeX but common in MathJax and arXiv abstracts: read it as `\#`
            '#' => Ok(Node::Sym(Sym::Hash)),
            '\\' => err(pos, ErrorKind::TrailingBackslash),
            '\'' => Ok(Node::Sym(Sym::Prime)),
            d if d.is_ascii_digit() => Ok(Node::Num(d.to_string().into())),
            _ => Ok(if let Some(s) = lookup_char(c) {
                Node::Sym(s)
            } else if let Some((f, l)) = font_letter(c) {
                Node::Font(f, bx(Node::Char(l)))
            } else {
                Node::Char(c)
            }),
        }
    }

    fn primes(&mut self, items: &mut Vec<Node>) {
        let mut n: u8 = 1;
        while self.s.peek() == Some(T::Char('\'')) {
            self.s.next();
            n = n.saturating_add(1);
        }
        match items.pop() {
            Some(Node::Scripts(mut s)) if s.sup.is_none() => {
                s.primes = s.primes.saturating_add(n);
                items.push(Node::Scripts(s));
            }
            last => {
                let base = last.unwrap_or_else(|| Node::Group(bx(empty())));
                items.push(Node::Scripts(Box::new(Scripts { base, sub: None, sup: None, primes: n })));
            }
        }
    }

    fn script(&mut self, items: &mut Vec<Node>, sup: bool, pos: u32) -> R<()> {
        let arg = self.arg(pos, if sup { "^" } else { "_" })?;
        match items.pop() {
            Some(Node::Scripts(mut s)) => {
                let slot = if sup { &mut s.sup } else { &mut s.sub };
                if slot.is_some() {
                    return err(pos, if sup { ErrorKind::DoubleSup } else { ErrorKind::DoubleSub });
                }
                *slot = Some(arg);
                items.push(Node::Scripts(s));
            }
            last => {
                let base = last.unwrap_or_else(|| Node::Group(bx(empty())));
                let mut s = Scripts { base, sub: None, sup: None, primes: 0 };
                if sup {
                    s.sup = Some(arg);
                } else {
                    s.sub = Some(arg);
                }
                items.push(Node::Scripts(Box::new(s)));
            }
        }
        Ok(())
    }

    /// Argument of a command or script: a group `{…}`, a single character, or one command with its arguments.
    /// A macro argument is taken as a whole token and expanded inside (as in LaTeX).
    fn arg(&mut self, pos: u32, what: &'static str) -> R<Node> {
        loop {
            let Some(tok) = self.s.next() else { return err(pos, ErrorKind::MissingArg(what)) };
            match tok.t {
                T::Space => continue,
                T::Open => return self.group(tok.pos),
                T::Char(c) => return self.char_atom(c, tok.pos),
                T::Cmd(name) | T::CmdRaw(name) => {
                    let raw = matches!(tok.t, T::CmdRaw(_));
                    if !raw && self.is_macro(name) {
                        self.s.push_one(Tok { t: T::End, pos: tok.pos });
                        self.s.push_one(tok);
                        let (items, _) = self.list(&Stop::Marker(tok.pos))?;
                        return Ok(norm(items));
                    }
                    match self.command(name, raw, tok.pos)? {
                        Out::Atom(n) => return Ok(n),
                        Out::Many(v) => return Ok(norm(v)),
                        Out::Nothing => continue,
                        _ => return err(tok.pos, ErrorKind::MissingArg(what)),
                    }
                }
                _ => {
                    self.s.push_one(tok);
                    return err(pos, ErrorKind::MissingArg(what));
                }
            }
        }
    }

    /// Unparsed argument (for text, environment names, colours).
    fn raw_arg(&mut self, pos: u32, what: &'static str) -> R<Vec<Tok<'a>>> {
        match self.s.arg_tokens() {
            Ok(Some(t)) => Ok(t),
            Ok(None) => err(pos, ErrorKind::MissingArg(what)),
            Err(p) => err(p, ErrorKind::UnclosedGroup),
        }
    }

    fn opt_raw(&mut self) -> R<Option<Vec<Tok<'a>>>> {
        self.s.opt_tokens().or_else(|p| err(p, ErrorKind::UnclosedOpt))
    }

    /// Optional argument `[…]`, parsed as a formula.
    fn opt_node(&mut self, pos: u32) -> R<Option<Node>> {
        let Some(mut toks) = self.opt_raw()? else { return Ok(None) };
        toks.push(Tok { t: T::End, pos });
        self.s.push_front(&toks);
        let (items, _) = self.list(&Stop::Marker(pos))?;
        Ok(Some(norm(items)))
    }

    fn after_rowsep(&mut self, _pos: u32) -> R<()> {
        // `\\*` and `\\[2pt]` only right after `\\` (a space before `[` means it is already row content)
        self.s.eat_star();
        if self.s.peek() == Some(T::Char('[')) {
            self.opt_raw()?;
        }
        Ok(())
    }

    fn skip_dimen(&mut self) {
        self.s.skip_spaces();
        while matches!(self.s.peek(), Some(T::Char('-' | '+'))) {
            self.s.next();
            self.s.skip_spaces();
        }
        while matches!(self.s.peek(), Some(T::Char(c)) if c.is_ascii_digit() || c == '.' || c == ',') {
            self.s.next();
        }
        self.s.skip_spaces();
        if matches!(self.s.peek(), Some(T::Cmd(_))) {
            self.s.next();
        } else {
            for _ in 0..2 {
                if matches!(self.s.peek(), Some(T::Char(c)) if c.is_ascii_alphabetic()) {
                    self.s.next();
                }
            }
        }
    }

    fn delim(&mut self, pos: u32) -> R<Option<Sym>> {
        loop {
            self.s.skip_spaces();
            let Some(tok) = self.s.next() else { return err(pos, ErrorKind::BadDelim) };
            match tok.t {
                T::Char('.') => return Ok(None),
                T::Char('<') => return Ok(Some(Sym::Langle)),
                T::Char('>') => return Ok(Some(Sym::Rangle)),
                T::Char(c) => {
                    return match lookup_char(c) {
                        Some(s) if s.is_delim() => Ok(Some(s)),
                        _ => err(tok.pos, ErrorKind::BadDelim),
                    };
                }
                T::Cmd(name) | T::CmdRaw(name) => {
                    if matches!(tok.t, T::Cmd(_)) && self.expand_if_macro(name, tok.pos)? {
                        continue;
                    }
                    return match lookup_cmd(name) {
                        Some(Entry::Sym(s)) if s.is_delim() => Ok(Some(s)),
                        _ => err(tok.pos, ErrorKind::BadDelim),
                    };
                }
                _ => {
                    self.s.push_one(tok);
                    return err(tok.pos, ErrorKind::BadDelim);
                }
            }
        }
    }

    fn env(&mut self, pos: u32) -> R<Node> {
        let name = render(&self.raw_arg(pos, "\\begin{…}")?);
        let kind = lookup_env(&name);
        let mut arg = None;
        match kind {
            Some(EnvKind::Array) | Some(EnvKind::Subarray) => {
                self.opt_raw()?;
                arg = Some(render(&self.raw_arg(pos, "array columns")?).into());
            }
            Some(EnvKind::AlignedAt) => arg = Some(render(&self.raw_arg(pos, "alignedat columns")?).into()),
            Some(EnvKind::Aligned) | Some(EnvKind::Gathered) => {
                self.opt_raw()?;
            }
            Some(_) if name.ends_with('*') => {
                self.opt_raw()?;
            }
            _ => {}
        }
        let rows = self.table(&Stop::Env(name.clone().into(), pos))?;
        let name = match kind {
            Some(k) => EnvName::Known(k),
            None => EnvName::Unknown(name.into(), Span { start: pos, end: pos + 6 }),
        };
        Ok(Node::Env(Box::new(Env { name, arg, rows })))
    }

    fn is_macro(&self, name: &str) -> bool {
        (!self.local.is_empty() && self.local.contains(name)) || (!self.macros.is_empty() && self.macros.contains(name))
    }

    /// If `name` is a macro, expand it (the body is put at the front of the stream).
    fn expand_if_macro(&mut self, name: &'a str, pos: u32) -> R<bool> {
        if !self.local.is_empty() {
            if let Some(m) = self.local.get(name) {
                let m = m.clone();
                self.expand(&m, pos)?;
                return Ok(true);
            }
        }
        let macros = self.macros;
        if !macros.is_empty() {
            if let Some(m) = macros.get(name) {
                self.expand(m, pos)?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn expand(&mut self, m: &Macro<'a>, pos: u32) -> R<()> {
        self.expansions += 1;
        if self.expansions > MAX_EXPANSIONS || self.s.len() > MAX_TOKENS {
            return err(pos, ErrorKind::MacroLoop);
        }
        let args: Vec<Vec<Tok<'a>>> = match &m.pattern {
            Some(p) => match_pattern(&mut self.s, p).ok_or(Error { pos, kind: ErrorKind::MissingArg("macro argument") })?,
            None => {
                let mut a = Vec::with_capacity(m.nargs as usize);
                for i in 0..m.nargs {
                    if i == 0 {
                        if let Some(d) = &m.default {
                            a.push(self.opt_raw()?.unwrap_or_else(|| d.clone()));
                            continue;
                        }
                    }
                    a.push(self.raw_arg(pos, "macro argument")?);
                }
                a
            }
        };
        let mut out: Vec<Tok<'a>> = Vec::with_capacity(m.body.len() + 8);
        for t in &m.body {
            match t.t {
                T::Param(k) => {
                    if let Some(arg) = args.get(k as usize - 1) {
                        out.extend(arg.iter().copied());
                    }
                }
                other => out.push(Tok { t: other, pos }),
            }
        }
        self.s.push_front(&out);
        Ok(())
    }

    fn define(&mut self, entry: Entry, pos: u32) -> R<()> {
        let r = match entry {
            Entry::NewCommand => read_newcommand(&mut self.s, pos).map(|d| (d, true)),
            Entry::ProvideCommand => read_newcommand(&mut self.s, pos).map(|d| (d, false)),
            Entry::Def => read_def(&mut self.s, pos).map(|d| (d, true)),
            Entry::DeclareOp => read_declareop(&mut self.s, pos).map(|d| (d, true)),
            Entry::Let => read_let(&mut self.s, pos).map(|(n, t)| {
                let m = match t.t {
                    T::Cmd(b) if self.local.contains(b) => self.local.get(b).unwrap().clone(),
                    _ => self.macros.let_meaning(t),
                };
                ((n, m), true)
            }),
            _ => unreachable!(),
        };
        match r {
            Ok(((name, m), overwrite)) => {
                if overwrite || !self.is_macro(name) {
                    self.local.insert(name, m);
                }
                Ok(())
            }
            Err((what, p)) => err(p, ErrorKind::BadDefinition(what)),
        }
    }

    /// Unknown command: its arguments are the `{…}` groups right after it. A group that does not parse as a
    /// formula (`\setpres{…}{$x$ for all $s$}`, an author macro with text) is kept as text.
    fn unknown(&mut self, name: &'a str, pos: u32) -> R<Out> {
        let mut args = Vec::new();
        loop {
            self.s.skip_spaces();
            if self.s.peek() != Some(T::Open) {
                break;
            }
            let open = self.s.next().unwrap();
            let Some(toks) = self.s.group_tokens() else { return err(open.pos, ErrorKind::UnclosedGroup) };
            let len0 = self.s.len();
            let mut sub = toks.clone();
            sub.push(Tok { t: T::End, pos: open.pos });
            self.s.push_front(&sub);
            match self.list(&Stop::Marker(open.pos)) {
                Ok((items, _)) => args.push(norm(items)),
                Err(e) => {
                    if self.s.len() < len0 {
                        return Err(e);
                    }
                    self.s.truncate(len0);
                    args.push(Node::Text(TextStyle::Plain, render(&toks).into()));
                }
            }
        }
        let span = Span { start: pos, end: pos + 1 + name.len() as u32 };
        Ok(Out::Atom(Node::Unknown(Box::new(Unknown { name: name.into(), args, span }))))
    }

    /// `\genfrac{left}{right}{thickness}{style}{a}{b}` → fraction or binomial; other delimiters → `\left…\right`.
    fn genfrac(&mut self, pos: u32) -> R<Node> {
        let l = self.raw_arg(pos, "\\genfrac: left delimiter")?;
        let r = self.raw_arg(pos, "\\genfrac: right delimiter")?;
        let thick = render(&self.raw_arg(pos, "\\genfrac: thickness")?);
        let style = render(&self.raw_arg(pos, "\\genfrac: style")?);
        let a = self.arg(pos, "\\genfrac: numerator")?;
        let b = self.arg(pos, "\\genfrac: denominator")?;
        let (Some(l), Some(r)) = (delim_of(&l), delim_of(&r)) else { return err(pos, ErrorKind::BadDelim) };
        let thick = thick.trim();
        let zero = !thick.is_empty() && thick.trim_start_matches(['0', '.']).chars().all(|c| c.is_ascii_alphabetic());
        let style = style.trim();
        use crate::table::FracKind as K;
        let pick = |plain: K, d: K, t: K| match style {
            "0" => d,
            "1" => t,
            _ => plain,
        };
        let kind = match (l, r, zero) {
            (Some(Sym::LParen), Some(Sym::RParen), true) => Some(pick(K::Binom, K::DBinom, K::TBinom)),
            (Some(Sym::LBrack), Some(Sym::RBrack), true) => Some(K::Brack),
            (Some(Sym::LBrace), Some(Sym::RBrace), true) => Some(K::Brace),
            _ => None,
        };
        // the infix form (`\brack`, `\atop`) prints as `{a \brack b}`, so here it is in a group too
        let grouped = |k: K, a: Node, b: Node| {
            let f = Node::Frac(k, bx(a), bx(b));
            if k.latex().is_none() { Node::Group(bx(f)) } else { f }
        };
        if let Some(k) = kind {
            return Ok(grouped(k, a, b));
        }
        let k = if zero { K::Atop } else { pick(K::Frac, K::DFrac, K::TFrac) };
        Ok(if l.is_none() && r.is_none() { grouped(k, a, b) } else { Node::LeftRight(l, bx(Node::Frac(k, bx(a), bx(b))), r) })
    }

    /// `\root n \of x` (plain TeX) → `\sqrt[n]{x}`.
    fn root(&mut self, pos: u32) -> R<Node> {
        let mut idx = Vec::new();
        loop {
            let Some(t) = self.s.next() else { return err(pos, ErrorKind::MissingArg("\\root … \\of")) };
            match t.t {
                T::Cmd("of") => break,
                T::End => {
                    self.s.push_one(t);
                    return err(pos, ErrorKind::MissingArg("\\root … \\of"));
                }
                _ => idx.push(t),
            }
        }
        idx.push(Tok { t: T::End, pos });
        self.s.push_front(&idx);
        let (items, _) = self.list(&Stop::Marker(pos))?;
        let body = self.arg(pos, "\\root … \\of")?;
        Ok(Node::Sqrt(Some(bx(norm(items))), bx(body)))
    }

    fn command(&mut self, name: &'a str, raw: bool, pos: u32) -> R<Out> {
        if !raw && self.expand_if_macro(name, pos)? {
            return Ok(Out::Nothing);
        }
        let Some(entry) = lookup_cmd(name) else {
            if name == "\\" {
                return Ok(Out::RowSep);
            }
            return self.unknown(name, pos);
        };
        Ok(match entry {
            Entry::Sym(s) => Out::Atom(Node::Sym(s)),
            Entry::Frac(k) => {
                let a = self.arg(pos, "numerator")?;
                let b = self.arg(pos, "denominator")?;
                Out::Atom(Node::Frac(k, bx(a), bx(b)))
            }
            Entry::Infix(k) => Out::Infix(k),
            Entry::Genfrac => Out::Atom(self.genfrac(pos)?),
            Entry::Root => Out::Atom(self.root(pos)?),
            Entry::Unicode => {
                let code = render(&self.raw_arg(pos, "\\unicode{code}")?);
                let code = code.trim();
                let n = match code.strip_prefix(['x', 'X']) {
                    Some(h) => u32::from_str_radix(h, 16).ok(),
                    None => code.parse::<u32>().ok(),
                };
                match n.and_then(char::from_u32) {
                    Some(c) if !c.is_control() && !matches!(c, '$' | '#' | '\\' | '{' | '}' | '^' | '_' | '&' | '%' | '~') => {
                        Out::Atom(self.char_atom(c, pos)?)
                    }
                    _ => return err(pos, ErrorKind::MissingArg("\\unicode{code}")),
                }
            }
            Entry::Sqrt => {
                let idx = self.opt_node(pos)?;
                let body = self.arg(pos, "\\sqrt")?;
                Out::Atom(Node::Sqrt(idx.map(bx), bx(body)))
            }
            Entry::Accent(a) => Out::Atom(Node::Accent(a, bx(self.arg(pos, "accent")?))),
            Entry::Font(f) => Out::Atom(Node::Font(f, bx(self.arg(pos, "font")?))),
            Entry::FontSwitch(f) => Out::FontSwitch(f),
            Entry::Text(st) => {
                let toks = self.raw_arg(pos, "\\text")?;
                Out::Atom(Node::Text(st, render(&toks).into()))
            }
            Entry::MathClass(c) => Out::Atom(Node::Class(c, bx(self.arg(pos, "\\mathrel…")?))),
            Entry::Stack(k) => {
                let a = self.arg(pos, "annotation")?;
                let b = self.arg(pos, "base")?;
                Out::Atom(Node::Stack(k, bx(a), bx(b)))
            }
            Entry::XArrow(k) => {
                let below = self.opt_node(pos)?;
                let above = self.arg(pos, "arrow label")?;
                Out::Atom(Node::XArrow(k, below.map(bx), bx(above)))
            }
            Entry::PlainEnv(k) => {
                self.s.skip_spaces();
                if self.s.peek() == Some(T::Open) {
                    // plain TeX: \pmatrix{…}
                    let t = self.s.next().unwrap();
                    let rows = self.table(&Stop::TableBrace(t.pos))?;
                    Out::Atom(Node::Env(Box::new(Env { name: EnvName::Known(k), arg: None, rows })))
                } else if k == EnvKind::Substack {
                    return err(pos, ErrorKind::MissingArg("\\substack{…}"));
                } else {
                    // AMS-TeX: \pmatrix … \endpmatrix
                    let arg = if k == EnvKind::AlignedAt { Some(render(&self.raw_arg(pos, "\\alignedat")?).into()) } else { None };
                    let end: Box<str> = format!("end{name}").into();
                    let rows = self.table(&Stop::Word(end, pos))?;
                    Out::Atom(Node::Env(Box::new(Env { name: EnvName::Known(k), arg, rows })))
                }
            }
            Entry::Eqno => {
                // equation number up to the end of the formula (or group)
                let mut depth = 0usize;
                while let Some(t) = self.s.next() {
                    match t.t {
                        T::Open => depth += 1,
                        T::Close if depth == 0 => {
                            self.s.push_one(t);
                            break;
                        }
                        T::Close => depth -= 1,
                        T::End => {
                            self.s.push_one(t);
                            break;
                        }
                        _ => {}
                    }
                }
                Out::Nothing
            }
            Entry::Left => {
                let l = self.delim(pos)?;
                let (items, ended) = self.list(&Stop::Right(pos))?;
                let r = match ended {
                    Ended::Right(r) => r,
                    _ => return err(pos, ErrorKind::MissingRight),
                };
                Out::Atom(Node::LeftRight(l, bx(norm(items)), r))
            }
            Entry::Right => Out::Right(self.delim(pos)?),
            Entry::Middle => Out::Atom(Node::Middle(self.delim(pos)?)),
            Entry::Begin => Out::Atom(self.env(pos)?),
            Entry::End => {
                let name = render(&self.raw_arg(pos, "\\end{…}")?);
                Out::End(name.into(), pos)
            }
            Entry::OpName => {
                let star = self.s.eat_star();
                Out::Atom(Node::OpName(bx(self.arg(pos, "\\operatorname")?), star))
            }
            Entry::OpNameLim => Out::Atom(Node::OpName(bx(self.arg(pos, "\\operatornamewithlimits")?), true)),
            Entry::Not => {
                let a = self.arg(pos, "\\not")?;
                Out::Atom(match a {
                    Node::Sym(s) if negated(s).is_some() => Node::Sym(negated(s).unwrap()),
                    other => Node::Not(bx(other)),
                })
            }
            Entry::Space | Entry::Ignore => Out::Nothing,
            Entry::SpaceArg | Entry::IgnoreArg => {
                self.s.eat_star();
                self.raw_arg(pos, "argument")?;
                Out::Nothing
            }
            Entry::Kern => {
                self.skip_dimen();
                Out::Nothing
            }
            Entry::Phantom => {
                self.raw_arg(pos, "\\phantom")?;
                Out::Nothing
            }
            Entry::Splice => Out::Many(into_items(self.arg(pos, "argument")?)),
            Entry::Smash => {
                self.opt_raw()?;
                Out::Many(into_items(self.arg(pos, "\\smash")?))
            }
            Entry::Color => {
                self.raw_arg(pos, "\\color")?;
                Out::Nothing
            }
            Entry::TextColor => {
                self.raw_arg(pos, "colour")?;
                Out::Many(into_items(self.arg(pos, "\\textcolor")?))
            }
            Entry::NewCommand | Entry::ProvideCommand | Entry::Def | Entry::Let | Entry::DeclareOp => {
                self.define(entry, pos)?;
                Out::Nothing
            }
            Entry::RowSep => Out::RowSep,
            Entry::Sup => Out::Sup,
            Entry::Sub => Out::Sub,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::{Accent, Font, FracKind};

    fn p(s: &str) -> Node {
        parse(s).unwrap_or_else(|e| panic!("{s}: {e}"))
    }

    #[test]
    fn atoms_and_numbers() {
        assert_eq!(p("x"), Node::Char('x'));
        assert_eq!(p("3.14"), Node::Num("3.14".into()));
        assert_eq!(p("1 000"), Node::Row(vec![Node::Num("1".into()), Node::Num("000".into())]));
        assert_eq!(p("\\alpha+1"), Node::Row(vec![Node::Sym(Sym::Alpha), Node::Sym(Sym::Plus), Node::Num("1".into())]));
        assert_eq!(p("ℝ"), Node::Font(Font::Bb, Box::new(Node::Char('R'))));
    }

    #[test]
    fn scripts_attach_like_tex() {
        let n = p("x_1^2");
        let Node::Scripts(s) = n else { panic!() };
        assert_eq!(s.base, Node::Char('x'));
        assert_eq!(s.sub, Some(Node::Num("1".into())));
        assert_eq!(s.sup, Some(Node::Num("2".into())));
        // x^23: only the first digit is the script
        let Node::Row(v) = p("x^23") else { panic!() };
        assert_eq!(v.len(), 2);
        // f'' and f'^2
        let Node::Scripts(s) = p("f''") else { panic!() };
        assert_eq!(s.primes, 2);
        let Node::Scripts(s) = p("f'^2") else { panic!() };
        assert_eq!((s.primes, s.sup.is_some()), (1, true));
        // a script at the start attaches to an empty group
        let Node::Row(v) = p("^{14}C") else { panic!() };
        let Node::Scripts(s) = &v[0] else { panic!() };
        assert_eq!(s.base, Node::Group(Box::new(empty())));
    }

    #[test]
    fn fracs_roots_accents() {
        assert_eq!(
            p("\\frac12"),
            Node::Frac(FracKind::Frac, Box::new(Node::Num("1".into())), Box::new(Node::Num("2".into())))
        );
        assert_eq!(p("{a \\over b}"), p("{\\frac{a}{b}}"));
        assert_eq!(p("{n \\choose k}"), p("{\\binom{n}{k}}"));
        let Node::Sqrt(Some(i), _) = p("\\sqrt[3]{x}") else { panic!() };
        assert_eq!(*i, Node::Num("3".into()));
        assert_eq!(p("\\hat x"), Node::Accent(Accent::Hat, Box::new(Node::Char('x'))));
        assert_eq!(p("\\not="), Node::Sym(Sym::Ne));
        assert_eq!(p("\\not\\in"), p("\\notin"));
        assert_eq!(p("{\\bf x}"), p("{\\mathbf{x}}"));
    }

    #[test]
    fn left_right_and_envs() {
        let Node::LeftRight(l, _, r) = p("\\left( x \\right.") else { panic!() };
        assert_eq!((l, r), (Some(Sym::LParen), None));
        let Node::Env(e) = p("\\begin{pmatrix} a & b \\\\ c & d \\end{pmatrix}") else { panic!() };
        assert_eq!(e.name, EnvName::Known(EnvKind::PMatrix));
        assert_eq!(e.rows.len(), 2);
        assert_eq!(e.rows[1].len(), 2);
        // top level with & is an implicit aligned
        let Node::Env(e) = p("a &= b \\\\ &= c \\\\") else { panic!() };
        assert_eq!(e.name, EnvName::Known(EnvKind::Aligned));
        assert_eq!(e.rows.len(), 2);
        let Node::Env(e) = p("\\begin{array}{c|c} 1 & 2 \\end{array}") else { panic!() };
        assert_eq!(e.arg.as_deref(), Some("c|c"));
    }

    #[test]
    fn unknown_commands_keep_name_args_position() {
        let n = p("a+\\foo{x}{y}");
        let u = n.unknowns();
        assert_eq!(u.len(), 1);
        assert_eq!(u[0].0, "\\foo");
        assert_eq!(u[0].1.start, 2);
        let Node::Row(v) = n else { panic!() };
        let Node::Unknown(u) = &v[2] else { panic!() };
        assert_eq!(u.args.len(), 2);
    }

    #[test]
    fn errors_have_positions() {
        let e = parse("\\frac{a}{b").unwrap_err();
        assert_eq!((e.kind.clone(), e.pos), (ErrorKind::UnclosedGroup, 8));
        let e = parse("a}+b").unwrap_err();
        assert_eq!((e.kind.clone(), e.pos), (ErrorKind::ExtraClose, 1));
        let e = parse("x+\\left( y").unwrap_err();
        assert_eq!((e.kind.clone(), e.pos), (ErrorKind::MissingRight, 2));
        let e = parse("y \\right)").unwrap_err();
        assert_eq!((e.kind.clone(), e.pos), (ErrorKind::ExtraRight, 2));
        let e = parse("\\begin{matrix} a").unwrap_err();
        assert_eq!(e.kind, ErrorKind::UnclosedEnv("matrix".into()));
        let e = parse("\\begin{matrix} a \\end{pmatrix}").unwrap_err();
        assert_eq!(e.kind.code(), "env-mismatch");
        assert_eq!(parse("x^2^3").unwrap_err().kind, ErrorKind::DoubleSup);
        assert_eq!(parse("x^").unwrap_err().kind.code(), "missing-arg");
    }

    #[test]
    fn macros_in_formula_and_from_document() {
        assert_eq!(p("\\newcommand{\\R}{\\mathbb{R}} x\\in\\R"), p("x\\in\\mathbb{R}"));
        assert_eq!(p("\\def\\sq#1{#1^2} \\sq{y}"), p("y^2"));
        assert_eq!(p("\\newcommand{\\f}[2][0]{#1+#2} \\f{x} \\f[1]{y}"), p("0+x 1+y"));
        let mut m = Macros::new();
        m.scan("\\newcommand{\\norm}[1]{\\left\\|#1\\right\\|}\\DeclareMathOperator{\\rank}{rank}");
        assert_eq!(parse_with("\\norm{x}+\\rank A", &m).unwrap(), p("\\left\\|x\\right\\|+\\operatorname{rank}A"));
        let e = parse("\\def\\a{\\a}\\a").unwrap_err();
        assert_eq!(e.kind, ErrorKind::MacroLoop);
    }
}
