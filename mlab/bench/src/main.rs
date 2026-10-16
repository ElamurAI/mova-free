//! Comparison of linear algebra backends for mlab: faer vs nalgebra.
//! Single thread (faer — Par::Seq), deterministic matrices (LCG), best of 3 runs.
//! Tasks: 1000×1000 multiplication, solving A x = b 1000×1000 (LU with partial pivoting),
//! 500×500 eig — nonsymmetric (eigenvalues; vectors too in faer) and symmetric (values and vectors).
//! Accuracy: residuals ‖A·x − b‖, ‖A·V − V·D‖ — so that speed is not bought with errors.

use std::time::Instant;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64) - 0.5
    }
}

fn best<F: FnMut() -> f64>(mut f: F) -> (f64, f64) {
    let mut t_best = f64::INFINITY;
    let mut acc = 0.0;
    for _ in 0..3 {
        let t0 = Instant::now();
        acc = f();
        let t = t0.elapsed().as_secs_f64();
        if t < t_best {
            t_best = t;
        }
    }
    (t_best, acc)
}

fn data(n: usize, seed: u64) -> Vec<f64> {
    let mut g = Lcg(seed);
    (0..n * n).map(|_| g.next()).collect()
}

fn main() {
    use faer::prelude::*;
    faer::set_global_parallelism(faer::Par::Seq);
    let n = 1000;
    let a = data(n, 1);
    let b = data(n, 2);
    let rhs: Vec<f64> = (0..n).map(|i| (i as f64).sin()).collect();

    // ---------- faer ----------
    let fa = faer::Mat::<f64>::from_fn(n, n, |i, j| a[j * n + i]);
    let fb = faer::Mat::<f64>::from_fn(n, n, |i, j| b[j * n + i]);
    let fr = faer::Mat::<f64>::from_fn(n, 1, |i, _| rhs[i]);
    let (t_mul_f, s_mul_f) = best(|| {
        let c = &fa * &fb;
        c[(0, 0)] + c[(n - 1, n - 1)]
    });
    let (t_sol_f, res_sol_f) = best(|| {
        let lu = fa.partial_piv_lu();
        let x = lu.solve(&fr);
        let r = &fa * &x - &fr;
        r.norm_l2()
    });
    let m = 500;
    let e = data(m, 3);
    let fe = faer::Mat::<f64>::from_fn(m, m, |i, j| e[j * m + i]);
    let fs = faer::Mat::<f64>::from_fn(m, m, |i, j| e[j * m + i] + e[i * m + j]);
    let (t_eig_f, res_eig_f) = best(|| {
        let ev = fe.eigen().unwrap();
        let u = ev.U();
        let s = ev.S();
        // ‖A·V − V·D‖ in complex
        let ac = faer::Mat::<faer::c64>::from_fn(m, m, |i, j| faer::c64::new(fe[(i, j)], 0.0));
        let av = &ac * u;
        let mut worst: f64 = 0.0;
        for j in 0..m {
            for i in 0..m {
                let d = av[(i, j)] - u[(i, j)] * s[j];
                worst = worst.max((d.re * d.re + d.im * d.im).sqrt());
            }
        }
        worst
    });
    let (t_eigvals_f, _) = best(|| {
        let v = fe.eigenvalues().unwrap();
        v[0].re
    });
    let (t_seig_f, res_seig_f) = best(|| {
        let ev = fs.self_adjoint_eigen(faer::Side::Lower).unwrap();
        let u = ev.U();
        let s = ev.S();
        let av = &fs * u;
        let mut worst: f64 = 0.0;
        for j in 0..m {
            for i in 0..m {
                worst = worst.max((av[(i, j)] - u[(i, j)] * s[j]).abs());
            }
        }
        worst
    });

    // ---------- nalgebra ----------
    let na = nalgebra::DMatrix::<f64>::from_column_slice(n, n, &a);
    let nb = nalgebra::DMatrix::<f64>::from_column_slice(n, n, &b);
    let nr = nalgebra::DVector::<f64>::from_column_slice(&rhs);
    let (t_mul_n, s_mul_n) = best(|| {
        let c = &na * &nb;
        c[(0, 0)] + c[(n - 1, n - 1)]
    });
    let (t_sol_n, res_sol_n) = best(|| {
        let lu = na.clone().lu();
        let x = lu.solve(&nr).unwrap();
        (&na * &x - &nr).norm()
    });
    let ne = nalgebra::DMatrix::<f64>::from_column_slice(m, m, &e);
    let ns = nalgebra::DMatrix::<f64>::from_fn(m, m, |i, j| e[j * m + i] + e[i * m + j]);
    let (t_eigvals_n, _) = best(|| {
        let v = ne.complex_eigenvalues();
        v[0].re
    });
    let (t_seig_n, res_seig_n) = best(|| {
        let ev = ns.clone().symmetric_eigen();
        let av = &ns * &ev.eigenvectors;
        let mut worst: f64 = 0.0;
        for j in 0..m {
            for i in 0..m {
                worst = worst.max((av[(i, j)] - ev.eigenvectors[(i, j)] * ev.eigenvalues[j]).abs());
            }
        }
        worst
    });

    println!("task\tfaer, s\tnalgebra, s\tfaer/nalgebra");
    println!("multiply 1000×1000\t{t_mul_f:.3}\t{t_mul_n:.3}\t{:.2}", t_mul_f / t_mul_n);
    println!("solve 1000×1000 (LU)\t{t_sol_f:.3}\t{t_sol_n:.3}\t{:.2}", t_sol_f / t_sol_n);
    println!("eig 500×500 nonsym., values only\t{t_eigvals_f:.3}\t{t_eigvals_n:.3}\t{:.2}", t_eigvals_f / t_eigvals_n);
    println!("eig 500×500 nonsym., values and vectors\t{t_eig_f:.3}\t—\tnot supported by nalgebra");
    println!("eig 500×500 sym., values and vectors\t{t_seig_f:.3}\t{t_seig_n:.3}\t{:.2}", t_seig_f / t_seig_n);
    println!();
    println!("check: sum of product corners faer {s_mul_f:.12} nalgebra {s_mul_n:.12}");
    println!("residual ‖Ax−b‖: faer {res_sol_f:.2e} nalgebra {res_sol_n:.2e}");
    println!("max|AV−VD| nonsym.: faer {res_eig_f:.2e}");
    println!("max|AV−VD| sym.: faer {res_seig_f:.2e} nalgebra {res_seig_n:.2e}");
}
