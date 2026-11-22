# Baseline status and prerequisites

This record separates source inspection from distinct executable gates. A preliminary cloud debug replay has now passed: **104 tests passed, none failed or ignored** using `cargo +1.95.0 test --locked`. The [receipt](cloud-corpus-replay-2026-11-22.json) and [log](cloud-corpus-replay-2026-11-22.log) identify its exact scope. No fresh R process or user-device runner was involved.

| Gate | Current status | Required evidence / owner |
|---|---|---|
| Source and capability audit | pass | LM00 audit at ba41fb6; documentation only |
| Conductor source and context | pass | Setup receipt; host activation remains separate |
| Existing committed tests on exact fork | pass: cloud debug replay only | 104 passed; receipt and log linked above |
| Cloud workspace smoke run | pass: bounded evidence | Linux x86_64; not local-device acceptance |
| Pinned toolchain 1.95.0 | pass: debug test command | rustc 1.95.0, terminal exit 0; no broader claim |
| Formatting, clippy and release-mode gates | pass: cloud workspace only | fmt --check and clippy -D warnings passed; release corpus104passed; [receipt](cloud-static-release-checks-2026-11-22.json) |
| Manifest MSRV 1.85 | not_run | Separate compiler compatibility experiment if needed |
| Fresh independent R reference | blocked | LM01: pinned public R environment and self-contained generation recipe |
| Official local Actions runner on Mac | blocked | Device availability, official runner setup and completed reviewed-source job |
| Official local Actions runner on Windows | blocked | Device availability, official runner setup and completed reviewed-source job |
| Methylation consumer qualification | not_run | LM02/LM03 plus eligible atlas input and matched reference result |
| End-to-end performance/resource claim | not_run | Identical accepted workload, measurements and limitations |

## Small executable checks

The smallest embedded R-reference smoke is:

```sh
cargo test --locked --test r_reference
```

It exercises the committed spline and NA-masked rank-deficient fit reference cases. It does not start R or cover all methods.

A focused core replay is:

```sh
cargo test --locked --test scalar_goldens --test matrix_lmfit --test matrix_ebayes
```

The full existing suite is `cargo test --locked`; release-mode coverage is `cargo test --locked --release`. The documented formatting and lint commands are in [the workflow](../workflow.md). Keep `MD_GOLDEN_CORPUS_DIR` unset for the committed corpus; a nonexistent explicitly set path deliberately fails instead of silently reducing coverage.

## Fresh oracle prerequisite

Reuse the committed cases and existing harness. First make a small synthetic fit/contrast/eBayes reference independently regenerable with pinned R/limma/statmod, an explicit script, hashes and licence/provenance. Include missingness-induced rank changes, non-estimable coefficients, contrasts, posterior variance, moderated SE, t/F statistics, p-values, BH and CIs where supported.

Do not require a rebuild of every upstream fixture before useful baseline replay. Reconcile the external scalar inputs, matrix-generation gap and block image recipe separately. A future expanded corpus cannot be treated as available until its access and provenance are established.

## Device runner prerequisite

The intended user-device acceptance uses GitHub's official [actions/runner](https://github.com/actions/runner). Record verified machine details and the actual runner/job receipt; the GPU model is not inferred from an ambiguous historical label. Registration and machine access are not performed by this setup. Trusted-source execution and least-privilege workflow permissions are part of the prerequisite.

The Mac and Windows jobs need not produce identical last digits. They must pass prespecified numerical contracts. Any platform-specific tolerance decision must be justified against the independent oracle, not chosen after seeing a failed result merely to pass it.

## Manual local workflow

The [official local workflow setup](local-workflow-setup-2026-11-22.md) now points to a real manual job. Static actionlint1.7.12 passed after moving runner-context paths into step-level environments. No GitHub job was dispatched. The additional cloud formatting/clippy/release checks are [recorded separately](cloud-static-release-checks-2026-11-22.json);104tests pass in each debug/release run, not208unique tests.

## Subsequent same-day completion

The initial pending independent-oracle task now has a completed bounded public synthetic increment: [fresh reference](../../tests-validation/FRESH_REFERENCE.md), [receipt](fresh-reference-2026-11-22/receipt.json), [review](fresh-reference-ponytail-review-2026-11-22.md). This does not regenerate every historical corpus component. The actual-device and consumer/methylation tasks remain open.
