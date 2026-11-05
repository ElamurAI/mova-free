# matrix-numerics

## descriptant

### matrix-numerics: wave30

polyfit→lstsq_svd joins real column-major Vandermonde with real y; roots/poly→Interp::call_function(eig) joins registry to linalg checks. ops routes matrix division to mldivide. elem cmap and FFT Mat::complex narrow zero im while explicit complex retains storage. FFT dim>=2 uses row slices, whereas reduce dims>2 use singleton slices. Local sort indices differ from global find indices. eval_nodes repeats callbacks after unsuitable vector attempt. Bench same column-major buffers feed both backends; main unexecuted.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave30 report. Registration is not a universal proof or whole-system runtime claim.

## model

# matrix-numerics

## forward

Registry-bound matrix functions share column-major index k=j*rows+i. Element maps apply scalar real/complex functions; reductions aggregate dimension slices. FFT computes F_N(P_N(x)) and inverse F_N^-1/N over those slices. Polynomial Horner uses descending coefficients; polyfit creates V[i,j]=x_i^(d-j), returns retained-SVD V^+ y, while roots builds companion C and calls eig. Dense solve chooses LU/QR/SVD by shape/rank and records warning residuals. Numeric callbacks implement GK15, Brent, Nelder-Mead and Dormand-Prince; trapz sums adjacent sample averages times spacing. Bench data/best are a separate diagnostic path, not runtime dispatch.

## invariant

Well-formed Mat vectors match rows*cols and optional im length. Coordinates are column-major except explicit reshaping. FFT dim>=2 aliases dim2, unlike reduction singleton slices for dim>2. Finite fixtures preserve stated values/tolerances; warnings do not imply rejection. Long deconv division, executed nonempty diff steps, cumulative and trapz read real parts; deconv short-dividend remainder and diff order0/zero-length-axis branches clone input and preserve imaginary storage. Callback retry is not transactional.

## cost

Elementwise O(N); slice construction/reductions/cumulative O(N+S), S=number of slices, including shaped empties N0,S>0. Sort O(N log(max(L,2))+S); FFT O(S*N log N) plus planning. Horner O(PX), convolution O(PQ); fit/eigen/dense solves O(mn min(m,n)) or square O(n^3) plus residual products. GK at most650 intervals with O(I) scans, Brent1000 iterations, ODE1e6 attempts plus callback cost. Bench helpers use n² samples/three callbacks only. No timings or speed claims; dimension overflow and allocations remain explicit.

## composition

polyfit→lstsq_svd joins real column-major Vandermonde with real y; roots/poly→Interp::call_function(eig) joins registry to linalg checks. ops routes matrix division to mldivide. elem cmap and FFT Mat::complex narrow zero im while explicit complex retains storage. FFT dim>=2 uses row slices, whereas reduce dims>2 use singleton slices. Local sort indices differ from global find indices. eval_nodes repeats callbacks after unsuitable vector attempt. Bench same column-major buffers feed both backends; main unexecuted.

## domain

Bounded synthetic CPU matrices and functions, valid storage, finite small arithmetic where required; no network, corpus, teacher, GPU, benchmark main or held-out suites. Models describe broader source branches with explicitly partial execution.

## limitations

Backend faer/rustfft semantics are dependencies; lexical projections do not resolve macros/types. Public malformed Mat storage, huge dimensions, allocation failure, nonfinite convergence, callback side effects and arbitrary complex support are not certified. Native benchmark helper selection uses cargo --bin because this package has no library target.

## checks

- Eight exact named native tests, twice debug and twice release; same ordered named outcomes
- Seven production compiled mutants, one per owned source; expected assertion failure exit101, exact-byte restoration and positive rerun in both profiles
- Seven lexical syntax projections generated twice identically; fragments reference parent projection with SHA256 byte scopes
- Focused rustfmt on added inline test blocks only; guarded own-source commit

## catalog

## behavior-complement

Wrong shape/order adapters are incompatible with bounded transport fixtures. FFT inverse without1/N violates impulse reconstruction. Long deconv division and executed diff/trapz arithmetic do not preserve imaginary parts, but cloned passthrough branches do. Callback retry excludes exactly-once effects. Warning gates and fzero success flag1 do not exclude nonzero residual results.

## reverse-reconstruction

Fixed-N Fourier outputs identify padded/truncated slices up to rounding, leaving arbitrary truncated tails. Poly quotient/remainder plus real divisor reconstructs dividend via padded convolution; short dividends clone remainder. Least-squares B lies in AX plus residual orthogonal to retained columns. Sort values plus permutation recover slices; sums have zero-sum fibers. fzero output is compatible with bracket-width stopping or1000-iteration exhaustion and nonzero residual, even discontinuous functions without a root; only sampled fb==0 establishes an observed zero.

## column-major-transport

Mat k=j*r+i, faer view and Vandermonde columns share layout. FFT dim1 gathers s*r+t; all admitted dim>=2 gather t*r+s and change cols toN. reduce::slices dims>2 instead emits singleton slices. Layout adapters must preserve these distinct rules.

## rank-and-coefficients

polyfit uses lstsq_svd threshold max(m,n)*smax*eps; rank builtin uses spacing-based tolerance and therefore is not a certified identical rank selector. roots scaling to monic companion discards nonzero leading coefficient scale.

## callback-effects

eval_nodes vector attempt can mutate Interp before fallback; subsequent scalar calls add effects. Model callback count separately from mathematical integrand evaluation. Bench helper best similarly calls exactly3 times, returns last value and independent minimum time.

## error-observations

linalg gate tolerance1000*n*eps concerns selected residuals, not component relative error. GK embedded K-G is estimated error and its650-interval exit warns. These do not compose into a percent bound for roots/fit/numeric callbacks.
