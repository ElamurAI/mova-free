//! Cross-kernel numerical contracts, independently anchored by norms, DFT sums and polynomials.
use mova_algorithms::{
    bluestein_fft, discrete_cosine_transform as dct, fast_fourier_transform::Complex, goertzel,
    haar_wavelet as haar, householder_qr, savitzky_golay,
};
fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 2e-8 * (1.0 + expected.abs()),
        "{actual} != {expected}"
    );
}
fn energy(x: &[f64]) -> f64 {
    x.iter().map(|v| v * v).sum()
}

#[test]
fn orthogonal_composition_parseval_and_roundtrip() {
    for n in [1, 2, 4, 8, 16, 64, 128] {
        for fixture in 0..3 {
            let x: Vec<_> = (0..n)
                .map(|j| match fixture {
                    0 => 0.0,
                    1 => {
                        if j == n / 2 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    _ => ((j * 17) % 29) as f64 - 14.0,
                })
                .collect();
            let h = haar::transform(&x, false).unwrap();
            let d = dct::transform(&x, false).unwrap();
            close(energy(&h), energy(&x));
            close(energy(&d), energy(&x));
            let composed = dct::transform(&h, false).unwrap();
            close(energy(&composed), energy(&x));
            let restored =
                haar::transform(&dct::transform(&composed, true).unwrap(), true).unwrap();
            for (&actual, &expected) in restored.iter().zip(&x) {
                close(actual, expected);
            }
            let d_back = dct::transform(&d, true).unwrap();
            for (&actual, &expected) in d_back.iter().zip(&x) {
                close(actual, expected);
            }
        }
    }
    assert_eq!(haar::transform(&[], false), Err(haar::Error::InvalidInput));
    assert_eq!(dct::transform(&[], false), Ok(vec![]));
    assert_eq!(
        haar::transform(&[0.0; 3], false),
        Err(haar::Error::InvalidInput)
    );
    assert_eq!(
        dct::transform(&[f64::NAN], true),
        Err(dct::Error::InvalidInput)
    );
}

#[test]
fn selected_spectrum_matches_bluestein_and_direct_dft() {
    for n in [1, 2, 3, 5, 7, 11, 16, 31] {
        let x: Vec<_> = (0..n).map(|j| ((j * 7 + 3) % 17) as f64 - 8.0).collect();
        let complex: Vec<_> = x.iter().map(|&re| Complex { re, im: 0.0 }).collect();
        let full = bluestein_fft::transform(&complex, false).unwrap();
        let indices = [n - 1, 0, n / 2, n - 1];
        let selected = goertzel::bins(&x, &indices).unwrap();
        for (&k, z) in indices.iter().zip(selected) {
            let mut direct = Complex { re: 0.0, im: 0.0 };
            for (j, &sample) in x.iter().enumerate() {
                let angle = -2.0 * std::f64::consts::PI * (j * k) as f64 / n as f64;
                direct.re += sample * angle.cos();
                direct.im += sample * angle.sin();
            }
            close(z.re, direct.re);
            close(z.im, direct.im);
            close(full[k].re, direct.re);
            close(full[k].im, direct.im);
            close(z.re, full[k].re);
            close(z.im, full[k].im);
        }
    }
    assert_eq!(goertzel::bins(&[], &[]), Ok(vec![]));
    assert_eq!(
        bluestein_fft::transform(&[], false),
        Err(bluestein_fft::Error::InvalidInput)
    );
    assert_eq!(
        goertzel::bins(&[1.0], &[1]),
        Err(goertzel::Error::InvalidInput)
    );
    assert_eq!(
        bluestein_fft::transform(
            &[Complex {
                re: 0.0,
                im: f64::INFINITY
            }],
            false
        ),
        Err(bluestein_fft::Error::InvalidInput)
    );
}

#[test]
fn smoothing_matches_polynomials_and_unscaled_qr() {
    for degree in 0..=4 {
        let n = 17;
        let polynomial = |i: usize| {
            let t = (i as f64 - 8.0) / 8.0;
            (0..=degree)
                .map(|k| (k + 1) as f64 * t.powi(k as i32))
                .sum::<f64>()
        };
        let x: Vec<_> = (0..n).map(polynomial).collect();
        let filtered = savitzky_golay::smooth(&x, 7, degree).unwrap();
        for (i, &actual) in filtered.iter().enumerate() {
            close(actual, polynomial(i));
            // Independent coordinates and boundary selection; no production scaling helper.
            let left = if i < 3 {
                0
            } else if i + 3 >= n {
                n - 7
            } else {
                i - 3
            };
            let design: Vec<Vec<_>> = (left..left + 7)
                .map(|j| {
                    let t = j as f64 - i as f64;
                    (0..=degree).map(|k| t.powi(k as i32)).collect()
                })
                .collect();
            let fit = householder_qr::least_squares(&design, &x[left..left + 7]).unwrap();
            close(fit.x[0], polynomial(i));
            close(actual, fit.x[0]);
        }
    }
    assert_eq!(
        savitzky_golay::smooth(&[], 1, 0),
        Err(savitzky_golay::Error::InvalidInput)
    );
    assert_eq!(
        savitzky_golay::smooth(&[1.0; 7], 4, 2),
        Err(savitzky_golay::Error::InvalidInput)
    );
    assert_eq!(
        savitzky_golay::smooth(&[f64::NAN], 1, 0),
        Err(savitzky_golay::Error::InvalidInput)
    );
    assert_eq!(
        householder_qr::least_squares(&[vec![1.0, 1.0]], &[1.0]),
        Err(householder_qr::Error::InvalidInput)
    );
}
