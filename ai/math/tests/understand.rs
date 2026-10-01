//! End-to-end tests: question → UD tree (`en`) → expression → answer. They need the `en` model
//! (data/en/models/ud-ewt-eslspok.bin or MATH_EN_MODEL); without it they are skipped with a message.

use std::path::PathBuf;

use math::err::Error;
use math::eval::{answer, fmt_q};
use math::suite::{self, Verdict};
use math::understand::{MODEL, Understander};

fn load() -> Option<Understander> {
    let p = PathBuf::from(std::env::var("MATH_EN_MODEL").unwrap_or_else(|_| MODEL.to_string()));
    if !p.exists() {
        eprintln!("no model {} — end-to-end tests skipped", p.display());
        return None;
    }
    Some(Understander::load(&p).expect("en model"))
}

fn ans(u: &Understander, q: &str) -> Result<String, Error> {
    let p = u.understand(q)?;
    Ok(fmt_q(&answer(&p.expr)?.q))
}

#[test]
fn brief_samples_end_to_end() {
    let Some(u) = load() else { return };
    assert_eq!(ans(&u, "What is 15% of 80?").unwrap(), "12");
    assert_eq!(ans(&u, "What is 12 plus 30?").unwrap(), "42");
    assert_eq!(ans(&u, "Convert 72°F to Celsius").unwrap(), "≈ 22.2222 °C (exactly 200/9 °C)");
    assert_eq!(ans(&u, "How much is 10 miles in km?").unwrap(), "16.09344 km");
    assert_eq!(ans(&u, "What is the average of 3, 5 and 10?").unwrap(), "6");
    assert_eq!(ans(&u, "If the chance of rain is 30% each day, what is the chance of rain at least once in 3 days?").unwrap(), "65.7 %");
    assert_eq!(ans(&u, "What is the difference between 72°F and 60°F?").unwrap(), "12 Δ°F");
}

/// Negative controls must give red: dimension, nonsense, overflow, absolute vs difference.
#[test]
fn negative_controls_are_red() {
    let Some(u) = load() else { return };
    assert!(matches!(ans(&u, "What is 5 km plus 3 kg?"), Err(Error::Dim { .. })));
    assert!(matches!(ans(&u, "Purple elephants dance on Tuesday."), Err(Error::NotUnderstood(_))));
    assert!(matches!(ans(&u, "What is the color of 15?"), Err(Error::NotUnderstood(_))));
    assert!(matches!(ans(&u, "Convert 72 to Celsius."), Err(Error::NotUnderstood(_))));
    assert!(matches!(ans(&u, "What is 2 to the power of 127?"), Err(Error::Overflow(_))));
    assert!(matches!(ans(&u, "What is 72°F plus 60°F?"), Err(Error::Temp(_))));
    // same number, different kind: a difference of 10 °F ≠ absolute 10 °F
    assert_eq!(ans(&u, "Convert a change of 10°F to Celsius.").unwrap(), "≈ 5.55556 Δ°C (exactly 50/9 Δ°C)");
    assert_eq!(ans(&u, "Convert 10°F to Celsius.").unwrap(), "≈ -12.2222 °C (exactly -110/9 °C)");
}

/// Wrong answers found by probing after the code was written (26.09): parentheses, «X minus p%», degrees without a scale,
/// «a/b of X» — now correct or an honest «don't understand».
#[test]
fn probe_regressions() {
    let Some(u) = load() else { return };
    assert_eq!(ans(&u, "What is (3 + 4) * 2?").unwrap(), "14");
    assert_eq!(ans(&u, "What is ((2 + 3) * (4 - 1))?").unwrap(), "15");
    assert_eq!(ans(&u, "What is 100 minus 20 percent?").unwrap(), "80");
    assert_eq!(ans(&u, "What is 3/4 of 100?").unwrap(), "75");
    assert!(matches!(ans(&u, "It is 20 degrees. What is that in Fahrenheit?"), Err(Error::NotUnderstood(_))));
    assert!(matches!(ans(&u, "What is (2 + 3?"), Err(Error::NotUnderstood(_))));
    assert_eq!(ans(&u, "What is the average high if Monday was 70°F, Tuesday 72°F and Wednesday 74°F?").unwrap(), "72 °F");
}

/// Regression on the frozen suite: no wrong answers; all negative controls (ERR/NU) are red.
#[test]
fn frozen_suite_has_no_wrong_answers() {
    let Some(u) = load() else { return };
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/data/suite-v1.tsv")).unwrap();
    let rep = suite::run(&u, &text).unwrap();
    let wrong: Vec<String> = rep.rows.iter().filter(|r| r.verdict == Verdict::Wrong).map(|r| format!("{} {} → {}", r.id, r.question, r.got)).collect();
    assert!(wrong.is_empty(), "wrong: {wrong:#?}");
    for r in rep.rows.iter().filter(|r| r.want == "NU" || r.want.starts_with("ERR:")) {
        assert_eq!(r.verdict, Verdict::Hit, "negative control is not red: {} {}", r.id, r.got);
    }
}
