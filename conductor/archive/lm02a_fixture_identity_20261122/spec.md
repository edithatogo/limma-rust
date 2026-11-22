# Fixture identifier hygiene and fresh named identity

## Agreed bounded scope

Reject duplicate, empty, and whitespace-only row/column identifiers in the shared CSV matrix reader. Reuse one existing-harness helper for exact input-sample/design-row and design-column/contrast-row alignment in the four fresh R-reference cases. Add meaningful negative cases before correction, preserving numerical algorithms, dependencies, tolerances, and every original reference byte.

Inspection of the original R recipes established that seven historical cases use named expression columns but default ordinal design rows. The scope was narrowed before acceptance: those cases retain their positional numerical-reference contract and receive shared identifier hygiene checks, but remain unqualified for named sample alignment. This increment must not silently reorder or rename them, invent a generic ordinal fallback, or add a compatibility map merely to make the original broad acceptance statement pass.

## Acceptance

- Duplicate, empty, and whitespace-only axis identifiers fail in the shared reader.
- Reordered or unknown sample/coefficient names and an unmapped ordinal design fail the strict named helper; valid fresh named inputs pass.
- Historical and fresh numerical reference cases pass their existing contracts without changed reference bytes, core code, dependencies, or tolerances.
- Focused negatives, full debug/release suites, formatting, and Clippy pass with recorded commands and observed durations.
- Root source review and independent Ponytail review accept the bounded change before archival.

## Remaining boundaries

Historical sample-order/provenance qualification, the production named consumer API, exchange round trips, and official owner-device runner qualification remain open in their parent tracks. See the [evidence and historical ceiling](../../evidence/fixture-identity-2026-11-22/README.md) and [independent review](../../evidence/fixture-identity-2026-11-22/independent-ponytail-review.md).
