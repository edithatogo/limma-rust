# Dependency and boundary decisions

Reviewed 2026-11-22 using Ponytail's source-based, dependency-first approach. These are project decisions, not claims of independent third-party certification.

| Area | Decision now | Revisit condition and required evidence |
|---|---|---|
| Existing harness | Reuse `tests/common`, CSV and Serde fixtures | Add only missing checks; one shared loader rather than parallel frameworks |
| Error type | Retain `thiserror` | Extend existing error contract if a public call needs a validated failure |
| Generic dense algebra | Defer `faer` / `nalgebra` adoption | An observed maintenance problem or measured resource bottleneck, then a bounded LM04 prototype |
| QR | Retain R LINPACK limited pivoting | Replacement must reproduce pivots, rank, non-estimable coefficients and all affected caller behaviour |
| Distribution tails | Retain existing R nmath | Generic library must pass extreme/log-tail, quantile and nonfinite grids without relaxed bounds |
| Lowess / splines / optimisers | Retain | Exact caller-level reference comparisons before any substitution |
| Parallelism | No new Rayon dependency | A profile shows independent work dominates, reproducibility and end-to-end benefit are demonstrated |
| Tabular/storage libraries | Atlas or thin optional adapter only | A named consumer needs Arrow/Parquet interchange; core remains independent |
| Property testing | Existing deterministic tests first | Add `proptest` only for a concrete missing invariant with smaller ongoing cost |
| Benchmark framework | Existing timing/receipt first | Add `Criterion` only if repeated stable microbenchmarks are actually needed |
| Python binding | Reuse upstream adapter if needed later | A real consumer requires it and its ABI/schema contract is qualified |
| GPU or automatic differentiation | Defer | A specific model requires gradients/accelerator work and end-to-end benefit can be assessed |

## Close additions versus new methods

Small potential additions are strict fixture-name validation, a shared loader, documented public shape checks and a consumer-side export of existing quantities. Moderated standard error is already computable as `sqrt(s2_post) * stdev_unscaled`; an exporter must not treat `stdev_unscaled` as that standard error.

Weighted fitting, TREAT and camera are substantive statistical methods, not metadata additions or generic-library substitutions. Do not add them simply to advertise broad parity. LM05 first requires a concrete consumer and an independent method reference.

Do not replace custom R-compatible code just because it is custom. Measure the actual problem and verify every affected caller. For example, shared Cholesky/eigendecomposition code is used by blocked models, contrasts and empirical-Bayes/F tests; a simple least-squares example cannot qualify a replacement.

## Standards and provenance

This fork adopts the relevant review principles from:

- [Scientific/native qualification at a8b675f](https://github.com/edithatogo/repository-standards/blob/a8b675feeb62cb39ba62cd3ab03cb5deaf281ad0/docs/scientific-native.md)
- [Rust dependency rationalisation at a8b675f](https://github.com/edithatogo/repository-standards/blob/a8b675feeb62cb39ba62cd3ab03cb5deaf281ad0/docs/rust-dependency-selection-standard.md)

A central candidate list does not install dependencies, qualify this consumer, set a universal compiler floor or activate requirements intended for another repository.

## Updated source comparison and fresh oracle

The 22 November expanded [atlas source review](https://github.com/edithatogo/rare-omics-atlas/blob/main/research/qualification/rust-backend-search-2026-11-22.md) retains MassDynamics/this fork as the first bounded numerical candidate. sinmojito is optional for a named additional method or triangulation, not a second mandatory runtime or independent oracle. Direct pinned R remains the independent reference. No wholesale merge or new numerical dependency is selected.

The completed LM01a increment makes a public R4.5.0/limma3.68.5/statmod1.5.2 slice independently regenerable. Its narrow covariance/LODS mixed tolerances and original failures are documented; no production kernel or historical tolerance changed. Before performance-driven replacement, reuse upstream MassDynamics benchmark branch `df2c50b5473de823dcfcc70c0ce851d3585559fc` after checking its actual consumer coverage, rather than creating another harness. Its historical M5 numbers do not predict M1/Windows/array performance.
