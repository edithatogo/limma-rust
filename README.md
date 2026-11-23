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

### Install the local commit gate

After tool bootstrap, install the tracked Git hook in each clone using an isolated
checkout/environment of the central privacy scanner:

```sh
uv run --locked --python 3.15.2 python scripts/hooks.py install --privacy-root /absolute/path/to/pinned-repository-standards --privacy-python /absolute/path/to/privacy-environment/bin/python --gitleaks /absolute/path/to/gitleaks
```

The scanner checkout must be clean at
`0be4668d17ac3b6d902d4354bd01f420c7c29272`, matching the hosted privacy action;
the isolated scanner environment uses Python 3.15.2 and Gitleaks 8.30.1.
Use the actual executable paths on the host, including `Scripts/python.exe` for a
Windows virtual environment. The installer preserves existing hook configuration
by refusing conflicts. It writes clone-local `core.hooksPath` and scanner paths;
tracking the hook alone does not install it in another clone.

Stage the complete candidate before committing. The hook rejects unstaged tracked
changes and nonignored untracked files, runs central staged privacy scanning and
the full strict verifier, and checks that HEAD and the index did not change during
verification. Missing tools or any failing gate abort the commit. Git hooks remain
locally bypassable; installed hooks and successful CI do not establish server-side
branch protection. See [workflow](conductor/workflow.md) for the separate enforcement
and owner-device boundaries.

The tests compare against the R goldens in `tests-validation/` (inputs in `data/`, R's results in
`expected/`, the R scripts in `scripts/`; see its README). Set `MD_GOLDEN_CORPUS_DIR` to run them
against another corpus during an explicitly labelled experiment. The strict gate
clears both corpus override variables so it always replays the committed inputs.

## Owner-fork qualification update

The [fresh independent reference](tests-validation/FRESH_REFERENCE.md) is now independently reproducible from public hash-pinned R/limma/statmod sources. Four synthetic cases regenerate 117 files byte-identically; the new slice compares 30,840 values (including 249 matching nonfinite values), and both cloud debug/release suites pass 110 tests. [Independent review](conductor/evidence/fresh-reference-ponytail-review-2026-11-22.md) accepted this bounded increment; [LM01a is archived](conductor/archive/lm01a_reference_20261122/index.md). Production numerical source and historical fixture tolerances remain unchanged.

That 110-test result is the historical LM01a milestone. LM02a subsequently reached
113 Rust tests after fixture-identity checks; its [bounded acceptance is archived](conductor/archive/lm02a_fixture_identity_20261122/index.md).
The strict CI work in [PR #5](https://github.com/edithatogo/limma-rust/pull/5) was
merged as `27ead66f39f834ddae0f8e49282fa5bf8e91a496`; its main
[strict run](https://github.com/edithatogo/limma-rust/actions/runs/45031763243) and
[privacy run](https://github.com/edithatogo/limma-rust/actions/runs/45031763219)
passed. Those receipts precede the current named-consumer increment.

### Named consumer and repository boundaries

[Named consumer v1](docs/named-consumer-v1.md) defines the single authoritative
numerical interface. `consumer::fit_named_contrasts` checks exact named alignment
around the existing unweighted fit/contrast/default-eBayes route and returns every
feature/contrast with ordinary and moderated SE, 95% CI, p/BH p and estimability.
The synthetic exporter provides ordered float64 CSV and contract metadata:

```sh
cargo run --locked --example export_named -- tests-validation/fresh-reference/complete artifacts/named-complete log2_expression
```

Use a new output directory. This example reuses the reference-fixture reader; it is
not a clinical ingestion CLI. Per-feature non-estimability with positive residual
df is preserved. All-missing/zero-df features, globally singular designs and other
unselected methods are explicitly outside v1. M-value scale declarations describe
log2 methylation odds, never automatically RNA expression fold changes.

Limma owns this numeric contract and independent reference comparisons. Atlas owns
the receiving executable, provenance, identifiers beyond exact matrix alignment,
preprocessing, storage and scientific admission. Central repository standards own
the shared privacy policy and scanner. Keep these repositories separate and use
versioned file exchange; there is no new numerical dependency or statistical-kernel
replacement. [LM02b](conductor/archive/lm02b_named_consumer_20261122/index.md) archives
the accepted bounded producer: 29 strict checks, 121 Rust tests per profile and
25 Python tests passed. Its actual publishing hook commit and Atlas immutable-pin
receiving remain coordinating follow-ups; the default tool's example source-line
coverage exclusion is explicit in the contract.

This is not methylation/clinical admission or a run on the owner's Mac/Windows devices.
Historical sample provenance, production exchange and official local-runner receipts
remain in Conductor. sinmojito/limma-rust is a separate, optional comparator for a named
method; neither agreement between ports nor language choice replaces the independent R oracle.
