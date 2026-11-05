use super::*;
#[test]
fn enumerated_mass_oracle() {
    for code in 1..256usize {
        let weights: Vec<f64> = (0..4).map(|i| ((code >> (2 * i)) & 3) as f64).collect();
        let total: f64 = weights.iter().sum();
        if total == 0.0 {
            continue;
        }
        let table = AliasTable::new(&weights).unwrap();
        let mut observed = [0.0; 4];
        for i in 0..4 {
            observed[i] += table.thresholds[i] / 4.0;
            observed[table.aliases[i]] += (1.0 - table.thresholds[i]) / 4.0;
        }
        for i in 0..4 {
            assert!(
                (observed[i] - weights[i] / total).abs() < 1e-14,
                "mass oracle"
            );
        }
    }
}
#[test]
fn explicit_draws_and_boundaries() {
    let table = AliasTable::new(&[0.0, 1.0, 3.0]).unwrap();
    for column in 0..3 {
        for k in 0..100 {
            assert_ne!(table.draw(column, k as f64 / 100.0).unwrap(), 0);
        }
    }
    assert_eq!(table.draw(3, 0.0), Err(Error::Coordinate));
    for x in [-1.0, 1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(table.draw(0, x), Err(Error::Coordinate));
    }
    let one = AliasTable::new(&[f64::MAX]).unwrap();
    assert_eq!(one.draw(0, 0.999).unwrap(), 0);
    assert_eq!(one.len(), 1);
    assert!(!one.is_empty());
    let t = AliasTable::new(&[1.0, 3.0]).unwrap();
    assert_eq!(t.draw(0, 0.499).unwrap(), 0);
    assert_eq!(t.draw(0, 0.5).unwrap(), 1);
}
#[test]
fn validation_and_extremes() {
    assert_eq!(AliasTable::new(&[]), Err(Error::Dimension));
    assert_eq!(AliasTable::new(&vec![1.0; 4097]), Err(Error::Dimension));
    assert_eq!(AliasTable::new(&[0.0, 0.0]), Err(Error::EmptyMass));
    for w in [-1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(AliasTable::new(&[w]), Err(Error::Weight));
    }
    let t = AliasTable::new(&[f64::MAX, f64::MAX]).unwrap();
    assert_eq!(t.draw(0, 0.99), Ok(0));
    assert_eq!(t.draw(1, 0.99), Ok(1));
    assert!(AliasTable::new(&vec![1.0; 4096]).is_ok());
}
