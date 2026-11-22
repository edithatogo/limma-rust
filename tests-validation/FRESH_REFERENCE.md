# Fresh public R reference (LM01a)

This bounded synthetic oracle was generated afresh with **R 4.5.0, limma 3.68.5 and statmod 1.5.2**. It exercises the existing Rust implementation; it is not a new statistical method, a patient-data benchmark or acceptance on the owner's Mac or Windows computers.

## What is independent

The [generator](scripts/fresh_reference.R) uses base R, limma and statmod. Its only input is a fixed synthetic design and an explicitly seeded R random-number generator. It does not read historical expected files, the private MassDynamics corpus, the upstream Python adapter or patient data. It writes numeric values with 17 significant digits, with missing/nonfinite values preserved.

The committed [fresh-reference/](fresh-reference/) directory is a small self-contained corpus with inputs and expected outputs together. This is the mixed layout already supported by the existing `Matrix` reader. [The Rust tests](../tests/fresh_reference.rs) reuse that reader and `Report` comparator. No runtime or development dependency was added.

| Case | Scope |
|---|---|
| complete |96 synthetic features × 12 arrays; intercept, three groups and a within-group numeric covariate; two group contrasts |
| missing | Same design; selected observations removed, including loss of one or both group effects, unequal residual degrees of freedom and non-estimable feature coefficients |
| missing_edges | Five features including all-missing data, one observation/zero residual degrees of freedom, group-specific rank loss and a complete feature; fit/contrast checks only |
| singular_design | Eight features, an explicitly redundant design column, valid contrasts and an independently generated error for a contrast on the non-estimable coefficient |

Complete and missing cases cover all four robust/trend eBayes combinations. Outputs include coefficients, unscaled SD, residual SD/df, covariance and pivot/rank, posterior/prior variance and df, moderated SE, t/F statistics, p-values, BH-adjusted t/F probabilities, log-odds and 95% confidence intervals.

The labels and values are synthetic. Their numeric `logFC` field follows the R API and does not establish methylation or clinical-effect semantics.

## Reproduce the public environment

[reference-packages.sha256.tsv](scripts/reference-packages.sha256.tsv) pins every downloaded archive by SHA256 and an exact public URL. [reference-environment.json](scripts/reference-environment.json) describes their provenance. The recipe uses a Debian binary distribution of R 4.5.0-3 and builds limma/statmod from public sources. limma's source DESCRIPTION records commit `825d1c8`; its archive hash identifies the full source artifact.

The bounded provisioning helper supports Debian 13 / amd64 with GCC 14, make and the shared libraries required by R already present. It extracts packages into a user-owned directory, relocates Debian's path wrappers/configuration and installs the two reference packages there. It does not require root or a private container. Host shared libraries remain part of the runtime; this is **not a hermetic or cross-platform environment claim**.

From the repository root:

```sh
bash tests-validation/scripts/install_reference_debian.sh /absolute/path/reference-runtime

R_LIBS_USER=/absolute/path/reference-runtime/library TZ=UTC LC_ALL=C \
  /absolute/path/reference-runtime/root/usr/bin/R --vanilla --slave \
  --file=tests-validation/scripts/fresh_reference.R \
  --args /absolute/path/new-reference

LIMMA_FRESH_REFERENCE_DIR=/absolute/path/new-reference \
  cargo test --locked --test fresh_reference -- --nocapture
```

Use a new empty output directory. The generator refuses to overwrite one, and rejects the wrong R/limma/statmod versions. A normal installation of the same versions can run the script with `Rscript --vanilla` instead. The local Debian extraction uses the relocated `R` launcher because its distribution's compiled `Rscript` launcher has an absolute install path.

`LIMMA_REFERENCE_CACHE` optionally points the installer to already downloaded archives; all hashes are checked even when cached. The public installer was actually replayed in a second runtime directory in this cloud workspace. Its freshly generated 117 files matched the independently generated reference byte-for-byte. The cache contained the hash-verified public source archives, not compiled private packages.

## Replay the committed reference

```sh
cargo test --locked --test fresh_reference
cargo test --locked
cargo test --locked --release
```

R is not needed for ordinary Rust test execution. An explicitly supplied missing `LIMMA_FRESH_REFERENCE_DIR` fails; it never falls back silently. The override affects only this independent corpus. `MD_GOLDEN_CORPUS_DIR` remains the existing historical corpus override.

## Numerical contract and retained initial failures

The bounds follow the established matrix/block harness, with two explicitly reviewed clarifications. For finite values, the comparison is:

`abs(Rust - R) <= max(relative * abs(R), absolute)`

| Quantity | Relative bound | Absolute floor |
|---|---:|---:|
| Coefficients and confidence limits |0|1e-10|
| Residual/unscaled/moderated SD, posterior variance, t/F, Amean |1e-8|0|
| p-values and BH-adjusted probabilities |1e-8|1e-12|
| Prior variance and prior/total df |1e-6|0|
| Covariance, only these unit-scale synthetic designs |1e-8|1e-14|
| Log-odds/B |1e-8|1e-10|
| Rank and pivot |0|0|

The first comparison deliberately used the existing pure-relative covariance/log-odds bounds. It failed, and the [initial failure log](../conductor/evidence/fresh-reference-2026-11-22/initial-pure-relative-failure.log) is retained:

- Analytically zero covariance terms were approximately 1e-17 in both implementations and differed by at most 1.57e-17 at those entries. Their relative errors were large because the reference was essentially zero. The design entries range from −1.5 to 2; the 1e-14 floor applies only to this declared scale. The test reports both maximum absolute and relative covariance errors, including near-zero references.
- One missing/robust/trend log-odds value near −2.75e-5 differed by 3.90e-13, just outside a pure-relative 1e-8 bound. The new corpus adopts the existing cancellation-aware `LODS_BLOCK` policy in `tests/matrix_ebayes.rs`.

These were explicit contract clarifications approved during independent coordination before acceptance. No oracle value, production numerical code, historical fixture, p/F/BH/SE/CI bound or old test tolerance was changed. NA/NaN and infinities still have to match in kind and sign.

## Observed verification

[The evidence directory](../conductor/evidence/fresh-reference-2026-11-22/) contains command logs, generated fixture hashes, public package receipts, sessionInfo, runtime linkage and per-comparison metrics.

The fresh comparisons cover 30,840 numeric values including 249 nonfinite values, with zero failures after the documented clarification. They add six tests to the existing 104, for 110 tests in each debug/release run. Repeated runs do not count as additional unique tests.

Negative controls were actually executed: a missing reference path, an altered version, a 1e-4 perturbation to a reference p-value, replacement of an expected NA with 0, and attempted output overwrite all failed as intended. This checks that absent or inconsistent evidence cannot become a passing reference silently.

The 68 historical parity-report rows remained byte-identical in the observed Linux / Rust 1.95.0 replay. The 12 new rows are recorded in the separate [comparisons.tsv](../conductor/evidence/fresh-reference-2026-11-22/comparisons.tsv). The historical Docker report was not regenerated here: its optional exact-report drift check will flag these additional rows until that canonical report is reviewed and updated. Do not describe the cloud native run as a Docker or official local-runner execution.

## Limits and next boundary

This establishes a bounded, independently regenerated oracle for the existing fit/contrast/moderation path. It does not qualify every limma feature, block model, extreme numerical configuration, real methylation consumer, cross-platform performance claim or future dependency replacement.

Keep the separate official GitHub Actions runner tasks for the intended Mac and Windows devices open. The clinical atlas's scientific admission requirements also remain separate.

