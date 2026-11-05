//! Rabiner (1989), A tutorial on hidden Markov models and selected applications
//! in speech recognition. Log-sum-exp forward/backward recurrence, transition
//! [from][to], emissions[time][state]. Posterior rows are normalized after
//! exponentiation. Impossible sequences return None; empty likelihood is one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Dimension,
    Probability,
    Normalization,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Posterior {
    pub states: Vec<Vec<f64>>,
    pub log_likelihood: f64,
}
fn log_sum(values: impl Iterator<Item = f64>) -> f64 {
    let values: Vec<f64> = values.collect();
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if max == f64::NEG_INFINITY {
        max
    } else {
        max + values.iter().map(|v| (v - max).exp()).sum::<f64>().ln()
    }
}
pub fn forward_backward(
    initial: &[f64],
    transition: &[Vec<f64>],
    emissions: &[Vec<f64>],
) -> Result<Option<Posterior>, Error> {
    validate(initial, transition, emissions)?;
    let n = initial.len();
    let t = emissions.len();
    if t == 0 {
        return Ok(Some(Posterior {
            states: Vec::new(),
            log_likelihood: 0.0,
        }));
    }
    let mut alpha = vec![vec![f64::NEG_INFINITY; n]; t];
    for (s, &p) in initial.iter().enumerate() {
        alpha[0][s] = p.ln() + emissions[0][s].ln();
    }
    for time in 1..t {
        for to in 0..n {
            alpha[time][to] = emissions[time][to].ln()
                + log_sum((0..n).map(|from| alpha[time - 1][from] + transition[from][to].ln()));
        }
    }
    let log_likelihood = log_sum(alpha[t - 1].iter().copied());
    if log_likelihood == f64::NEG_INFINITY {
        return Ok(None);
    }
    let mut beta = vec![vec![0.0; n]; t];
    for time in (0..t - 1).rev() {
        for (from, row) in transition.iter().enumerate() {
            beta[time][from] = log_sum(
                (0..n).map(|to| row[to].ln() + emissions[time + 1][to].ln() + beta[time + 1][to]),
            );
        }
    }
    let mut states = vec![vec![0.0; n]; t];
    for time in 0..t {
        for s in 0..n {
            states[time][s] = (alpha[time][s] + beta[time][s] - log_likelihood).exp();
        }
        let total: f64 = states[time].iter().sum();
        for v in &mut states[time] {
            *v /= total;
        }
    }
    Ok(Some(Posterior {
        states,
        log_likelihood,
    }))
}

fn validate(initial: &[f64], transition: &[Vec<f64>], emissions: &[Vec<f64>]) -> Result<(), Error> {
    let n = initial.len();
    if n == 0
        || n > 64
        || emissions.len() > 4096
        || transition.len() != n
        || transition.iter().chain(emissions).any(|row| row.len() != n)
    {
        return Err(Error::Dimension);
    }
    if initial
        .iter()
        .chain(transition.iter().flatten())
        .chain(emissions.iter().flatten())
        .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
    {
        return Err(Error::Probability);
    }
    if (initial.iter().sum::<f64>() - 1.0).abs() > 1e-10
        || transition
            .iter()
            .any(|row| (row.iter().sum::<f64>() - 1.0).abs() > 1e-10)
    {
        return Err(Error::Normalization);
    }
    Ok(())
}
#[cfg(test)]
mod tests;
