# LM02b named consumer — executed provider record

Review date: **23 November 2026, Australia/Brisbane**. The filename and all recorded
instrument timestamps use UTC 22 November.

Status: **bounded producer accepted and archived** after all 29 strict checks
and both independent peer source reviews passed. Actual publishing hook commit
and Atlas immutable-pin receiving remain coordinating follow-ups. This record
does not claim scientific, production or actual owner-device acceptance.

## Source and contract

The strict baseline ran on clean `c88a1aad9c71a347ed2283e4b4fb4cb6565b243e`.
Its existing strict CI PR was then merged at
`27ead66f39f834ddae0f8e49282fa5bf8e91a496`, which is this increment's accepted
base and local patch-coverage comparator. The coordinator observed main
[strict CI](https://github.com/edithatogo/limma-rust/actions/runs/45031763243) and
[privacy](https://github.com/edithatogo/limma-rust/actions/runs/45031763219)
passing on that merged source. Those hosted results precede the new consumer.

The canonical [v1 contract](../../docs/named-consumer-v1.md) names the actual
Atlas qualification caller, supported domain and exact file schema. Limma owns
numeric validation and references; Atlas owns its receiver and provenance;
central standards own shared privacy policy. No numerical kernel, existing
golden/tolerance, dependency or Cargo lockfile changed. The only Cargo manifest
addition selects the exporter example tests with native `test = true`, keeping
them in the ordinary strict test/coverage gate.

## Commands observed

| Command | Observed result |
|---|---|
| `uv run --locked python scripts/verify.py --output artifacts/verification/lm02-consumer-baseline` | All 29 checks passed; 2026-11-22 14:10:15–14:10:57 UTC; receipt is an ignored local artifact |
| Initial `cargo test --locked --test named_consumer` before implementation | Exit 101, compiler E0432: public `limma_core::consumer` module did not exist; no numerical defect or changed kernel was claimed |
| `cargo test --locked --test named_consumer -- --nocapture` | First five cases passed: complete/missing reference outputs, IDs/order, shape/nonfinite rejection, excluded domains and explicit scale |
| `cargo test --locked --test named_consumer diagnostic_warnings -- --nocapture` | Sixth named test passed: original eBayes fallback warning and all non-estimable rows preserved |
| `cargo test --locked --example export_named` | Two tests passed: real CSV/JSON float64 round trip and invalid-invocation/existing-output refusal |
| `cargo clippy --locked --all-targets --all-features -- -D warnings` | Passed on the implemented provider/exporter candidate |
| First full `scripts/verify.py --output artifacts/verification/lm02b-final` | 29/29 checks passed in 123.86 s: 119 Rust tests per profile, 25 Python tests, 98% branch-aware patch coverage. Counting tests exposed that Cargo's default example target selection omitted the two separately passed exporter tests; the native target declaration was then added before final acceptance. |
| Final `scripts/verify.py --output artifacts/verification/lm02b-complete` | 29/29 checks passed in 62.135915 s, 14:42:31–14:43:33 UTC: 121 Rust tests in each debug/release/instrumented suite, zero failed/ignored; 25 Python tests. Both exporter tests are now required. Source snapshot remained unchanged during the run. |

The toolchain was the existing pinned Rust 1.95.0 environment on Linux x86-64,
with Cargo build parallelism two. R was not executed in this increment: tests
read the unchanged, independently generated R 4.5.0 / limma 3.68.5 / statmod
1.5.2 reference files.

The complete case checked 2,496 values, with the comparator's maximum relative error
`7.273792555232767e-14`. The missing case checked 2,493 values, maximum relative
error `2.3389187060382044e-12` for relative-comparison fields, with all 27 nonfinite reference values matching in
kind/sign. The largest fraction of an allowed comparison bound was approximately
0.01056 for a missing-case CI bound. Tolerances are unchanged and specified in the
contract. Both cases retain all 96 features and both contrasts.

The comparator's relative-error statistic does not combine absolute-tolerance CI
fields into that metric. Atlas's development receiver reports a separate worst
finite relative error over all numeric fields, including near-zero CI bounds;
that larger diagnostic does not change the prespecified absolute CI criterion.

The executable test reads every exported numeric value back as float64 and
requires identical bits for non-NaN values. It also checks exact IDs, order,
declared scales, metadata, warnings and preserved NaN. A test-only M-scale label
checks metadata transport; it is not a methylation qualification result.

## Strict identity, coverage and source review

The durable [verification record](lm02b-named-consumer-2026-11-22/verification.json)
retains all command statuses, log hashes, tested code/reference hashes and the
runtime receipt identity. The full tested source snapshot has 756 files; its
canonical mapping digest is
`2e84caab914d5e42314143f617e88f8cec292fb8f47f9f928db5f2f463fd0766`.
The runtime receipt digest is
`6f24e52df71e2f5e8c5b56ecdcd2c751479caff6e2a56a8bccbe3dcac1e9a52e`.
Both use SHA-256; the record specifies the snapshot encoding. Evidence/status
updates and the archive move follow that run and must pass the actual final
commit hook.

The measured branch-aware patch report has 258 covered changed lines out of 263,
displaying 98% against the required 90%; `src/consumer.rs` reports 100%. Pinned
cargo-llvm-cov 0.8.7 excludes example paths from LCOV/Cobertura by default even
when their tests run. Therefore this record makes **no exporter source-line
coverage claim**. Inspection of the installed upstream reporter found no separate
include-examples option; the debugging/unstable override removes dynamic vendor,
dependency, toolchain, build, test and workspace-exclusion defaults together.
The bounded change retains those defaults, keeps the exporter tests mandatory,
and documents the reporting limit rather than duplicating tool policy or altering
coverage reports.

Both independent peer reviews accepted the actual named boundary, exporter and
contract: shape/ID/order guards, pre-contrast rank, positive residual df, complete
family BH/CI, ordinary/moderated SE, exact output order/precision and warning/NaN
transport. Neither review claims production, device or scientific qualification.
The installed native hook and real central scanner proof are recorded separately
in [native-hook evidence](native-hooks-2026-11-23.md). The actual publishing commit
must still pass that hook before a source pin is delivered.

## Receiving and remaining parent gates

The coordinator observed three actual development exporter-to-Atlas exchanges
passing, each with 192 rows and 2,304 numeric fields; missing-case output retains
27 nonfinite fields. This is development evidence, not immutable provider-pin
acceptance. The pinned generator's `write_mat` fallback supplies sequential row
IDs only for the missing default eBayes scalar oracle; the canonical contract
documents that exact file/index mapping. All received output and other oracle
files remain strictly named; no reference bytes or tolerances changed.

Atlas must receive the published exporter pin; provider tests and the development
run do not replace parent LM02-5 immutable-source acceptance. Historical sample-name provenance,
production ingestion, all-missing/zero-df/global-singular downstream qualification,
actual-device execution and LM03 science remain open.
