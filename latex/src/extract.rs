//! Extraction of formulas from `.tex` text (or arXiv abstracts): `$…$`, `$$…$$`, `\(…\)`, `\[…\]`, environments
//! `equation`, `align`, `gather`, `multline`, `eqnarray`, `alignat`, `flalign`, `displaymath`, `math`
//! (and starred ones). `%` comments, escaped `\$`, `\verb` and `verbatim`-like environments
//! are skipped. Inline `$…$` does not cross an empty line (as in TeX) and closes only at
//! brace depth 0 (`\text{…$x$…}` inside is part of the formula).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delims {
    Dollar,
    DoubleDollar,
    Paren,
    Bracket,
    Env,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Formula<'a> {
    pub body: &'a str,
    /// Byte offset of the body in the text.
    pub start: usize,
    pub display: bool,
    pub delims: Delims,
    /// Environment (`align*`), if the formula comes from an environment.
    pub env: Option<&'a str>,
}

/// An unterminated formula: where it started and what is missing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unterminated {
    pub start: usize,
    pub what: &'static str,
}

const MATH_ENVS: [&str; 20] = [
    "equation", "equation*", "align", "align*", "alignat", "alignat*", "flalign", "flalign*", "gather", "gather*",
    "multline", "multline*", "eqnarray", "eqnarray*", "displaymath", "math", "dmath", "dmath*", "IEEEeqnarray",
    "IEEEeqnarray*",
];
/// Environments with a mandatory argument right after `\begin{…}`.
const ENVS_WITH_ARG: [&str; 4] = ["alignat", "alignat*", "IEEEeqnarray", "IEEEeqnarray*"];
const VERBATIM_ENVS: [&str; 8] = ["verbatim", "verbatim*", "Verbatim", "lstlisting", "minted", "comment", "spverbatim", "alltt"];

/// All formulas of the text in order of appearance, plus the unterminated ones.
pub fn extract(tex: &str) -> (Vec<Formula<'_>>, Vec<Unterminated>) {
    let b = tex.as_bytes();
    let mut out = Vec::new();
    let mut bad = Vec::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'%' => i = skip_comment(b, i),
            b'\\' => {
                let Some(&n) = b.get(i + 1) else { break };
                match n {
                    b'(' | b'[' => {
                        let closer: &[u8] = if n == b'(' { b"\\)" } else { b"\\]" };
                        match find(b, i + 2, closer) {
                            Some(end) => {
                                out.push(Formula {
                                    body: &tex[i + 2..end],
                                    start: i + 2,
                                    display: n == b'[',
                                    delims: if n == b'(' { Delims::Paren } else { Delims::Bracket },
                                    env: None,
                                });
                                i = end + 2;
                            }
                            None => {
                                bad.push(Unterminated { start: i, what: if n == b'(' { "\\( without \\)" } else { "\\[ without \\]" } });
                                i += 2;
                            }
                        }
                    }
                    c if c.is_ascii_alphabetic() => {
                        let mut j = i + 1;
                        while j < b.len() && b[j].is_ascii_alphabetic() {
                            j += 1;
                        }
                        let name = &tex[i + 1..j];
                        match name {
                            "begin" => i = begin(tex, i, j, &mut out, &mut bad),
                            "verb" => {
                                let mut k = j;
                                if b.get(k) == Some(&b'*') {
                                    k += 1;
                                }
                                match b.get(k) {
                                    Some(&d) => {
                                        let end = b[k + 1..].iter().position(|&x| x == d || x == b'\n');
                                        i = end.map_or(b.len(), |e| k + 1 + e + 1);
                                    }
                                    None => i = b.len(),
                                }
                            }
                            _ => i = j,
                        }
                    }
                    _ => i += 2 + utf8_extra(n),
                }
            }
            b'$' => {
                if b.get(i + 1) == Some(&b'$') {
                    match find_dollars(b, i + 2) {
                        Some(end) => {
                            out.push(Formula {
                                body: &tex[i + 2..end],
                                start: i + 2,
                                display: true,
                                delims: Delims::DoubleDollar,
                                env: None,
                            });
                            i = end + 2;
                        }
                        None => {
                            bad.push(Unterminated { start: i, what: "$$ without $$" });
                            i += 2;
                        }
                    }
                } else {
                    match find_dollar(b, i + 1) {
                        Some(end) => {
                            out.push(Formula { body: &tex[i + 1..end], start: i + 1, display: false, delims: Delims::Dollar, env: None });
                            i = end + 1;
                        }
                        None => {
                            bad.push(Unterminated { start: i, what: "$ without $" });
                            i += 1;
                        }
                    }
                }
            }
            _ => i += 1,
        }
    }
    (out, bad)
}

fn utf8_extra(first: u8) -> usize {
    match first {
        0xC0..=0xDF => 1,
        0xE0..=0xEF => 2,
        0xF0..=0xFF => 3,
        _ => 0,
    }
}

fn skip_comment(b: &[u8], mut i: usize) -> usize {
    while i < b.len() && b[i] != b'\n' {
        i += 1;
    }
    i
}

fn find(b: &[u8], from: usize, pat: &[u8]) -> Option<usize> {
    if from >= b.len() {
        return None;
    }
    b[from..].windows(pat.len()).position(|w| w == pat).map(|p| from + p)
}

/// After `\begin` (the name ended at `j`): `{env}`; a formula or a skipped verbatim.
fn begin<'a>(tex: &'a str, i: usize, j: usize, out: &mut Vec<Formula<'a>>, bad: &mut Vec<Unterminated>) -> usize {
    let b = tex.as_bytes();
    let mut k = j;
    while k < b.len() && b[k].is_ascii_whitespace() {
        k += 1;
    }
    if b.get(k) != Some(&b'{') {
        return j;
    }
    let Some(close) = b[k..].iter().position(|&x| x == b'}').map(|p| k + p) else { return j };
    let env = &tex[k + 1..close];
    let is_math = MATH_ENVS.contains(&env);
    let is_verb = VERBATIM_ENVS.contains(&env);
    if !is_math && !is_verb {
        return close + 1;
    }
    let end_pat = format!("\\end{{{env}}}");
    let mut body_start = close + 1;
    if is_math && ENVS_WITH_ARG.contains(&env) {
        // `{n}` right after \begin{alignat}
        let mut m = body_start;
        while m < b.len() && b[m].is_ascii_whitespace() {
            m += 1;
        }
        if b.get(m) == Some(&b'{') {
            if let Some(p) = b[m..].iter().position(|&x| x == b'}') {
                body_start = m + p + 1;
            }
        }
    }
    match find(b, body_start, end_pat.as_bytes()) {
        Some(end) => {
            if is_math {
                out.push(Formula {
                    body: &tex[body_start..end],
                    start: body_start,
                    display: env != "math",
                    delims: Delims::Env,
                    env: Some(env),
                });
            }
            end + end_pat.len()
        }
        None => {
            if is_math {
                bad.push(Unterminated { start: i, what: "\\begin without \\end" });
            }
            close + 1
        }
    }
}

/// The closing `$$` (escaped text and comments are skipped).
fn find_dollars(b: &[u8], mut i: usize) -> Option<usize> {
    while i < b.len() {
        match b[i] {
            b'\\' => i += 2,
            b'%' => i = skip_comment(b, i),
            b'$' if b.get(i + 1) == Some(&b'$') => return Some(i),
            _ => i += 1,
        }
    }
    None
}

/// The closing `$` at brace depth 0; an empty line ends the paragraph, leaving the formula unterminated.
fn find_dollar(b: &[u8], mut i: usize) -> Option<usize> {
    let mut depth = 0i32;
    while i < b.len() {
        match b[i] {
            b'\\' => i += 2,
            b'%' => i = skip_comment(b, i),
            b'{' => {
                depth += 1;
                i += 1;
            }
            b'}' => {
                depth -= 1;
                i += 1;
            }
            b'$' if depth <= 0 => return Some(i),
            b'\n' => {
                let mut k = i + 1;
                while k < b.len() && matches!(b[k], b' ' | b'\t' | b'\r') {
                    k += 1;
                }
                if b.get(k) == Some(&b'\n') {
                    return None;
                }
                i += 1;
            }
            _ => i += 1,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_all_kinds() {
        let tex = r"Let $x^2$ and \(y\) be. % $not$ this
Costs \$5. $$a=b$$ \[c\]
\begin{align*} a &= b \\ c &= d \end{align*}
\begin{verbatim} $v$ \end{verbatim} \verb|$w$| $\text{if $z$}$
\begin{alignat}{2} p &= q \end{alignat}";
        let (f, bad) = extract(tex);
        let bodies: Vec<&str> = f.iter().map(|f| f.body).collect();
        assert_eq!(bodies, vec!["x^2", "y", "a=b", "c", " a &= b \\\\ c &= d ", "\\text{if $z$}", " p &= q "]);
        assert!(bad.is_empty());
        assert_eq!(f[4].env, Some("align*"));
        assert!(f[4].display && !f[0].display);
        assert_eq!(&tex[f[0].start..f[0].start + 3], "x^2");
    }

    #[test]
    fn unterminated_inline_stops_at_paragraph() {
        let (f, bad) = extract("a $x+1\n\nb $y$");
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].body, "y");
        assert_eq!(bad.len(), 1);
    }
}
