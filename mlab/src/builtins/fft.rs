//! Spectrum: fft, ifft via rustfft. A vector — along its dimension, a matrix — by columns;
//! `fft(x, n)` pads with zeros or truncates. A result with zero imaginary part is narrowed to real.

use super::*;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

pub fn register(r: &mut Registry) {
    r.register("fft", |_, a, _| transform(a, "fft", false));
    r.register("ifft", |_, a, _| transform(a, "ifft", true));
}

fn transform(a: &[Value], name: &str, inverse: bool) -> Result<Vec<Value>, MError> {
    let m = mat_arg(a, 0, name)?;
    let n_arg = match a.get(1) {
        Some(Value::Mat(x)) if !x.is_empty() => {
            let n = x.re[0];
            if n < 1.0 || n != n.trunc() {
                return Err(MError::new(format!("{name}: number of points N must be greater than zero")));
            }
            Some(n as usize)
        }
        _ => None,
    };
    let dim = dim_arg(a, 2, name)?.unwrap_or(if m.rows == 1 { 2 } else { 1 });
    if m.is_empty() {
        return one(m.clone());
    }
    let im = m.im_or_zeros();
    // slices along dim
    let (len, count) = if dim == 1 { (m.rows, m.cols) } else { (m.cols, m.rows) };
    let n = n_arg.unwrap_or(len);
    let mut planner = FftPlanner::<f64>::new();
    let plan = if inverse { planner.plan_fft_inverse(n) } else { planner.plan_fft_forward(n) };
    let (out_r, out_c) = if dim == 1 { (n, m.cols) } else { (m.rows, n) };
    let mut re = vec![0.0; out_r * out_c];
    let mut imv = vec![0.0; out_r * out_c];
    let scale = if inverse { 1.0 / n as f64 } else { 1.0 };
    for s in 0..count {
        let mut buf: Vec<Complex<f64>> = (0..n)
            .map(|t| {
                if t < len {
                    let k = if dim == 1 { s * m.rows + t } else { t * m.rows + s };
                    Complex::new(m.re[k], im[k])
                } else {
                    Complex::new(0.0, 0.0)
                }
            })
            .collect();
        plan.process(&mut buf);
        for (t, z) in buf.iter().enumerate() {
            let k = if dim == 1 { s * out_r + t } else { t * out_r + s };
            re[k] = z.re * scale;
            imv[k] = z.im * scale;
        }
    }
    one(Mat::complex(out_r, out_c, re, imv))
}
