# LM02 — Consumer contract and bounded validation

## Overview

Make the existing numerical core safe to consume from a named atlas adapter. Correct demonstrated interface gaps while keeping tabular storage, ontology logic and preprocessing outside the crate.

## Contract

Input has explicit feature/sample order, dimensions, column-major float64 values, finite-value/missingness policy, named design columns and contrasts, feature universe and provenance. The caller validates unique IDs and exact alignment before numeric conversion. A reordered sample table cannot silently change the model.

Output identifies feature and contrast and preserves estimate, unscaled standard deviation, posterior variance, moderated standard error, residual/total degrees of freedom, t/F statistics where applicable, raw/adjusted p-values and non-estimable state. Moderate SE is computed from existing quantities as sqrt(s2_post) × stdev_unscaled; M-value effects have methylation-log-odds units.

## Acceptance cases

- Equal dimensions with reordered sample labels must be explicitly realigned by an adapter or rejected, never silently accepted as a different design.
- Duplicate/missing feature, sample, design or contrast names fail the supported identity contract.
- Existing fixture loaders share strict checks for row/column identity and schema.
- Invalid supported API shapes, empty/unsupported designs, nonfinite design and correlation-domain errors have documented outcomes without accidental indexing panics.
- All-missing features, zero residual df, rank loss and non-estimable coefficients preserve the agreed reference behaviour.
- Exported moderated SE agrees with CI/t-statistic calculations where defined; float64, missingness and row order survive the agreed exchange round trip.
- Existing numerical goldens remain unchanged within their original tolerances.

## Smallest implementation

Use one existing-harness helper and one thin consumer boundary. Decide whether an observed low-level issue needs validation, restricted visibility or a documented precondition before changing APIs. Arrow/Parquet/Polars are optional adapter choices, not mandatory core dependencies.

## Out of scope

Raw-array processing, sample matching by heuristic, reimplementation of statistical algorithms and a generic new workflow framework.
