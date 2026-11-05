//! Fused CUDA F32 softmax with a differentiable vector-Jacobian product.
//!
//! Forward uses Candle 0.9.2's existing row kernel, without copying its implementation:
//! https://github.com/huggingface/candle/blob/0.9.2/candle-nn/src/ops.rs
//! Candle's public softmax_last_dim explicitly disables backward; this wrapper supplies it.
//! For p = softmax(z), dp_i/dz_j = p_i (delta_ij - p_j), hence
//! dL/dz = p * (g - sum_j(p_j*g_j)). The reduction is per final-dimension row.
//! See Goodfellow, Bengio & Courville (2016), Deep Learning, section 6.2.2.3:
//! https://www.deeplearningbook.org/contents/mlp.html
//! formal/Softmax.lean derives the real-valued directional derivative and proves
//! the dense transpose-Jacobian reduction used here. It does not verify CUDA or floating-point rounding.
//! Only forward is fused; backward uses ordinary differentiable tensor operations.
use candle_core::{CpuStorage, CudaStorage, CustomOp1, DType, Layout, Result, Shape, Tensor, D};

struct Softmax;

impl CustomOp1 for Softmax {
    fn name(&self) -> &'static str { "mova-softmax-with-backward" }

    fn cpu_fwd(&self, _: &CpuStorage, _: &Layout) -> Result<(CpuStorage, Shape)> {
        candle_core::bail!("the fused softmax operation requires CUDA F32")
    }

    fn cuda_fwd(&self, storage: &CudaStorage, layout: &Layout) -> Result<(CudaStorage, Shape)> {
        use candle_core::cuda_backend::{cudarc::driver::{LaunchConfig, PushKernelArg}, kernels, CudaStorageSlice, WrapErr};
        let Some((start, end)) = layout.contiguous_offsets() else {
            candle_core::bail!("fused softmax requires contiguous input")
        };
        let cols = *layout.dims().last().ok_or_else(|| candle_core::Error::Msg("softmax requires a row dimension".into()))?;
        if cols == 0 || layout.shape().elem_count() == 0 {
            candle_core::bail!("fused softmax requires nonempty rows")
        }
        let rows = layout.shape().elem_count() / cols;
        let cols = i32::try_from(cols).map_err(candle_core::Error::wrap)?;
        let rows = u32::try_from(rows).map_err(candle_core::Error::wrap)?;
        let device = &storage.device;
        let src = storage.as_cuda_slice::<f32>()?.slice(start..end);
        let kernel = device.get_or_load_func("softmax_f32", &kernels::REDUCE)?;
        // SAFETY: the kernel writes every element, one block per nonempty row.
        let dst = unsafe { device.alloc::<f32>(end - start)? };
        let mut launch = kernel.builder();
        launch.arg(&src).arg(&dst).arg(&cols);
        // Launch ABI from Candle 0.9.2: source, destination, i32 row width;
        // one warp across block.y, one block per row. Input offset is retained.
        unsafe { launch.launch(LaunchConfig {
            grid_dim: (rows, 1, 1), block_dim: (1, 32, 1), shared_mem_bytes: 0,
        }) }.w()?;
        Ok((CudaStorage { slice: CudaStorageSlice::F32(dst), device: device.clone() }, layout.shape().clone()))
    }

    fn bwd(&self, _: &Tensor, probabilities: &Tensor, upstream: &Tensor) -> Result<Option<Tensor>> {
        let dot = (probabilities * upstream)?.sum_keepdim(D::Minus1)?;
        let centered = upstream.broadcast_sub(&dot)?;
        Ok(Some((probabilities * centered)?))
    }
}

/// Fused CUDA F32 forward plus analytical backward; other devices/dtypes use
/// Candle's reference operations. Contiguous conversion also handles transposes.
pub fn last_dim(xs: &Tensor) -> Result<Tensor> {
    if xs.device().is_cuda() && xs.dtype() == DType::F32 && xs.elem_count() > 0 {
        xs.contiguous()?.apply_op1(Softmax)
    } else {
        candle_nn::ops::softmax(xs, D::Minus1)
    }
}
