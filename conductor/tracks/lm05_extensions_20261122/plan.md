# LM05 plan

## Conditional executable batches

This track is activated by an actual named consumer gap, not by a desire to port
every limma method. It may unblock LM03; it does not depend on LM04.

| ID | Concrete action | Exit evidence | Depends on |
|---|---|---|---|
| LM05-1 | Reproduce the chosen consumer's unsupported input/method; distinguish an LM02 boundary/export fix from a new statistical method | Minimal synthetic example, expected behaviour and named consumer request | LM01 numerical baseline + LM02 boundary |
| LM05-2 | Review current primary sources in Rust > Julia > Python > R order, conditional on equivalence; include the existing independent R route | Dated capability/licence/dependency comparison and reuse/retain/implement decision, cross-linked to atlas N0/N1 | LM05-1 |
| LM05-3 | Freeze one method's inputs, outputs/errors, nonfinite/rank/tail semantics, reference recipe, tolerances, resource objective and rollback | Implementable contract and failing independent reference/regression case in the existing harness | LM05-2 selects bounded implementation |
| LM05-4 | Implement or integrate only the selected capability and run its targeted comparison plus `uv run --locked python scripts/verify.py` | No existing numerical regression; actual input/reference/tool/source receipts and measured dependency/adapter costs | LM05-3 |
| LM05-5 | Run the named consumer and relevant official-device jobs; atlas owner completes N2 and publication-bundling review | Bounded supported-capability statement or explicit rejection; LM03 resumes only for the validated domain | LM05-4 + required LM01 device evidence |

If reuse or retaining R solves LM05-2, record that decision and close this bounded
request without inventing an implementation task or a standalone paper.

## Phase 1 — Consumer gap and reuse

- [ ] Task: Identify a required consumer capability and the current blocker.
- [ ] Task: Check current software/literature and existing-language alternatives.
- [ ] Task: Choose reuse, a minor addition, a bounded method project or the reference fallback.
- [ ] Task: Define independent-reference and consumer acceptance before implementation.
- [ ] Task: Phase verification and checkpoint — no speculative method begins without a concrete need.

## Phase 2 — Conditional implementation

- [ ] Task: Write failing contract/reference cases for the selected capability.
- [ ] Task: Implement or integrate the smallest qualified solution.
- [ ] Task: Run affected numerical, consumer and resource gates.
- [ ] Task: Reassess literature, publication bundling and the capability claim.
- [ ] Task: Phase verification and checkpoint — advertise only the method and domain actually validated.
