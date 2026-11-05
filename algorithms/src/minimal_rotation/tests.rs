use super::{advance, cyclic_offset, minimal_rotation};

fn naive(bytes: &[u8]) -> usize {
    let mut rotations: Vec<(Vec<u8>, usize)> = (0..bytes.len())
        .map(|start| {
            let rotation = bytes[start..]
                .iter()
                .chain(&bytes[..start])
                .copied()
                .collect();
            (rotation, start)
        })
        .collect();
    rotations.sort();
    rotations.first().map_or(0, |entry| entry.1)
}

#[test]
fn exhaustive_all_rotations() {
    let mut level = vec![vec![]];
    for _ in 0..=8 {
        for word in &level {
            assert_eq!(minimal_rotation(word), naive(word), "{word:?}");
        }
        let mut next = Vec::new();
        for word in level {
            for byte in [0, 128, 255] {
                let mut extended = word.clone();
                extended.push(byte);
                next.push(extended);
            }
        }
        level = next;
    }
}

#[test]
fn periodic_and_byte_boundaries() {
    for (bytes, expected) in [
        (&b""[..], 0),
        (&b"x"[..], 0),
        (&b"aaaa"[..], 0),
        (&b"baba"[..], 1),
        (&b"cabcab"[..], 1),
        (&b"baaa"[..], 1),
        (&b"alfa"[..], 3),
        (&b"0010"[..], 3),
        (&[255, 128, 0, 255, 128, 0][..], 2),
    ] {
        assert_eq!(minimal_rotation(bytes), expected);
    }
    let mut long = vec![255; 4096];
    long[4095] = 0;
    assert_eq!(minimal_rotation(&long), 4095);
}

#[test]
fn arithmetic_without_doubled_size() {
    let n = usize::MAX;
    assert_eq!(cyclic_offset(n - 2, 3, n), 1);
    assert_eq!(cyclic_offset(n - 2, 1, n), n - 1);
    assert_eq!(advance(n - 2, 3, n), n);
    assert_eq!(advance(1, 2, n), 4);
}

#[test]
fn oracle_control_and_fibers() {
    assert_ne!(naive(b"baba"), 3);
    assert_eq!(minimal_rotation(b"abc"), minimal_rotation(b"xyz"));
    assert_ne!(minimal_rotation(b"baba"), 3);
}
