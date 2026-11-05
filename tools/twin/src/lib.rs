//! A deliberately small mathematical twin: seconds = x^T theta.
//! Recursive least squares with a unit observation weight, P0 = I/ridge.
//! Sherman & Morrison (1950), doi:10.1214/aoms/1177729893, rank-one inverse;
//! Kalman (1960), doi:10.1115/1.3662552, sequential prediction and correction.
//! P is inverse information, NOT a calibrated predictive covariance.
//! No forgetting: changed software/data/hardware starts a separate cohort.
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Model {
    pub theta: Vec<f64>,
    pub p: Vec<Vec<f64>>,
    pub n: u64,
    pub squared_error: f64,
    pub min: Vec<f64>,
    pub max: Vec<f64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prediction {
    pub seconds: f64,
    pub leverage: f64,
    pub observations: u64,
    pub historical_rmse: Option<f64>,
    /// Only an axis-aligned observed box; this is not a statistical guarantee.
    pub outside_observed_box: bool,
}
pub fn dot(x: &[f64], y: &[f64]) -> f64 {
    x.iter().zip(y).map(|(a, b)| a * b).sum()
}
impl Model {
    pub fn new(prior: Vec<f64>, ridge: f64) -> Result<Self> {
        ensure!(
            !prior.is_empty() && prior.len() <= 32,
            "need 1..32 features"
        );
        ensure!(prior.iter().all(|v| v.is_finite()), "nonfinite prior");
        ensure!(ridge.is_finite() && ridge > 0., "ridge must be positive");
        let d = prior.len();
        let p = (0..d)
            .map(|i| {
                (0..d)
                    .map(|j| if i == j { 1. / ridge } else { 0. })
                    .collect()
            })
            .collect();
        Ok(Self {
            theta: prior,
            p,
            n: 0,
            squared_error: 0.,
            min: vec![0.; d],
            max: vec![0.; d],
        })
    }
    pub fn validate(&self) -> Result<()> {
        let d = self.theta.len();
        ensure!(
            d > 0 && d <= 32 && self.p.len() == d && self.min.len() == d && self.max.len() == d,
            "invalid model dimensions"
        );
        ensure!(
            self.theta
                .iter()
                .chain(&self.min)
                .chain(&self.max)
                .all(|v| v.is_finite()),
            "nonfinite model"
        );
        ensure!(
            self.squared_error.is_finite() && self.squared_error >= 0.,
            "invalid residual history"
        );
        // Cholesky validates positive definiteness on loaded state and after updates.
        let mut l = vec![vec![0.; d]; d];
        for i in 0..d {
            ensure!(
                self.p[i].len() == d && self.p[i].iter().all(|v| v.is_finite()),
                "invalid information matrix"
            );
        }
        for i in 0..d {
            for j in 0..=i {
                ensure!(
                    (self.p[i][j] - self.p[j][i]).abs() <= 1e-10 * (1. + self.p[i][j].abs()),
                    "asymmetric information matrix"
                );
                let a = self.p[i][j] - (0..j).map(|k| l[i][k] * l[j][k]).sum::<f64>();
                if i == j {
                    ensure!(
                        a > 0. && a.is_finite(),
                        "information matrix lost positive definiteness"
                    );
                    l[i][j] = a.sqrt();
                } else {
                    l[i][j] = a / l[j][j];
                }
            }
        }
        Ok(())
    }
    pub fn predict(&self, x: &[f64]) -> Result<Prediction> {
        self.validate()?;
        ensure!(
            x.len() == self.theta.len() && x.iter().all(|v| v.is_finite()),
            "invalid features"
        );
        let px: Vec<f64> = self.p.iter().map(|r| dot(r, x)).collect();
        let h = dot(x, &px);
        let seconds = dot(x, &self.theta);
        ensure!(
            h.is_finite() && h >= 0. && seconds.is_finite(),
            "unstable prediction"
        );
        Ok(Prediction {
            seconds,
            leverage: h,
            observations: self.n,
            historical_rmse: (self.n > 0).then(|| (self.squared_error / self.n as f64).sqrt()),
            outside_observed_box: self.n == 0
                || x.iter()
                    .enumerate()
                    .any(|(i, v)| *v < self.min[i] || *v > self.max[i]),
        })
    }
    pub fn observe(&mut self, x: &[f64], seconds: f64) -> Result<()> {
        ensure!(seconds.is_finite() && seconds > 0., "invalid duration");
        let f = self.predict(x)?;
        let e = seconds - f.seconds;
        let px: Vec<f64> = self.p.iter().map(|r| dot(r, x)).collect();
        let den = 1. + f.leverage;
        let mut next = self.clone();
        for i in 0..x.len() {
            next.theta[i] += px[i] * e / den;
            next.min[i] = if self.n == 0 {
                x[i]
            } else {
                self.min[i].min(x[i])
            };
            next.max[i] = if self.n == 0 {
                x[i]
            } else {
                self.max[i].max(x[i])
            };
            for j in 0..x.len() {
                next.p[i][j] -= px[i] * px[j] / den;
            }
        }
        next.n += 1;
        next.squared_error += e * e; // prospective residual, never the fitted residual
        next.validate()?;
        *self = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn online_solution_matches_independent_two_dimensional_ridge() {
        let prior = vec![1., 0.5];
        let ridge = 0.25;
        let mut m = Model::new(prior.clone(), ridge).unwrap();
        let mut a = [[ridge, 0.], [0., ridge]];
        let mut b = [ridge * prior[0], ridge * prior[1]];
        for (x, y) in [
            (vec![1., 0.], 2.),
            (vec![1., 1.], 3.1),
            (vec![1., 2.], 3.9),
            (vec![1., 4.], 6.2),
        ] {
            let f = m.predict(&x).unwrap();
            m.observe(&x, y).unwrap();
            let after = m.predict(&x).unwrap().seconds;
            assert!(((y - after) - (y - f.seconds) / (1. + f.leverage)).abs() < 1e-12);
            for i in 0..2 {
                b[i] += x[i] * y;
                for j in 0..2 {
                    a[i][j] += x[i] * x[j];
                }
            }
        }
        let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
        let expected = [
            (b[0] * a[1][1] - b[1] * a[0][1]) / det,
            (a[0][0] * b[1] - a[1][0] * b[0]) / det,
        ];
        for (got, want) in m.theta.iter().zip(expected) {
            assert!((got - want).abs() < 1e-12);
        }
        assert!(m.predict(&[1., 5.]).unwrap().outside_observed_box);
        assert!(!m.predict(&[1., 2.]).unwrap().outside_observed_box);
    }
    #[test]
    fn rejects_nonfinite_and_nonpositive_information() {
        let mut m = Model::new(vec![1.], 1.).unwrap();
        let before = m.clone();
        assert!(m.observe(&[1.], f64::NAN).is_err());
        assert_eq!(m, before);
        assert!(m.predict(&[f64::INFINITY]).is_err());
        m.p[0][0] = -1.;
        assert!(m.predict(&[1.]).is_err());
    }
    #[test]
    fn repeated_corrections_do_not_rewrite_past_forecasts() {
        let mut m = Model::new(vec![0.], 1.).unwrap();
        let first = m.predict(&[1.]).unwrap();
        m.observe(&[1.], 2.).unwrap();
        assert_eq!(first.seconds, 0.);
        assert_eq!(m.squared_error, 4.);
        assert_eq!(m.predict(&[1.]).unwrap().seconds, 1.);
    }
}
