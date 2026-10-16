//! `world induce` on the GPU (MOVA_EXEC=gpu, `--features gpu`; docs/gpu-exec.md): the break counting — for every
//! word labelled right, every 1–3 subset of its atoms looked up in the candidate groups — as one exact integer
//! kernel (open-addressing table, atomic adds of ones). The counts are the CPU's, number for number.

use std::sync::{Arc, OnceLock};

use candle_core::cuda_backend::cudarc::driver::{CudaContext, CudaFunction, CudaStream, LaunchConfig, PushKernelArg};
use candle_core::cuda_backend::cudarc::nvrtc::compile_ptx;

const SRC: &str = r#"
__device__ __forceinline__ unsigned int slot(unsigned long long key, unsigned int rel, unsigned int mask) {
    unsigned long long h = (key ^ ((unsigned long long)rel * 0x9E3779B97F4A7C15ULL)) * 0xBF58476D1CE4E5B9ULL;
    return (unsigned int)(h >> 32) & mask;
}

__device__ __forceinline__ void probe(unsigned long long key, unsigned int rel, const unsigned long long* tkey,
    const unsigned int* trel, const unsigned int* tval, unsigned int mask, unsigned int* cnt) {
    unsigned int h = slot(key, rel, mask);
    while (trel[h] != 0) {
        if (trel[h] == rel + 1 && tkey[h] == key) {
            atomicAdd(&cnt[tval[h]], 1u);
            return;
        }
        h = (h + 1) & mask;
    }
}

extern "C" __global__ void breaks(const unsigned long long* codes, const unsigned int* off, const unsigned int* rel,
    unsigned int nwords, const unsigned long long* tkey, const unsigned int* trel, const unsigned int* tval,
    unsigned int mask, unsigned int* cnt) {
    unsigned int w = blockIdx.x * blockDim.x + threadIdx.x;
    if (w >= nwords) return;
    unsigned int a = off[w], b = off[w + 1], r = rel[w];
    for (unsigned int i = a; i < b; i++) {
        unsigned long long ci = codes[i];
        probe(ci, r, tkey, trel, tval, mask, cnt);
        for (unsigned int j = i + 1; j < b; j++) {
            unsigned long long cij = ci | (codes[j] << 21);
            probe(cij, r, tkey, trel, tval, mask, cnt);
            for (unsigned int k = j + 1; k < b; k++) {
                probe(cij | (codes[k] << 42), r, tkey, trel, tval, mask, cnt);
            }
        }
    }
}
"#;

struct Gpu {
    stream: Arc<CudaStream>,
    f: CudaFunction,
}

fn gpu() -> &'static Gpu {
    static G: OnceLock<Gpu> = OnceLock::new();
    G.get_or_init(|| {
        let ctx = CudaContext::new(0).unwrap_or_else(|e| fail(&format!("no CUDA device: {e}")));
        let stream = ctx.default_stream();
        let ptx = compile_ptx(SRC).unwrap_or_else(|e| fail(&format!("kernel: {e}")));
        let m = ctx.load_module(ptx).unwrap_or_else(|e| fail(&format!("module: {e}")));
        let f = m.load_function("breaks").unwrap_or_else(|e| fail(&format!("function: {e}")));
        Gpu { stream, f }
    })
}

fn fail(msg: &str) -> ! {
    eprintln!("MOVA_EXEC=gpu: induce: {msg}");
    std::process::exit(2);
}

/// The table slot of (key, label) — the kernel's `slot`.
fn slot(key: u64, rel: u32, mask: u32) -> u32 {
    let h = (key ^ (rel as u64).wrapping_mul(0x9E3779B97F4A7C15)).wrapping_mul(0xBF58476D1CE4E5B9);
    ((h >> 32) as u32) & mask
}

/// Break counts per group. `codes`/`off`/`rel`: the words labelled right (their sorted atom codes, offsets, label);
/// `groups`: (label, key) → group number.
pub fn break_counts(codes: &[u64], off: &[u32], rel: &[u32], groups: &[((u32, u64), u32)]) -> Vec<u32> {
    let n = groups.len();
    if n == 0 || rel.is_empty() {
        return vec![0; n];
    }
    let size = (2 * n).next_power_of_two().max(1024);
    let mask = (size - 1) as u32;
    let (mut tkey, mut trel, mut tval) = (vec![0u64; size], vec![0u32; size], vec![0u32; size]);
    for &((r, k), g) in groups {
        let mut h = slot(k, r, mask) as usize;
        while trel[h] != 0 {
            h = (h + 1) & mask as usize;
        }
        tkey[h] = k;
        trel[h] = r + 1;
        tval[h] = g;
    }
    let g = gpu();
    let s = &g.stream;
    let run = || -> Result<Vec<u32>, candle_core::cuda_backend::cudarc::driver::DriverError> {
        let d_codes = s.clone_htod(codes)?;
        let d_off = s.clone_htod(off)?;
        let d_rel = s.clone_htod(rel)?;
        let d_tkey = s.clone_htod(&tkey)?;
        let d_trel = s.clone_htod(&trel)?;
        let d_tval = s.clone_htod(&tval)?;
        let mut d_cnt = s.alloc_zeros::<u32>(n)?;
        let nw = rel.len() as u32;
        let cfg = LaunchConfig { grid_dim: (nw.div_ceil(256), 1, 1), block_dim: (256, 1, 1), shared_mem_bytes: 0 };
        let mut b = s.launch_builder(&g.f);
        b.arg(&d_codes).arg(&d_off).arg(&d_rel).arg(&nw).arg(&d_tkey).arg(&d_trel).arg(&d_tval).arg(&mask).arg(&mut d_cnt);
        unsafe { b.launch(cfg) }?;
        s.clone_dtoh(&d_cnt)
    };
    run().unwrap_or_else(|e| fail(&format!("kernel run: {e}")))
}
