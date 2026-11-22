# LM01 — Reproducible baseline

## Overview

Establish which results can actually be reproduced on the current source before changing numerical behaviour. Keep committed-reference replay, fresh independent R output and user-device execution as distinct gates.

## Requirements

1. Record an exact source/patch and fixture identity, compiler, target and command for existing fmt/clippy/debug/release checks. A cloud smoke is useful but labelled as such.
2. Inventory the existing comparisons rather than duplicating them. Reuse the corpus and harness.
3. Make a bounded independent synthetic R oracle reproducible without a private full corpus or an undocumented local image.
4. Resolve the actual official Actions runner/device prerequisite before scheduling local acceptance. Document OS/architecture, exact runner version, toolchain and trusted-source execution. Do not infer hardware specifications.
5. Reconcile corpus R 4.5.0 / limma 3.68.5 / statmod 1.5.2 with other source-version annotations.

## Acceptance cases

Existing embedded references, scalar tail/quantile grids, missingness and rank changes, contrasts, all robust/trend combinations, posterior variance, t/F and p-values, BH, CIs, and relevant block cases must retain their prescribed tolerances and nonfinite semantics.

The independent synthetic oracle covers a minimal fit/contrast/eBayes path plus missingness-induced rank loss and non-estimable output. It records generated values, expected errors, versions, hashes and generator recipe. Fresh R execution must be observed before the oracle gate passes.

Official Mac/Windows receipts must identify the reviewed source and actual completed job; they cannot be replaced by a cloud shell or upstream Buildkite result. Cross-platform numerical tolerances are specified before evaluation.

## Boundaries

No new statistical method or general test framework. Historical parity numbers remain historical. If a baseline fails, record and minimise the failure; do not silently update expected values or relax bounds.
