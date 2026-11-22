# LM02b specification

## Outcome

Implement the smallest named consumer required by Atlas T07's synthetic
qualification caller. Keep the source-reviewed sequence `lm_fit_matrix` →
`contrasts_fit` → default `ebayes` → existing BH/CI helper. The canonical
versioned fields, domain and tolerances live only in
[named consumer v1](../../../docs/named-consumer-v1.md).

## Required acceptance

- Explicit unique axes, exact sample/coefficient alignment, column-major float64,
  declared input scale and complete output feature universe.
- Clear errors for invalid shapes, ambiguous/misaligned IDs, infinities/nonfinite
  model values, zero contrasts and domains excluded from v1.
- Independent complete/missing R comparisons for estimates, ordinary/moderated
  uncertainty, df, t, raw/BH probabilities and 95% CI; preserved non-estimability
  and diagnostic warnings.
- Synthetic executable output preserves float64, NaN, names, order and metadata;
  prior evidence cannot be overwritten by rerunning into an existing directory.
- Full existing strict gate passes on the reviewed candidate, including the
  exporter example tests selected by Cargo's native `test = true` target setting.
  No changed numerical kernels, reference bytes, tolerances or core dependencies.
- Source review accepts the bounded producer, and the native commit hook is
  installed and independently proven. The coordinator's actual publishing commit
  must still pass that hook; do not claim its outcome before execution.

## Coordinating receiving follow-up

Parent LM02-5 requires Atlas to invoke the published provider pin, verify the
actual exported artifact against the independent reference and record exact
source/input/output identity. This follow-up is explicitly pending at bounded
producer acceptance; it is not replaced by the example's own round-trip tests.

## Boundaries

This increment accepts only the synthetic producer under the specified v1 domain.
Global singular designs, all-missing/zero-df features, unselected weighting/block
or robust/trend methods and joint F export remain outside that contract. Existing
low-level functions and their reference evidence remain intact.

Atlas owns ingestion, provenance, study/ontology semantics, storage,
preprocessing and scientific admission. Limma owns the numerical contract.
Central standards own scanner policy. The historical seven positional cases do
not establish named sample identity for this increment. Official actual-device,
production ingestion and LM03 science remain separate gates.
