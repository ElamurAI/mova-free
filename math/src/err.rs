//! Errors with an explanation. Each corresponds to a code in the test set: `ERR:overflow`, `ERR:div0`,
//! `ERR:dim`, `ERR:temp`, `NU` (not understood). A failed independent check is an error too: an answer
//! that did not pass the gate is not returned.

use std::fmt;

use crate::units::Dim;

pub type R<T> = Result<T, Error>;

/// Wrong handling of an absolute temperature (affine scale) or a temperature difference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TempMisuse {
    /// 72 °F + 60 °F — two absolute temperatures do not add.
    AddPoints,
    /// 2 · 20 °C — an absolute temperature is not multiplied or divided.
    ScalePoint,
    /// Sum of absolute temperatures.
    SumPoints,
    /// −(20 °C) or a power of an absolute temperature.
    NegPoint,
    /// A difference minus an absolute temperature.
    DeltaMinusPoint,
    /// An absolute temperature together with a plain number in a sum or list.
    MixedWithPlain,
    /// An absolute temperature is asked to be converted to a difference unit (or vice versa) without a scale.
    KindMismatch,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// The exact core could not hold the number (i128) — the operation is named.
    Overflow(&'static str),
    DivZero,
    /// Dimensions do not match: what was done and with what.
    Dim { op: &'static str, a: Dim, b: Dim },
    Temp(TempMisuse),
    /// Outside the domain (square root of a negative, logarithm of zero…).
    Domain(&'static str),
    /// The answer did not pass the independent check.
    CheckFailed(String),
    /// An honest "not understood" with a reason.
    NotUnderstood(String),
}

impl Error {
    /// Code for the test set.
    pub fn code(&self) -> &'static str {
        match self {
            Error::Overflow(_) => "ERR:overflow",
            Error::DivZero => "ERR:div0",
            Error::Dim { .. } => "ERR:dim",
            Error::Temp(_) => "ERR:temp",
            Error::Domain(_) => "ERR:domain",
            Error::CheckFailed(_) => "ERR:check",
            Error::NotUnderstood(_) => "NU",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Overflow(op) => write!(f, "exact core overflow ({op}): the number does not fit in i128 — I will not silently approximate"),
            Error::DivZero => write!(f, "division by zero"),
            Error::Dim { op, a, b } => write!(f, "dimension error ({op}): {} and {} — different dimensions, no result", a.name(), b.name()),
            Error::Temp(m) => {
                let s = match m {
                    TempMisuse::AddPoints => "two absolute temperatures do not add (72 °F + 60 °F makes no sense); a difference is «difference between», a mean is «average», a change is «a change of 10 °F»",
                    TempMisuse::ScalePoint => "an absolute temperature is not multiplied or divided: 2 · 20 °C ≠ 40 °C (in kelvins it is 2 · 293.15 K); a temperature difference can be multiplied",
                    TempMisuse::SumPoints => "a sum of absolute temperatures makes no sense; for a mean use «average»",
                    TempMisuse::NegPoint => "an absolute temperature is not negated or raised to a power",
                    TempMisuse::DeltaMinusPoint => "an absolute temperature is not subtracted from a temperature difference",
                    TempMisuse::MixedWithPlain => "an absolute temperature together with a number without a scale — I don't know which scale the number is in",
                    TempMisuse::KindMismatch => "an absolute temperature and a temperature difference are different things: +10 °F as a change is a 50/9 °C difference, not −12.2 °C",
                };
                write!(f, "temperature: {s}")
            }
            Error::Domain(s) => write!(f, "outside the domain: {s}"),
            Error::CheckFailed(s) => write!(f, "the answer did not pass the independent check: {s}"),
            Error::NotUnderstood(s) => write!(f, "not understood: {s}"),
        }
    }
}

impl std::error::Error for Error {}
