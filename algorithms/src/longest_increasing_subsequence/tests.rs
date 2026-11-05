use super::*;

fn brute_length(values: &[i32]) -> usize {
    (0..1usize << values.len())
        .filter_map(|mask| {
            let picked: Vec<_> = values
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| ((mask >> i) & 1 != 0).then_some(v))
                .collect();
            picked
                .windows(2)
                .all(|w| w[0] < w[1])
                .then_some(picked.len())
        })
        .max()
        .unwrap()
}

fn valid<T: Ord>(values: &[T], result: &[usize]) {
    assert!(result.iter().all(|&i| i < values.len()));
    assert!(
        result
            .windows(2)
            .all(|w| w[0] < w[1] && values[w[0]] < values[w[1]])
    );
}

#[test]
fn exhaustive_subsequence_oracle() {
    for n in 0..=7u32 {
        for mut code in 0..3usize.pow(n) {
            let values: Vec<_> = (0..n)
                .map(|_| {
                    let v = (code % 3) as i32 - 1;
                    code /= 3;
                    v
                })
                .collect();
            let result = indices(&values);
            valid(&values, &result);
            assert_eq!(
                result.len(),
                brute_length(&values),
                "values={values:?}, result={result:?}"
            );
        }
    }
}

#[test]
fn deterministic_equal_tail_policy() {
    assert_eq!(indices::<i32>(&[]), vec![]);
    assert_eq!(indices(&[7]), vec![0]);
    assert_eq!(indices(&[2, 2, 2]), vec![2]);
    assert_eq!(indices(&[1, 2, 2, 3]), vec![0, 2, 3]);
    assert_eq!(indices(&[3, 1, 2, 1, 2, 3]), vec![3, 4, 5]);
    assert_eq!(indices(&[3, 2, 1]), vec![2]);
}

#[test]
fn larger_fixed_inputs_match_quadratic_oracle() {
    let mut seed = 0x94d049bb133111ebu64;
    let mut values = vec![i64::MIN, i64::MAX, 0, -1];
    for _ in 0..1024 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        values.push(seed as i64);
    }
    let mut lengths = vec![1; values.len()];
    for i in 0..values.len() {
        for j in 0..i {
            if values[j] < values[i] {
                lengths[i] = lengths[i].max(lengths[j] + 1);
            }
        }
    }
    let result = indices(&values);
    valid(&values, &result);
    assert_eq!(result.len(), *lengths.iter().max().unwrap());
    assert_eq!(indices(&values), result);
}

#[test]
fn reverse_order_without_clone() {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Value(i32);
    let values = [3, 1, 2, 0].map(|n| std::cmp::Reverse(Value(n)));
    let result = indices(&values);
    valid(&values, &result);
    assert_eq!(result, vec![0, 2, 3]);
}
