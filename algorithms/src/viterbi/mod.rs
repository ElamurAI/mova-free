//! Viterbi (1967), Error bounds for convolutional codes and an asymptotically
//! optimum decoding algorithm. Log max-plus recurrence. Transition[from][to],
//! emissions[time][state]; normalized initial and transition rows.
//! Exact floating ties choose lowest predecessor, then lowest terminal state;
//! this is not a promise of globally lexicographically smallest paths.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Dimension,
    Probability,
    Normalization,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    pub states: Vec<usize>,
    pub log_probability: f64,
}
pub fn viterbi(
    initial: &[f64],
    transition: &[Vec<f64>],
    emissions: &[Vec<f64>],
) -> Result<Option<Path>, Error> {
    validate(initial, transition, emissions)?;
    let n = initial.len();
    let t = emissions.len();
    if t == 0 {
        return Ok(Some(Path {
            states: Vec::new(),
            log_probability: 0.0,
        }));
    }
    let mut score: Vec<f64> = initial
        .iter()
        .zip(&emissions[0])
        .map(|(a, b)| a.ln() + b.ln())
        .collect();
    let mut predecessors = vec![vec![0; n]; t];
    for time in 1..t {
        let mut next = vec![f64::NEG_INFINITY; n];
        for (to, value) in next.iter_mut().enumerate() {
            for (from, &previous) in score.iter().enumerate() {
                let candidate = previous + transition[from][to].ln() + emissions[time][to].ln();
                if candidate > *value {
                    *value = candidate;
                    predecessors[time][to] = from;
                }
            }
        }
        score = next;
    }
    let mut last = 0;
    for i in 1..n {
        if score[i] > score[last] {
            last = i;
        }
    }
    if score[last] == f64::NEG_INFINITY {
        return Ok(None);
    }
    let log_probability = score[last];
    let mut states = vec![0; t];
    states[t - 1] = last;
    for time in (1..t).rev() {
        states[time - 1] = predecessors[time][states[time]];
    }
    Ok(Some(Path {
        states,
        log_probability,
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
