# LM02a fixture hygiene and fresh named identity

Date: 2026-11-22 UTC. Implementation base: `85fd7c77b7be69cab38ec5fc799ec257db9c3785`.

## Bounded result

The shared CSV matrix reader now rejects duplicate, empty, and whitespace-only row or column identifiers. It preserves valid names exactly, including quoted commas, and preserves the existing column-major numeric and missing-value representation. A single shared named-input helper checks uniqueness, sample dimensions and exact sample order, followed by design/contrast coefficient dimensions and exact coefficient order. All four fresh R-reference cases use this helper, including the separate intentionally non-estimable contrast test.

There is no production-core, dependency, numerical-tolerance, or reference-data change. This increment does not introduce an atlas adapter or claim complete sample-identity qualification of the historical corpus.

## Demonstrated gaps before correction

`baseline-negative-identity.log` records a failing contract test against the original reader: all six duplicate/empty/blank row/column cases were incorrectly accepted. Its valid quoted-identifier and numeric-order control passed.

`baseline-corrupt-input-accepted.json` and its log record an actual run of the original `matrix_lmfit` integration tests against copies of the three historical matrix cases. The copies had reordered expression sample labels, reordered design row labels, an empty feature ID, and duplicate feature IDs; numeric values and expected results were unchanged. Both tests passed, demonstrating that numerical agreement alone did not protect fixture identity.

After the change, the same corrupted copies fail at the shared reader with `input_log2.csv: empty row ID at position 1`; see `corrupt-input-rejected.json` and its log. This result demonstrates rejection of the ambiguous IDs. It must not be interpreted as correction of the historical sample-order gap described below.

The permanent isolated tests cover all six ambiguous-axis cases, exact valid quoted names/numeric order, valid named alignment, reordered expression samples, reordered design rows, reordered contrast rows, unknown samples, unknown coefficients, and rejection of a generic ordinal-design fallback. The alignment negatives check the actual failure message. Fixtures are synthetic and created in isolated temporary directories without a new dependency.

## Historical fixture ceiling

`historical-identity-audit.json` records a read-only inspection of **531 named CSV matrices**: every existing axis was unique and nonempty. All **563 files** under the three reference/input roots were hashed before and after the increment and remain byte-identical, with an unchanged file set; see `fixture-hashes-before.json` and `integrity-after.json`.

The audit found that the four fresh cases have exact sample/design and design/contrast names, while these seven historical cases have named expression columns but default ordinal design rows (`1..N`):

- `matrix/conditions_only`
- `matrix/conditions_numeric_covariate`
- `matrix/conditions_categorical_numeric_covariates`
- `block/bojkova_pairs`
- `block/techrep_synth`
- `block/techrep_synth_na`
- `block/techrep_mixed`

The original `tests-validation/scripts/scalar_goldens.R` explicitly orders expression columns by `design_df$sample_name`, then constructs designs with `model.matrix` retaining ordinal rows. The original `block_goldens.R` constructs named expression columns and positional designs/blocks from the same condition/biological-sample vectors; `bojkova_pairs` reuses the historical conditions-only inputs. The audit binds these inspected recipes by SHA-256. Those recipes explain the positional provenance, but they do not turn the historical CSVs into independently named sample metadata.

Following the root's bounded scope decision and independent Ponytail consultation, the seven historical loaders retain their existing **positional numerical-reference** contract. They benefit from global ambiguous-axis rejection, but they do not invoke the fresh named-alignment helper and are **not fully sample-identity qualified**. Historical sample-label reordering therefore remains an explicit gap. No generic positional fallback, compatibility map, fixture relabeling, or new hashing framework was added. A later consumer/provenance increment must address that gap if those legacy fixtures are used to claim named sample alignment.

## Executed verification

All commands and measured durations are retained in the JSON receipts. These are observed cloud executions, not owner-device qualification or performance benchmarks.

| Check | Observed result | Seconds |
| --- | --- | ---: |
| Focused identity + fresh-reference tests | 9 passed, zero failures | 0.859 |
| Final format check | Passed | 1.715 |
| Clippy, all targets, warnings denied | Passed | 0.536 |
| Full debug suite | 113 passed, zero failed or ignored | 3.320 |
| Full release suite | 113 passed, zero failed or ignored | 18.618 |
| Repeated corrupted historical-input control | Expected exit 101; explicit empty-ID failure | 0.080 |

The configured commands use the already provisioned Rust **1.95.0** x86-64 toolchain, with both `CARGO_HOME` and `RUSTUP_HOME` set to the isolated bootstrap installation. An initial format command omitted the isolated `RUSTUP_HOME`, attempted a second toolchain installation and failed during extraction; its log and receipt are retained. Correcting the environment reused the installed toolchain and all final gates passed. This tooling failure was not a scientific-test failure or an omitted gate.

`integrity-after.json` binds the three changed test-source files, confirms the 563 reference files are unchanged, and records that the production/dependency diff is empty. Root source review and the [independent Ponytail review](independent-ponytail-review.md) accepted the bounded change. [LM02a is completed and archived](../../archive/lm02a_fixture_identity_20261122/index.md); parent LM02 remains in progress, and the production named-consumer API and official owner-device runner evidence remain separate gates.
