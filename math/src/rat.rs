//! Exact rational numbers on i128: numerator and denominator always reduced, denominator positive.
//! Every operation is overflow-checked: overflow is an explicit `Error::Overflow` error,
//! not a silent fallback to f64. Comparison never overflows (continued fraction).
//!
//! The representation is hidden behind the API: when big integers are needed (combinatorics, level 5 of the
//! roadmap), the type is replaced here and the rest of the crate does not change.

use std::cmp::Ordering;
use std::fmt;

use crate::err::{Error, R};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Rat {
    n: i128,
    d: i128,
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    // only for non-negatives (the calls below pass abs; i128::MIN is filtered out earlier)
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn ck(x: Option<i128>, op: &'static str) -> R<i128> {
    x.ok_or(Error::Overflow(op))
}

impl Rat {
    pub const ZERO: Rat = Rat { n: 0, d: 1 };
    pub const ONE: Rat = Rat { n: 1, d: 1 };

    pub const fn int(n: i128) -> Rat {
        Rat { n, d: 1 }
    }

    /// n/d with reduction; d = 0 is division by zero.
    pub fn new(n: i128, d: i128) -> R<Rat> {
        if d == 0 {
            return Err(Error::DivZero);
        }
        if n == i128::MIN || d == i128::MIN {
            return Err(Error::Overflow("fraction"));
        }
        let g = gcd(n.abs(), d.abs()).max(1);
        let (mut n, mut d) = (n / g, d / g);
        if d < 0 {
            n = -n;
            d = -d;
        }
        Ok(Rat { n, d })
    }

    pub fn num(self) -> i128 {
        self.n
    }
    pub fn den(self) -> i128 {
        self.d
    }
    pub fn is_int(self) -> bool {
        self.d == 1
    }
    pub fn is_zero(self) -> bool {
        self.n == 0
    }
    pub fn is_neg(self) -> bool {
        self.n < 0
    }
    pub fn abs(self) -> Rat {
        Rat { n: self.n.abs(), d: self.d }
    }

    pub fn neg(self) -> R<Rat> {
        Ok(Rat { n: ck(self.n.checked_neg(), "negation")?, d: self.d })
    }

    pub fn add(self, o: Rat) -> R<Rat> {
        let g = gcd(self.d, o.d);
        let (s, t) = (self.d / g, o.d / g);
        let n = ck(ck(self.n.checked_mul(t), "addition")?.checked_add(ck(o.n.checked_mul(s), "addition")?), "addition")?;
        let d = ck(s.checked_mul(o.d), "addition")?;
        Rat::new(n, d)
    }

    pub fn sub(self, o: Rat) -> R<Rat> {
        self.add(o.neg()?)
    }

    pub fn mul(self, o: Rat) -> R<Rat> {
        let g1 = gcd(self.n.abs(), o.d).max(1);
        let g2 = gcd(o.n.abs(), self.d).max(1);
        let n = ck((self.n / g1).checked_mul(o.n / g2), "multiplication")?;
        let d = ck((self.d / g2).checked_mul(o.d / g1), "multiplication")?;
        Rat::new(n, d)
    }

    pub fn recip(self) -> R<Rat> {
        Rat::new(self.d, self.n)
    }

    pub fn div(self, o: Rat) -> R<Rat> {
        if o.n == 0 {
            return Err(Error::DivZero);
        }
        self.mul(o.recip()?)
    }

    /// Integer power (negative via the reciprocal), exponentiation by squaring with checks.
    pub fn pow(self, e: i64) -> R<Rat> {
        if e < 0 {
            if self.n == 0 {
                return Err(Error::DivZero);
            }
            return self.recip()?.pow(e.checked_neg().ok_or(Error::Overflow("power"))?);
        }
        let (mut base, mut e, mut acc) = (self, e as u64, Rat::ONE);
        while e > 0 {
            if e & 1 == 1 {
                acc = acc.mul(base).map_err(|_| Error::Overflow("power"))?;
            }
            e >>= 1;
            if e > 0 {
                base = base.mul(base).map_err(|_| Error::Overflow("power"))?;
            }
        }
        Ok(acc)
    }

    /// Exact k-th root if numerator and denominator are exact powers; otherwise None.
    pub fn root(self, k: u32) -> Option<Rat> {
        if k == 0 || (self.n < 0 && k % 2 == 0) {
            return None;
        }
        let rn = iroot(self.n.unsigned_abs(), k)?;
        let rd = iroot(self.d as u128, k)?;
        let n = if self.n < 0 { -(rn as i128) } else { rn as i128 };
        Rat::new(n, rd as i128).ok()
    }

    pub fn to_f64(self) -> f64 {
        self.n as f64 / self.d as f64
    }

    /// Decimal string notation: "-12", "16.09344", "1,000" (comma is a thousands separator); "1e3" is not accepted.
    pub fn parse_decimal(s: &str) -> Option<Rat> {
        let (neg, body) = match s.strip_prefix('-').or_else(|| s.strip_prefix('−')) {
            Some(b) => (true, b),
            None => (false, s.strip_prefix('+').unwrap_or(s)),
        };
        let (ip, fp) = match body.split_once('.') {
            Some((i, f)) => (i, f),
            None => (body, ""),
        };
        if ip.is_empty() && fp.is_empty() {
            return None;
        }
        // comma only as a thousands separator: groups of three digits
        let ip_digits: String = if ip.contains(',') {
            let parts: Vec<&str> = ip.split(',').collect();
            if parts[0].is_empty() || parts[0].len() > 3 || parts[1..].iter().any(|p| p.len() != 3) {
                return None;
            }
            parts.concat()
        } else {
            ip.to_string()
        };
        if !ip_digits.bytes().all(|b| b.is_ascii_digit()) || !fp.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let digits = format!("{ip_digits}{fp}");
        if digits.is_empty() || digits.len() > 39 {
            return None;
        }
        let n: i128 = digits.parse().ok()?;
        let d = 10i128.checked_pow(fp.len() as u32)?;
        let r = Rat::new(if neg { -n } else { n }, d).ok()?;
        Some(r)
    }

    /// Fraction "a/b" where a, b are decimals; or just a decimal.
    pub fn parse(s: &str) -> Option<Rat> {
        match s.split_once('/') {
            Some((a, b)) => Rat::parse_decimal(a.trim())?.div(Rat::parse_decimal(b.trim())?).ok(),
            None => Rat::parse_decimal(s.trim()),
        }
    }

    /// Finite decimal notation if the denominator is 2^a·5^b and there are at most `max` digits after the point.
    pub fn exact_decimal(self, max: u32) -> Option<String> {
        let (mut d, mut k2, mut k5) = (self.d, 0u32, 0u32);
        while d % 2 == 0 {
            d /= 2;
            k2 += 1;
        }
        while d % 5 == 0 {
            d /= 5;
            k5 += 1;
        }
        if d != 1 {
            return None;
        }
        let k = k2.max(k5);
        if k > max {
            return None;
        }
        let scale = 10i128.checked_pow(k)?;
        let m = self.n.checked_mul(scale / self.d)?;
        Some(fmt_scaled(m < 0, m.unsigned_abs(), k))
    }

    /// Rounding to `places` decimal places (half away from zero), for output only.
    pub fn round_decimal(self, places: u32) -> String {
        let scale = 10i128.pow(places);
        // m = round(n·scale/d) without overflow: integer part separately
        let q = self.n / self.d;
        let r = self.n % self.d; // same sign as n
        // overflow here is possible only for gigantic denominators; then print via f64
        let frac = (r.unsigned_abs())
            .checked_mul(scale as u128 * 2)
            .and_then(|x| x.checked_add(self.d as u128))
            .map(|x| x / (2 * self.d as u128));
        let Some(frac) = frac else { return trim_zeros(&format!("{:.*}", places as usize, self.to_f64())) };
        let Some(m) = q.unsigned_abs().checked_mul(scale as u128).and_then(|x| x.checked_add(frac)) else {
            return trim_zeros(&format!("{:.*}", places as usize, self.to_f64()));
        };
        trim_zeros(&fmt_scaled(self.n < 0 && m != 0, m, places))
    }
}

fn fmt_scaled(neg: bool, a: u128, k: u32) -> String {
    let s = if k == 0 {
        a.to_string()
    } else {
        let p = 10u128.pow(k);
        format!("{}.{:0width$}", a / p, a % p, width = k as usize)
    };
    if neg { format!("-{s}") } else { s }
}

fn trim_zeros(s: &str) -> String {
    if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s.to_string() }
}

/// Integer k-th root, if it is exact.
fn iroot(x: u128, k: u32) -> Option<u128> {
    if x < 2 {
        return Some(x);
    }
    let mut r = (x as f64).powf(1.0 / k as f64).round() as u128;
    for c in [r.saturating_sub(1), r, r + 1] {
        if c.checked_pow(k) == Some(x) {
            r = c;
            return Some(r);
        }
    }
    None
}

impl Ord for Rat {
    /// a/b vs c/d without cross-multiplication: integer parts, then reciprocal remainders (continued fraction).
    fn cmp(&self, o: &Rat) -> Ordering {
        let (mut a, mut b, mut c, mut d) = (self.n, self.d, o.n, o.d);
        let mut flip = false;
        loop {
            let (q1, r1) = (a.div_euclid(b), a.rem_euclid(b));
            let (q2, r2) = (c.div_euclid(d), c.rem_euclid(d));
            if q1 != q2 {
                let ord = q1.cmp(&q2);
                return if flip { ord.reverse() } else { ord };
            }
            match (r1 == 0, r2 == 0) {
                (true, true) => return Ordering::Equal,
                (true, false) => return if flip { Ordering::Greater } else { Ordering::Less },
                (false, true) => return if flip { Ordering::Less } else { Ordering::Greater },
                _ => {}
            }
            // r1/b vs r2/d  ⇔  d/r2 vs b/r1 (reversed)
            (a, b, c, d) = (b, r1, d, r2);
            flip = !flip;
        }
    }
}

impl PartialOrd for Rat {
    fn partial_cmp(&self, o: &Rat) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl fmt::Display for Rat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.d == 1 { write!(f, "{}", self.n) } else { write!(f, "{}/{}", self.n, self.d) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(n: i128, d: i128) -> Rat {
        Rat::new(n, d).unwrap()
    }

    #[test]
    fn reduces_and_normalizes_sign() {
        assert_eq!(r(6, 8), r(3, 4));
        assert_eq!(r(3, -4), r(-3, 4));
        assert_eq!((r(-6, -8).num(), r(-6, -8).den()), (3, 4));
        assert_eq!(Rat::new(1, 0), Err(Error::DivZero));
    }

    #[test]
    fn exact_arithmetic_no_float_error() {
        let a = Rat::parse("0.1").unwrap().add(Rat::parse("0.2").unwrap()).unwrap();
        assert_eq!(a, Rat::parse("0.3").unwrap()); // in f64 0.1+0.2 ≠ 0.3
        assert_ne!(0.1f64 + 0.2, 0.3);
        assert_eq!(r(1, 3).add(r(1, 6)).unwrap(), r(1, 2));
        assert_eq!(r(7, 3).mul(r(3, 7)).unwrap(), Rat::ONE);
        assert_eq!(r(2, 3).div(r(4, 9)).unwrap(), r(3, 2));
        assert_eq!(r(2, 3).pow(-2).unwrap(), r(9, 4));
    }

    /// Associativity and commutativity on a sample of fractions: exact, no tolerance.
    #[test]
    fn associativity_and_commutativity() {
        let xs = [r(1, 3), r(-5, 7), r(22, 9), r(3, 1), r(-1, 12), r(1000, 999)];
        for &a in &xs {
            for &b in &xs {
                assert_eq!(a.add(b).unwrap(), b.add(a).unwrap());
                assert_eq!(a.mul(b).unwrap(), b.mul(a).unwrap());
                for &c in &xs {
                    assert_eq!(a.add(b).unwrap().add(c).unwrap(), a.add(b.add(c).unwrap()).unwrap());
                    assert_eq!(a.mul(b).unwrap().mul(c).unwrap(), a.mul(b.mul(c).unwrap()).unwrap());
                    assert_eq!(a.mul(b.add(c).unwrap()).unwrap(), a.mul(b).unwrap().add(a.mul(c).unwrap()).unwrap());
                }
            }
        }
    }

    /// Negative control: overflow is an error, not a silent number.
    #[test]
    fn overflow_is_an_error() {
        let big = Rat::int(10).pow(30).unwrap();
        assert_eq!(big.mul(big), Err(Error::Overflow("multiplication")));
        assert!(Rat::int(2).pow(126).is_ok());
        assert_eq!(Rat::int(2).pow(127), Err(Error::Overflow("power")));
        assert!(Rat::int(i128::MAX).add(Rat::ONE).is_err());
        assert!(Rat::new(i128::MIN, 1).is_err());
    }

    #[test]
    fn compare_without_overflow() {
        let a = r(i128::MAX - 1, i128::MAX);
        let b = r(i128::MAX - 2, i128::MAX - 1);
        assert!(a > b); // cross-multiplying is impossible: it would overflow
        assert!(r(-1, 2) < r(1, 3));
        assert!(r(-1, 2) < r(-1, 3));
        assert_eq!(r(4, 6).cmp(&r(2, 3)), Ordering::Equal);
        let mut v = vec![r(10, 1), r(3, 1), r(5, 1), r(-7, 2)];
        v.sort();
        assert_eq!(v, vec![r(-7, 2), r(3, 1), r(5, 1), r(10, 1)]);
    }

    #[test]
    fn decimal_io() {
        assert_eq!(Rat::parse_decimal("1,000"), Some(Rat::int(1000)));
        assert_eq!(Rat::parse_decimal("1,00"), None);
        assert_eq!(Rat::parse("609.344/1609.344"), Some(r(9521, 25146)));
        assert_eq!(r(200, 9).exact_decimal(6), None);
        assert_eq!(r(200, 9).round_decimal(2), "22.22");
        assert_eq!(r(-110, 9).round_decimal(2), "-12.22");
        assert_eq!(r(1609344, 100000).exact_decimal(6).unwrap(), "16.09344");
        assert_eq!(r(1, 3).root(2), None);
        assert_eq!(r(9, 4).root(2), Some(r(3, 2)));
        assert_eq!(Rat::int(144).root(2), Some(Rat::int(12)));
    }
}
