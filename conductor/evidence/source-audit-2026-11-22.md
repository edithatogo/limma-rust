# Source audit: the actual fork

## Identity and provenance

Observed 2026-11-22. Fork: [edithatogo/limma-rust](https://github.com/edithatogo/limma-rust). Parent/source: [MassDynamics/limma-rust](https://github.com/MassDynamics/limma-rust). Audited base: [ba41fb6bbf6e3f2524bd44f5be017c827d5c4aaa](https://github.com/edithatogo/limma-rust/tree/ba41fb6bbf6e3f2524bd44f5be017c827d5c4aaa).

The initial GitHub inspection reported main and issues disabled. A later prepublication refresh found an unrelated StepSecurity bot pull request adding pre-commit configuration; it was left unchanged. These are dated observations, not enduring settings. Conductor is therefore used as the work register; enabling issues merely to duplicate it is unnecessary.

The original README attributes the split to `md-limma/crates/limma-core` at md-limma `a4c3893`. The upstream Python binding remains in that project. Cargo declares `limma-core` 0.1.0 and GPL-3.0-or-later. This review preserves the licence and R-derived source attribution.

**The earlier audit of sinmojito/limma-rust at c569f87 concerned a different implementation. It is not this fork's defect list.**

## Observed implementation

| Capability | Source evidence | Meaning for the roadmap |
|---|---|---|
| Least-squares fit and per-feature missingness | [fit.rs](../../src/fit.rs), [linpack.rs](../../src/linpack.rs) | Existing implementation; verify before modifying |
| Contrasts and covariance propagation | [contrasts.rs](../../src/contrasts.rs) | Include non-orthogonal/rank-deficient cases in acceptance |
| Robust/trended empirical Bayes | [ebayes.rs](../../src/ebayes.rs) | Existing tests cover all four robust/trend combinations |
| Block correlation and GLS | [block.rs](../../src/block.rs) | Include block callers in kernel changes |
| T/F tables, BH decisions and CIs | [toptable.rs](../../src/toptable.rs) | Existing quantities can support a thin consumer export |
| R-compatible distribution tails and numerical helpers | [lib.rs](../../src/lib.rs), nmath, lowess, splines, optim, quad | Broad generic-library replacement is not a small cleanup |
| Production input parser/CLI | None found in source | Identifier alignment belongs to the caller/adapter contract |
| camera | No module or public implementation found | Removed from README and Cargo description; no method added |
| Weighted fitting, TREAT, complete limma API | Not established by this audit | Do not advertise or assume them |

The top-level fit APIs validate dimensions and finite design values. Some public lower-level helpers assume shapes already checked by their caller. LM02 should define the supported entry points and test misuse before deciding whether to add validation, restrict an interface or document a precondition.

The R LINPACK translation deliberately preserves limited pivoting and rank decisions. A generic QR decomposition can give correct fitted values while selecting different coefficients as non-estimable. It is not automatically interchangeable.

## Prior findings reconciled

| Claim from another implementation | What this source actually shows |
|---|---|
| A production file reader silently aligns mismatched identifiers | There is no such production reader; the numeric API is positional |
| A field called lfcSE contains unscaled standard deviations | There is no lfcSE field; CI computation uses `sqrt(s2_post) * stdev_unscaled` |
| Missingness does not update feature rank | Per-feature finite-observation refits use LINPACK rank, pivot and residual df |
| NaN sorting panics | `cmp_nan_last` provides explicit ordering |
| p-values and moderated F p-values are not gated | The current matrix tests compare both, alongside BH and CIs |
| One-sided nonfinite mismatches pass | `Report::check` rejects mismatched nonfinite kind/sign and finite-versus-nonfinite values |
| faer and statrs need updating | Neither is a dependency; adopting either would be a new decision |

Sources: [fit](../../src/fit.rs), [topTable](../../src/toptable.rs), [linear algebra](../../src/linalg.rs), [matrix empirical-Bayes tests](../../tests/matrix_ebayes.rs), [comparison harness](../../tests/common/mod.rs), [Cargo manifest](../../Cargo.toml).

These are source-inspection findings. They are not a proof that all edge cases pass.

## Existing reference evidence and limits

[The validation README](../../tests-validation/README.md) describes R 4.5.0, limma 3.68.5 and statmod 1.5.2. It identifies a committed corpus of about 30 MB, with a 600-protein slice of bojkova2020 plus synthetic/derived block cases. This is a useful mixed synthetic/proteomics regression corpus, not methylation validation.

The committed [parity report](../../tests-validation/report/parity.md) reports 68 compared files, 1,481,501 checked values and zero outside tolerance. Its source identity is **bfe6bad plus uncommitted changes**, using rust:1.95.0 on Linux amd64 in release mode. These numbers describe that historical upstream report. They do not constitute a new run at the fork revision.

`cargo test` replays committed expected values without executing R. The report script additionally checks exact drift in per-file summary values under a documented Docker environment; its last-digit rule must not be imposed blindly across Mac ARM and Windows.

Fresh regeneration is not yet self-contained:

- The scalar script reads the full external MDFlexiComparisons corpus and emits a layout requiring separation into inputs/results.
- Block regeneration refers to a locally named R Docker image; a reproducible image recipe or identified public environment is still required.
- The fork does not include a self-contained matrix-reference generation recipe.
- Some source headers mention limma 3.66.0 while the corpus README names 3.68.5. LM01 should record the exact reference for each case rather than relabel those versions as equivalent.

## Bounded improvements

1. **Fixture identity:** the CSV helper rejects malformed numbers and ragged rows, but fixture loaders do not consistently verify unique names and exact expression-column/design-row alignment. One shared strict helper is a small addition; it is not a production CSV repair.
2. **Consumer interface:** specify feature/sample IDs outside the numeric core, order, dimensions, missingness, finite-value handling, design/contrast names, multiple-testing universe and output units. Export actual moderated SE from existing quantities if required.
3. **Public helper contract:** test invalid shapes and unsupported domains at supported public entry points, then make the smallest demonstrated change.
4. **Independent oracle:** add only the missing reproducible reference cases, keeping existing comparisons and tolerances.
5. **Conditional algebra review:** `linalg.rs` includes hand-written dense kernels and a bounded Jacobi eigensolver. Consider a library only for an observed need and qualify all callers, including blocked models and empirical-Bayes/F paths.

No numerical defect is asserted merely because a function is custom, exposes a low-level precondition or differs in structure from another library. Runtime, source-version, scientific and resource qualification remain separate gates.

## Ponytail review decision

Proceed with provenance/context, baseline and a minimal consumer contract. Retain the existing runtime dependency set. Defer speculative kernel replacement, generic framework creation, automatic differentiation/GPU work and method expansion. The atlas reference backend proceeds while optional Rust admission is evaluated.
