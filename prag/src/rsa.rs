//! Rational Speech Acts over small closed sets, exact and deterministic (docs/pragmatics-algorithms.md).
//!
//! The recursion of Frank & Goodman 2012 and Monroe & Potts 2015, with the layers as in Anderson 2019/2021:
//!
//! - literal listener L0(w|u) ∝ [[u]](w)·P(w);
//! - pragmatic speaker S1(u|w) ∝ L0(w|u)^α · 2^(−α·C(u));
//! - pragmatic listener L1(w|u) ∝ S1(u|w)·P(w).
//!
//! α is a small positive integer, costs are whole bits and priors positive integers, so every value is an exact
//! fraction (`Ratio`, u128 with gcd reduction; overflow fails loudly instead of rounding). Worlds and utterances keep
//! the order they are given in; ties go to the lowest index. The L0/S1/L1 rows are the explanation of a reading.

use std::fmt::Write as _;

/// An exact non-negative fraction, always reduced; the denominator is never zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ratio {
    pub num: u128,
    pub den: u128,
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl Ratio {
    pub const ZERO: Ratio = Ratio { num: 0, den: 1 };
    pub const ONE: Ratio = Ratio { num: 1, den: 1 };

    pub fn new(num: u128, den: u128) -> Ratio {
        assert!(den != 0, "ratio with zero denominator");
        let g = gcd(num, den).max(1);
        Ratio { num: num / g, den: den / g }
    }

    pub fn int(n: u128) -> Ratio {
        Ratio { num: n, den: 1 }
    }

    pub fn is_zero(self) -> bool {
        self.num == 0
    }

    pub fn add(self, o: Ratio) -> Ratio {
        let g = gcd(self.den, o.den);
        let den = (self.den / g).checked_mul(o.den).expect("rsa: overflow");
        let a = self.num.checked_mul(o.den / g).expect("rsa: overflow");
        let b = o.num.checked_mul(self.den / g).expect("rsa: overflow");
        Ratio::new(a.checked_add(b).expect("rsa: overflow"), den)
    }

    pub fn mul(self, o: Ratio) -> Ratio {
        let g1 = gcd(self.num, o.den).max(1);
        let g2 = gcd(o.num, self.den).max(1);
        let num = (self.num / g1).checked_mul(o.num / g2).expect("rsa: overflow");
        let den = (self.den / g2).checked_mul(o.den / g1).expect("rsa: overflow");
        Ratio::new(num, den)
    }

    pub fn div(self, o: Ratio) -> Ratio {
        assert!(!o.is_zero(), "rsa: division by zero");
        self.mul(Ratio { num: o.den, den: o.num })
    }

    pub fn pow(self, e: u32) -> Ratio {
        (0..e).fold(Ratio::ONE, |acc, _| acc.mul(self))
    }

    /// Percentage rounded half up to one decimal, for display only.
    pub fn percent(self) -> String {
        let tenths = (self.num * 2000 + self.den) / (2 * self.den);
        format!("{}.{}%", tenths / 10, tenths % 10)
    }
}

impl Ord for Ratio {
    fn cmp(&self, o: &Ratio) -> std::cmp::Ordering {
        self.num.checked_mul(o.den).expect("rsa: overflow").cmp(&o.num.checked_mul(self.den).expect("rsa: overflow"))
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, o: &Ratio) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}

impl std::fmt::Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.den == 1 { write!(f, "{}", self.num) } else { write!(f, "{}/{}", self.num, self.den) }
    }
}

/// Scales a row to sum to one; an all-zero row stays zero (nothing to normalise).
fn normalise(row: Vec<Ratio>) -> Vec<Ratio> {
    let sum = row.iter().fold(Ratio::ZERO, |a, &x| a.add(x));
    if sum.is_zero() { row } else { row.into_iter().map(|x| x.div(sum)).collect() }
}

/// A reference game: worlds, utterances, truth `[u][w]`, integer priors, costs in bits, rationality α.
#[derive(Clone, Debug)]
pub struct Game {
    pub worlds: Vec<String>,
    pub utterances: Vec<String>,
    pub truth: Vec<Vec<bool>>,
    pub prior: Vec<u64>,
    pub cost: Vec<u32>,
    pub alpha: u32,
}

impl Game {
    /// L0 `[u][w]`: the prior restricted to the worlds where u is true.
    pub fn l0(&self) -> Vec<Vec<Ratio>> {
        self.truth
            .iter()
            .map(|t| normalise(t.iter().zip(&self.prior).map(|(&ok, &p)| if ok { Ratio::int(p as u128) } else { Ratio::ZERO }).collect()))
            .collect()
    }

    /// S1 `[w][u]`: how likely the speaker who sees w is to say u.
    pub fn s1(&self) -> Vec<Vec<Ratio>> {
        let l0 = self.l0();
        (0..self.worlds.len())
            .map(|w| {
                normalise(
                    (0..self.utterances.len())
                        .map(|u| l0[u][w].pow(self.alpha).div(Ratio::int(2).pow(self.alpha * self.cost[u])))
                        .collect(),
                )
            })
            .collect()
    }

    /// L1 `[u][w]`: what the hearer infers from u, reasoning about the speaker.
    pub fn l1(&self) -> Vec<Vec<Ratio>> {
        let s1 = self.s1();
        (0..self.utterances.len())
            .map(|u| normalise((0..self.worlds.len()).map(|w| s1[w][u].mul(Ratio::int(self.prior[w] as u128))).collect()))
            .collect()
    }

    /// The world L1 prefers for u; ties go to the lowest index.
    pub fn best_world(&self, u: usize) -> usize {
        let row = &self.l1()[u];
        (0..row.len()).fold(0, |b, w| if row[w] > row[b] { w } else { b })
    }

    /// The L0, S1 and L1 rows for u, as fractions and percentages, and the reading L1 picks.
    pub fn explain(&self, u: usize) -> String {
        let (l0, s1, l1) = (self.l0(), self.s1(), self.l1());
        let cell = |r: Ratio| format!("{} ({})", r, r.percent());
        let mut s = String::new();
        let _ = writeln!(s, "\"{}\" (cost {}, alpha {}), true of: {}", self.utterances[u], self.cost[u], self.alpha,
            self.worlds.iter().zip(&self.truth[u]).filter(|(_, t)| **t).map(|(w, _)| w.as_str()).collect::<Vec<_>>().join(", "));
        for (w, name) in self.worlds.iter().enumerate() {
            let _ = writeln!(s, "  {name:16} prior {:<4} L0 {:18} S1 {:18} L1 {}", self.prior[w], cell(l0[u][w]), cell(s1[w][u]), cell(l1[u][w]));
        }
        let b = self.best_world(u);
        let _ = writeln!(s, "  → {} (L1 {}; literal L0 {})", self.worlds[b], l1[u][b], l0[u][b]);
        s
    }
}

fn names(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|x| x.to_string()).collect()
}

/// The scalar game: "some" is true of some-not-all and of all, but a speaker who saw all would have said "all".
pub fn scalar() -> Game {
    Game {
        worlds: names(&["none", "some-not-all", "all"]),
        utterances: names(&["none", "some", "all"]),
        truth: vec![vec![true, false, false], vec![false, true, true], vec![false, false, true]],
        prior: vec![1, 1, 1],
        cost: vec![0, 0, 0],
        alpha: 1,
    }
}

/// The reference game of Frank & Goodman 2012: a blue square, a blue circle, a green square.
pub fn reference() -> Game {
    Game {
        worlds: names(&["blue square", "blue circle", "green square"]),
        utterances: names(&["blue", "green", "square", "circle"]),
        truth: vec![vec![true, true, false], vec![false, false, true], vec![true, false, true], vec![false, true, false]],
        prior: vec![1, 1, 1],
        cost: vec![0, 0, 0, 0],
        alpha: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(n: u128, d: u128) -> Ratio {
        Ratio::new(n, d)
    }

    #[test]
    fn ratio_arithmetic() {
        assert_eq!(r(2, 4), r(1, 2));
        assert_eq!(r(1, 2).add(r(1, 3)), r(5, 6));
        assert_eq!(r(2, 3).div(r(4, 9)), r(3, 2));
        assert_eq!(r(1, 2).pow(3), r(1, 8));
        assert!(r(1, 3) < r(1, 2));
        assert_eq!(r(3, 4).percent(), "75.0%");
        assert_eq!(r(1, 3).percent(), "33.3%");
        assert_eq!(r(2, 3).percent(), "66.7%");
    }

    /// By hand: L0(some) = (0, 1/2, 1/2). S1 at all: some ∝ 1/2, all ∝ 1 → (1/3, 2/3); at some-not-all only "some"
    /// is true → 1. L1(some) ∝ (0, 1, 1/3) → (0, 3/4, 1/4): the scalar implicature "not all".
    #[test]
    fn scalar_implicature() {
        let g = scalar();
        assert_eq!(g.l0()[1], vec![r(0, 1), r(1, 2), r(1, 2)]);
        assert_eq!(g.s1()[2], vec![r(0, 1), r(1, 3), r(2, 3)]);
        assert_eq!(g.l1()[1], vec![r(0, 1), r(3, 4), r(1, 4)]);
        assert_eq!(g.best_world(1), 1);
        assert_eq!(g.best_world(2), 2);
        assert_eq!(g.l1()[2], vec![r(0, 1), r(0, 1), r(1, 1)]);
    }

    /// A costlier "all" weakens the implicature: with C(all)=2, S1 at all ∝ (1/2, 1·2^-2) → (2/3, 1/3) for some/all,
    /// and L1(some) ∝ (0, 1, 2/3) → (0, 3/5, 2/5). With α=2 the cost counts double: S1 at all ∝ (1/4, 1/16) →
    /// (4/5, 1/5), L1(some) ∝ (0, 1, 4/5) → (0, 5/9, 4/9).
    #[test]
    fn cost_weakens_the_implicature() {
        let mut g = scalar();
        g.cost[2] = 2;
        assert_eq!(g.l1()[1], vec![r(0, 1), r(3, 5), r(2, 5)]);
        g.alpha = 2;
        assert_eq!(g.l1()[1], vec![r(0, 1), r(5, 9), r(4, 9)]);
    }

    /// By hand: L0(blue) = (1/2, 1/2, 0), L0(square) = (1/2, 0, 1/2), L0(circle) = (0, 1, 0), L0(green) = (0, 0, 1).
    /// S1 at blue square: blue 1/2, square 1/2 → (1/2, 1/2); at blue circle: blue 1/2, circle 1 → (1/3, 2/3); at
    /// green square: green 1, square 1/2 → (2/3, 1/3). L1(blue) ∝ (1/2, 1/3, 0) → (3/5, 2/5, 0): the blue square,
    /// since for the blue circle the speaker would rather have said "circle". Likewise L1(square) → (3/5, 0, 2/5).
    #[test]
    fn reference_game() {
        let g = reference();
        assert_eq!(g.s1()[1], vec![r(1, 3), r(0, 1), r(0, 1), r(2, 3)]);
        assert_eq!(g.l1()[0], vec![r(3, 5), r(2, 5), r(0, 1)]);
        assert_eq!(g.l1()[2], vec![r(3, 5), r(0, 1), r(2, 5)]);
        assert_eq!(g.best_world(0), 0);
        assert_eq!(g.best_world(2), 0);
        assert_eq!(g.best_world(3), 1);
        assert_eq!(g.best_world(1), 2);
    }

    /// Negative control: words true of every world carry no information, so L1 is the prior.
    #[test]
    fn uninformative_words_give_the_prior() {
        let g = Game {
            worlds: names(&["a", "b", "c"]),
            utterances: names(&["x", "y"]),
            truth: vec![vec![true; 3], vec![true; 3]],
            prior: vec![1, 2, 3],
            cost: vec![0, 1],
            alpha: 1,
        };
        for row in g.l1() {
            assert_eq!(row, vec![r(1, 6), r(1, 3), r(1, 2)]);
        }
        assert_eq!(g.best_world(0), 2);
    }

    #[test]
    fn ties_go_to_the_lowest_index() {
        let mut g = reference();
        g.truth = vec![vec![true, true, false]; 4];
        assert_eq!(g.l1()[0], vec![r(1, 2), r(1, 2), r(0, 1)]);
        assert_eq!(g.best_world(0), 0);
    }

    #[test]
    fn explain_shows_the_rows() {
        let e = scalar().explain(1);
        assert!(e.contains("some-not-all"));
        assert!(e.contains("3/4 (75.0%)"));
        assert!(e.contains("→ some-not-all"));
    }
}
