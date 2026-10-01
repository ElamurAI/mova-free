//! Units of measurement with dimension checking. Dimensions are a vector of exponents over the base
//! (length, mass, time, temperature); "fraction" (%) is dimensionless: 15 % of 80 is the number 12.
//! Units are an enum with exact definitions via rational factors to SI
//! (1 in = 2.54 cm, 1 mi = 1609.344 m, 1 lb = 0.45359237 kg, 1 inHg = 25.4 · 133.322387415 Pa).
//!
//! Temperature is affine: an absolute temperature is a point on a scale (`Scale`), a difference is an ordinary
//! multiplicative unit (Δ°C = 1 K, Δ°F = 5/9 K). Point ≠ difference.

use std::fmt;

use crate::err::{Error, R};
use crate::rat::Rat;

/// Dimension: exponents of length, mass, time, temperature.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct Dim {
    pub l: i8,
    pub m: i8,
    pub t: i8,
    pub th: i8,
}

impl Dim {
    pub const NONE: Dim = Dim { l: 0, m: 0, t: 0, th: 0 };
    pub const L: Dim = Dim { l: 1, m: 0, t: 0, th: 0 };
    pub const M: Dim = Dim { l: 0, m: 1, t: 0, th: 0 };
    pub const T: Dim = Dim { l: 0, m: 0, t: 1, th: 0 };
    pub const TH: Dim = Dim { l: 0, m: 0, t: 0, th: 1 };
    pub const SPEED: Dim = Dim { l: 1, m: 0, t: -1, th: 0 };
    pub const PRESSURE: Dim = Dim { l: -1, m: 1, t: -2, th: 0 };

    pub fn mul(self, o: Dim) -> Dim {
        Dim { l: self.l + o.l, m: self.m + o.m, t: self.t + o.t, th: self.th + o.th }
    }
    pub fn pow(self, k: i8) -> Dim {
        Dim { l: self.l * k, m: self.m * k, t: self.t * k, th: self.th * k }
    }
    pub fn is_none(self) -> bool {
        self == Dim::NONE
    }

    /// Dimension name for explanations.
    pub fn name(self) -> String {
        match self {
            Dim::NONE => "number (dimensionless)".into(),
            Dim::L => "length".into(),
            Dim::M => "mass".into(),
            Dim::T => "time".into(),
            Dim::TH => "temperature".into(),
            Dim::SPEED => "speed".into(),
            Dim::PRESSURE => "pressure".into(),
            d => {
                let mut parts = Vec::new();
                for (e, s) in [(d.l, "L"), (d.m, "M"), (d.t, "T"), (d.th, "Θ")] {
                    if e == 1 {
                        parts.push(s.to_string());
                    } else if e != 0 {
                        parts.push(format!("{s}^{e}"));
                    }
                }
                format!("dimension {}", parts.join("·"))
            }
        }
    }
}

macro_rules! units {
    ($($v:ident = $sym:literal, $dim:expr, $num:literal / $den:literal;)+) => {
        /// Multiplicative unit (atom). Absolute temperatures are separate: `Scale`.
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
        pub enum Unit { $($v),+ }

        impl Unit {
            pub const ALL: &'static [Unit] = &[$(Unit::$v),+];
            /// Canonical symbol.
            pub fn sym(self) -> &'static str {
                match self { $(Unit::$v => $sym),+ }
            }
            pub fn dim(self) -> Dim {
                match self { $(Unit::$v => $dim),+ }
            }
            /// Exact factor to SI units (m, kg, s, K; Pa = kg·m⁻¹·s⁻²).
            pub fn scale(self) -> Rat {
                match self { $(Unit::$v => Rat::new($num, $den).expect("unit definition")),+ }
            }
            pub fn parse_sym(s: &str) -> Option<Unit> {
                match s { $($sym => Some(Unit::$v),)+ _ => None }
            }
        }
    };
}

units! {
    M = "m", Dim::L, 1 / 1;
    Km = "km", Dim::L, 1000 / 1;
    Cm = "cm", Dim::L, 1 / 100;
    Mm = "mm", Dim::L, 1 / 1000;
    In = "in", Dim::L, 254 / 10000;
    Ft = "ft", Dim::L, 3048 / 10000;
    Mi = "mi", Dim::L, 1609344 / 1000;
    Kg = "kg", Dim::M, 1 / 1;
    G = "g", Dim::M, 1 / 1000;
    Lb = "lb", Dim::M, 45359237 / 100000000;
    S = "s", Dim::T, 1 / 1;
    Min = "min", Dim::T, 60 / 1;
    H = "h", Dim::T, 3600 / 1;
    Day = "day", Dim::T, 86400 / 1;
    Week = "week", Dim::T, 604800 / 1;
    DK = "ΔK", Dim::TH, 1 / 1;
    DC = "Δ°C", Dim::TH, 1 / 1;
    DF = "Δ°F", Dim::TH, 5 / 9;
    Pct = "%", Dim::NONE, 1 / 100;
    Pa = "Pa", Dim::PRESSURE, 1 / 1;
    HPa = "hPa", Dim::PRESSURE, 100 / 1;
    InHg = "inHg", Dim::PRESSURE, 3386388640341 / 1000000000;
}

/// Absolute temperature scale: K = (x + offset) · factor.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Scale {
    C,
    F,
    K,
}

impl Scale {
    pub fn sym(self) -> &'static str {
        match self {
            Scale::C => "°C",
            Scale::F => "°F",
            Scale::K => "K",
        }
    }
    /// Difference unit on this scale.
    pub fn delta(self) -> Unit {
        match self {
            Scale::C => Unit::DC,
            Scale::F => Unit::DF,
            Scale::K => Unit::DK,
        }
    }
    /// Zero offset of the scale (in scale degrees): K = (x + off) · delta.scale.
    fn offset(self) -> Rat {
        match self {
            Scale::C => Rat::new(27315, 100).unwrap(),
            Scale::F => Rat::new(45967, 100).unwrap(),
            Scale::K => Rat::ZERO,
        }
    }
    pub fn to_kelvin(self, x: Rat) -> R<Rat> {
        x.add(self.offset())?.mul(self.delta().scale())
    }
    pub fn from_kelvin(self, k: Rat) -> R<Rat> {
        k.div(self.delta().scale())?.sub(self.offset())
    }
    pub fn to_kelvin_f64(self, x: f64) -> f64 {
        (x + self.offset().to_f64()) * self.delta().scale().to_f64()
    }
    pub fn from_kelvin_f64(self, k: f64) -> f64 {
        k / self.delta().scale().to_f64() - self.offset().to_f64()
    }
    pub fn parse_sym(s: &str) -> Option<Scale> {
        match s {
            "°C" => Some(Scale::C),
            "°F" => Some(Scale::F),
            "K" => Some(Scale::K),
            _ => None,
        }
    }
}

/// Product of units with integer exponents: km·h⁻¹, m·s⁻¹. Empty means a dimensionless number.
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
pub struct Mono(pub Vec<(Unit, i8)>);

impl Mono {
    pub fn one() -> Mono {
        Mono(Vec::new())
    }
    pub fn of(u: Unit) -> Mono {
        Mono(vec![(u, 1)])
    }
    pub fn per(a: Unit, b: Unit) -> Mono {
        Mono::of(a).mul(&Mono(vec![(b, -1)]))
    }
    pub fn is_one(&self) -> bool {
        self.0.is_empty()
    }
    pub fn mul(&self, o: &Mono) -> Mono {
        let mut v = self.0.clone();
        for &(u, e) in &o.0 {
            match v.iter_mut().find(|x| x.0 == u) {
                Some(x) => x.1 += e,
                None => v.push((u, e)),
            }
        }
        v.retain(|x| x.1 != 0);
        Mono(v)
    }
    pub fn pow(&self, k: i8) -> Mono {
        let mut v: Vec<(Unit, i8)> = self.0.iter().map(|&(u, e)| (u, e * k)).collect();
        v.retain(|x| x.1 != 0);
        Mono(v)
    }
    pub fn inv(&self) -> Mono {
        self.pow(-1)
    }
    pub fn dim(&self) -> Dim {
        self.0.iter().fold(Dim::NONE, |d, &(u, e)| d.mul(u.dim().pow(e)))
    }
    /// Exact factor to SI.
    pub fn scale(&self) -> R<Rat> {
        let mut s = Rat::ONE;
        for &(u, e) in &self.0 {
            s = s.mul(u.scale().pow(e as i64)?)?;
        }
        Ok(s)
    }
    pub fn is_pct(&self) -> bool {
        self.0 == [(Unit::Pct, 1)]
    }
    /// Temperature-difference unit, if this is one (Δ°C, Δ°F, ΔK).
    pub fn temp_delta(&self) -> Option<Unit> {
        match self.0.as_slice() {
            [(u @ (Unit::DC | Unit::DF | Unit::DK), 1)] => Some(*u),
            _ => None,
        }
    }

    /// Parse canonical notation: "km/h", "mph", "m/s", "Δ°C", "%"…
    pub fn parse_sym(s: &str) -> Option<Mono> {
        if s == "mph" {
            return Some(Mono::per(Unit::Mi, Unit::H));
        }
        if let Some((a, b)) = s.split_once('/') {
            return Some(Mono::per(Unit::parse_sym(a)?, Unit::parse_sym(b)?));
        }
        Unit::parse_sym(s).map(Mono::of)
    }
}

impl fmt::Display for Mono {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == Mono::per(Unit::Mi, Unit::H).0 {
            return f.write_str("mph");
        }
        let num: Vec<String> = self.0.iter().filter(|x| x.1 > 0).map(|&(u, e)| if e == 1 { u.sym().to_string() } else { format!("{}^{e}", u.sym()) }).collect();
        let den: Vec<String> = self.0.iter().filter(|x| x.1 < 0).map(|&(u, e)| if e == -1 { u.sym().to_string() } else { format!("{}^{}", u.sym(), -e) }).collect();
        let n = if num.is_empty() { "1".to_string() } else { num.join("·") };
        if den.is_empty() { f.write_str(&n) } else { write!(f, "{n}/{}", den.join("·")) }
    }
}

/// Exact conversion factor from unit `from` to `to` (same dimensions).
pub fn factor(from: &Mono, to: &Mono, op: &'static str) -> R<Rat> {
    if from.dim() != to.dim() {
        return Err(Error::Dim { op, a: from.dim(), b: to.dim() });
    }
    from.scale()?.div(to.scale()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(s: &str) -> Rat {
        Rat::parse(s).unwrap()
    }

    /// Conversions by exact definitions, no tolerance.
    #[test]
    fn exact_definitions() {
        assert_eq!(factor(&Mono::of(Unit::In), &Mono::of(Unit::Cm), "t").unwrap(), r("2.54"));
        assert_eq!(factor(&Mono::of(Unit::Mi), &Mono::of(Unit::M), "t").unwrap(), r("1609.344"));
        assert_eq!(factor(&Mono::of(Unit::Mi), &Mono::of(Unit::Ft), "t").unwrap(), r("5280"));
        assert_eq!(factor(&Mono::of(Unit::Ft), &Mono::of(Unit::In), "t").unwrap(), r("12"));
        assert_eq!(factor(&Mono::of(Unit::Lb), &Mono::of(Unit::G), "t").unwrap(), r("453.59237"));
        assert_eq!(factor(&Mono::of(Unit::Week), &Mono::of(Unit::Min), "t").unwrap(), r("10080"));
        assert_eq!(factor(&Mono::per(Unit::M, Unit::S), &Mono::per(Unit::Km, Unit::H), "t").unwrap(), r("3.6"));
        assert_eq!(factor(&Mono::per(Unit::Mi, Unit::H), &Mono::per(Unit::Km, Unit::H), "t").unwrap(), r("1.609344"));
        assert_eq!(factor(&Mono::of(Unit::InHg), &Mono::of(Unit::HPa), "t").unwrap(), r("33.86388640341"));
        assert_eq!(factor(&Mono::of(Unit::DF), &Mono::of(Unit::DC), "t").unwrap(), r("5/9"));
    }

    /// °F = °C·9/5 + 32 exactly; affinity: 0 °C ≠ 0 °F, a difference of 1 °F = 5/9 °C.
    #[test]
    fn temperature_is_affine() {
        let k = Scale::F.to_kelvin(r("72")).unwrap();
        assert_eq!(Scale::C.from_kelvin(k).unwrap(), r("200/9"));
        let k = Scale::C.to_kelvin(r("100")).unwrap();
        assert_eq!(Scale::F.from_kelvin(k).unwrap(), r("212"));
        assert_eq!(Scale::C.to_kelvin(Rat::ZERO).unwrap(), r("273.15"));
        assert_eq!(Scale::F.from_kelvin(Scale::C.to_kelvin(r("-40")).unwrap()).unwrap(), r("-40"));
        // a difference of 10 °F is 50/9 °C, while an absolute 10 °F is −110/9 °C
        assert_eq!(factor(&Mono::of(Unit::DF), &Mono::of(Unit::DC), "t").unwrap().mul(r("10")).unwrap(), r("50/9"));
        assert_eq!(Scale::C.from_kelvin(Scale::F.to_kelvin(r("10")).unwrap()).unwrap(), r("-110/9"));
    }

    /// Negative control: different dimensions do not convert.
    #[test]
    fn dimension_mismatch_is_error() {
        assert!(matches!(factor(&Mono::of(Unit::Kg), &Mono::of(Unit::Km), "t"), Err(Error::Dim { .. })));
        assert!(matches!(factor(&Mono::per(Unit::Km, Unit::H), &Mono::of(Unit::Kg), "t"), Err(Error::Dim { .. })));
        assert!(matches!(factor(&Mono::of(Unit::DC), &Mono::of(Unit::M), "t"), Err(Error::Dim { .. })));
    }

    #[test]
    fn mono_algebra_and_display() {
        let kmh = Mono::per(Unit::Km, Unit::H);
        assert_eq!(kmh.to_string(), "km/h");
        assert_eq!(kmh.dim(), Dim::SPEED);
        assert_eq!(kmh.mul(&Mono::of(Unit::H)), Mono::of(Unit::Km));
        assert_eq!(Mono::per(Unit::Mi, Unit::H).to_string(), "mph");
        assert_eq!(Mono::parse_sym("m/s"), Some(Mono::per(Unit::M, Unit::S)));
        assert_eq!(Mono::parse_sym("Δ°F"), Some(Mono::of(Unit::DF)));
    }
}
