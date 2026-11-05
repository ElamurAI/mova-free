# mova-approx

Standard-library-only contracts for experimental executable groupant surrogates.
ErrorBudget admits only 1%, 2%, 5% and 10% maximum error, in that order.
Exact outputs qualify at every tier. Unsupported caps and errors above the
selected cap fail closed, including the next representable float above 10%.
No production implementation is replaced by this crate.

Use relative_error(reference, approximation) for finite signed scalar values;
zero reference requires exact zero output. Use observed_certificate for a
nonempty collection of finite nonnegative error fractions. It records their
maximum and sample count. Use analytic_certificate for a caller-established
bound and nonblank scope. It checks the numeric contract, not the proof.
Certificate fields are private and available through getters.

Three independent typed axes provide stable snake_case labels through as_str():

- MetricKind: scalar_relative, normalized_rank, objective_loss,
  probability_relative.
- MethodKind: quantized_numeric, certified_sample, relaxation_certificate,
  floating_normalization, interval_certificate.
- AssuranceKind: analytical, per_input_certificate, finite_observation.

Classification describes the caller's metric, mechanism and evidence scope.
It does not certify a claim or convert finite observations into guarantees.
The caller supplies the metric definition, domain and supporting evidence;
a method name such as certified_sample alone establishes no assurance.

Run `cargo test -p mova-approx --lib`. The two measurement runners are the
examples `groupants` and `groupants_next` (`cargo test --release -p mova-approx
--example groupants`, then `cargo run --release -p mova-approx --example
groupants`; the same for `groupants_next`); `cargo test -p mova-approx
--all-targets` includes both. Their profiles are described in
[executable Rust approximants](../../docs/mathematical-modeling.md#executable-rust-approximants). Mathematical models, typed obernants and
source binding live in `src/lib.rs.groupant/`. Registration alone does not
establish validity.

The example `student_vidtvorant` runs the school student's executable vidtvorant
(`world/src/knowledge.rs.groupant/student-vidtvorant.groupant/rust_approximant.rs`) on traces of
`world`'s `student_probe` example; it meets no cap and registers no profile (see `docs/school.md`, «Speed and the
student's vidtvorant»).
