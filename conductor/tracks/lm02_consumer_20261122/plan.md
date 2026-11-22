# LM02 plan

## Phase 1 — Specify and test the boundary

- [ ] Task: Confirm LM01 numerical baseline and select the actual atlas consumer.
- [ ] Task: Specify supported entry points, ordered input/output schema and error/missingness contract.
- [ ] Task: Write failing fixture-identity and supported-interface cases for demonstrated gaps.
- [ ] Task: Phase verification and checkpoint — agree the smallest change from observed failures.

## Phase 2 — Minimal implementation and acceptance

- [ ] Task: Reuse a shared strict fixture loader and existing error machinery.
- [ ] Task: Implement only required validation/export additions, keeping storage outside the core.
- [ ] Task: Run targeted cases, existing reference comparisons and the consumer round trip.
- [ ] Task: Record source, schema, dependency impact, results and rollback.
- [ ] Task: Phase verification and checkpoint — the named consumer contract passes without a scientific behaviour regression.
