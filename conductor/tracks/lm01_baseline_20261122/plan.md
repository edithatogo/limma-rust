# LM01 plan

## Next executable batches

Run these in the stated order. Each batch has one writer and records the exact
source/dirty patch, command, exit code and a receipt before the next dependent
batch starts. A missing machine is a device blocker, not a reason to stop a
synthetic contract increment whose prerequisites already passed.

| ID | Input and concrete action | Exit evidence | State |
|---|---|---|---|
| LM01-CI1 | Inspect `scripts/bootstrap.py`, `scripts/verify.py`, workflow pins and central/template policy; run `python scripts/bootstrap.py`, sync locked tooling, then `uv run --locked python scripts/verify.py` on the current candidate | All 29 strict checks passed, including actual 97% changed-line coverage; [source-bound receipt](../../evidence/strict-ci-2026-11-22.md), no core/reference/tolerance change | Complete: bounded cloud code increment |
| LM01-CI2 | Push the reviewed candidate through the coordinating lane; inspect `Strict native checks`, `privacy-and-secrets` and Codecov results on that exact revision | Hosted run URLs and terminal conclusions, including any unavailable provider or required-rule gap | Native/privacy/Codecov passed on `9f7ae4a1`; fork API correction pending; required settings still absent |
| LM01-D1 | Inventory intended Mac/Windows machines and installed official runner release using the owner's existing access; verify labels and native toolchain/linker prerequisites | Device, OS/architecture, runner release, Git/rustup/Python availability and reviewer-approved main revision; otherwise named owner/revisit condition | Blocked: no machine/runner surface |
| LM01-D2 | After D1, dispatch `local-qualification.yml` on reviewed `main` for each actual target; the workflow installs pinned tools and calls the identical strict command | Completed run/job URL plus retained receipt and native OS/device identity for each required target | Blocked by D1 |
| LM01-D3 | Review D2 results against the existing numerical tolerances and reference hashes; record device-specific failures without editing expectations | Explicit per-device accept/return decision; LM01 closes only when its actual-device requirements are met | Blocked by D2 |

CI1/CI2 concern code qualification. Neither claims a fresh R generation, clinical
admission or owner-device execution. Preserve the completed phases below as
historical evidence rather than replaying them unnecessarily.

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
