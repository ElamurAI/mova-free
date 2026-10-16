//! Eisner (1996) decoding: the best projective tree with exactly one word under the root.
//! `sc[h][d]` — score of head h for dependent d, 0 = root, words 1..=n.

pub fn decode(sc: &[Vec<f32>]) -> Vec<usize> {
    let n = sc.len() - 1;
    if n == 1 {
        return vec![0];
    }
    let neg = f32::NEG_INFINITY;
    let idx = |s: usize, t: usize| s * (n + 1) + t;
    // complete / incomplete spans over words 1..=n; [0] head on the right, [1] head on the left
    let mut c = [vec![neg; (n + 1) * (n + 1)], vec![neg; (n + 1) * (n + 1)]];
    let mut i = [vec![neg; (n + 1) * (n + 1)], vec![neg; (n + 1) * (n + 1)]];
    let mut cb = [vec![0usize; (n + 1) * (n + 1)], vec![0usize; (n + 1) * (n + 1)]];
    let mut ib = [vec![0usize; (n + 1) * (n + 1)], vec![0usize; (n + 1) * (n + 1)]];
    for s in 1..=n {
        c[0][idx(s, s)] = 0.;
        c[1][idx(s, s)] = 0.;
    }
    for k in 1..n {
        for s in 1..=n - k {
            let t = s + k;
            let (mut best, mut arg) = (neg, s);
            for r in s..t {
                let v = c[1][idx(s, r)] + c[0][idx(r + 1, t)];
                if v > best {
                    best = v;
                    arg = r;
                }
            }
            i[0][idx(s, t)] = best + sc[t][s];
            ib[0][idx(s, t)] = arg;
            i[1][idx(s, t)] = best + sc[s][t];
            ib[1][idx(s, t)] = arg;
            let (mut b0, mut a0) = (neg, s);
            for r in s..t {
                let v = c[0][idx(s, r)] + i[0][idx(r, t)];
                if v > b0 {
                    b0 = v;
                    a0 = r;
                }
            }
            c[0][idx(s, t)] = b0;
            cb[0][idx(s, t)] = a0;
            let (mut b1, mut a1) = (neg, t);
            for r in s + 1..=t {
                let v = i[1][idx(s, r)] + c[1][idx(r, t)];
                if v > b1 {
                    b1 = v;
                    a1 = r;
                }
            }
            c[1][idx(s, t)] = b1;
            cb[1][idx(s, t)] = a1;
        }
    }
    let (mut best, mut root) = (neg, 1);
    for r in 1..=n {
        let v = sc[0][r] + c[0][idx(1, r)] + c[1][idx(r, n)];
        if v > best {
            best = v;
            root = r;
        }
    }
    let mut heads = vec![0usize; n + 1];
    heads[root] = 0;
    // explicit stack: (kind, dir, s, t) — kind 0 complete, 1 incomplete
    let mut st = vec![(0u8, 0usize, 1usize, root), (0u8, 1usize, root, n)];
    while let Some((kind, d, s, t)) = st.pop() {
        if s == t {
            continue;
        }
        if kind == 0 {
            let r = cb[d][idx(s, t)];
            if d == 0 {
                st.push((0, 0, s, r));
                st.push((1, 0, r, t));
            } else {
                st.push((1, 1, s, r));
                st.push((0, 1, r, t));
            }
        } else {
            let r = ib[d][idx(s, t)];
            if d == 0 {
                heads[s] = t;
            } else {
                heads[t] = s;
            }
            st.push((0, 1, s, r));
            st.push((0, 0, r + 1, t));
        }
    }
    heads[1..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_tree_with_one_root() {
        // "dogs bark loudly": bark is the root, dogs and loudly depend on it
        let mut sc = vec![vec![-5f32; 4]; 4];
        sc[0][2] = 0.;
        sc[2][1] = 0.;
        sc[2][3] = 0.;
        assert_eq!(decode(&sc), vec![2, 0, 2]);
        // negative control: two words both love the root — only one gets it
        sc[0][1] = 0.;
        sc[0][3] = 0.;
        let h = decode(&sc);
        assert_eq!(h.iter().filter(|&&x| x == 0).count(), 1);
        // projective: no crossing arcs
        let n = 6;
        let mut sc = vec![vec![0f32; n + 1]; n + 1];
        for (hh, d) in [(0, 3), (3, 1), (1, 5), (3, 2), (5, 4), (3, 6)] {
            sc[hh][d] = 10.;
        }
        let h = decode(&sc);
        assert_eq!(h.iter().filter(|&&x| x == 0).count(), 1);
    }
}
