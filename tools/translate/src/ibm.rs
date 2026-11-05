//! IBM Model 1 (Brown et al. 1993) for word alignment where two texts share no words (Hebrew ↔ the meta-world).
//! Deterministic: the vocabulary is numbered in sorted order, the translation table is a sorted row per source word,
//! and every sum runs in a fixed order.

use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct Vocab {
    pub ids: BTreeMap<String, u32>,
}

impl Vocab {
    /// Number the strings in sorted order; id 0 is kept for NULL.
    pub fn of<'a>(words: impl Iterator<Item = &'a String>) -> Vocab {
        let mut ids: BTreeMap<String, u32> = words.map(|w| (w.clone(), 0)).collect();
        for (i, v) in ids.values_mut().enumerate() {
            *v = i as u32 + 1;
        }
        Vocab { ids }
    }
    pub fn id(&self, w: &str) -> u32 {
        self.ids.get(w).copied().unwrap_or(0)
    }
}

/// t(e | f): a row per source word f (0 — NULL), sorted by e.
#[derive(Clone, Debug, Default)]
pub struct Ibm1 {
    rows: Vec<Vec<(u32, f64)>>,
}

impl Ibm1 {
    pub fn t(&self, f: u32, e: u32) -> f64 {
        let Some(row) = self.rows.get(f as usize) else { return 0.0 };
        match row.binary_search_by_key(&e, |x| x.0) {
            Ok(i) => row[i].1,
            Err(_) => 0.0,
        }
    }

    /// Train on (source ids, target ids) pairs; `nf` source words (ids 1..=nf).
    #[cfg(test)]
    pub fn train(pairs: &[(Vec<u32>, Vec<u32>)], nf: usize, iters: usize) -> Ibm1 {
        Self::train_prior(pairs, nf, iters, &BTreeMap::new(), 0.0)
    }

    /// The same with a prior: `alpha` pseudo-counts for each (f, e) of `prior` in every iteration (a dictionary's
    /// translations of f).
    pub fn train_prior(pairs: &[(Vec<u32>, Vec<u32>)], nf: usize, iters: usize, prior: &BTreeMap<u32, Vec<u32>>, alpha: f64) -> Ibm1 {
        let mut cols: Vec<Vec<u32>> = vec![Vec::new(); nf + 1];
        for (f, es) in prior {
            if (*f as usize) <= nf {
                cols[*f as usize].extend(es.iter().copied());
            }
        }
        for (f, e) in pairs {
            for &fi in f.iter().chain(std::iter::once(&0)) {
                cols[fi as usize].extend(e.iter().copied());
            }
        }
        let mut rows: Vec<Vec<(u32, f64)>> = cols
            .into_iter()
            .map(|mut c| {
                c.sort_unstable();
                c.dedup();
                let u = if c.is_empty() { 0.0 } else { 1.0 / c.len() as f64 };
                c.into_iter().map(|e| (e, u)).collect()
            })
            .collect();
        for _ in 0..iters {
            let model = Ibm1 { rows: rows.clone() };
            let mut counts: Vec<Vec<f64>> = rows.iter().map(|r| vec![0.0; r.len()]).collect();
            for (f, e) in pairs {
                for &ei in e {
                    let z: f64 = f.iter().chain(std::iter::once(&0)).map(|&fi| model.t(fi, ei)).sum();
                    if z <= 0.0 {
                        continue;
                    }
                    for &fi in f.iter().chain(std::iter::once(&0)) {
                        let row = &rows[fi as usize];
                        if let Ok(i) = row.binary_search_by_key(&ei, |x| x.0) {
                            counts[fi as usize][i] += model.t(fi, ei) / z;
                        }
                    }
                }
            }
            for (f, es) in prior {
                if let Some(row) = rows.get(*f as usize) {
                    for e in es {
                        if let Ok(i) = row.binary_search_by_key(e, |x| x.0) {
                            counts[*f as usize][i] += alpha;
                        }
                    }
                }
            }
            for (r, c) in rows.iter_mut().zip(&counts) {
                let s: f64 = c.iter().sum();
                if s > 0.0 {
                    for (x, v) in r.iter_mut().zip(c) {
                        x.1 = v / s;
                    }
                }
            }
        }
        Ibm1 { rows }
    }
}

/// Align each target word to a source word or to nothing: the highest t(e | f) weighted by the distance of their
/// relative places, against NULL.
pub fn align(m: &Ibm1, f: &[u32], e: &[u32]) -> Vec<Option<usize>> {
    let (n, k) = (f.len().max(1) as f64, e.len().max(1) as f64);
    e.iter()
        .enumerate()
        .map(|(j, &ej)| {
            let mut best = (m.t(0, ej) * 0.3, None);
            for (i, &fi) in f.iter().enumerate() {
                let d = ((i as f64 + 0.5) / n - (j as f64 + 0.5) / k).abs();
                let s = m.t(fi, ej) * (-2.0 * d).exp();
                if s > best.0 {
                    best = (s, Some(i));
                }
            }
            best.1
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learns_a_toy_dictionary() {
        // f: 1 = bayit, 2 = gadol; e: 1 = house, 2 = big, 3 = the
        let pairs = vec![(vec![1, 2], vec![3, 2, 1]), (vec![1], vec![3, 1]), (vec![2], vec![2])];
        let m = Ibm1::train(&pairs, 2, 10);
        assert!(m.t(1, 1) > m.t(1, 2));
        assert!(m.t(2, 2) > m.t(2, 1));
        let a = align(&m, &[1, 2], &[3, 2, 1]);
        assert_eq!(a[1], Some(1));
        assert_eq!(a[2], Some(0));
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn zero_prior_and_normalization() {
        let z = Ibm1::train(&[], 1, 2);
        assert_eq!(align(&z, &[1], &[1, 2]), vec![None, None]);
        let prior = [(1, vec![1, 1, 2])].into();
        let m = Ibm1::train_prior(&[], 1, 1, &prior, 2.0);
        assert!((m.t(1, 1) - 2.0 / 3.0).abs() < 1e-12);
        assert!((m.t(1, 1) + m.t(1, 2) - 1.0).abs() < 1e-12);
        assert_eq!(m.t(99, 1), 0.0);
        assert_eq!(align(&m, &[], &[1]), vec![None]);
        let words = vec!["z".into(), "a".into(), "z".into()];
        let v = Vocab::of(words.iter());
        assert_eq!((v.id("a"), v.id("z"), v.id("?")), (1, 2, 0));
    }
}
