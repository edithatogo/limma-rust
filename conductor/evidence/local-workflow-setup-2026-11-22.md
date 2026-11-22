# Official local workflow setup

Prepared 22 November 2026; strict parity updated in the same workstream. The [manual workflow](../../.github/workflows/local-qualification.yml) runs the identical strict native command as hosted CI: `uv run --locked python scripts/verify.py`. It includes locked debug/release tests, docs/lint, security checks, actual Rust/Python coverage and a 90% changed-line coverage gate against the previous reviewed main commit. It does not generate a fresh R oracle, qualify methylation data or benchmark performance.

## Required host setup

Install the official [actions/runner](https://github.com/actions/runner) through the repository's current GitHub runner setup instructions, using a dedicated clean account/environment. Register only the selected actual owner machine with its standard OS/architecture labels and the custom rare-omics-local label. Verify supported OS and runner version against current GitHub documentation.

The pinned official actions/checkout v6 revision is d23441a48e516b6c34aea4fa41551a30e30af803. Its Node24 runtime requires Actions Runner 2.327.1 or later. This workflow does not use authenticated Docker actions; that separate checkout feature has a newer runner requirement. Use a maintained current runner release rather than treating a minimum version as the preferred version.

Git and rustup must be available to the runner service account; on native Windows the supported compiler/linker/build tools must also be installed. The job installs the repository-pinned Rust 1.95.0 in isolated per-run temporary compiler directories, sets up Python 3.15.2 and uv 0.13.9, and synchronises the locked developer tooling. Cargo's manifest MSRV 1.85 is a separate compatibility claim, not tested by this lane.

## Invocation and evidence

Dispatch on trusted main with macos-arm64, windows-x64 or linux-x64. The job requires the owner repository and main ref; there is no pull-request trigger or arbitrary revision input. No user secret, patient data path or source-file upload is configured. Read-only permissions and disabled persisted checkout credentials remain. The job retains its source hashes, command logs and coverage reports for 14 days as a verification artifact; secret matches are redacted before logs are written. Personal-identifier and Git-history scanning remain the separate central privacy job.

The receipt records the checked-out source, candidate file hashes, compiler/tool versions, platform and per-command exit status. Record the actual run/job URL, official runner release, OS/architecture, selected device and test outcomes in LM01 before accepting the local lane. Add measured memory/timing only when actually collected; this workflow is not a performance benchmark.

Windows and Linux dual boot are separate OS qualifications on the same desktop. Neither substitutes for Apple ARM64. A configured workflow with no registered matching runner will queue; that is not a passed gate.

## Current observation

No owner device or authenticated runner-registration surface was available in this session. No runner was registered and this workflow was not dispatched. The already passed [cloud corpus replay](cloud-corpus-replay-2026-11-22.json) remains a separate Linux workspace baseline.

The workflow deliberately uses the checked-in public corpus: an empty MD_GOLDEN_CORPUS_DIR excludes a machine's private external override. Local-runner labels are routing criteria, not a security sandbox; do not expose a personal runner to untrusted public PR execution.
