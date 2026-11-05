use super::*;

fn verify(input: &[i32], rank: usize) {
    let mut values = input.to_vec();
    let mut expected = input.to_vec();
    expected.sort();
    assert_eq!(
        select_nth(&mut values, rank).copied(),
        expected.get(rank).copied(),
        "input={input:?}, rank={rank}"
    );
    if rank < input.len() {
        assert!(
            values[..rank].iter().all(|v| *v <= values[rank]),
            "prefix input={input:?}, rank={rank}"
        );
        assert!(
            values[rank + 1..].iter().all(|v| *v >= values[rank]),
            "suffix input={input:?}, rank={rank}"
        );
        values.sort();
        assert_eq!(values, expected, "multiset input={input:?}, rank={rank}");
    } else {
        assert_eq!(values, input, "invalid changed input");
    }
}

#[test]
fn exhaustive_all_ranks_and_partitions() {
    for n in 0..=7u32 {
        for mut code in 0..3usize.pow(n) {
            let input: Vec<_> = (0..n)
                .map(|_| {
                    let v = (code % 3) as i32 - 1;
                    code /= 3;
                    v
                })
                .collect();
            for rank in 0..=input.len() {
                verify(&input, rank);
            }
        }
    }
}

#[test]
fn invalid_indices_preserve_exact_input() {
    for input in [vec![], vec![9], vec![3, 1, 2, 1]] {
        verify(&input, input.len());
        verify(&input, usize::MAX);
    }
}

#[test]
fn full_width_and_larger_adversarial_inputs() {
    let mut seed = 0xd1b54a32d192ed03u64;
    let mut input = vec![i32::MIN, i32::MAX, 0, -1];
    for _ in 0..512 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        input.push((seed >> 32) as i32);
    }
    for rank in [0, 1, 128, 258, 515] {
        verify(&input, rank);
    }
    for input in [(0..256).collect(), (0..256).rev().collect(), vec![7; 256]] {
        for rank in [0, 127, 255] {
            verify(&input, rank);
        }
    }
}

#[test]
fn reverse_order_without_clone() {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Value(i32);
    let mut values = [3, -1, 3, i32::MIN, i32::MAX].map(|n| std::cmp::Reverse(Value(n)));
    assert_eq!(select_nth(&mut values, 2).map(|v| v.0.0), Some(3));
    assert!(values[..2].iter().all(|v| v <= &values[2]));
    assert!(values[3..].iter().all(|v| v >= &values[2]));
}
