# Project Workflow

Adapted from the pinned official Conductor workflow for this numerical Rust library and the user's authorised context-first setup. The original template remains in the submodule. Its web/mobile examples, blanket coverage quota, per-step approval prompts and automatic commit/git-notes sequence are not applicable acceptance gates for this work.

## Guiding principles

1. Work from the selected track's specification and next eligible task.
2. Preserve statistical behaviour before pursuing performance or fewer lines.
3. Use existing code and dependencies first.
4. Qualify exact source, toolchain, reference, consumer and resource claims separately.
5. Keep initial documentation/setup separate from implementation and runner registration.
6. Record blockers before downstream work; allow the atlas reference route to proceed independently.

## Task lifecycle

1. Read the context, dependencies, relevant implementation and existing tests.
2. Mark the selected task `[~]`; record any unfulfilled prerequisite with an owner and revisit condition.
3. For a runtime behaviour change, write a failing test that expresses the intended contract, reproduce the failure, implement the smallest fix and run the targeted regression. Extend the existing golden harness rather than create a parallel framework.
4. For a documentation change, check source accuracy, relative links, JSON and whitespace. Do not create numerical tests solely for prose.
5. Run broader checks only to cover the affected callers or a defined gate. Keep scientific-output tolerances fixed unless an explicit, justified method decision changes them.
6. Record command, source revision and dirty patch identity, toolchain/target, reference and input hashes, tolerance, result and limitations. Distinguish a command that was proposed from a command that ran.
7. Apply Ponytail review to the actual change: smallest scope, existing-library reuse, scientific contract, dependency/build cost, risk, rollback and avoidable custom code.
8. Mark completion only when that task's acceptance is evidenced. The coordinating change owns commits and publication; do not invent a commit ID or require an empty checkpoint commit.

Track metadata records completed, in-progress or new work. Individual evidence records use `pass`, `fail`, `not_run` or `blocked`. A completed documentation track is not a passed scientific qualification.

## Phase verification and checkpoints

At each phase boundary, inspect the changed files against its specification, validate the relevant artefacts and record what remains unverified. Each plan contains a phase verification task. No extra user confirmation is needed for already authorised reversible checks; ask only when an actual new decision or external authority is needed. A pending runtime gate remains pending even if all documents are valid.

## Official local runner requirement

Device acceptance uses GitHub's official `actions/runner` on the intended Mac or Windows system. First record availability, runner version, OS/architecture, CPU/RAM, toolchain and supported job requirements. A maintainer-controlled workflow must run reviewed code with appropriately scoped permissions; persistent personal runners must not execute arbitrary untrusted pull-request code.

Installing/registering a runner and demonstrating a completed job are distinct actions. Source checkouts, hosted CI, Buildkite and local emulation do not prove official local-runner acceptance. LM01 owns this prerequisite; no registration, credentials or live machine access are implied by Conductor setup.

## Existing commands

The first and final implementation checks use the same strict native command as
hosted CI. Git, Python 3.11+ and rustup/native linker are prerequisites:

```sh
python scripts/bootstrap.py
uv sync --locked --python 3.15.2
uv run --locked python scripts/verify.py
```

The command is fail-closed for missing/wrong tool versions, documents, workflow
syntax, fmt, Clippy, debug/release tests, Rustdoc warnings, dependency advisories,
empty/missing coverage and source secrets. Source content hashes include tracked
and visible pending files; per-command statuses never replace earlier failures.
It clears corpus overrides before the ordinary gate. Python 3.15.2 and uv 0.13.9
are the isolated tooling candidate qualified by these exact script tests; they
are not a numerical-core dependency. Developer tools are hash-locked in `uv.lock`.
Ruff, strict Pyright, Python branch coverage and the tooling dependency audit run
alongside the native checks. Coverage uploads include both language reports.

The shared central privacy workflow remains an independent gate. Source-snapshot
Gitleaks is useful before upload but does not prove complete Git-history or
personal-identifier coverage. Codecov OIDC uses project target `auto`, threshold
`1%`, and patch target `90%` from `template-solo-rust`; no new blanket whole-crate
coverage quota replaces the numerical reference tests. A configured check is not
proof that branch protection requires it or that its hosted execution passed.

These smaller commands are diagnostic subsets, never a substitute for the full gate:

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo test --locked --release
```

An explicitly selected older compiler can test the manifest MSRV; it is reported as a separate compatibility experiment, not relabelled pinned-toolchain validation. The committed tests need no R process. Keep `MD_GOLDEN_CORPUS_DIR` unset for the committed corpus; use it only for an identified, hashed external corpus.

`tests-validation/scripts/report` uses its documented Docker/Linux environment and a historical report-drift gate. Read the script before use. Exact last-digit report agreement from that platform is not an appropriate blanket Mac/Windows acceptance rule.

Regenerated R reference output is a separate gate with pinned R, limma and statmod versions. A Rust test against old expected files is not a new R execution.

## Dependency and performance gate

Follow [the decision ledger](decisions.md) and the referenced central standards. Record package version and features, licence/advisories, MSRV, dependency tree, build/resource cost, affected callers and rollback. Benchmark an identical workload only when performance is a reason for the change. Missing resource measurements remain unknown.

Keep core, adapters, optional backends, build-only tooling, isolated test oracle and development dependencies separate. Require source/build/numerical/consumer/resource evidence before calling a replacement qualified for the atlas.

## Definition of done

A task is done when its own specified outcome is present and verified, its evidence and limitations are recorded, and the plan reflects its actual state. No scientific eligibility or device performance follows automatically from context setup, a successful build or a synthetic smoke test.
