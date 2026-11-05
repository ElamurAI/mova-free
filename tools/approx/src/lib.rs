//! Shared error contracts for experimental surrogates; no production replacement.
//! Budgets are upper bounds, so an exact answer satisfies every tier.
use std::fmt;

/// The only supported maximum relative-error tiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorBudget {
    OnePercent,
    TwoPercent,
    FivePercent,
    TenPercent,
}

impl ErrorBudget {
    pub const ALL: [Self; 4] = [
        Self::OnePercent,
        Self::TwoPercent,
        Self::FivePercent,
        Self::TenPercent,
    ];

    pub const fn percent(self) -> u32 {
        match self {
            Self::OnePercent => 1,
            Self::TwoPercent => 2,
            Self::FivePercent => 5,
            Self::TenPercent => 10,
        }
    }

    pub fn max_fraction(self) -> f64 {
        f64::from(self.percent()) / 100.0
    }

    pub fn from_percent(percent: u32) -> Result<Self, ApproxError> {
        Self::ALL
            .into_iter()
            .find(|b| b.percent() == percent)
            .ok_or(ApproxError::UnsupportedBudget)
    }

    /// No rounding or snapping: raw configuration must name an exact tier.
    pub fn from_fraction(fraction: f64) -> Result<Self, ApproxError> {
        Self::ALL
            .into_iter()
            .find(|b| b.max_fraction() == fraction)
            .ok_or(ApproxError::UnsupportedBudget)
    }
}

/// Observation metric; the caller must define its domain and zero behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MetricKind {
    ScalarRelative,
    NormalizedRank,
    ObjectiveLoss,
    ProbabilityRelative,
}

impl MetricKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScalarRelative => "scalar_relative",
            Self::NormalizedRank => "normalized_rank",
            Self::ObjectiveLoss => "objective_loss",
            Self::ProbabilityRelative => "probability_relative",
        }
    }
}

/// Mechanism classification, independent of metric and evidence strength.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MethodKind {
    QuantizedNumeric,
    CertifiedSample,
    RelaxationCertificate,
    FloatingNormalization,
    IntervalCertificate,
}

impl MethodKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuantizedNumeric => "quantized_numeric",
            Self::CertifiedSample => "certified_sample",
            Self::RelaxationCertificate => "relaxation_certificate",
            Self::FloatingNormalization => "floating_normalization",
            Self::IntervalCertificate => "interval_certificate",
        }
    }
}

/// Declared assurance scope; labels do not establish or upgrade a guarantee.
/// Finite observations cannot establish analytical or per-input certificates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssuranceKind {
    Analytical,
    PerInputCertificate,
    FiniteObservation,
}

impl AssuranceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Analytical => "analytical",
            Self::PerInputCertificate => "per_input_certificate",
            Self::FiniteObservation => "finite_observation",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ApproxError {
    UnsupportedBudget,
    InvalidMetric,
    NonFiniteValue,
    ZeroReferenceMismatch,
    EmptyValidation,
    EmptyScope,
    BudgetExceeded { measured: f64, maximum: f64 },
}

impl fmt::Display for ApproxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ApproxError {}

/// Provenance is never upgraded from a finite sample to an analytic claim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Evidence {
    /// A caller-supplied analytic bound under the named assumptions.
    /// The contract validates its size, not the mathematical proof.
    AnalyticBound { scope: String },
    /// Maximum across a nonempty finite collection; unsampled cases are unknown.
    ObservedFinite { samples: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Certificate {
    budget: ErrorBudget,
    error_fraction: f64,
    evidence: Evidence,
}

impl Certificate {
    pub fn budget(&self) -> ErrorBudget {
        self.budget
    }
    pub fn error_fraction(&self) -> f64 {
        self.error_fraction
    }
    pub fn evidence(&self) -> &Evidence {
        &self.evidence
    }
}

fn validate_metric(error: f64) -> Result<(), ApproxError> {
    if !error.is_finite() || error < 0.0 {
        Err(ApproxError::InvalidMetric)
    } else {
        Ok(())
    }
}

fn certificate(
    budget: ErrorBudget,
    error: f64,
    evidence: Evidence,
) -> Result<Certificate, ApproxError> {
    validate_metric(error)?;
    if error > budget.max_fraction() {
        return Err(ApproxError::BudgetExceeded {
            measured: error,
            maximum: budget.max_fraction(),
        });
    }
    Ok(Certificate {
        budget,
        error_fraction: error,
        evidence,
    })
}

/// Relative absolute scalar error |approximation-reference|/|reference|.
/// Signed scalar values are valid; error metrics must be nonnegative.
/// At zero reference, exact zero is error zero; any mismatch is an explicit error.
/// Arithmetic overflow/nonfinite results fail closed.
pub fn relative_error(reference: f64, approximation: f64) -> Result<f64, ApproxError> {
    if !reference.is_finite() || !approximation.is_finite() {
        return Err(ApproxError::NonFiniteValue);
    }
    if reference == 0.0 {
        return if approximation == 0.0 {
            Ok(0.0)
        } else {
            Err(ApproxError::ZeroReferenceMismatch)
        };
    }
    let error = ((approximation - reference) / reference).abs();
    validate_metric(error)?;
    Ok(error)
}

/// Validate the maximum of a nonempty finite sample of already-defined metrics.
pub fn observed_certificate(
    budget: ErrorBudget,
    errors: &[f64],
) -> Result<Certificate, ApproxError> {
    if errors.is_empty() {
        return Err(ApproxError::EmptyValidation);
    }
    let mut maximum: f64 = 0.0;
    for &error in errors {
        validate_metric(error)?;
        maximum = maximum.max(error);
    }
    certificate(
        budget,
        maximum,
        Evidence::ObservedFinite {
            samples: errors.len(),
        },
    )
}

/// Record a caller's analytic upper bound and its explicit nonempty scope.
/// This constructor is not a theorem prover.
pub fn analytic_certificate(
    budget: ErrorBudget,
    bound: f64,
    scope: &str,
) -> Result<Certificate, ApproxError> {
    if scope.trim().is_empty() {
        return Err(ApproxError::EmptyScope);
    }
    certificate(
        budget,
        bound,
        Evidence::AnalyticBound {
            scope: scope.to_owned(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_four_tiers_and_no_lower_error_floor() {
        for b in ErrorBudget::ALL {
            assert_eq!(ErrorBudget::from_percent(b.percent()), Ok(b));
            assert_eq!(ErrorBudget::from_fraction(b.max_fraction()), Ok(b));
            assert!(observed_certificate(b, &[0.0, b.max_fraction()]).is_ok());
            assert!(analytic_certificate(b, 0.0, "exact arithmetic").is_ok());
            assert!(observed_certificate(b, &[b.max_fraction() + 0.001]).is_err());
        }
        for p in [0, 3, 11, 15, 25, 26, 100, u32::MAX] {
            assert_eq!(
                ErrorBudget::from_percent(p),
                Err(ApproxError::UnsupportedBudget)
            );
        }
        for f in [
            f64::NAN,
            f64::INFINITY,
            -0.01,
            0.0,
            0.03,
            0.11,
            0.15,
            0.25,
            1.0,
        ] {
            assert_eq!(
                ErrorBudget::from_fraction(f),
                Err(ApproxError::UnsupportedBudget)
            );
        }
    }

    #[test]
    fn ten_percent_boundary_and_cap_negative_control() {
        assert_eq!(ErrorBudget::ALL.map(ErrorBudget::percent), [1, 2, 5, 10]);
        let cap = ErrorBudget::TenPercent;
        assert!(observed_certificate(cap, &[0.0, 0.1]).is_ok());
        assert!(analytic_certificate(cap, 0.1, "caller bound").is_ok());
        let adjacent_above = f64::from_bits(0.1_f64.to_bits() + 1);
        for error in [adjacent_above, 0.11, 0.15, 0.25] {
            assert_eq!(
                ErrorBudget::from_fraction(error),
                Err(ApproxError::UnsupportedBudget)
            );
            for budget in ErrorBudget::ALL {
                assert!(matches!(
                    observed_certificate(budget, &[0.0, error]),
                    Err(ApproxError::BudgetExceeded { .. })
                ));
                assert!(matches!(
                    analytic_certificate(budget, error, "caller bound"),
                    Err(ApproxError::BudgetExceeded { .. })
                ));
            }
        }
        for budget in ErrorBudget::ALL {
            let adjacent = f64::from_bits(budget.max_fraction().to_bits() + 1);
            assert_eq!(
                ErrorBudget::from_fraction(adjacent),
                Err(ApproxError::UnsupportedBudget)
            );
        }
    }

    #[test]
    fn classification_names_are_stable_and_axes_independent() {
        assert_eq!(
            [
                MetricKind::ScalarRelative,
                MetricKind::NormalizedRank,
                MetricKind::ObjectiveLoss,
                MetricKind::ProbabilityRelative,
            ]
            .map(MetricKind::as_str),
            [
                "scalar_relative",
                "normalized_rank",
                "objective_loss",
                "probability_relative"
            ]
        );
        assert_eq!(
            [
                MethodKind::QuantizedNumeric,
                MethodKind::CertifiedSample,
                MethodKind::RelaxationCertificate,
                MethodKind::FloatingNormalization,
                MethodKind::IntervalCertificate,
            ]
            .map(MethodKind::as_str),
            [
                "quantized_numeric",
                "certified_sample",
                "relaxation_certificate",
                "floating_normalization",
                "interval_certificate"
            ]
        );
        assert_eq!(
            [
                AssuranceKind::Analytical,
                AssuranceKind::PerInputCertificate,
                AssuranceKind::FiniteObservation,
            ]
            .map(AssuranceKind::as_str),
            ["analytical", "per_input_certificate", "finite_observation"]
        );
        // Declaring a mechanism cannot change the provenance of a certificate.
        let _method = MethodKind::CertifiedSample;
        let observed = observed_certificate(ErrorBudget::TenPercent, &[0.0]).unwrap();
        assert_eq!(
            observed.evidence(),
            &Evidence::ObservedFinite { samples: 1 }
        );
    }

    #[test]
    fn metrics_fail_closed_and_zero_reference_is_explicit() {
        let b = ErrorBudget::TenPercent;
        for invalid in [-0.001, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                observed_certificate(b, &[0.0, invalid]),
                Err(ApproxError::InvalidMetric)
            );
            assert_eq!(
                analytic_certificate(b, invalid, "scope"),
                Err(ApproxError::InvalidMetric)
            );
        }
        assert_eq!(relative_error(0.0, -0.0), Ok(0.0));
        assert_eq!(
            relative_error(0.0, 1.0),
            Err(ApproxError::ZeroReferenceMismatch)
        );
        assert_eq!(relative_error(-8.0, -7.0), Ok(0.125));
        assert_eq!(relative_error(8.0, 7.0), Ok(0.125));
        assert_eq!(
            relative_error(f64::NAN, 0.0),
            Err(ApproxError::NonFiniteValue)
        );
        assert!(relative_error(f64::MAX, -f64::MAX).is_err());
        assert_eq!(
            observed_certificate(b, &[]),
            Err(ApproxError::EmptyValidation)
        );
        assert_eq!(
            analytic_certificate(b, 0.0, " "),
            Err(ApproxError::EmptyScope)
        );
    }

    #[test]
    fn evidence_kind_and_maximum_are_preserved() {
        let b = ErrorBudget::TenPercent;
        let finite = observed_certificate(b, &[0.02, 0.1, 0.0]).unwrap();
        assert_eq!(finite.budget(), b);
        assert_eq!(finite.error_fraction(), 0.1);
        assert_eq!(finite.evidence(), &Evidence::ObservedFinite { samples: 3 });
        let analytic =
            analytic_certificate(b, 0.1, "finite scalars; bound proved by caller").unwrap();
        assert!(matches!(
            analytic.evidence(),
            Evidence::AnalyticBound { .. }
        ));
    }

    #[test]
    fn negative_control_rejects_mean_instead_of_maximum() {
        // Mean=0.01 would pass the 1% gate, but one point is 2% wrong.
        let errors = [0.0, 0.02];
        assert!(errors.iter().sum::<f64>() / errors.len() as f64 <= 0.01);
        assert!(matches!(
            observed_certificate(ErrorBudget::OnePercent, &errors),
            Err(ApproxError::BudgetExceeded { .. })
        ));
    }
}
