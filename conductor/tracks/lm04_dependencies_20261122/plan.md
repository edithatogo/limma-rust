# LM04 plan

## Conditional executable batches

No current replacement is selected. Do not run this track before a recorded
maintenance defect or measured bottleneck demonstrates its value.

| ID | Concrete action | Evidence needed to advance | Depends on |
|---|---|---|---|
| LM04-1 | Pin current source and profile the actual named consumer workload; inspect all callers of the affected kernel | Reproduced defect or measured resource problem, affected paths and prespecified benefit target | LM01 numerical baseline + LM02 consumer |
| LM04-2 | Inspect one candidate's primary source/docs and compare the existing implementation; record exact version/features, licence, advisories, MSRV, transitive graph and rollback | Existing decision-ledger entry says why this candidate solves the observed problem; reject/defer is valid | LM04-1 |
| LM04-3 | Add missing rank/pivot/tail/nonfinite/consumer regression in the current harness before replacing one internal component | Demonstrated current contract and immutable reference tolerances; no multi-library or generic-backend rewrite | LM04-2 |
| LM04-4 | Implement the smallest reversible substitution and run the targeted checks, `uv run --locked python scripts/verify.py`, and identical-input measurements | Numerical and consumer results, compile/dependency cost, measured time/memory; no missing counter treated as zero | LM04-3 |
| LM04-5 | Review benefit against maintenance cost, then retain or revert the prototype | Exact accept/reject and rollback receipt; qualified device/domain claims only, with no changed oracle to mask a failure | LM04-4 |

Keep this work off the first atlas analysis's critical path. Unmet LM01 device
acceptance does not bar a labelled synthetic prototype, but does bar claiming
that the replacement is an accepted production backend.

## Phase 1 — Need and candidate decision

- [ ] Task: Confirm a concrete maintenance issue or measured resource bottleneck.
- [ ] Task: Search current primary sources for a bounded candidate and existing alternatives.
- [ ] Task: Record exact dependency, feature, licence, MSRV, caller and rollback details.
- [ ] Task: Phase verification and checkpoint — proceed only if expected benefit is specific and testable.

## Phase 2 — Bounded prototype

- [ ] Task: Add missing contract tests before substituting one internal component.
- [ ] Task: Implement the smallest reversible prototype.
- [ ] Task: Run affected reference/consumer gates and identical-input measurements.
- [ ] Task: Review with Ponytail and retain or reject based on evidence.
- [ ] Task: Phase verification and checkpoint — update the ledger and source-bound receipts without overstating qualification.
