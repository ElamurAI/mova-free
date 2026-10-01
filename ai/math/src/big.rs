//! v2: big integers and rationals without overflow — `dashu` 0.6 (MIT OR Apache-2.0; chosen by the benchmark
//! `bigbench/`: GCD 16 times, rationals ~100 times faster than num-bigint/num-rational). Here — only
//! thin helpers: parsing, printing, conversions. v1 (`rat.rs`, i128) remains for units and weather.

pub use dashu::base::{Abs, BitTest, ExtendedGcd, Gcd, Sign, Signed, SquareRoot};
pub use dashu::integer::{IBig, UBig};
pub use dashu::rational::RBig;

/// Arbitrary-length integer.
pub type Z = IBig;
/// Arbitrary-length rational, always reduced.
pub type Q = RBig;

pub fn z(i: i64) -> Z {
    IBig::from(i)
}

pub fn q(i: i64) -> Q {
    RBig::from(IBig::from(i))
}

pub fn qz(x: Z) -> Q {
    RBig::from(x)
}

/// n/d; d = 0 — None.
pub fn frac(n: Z, d: Z) -> Option<Q> {
    if d == IBig::ZERO {
        return None;
    }
    let neg = (n < IBig::ZERO) != (d < IBig::ZERO);
    let nu: IBig = if n < IBig::ZERO { -n } else { n };
    let du: UBig = UBig::try_from(if d < IBig::ZERO { -d } else { d }).ok()?;
    let r = RBig::from_parts(nu, du);
    Some(if neg { -r } else { r })
}

pub fn is_int(x: &Q) -> bool {
    x.is_int()
}

pub fn to_z(x: &Q) -> Option<Z> {
    x.is_int().then(|| x.numerator().clone())
}

pub fn to_i64(x: &Q) -> Option<i64> {
    to_z(x).and_then(|z| i64::try_from(&z).ok())
}

pub fn z_to_i64(x: &Z) -> Option<i64> {
    i64::try_from(x).ok()
}

pub fn z_to_u64(x: &Z) -> Option<u64> {
    u64::try_from(x).ok()
}

pub fn to_f64(x: &Q) -> f64 {
    x.to_f64().value()
}

pub fn z_f64(x: &Z) -> f64 {
    x.to_f64().value()
}

pub fn abs_z(x: &Z) -> Z {
    if *x < IBig::ZERO { -x.clone() } else { x.clone() }
}

pub fn uz(x: &Z) -> UBig {
    UBig::try_from(abs_z(x)).expect("abs ≥ 0")
}

pub fn zu(x: UBig) -> Z {
    IBig::from(x)
}

/// "123", "-4.25", "3/4", "1,234", "1e6", "2.5e-3". A comma between digit triples is a thousands separator.
pub fn parse_q(s: &str) -> Option<Q> {
    let s = s.trim().replace(['_', ' '], "");
    if s.is_empty() {
        return None;
    }
    if let Some((a, b)) = s.split_once('/') {
        let a = parse_q(a)?;
        let b = parse_q(b)?;
        if b.is_zero() {
            return None;
        }
        return Some(a / b);
    }
    let s = if s.contains(',') {
        // only "1,234,567": groups of three
        let parts: Vec<&str> = s.split(',').collect();
        if parts[1..].iter().all(|p| p.len() >= 3 && p[..3].chars().all(|c| c.is_ascii_digit())) { s.replace(',', "") } else { return None }
    } else {
        s
    };
    let (mant, exp) = match s.find(['e', 'E']) {
        Some(i) => (&s[..i], s[i + 1..].parse::<i64>().ok()?),
        None => (s.as_str(), 0),
    };
    let (neg, mant) = match mant.strip_prefix('-') {
        Some(m) => (true, m),
        None => (false, mant.strip_prefix('+').unwrap_or(mant)),
    };
    let (ip, fp) = mant.split_once('.').unwrap_or((mant, ""));
    if (ip.is_empty() && fp.is_empty()) || !ip.chars().all(|c| c.is_ascii_digit()) || !fp.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let digits = format!("{ip}{fp}");
    let n: IBig = if digits.is_empty() { IBig::ZERO } else { digits.parse().ok()? };
    let scale = exp - fp.len() as i64;
    if scale.unsigned_abs() > 100_000 {
        return None;
    }
    let ten = RBig::from(IBig::from(10));
    let mut r = RBig::from(n) * ten.pow(scale as isize);
    if neg {
        r = -r;
    }
    Some(r)
}

/// Exact printing: an integer — digits, otherwise "n/d".
pub fn show(x: &Q) -> String {
    if x.is_int() { x.numerator().to_string() } else { format!("{}/{}", x.numerator(), x.denominator()) }
}

/// Terminating decimal representation (up to `max` digits after the point), if one exists.
pub fn exact_decimal(x: &Q, max: usize) -> Option<String> {
    let mut d = x.denominator().clone();
    let (mut twos, mut fives) = (0usize, 0usize);
    let two = UBig::from(2u8);
    let five = UBig::from(5u8);
    while (&d % &two) == UBig::ZERO {
        d /= &two;
        twos += 1;
    }
    while (&d % &five) == UBig::ZERO {
        d /= &five;
        fives += 1;
    }
    if d != UBig::ONE {
        return None;
    }
    let places = twos.max(fives);
    if places > max {
        return None;
    }
    if places == 0 {
        return Some(x.numerator().to_string());
    }
    let scaled = x.clone() * RBig::from(IBig::from(10).pow(places));
    let n = scaled.numerator().clone();
    let neg = n < IBig::ZERO;
    let s = abs_z(&n).to_string();
    let s = format!("{s:0>width$}", width = places + 1);
    let (a, b) = s.split_at(s.len() - places);
    let b = b.trim_end_matches('0');
    Some(format!("{}{}{}{}", if neg { "-" } else { "" }, a, if b.is_empty() { "" } else { "." }, b))
}

/// Human-readable printing: integer; terminating decimal; otherwise "n/d (≈ 0.3333)".
pub fn show_nice(x: &Q) -> String {
    if x.is_int() {
        return x.numerator().to_string();
    }
    if let Some(s) = exact_decimal(x, 12) {
        return s;
    }
    format!("{} (≈ {})", show(x), fmt_f64(to_f64(x)))
}

pub fn fmt_f64(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    let a = v.abs();
    if (1e-4..1e15).contains(&a) {
        let s = format!("{v:.10}");
        let s = s.trim_end_matches('0').trim_end_matches('.');
        s.to_string()
    } else {
        format!("{v:.10e}")
    }
}

/// The nearest rational to an f64 with a small denominator (for hints; not for exact answers).
pub fn simplest_near(v: f64) -> Option<Q> {
    RBig::simplest_from_f64(v)
}

/// Integer k-th root, if exact (for n ≥ 0).
pub fn exact_root_z(n: &Z, k: usize) -> Option<Z> {
    if k == 0 {
        return None;
    }
    if *n < IBig::ZERO {
        if k % 2 == 0 {
            return None;
        }
        return exact_root_z(&(-n.clone()), k).map(|r| -r);
    }
    let u = uz(n);
    let r = u.nth_root(k);
    (r.pow(k) == u).then(|| IBig::from(r))
}

/// Exact k-th root of a rational, if it exists.
pub fn exact_root_q(x: &Q, k: usize) -> Option<Q> {
    let n = exact_root_z(x.numerator(), k)?;
    let d = exact_root_z(&IBig::from(x.denominator().clone()), k)?;
    frac(n, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_show() {
        assert_eq!(show(&parse_q("1,234").unwrap()), "1234");
        assert_eq!(show(&parse_q("-4.25").unwrap()), "-17/4");
        assert_eq!(show(&parse_q("3/4").unwrap()), "3/4");
        assert_eq!(show(&parse_q("2.5e-3").unwrap()), "1/400");
        assert_eq!(exact_decimal(&parse_q("-17/4").unwrap(), 10).unwrap(), "-4.25");
        assert_eq!(exact_decimal(&parse_q("1/3").unwrap(), 10), None);
        assert!(parse_q("1,23").is_none());
        assert_eq!(exact_root_q(&parse_q("9/4").unwrap(), 2).unwrap(), parse_q("3/2").unwrap());
        assert_eq!(exact_root_z(&z(-27), 3).unwrap(), z(-3));
        assert!(exact_root_z(&z(2), 2).is_none());
    }

    #[test]
    fn big_exact() {
        // 3^100 and 100! — exact (v1 on i128 gave Overflow)
        let p = IBig::from(3).pow(100);
        assert_eq!(p.to_string(), "515377520732011331036461129765621272702107522001");
        let mut f = IBig::ONE;
        for k in 1..=100 {
            f *= IBig::from(k);
        }
        assert_eq!(f.to_string().len(), 158);
        assert!(f.to_string().starts_with("93326215443944152681"));
    }
}
