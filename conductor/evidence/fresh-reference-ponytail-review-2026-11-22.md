# LM01a independent Ponytail review

Date: 2026-11-22 UTC. Reviewer: independent research/review agent, `/root/rust_backend_search_v2`.

## Decision

**Accept the bounded LM01a fresh-reference increment.** No material unresolved finding was identified in the reviewed source, package provenance, fresh fixtures, or retained execution evidence. The increment supplies an independent, publicly provisionable R reference for the stated synthetic cases without changing the production numerical core or adding a production runtime dependency. It can be reviewed and archived before the next track proceeds.

This acceptance covers the new reference slice. It does not certify a complete replacement for R limma, the entire historical golden corpus, methylation-specific scientific validity, or execution on the owner's Mac and Windows devices. Those remain separate track gates.

## Scope and independence

Read the LM01a specification and plan; `tests-validation/scripts/fresh_reference.R`, `install_reference_debian.sh`, `reference-environment.json`, and `reference-packages.sha256.tsv`; `tests/fresh_reference.rs`; the relevant shared comparison harness; the generated fixtures; and the evidence directory `conductor/evidence/fresh-reference-2026-11-22/`.

The R recipe generates its own synthetic observations with a stated RNG kind and seed 20261122. It calls R limma's `lmFit`, `contrasts.fit`, `eBayes`, and `topTable` directly. It does not read Rust results, historical golden output, private MassDynamics inputs, or patient data. The statistical oracle is therefore independent of the Rust implementation. The shared Rust comparison harness is reused rather than replaced by a second custom validation framework.

The fresh cases are explicit:

| Case | Scope exercised |
| --- | --- |
| Complete, 96 features and 12 samples | Matrix fit, two contrasts, all four robust/trend combinations, moderated inference and top tables |
| Missing observations, 96 features and 12 samples | Unequal residual degrees of freedom and feature-specific rank loss; the same four inference configurations |
| Missing-data edges, 5 features | Entirely missing/one-observation/rank-loss fitting and contrast behavior; no claim of downstream empirical-Bayes coverage |
| Singular design, 8 features | Redundant design column, fit rank/pivot and estimable contrasts; explicit rejection of a contrast targeting a non-estimable coefficient |

The comparisons include coefficients, standard-error components, covariance, sigma, residual degrees of freedom, averages, rank and pivot, posterior variance/degrees of freedom, moderated standard errors and t statistics, raw p-values, F statistics and F p-values, log odds, confidence intervals, BH-adjusted p-values, and top-table ordering. Nonfinite entries are checked rather than silently removed. The serialization maps R missing/NaN values to the fixture missing-value representation; this is not a claim to distinguish R NA from R NaN payloads.

Fresh weighted/block fits, voom, TREAT, full methylation preprocessing, and complete regeneration of every historical fixture are outside this increment. No production `src/` or shared-harness diff was present during review.

## Independent checks executed by this reviewer

The following checks were personally executed as read-only audits:

- Recomputed all SHA-256 values in `fixture-hashes.json`: exactly **117 files**, exact file-set agreement, and all digests matched.
- Independently hashed both retained fresh-generation directories, `fresh-r-oracle-run2` and `fresh-r-oracle-provisioned`: both contained exactly the same 117 files and bytes as the checked-in reference.
- Recomputed all five cached provisioning-input SHA-256 values: each matched `reference-packages.sha256.tsv`, including the R Debian package, both R source packages, and the two Fortran packages.
- Read the installed and source-package DESCRIPTION records: both independent installations and source archives report **limma 3.68.5** and **statmod 1.5.2**. limma's source record reports `RELEASE_3_23`, commit `825d1c8`; the full source archive is additionally pinned by SHA-256.
- Checked the fixture version CSV and retained R session information: **R 4.5.0**, limma 3.68.5, statmod 1.5.2, Debian 13 x86-64, C locale, and UTC agree with the declared version contract.
- Summed the 12 rows in `comparisons.tsv`: **30,840 compared values**, **249 matching nonfinite values**, and **zero failures**.
- Parsed both retained suite logs: each contains **110 passed, zero failed, zero ignored**, across 18 result groups.

The full test suites, provisioning, and fresh R executions were performed by the implementation agent; this reviewer inspected their receipts and independently audited the resulting files and counts. They were not redundantly rerun during this review. The retained format and Clippy receipts report success. Cloud execution is not evidence of the separate official owner-device runner qualification.

## Failure detection and tolerance decision

Inspected the failure logs, not just their exit codes. A missing reference fails at the reference-directory assertion; the edited version fails the exact version contract; a corrupted p-value fails its numeric comparison; replacing a missing reference coefficient by zero fails nonfinite matching; and R refuses to overwrite a nonempty reference directory. These are intended guard failures rather than compiler or missing-tool failures.

The original pure-relative failures remain recorded in `initial-pure-relative-failure.log`. Near-zero covariance entries from orthogonal, unit-scaled designs differ by at most about `1.57e-17`, for which a relative-only bound is inappropriate. The approved covariance contract adds an absolute `1e-14` floor while retaining relative `1e-8`. A near-zero log-odds value differed by about `3.90e-13`; the approved log-odds/top-table B contract uses absolute `1e-10` with relative `1e-8`, consistent with the existing block log-odds floor. These floors are narrowly scoped and documented in the tests. No other statistical tolerance was relaxed to obtain the final pass.

Covariance diagnostics expose their near-zero relative errors; the aggregate harness maximum-relative field excludes values governed by an absolute floor. Consequently, the report must not be interpreted as every individual relative error being below `1e-8`. The retained original failure and final receipts make this distinction reviewable.

## Provisioning and remaining boundaries

The public provisioning recipe checks the operating-system/architecture boundary and every downloaded input digest, builds the pinned R packages, then asserts the runtime/package versions. It records the host BLAS/LAPACK and compiler context. It is deliberately a Debian 13 amd64 recipe with host shared-library dependencies, **not a hermetic cross-platform environment**. The package/source URLs and checksums are public; local absolute workspace paths in command receipts identify execution locations, not private input dependencies.

No new dependency or numerical rewrite is warranted for this increment. Reuse the established harness and retain R as the independent oracle. Continue with the already scoped consumer/identifier contracts and owner-device qualification before promoting this narrow numerical qualification into an atlas analysis backend claim.

## Reviewed source hashes

These hashes bind the acceptance to the implementation inspected here:

| File | SHA-256 |
| --- | --- |
| `tests-validation/scripts/fresh_reference.R` | `14a3a49a571c31afeed82b00274b646f879ebe2fa7727e092b6ad0d1f769d761` |
| `tests-validation/scripts/install_reference_debian.sh` | `500938ebdb70ce7c278376a66e5be087976f92f80099d0a1dec859eea8b8b476` |
| `tests-validation/scripts/reference-environment.json` | `69ea6d4f39cc3db227f33dc3758e3ca5abb376d124b5a760a2f8b0fbb9060e6c` |
| `tests-validation/scripts/reference-packages.sha256.tsv` | `72f86150fb77280396918e365475fb615deaf016238e4a726b6024042201e0eb` |
| `tests/fresh_reference.rs` | `865f74d7a1a3a6f1acf38ff122b153193a062f54ba6c1ca6acacb498676e8c6b` |

