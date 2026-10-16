//! The mathematics lexicon in code: form → class (enum). The `lookup` table is built by build.rs from
//! `data/lexicon.tsv` into a constant `match`; rules then compare only enums.

use crate::units::{Mono, Scale, Unit};

/// A unit word (including compound ones: km/h, mph, m/s).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnitW {
    M,
    Km,
    Cm,
    Mm,
    In,
    Ft,
    Mi,
    Kg,
    G,
    Lb,
    S,
    Min,
    H,
    Day,
    Week,
    Kmh,
    Mph,
    Ms,
    Pa,
    HPa,
    InHg,
}

impl UnitW {
    pub fn mono(self) -> Mono {
        let u = match self {
            UnitW::M => Unit::M,
            UnitW::Km => Unit::Km,
            UnitW::Cm => Unit::Cm,
            UnitW::Mm => Unit::Mm,
            UnitW::In => Unit::In,
            UnitW::Ft => Unit::Ft,
            UnitW::Mi => Unit::Mi,
            UnitW::Kg => Unit::Kg,
            UnitW::G => Unit::G,
            UnitW::Lb => Unit::Lb,
            UnitW::S => Unit::S,
            UnitW::Min => Unit::Min,
            UnitW::H => Unit::H,
            UnitW::Day => Unit::Day,
            UnitW::Week => Unit::Week,
            UnitW::Pa => Unit::Pa,
            UnitW::HPa => Unit::HPa,
            UnitW::InHg => Unit::InHg,
            UnitW::Kmh => return Mono::per(Unit::Km, Unit::H),
            UnitW::Mph => return Mono::per(Unit::Mi, Unit::H),
            UnitW::Ms => return Mono::per(Unit::M, Unit::S),
        };
        Mono::of(u)
    }
}

/// Operator.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Sq,
    Cube,
}

/// Function word (a mathematical function).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FnW {
    Mean,
    Median,
    Max,
    Min,
    Sum,
    Range,
    Product,
    Root,
    Diff,
}

/// Function and frame words.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kw {
    Add,
    Subtract,
    Convert,
    Change,
    To,
    In,
    Of,
    From,
    Between,
    By,
    Than,
    For,
    On,
    Per,
    Each,
    Every,
    Daily,
    Chance,
    Once,
    Least,
    At,
    No,
    All,
    Up,
    Down,
    More,
    Less,
    Become,
    Wh,
    Many,
    Much,
    Be,
    Det,
    A,
    If,
    With,
    And,
    Event,
    DayName,
    Topic,
    Arith,
    Square,
    Cube,
    Ask,
    Pron,
    That,
    Punct,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lex {
    /// Numeral n/d.
    Num(i64, i64),
    /// Multiplier: hundred, thousand, million.
    Mult(i64),
    Dozen,
    /// Fraction: half, third, quarter.
    Frac(i64, i64),
    /// twice, double, triple.
    Times(i64),
    Unit(UnitW),
    Scale(Scale),
    Degree,
    Pct,
    Op(Op),
    Fn(FnW),
    Kw(Kw),
}

include!(concat!(env!("OUT_DIR"), "/lexicon.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_gives_enums() {
        assert_eq!(lookup("miles"), Some(Lex::Unit(UnitW::Mi)));
        assert_eq!(lookup("°f"), Some(Lex::Scale(Scale::F)));
        assert_eq!(lookup("twenty"), Some(Lex::Num(20, 1)));
        assert_eq!(lookup("half"), Some(Lex::Frac(1, 2)));
        assert_eq!(lookup("average"), Some(Lex::Fn(FnW::Mean)));
        assert_eq!(lookup("banana"), None);
        assert_eq!(UnitW::Mph.mono().to_string(), "mph");
    }
}
