//! Output: disp, fprintf/printf, sprintf (a subset of C format with cycling over the arguments), num2str, mat2str, int2str.

use super::*;
use crate::display::format_disp;

pub fn register(r: &mut Registry) {
    r.register("disp", |it, a, _| {
        let v = arg(a, 0, "disp")?;
        let s = format_disp(v, it.fmt);
        it.out(&s);
        Ok(vec![])
    });
    r.register("display", |it, a, _| {
        let v = arg(a, 0, "display")?.clone();
        it.display("ans", &v);
        Ok(vec![])
    });
    r.register("fdisp", |it, a, _| {
        let v = arg(a, 1, "fdisp")?;
        let s = format_disp(v, it.fmt);
        it.out(&s);
        Ok(vec![])
    });
    r.register("puts", |it, a, _| {
        let s = str_arg(a, 0, "puts")?;
        it.out(&s);
        Ok(vec![])
    });
    r.register("fputs", |it, a, _| {
        let s = str_arg(a, 1, "fputs")?;
        it.out(&s);
        Ok(vec![])
    });
    r.register("fprintf", fprintf);
    r.register("printf", fprintf);
    r.register("sprintf", |it, a, _| {
        let fmt = str_arg(a, 0, "sprintf")?;
        let s = sprintf_impl(it, &fmt, &a[1..])?;
        Ok(vec![Value::str(&s)])
    });
    r.register("num2str", num2str);
    r.register("int2str", |it, a, n| {
        let m = mat_arg(a, 0, "int2str")?;
        let rounded = Mat::new(m.rows, m.cols, m.re.iter().map(|x| x.round()).collect());
        num2str(it, &[Value::Mat(rounded)], n)
    });
    r.register("mat2str", mat2str);
}

fn fprintf(it: &mut Interp, a: &[Value], nargout: usize) -> Result<Vec<Value>, MError> {
    let mut args = a;
    // fprintf(fid, fmt, ...)
    if let Some(Value::Mat(m)) = args.first() {
        if m.class != Class::Char && m.numel() == 1 && args.len() >= 2 && is_str(&args[1]) {
            args = &args[1..];
        } else if m.class != Class::Char {
            // fprintf(x) — print the number in a %g-like way
            let s = sprintf_impl(it, "%g\n", args)?;
            it.out(&s);
            return Ok(vec![]);
        }
    }
    let fmt = str_arg(args, 0, "fprintf")?;
    let s = sprintf_impl(it, &fmt, &args[1..])?;
    let n = s.len();
    it.out(&s);
    if nargout > 0 { num(n as f64) } else { Ok(vec![]) }
}

#[derive(Clone, Debug)]
enum Atom {
    Num(f64),
    Str(String),
}

#[derive(Debug)]
enum Seg {
    Lit(String),
    Spec { flags: String, width: Option<usize>, prec: Option<usize>, conv: char },
}

fn unescape(s: &str) -> String {
    let mut out = String::new();
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == '\\' && i + 1 < cs.len() {
            let e = cs[i + 1];
            let r = match e {
                'n' => Some('\n'),
                't' => Some('\t'),
                'r' => Some('\r'),
                'a' => Some('\u{7}'),
                'b' => Some('\u{8}'),
                'f' => Some('\u{c}'),
                'v' => Some('\u{b}'),
                '\\' => Some('\\'),
                '"' => Some('"'),
                '\'' => Some('\''),
                '0' => Some('\0'),
                _ => None,
            };
            if let Some(r) = r {
                out.push(r);
                i += 2;
                continue;
            }
        }
        out.push(cs[i]);
        i += 1;
    }
    out
}

fn parse_format(fmt: &str) -> Vec<Seg> {
    let cs: Vec<char> = fmt.chars().collect();
    let mut segs = Vec::new();
    let mut lit = String::new();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] != '%' {
            lit.push(cs[i]);
            i += 1;
            continue;
        }
        if i + 1 < cs.len() && cs[i + 1] == '%' {
            lit.push('%');
            i += 2;
            continue;
        }
        // specifier
        let start = i;
        i += 1;
        let mut flags = String::new();
        while i < cs.len() && "-+ 0#".contains(cs[i]) {
            flags.push(cs[i]);
            i += 1;
        }
        let mut width = None;
        let mut w = String::new();
        while i < cs.len() && cs[i].is_ascii_digit() {
            w.push(cs[i]);
            i += 1;
        }
        if !w.is_empty() {
            width = w.parse().ok();
        }
        let mut prec = None;
        if i < cs.len() && cs[i] == '.' {
            i += 1;
            let mut p = String::new();
            while i < cs.len() && cs[i].is_ascii_digit() {
                p.push(cs[i]);
                i += 1;
            }
            prec = Some(p.parse().unwrap_or(0));
        }
        // length modifiers (l, h) are skipped
        while i < cs.len() && matches!(cs[i], 'l' | 'h' | 'L' | 'q' | 'j' | 'z' | 't') {
            i += 1;
        }
        if i < cs.len() && "diufFeEgGxXocs".contains(cs[i]) {
            if !lit.is_empty() {
                segs.push(Seg::Lit(unescape(&std::mem::take(&mut lit))));
            }
            segs.push(Seg::Spec { flags, width, prec, conv: cs[i] });
            i += 1;
        } else {
            // unknown — as text
            let end = i.min(cs.len());
            lit.extend(&cs[start..end]);
        }
    }
    if !lit.is_empty() {
        segs.push(Seg::Lit(unescape(&lit)));
    }
    segs
}

fn atoms(args: &[Value]) -> Result<std::collections::VecDeque<Atom>, MError> {
    let mut out = std::collections::VecDeque::new();
    for v in args {
        match v {
            Value::Mat(m) if m.class == Class::Char => out.push_back(Atom::Str(m.to_string_lossy())),
            Value::Mat(m) => {
                for &x in &m.re {
                    out.push_back(Atom::Num(x));
                }
            }
            Value::Func(f) => out.push_back(Atom::Str(crate::display::unparse_func(f))),
            other => return Err(MError::new(format!("printf: wrong type argument '{}'", other.class_name()))),
        }
    }
    Ok(out)
}

/// Shortest representation that reproduces the number (like «%d» for non-integers in Octave: 1.5 → «1.5»).
pub fn shortest(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Inf".into() } else { "-Inf".into() };
    }
    if x == x.trunc() && x.abs() < 1e15 {
        return format!("{}", x as i64);
    }
    let s = format!("{x}");
    if s.len() > 20 { c_exp(x, 15, false) } else { s }
}

pub fn c_exp(x: f64, prec: usize, upper: bool) -> String {
    let s = crate::display::exp_str(x, prec);
    if upper { s.to_uppercase() } else { s }
}

/// C-like %g.
pub fn c_g(x: f64, prec: usize, alt: bool, upper: bool) -> String {
    if x == 0.0 {
        return if alt { format!("{:.*}", prec.max(1) - 1, 0.0) } else { "0".into() };
    }
    let p = if prec == 0 { 1 } else { prec };
    let e = crate::display::exp_str(x, p - 1);
    let exp: i32 = e.split('e').nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let mut s = if exp < -4 || exp >= p as i32 {
        c_exp(x, p - 1, upper)
    } else {
        format!("{:.*}", (p as i32 - 1 - exp).max(0) as usize, x)
    };
    if !alt {
        // strip trailing zeros of the mantissa
        if let Some(epos) = s.find(['e', 'E']) {
            let (m, ex) = s.split_at(epos);
            let m = if m.contains('.') { m.trim_end_matches('0').trim_end_matches('.') } else { m };
            s = format!("{m}{ex}");
        } else if s.contains('.') {
            s = s.trim_end_matches('0').trim_end_matches('.').to_string();
        }
    }
    s
}

fn pad(s: String, flags: &str, width: Option<usize>, numeric: bool) -> String {
    let w = match width {
        Some(w) => w,
        None => return s,
    };
    let len = s.chars().count();
    if len >= w {
        return s;
    }
    if flags.contains('-') {
        return format!("{s}{}", " ".repeat(w - len));
    }
    if flags.contains('0') && numeric && !s.contains("Inf") && !s.contains("NaN") {
        let (sign, rest) = if s.starts_with(['-', '+', ' ']) { s.split_at(1) } else { ("", s.as_str()) };
        return format!("{sign}{}{rest}", "0".repeat(w - len));
    }
    format!("{}{s}", " ".repeat(w - len))
}

fn with_sign(s: String, x: f64, flags: &str) -> String {
    if x >= 0.0 && !s.starts_with('-') {
        if flags.contains('+') {
            return format!("+{s}");
        }
        if flags.contains(' ') {
            return format!(" {s}");
        }
    }
    s
}

fn fmt_num(x: f64, flags: &str, width: Option<usize>, prec: Option<usize>, conv: char) -> String {
    let nonfinite = if x.is_nan() {
        Some("NaN".to_string())
    } else if x.is_infinite() {
        Some(if x > 0.0 { "Inf".to_string() } else { "-Inf".to_string() })
    } else {
        None
    };
    if let Some(s) = nonfinite {
        return pad(with_sign(s, x, flags), flags, width, false);
    }
    let s = match conv {
        'd' | 'i' | 'u' => {
            if x == x.trunc() && x.abs() < 1e18 {
                format!("{}", x as i64)
            } else {
                // non-integers under %d — shortest representation
                return pad(with_sign(shortest(x), x, flags), flags, width, true);
            }
        }
        'f' | 'F' => format!("{:.*}", prec.unwrap_or(6), x),
        'e' | 'E' => c_exp(x, prec.unwrap_or(6), conv == 'E'),
        'g' | 'G' => {
            // C: printf('%g', -0) prints «-0» (vmm-edge-023, tests3)
            let s = c_g(x, prec.unwrap_or(6), flags.contains('#'), conv == 'G');
            if x == 0.0 && x.is_sign_negative() { format!("-{s}") } else { s }
        }
        'x' | 'X' | 'o' => {
            if x == x.trunc() && x >= 0.0 {
                let n = x as u64;
                match conv {
                    'x' => format!("{n:x}"),
                    'X' => format!("{n:X}"),
                    _ => format!("{n:o}"),
                }
            } else {
                shortest(x)
            }
        }
        _ => shortest(x),
    };
    pad(with_sign(s, x, flags), flags, width, true)
}

pub fn sprintf_impl(_it: &mut Interp, fmt: &str, args: &[Value]) -> Result<String, MError> {
    let segs = parse_format(fmt);
    let mut q = atoms(args)?;
    let nspecs = segs.iter().filter(|s| matches!(s, Seg::Spec { .. })).count();
    let mut out = String::new();
    if nspecs == 0 {
        for s in &segs {
            if let Seg::Lit(l) = s {
                out.push_str(l);
            }
        }
        return Ok(out);
    }
    let no_args = q.is_empty();
    loop {
        for seg in &segs {
            match seg {
                Seg::Lit(l) => out.push_str(l),
                Seg::Spec { flags, width, prec, conv } => {
                    if q.is_empty() {
                        if no_args {
                            // format without data: specifiers produce empty output
                            out.push_str(&pad(String::new(), flags, *width, false));
                            continue;
                        }
                        return Ok(out);
                    }
                    let atom = q.pop_front().unwrap();
                    match (conv, atom) {
                        ('s', Atom::Str(s)) | ('c', Atom::Str(s)) => {
                            let s = match prec {
                                Some(p) if *conv == 's' => s.chars().take(*p).collect(),
                                _ => s,
                            };
                            out.push_str(&pad(s, flags, *width, false));
                        }
                        ('s', Atom::Num(x)) | ('c', Atom::Num(x)) => {
                            let s = if x == x.trunc() && x >= 0.0 && x < 1_114_112.0 {
                                char::from_u32(x as u32).map(|c| c.to_string()).unwrap_or_else(|| shortest(x))
                            } else {
                                shortest(x)
                            };
                            out.push_str(&pad(s, flags, *width, false));
                        }
                        (_, Atom::Str(s)) => {
                            // numeric specifier and a string — character codes
                            let codes: Vec<Atom> = s.chars().map(|c| Atom::Num(c as u32 as f64)).collect();
                            if codes.is_empty() {
                                continue;
                            }
                            for a in codes.into_iter().rev() {
                                q.push_front(a);
                            }
                            if let Some(Atom::Num(x)) = q.pop_front() {
                                out.push_str(&fmt_num(x, flags, *width, *prec, *conv));
                            }
                        }
                        (_, Atom::Num(x)) => out.push_str(&fmt_num(x, flags, *width, *prec, *conv)),
                    }
                }
            }
        }
        if q.is_empty() {
            break;
        }
    }
    Ok(out)
}

fn g_prec(x: f64, p: usize) -> String {
    c_g(x, p, false, false)
}

fn num2str(it: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let v = arg(a, 0, "num2str")?;
    if is_str(v) {
        return Ok(vec![v.clone()]);
    }
    let m = mat_arg(a, 0, "num2str")?;
    if let Some(f) = a.get(1) {
        if let Some(fmt) = str_of(f) {
            let s = sprintf_impl(it, &fmt, &[v.clone()])?;
            return Ok(vec![Value::str(&s)]);
        }
        let p = scalar_arg(a, 1, "num2str")? as usize;
        let s: Vec<String> = m.re.iter().map(|&x| g_prec(x, p)).collect();
        return Ok(vec![Value::str(&s.join("  "))]);
    }
    if m.is_empty() {
        return Ok(vec![Value::str("")]);
    }
    let all_int = m.re.iter().all(|&x| x == x.trunc() || !x.is_finite());
    let fmt_one = |x: f64| -> String {
        if !x.is_finite() {
            return shortest(x);
        }
        if all_int {
            return format!("{}", x as i64);
        }
        let d = if x == 0.0 { 0 } else { x.abs().log10().floor() as i32 };
        let p = (d + 5).clamp(5, 16) as usize;
        g_prec(x, p)
    };
    if let Some(im) = &m.im {
        if m.is_scalar() {
            let re = fmt_one(m.re[0]);
            let iv = fmt_one(im[0].abs());
            let sign = if im[0] < 0.0 { "-" } else { "+" };
            return Ok(vec![Value::str(&format!("{re}{sign}{iv}i"))]);
        }
    }
    if m.is_scalar() {
        return Ok(vec![Value::str(&fmt_one(m.re[0]))]);
    }
    // matrix: columns aligned, separator — two spaces
    let cells: Vec<Vec<String>> = (0..m.rows).map(|i| (0..m.cols).map(|j| fmt_one(m.at(i, j))).collect()).collect();
    let w = cells.iter().flatten().map(|s| s.len()).max().unwrap_or(1);
    let lines: Vec<String> = cells
        .iter()
        .map(|row| row.iter().map(|s| format!("{s:>w$}")).collect::<Vec<_>>().join("  "))
        .collect();
    // strip the common leading indentation
    let lead = lines.iter().map(|l| l.len() - l.trim_start().len()).min().unwrap_or(0);
    let rows: Vec<Mat> = lines.iter().map(|l| Mat::str(&l[lead..])).collect();
    Ok(vec![Value::Mat(crate::ops::vcat(rows)?)])
}

fn mat2str(_: &mut Interp, a: &[Value], _: usize) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, "mat2str")?;
    let p = if a.len() > 1 { scalar_arg(a, 1, "mat2str")? as usize } else { 15 };
    let el = |k: usize| -> String {
        match m.class {
            Class::Logical => (if m.re[k] != 0.0 { "true" } else { "false" }).to_string(),
            _ => {
                let re = g_prec(m.re[k], p);
                match &m.im {
                    Some(im) => {
                        let iv = im[k];
                        let s = g_prec(iv.abs(), p);
                        format!("{re}{}{s}i", if iv < 0.0 { "-" } else { "+" })
                    }
                    None => re,
                }
            }
        }
    };
    if m.class == Class::Char {
        if m.rows <= 1 {
            return Ok(vec![Value::str(&format!("\"{}\"", m.to_string_lossy()))]);
        }
        let rows: Vec<String> = (0..m.rows).map(|i| format!("\"{}\"", m.row_string(i))).collect();
        return Ok(vec![Value::str(&format!("[{}]", rows.join(";")))]);
    }
    if m.is_scalar() {
        return Ok(vec![Value::str(&el(0))]);
    }
    let rows: Vec<String> = (0..m.rows).map(|i| (0..m.cols).map(|j| el(j * m.rows + i)).collect::<Vec<_>>().join(" ")).collect();
    Ok(vec![Value::str(&format!("[{}]", rows.join(";")))])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn g_format() {
        assert_eq!(c_g(0.0001, 6, false, false), "0.0001");
        assert_eq!(c_g(1e6, 6, false, false), "1e+06");
        assert_eq!(c_g(3.14159265, 5, false, false), "3.1416");
        assert_eq!(c_g(100000.0, 6, false, false), "100000");
        assert_eq!(c_g(0.5, 15, false, false), "0.5");
    }
}
