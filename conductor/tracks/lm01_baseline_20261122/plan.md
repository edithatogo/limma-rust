# LM01 plan

## Phase 1 — Inventory and executable baseline

- [x] Task: Record the committed corpus, existing comparisons and regeneration gaps.
- [x] Task: Separate cloud smoke, pinned toolchain, MSRV, fresh oracle and local-device evidence.
- [x] Task: Replay the existing committed debug corpus on exact source with pinned Rust 1.95.0 in the cloud workspace.
- [x] Task: Run formatting, clippy and release-mode gates in the cloud workspace; all passed with a [separate receipt](../../evidence/cloud-static-release-checks-2026-11-22.json). This is not local-device acceptance.
- [x] Task: Inspect the debug test result and retain logs, source/fixture hashes and terminal results.
- [x] Task: Phase verification and checkpoint — debug/release cloud baseline, lint and exact source receipts reviewed; device evidence remains separate.

## Phase 2 — Independent reference

- [x] Task: Resolve a public, pinned R/limma/statmod environment and document its provenance.
- [x] Task: Specify the minimal independent synthetic cases and outputs using the existing harness.
- [x] Task: Add the smallest generation recipe and its contract checks when implementation begins.
- [x] Task: Regenerate the oracle, replay Rust and record numerical agreement.
- [x] Task: Phase verification and checkpoint — confirm independence and reproducibility, retaining failures.

## Phase 3 — Official local runner

- [ ] Task: Verify available Mac/Windows machines and official runner prerequisites.
- [x] Task: Configure a trusted manual official-local-runner workflow and pass static actionlint validation; see [setup](../../evidence/local-workflow-setup-2026-11-22.md).
- [ ] Task: Register only the intended actual runner with the necessary authority and safe job boundary.
- [ ] Task: Run reviewed-source baseline jobs and record actual device receipts.
- [ ] Task: Phase verification and checkpoint — distinguish device acceptance from cloud smoke and reference regeneration.

See [baseline status](../../evidence/baseline-status-2026-11-22.md) for the initially unexecuted or blocked gates.

Cloud debug replay: 104 passed, zero failed/ignored, terminal exit 0. See [receipt](../../evidence/cloud-corpus-replay-2026-11-22.json). This completes that subtask only; LM01 remains in progress.

Fresh public synthetic reference: [LM01a archive](../../archive/lm01a_reference_20261122/index.md), [receipt](../../evidence/fresh-reference-2026-11-22/receipt.json) and [independent review](../../evidence/fresh-reference-ponytail-review-2026-11-22.md). This closes the bounded fresh-reference increment; it does not regenerate every historical/private corpus component or qualify a methylation consumer.
