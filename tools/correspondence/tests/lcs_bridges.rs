//! LCS correspondence bridges: actual `en::text::lcs` against the actual algorithm LCS APIs,
//! a brute-force subsequence oracle, the typed catalogue rows, and the histogram upper bound
//! as a partial excluded relation (never the value). Finite evidence only; no universal claim.
use mova_algorithms::{bit_parallel_lcs, longest_common_subsequence};
use serde_json::Value;

const ROWS: [&str; 3] = ["lcs.ordinary", "lcs.algorithm", "lcs.bitparallel"];
const INPUTS: [(&str, &[u8], &[u8]); 7] = [
    ("[]/[]", &[], &[]),
    ("[]/[1]", &[], &[1]),
    ("[1]/[1]", &[1], &[1]),
    ("[1,2]/[2,1]", &[1, 2], &[2, 1]),
    ("[1,2,3]/[2,1,3]", &[1, 2, 3], &[2, 1, 3]),
    ("[0,1,0,1]/[1,0,1,0]", &[0, 1, 0, 1], &[1, 0, 1, 0]),
    ("[1,1,2]/[1,2,2]", &[1, 1, 2], &[1, 2, 2]),
];

fn formula(id: &str) -> Value {
    let c: Value =
        serde_json::from_str(include_str!("../../../models/correspondences.json")).unwrap();
    c["formulas"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap_or_else(|| panic!("missing catalogue row {id}"))
        .clone()
}

/// Checks a forward row against actual outputs; returns the mismatch instead of panicking so a
/// negative control can show red.
fn observed(id: &str, inputs: &[&str], outputs: &[usize]) -> Result<(), String> {
    let f = formula(id);
    let names: Vec<String> =
        serde_json::from_value(f["contract"]["domain"]["values"].clone()).unwrap();
    if names != inputs {
        return Err(format!("{id}: domain labels differ"));
    }
    let codomain: Vec<String> =
        serde_json::from_value(f["contract"]["codomain"]["values"].clone()).unwrap();
    let actual: Vec<[usize; 2]> = outputs
        .iter()
        .enumerate()
        .map(|(i, o)| {
            let label = o.to_string();
            let j = codomain
                .iter()
                .position(|v| *v == label)
                .ok_or_else(|| format!("{id}: output {label} absent from codomain"))?;
            Ok::<_, String>([i, j])
        })
        .collect::<Result<_, _>>()?;
    let pairs: Vec<[usize; 2]> = serde_json::from_value(f["expr"]["pairs"].clone()).unwrap();
    if pairs != actual {
        return Err(format!("{id}: pairs {pairs:?} != actual {actual:?}"));
    }
    Ok(())
}

fn oracle(a: &[u8], b: &[u8]) -> usize {
    assert!(a.len() < 16);
    (0..1u32 << a.len())
        .filter(|m| {
            let mut k = 0;
            for (i, x) in a.iter().enumerate() {
                if m >> i & 1 == 1 {
                    match b[k..].iter().position(|y| y == x) {
                        Some(p) => k += p + 1,
                        None => return false,
                    }
                }
            }
            true
        })
        .map(|m| m.count_ones() as usize)
        .max()
        .unwrap()
}

/// Histogram (multiset-intersection) upper bound: an independent sufficient statistic.
fn histogram_bound(a: &[u8], b: &[u8]) -> usize {
    let mut ca = [0usize; 256];
    let mut cb = [0usize; 256];
    for &x in a {
        ca[x as usize] += 1;
    }
    for &x in b {
        cb[x as usize] += 1;
    }
    (0..256).map(|i| ca[i].min(cb[i])).sum()
}

fn all_pairs(max_len: usize, sigma: u8) -> Vec<(Vec<u8>, Vec<u8>)> {
    fn seqs(max_len: usize, sigma: u8) -> Vec<Vec<u8>> {
        let mut out = vec![vec![]];
        let mut layer = vec![vec![]];
        for _ in 0..max_len {
            let mut next = vec![];
            for s in &layer {
                for c in 0..sigma {
                    let mut t = s.clone();
                    t.push(c);
                    next.push(t);
                }
            }
            out.extend(next.iter().cloned());
            layer = next;
        }
        out
    }
    let s = seqs(max_len, sigma);
    s.iter()
        .flat_map(|a| s.iter().map(move |b| (a.clone(), b.clone())))
        .collect()
}

#[test]
fn actual_en_algorithm_bitparallel_and_oracle_agree_on_small_byte_domain() {
    let pairs = all_pairs(5, 2);
    assert_eq!(pairs.len(), 63 * 63);
    let mut tight = 0;
    for (a, b) in &pairs {
        let e = en::text::lcs(a, b);
        assert_eq!(e, longest_common_subsequence::length(a, b), "{a:?} {b:?}");
        assert_eq!(e, bit_parallel_lcs::length(a, b).unwrap(), "{a:?} {b:?}");
        assert_eq!(e, oracle(a, b), "{a:?} {b:?}");
        let bound = histogram_bound(a, b);
        assert!(e <= bound && bound <= a.len().min(b.len()));
        tight += usize::from(e == bound);
    }
    assert!(tight < pairs.len(), "bound must not be tight everywhere");
    // Three symbols, lengths through 4: 121*121 pairs.
    for (a, b) in all_pairs(4, 3) {
        let e = en::text::lcs(&a, &b);
        assert_eq!(e, longest_common_subsequence::length(&a, &b));
        assert_eq!(e, oracle(&a, &b));
    }
}

#[test]
fn catalogue_forward_rows_bind_actual_outputs() {
    let names: Vec<&str> = INPUTS.iter().map(|x| x.0).collect();
    let en_out: Vec<usize> = INPUTS.iter().map(|(_, a, b)| en::text::lcs(a, b)).collect();
    let alg_out: Vec<usize> = INPUTS
        .iter()
        .map(|(_, a, b)| longest_common_subsequence::length(a, b))
        .collect();
    let bit_out: Vec<usize> = INPUTS
        .iter()
        .map(|(_, a, b)| bit_parallel_lcs::length(a, b).unwrap())
        .collect();
    observed("lcs.ordinary", &names, &en_out).unwrap();
    observed("lcs.algorithm", &names, &alg_out).unwrap();
    observed("lcs.bitparallel", &names, &bit_out).unwrap();
    for id in ROWS {
        let f = formula(id);
        assert_eq!(f["role"], "forward");
        assert_eq!(f["source"]["sha256"].as_str().unwrap().len(), 64);
    }
    // Negative control: swapping two actual outputs must be rejected.
    let mut wrong = en_out.clone();
    assert_ne!(wrong[0], wrong[2], "control must swap distinct outputs (0 and 1)");
    wrong.swap(0, 2);
    assert!(observed("lcs.ordinary", &names, &wrong).is_err());
    assert!(observed("lcs.ordinary", &names[1..], &en_out[1..]).is_err());
}

#[test]
fn histogram_bound_is_a_partial_excluded_relation_not_the_value() {
    // The refuting pair: equal multisets, different order.
    assert_eq!(histogram_bound(&[1, 2], &[2, 1]), 2);
    assert_eq!(en::text::lcs(&[1u8, 2], &[2, 1]), 1);
    // Independently computed excluded outputs on the catalogue domain: every v > bound.
    let f = formula("lcs.ordinary");
    let codomain: Vec<String> =
        serde_json::from_value(f["contract"]["codomain"]["values"].clone()).unwrap();
    let excluded: Vec<[usize; 2]> = INPUTS
        .iter()
        .enumerate()
        .flat_map(|(i, (_, a, b))| {
            let bound = histogram_bound(a, b);
            codomain
                .iter()
                .enumerate()
                .filter(move |(_, v)| v.parse::<usize>().unwrap() > bound)
                .map(move |(j, _)| [i, j])
        })
        .collect();
    // Bounds 0,0,1,2,3,4,2 exclude 3+3+2+1+0+0+1 = 10 of the 28 rectangle pairs.
    assert_eq!(excluded.len(), 10, "partial exclusion on the 7x4 rectangle");
    for id in ROWS {
        let pairs: Vec<[usize; 2]> =
            serde_json::from_value(formula(id)["expr"]["pairs"].clone()).unwrap();
        assert!(pairs.iter().all(|p| !excluded.contains(p)), "{id} forward meets exclusion");
    }
    // Partial: the exclusion does not cover every non-forward pair ([1,2]/[2,1] -> 2 survives).
    let survivor = [3usize, codomain.iter().position(|v| v == "2").unwrap()];
    assert!(!excluded.contains(&survivor));
}

#[test]
fn unicode_characters_are_not_bytes_and_bitparallel_boundary() {
    let (x, y): (Vec<char>, Vec<char>) = ("éa".chars().collect(), "aé".chars().collect());
    assert_eq!(en::text::lcs(&x, &y), 1);
    assert_eq!(longest_common_subsequence::length(&x, &y), 1);
    assert_eq!(en::text::lcs("éa".as_bytes(), "aé".as_bytes()), 2);
    assert_eq!(bit_parallel_lcs::length("éa".as_bytes(), "aé".as_bytes()).unwrap(), 2);
    let a64: Vec<u8> = (0..64).map(|i| (i % 3) as u8).collect();
    let a65: Vec<u8> = (0..65).map(|i| (i % 3) as u8).collect();
    let b: Vec<u8> = (0..100).map(|i| (i % 5) as u8).collect();
    assert_eq!(bit_parallel_lcs::length(&a64, &b).unwrap(), en::text::lcs(&a64, &b));
    assert!(bit_parallel_lcs::length(&a65, &b).is_err());
    assert_eq!(en::text::lcs(&a65, &b), longest_common_subsequence::length(&a65, &b));
}

/// API obligation beyond `u8: Eq`: the comparison order is a-major, each probe is `x == y` with
/// the left element as receiver, exactly once per cell. An asymmetric, logging `PartialEq`
/// records the actual sequence; the expectation is the baseline row recurrence's order.
#[test]
fn comparison_order_is_a_major_with_left_receiver() {
    use std::cell::RefCell;
    thread_local! { static LOG: RefCell<Vec<(u8, u8, char)>> = const { RefCell::new(Vec::new()) }; }
    #[derive(Clone, Copy)]
    struct Probe {
        id: u8,
        side: char,
    }
    impl PartialEq for Probe {
        fn eq(&self, other: &Self) -> bool {
            LOG.with(|l| l.borrow_mut().push((self.id, other.id, self.side)));
            // Asymmetric on purpose: only a left-side receiver can report equality.
            self.side == 'a' && self.id == other.id
        }
    }
    let a: Vec<Probe> = [3u8, 1, 2].iter().map(|&id| Probe { id, side: 'a' }).collect();
    let b: Vec<Probe> = [1u8, 2, 3, 1].iter().map(|&id| Probe { id, side: 'b' }).collect();
    LOG.with(|l| l.borrow_mut().clear());
    assert_eq!(en::text::lcs(&a, &b), 2);
    let expected: Vec<(u8, u8, char)> = a
        .iter()
        .flat_map(|x| b.iter().map(move |y| (x.id, y.id, 'a')))
        .collect();
    let got = LOG.with(|l| l.borrow().clone());
    assert_eq!(got.len(), a.len() * b.len());
    assert_eq!(got, expected, "a-major order with the left element as receiver");
    // Control: the swapped call records b-side receivers and yields 0 under the asymmetric Eq.
    LOG.with(|l| l.borrow_mut().clear());
    assert_eq!(en::text::lcs(&b, &a), 0);
    assert!(LOG.with(|l| l.borrow().iter().all(|r| r.2 == 'b')));
}

/// Scanner integration: the three rows share one finite observation contract, so the library's
/// core compare reports finite equality between the ordinary source and both algorithm sources.
/// Implementation-specific schedules, costs and domains stay in the source groupants.
#[test]
fn common_contract_makes_ordinary_and_algorithm_rows_finite_equal() {
    use mova_correspondence::{compare, Formula};
    let rows: Vec<Formula> = ROWS
        .iter()
        .map(|id| serde_json::from_value(formula(id)).unwrap())
        .collect();
    assert!(rows.iter().all(|r| r.contract == rows[0].contract), "contracts must be identical");
    for right in &rows[1..] {
        let c = compare(&rows[0], right).unwrap();
        assert!(c.finite_equal, "{} vs {}: {}", c.left, c.right, c.status);
        assert_eq!(c.status, "finite_coincidence", "distinct atom symbols, no structural match");
        assert_eq!(c.counterexample, None);
    }
    // Control: a row with a different assumption list is an incompatible contract, not a match.
    let mut other = rows[1].clone();
    other.contract.assumptions.push("Row schedule specific to this implementation.".into());
    let c = compare(&rows[0], &other).unwrap();
    assert_eq!(c.status, "incompatible_contract");
    assert!(!c.finite_equal);
}
