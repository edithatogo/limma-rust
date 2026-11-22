# LM02b executable plan

| Order | Action and real executable | Receipt and pass criterion | State |
|---|---|---|---|
| A | Run `uv run --locked python scripts/verify.py --output artifacts/verification/lm02-consumer-baseline` before implementation; read public API and the independent R recipe | 29 checks pass on clean `c88a1aa`; accepted strict PR then merges to `27ead66`, the next coverage base | Complete |
| B | Name Atlas `scripts/qualify_limma.py` as the synthetic receiver and select existing unweighted/default-eBayes entry points; freeze the single v1 contract | Explicit IDs/order/units/full-family testing and supported missingness; no inferred production/scientific acceptance | Complete |
| C | Add `tests/named_consumer.rs` before the module; run `cargo test --locked --test named_consumer` | Observed missing-public-module failure, followed by independent-reference and rejection regressions passing after implementation | Complete |
| D | Implement `src/consumer.rs` and `examples/export_named.rs`; run `cargo test --locked --test named_consumer` and `cargo test --locked --example export_named` | Six named tests and two real file exchange/refusal tests pass; unchanged kernels, references and dependencies | Complete |
| E | Freeze the entire candidate with the hook owner; run `uv run --locked python scripts/verify.py --output artifacts/verification/lm02b-complete` | 29 checks pass, 121 Rust tests per profile including the exporter, 25 Python tests; unchanged source, actual advisories/privacy and native coverage reports. Default example-path reporting exclusion is explicit. | Complete; 62.14 s |
| F | Review the actual diff, receipts and unchanged scientific boundaries; archive only the bounded producer acceptance | Both independent peer source reviews accepted; E passed; Atlas receiving remains explicit parent LM02-5 follow-up | Complete |
| Handoff | Coordinator stages the reviewed complete candidate and makes the actual hook-enforced commit, then publishes its source pin | Actual native hook result and local/remote source mapping recorded by coordinating owner; no bypass | Pending: coordinator |
| Parent LM02-5 | Atlas invokes `cargo run --locked --example export_named -- FIXTURE_DIR NEW_OUTPUT_DIR log2_expression` for complete and missing, then validates `estimates.csv`/`metadata.json` | Atlas's real qualification receipt binds the published provider source, caller revision, inputs/outputs, independent comparisons, no dropped rows, warnings and unsupported cases | Pending: Atlas receiver owner |

The output parent directory must exist and the selected output directory must be
new. `FIXTURE_DIR` is one of the committed independent fresh-reference case
directories, not a historical positional fixture or clinical dataset.

The provider uses only existing dependencies; the exporter is a development
example with `test = true` so Cargo's default test and coverage commands execute
its tests. Atlas consumes files through its own qualification executable and does
not add a linked numerical dependency for this increment. Rollback removes the
new public module/exporter and restores the previous receiver selection; existing
numerical APIs and reference files require no migration.

Keep parent LM02-5 pending until the actual receiving command runs. Archiving the
bounded producer does not equate its tests with an Atlas run. No task here closes
actual owner-device execution, historical names, clinical/preprocessing eligibility
or the server-side required-check enforcement hold.
