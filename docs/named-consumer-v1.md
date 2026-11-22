# Named contrast consumer v1

Review date: **23 November 2026, Australia/Brisbane**. Machine timestamps and the
associated `2026-11-22` evidence/track filenames use UTC.

**Contract ID: `limma-named-contrasts-v1`.** This is the authoritative bounded numerical contract for `consumer::fit_named_contrasts` and the synthetic `export_named` example. Atlas owns its receiving adapter, storage, source provenance and scientific admission. This increment adds a named boundary around existing numerical functions; it does not qualify methylation preprocessing, clinical analysis or an actual owner device.

## Supported input

The Rust entry point takes three borrowed `NamedMatrix` values and an explicit `InputScale`. A matrix contains `data: &[f64]`, `row_ids: &[String]` and `column_ids: &[String]`. Axis lengths define the dimensions; values are **column-major float64**.

| Matrix | Row identity | Column identity | Values |
|---|---|---|---|
| Expression or M matrix | Complete supplied feature universe | Ordered samples | Finite values or NaN for missing observations |
| Design | Exactly the same samples in the same order | Named design coefficients | Finite values only |
| Contrasts | Exactly the design coefficients in the same order | Named contrasts | Finite values; every contrast column must contain a nonzero coefficient |

Every axis must be nonempty. IDs must be unique within their own axis and must not be empty or whitespace-only. Identity is exact string equality: the boundary does not trim, normalise, infer, sort or realign names. Different axes may use the same string without making them the same entity. Matrix shapes must match the named axes. Invalid inputs return `LimmaError::Invalid` before reaching the corresponding numerical operation.

The selected method is unweighted least squares with more samples than design coefficients and a globally full-rank design. Every feature must have positive residual degrees of freedom. Per-feature rank loss caused by missing observations is supported when residual df remains positive: non-estimable feature/contrast pairs retain their NaN results and their original positions. All-missing features, zero-residual-df features, saturated designs and globally singular designs are explicitly outside v1. These are restrictions of this boundary, not claims that the existing low-level functions cannot represent those cases.

`InputScale::Log2Expression` serialises as `log2_expression`; `InputScale::Log2MethylationOdds` serialises as `log2_methylation_odds`. The caller declares the scale; this function performs no transformation and cannot establish the declaration's provenance. An estimate is the specified **linear contrast on the declared scale**. A disease-minus-control M-value contrast is a difference in log2 methylation odds. A slope or rescaled contrast requires its corresponding interpretation; the generic `estimate` field never automatically means an expression fold change.

There is no weights, block/correlation, robust/trend, preprocessing, filtering, chunking or joint-F option in this v1 interface. Use only the declared supported contract; another entry point or an unvalidated upstream transformation does not extend this qualification.

## Numerical operation and output

The boundary calls the existing `lm_fit_matrix`, `contrasts_fit`, `ebayes` and `adj_p_and_ci_margin` functions. eBayes uses `EBayesOptions::default()`: proportion 0.01, coefficient standard-deviation limits `[0.1, 4.0]`, robust false, trend false, no explicit span or legacy override, and default winsor tails `[0.05, 0.1]`. It preserves the existing numerical algorithms and diagnostics.

Moderation receives the entire supplied feature universe once. BH adjustment is applied separately to each complete contrast vector through the existing helper, retaining its R-compatible missing-value semantics. No significance filter is applied. Consumers must not split this operation into independent storage chunks or silently drop non-estimable features.

`NamedAnalysis` carries the declared input scale, ordered feature/sample/design/contrast IDs, original eBayes warnings and the result rows. Rows are **contrast-major, then original feature order**. Each feature appears once for every supplied contrast.

| Field | Meaning |
|---|---|
| `feature_id`, `contrast_id` | Exact input identities |
| `estimate` | Requested linear contrast on the declared input scale |
| `stdev_unscaled` | Unscaled design standard deviation; not an effect SE on its own |
| `s2_post` | Posterior residual variance from eBayes |
| `ordinary_se` | Residual sigma × `stdev_unscaled` |
| `moderated_se` | √`s2_post` × `stdev_unscaled` |
| `df_residual` | Feature's residual degrees of freedom |
| `df_total` | Total degrees of freedom used for moderated t inference |
| `t`, `p_value` | Existing moderated t statistic and two-sided raw p-value |
| `adj_p_value` | BH-adjusted p-value within that full contrast family |
| `ci_lower`, `ci_upper` | Two-sided 95% moderated t confidence interval |
| `non_estimable` | True when the fitted coefficient or unscaled design SD is NaN |

The estimability flag is not inferred from significance or from an arbitrary nonfinite downstream value. NaN and infinite numerical values are not coerced to zero or discarded. Warnings remain available at result level, in order, including any existing eBayes fallback warning. No joint F statistic is exported by this contract.

## Synthetic executable exchange

The example uses the existing CSV reference-fixture reader and already-present development dependencies. It is a qualification executable for the committed synthetic fixture format, **not a production data parser**. The reader fails on malformed fixtures; it accepts the reference `NA`/`NaN` spellings as float64 NaN and the named API then enforces the domain above. Clinical ingestion, source rights and manifest validation belong to the receiving project.

Run from this repository with the pinned toolchain, using a new output directory whose parent exists:

```sh
cargo run --locked --example export_named -- tests-validation/fresh-reference/complete artifacts/named-complete log2_expression
cargo run --locked --example export_named -- tests-validation/fresh-reference/missing artifacts/named-missing log2_expression
```

Each successful command creates exactly `estimates.csv` and `metadata.json` in its new directory. Existing output directories are rejected to preserve earlier evidence. A failed command does not qualify partial output. The same executable accepts `log2_methylation_odds` to transport that explicit declaration; relabelling a synthetic expression fixture does not establish methylation validity.

The CSV header and order are fixed:

```text
feature_id,contrast_id,estimate,stdev_unscaled,s2_post,ordinary_se,moderated_se,df_residual,df_total,t,p_value,adj_p_value,ci_lower,ci_upper,non_estimable,input_scale
```

Numeric cells use Rust's shortest float64 round-trip decimal representation, including `NaN`, `inf` and `-inf`; booleans are `true` or `false`. CSV quoting preserves opaque IDs. `input_scale` repeats the metadata declaration on every row.

The JSON metadata records schema version 1, contract ID, crate version, `purpose: synthetic_qualification`, input scale, all four ordered ID arrays, `layout: contrast_major_then_feature`, exact CSV columns, the fixed method settings and warnings. Metadata alone is not a source identity receipt: the caller must additionally retain the exact source commit and any candidate file hashes, toolchain, command, input/output file hashes, exit code and comparison results. The output contains no invented clinical metadata or implied admission flag.

## Independent evidence and receiving gate

The named reference tests use the unchanged [fresh independent generator](../tests-validation/scripts/fresh_reference.R) and its [pinned reference environment](../tests-validation/scripts/reference-environment.json): R 4.5.0, limma 3.68.5 and statmod 1.5.2. `complete` and `missing` independently cover the selected default eBayes route, including per-feature non-estimability. Historical positional fixtures remain separate evidence.

One independently generated scalar table has an explicit positional row convention:
`missing/ebayes_robFALSE_trendFALSE_scalars.csv` uses exactly `1` through `96`.
The pinned recipe directly combines the eBayes vectors in input feature order;
`write_mat` supplies sequential row names only when R's matrix has no row names.
The receiving reference reader may map only this exact file's complete sequential
index to the ordered input feature IDs. It must reject gaps, duplicates, reordering
or any other unnamed reference table; there is no blanket positional fallback.
The complete scalar file, missing fit scalars, other named oracle outputs and every
received exporter row retain strict feature-name alignment. Reference bytes are
unchanged.

The tests compare estimates and confidence bounds at absolute `1e-10`; unscaled SD, ordinary/moderated SE, posterior variance, t and residual df at relative `1e-8`; total df at relative `1e-6`; and p/BH p at `max(1e-12, 1e-8 × |reference|)`. Ordinary SE is checked against the independent R residual sigma and unscaled SD; moderated SE has its own independently exported reference. Existing tolerance policies and reference files are unchanged. NaNs/infinities must agree in kind and sign where applicable.

The executable round-trip tests parse every exported numerical field back to float64, require exact bits for non-NaN values, and preserve NaN, all IDs, row order, scales and warnings. Invalid-shape, ID, ordering, nonfinite-input, zero-contrast and unsupported-domain tests exercise rejection paths. Atlas must run this actual executable and validate the received files against this contract and the independent reference before recording its bounded synthetic consumer acceptance.

Cargo's native `test = true` declaration makes the example's two tests mandatory
in ordinary debug, release and instrumented test suites. Coverage has a narrower
reporting scope: pinned cargo-llvm-cov 0.8.7 excludes example paths by default, so
its LCOV/Cobertura reports do not establish exporter source-line coverage. The
named API is included. There is no independent native include-examples override;
disabling all defaults would also remove dynamic vendor, dependency, build,
toolchain, test and excluded-workspace filters. This increment keeps those native
defaults and states the limitation rather than duplicating their policy or
transforming reports. Exporter execution and round-trip assertions remain required.

This increment does not close actual-device execution, historical named-identity provenance, all-missing/zero-df/global-singular downstream qualification, production data exchange, preprocessing or scientific admission. Those remain in the existing LM01/LM02/LM03 and Atlas T07 gates.
