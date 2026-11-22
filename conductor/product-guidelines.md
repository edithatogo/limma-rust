# Product guidelines

Use precise method names, supported input domains and source-bound results. Distinguish implemented, inspected, executed, compared against a fresh oracle and qualified for a named consumer.

Explain errors in terms a caller can act on: dimensions, sample order, design rank, non-estimable contrast or unsupported input. Preserve error texts already covered by reference fixtures unless a documented compatibility decision changes them.

Use explicit units. The generic expression-oriented field names inherited from limma do not convert methylation effects into RNA expression effects.

Prefer a small documented API and existing test machinery over new frameworks. Keep developer setup separate from patient data, credentials and publication artefacts.

Keep the GPL licence and provenance of translated R algorithms. Open accessibility of a validation dataset is not itself permission to redistribute it.

Document the limitation of each result beside the result. No performance claim is made without measurements on identical accepted inputs, and no correctness claim is inferred from speed.
