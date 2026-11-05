use super::*;

#[test]
fn exhaustive_bucket_oracle() {
    for n in 0..=7u32 {
        for mut code in 0..3usize.pow(n) {
            let mut values: Vec<_> = (0..n)
                .map(|_| {
                    let v = code % 3;
                    code /= 3;
                    v
                })
                .collect();
            let input = values.clone();
            let mut counts = [0; 3];
            for &v in &input {
                counts[v] += 1;
            }
            let expected: Vec<_> = counts
                .iter()
                .enumerate()
                .flat_map(|(v, &count)| std::iter::repeat_n(v, count))
                .collect();
            heap_sort(&mut values);
            assert_eq!(values, expected, "input={input:?}");
        }
    }
}

#[test]
fn full_width_and_fixed_larger_inputs() {
    let mut seed = 0x9e3779b97f4a7c15u64;
    let mut data = vec![0, u64::MAX, 1, u64::MAX - 1, 1 << 63];
    for _ in 0..2048 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        data.push(seed);
    }
    let mut expected = data.clone();
    expected.sort();
    heap_sort(&mut data);
    assert_eq!(data, expected);
    data.reverse();
    heap_sort(&mut data);
    assert_eq!(data, expected);
    heap_sort(&mut data);
    assert_eq!(data, expected);
}

#[test]
fn custom_reverse_order_without_clone() {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Value(i32);
    let mut values = [3, -1, 3, i32::MIN, i32::MAX].map(|n| std::cmp::Reverse(Value(n)));
    heap_sort(&mut values);
    assert_eq!(values.map(|v| v.0.0), [i32::MAX, 3, 3, -1, i32::MIN]);
}

#[test]
fn empty_singleton_and_equal_values() {
    heap_sort::<i32>(&mut []);
    let mut one = [42];
    heap_sort(&mut one);
    assert_eq!(one, [42]);
    let mut equal = [7; 1024];
    heap_sort(&mut equal);
    assert_eq!(equal, [7; 1024]);
}
