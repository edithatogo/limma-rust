# LM02 plan

Status: in progress. The completed [LM02a bounded increment](../../archive/lm02a_fixture_identity_20261122/index.md) supplies fixture identifier hygiene and fresh named alignment. Historical sample provenance, production consumption, exchange, and device admission remain open.

## Remaining execution order

| ID | Input and exact work | Required output/check | Prerequisite |
|---|---|---|---|
| LM02-1 | Read the atlas T07 selected-consumer request and `src/lib.rs`, `fit`, `contrasts`, `ebayes`, `toptable`; name the real caller and supported entry points | Small versioned input/output contract with identifiers, column-major float64 order, units, shapes, design/contrast names, missingness/errors and actual moderated SE | Completed LM01a/LM02a; strict code baseline |
| LM02-2 | Map the seven historical positional cases to available source provenance; choose explicit exclusion from named-identity claims if names cannot be recovered | Provenance table and named owner/revisit condition; historical numerical reference values unchanged | LM02-1 |
| LM02-3 | Add only missing tests for the chosen production boundary in the existing harness: reorder/duplicate/missing IDs, invalid dimensions/design, all-missing/zero-df/rank-loss outputs | New tests fail before the boundary fix for the stated reason; retain the failing command/result | LM02-1; LM02-2 if relying on historical names |
| LM02-4 | Implement one thin validation/export boundary; preserve `sqrt(s2_post) * stdev_unscaled` and place storage/ontology work in the atlas | New regressions pass; `uv run --locked python scripts/verify.py` passes without changed goldens/tolerances or unneeded dependencies | LM02-3 |
| LM02-5 | Round-trip the actual atlas consumer's chosen exchange format using synthetic named data and run its own acceptance command | Exact adapter/schema revision, float64 and nonfinite preservation, order checks, dependency/build impact and rollback; receiving acknowledgement in the coordinating lane | LM02-4 |
| LM02-6 | Review all prior receipts and update both repositories' dispositions | Bounded consumer accept/return; LM01 device and LM03 scientific admission remain separately visible | LM02-5 |

LM02-1 is the next eligible scientific-engineering batch after strict code
qualification. LM01 device provisioning may progress independently; device
acceptance is mandatory before advertising the atlas production backend.

## Phase 1 — Specify and test the boundary

- [x] Task: Confirm the cloud numerical baseline and completed [LM01a fresh-reference milestone](../../archive/lm01a_reference_20261122/index.md).
- [ ] Task: Select the actual atlas consumer.
- [ ] Task: Specify supported entry points, ordered input/output schema and error/missingness contract.
- [x] Task: Reproduce ambiguous fixture identifiers and add strict fresh named-alignment negatives in LM02a.
- [ ] Task: Write remaining supported production-interface cases for demonstrated consumer gaps.
- [x] Task: Review the smallest bounded fixture change; preserve the seven historical positional cases without asserting named sample qualification.
- [ ] Task: Phase checkpoint — agree the actual consumer boundary and required historical provenance evidence.

## Phase 2 — Minimal implementation and acceptance

- [x] Task: Reuse the shared reader for unique/nonempty axes and one strict fresh named-input helper.
- [ ] Task: Resolve historical sample-identity/provenance requirements before using the seven positional cases to claim named consumer qualification.
- [ ] Task: Implement only required validation/export additions, keeping storage outside the core.
- [x] Task: Run LM02a targeted checks and existing full reference suites; retain [source and reference integrity](../../evidence/fixture-identity-2026-11-22/integrity-after.json) and [accepted review](../../evidence/fixture-identity-2026-11-22/independent-ponytail-review.md).
- [ ] Task: Execute the actual consumer round trip and record its schema, dependency impact, outcomes, and rollback.
- [ ] Task: Phase verification and checkpoint — the named consumer contract passes without a scientific behaviour regression.

The separate official owner-device runner gate in [LM01](../lm01_baseline_20261122/index.md) remains required before atlas production-backend admission. Cloud harness checks do not close it.
