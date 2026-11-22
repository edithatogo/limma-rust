# LM03 — Methylation admission

## Overview

Determine whether the supported Rust path can replace the reference fitting step for a bounded atlas methylation analysis. Proteomics or synthetic agreement alone is insufficient.

## Additional prerequisites

The atlas supplies an eligible public dataset, a reuse decision, preprocessed values, sample/design metadata, feature universe, preprocessing version and hashes. A matched R reference consumes exactly those inputs and contrasts. Raw IDAT processing is not a task for this crate.

Select the initial atlas conditions only after accession, phenotype, controls and preprocessing have passed the atlas gates. Do not hardcode a disease or claim matched cohorts here merely because a dataset was suggested elsewhere.

## Experiment and acceptance

1. Lock one small processed study and a synthetic null/known-effect set.
2. Compare coefficients, actual moderated SE, posterior variance/df, t/F and p-values, BH, CIs, missingness/rank decisions and non-estimable outputs with the independent oracle.
3. Audit numerical disagreements near decision thresholds and evaluate false-positive behaviour on the null case. Correlation or overlap of top hits alone does not establish equivalence.
4. Exercise a second study or reserved subset with prespecified parameters; keep subject/study splits from leaking into method tuning.
5. Measure end-to-end time and peak memory on identical accepted inputs, including adapter overhead.
6. Record a bounded admit/reject decision for the exact method, design family, source and data domain.

Literature and software use are checked before and after the experiment by the atlas workflow. This work is normally a validation section or supplement bundled with the clinical analysis, not an automatic separate paper.

## Boundaries

No biological causal claim follows from backend equivalence. Cross-platform arrays, batch confounding, cell composition and age/tissue compatibility remain atlas design questions. Failure or delay here does not block the atlas's established reference analysis route.

## Clinical holdout boundary

The atlas must freeze its scientific and numerical protocol before disease-conditioned backend comparisons. Prefer unrelated technical/calibration fixtures and designated discovery data. A reserved numerical validation subset is not automatically untouched clinical replication. Keep the eventual clinical replication disease-effect outputs concealed until discovery choices are locked, or prespecify a blinded engineering-only parity check that cannot retune the scientific analysis. Record the exact role of each reserve.
