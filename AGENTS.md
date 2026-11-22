# Working on this fork

Read [the Conductor index](conductor/index.md), the selected track's specification and plan, and the source files touched by a change before implementation.

## Identity and scope

This repository is Dylan's fork of **MassDynamics/limma-rust**. Its package is `limma-core`; the audited upstream base is `ba41fb6bbf6e3f2524bd44f5be017c827d5c4aaa`. It is not `sinmojito/limma-rust`. Findings about another implementation must be reproduced here before they become work items. Preserve GPL-3.0-or-later and upstream attribution.

The current programme establishes provenance, executable reference comparisons and a small consumer contract before considering methylation admission or replacements of numerical kernels. Conductor setup and plans do not qualify this crate for scientific analysis. Never present an upstream report, documentation inspection or cloud smoke run as a fresh R comparison or acceptance on Dylan's computers.

## Change discipline

- Follow the dependency order in [tracks.md](conductor/tracks.md). Resolve an actual prerequisite or record its owner and revisit condition; do not start a downstream implementation that requires unavailable evidence.
- Keep the statistical core small. Use the standard library and existing dependencies first. Add a dependency only for an identified need with a recorded benefit, compatibility checks and rollback. Do not put Polars, Arrow/Parquet, Python, plotting or GPU dependencies in the core merely because the atlas uses them elsewhere.
- Reuse the existing test harness. A scientific behaviour change needs a failing contract/reference case before the fix and appropriate regression checks afterward. Documentation-only work needs document checks, not artificial numerical tests.
- Preserve the R numerical contract, including tail probabilities, nonfinite values, rank/pivot semantics and non-estimable coefficients. A generic QR or probability library is not an equivalent replacement merely because the method name matches.
- Record exact source revisions, toolchain, command, target, fixture/reference hashes, tolerances and actual outcomes. Use `not_run`, `blocked` or `failed` where appropriate; missing counters are unknown, not zero.
- No new scientific implementation is part of the initial context setup. Future implementation follows the relevant track. Do not fabricate completion by relaxing acceptance criteria.
- Keep patient-level data out of this repository. Use synthetic fixtures or separately cleared public data for tests; access and redistribution are separate decisions.

## Language and interfaces

The programme's preference is **Rust, then Julia, then Python, then R**, conditional on scientific equivalence and maintainability. R remains an isolated reference oracle or an established atlas analysis backend until a replacement passes its own contract; it is not a runtime dependency of this core. The upstream Python adapter lives elsewhere and is not required here.

The atlas owns identifiers, ontologies, study metadata, Arrow/Parquet exchange and preprocessing. This core receives explicitly ordered numeric matrices. The consumer must validate names, shape, feature universe, design, contrasts, missingness and units. M-value coefficients represent differences in log methylation odds; they must not be described as RNA expression fold changes.

## Review and verification

Use Ponytail's dependency-first, source-based review for each adopted change. This records a review decision; it is not an independent numerical certificate or a claim that a separate service approved the science.

Local development acceptance uses **GitHub's official Actions runner** on the intended device. A cloud shell, an upstream Buildkite result or an emulator does not satisfy that condition. LM01 records the prerequisites and distinct evidence classes. Do not register a runner, change repository security settings or expose a persistent runner to untrusted public pull-request code as an incidental documentation task.

The [workflow](conductor/workflow.md) adopts the relevant parts of Dylan's [scientific/native standard](https://github.com/edithatogo/repository-standards/blob/a8b675feeb62cb39ba62cd3ab03cb5deaf281ad0/docs/scientific-native.md) and [Rust dependency selection standard](https://github.com/edithatogo/repository-standards/blob/a8b675feeb62cb39ba62cd3ab03cb5deaf281ad0/docs/rust-dependency-selection-standard.md). Their unrelated product-specific requirements do not become dependencies here.
