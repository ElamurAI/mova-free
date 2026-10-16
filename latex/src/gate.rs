//! Round-trip gate: `parse(print(parse(x))) == parse(x)`. The printout must parse back into the
//! same tree (positions are not compared). Negative control: printing with a deliberate defect
//! (`print::Fault`): the gate must turn red.

use crate::ast::Node;
use crate::parse::{Error, parse};
use crate::print::{Fault, to_latex, to_latex_faulty};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoundTrip {
    Ok,
    /// The printout parsed into a different tree.
    Mismatch { printed: String, reparsed: Box<Node> },
    /// The printout failed to parse.
    ReparseError { printed: String, error: Error },
}

impl RoundTrip {
    pub fn is_ok(&self) -> bool {
        matches!(self, RoundTrip::Ok)
    }
}

/// Check a tree with canonical printing.
pub fn check(tree: &Node) -> RoundTrip {
    check_printed(tree, to_latex(tree))
}

/// Check a tree with defective printing (negative control).
pub fn check_faulty(tree: &Node, fault: Fault) -> RoundTrip {
    check_printed(tree, to_latex_faulty(tree, fault))
}

fn check_printed(tree: &Node, printed: String) -> RoundTrip {
    match parse(&printed) {
        Ok(t) if t == *tree => RoundTrip::Ok,
        Ok(t) => RoundTrip::Mismatch { printed, reparsed: Box::new(t) },
        Err(error) => RoundTrip::ReparseError { printed, error },
    }
}

/// Whether the defect affects the tree (independently of printing, by structure): a fraction with different parts, a
/// superscript, a row of three or more atoms. For the negative control, exactly this many must be red.
pub fn affected(tree: &Node, fault: Fault) -> bool {
    let mut hit = false;
    tree.walk(&mut |n| {
        hit |= match (fault, n) {
            (Fault::SwapFrac, Node::Frac(_, a, b)) => a != b,
            (Fault::DropSup, Node::Scripts(s)) => s.sup.is_some(),
            (Fault::DropLast, Node::Row(v)) => v.len() >= 3,
            _ => false,
        };
    });
    hit
}

/// Parse the source and check the round trip.
pub fn roundtrip(src: &str) -> Result<RoundTrip, Error> {
    Ok(check(&parse(src)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn green_on_good_printer_red_on_faulty() {
        let t = parse("\\frac{a}{b}+x^{2}").unwrap();
        assert!(check(&t).is_ok());
        assert!(!check_faulty(&t, Fault::SwapFrac).is_ok());
        assert!(!check_faulty(&t, Fault::DropSup).is_ok());
        assert!(!check_faulty(&t, Fault::DropLast).is_ok());
    }
}
