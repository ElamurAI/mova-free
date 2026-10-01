//! Exact-arithmetic benchmark for math v2 ("for speed, let it call some module in
//! Rust or one rewritten from C++ for exact math"). Identical algorithms on both libraries where
//! a library lacks its own (factorial — product tree); results are cross-checked between libraries.
//! Run: `cargo run --release` in this directory; output — TSV: task, library, seconds (min. of N).

use std::time::Instant;

fn best<R>(n: usize, mut f: impl FnMut() -> R) -> (R, f64) {
    let mut best = f64::MAX;
    let mut out = None;
    for _ in 0..n {
        let t = Instant::now();
        let r = f();
        best = best.min(t.elapsed().as_secs_f64());
        out = Some(r);
    }
    (out.unwrap(), best)
}

mod d {
    use dashu::base::Gcd;
    use dashu::integer::{UBig, fast_div::ConstDivisor};
    use dashu::rational::RBig;

    fn prod(lo: u64, hi: u64) -> UBig {
        if hi - lo < 16 {
            let mut p = UBig::ONE;
            for k in lo..=hi {
                p *= UBig::from(k);
            }
            return p;
        }
        let m = (lo + hi) / 2;
        prod(lo, m) * prod(m + 1, hi)
    }
    pub fn fact(n: u64) -> UBig {
        prod(1, n)
    }
    pub fn pow3(e: usize) -> UBig {
        UBig::from(3u8).pow(e)
    }
    pub fn gcd_pair() -> (UBig, UBig) {
        let g = UBig::from(2u8).pow(1000) * UBig::from(1_000_003u64);
        let a = (UBig::from(3u8).pow(209_590) + UBig::from(12_345u32)) * &g;
        let b = (UBig::from(7u8).pow(118_330) + UBig::from(6_789u32)) * &g;
        (a, b)
    }
    pub fn gcd(a: &UBig, b: &UBig) -> UBig {
        a.gcd(b)
    }
    pub fn modpow(base: &UBig, exp: &UBig, m: &UBig) -> UBig {
        let ring = ConstDivisor::new(m.clone());
        ring.reduce(base.clone()).pow(exp).residue()
    }
    pub fn big2048(seed: u64) -> UBig {
        // deterministic 2048-bit: digits from an LCG
        let mut x = seed;
        let mut v = UBig::ONE;
        for _ in 0..32 {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            v = (v << 64) + UBig::from(x);
        }
        (v >> 1) | UBig::ONE
    }
    pub fn harmonic(n: u64) -> RBig {
        let mut s = RBig::ZERO;
        for k in 1..=n {
            s += RBig::from_parts(dashu::integer::IBig::ONE, UBig::from(k));
        }
        s
    }
    pub fn to_dec(x: &UBig) -> String {
        x.to_string()
    }
    pub fn rat_str(x: &RBig) -> String {
        x.to_string()
    }
}

mod n {
    use num_bigint::BigUint;
    use num_integer::Integer;
    use num_rational::BigRational;
    use num_traits::{One, Pow, Zero};

    fn prod(lo: u64, hi: u64) -> BigUint {
        if hi - lo < 16 {
            let mut p = BigUint::one();
            for k in lo..=hi {
                p *= k;
            }
            return p;
        }
        let m = (lo + hi) / 2;
        prod(lo, m) * prod(m + 1, hi)
    }
    pub fn fact(n: u64) -> BigUint {
        prod(1, n)
    }
    pub fn pow3(e: u32) -> BigUint {
        Pow::pow(BigUint::from(3u8), e)
    }
    pub fn gcd_pair() -> (BigUint, BigUint) {
        let g = Pow::pow(BigUint::from(2u8), 1000u32) * BigUint::from(1_000_003u64);
        let a = (Pow::pow(BigUint::from(3u8), 209_590u32) + BigUint::from(12_345u32)) * &g;
        let b = (Pow::pow(BigUint::from(7u8), 118_330u32) + BigUint::from(6_789u32)) * &g;
        (a, b)
    }
    pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
        a.gcd(b)
    }
    pub fn modpow(base: &BigUint, exp: &BigUint, m: &BigUint) -> BigUint {
        base.modpow(exp, m)
    }
    pub fn big2048(seed: u64) -> BigUint {
        let mut x = seed;
        let mut v = BigUint::one();
        for _ in 0..32 {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            v = (v << 64u32) + BigUint::from(x);
        }
        (v >> 1u32) | BigUint::one()
    }
    pub fn harmonic(n: u64) -> BigRational {
        let mut s = BigRational::zero();
        for k in 1..=n {
            s += BigRational::new(1.into(), k.into());
        }
        s
    }
    pub fn to_dec(x: &BigUint) -> String {
        x.to_str_radix(10)
    }
    pub fn rat_str(x: &BigRational) -> String {
        x.to_string()
    }
}

mod n5 {
    use num_bigint_05::BigUint;
    use num_integer::Integer;
    use num_traits::{One, Pow};

    fn prod(lo: u64, hi: u64) -> BigUint {
        if hi - lo < 16 {
            let mut p = BigUint::one();
            for k in lo..=hi {
                p *= k;
            }
            return p;
        }
        let m = (lo + hi) / 2;
        prod(lo, m) * prod(m + 1, hi)
    }
    pub fn fact(n: u64) -> BigUint {
        prod(1, n)
    }
    pub fn pow3(e: u32) -> BigUint {
        Pow::pow(BigUint::from(3u8), e)
    }
    pub fn gcd_pair() -> (BigUint, BigUint) {
        let g = Pow::pow(BigUint::from(2u8), 1000u32) * BigUint::from(1_000_003u64);
        let a = (Pow::pow(BigUint::from(3u8), 209_590u32) + BigUint::from(12_345u32)) * &g;
        let b = (Pow::pow(BigUint::from(7u8), 118_330u32) + BigUint::from(6_789u32)) * &g;
        (a, b)
    }
    pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
        a.gcd(b)
    }
    pub fn modpow(base: &BigUint, exp: &BigUint, m: &BigUint) -> BigUint {
        base.modpow(exp, m)
    }
    pub fn big2048(seed: u64) -> BigUint {
        let mut x = seed;
        let mut v = BigUint::one();
        for _ in 0..32 {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            v = (v << 64u32) + BigUint::from(x);
        }
        (v >> 1u32) | BigUint::one()
    }
    pub fn to_dec(x: &BigUint) -> String {
        x.to_str_radix(10)
    }
}

fn row(task: &str, lib: &str, secs: f64) {
    println!("{task}\t{lib}\t{secs:.4}");
}

fn main() {
    let reps = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(3usize);
    println!("task\tlibrary\ts (min. of {reps})");

    // 1. 100 000! (product tree) and its decimal representation
    let (fd, t) = best(reps, || d::fact(100_000));
    row("100000!", "dashu 0.6.1", t);
    let (fnm, t) = best(reps, || n::fact(100_000));
    row("100000!", "num-bigint 0.4.8", t);
    let (f5, t) = best(reps, || n5::fact(100_000));
    row("100000!", "num-bigint 0.5.1", t);
    let (sd, t) = best(reps, || d::to_dec(&fd));
    row("100000! → decimal (456 574 digits)", "dashu 0.6.1", t);
    let (sn, t) = best(reps, || n::to_dec(&fnm));
    row("100000! → decimal (456 574 digits)", "num-bigint 0.4.8", t);
    let (s5, t) = best(reps, || n5::to_dec(&f5));
    row("100000! → decimal (456 574 digits)", "num-bigint 0.5.1", t);
    assert_eq!(sd, sn);
    assert_eq!(sd, s5);
    assert_eq!(sd.len(), 456_574);

    // 2. 3^1 000 000
    let (pd, t) = best(reps, || d::pow3(1_000_000));
    row("3^1000000", "dashu 0.6.1", t);
    let (pn, t) = best(reps, || n::pow3(1_000_000));
    row("3^1000000", "num-bigint 0.4.8", t);
    let (p5, t) = best(reps, || n5::pow3(1_000_000));
    row("3^1000000", "num-bigint 0.5.1", t);
    let (sd, t) = best(reps, || d::to_dec(&pd));
    row("3^1000000 → decimal (477 122 digits)", "dashu 0.6.1", t);
    let (sn, t) = best(reps, || n::to_dec(&pn));
    row("3^1000000 → decimal (477 122 digits)", "num-bigint 0.4.8", t);
    let (s5, t) = best(reps, || n5::to_dec(&p5));
    row("3^1000000 → decimal (477 122 digits)", "num-bigint 0.5.1", t);
    assert_eq!(sd, sn);
    assert_eq!(sd, s5);

    // 3. GCD of two numbers of ~10^5 digits each
    let (ad, bd) = d::gcd_pair();
    let (an, bn) = n::gcd_pair();
    let (a5, b5) = n5::gcd_pair();
    let (gd, t) = best(reps, || d::gcd(&ad, &bd));
    row("GCD of two 10^5-digit numbers", "dashu 0.6.1", t);
    let (gn, t) = best(reps, || n::gcd(&an, &bn));
    row("GCD of two 10^5-digit numbers", "num-bigint 0.4.8", t);
    let (g5, t) = best(reps, || n5::gcd(&a5, &b5));
    row("GCD of two 10^5-digit numbers", "num-bigint 0.5.1", t);
    assert_eq!(gd.to_string(), gn.to_string());
    assert_eq!(gd.to_string(), g5.to_string());

    // 4. modular power, 2048 bits (100 different bases)
    let md = d::big2048(7);
    let ed = d::big2048(11);
    let mn = n::big2048(7);
    let en = n::big2048(11);
    let m5 = n5::big2048(7);
    let e5 = n5::big2048(11);
    let (rd, t) = best(reps, || (0..100u64).map(|s| d::modpow(&d::big2048(100 + s), &ed, &md)).last().unwrap());
    row("modpow 2048 bits ×100", "dashu 0.6.1", t);
    let (rn, t) = best(reps, || (0..100u64).map(|s| n::modpow(&n::big2048(100 + s), &en, &mn)).last().unwrap());
    row("modpow 2048 bits ×100", "num-bigint 0.4.8", t);
    let (r5, t) = best(reps, || (0..100u64).map(|s| n5::modpow(&n5::big2048(100 + s), &e5, &m5)).last().unwrap());
    row("modpow 2048 bits ×100", "num-bigint 0.5.1", t);
    assert_eq!(rd.to_string(), rn.to_string());
    assert_eq!(rd.to_string(), r5.to_string());

    // 5. rationals: harmonic H(3000) exactly
    let (hd, t) = best(reps, || d::harmonic(3000));
    row("H(3000) exact, rationals", "dashu 0.6.1", t);
    let (hn, t) = best(reps, || n::harmonic(3000));
    row("H(3000) exact, rationals", "num-rational 0.4.2", t);
    assert_eq!(d::rat_str(&hd), n::rat_str(&hn));
}
