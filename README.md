# limma-rust

A pure-Rust port of the limma linear-model and empirical-Bayes core (`lmFit`, `contrasts.fit`,
`eBayes`, `topTable`, `decideTests`) with R-faithful nmath and lowess. The crate is
`limma-core`.

It was split out of `md-limma` (`crates/limma-core` at md-limma `main` a4c3893) with its history.
The Python binding and the Mass Dynamics job layer stay in md-limma.

## Test

Use the strict native gate from the start of work:

```sh
python scripts/bootstrap.py
uv sync --locked --python 3.15.2
uv run --locked python scripts/verify.py
```

Python is isolated verification tooling, not a library/runtime dependency.
Bootstrap supports Python 3.11+; strict checks use Python 3.15.2 and uv 0.13.9.
Git, rustup and the native Rust
linker are prerequisites. Bootstrap pins Rust 1.95.0, cargo-llvm-cov 0.8.7,
cargo-audit 0.23.0, Gitleaks 8.30.1 and actionlint 1.7.12. Native release archives
and their checksum manifests are hash-verified. The default build parallelism is
two jobs; an explicit `CARGO_BUILD_JOBS` can choose the host's capacity.

The command checks tool versions, documents, workflow syntax, formatting, Clippy
with warnings denied, locked debug and release tests, Rust documentation,
dependency advisories, real Rust and Python coverage, strict Python types/lint,
and the source snapshot for secrets.
Missing tools and failed commands return failure; a later pass cannot overwrite
an earlier failure. Logs and a source-hashed receipt are written under the ignored
`artifacts/verification/` directory. No statistical tolerance is relaxed.

Hosted CI runs the same command on disposable workers. The separate central
privacy workflow checks introduced/full Git history and personal identifiers.
Codecov uses the Rust template's project nonregression and 90% patch policy;
measured coverage is not numerical acceptance. Official owner-device acceptance
still requires the trusted manual workflow and its actual run/device evidence.
The local patch gate counts partially covered branches as misses. GitHub's
dependency-diff API is unsupported for this fork; the strict job instead retains
the complete Cargo.lock audit, the installed locked Python developer environment
audit, and actual source SBOM. The Python result covers the executing platform's
environment; other device environments require their own qualification.

The tests compare against the R goldens in `tests-validation/` (inputs in `data/`, R's results in
`expected/`, the R scripts in `scripts/`; see its README). Set `MD_GOLDEN_CORPUS_DIR` to run them
against another corpus during an explicitly labelled experiment. The strict gate
clears both corpus override variables so it always replays the committed inputs.

## Owner-fork qualification update

The [fresh independent reference](tests-validation/FRESH_REFERENCE.md) is now independently reproducible from public hash-pinned R/limma/statmod sources. Four synthetic cases regenerate 117 files byte-identically; the new slice compares 30,840 values (including 249 matching nonfinite values), and both cloud debug/release suites pass 110 tests. [Independent review](conductor/evidence/fresh-reference-ponytail-review-2026-11-22.md) accepted this bounded increment; [LM01a is archived](conductor/archive/lm01a_reference_20261122/index.md). Production numerical source and historical fixture tolerances remain unchanged.

This is not methylation/clinical admission or a run on the owner's Mac/Windows devices. Consumer identity contracts and the official local-runner receipts remain in Conductor. sinmojito/limma-rust is a separate, optional comparator for a named method; neither agreement between ports nor language choice replaces the independent R oracle.
