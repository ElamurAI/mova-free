//! v2: combinatorics on big integers. Every function has a second, independent path for checking:
//! binomial — multiplicative formula vs Legendre's prime factorization; Stirling II — table
//! vs explicit inclusion–exclusion formula; partitions — Euler's pentagonal recurrence vs
//! the "coin change" dynamic programme; Catalan — via C(2n,n)/(n+1) vs C(2n,n) − C(2n,n+1).

use crate::big::*;
use crate::nt::{Check, sieve};

pub const MAX_N: u64 = 1_000_000;

fn prod_range(lo: u64, hi: u64) -> Z {
    if lo > hi {
        return IBig::ONE;
    }
    if hi - lo < 16 {
        let mut p = IBig::ONE;
        for k in lo..=hi {
            p *= IBig::from(k);
        }
        return p;
    }
    let m = lo + (hi - lo) / 2;
    prod_range(lo, m) * prod_range(m + 1, hi)
}

pub fn factorial(n: u64) -> Result<Z, String> {
    if n > MAX_N {
        return Err(format!("{n}! — beyond the limit {MAX_N}"));
    }
    Ok(prod_range(1, n))
}

/// Independent path for n!: Legendre's formula — n! = ∏ p^(Σ ⌊n/pⁱ⌋).
pub fn factorial_legendre(n: u64) -> Z {
    let mut r = IBig::ONE;
    for p in sieve(n) {
        let mut e = 0u64;
        let mut q = p;
        while q <= n {
            e += n / q;
            match q.checked_mul(p) {
                Some(x) => q = x,
                None => break,
            }
        }
        r *= IBig::from(p).pow(e as usize);
    }
    r
}

/// C(n, k) for integer n (negative n — generalization (−1)^k C(k−n−1, k)).
pub fn binomial(n: &Z, k: &Z) -> Result<Z, String> {
    if *k < IBig::ZERO {
        return Ok(IBig::ZERO);
    }
    let k64 = z_to_u64(k).ok_or("C(n,k): k too large")?;
    if *n < IBig::ZERO {
        let m = k - n - IBig::ONE;
        let v = binomial(&m, k)?;
        return Ok(if k64 % 2 == 0 { v } else { -v });
    }
    if k > n {
        return Ok(IBig::ZERO);
    }
    let nk = n - k;
    let k64 = if nk < *k { z_to_u64(&nk).ok_or("C(n,k): too large")? } else { k64 };
    if k64 > MAX_N {
        return Err(format!("C(n,k): min(k, n−k) = {k64} — beyond the limit"));
    }
    // ∏_{i=1..k} (n−k+i) / k! — numerator via a product tree
    let base = n - IBig::from(k64);
    let num = prod_offset(&base, 1, k64);
    Ok(num / prod_range(1, k64))
}

fn prod_offset(base: &Z, lo: u64, hi: u64) -> Z {
    if lo > hi {
        return IBig::ONE;
    }
    if hi - lo < 16 {
        let mut p = IBig::ONE;
        for i in lo..=hi {
            p *= base + IBig::from(i);
        }
        return p;
    }
    let m = lo + (hi - lo) / 2;
    prod_offset(base, lo, m) * prod_offset(base, m + 1, hi)
}

/// Independent path for C(n,k), 0 ≤ k ≤ n ≤ 10^7: v_p(C(n,k)) = Σᵢ (⌊n/pⁱ⌋ − ⌊k/pⁱ⌋ − ⌊(n−k)/pⁱ⌋).
pub fn binomial_legendre(n: u64, k: u64) -> Option<Z> {
    if k > n || n > 10_000_000 {
        return None;
    }
    let mut r = IBig::ONE;
    for p in sieve(n) {
        let mut e = 0u64;
        let mut q = p;
        while q <= n {
            e += n / q - k / q - (n - k) / q;
            match q.checked_mul(p) {
                Some(x) => q = x,
                None => break,
            }
        }
        if e > 0 {
            r *= IBig::from(p).pow(e as usize);
        }
    }
    Some(r)
}

pub fn check_binomial(n: &Z, k: &Z, v: &Z) -> Check {
    match (z_to_u64(n), z_to_u64(k)) {
        (Some(n), Some(k)) if n <= 10_000_000 => match binomial_legendre(n, k) {
            Some(w) => Check::new("C(n,k): Legendre prime factorization", w == *v, format!("second path gave {}", short(&w))),
            None => Check::new("C(n,k): k > n ⇒ 0", v.is_zero(), ""),
        },
        _ => Check::new("C(n,k): second path unavailable (n outside 0..10^7)", true, "multiplicative formula only"),
    }
}

pub fn perm(n: &Z, k: &Z) -> Result<Z, String> {
    if *k < IBig::ZERO || k > n || *n < IBig::ZERO {
        return Ok(IBig::ZERO);
    }
    let k64 = z_to_u64(k).filter(|k| *k <= MAX_N).ok_or("P(n,k): k beyond the limit")?;
    Ok(prod_offset(&(n - IBig::from(k64)), 1, k64))
}

pub fn catalan(n: u64) -> Result<Z, String> {
    let c = binomial(&IBig::from(2 * n), &IBig::from(n))?;
    Ok(c / IBig::from(n + 1))
}

pub fn check_catalan(n: u64, v: &Z) -> Check {
    let a = binomial(&IBig::from(2 * n), &IBig::from(n)).unwrap_or_default();
    let b = binomial(&IBig::from(2 * n), &IBig::from(n + 1)).unwrap_or_default();
    Check::new("Catalan: C(2n,n) − C(2n,n+1)", a - b == *v, "")
}

/// Stirling II S(n,k): table S(i,j) = j·S(i−1,j) + S(i−1,j−1).
pub fn stirling2(n: u64, k: u64) -> Result<Z, String> {
    if n > 3000 {
        return Err("S(n,k): n > 3000".into());
    }
    if k > n {
        return Ok(IBig::ZERO);
    }
    let k = k as usize;
    let mut row = vec![IBig::ZERO; k + 1];
    row[0] = IBig::ONE;
    for i in 1..=n as usize {
        for j in (1..=k.min(i)).rev() {
            row[j] = IBig::from(j) * &row[j] + &row[j - 1];
        }
        row[0] = IBig::ZERO;
    }
    Ok(row[k].clone())
}

/// Independent path: S(n,k) = (1/k!) Σ_{j=0..k} (−1)^j C(k,j) (k−j)^n.
pub fn stirling2_explicit(n: u64, k: u64) -> Z {
    let mut s = IBig::ZERO;
    for j in 0..=k {
        let t = binomial(&IBig::from(k), &IBig::from(j)).unwrap_or_default() * IBig::from(k - j).pow(n as usize);
        if j % 2 == 0 {
            s += t;
        } else {
            s -= t;
        }
    }
    s / prod_range(1, k)
}

/// Unsigned Stirling I c(n,k): c(i,j) = (i−1)·c(i−1,j) + c(i−1,j−1). Also returns the row sum (= n!).
pub fn stirling1(n: u64, k: u64) -> Result<(Z, Z), String> {
    if n > 3000 {
        return Err("s(n,k): n > 3000".into());
    }
    let n = n as usize;
    let mut row = vec![IBig::ZERO; n + 1];
    row[0] = IBig::ONE;
    for i in 1..=n {
        for j in (1..=i).rev() {
            row[j] = IBig::from(i - 1) * &row[j] + &row[j - 1];
        }
        row[0] = IBig::ZERO;
    }
    let sum = row.iter().fold(IBig::ZERO, |a, b| a + b);
    Ok((if (k as usize) <= n { row[k as usize].clone() } else { IBig::ZERO }, sum))
}

pub fn bell(n: u64) -> Result<Z, String> {
    if n > 2000 {
        return Err("Bell: n > 2000".into());
    }
    // Bell triangle
    let mut row = vec![IBig::ONE];
    for _ in 0..n {
        let mut next = Vec::with_capacity(row.len() + 1);
        next.push(row.last().unwrap().clone());
        for x in &row {
            let v = next.last().unwrap() + x;
            next.push(v);
        }
        row = next;
    }
    Ok(row[0].clone())
}

pub fn check_bell(n: u64, v: &Z) -> Check {
    if n > 400 {
        return Check::new("Bell: second path only up to n ≤ 400", true, "");
    }
    let s = (0..=n).fold(IBig::ZERO, |acc, k| acc + stirling2(n, k).unwrap_or_default());
    Check::new("Bell: Σₖ S(n,k)", s == *v, "")
}

/// p(n) — Euler's pentagonal recurrence.
pub fn partitions(n: u64) -> Result<Z, String> {
    if n > 200_000 {
        return Err("p(n): n > 2·10^5".into());
    }
    let n = n as usize;
    let mut p = vec![IBig::ZERO; n + 1];
    p[0] = IBig::ONE;
    for i in 1..=n {
        let mut s = IBig::ZERO;
        let mut k = 1i64;
        loop {
            let g1 = (k * (3 * k - 1) / 2) as usize;
            if g1 > i {
                break;
            }
            let sign_pos = k % 2 == 1;
            let add = |s: &mut Z, v: &Z| if sign_pos { *s += v } else { *s -= v };
            add(&mut s, &p[i - g1]);
            let g2 = (k * (3 * k + 1) / 2) as usize;
            if g2 <= i {
                add(&mut s, &p[i - g2]);
            }
            k += 1;
        }
        p[i] = s;
    }
    Ok(p[n].clone())
}

/// Independent path: "coin change" dynamic programme — partitions into parts ≤ k, O(n²).
pub fn partitions_dp(n: u64) -> Option<Z> {
    if n > 3000 {
        return None;
    }
    let n = n as usize;
    let mut p = vec![IBig::ZERO; n + 1];
    p[0] = IBig::ONE;
    for part in 1..=n {
        for s in part..=n {
            let v = p[s - part].clone();
            p[s] += v;
        }
    }
    Some(p[n].clone())
}

/// Partitions of n into exactly k parts: p(n,k) = p(n−1,k−1) + p(n−k,k).
pub fn partitions_k(n: u64, k: u64) -> Result<Z, String> {
    if n > 5000 {
        return Err("p(n,k): n > 5000".into());
    }
    let (n, k) = (n as usize, k as usize);
    if k > n {
        return Ok(if n == 0 && k == 0 { IBig::ONE } else { IBig::ZERO });
    }
    let mut t = vec![vec![IBig::ZERO; k + 1]; n + 1];
    t[0][0] = IBig::ONE;
    for i in 1..=n {
        for j in 1..=k.min(i) {
            t[i][j] = t[i - 1][j - 1].clone() + t[i - j][j].clone();
        }
    }
    Ok(t[n][k].clone())
}

/// Partitions into distinct parts q(n) and (independently) into odd parts — equal by Euler.
pub fn partitions_distinct(n: u64) -> Result<(Z, Z), String> {
    if n > 5000 {
        return Err("q(n): n > 5000".into());
    }
    let n = n as usize;
    let mut d = vec![IBig::ZERO; n + 1];
    d[0] = IBig::ONE;
    for part in 1..=n {
        for s in (part..=n).rev() {
            let v = d[s - part].clone();
            d[s] += v;
        }
    }
    let mut o = vec![IBig::ZERO; n + 1];
    o[0] = IBig::ONE;
    for part in (1..=n).step_by(2) {
        for s in part..=n {
            let v = o[s - part].clone();
            o[s] += v;
        }
    }
    Ok((d[n].clone(), o[n].clone()))
}

pub fn derangements(n: u64) -> Result<Z, String> {
    if n > 100_000 {
        return Err("D(n): n > 10^5".into());
    }
    let (mut a, mut b) = (IBig::ONE, IBig::ZERO); // D0, D1
    if n == 0 {
        return Ok(a);
    }
    for i in 2..=n {
        let c = IBig::from(i - 1) * (&a + &b);
        a = b;
        b = c;
    }
    Ok(b)
}

/// Independent path: D(n) = Σ_{k=0..n} (−1)^k n!/k!.
pub fn derangements_sum(n: u64) -> Option<Z> {
    if n > 3000 {
        return None;
    }
    let mut s = IBig::ZERO;
    let mut t = IBig::ONE; // n!/k! for k = n, n−1, …
    for k in (0..=n).rev() {
        if k % 2 == 0 {
            s += &t;
        } else {
            s -= &t;
        }
        t *= IBig::from(k.max(1));
    }
    Some(s)
}

/// Fibonacci by fast doubling: (F(n), F(n+1)).
pub fn fib_pair(n: u64) -> (Z, Z) {
    if n == 0 {
        return (IBig::ZERO, IBig::ONE);
    }
    let (a, b) = fib_pair(n / 2);
    let c = &a * (IBig::from(2) * &b - &a);
    let d = &a * &a + &b * &b;
    if n % 2 == 0 { (c, d) } else { (d.clone(), c + d) }
}

/// Cassini: F(n+1)·F(n−1) − F(n)² = (−1)^n.
pub fn check_fib(n: u64, f: &Z, f1: &Z) -> Check {
    if n == 0 {
        return Check::new("Fibonacci: F(0) = 0", f.is_zero(), "");
    }
    let fm1 = f1 - f;
    let lhs = f1 * &fm1 - f * f;
    let rhs = if n % 2 == 0 { IBig::ONE } else { -IBig::ONE };
    Check::new("Fibonacci: Cassini identity", lhs == rhs, "")
}

pub fn short(x: &Z) -> String {
    let s = x.to_string();
    if s.len() > 60 { format!("{}…{} ({} digits)", &s[..25], &s[s.len() - 25..], s.trim_start_matches('-').len()) } else { s }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_values() {
        assert_eq!(factorial(20).unwrap(), z(2_432_902_008_176_640_000));
        assert_eq!(factorial(1000).unwrap(), factorial_legendre(1000));
        assert_eq!(binomial(&z(52), &z(5)).unwrap(), z(2_598_960));
        assert_eq!(binomial(&z(-3), &z(2)).unwrap(), z(6));
        let c = binomial(&z(1000), &z(500)).unwrap();
        assert!(check_binomial(&z(1000), &z(500), &c).ok);
        assert!(!check_binomial(&z(1000), &z(500), &(c + IBig::ONE)).ok, "negative control");
        assert_eq!(catalan(10).unwrap(), z(16796));
        assert!(check_catalan(30, &catalan(30).unwrap()).ok);
        assert_eq!(stirling2(10, 3).unwrap(), z(9330));
        assert_eq!(stirling2(25, 7).unwrap(), stirling2_explicit(25, 7));
        let (s1, sum) = stirling1(6, 3).unwrap();
        assert_eq!(s1, z(225));
        assert_eq!(sum, factorial(6).unwrap());
        assert_eq!(bell(10).unwrap(), z(115_975));
        assert_eq!(partitions(100).unwrap(), z(190_569_292));
        assert_eq!(partitions(1000).unwrap(), partitions_dp(1000).unwrap());
        assert_eq!(partitions_k(10, 3).unwrap(), z(8));
        let (d, o) = partitions_distinct(50).unwrap();
        assert_eq!(d, o);
        assert_eq!(derangements(10).unwrap(), z(1_334_961));
        assert_eq!(derangements(200).unwrap(), derangements_sum(200).unwrap());
        let (f, f1) = fib_pair(100);
        assert_eq!(f.to_string(), "354224848179261915075");
        assert!(check_fib(100, &f, &f1).ok);
        assert!(!check_fib(100, &(f + IBig::ONE), &f1).ok);
    }
}
