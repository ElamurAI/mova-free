//! v2: number theory on big integers. Where possible, every function also returns a certificate, an independent
//! check by another route: GCD via divisibility and Bézout's identity, inverse via a·x ≡ 1, CRT via substitution,
//! factorization via the product and primality of the factors, primality via deterministic Miller–Rabin for u64,
//! a Pocklington certificate or an honest "probably prime" for big numbers.

use std::sync::OnceLock;
use std::time::Instant;

use dashu::integer::fast_div::ConstDivisor;

use crate::big::*;

/// One check: what was compared, whether it matched, details.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Check {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

impl Check {
    pub fn new(name: &str, ok: bool, detail: impl Into<String>) -> Check {
        Check { name: name.to_string(), ok, detail: detail.into() }
    }
}

/// Search and time limits (fail-fast instead of hanging).
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_steps: u64,
    pub deadline: Option<Instant>,
}

impl Default for Limits {
    fn default() -> Self {
        Limits { max_steps: 20_000_000, deadline: None }
    }
}

impl Limits {
    pub fn with_secs(secs: f64) -> Limits {
        Limits { max_steps: 20_000_000, deadline: Some(Instant::now() + std::time::Duration::from_secs_f64(secs)) }
    }
    pub fn timed_out(&self) -> bool {
        self.deadline.is_some_and(|d| Instant::now() > d)
    }
}

// ---------- GCD, LCM, Bézout ----------

pub fn gcd(a: &Z, b: &Z) -> Z {
    zu(a.gcd(b))
}

pub fn lcm(a: &Z, b: &Z) -> Z {
    if a.is_zero() || b.is_zero() {
        return IBig::ZERO;
    }
    abs_z(&(a / gcd(a, b) * b))
}

/// Our own extended Euclid (independent of `dashu::gcd`): (g, s, t), s·a + t·b = g ≥ 0.
pub fn ext_gcd(a: &Z, b: &Z) -> (Z, Z, Z) {
    let (mut r0, mut r1) = (a.clone(), b.clone());
    let (mut s0, mut s1) = (IBig::ONE, IBig::ZERO);
    let (mut t0, mut t1) = (IBig::ZERO, IBig::ONE);
    while !r1.is_zero() {
        let q = &r0 / &r1; // division rounding toward zero; irrelevant for the algorithm
        let r2 = &r0 - &q * &r1;
        r0 = std::mem::replace(&mut r1, r2);
        let s2 = &s0 - &q * &s1;
        s0 = std::mem::replace(&mut s1, s2);
        let t2 = &t0 - &q * &t1;
        t0 = std::mem::replace(&mut t1, t2);
    }
    if r0 < IBig::ZERO {
        (-r0, -s0, -t0)
    } else {
        (r0, s0, t0)
    }
}

/// GCD certificate: g | a, g | b, s·a + t·b = g (then any common divisor divides g).
pub fn check_gcd(a: &Z, b: &Z, g: &Z) -> Check {
    let (g2, s, t) = ext_gcd(a, b);
    let divides = |x: &Z| if g.is_zero() { x.is_zero() } else { (x % g).is_zero() };
    let ok = divides(a) && divides(b) && &s * a + &t * b == *g && g2 == *g;
    Check::new("GCD: divides both and s·a + t·b = g", ok, format!("s = {s}, t = {t}"))
}

pub fn mod_floor(a: &Z, m: &Z) -> Z {
    let r = a % m;
    if r < IBig::ZERO { r + abs_z(m) } else { r }
}

pub fn modinv(a: &Z, m: &Z) -> Option<Z> {
    if m.is_zero() {
        return None;
    }
    let m = abs_z(m);
    let (g, s, _) = ext_gcd(&mod_floor(a, &m), &m);
    (g == IBig::ONE).then(|| mod_floor(&s, &m))
}

/// b^e mod m via the dashu residue ring (Montgomery/constant divisor). e < 0 via the inverse.
pub fn modpow(b: &Z, e: &Z, m: &Z) -> Option<Z> {
    let m = abs_z(m);
    if m.is_zero() {
        return None;
    }
    if m == IBig::ONE {
        return Some(IBig::ZERO);
    }
    let (b, e) = if *e < IBig::ZERO { (modinv(b, &m)?, -e.clone()) } else { (mod_floor(b, &m), e.clone()) };
    let ring = ConstDivisor::new(uz(&m));
    let r = ring.reduce(uz(&b)).pow(&uz(&e)).residue();
    Some(zu(r))
}

/// Independent route for modpow: square-and-multiply with plain `%` (different code than the dashu ring).
pub fn modpow_plain(b: &Z, e: &Z, m: &Z) -> Option<Z> {
    let m = abs_z(m);
    if m.is_zero() {
        return None;
    }
    let (mut base, e) = if *e < IBig::ZERO { (modinv(b, &m)?, -e.clone()) } else { (mod_floor(b, &m), e.clone()) };
    let mut r = mod_floor(&IBig::ONE, &m);
    let bits = uz(&e);
    let n = bits.bit_len();
    for i in 0..n {
        if bits.bit(i) {
            r = mod_floor(&(&r * &base), &m);
        }
        base = mod_floor(&(&base * &base), &m);
    }
    Some(r)
}

/// Chinese remainder theorem, general (moduli not necessarily coprime): (x, M), 0 ≤ x < M.
pub fn crt(rs: &[Z], ms: &[Z]) -> Option<(Z, Z)> {
    if rs.len() != ms.len() || rs.is_empty() {
        return None;
    }
    let mut x = IBig::ZERO;
    let mut m = IBig::ONE;
    for (r, mi) in rs.iter().zip(ms) {
        let mi = abs_z(mi);
        if mi.is_zero() {
            return None;
        }
        let r = mod_floor(r, &mi);
        let g = gcd(&m, &mi);
        let diff = &r - &x;
        if !(&diff % &g).is_zero() {
            return None;
        }
        let m_g = &m / &g;
        let mi_g = &mi / &g;
        let t = if mi_g == IBig::ONE { IBig::ZERO } else { mod_floor(&(&diff / &g * modinv(&m_g, &mi_g)?), &mi_g) };
        x = &x + &m * t;
        m = &m * &mi_g;
        x = mod_floor(&x, &m);
    }
    Some((x, m))
}

pub fn check_crt(rs: &[Z], ms: &[Z], x: &Z) -> Check {
    let ok = rs.iter().zip(ms).all(|(r, m)| mod_floor(x, m) == mod_floor(r, m));
    Check::new("CRT: substitution x mod mᵢ = rᵢ", ok, format!("x = {x}"))
}

// ---------- primality ----------

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, b, m);
        }
        b = mulmod(b, b, m);
        e >>= 1;
    }
    r
}

const MR_BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

/// Deterministic Miller–Rabin for u64 (bases are the first 12 primes; exact up to 3.3·10^24).
pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in MR_BASES {
        if n % p == 0 {
            return n == p;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'outer: for a in MR_BASES {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// Sieve up to 10^6: small primes for trial division.
pub fn small_primes() -> &'static [u64] {
    static P: OnceLock<Vec<u64>> = OnceLock::new();
    P.get_or_init(|| sieve(1_000_000))
}

pub fn sieve(n: u64) -> Vec<u64> {
    let n = n as usize;
    if n < 2 {
        return Vec::new();
    }
    let mut is = vec![true; n + 1];
    is[0] = false;
    is[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is[i] {
            let mut j = i * i;
            while j <= n {
                is[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is.iter().enumerate().filter(|(_, b)| **b).map(|(i, _)| i as u64).collect()
}

/// Miller–Rabin compositeness witness for a big odd n > 3 (None means all bases passed).
fn mr_witness_big(n: &UBig, bases: &[u64]) -> Option<u64> {
    let one = UBig::ONE;
    let nm1 = n - &one;
    let mut d = nm1.clone();
    let mut s = 0usize;
    while !d.bit(0) {
        d >>= 1;
        s += 1;
    }
    let ring = ConstDivisor::new(n.clone());
    let r_one = ring.reduce(UBig::ONE);
    let r_m1 = ring.reduce(nm1.clone());
    'outer: for &a in bases {
        let au = UBig::from(a);
        if au >= *n {
            continue;
        }
        let mut x = ring.reduce(au).pow(&d);
        if x == r_one || x == r_m1 {
            continue;
        }
        for _ in 1..s {
            x = &x * &x;
            if x == r_m1 {
                continue 'outer;
            }
        }
        return Some(a);
    }
    None
}

/// Primality verdict with an explanation.
#[derive(Clone, Debug, PartialEq)]
pub enum Primality {
    /// composite: the certificate is a divisor or a Miller–Rabin witness
    Composite(String),
    /// prime, proven: deterministic MR (u64) or a Pocklington certificate
    Prime(String),
    /// probably prime: passed MR with N bases (error < 4^-N), no proof found within limits
    Probable(String),
}

impl Primality {
    pub fn is_prime_like(&self) -> bool {
        !matches!(self, Primality::Composite(_))
    }
}

const BIG_BASES: [u64; 30] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113];

pub fn primality(n: &Z) -> Primality {
    primality_depth(n, 0)
}

fn primality_depth(n: &Z, depth: u32) -> Primality {
    if *n < IBig::from(2) {
        return Primality::Composite(format!("{n} < 2: not prime by definition"));
    }
    if let Some(v) = z_to_u64(n) {
        return if is_prime_u64(v) {
            Primality::Prime("deterministic Miller–Rabin for u64 (12 bases)".into())
        } else {
            match small_factor_u64(v) {
                Some(p) => Primality::Composite(format!("divisor {p}")),
                None => Primality::Composite("Miller–Rabin witness".into()),
            }
        };
    }
    for &p in small_primes().iter().take(2000) {
        let pz = IBig::from(p);
        if (n % &pz).is_zero() {
            return Primality::Composite(format!("divisor {p}"));
        }
    }
    let nu = uz(n);
    if mr_witness_big(&nu, &BIG_BASES).is_some() {
        return Primality::Composite("Miller–Rabin witness (base among the first 30 primes)".into());
    }
    if depth < 3 {
        if let Some(cert) = pocklington(n, depth) {
            return Primality::Prime(cert);
        }
    }
    Primality::Probable("passed Miller–Rabin with 30 bases (error < 4^-30); no Pocklington certificate found within limits".into())
}

fn small_factor_u64(v: u64) -> Option<u64> {
    for &p in small_primes() {
        if p * p > v {
            break;
        }
        if v % p == 0 {
            return Some(p);
        }
    }
    None
}

/// Pocklington certificate: n − 1 = F·R, F² > n, F fully factored into proven primes q, and for each
/// q there is a: a^(n−1) ≡ 1 (mod n), gcd(a^((n−1)/q) − 1, n) = 1 ⇒ n is prime.
fn pocklington(n: &Z, depth: u32) -> Option<String> {
    let nm1 = n - IBig::ONE;
    let fz = factor_limited(&nm1, 200_000, Limits::with_secs(2.0));
    let mut f = IBig::ONE;
    let mut qs = Vec::new();
    for (p, e) in &fz.factors {
        if !matches!(primality_depth(p, depth + 1), Primality::Prime(_)) {
            continue;
        }
        f *= p.pow(*e as usize);
        qs.push(p.clone());
    }
    if &f * &f <= *n {
        return None;
    }
    for q in &qs {
        let e = &nm1 / q;
        let mut found = false;
        for a in 2..200i64 {
            let a = IBig::from(a);
            if modpow(&a, &nm1, n)? != IBig::ONE {
                return None; // Fermat fails: composite (should not get here after MR)
            }
            let t = modpow(&a, &e, n)? - IBig::ONE;
            if gcd(&t, n) == IBig::ONE {
                found = true;
                break;
            }
        }
        if !found {
            return None;
        }
    }
    Some(format!("Pocklington certificate: n−1 = F·R, F = {} (primes {}), F² > n", f, qs.iter().map(|q| q.to_string()).collect::<Vec<_>>().join("·")))
}

// ---------- factorization ----------

#[derive(Clone, Debug)]
pub struct Factored {
    /// (prime or unfactored composite, exponent), ascending
    pub factors: Vec<(Z, u32)>,
    /// whether all factors are prime (proven or probable)
    pub complete: bool,
    pub note: String,
}

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Pollard–Brent for u64.
fn pollard_u64(n: u64, budget: &mut u64) -> Option<u64> {
    if n % 2 == 0 {
        return Some(2);
    }
    for c in 1..64u64 {
        let f = |x: u64| ((mulmod(x, x, n) as u128 + c as u128) % n as u128) as u64;
        let (mut y, mut r, mut q, mut g) = (2u64, 1u64, 1u64, 1u64);
        let m = 128u64;
        let mut x = 0u64;
        let mut ys = 0u64;
        while g == 1 {
            x = y;
            for _ in 0..r {
                y = f(y);
            }
            let mut k = 0;
            while k < r && g == 1 {
                ys = y;
                for _ in 0..m.min(r - k) {
                    y = f(y);
                    q = mulmod(q, x.abs_diff(y), n);
                }
                g = gcd_u64(q, n);
                k += m;
                *budget = budget.saturating_sub(m);
                if *budget == 0 {
                    return None;
                }
            }
            r *= 2;
        }
        if g == n {
            loop {
                ys = f(ys);
                g = gcd_u64(x.abs_diff(ys), n);
                if g > 1 {
                    break;
                }
            }
        }
        if g != n && g != 1 {
            return Some(g);
        }
    }
    None
}

/// Pollard–Brent for big numbers (dashu residue ring).
fn pollard_big(n: &Z, budget: &mut u64, lim: &Limits) -> Option<Z> {
    let nu = uz(n);
    let ring = ConstDivisor::new(nu.clone());
    for c in 1..16u64 {
        let rc = ring.reduce(UBig::from(c));
        fn step<'a>(x: &dashu::integer::modular::Reduced<'a>, c: &dashu::integer::modular::Reduced<'a>) -> dashu::integer::modular::Reduced<'a> {
            x * x + c
        }
        let mut y = ring.reduce(UBig::from(2u8));
        let mut r = 1u64;
        let mut q = ring.reduce(UBig::ONE);
        let mut g = UBig::ONE;
        let m = 64u64;
        let mut x = y.clone();
        let mut ys = y.clone();
        while g == UBig::ONE {
            x = y.clone();
            for _ in 0..r {
                y = step(&y, &rc);
            }
            let mut k = 0;
            while k < r && g == UBig::ONE {
                ys = y.clone();
                for _ in 0..m.min(r - k) {
                    y = step(&y, &rc);
                    q = &q * (&x - &y);
                }
                g = zu(q.residue()).gcd(&IBig::from(nu.clone()));
                k += m;
                *budget = budget.saturating_sub(m);
                if *budget == 0 || lim.timed_out() {
                    return None;
                }
            }
            r *= 2;
        }
        if g == nu {
            loop {
                ys = step(&ys, &rc);
                g = zu((&x - &ys).residue()).gcd(&IBig::from(nu.clone()));
                if g != UBig::ONE {
                    break;
                }
            }
        }
        if g != nu && g != UBig::ONE {
            return Some(zu(g));
        }
    }
    None
}

pub fn factor(n: &Z) -> Factored {
    factor_limited(n, 50_000_000, Limits::with_secs(20.0))
}

/// Factorization: trial division up to 10^6, then Pollard–Brent with a step and time budget.
pub fn factor_limited(n: &Z, mut budget: u64, lim: Limits) -> Factored {
    let mut out: Vec<(Z, u32)> = Vec::new();
    let mut m = abs_z(n);
    let mut note = String::new();
    if m <= IBig::ONE {
        return Factored { factors: Vec::new(), complete: true, note: format!("{n}: one or zero, no prime factors") };
    }
    for &p in small_primes() {
        let pz = IBig::from(p);
        if &pz * &pz > m {
            break;
        }
        let mut e = 0u32;
        while (&m % &pz).is_zero() {
            m /= &pz;
            e += 1;
        }
        if e > 0 {
            out.push((pz, e));
        }
    }
    let mut complete = true;
    let mut stack = if m > IBig::ONE { vec![m] } else { Vec::new() };
    while let Some(x) = stack.pop() {
        let pr = primality(&x);
        if pr.is_prime_like() {
            if matches!(pr, Primality::Probable(_)) {
                note.push_str(&format!("{x}: probably prime; "));
            }
            out.push((x, 1));
            continue;
        }
        let d = match z_to_u64(&x) {
            Some(v) => pollard_u64(v, &mut budget).map(|d| IBig::from(d)),
            None => pollard_big(&x, &mut budget, &lim),
        };
        match d {
            Some(d) => {
                let e = &x / &d;
                stack.push(d);
                stack.push(e);
            }
            None => {
                complete = false;
                note.push_str(&format!("{x}: composite, not factored within budget; "));
                out.push((x, 1));
            }
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<(Z, u32)> = Vec::new();
    for (p, e) in out {
        match merged.last_mut() {
            Some((q, f)) if *q == p => *f += e,
            _ => merged.push((p, e)),
        }
    }
    Factored { factors: merged, complete, note }
}

/// Factorization certificate: product = |n|, each factor prime (independent primality check).
pub fn check_factor(n: &Z, f: &Factored) -> Check {
    let prod = f.factors.iter().fold(IBig::ONE, |acc, (p, e)| acc * p.pow(*e as usize));
    let all_prime = f.factors.iter().all(|(p, _)| primality(p).is_prime_like());
    let ok = prod == abs_z(n) || (n.is_zero() && f.factors.is_empty()) || (abs_z(n) == IBig::ONE && f.factors.is_empty());
    Check::new("factorization: product of factors = n, each factor prime", ok && all_prime && f.complete, format!("product = {prod}{}", if f.note.is_empty() { String::new() } else { format!("; {}", f.note) }))
}

pub fn show_factored(f: &Factored) -> String {
    if f.factors.is_empty() {
        return "1".into();
    }
    f.factors.iter().map(|(p, e)| if *e == 1 { p.to_string() } else { format!("{p}^{e}") }).collect::<Vec<_>>().join(" · ")
}

// ---------- multiplicative functions ----------

fn complete_factors(n: &Z) -> Result<Vec<(Z, u32)>, String> {
    if *n <= IBig::ZERO {
        return Err(format!("a positive integer is required, not {n}"));
    }
    let f = factor(n);
    if !f.complete {
        return Err(format!("failed to factor {n}: {}", f.note));
    }
    Ok(f.factors)
}

pub fn phi(n: &Z) -> Result<Z, String> {
    let fs = complete_factors(n)?;
    Ok(fs.iter().fold(IBig::ONE, |acc, (p, e)| acc * p.pow(*e as usize - 1) * (p - IBig::ONE)))
}

pub fn tau(n: &Z) -> Result<Z, String> {
    let fs = complete_factors(n)?;
    Ok(fs.iter().fold(IBig::ONE, |acc, (_, e)| acc * IBig::from(*e + 1)))
}

pub fn sigma(n: &Z, k: u32) -> Result<Z, String> {
    let fs = complete_factors(n)?;
    Ok(fs.iter().fold(IBig::ONE, |acc, (p, e)| {
        let pk = p.pow(k as usize);
        let mut s = IBig::ZERO;
        let mut t = IBig::ONE;
        for _ in 0..=*e {
            s += &t;
            t *= &pk;
        }
        acc * s
    }))
}

pub fn mobius(n: &Z) -> Result<Z, String> {
    let fs = complete_factors(n)?;
    if fs.iter().any(|(_, e)| *e > 1) {
        return Ok(IBig::ZERO);
    }
    Ok(if fs.len() % 2 == 0 { IBig::ONE } else { -IBig::ONE })
}

pub fn divisors(n: &Z, max: usize) -> Result<Vec<Z>, String> {
    let fs = complete_factors(&abs_z(n))?;
    let count: u128 = fs.iter().map(|(_, e)| *e as u128 + 1).product();
    if count > max as u128 {
        return Err(format!("{count} divisors exceed the limit {max}"));
    }
    let mut ds = vec![IBig::ONE];
    for (p, e) in fs {
        let mut next = Vec::with_capacity(ds.len() * (e as usize + 1));
        for d in &ds {
            let mut t = d.clone();
            for _ in 0..=e {
                next.push(t.clone());
                t *= &p;
            }
        }
        ds = next;
    }
    ds.sort();
    Ok(ds)
}

pub fn next_prime(n: &Z) -> Z {
    let mut x = if *n < IBig::from(2) { IBig::from(2) } else { n + IBig::ONE };
    while !primality(&x).is_prime_like() {
        x += IBig::ONE;
    }
    x
}

pub fn prev_prime(n: &Z) -> Option<Z> {
    let mut x = n - IBig::ONE;
    while x >= IBig::from(2) {
        if primality(&x).is_prime_like() {
            return Some(x);
        }
        x -= IBig::ONE;
    }
    None
}

/// π(n) by sieve up to 2·10^8.
pub fn prime_pi(n: &Z) -> Result<Z, String> {
    let v = z_to_u64(n).filter(|v| *v <= 200_000_000).ok_or("π(n): n beyond the limit 2·10^8")?;
    Ok(IBig::from(count_primes_sieve(v)))
}

fn count_primes_sieve(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }
    // sieve of odd numbers only, as bits in u64
    let n = n as usize;
    let half = n.div_ceil(2); // indices of odd numbers 1,3,5,… (0 ↔ 1)
    let mut bits = vec![0u64; half.div_ceil(64)];
    let mut i = 1usize; // the number 2i+1
    while (2 * i + 1) * (2 * i + 1) <= n {
        if bits[i / 64] >> (i % 64) & 1 == 0 {
            let p = 2 * i + 1;
            let mut j = (p * p) / 2;
            while j < half {
                bits[j / 64] |= 1 << (j % 64);
                j += p;
            }
        }
        i += 1;
    }
    let mut c = 1u64; // the two
    for k in 1..half {
        if bits[k / 64] >> (k % 64) & 1 == 0 && 2 * k + 1 <= n {
            c += 1;
        }
    }
    c
}

/// k-th prime (p₁ = 2), by sieve.
pub fn nth_prime(k: &Z) -> Result<Z, String> {
    let k = z_to_u64(k).filter(|k| (1..=10_000_000).contains(k)).ok_or("nthprime: k outside 1..10^7")?;
    let kf = k as f64;
    let bound = if k < 6 { 15 } else { (kf * (kf.ln() + kf.ln().ln())) as u64 + 10 };
    let ps = sieve(bound);
    Ok(IBig::from(ps[(k - 1) as usize]))
}

// ---------- digits ----------

pub fn digits(n: &Z, base: u32) -> Result<Vec<u32>, String> {
    if base < 2 {
        return Err("base < 2".into());
    }
    let mut m = abs_z(n);
    if base == 10 {
        return Ok(m.to_string().bytes().map(|b| (b - b'0') as u32).collect());
    }
    if m.is_zero() {
        return Ok(vec![0]);
    }
    let b = IBig::from(base);
    let mut out = Vec::new();
    while !m.is_zero() {
        let r = &m % &b;
        out.push(u32::try_from(&r).unwrap_or(0));
        m /= &b;
    }
    out.reverse();
    Ok(out)
}

pub fn from_digits(ds: &[Z], base: &Z) -> Z {
    ds.iter().fold(IBig::ZERO, |acc, d| acc * base + d)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zs(s: &str) -> Z {
        s.parse().unwrap()
    }

    #[test]
    fn gcd_lcm_bezout() {
        let a = zs("123456789012345678901234567890");
        let b = zs("987654321098765432109876543210");
        let g = gcd(&a, &b);
        assert!(check_gcd(&a, &b, &g).ok);
        assert!(!check_gcd(&a, &b, &(g.clone() * IBig::from(2))).ok, "negative control: false GCD");
        assert_eq!(lcm(&z(4), &z(6)), z(12));
    }

    #[test]
    fn modular() {
        assert_eq!(modpow(&z(3), &z(200), &z(1_000_000_007)), modpow_plain(&z(3), &z(200), &z(1_000_000_007)));
        assert_eq!(modpow(&z(2), &z(10), &z(1000)).unwrap(), z(24));
        assert_eq!(modinv(&z(3), &z(11)).unwrap(), z(4));
        assert!(modinv(&z(4), &z(8)).is_none());
        let (x, m) = crt(&[z(2), z(3), z(2)], &[z(3), z(5), z(7)]).unwrap();
        assert_eq!((x.clone(), m), (z(23), z(105)));
        assert!(check_crt(&[z(2), z(3), z(2)], &[z(3), z(5), z(7)], &x).ok);
        assert!(!check_crt(&[z(2), z(3), z(2)], &[z(3), z(5), z(7)], &z(24)).ok);
        // non-coprime moduli
        assert_eq!(crt(&[z(1), z(3)], &[z(4), z(6)]).unwrap(), (z(9), z(12)));
        assert!(crt(&[z(1), z(2)], &[z(4), z(6)]).is_none());
    }

    #[test]
    fn primes() {
        assert!(is_prime_u64(1_000_000_007));
        assert!(!is_prime_u64(3_215_031_751)); // strong pseudoprime to bases 2,3,5,7
        assert!(is_prime_u64(18_446_744_073_709_551_557));
        let m127 = IBig::from(2).pow(127) - IBig::ONE;
        assert!(matches!(primality(&m127), Primality::Prime(_)), "{:?}", primality(&m127));
        let c = &m127 * IBig::from(1_000_000_007);
        assert!(matches!(primality(&c), Primality::Composite(_)));
        assert_eq!(prime_pi(&z(1_000_000)).unwrap(), z(78498));
        assert_eq!(nth_prime(&z(10_000)).unwrap(), z(104_729));
        assert_eq!(next_prime(&z(100)), z(101));
    }

    #[test]
    fn factoring() {
        let n = zs("600851475143");
        let f = factor(&n);
        assert_eq!(show_factored(&f), "71 · 839 · 1471 · 6857");
        assert!(check_factor(&n, &f).ok);
        // two primes ~10^9: Pollard
        let n = IBig::from(1_000_000_007u64) * IBig::from(998_244_353u64);
        let f = factor(&n);
        assert_eq!(f.factors.len(), 2);
        assert!(check_factor(&n, &f).ok);
        // big: 2^64+1 = 274177 · 67280421310721
        let n = IBig::from(2).pow(64) + IBig::ONE;
        assert_eq!(show_factored(&factor(&n)), "274177 · 67280421310721");
        assert_eq!(phi(&z(36)).unwrap(), z(12));
        assert_eq!(tau(&z(36)).unwrap(), z(9));
        assert_eq!(sigma(&z(12), 1).unwrap(), z(28));
        assert_eq!(divisors(&z(12), 100).unwrap(), vec![z(1), z(2), z(3), z(4), z(6), z(12)]);
        assert_eq!(mobius(&z(30)).unwrap(), z(-1));
    }
}
