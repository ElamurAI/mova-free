//! Lexer. Context rules of the language: an apostrophe — transpose or start of a string; a space in `[...]` —
//! element separator (the parser decides this via the `space_before` flag); `%`/`#` — comments,
//! `%{ … %}` — block comment; `...` — line continuation; a newline inside `(...)` — whitespace.

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Num(f64, String),
    Imag(f64, String),
    Str(String, bool),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Backslash,
    Caret,
    DotStar,
    DotSlash,
    DotBackslash,
    DotCaret,
    Quote,
    DotQuote,
    Assign,
    EqEq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    AndAnd,
    OrOr,
    Not,
    Colon,
    Comma,
    Semi,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    At,
    Dot,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    Newline,
    Eof,
}

impl Tok {
    pub fn describe(&self) -> String {
        match self {
            Tok::Num(_, t) | Tok::Imag(_, t) => t.clone(),
            Tok::Str(s, _) => format!("'{s}'"),
            Tok::Ident(s) => s.clone(),
            Tok::Newline => "end of line".into(),
            Tok::Eof => "end of input".into(),
            other => other.text().into(),
        }
    }
    pub fn text(&self) -> &'static str {
        match self {
            Tok::Plus => "+",
            Tok::Minus => "-",
            Tok::Star => "*",
            Tok::Slash => "/",
            Tok::Backslash => "\\",
            Tok::Caret => "^",
            Tok::DotStar => ".*",
            Tok::DotSlash => "./",
            Tok::DotBackslash => ".\\",
            Tok::DotCaret => ".^",
            Tok::Quote => "'",
            Tok::DotQuote => ".'",
            Tok::Assign => "=",
            Tok::EqEq => "==",
            Tok::Ne => "!=",
            Tok::Lt => "<",
            Tok::Le => "<=",
            Tok::Gt => ">",
            Tok::Ge => ">=",
            Tok::And => "&",
            Tok::Or => "|",
            Tok::AndAnd => "&&",
            Tok::OrOr => "||",
            Tok::Not => "!",
            Tok::Colon => ":",
            Tok::Comma => ",",
            Tok::Semi => ";",
            Tok::LParen => "(",
            Tok::RParen => ")",
            Tok::LBracket => "[",
            Tok::RBracket => "]",
            Tok::LBrace => "{",
            Tok::RBrace => "}",
            Tok::At => "@",
            Tok::Dot => ".",
            Tok::PlusEq => "+=",
            Tok::MinusEq => "-=",
            Tok::StarEq => "*=",
            Tok::SlashEq => "/=",
            _ => "?",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub tok: Tok,
    pub line: usize,
    pub col: usize,
    pub space_before: bool,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub col: usize,
    pub msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error near line {}, column {}: {}", self.line, self.col, self.msg)
    }
}

pub const KEYWORDS: &[&str] = &[
    "if", "elseif", "else", "end", "endif", "for", "endfor", "parfor", "endparfor", "while", "endwhile", "do",
    "until", "switch", "case", "otherwise", "endswitch", "function", "endfunction", "return", "break",
    "continue", "try", "catch", "end_try_catch", "global", "persistent", "unwind_protect",
    "unwind_protect_cleanup", "end_unwind_protect",
];

pub fn is_keyword(s: &str) -> bool {
    KEYWORDS.contains(&s)
}

pub fn lex(src: &str) -> Result<Vec<Token>, ParseError> {
    let chars: Vec<char> = src.chars().collect();
    let n = chars.len();
    let mut i = 0usize;
    let mut line = 1usize;
    let mut col = 1usize;
    let mut out: Vec<Token> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    let mut space = false;
    let mut line_start = true;

    // helper: contents of the current line from position p, trimmed
    let line_rest = |p: usize| -> String {
        let mut e = p;
        while e < n && chars[e] != '\n' {
            e += 1;
        }
        chars[p..e].iter().collect::<String>().trim().to_string()
    };

    while i < n {
        let c = chars[i];
        if c == ' ' || c == '\t' || c == '\r' {
            space = true;
            i += 1;
            col += 1;
            continue;
        }
        if c == '\n' {
            // a newline is always a token: inside (...) without «...» it is a parse error, as in MATLAB
            out.push(Token { tok: Tok::Newline, line, col, space_before: space });
            i += 1;
            line += 1;
            col = 1;
            space = false;
            line_start = true;
            continue;
        }
        // block comment: «%{» / «#{» alone on a line
        if line_start && (c == '%' || c == '#') {
            let rest = line_rest(i);
            if rest == "%{" || rest == "#{" {
                let mut depth = 0usize;
                // go line by line
                loop {
                    let r = line_rest(i);
                    if r == "%{" || r == "#{" {
                        depth += 1;
                    } else if r == "%}" || r == "#}" {
                        depth -= 1;
                    }
                    while i < n && chars[i] != '\n' {
                        i += 1;
                    }
                    if i < n {
                        i += 1;
                        line += 1;
                        col = 1;
                    }
                    if depth == 0 || i >= n {
                        break;
                    }
                    // skip the next line's indentation so that line_rest sees its start
                }
                continue;
            }
        }
        if c == '%' || c == '#' {
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '.' && i + 2 < n && chars[i + 1] == '.' && chars[i + 2] == '.' {
            // continuation: the rest of the line is a comment
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            if i < n {
                i += 1;
                line += 1;
                col = 1;
            }
            space = true;
            continue;
        }
        line_start = false;
        let start_col = col;
        let sb = space;
        space = false;
        let push = |out: &mut Vec<Token>, tok: Tok| {
            out.push(Token { tok, line, col: start_col, space_before: sb });
        };

        // numbers
        if c.is_ascii_digit() || (c == '.' && i + 1 < n && chars[i + 1].is_ascii_digit()) {
            let st = i;
            while i < n && chars[i].is_ascii_digit() {
                i += 1;
            }
            if i < n && chars[i] == '.' {
                let nx = if i + 1 < n { chars[i + 1] } else { ' ' };
                let is_op = matches!(nx, '*' | '/' | '\\' | '^' | '\'');
                if !is_op {
                    i += 1;
                    while i < n && chars[i].is_ascii_digit() {
                        i += 1;
                    }
                }
            }
            if i < n && matches!(chars[i], 'e' | 'E' | 'd' | 'D') {
                let mut k = i + 1;
                if k < n && (chars[k] == '+' || chars[k] == '-') {
                    k += 1;
                }
                if k < n && chars[k].is_ascii_digit() {
                    i = k;
                    while i < n && chars[i].is_ascii_digit() {
                        i += 1;
                    }
                }
            }
            let text: String = chars[st..i].iter().collect();
            let norm = text.replace(['d', 'D'], "e");
            let v: f64 = norm.parse().map_err(|_| ParseError { line, col: start_col, msg: format!("bad number '{text}'") })?;
            col += i - st;
            if i < n && matches!(chars[i], 'i' | 'j' | 'I' | 'J') && !(i + 1 < n && (chars[i + 1].is_alphanumeric() || chars[i + 1] == '_')) {
                i += 1;
                col += 1;
                push(&mut out, Tok::Imag(v, format!("{text}i")));
            } else {
                push(&mut out, Tok::Num(v, text));
            }
            continue;
        }
        // identifiers
        if c.is_alphabetic() || c == '_' {
            let st = i;
            while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let text: String = chars[st..i].iter().collect();
            col += i - st;
            push(&mut out, Tok::Ident(text));
            continue;
        }
        // strings
        let in_matrix = matches!(stack.last(), Some('[') | Some('{'));
        if c == '\'' {
            let prev_value = match out.last() {
                Some(t) => {
                    matches!(t.tok, Tok::Num(..) | Tok::Imag(..) | Tok::RParen | Tok::RBracket | Tok::RBrace | Tok::Quote | Tok::DotQuote | Tok::Str(..))
                        || matches!(&t.tok, Tok::Ident(s) if !is_keyword(s))
                }
                None => false,
            };
            if prev_value && !(sb && in_matrix) {
                i += 1;
                col += 1;
                push(&mut out, Tok::Quote);
                continue;
            }
            // single-quoted string: '' — a quote
            let mut s = String::new();
            i += 1;
            col += 1;
            loop {
                if i >= n || chars[i] == '\n' {
                    return Err(ParseError { line, col: start_col, msg: "unterminated character string constant".into() });
                }
                if chars[i] == '\'' {
                    if i + 1 < n && chars[i + 1] == '\'' {
                        s.push('\'');
                        i += 2;
                        col += 2;
                        continue;
                    }
                    i += 1;
                    col += 1;
                    break;
                }
                s.push(chars[i]);
                i += 1;
                col += 1;
            }
            push(&mut out, Tok::Str(s, false));
            continue;
        }
        if c == '"' {
            let mut s = String::new();
            i += 1;
            col += 1;
            loop {
                if i >= n || chars[i] == '\n' {
                    return Err(ParseError { line, col: start_col, msg: "unterminated character string constant".into() });
                }
                let ch = chars[i];
                if ch == '"' {
                    if i + 1 < n && chars[i + 1] == '"' {
                        s.push('"');
                        i += 2;
                        col += 2;
                        continue;
                    }
                    i += 1;
                    col += 1;
                    break;
                }
                if ch == '\\' && i + 1 < n {
                    let e = chars[i + 1];
                    let r = match e {
                        'n' => Some('\n'),
                        't' => Some('\t'),
                        'r' => Some('\r'),
                        'a' => Some('\u{7}'),
                        '0' => Some('\0'),
                        '\\' => Some('\\'),
                        '"' => Some('"'),
                        '\'' => Some('\''),
                        _ => None,
                    };
                    if let Some(r) = r {
                        s.push(r);
                        i += 2;
                        col += 2;
                        continue;
                    }
                }
                s.push(ch);
                i += 1;
                col += 1;
            }
            push(&mut out, Tok::Str(s, true));
            continue;
        }
        // operators
        let nx = if i + 1 < n { chars[i + 1] } else { '\0' };
        let nx2 = if i + 2 < n { chars[i + 2] } else { '\0' };
        let (tok, len) = match c {
            '+' if nx == '=' && nx2 != '=' => (Tok::PlusEq, 2),
            '-' if nx == '=' && nx2 != '=' => (Tok::MinusEq, 2),
            '*' if nx == '=' && nx2 != '=' => (Tok::StarEq, 2),
            '/' if nx == '=' && nx2 != '=' => (Tok::SlashEq, 2),
            '+' => (Tok::Plus, 1),
            '-' => (Tok::Minus, 1),
            '*' if nx == '*' => (Tok::Caret, 2),
            '*' => (Tok::Star, 1),
            '/' => (Tok::Slash, 1),
            '\\' => (Tok::Backslash, 1),
            '^' => (Tok::Caret, 1),
            '.' if nx == '*' => (Tok::DotStar, 2),
            '.' if nx == '/' => (Tok::DotSlash, 2),
            '.' if nx == '\\' => (Tok::DotBackslash, 2),
            '.' if nx == '^' => (Tok::DotCaret, 2),
            '.' if nx == '\'' => (Tok::DotQuote, 2),
            '.' => (Tok::Dot, 1),
            '=' if nx == '=' => (Tok::EqEq, 2),
            '=' => (Tok::Assign, 1),
            '~' | '!' if nx == '=' => (Tok::Ne, 2),
            '~' | '!' => (Tok::Not, 1),
            '<' if nx == '=' => (Tok::Le, 2),
            '<' => (Tok::Lt, 1),
            '>' if nx == '=' => (Tok::Ge, 2),
            '>' => (Tok::Gt, 1),
            '&' if nx == '&' => (Tok::AndAnd, 2),
            '&' => (Tok::And, 1),
            '|' if nx == '|' => (Tok::OrOr, 2),
            '|' => (Tok::Or, 1),
            ':' => (Tok::Colon, 1),
            ',' => (Tok::Comma, 1),
            ';' => (Tok::Semi, 1),
            '(' => {
                stack.push('(');
                (Tok::LParen, 1)
            }
            '[' => {
                stack.push('[');
                (Tok::LBracket, 1)
            }
            '{' => {
                stack.push('{');
                (Tok::LBrace, 1)
            }
            ')' | ']' | '}' => {
                stack.pop();
                (
                    match c {
                        ')' => Tok::RParen,
                        ']' => Tok::RBracket,
                        _ => Tok::RBrace,
                    },
                    1,
                )
            }
            '@' => (Tok::At, 1),
            _ => {
                return Err(ParseError { line, col: start_col, msg: format!("invalid character '{c}'") });
            }
        };
        i += len;
        col += len;
        push(&mut out, tok);
    }
    out.push(Token { tok: Tok::Eof, line, col, space_before: space });
    Ok(out)
}
