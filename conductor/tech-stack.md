# Technology stack

Source inspection: 2026-11-22 at `ba41fb6bbf6e3f2524bd44f5be017c827d5c4aaa`.

| Layer | Observed state | Decision |
|---|---|---|
| Package | `limma-core` 0.1.0, edition 2021, GPL-3.0-or-later | Preserve package and attribution |
| Compiler | Manifest `rust-version = "1.85"`; toolchain file pins 1.95.0 | Treat MSRV and pinned-toolchain acceptance separately |
| Runtime dependencies | `thiserror = "2"` | Retain; no additional runtime dependency approved |
| Test dependencies | `csv`, `serde` derive, `serde_json` | Reuse existing golden-test machinery |
| Numerical code | R-derived nmath, LINPACK QR, lowess, splines, optimisation, quadrature and small dense algebra | Retain until an exact-contract replacement passes |
| Test fixtures | Committed scalar, matrix and block inputs/results plus embedded R reference cases | Replay before changing numerics |
| Current automation | Buildkite and Docker-based lint/test/report scripts | Preserve; not proof of a GitHub official local-runner result |
| Conductor | Official source submodule, pin in setup receipt | Project context and tracks; host activation is a separate observation |
| Data exchange | Numeric column-major slices in the library | Named Arrow/Parquet tables are the atlas/adapter's responsibility |
| R | Historical fixture generator and proposed isolated independent oracle | No R runtime dependency in `limma-core` |
| Python | Upstream binding remains in md-limma | No Python interface is required for this fork's initial work |
| Julia | An atlas modelling option outside this core | No Julia dependency in this crate |

## Candidate policy

The preference Rust → Julia → Python → R is conditional on demonstrated scientific equivalence. There is no compulsory migration deadline. Existing dependencies and standard-library facilities come first.

`faer`, `nalgebra`, `statrs`, `Rayon`, `Polars`, `Arrow`, `Parquet`, `PyO3`, `proptest` and `Criterion` are not existing runtime dependencies. Names in a candidate review are not installation decisions. [The decision ledger](decisions.md) specifies when reconsideration is justified.

The central scientific/native and Rust dependency standards are used for source-bound qualification and scoped dependency review. Rules specific to other products in those standards are not inherited here.
