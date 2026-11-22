# LM02a independent Ponytail review

Date: 22 November 2026 UTC. Reviewed implementation base: `85fd7c77b7be69cab38ec5fc799ec257db9c3785` plus the three source files identified below.

**Decision: accept the agreed bounded increment.** The shared reader rejects ambiguous axis identifiers, and all four fresh reference cases require exact named input/design/contrast alignment. No material implementation finding remains. The seven historical cases retain their explicitly documented positional numerical-reference contract; this review does not close that sample-identity gap. The archived specification/closure must state this narrower scope rather than imply universal named identity.

## Source and caller review

The reviewer inspected `tests/common/matrix.rs`, `tests/fixture_identity.rs`, the complete relevant `tests/fresh_reference.rs` load/inference paths, and every `Matrix::read`/`assert_model_identity` caller. The new validation remains in the existing test harness; production functions continue to receive ordered numeric slices.

The reader checks every parsed row/column name for an empty or whitespace-only value and for exact duplicates. It preserves valid identifier bytes rather than trimming or normalizing them. Numeric values, CSV quoting, column-major placement and existing nonfinite parsing remain unchanged. The header check now gives a controlled assertion for an absent header instead of indexing an empty vector.

The shared named helper checks input/design sample dimensions and exact name order, then design/contrast coefficient dimensions and exact name order. All four fresh fit cases and the two fresh inference cases call it before numerical evaluation through their common loader. The intentionally non-estimable contrast case additionally validates the separate invalid contrast's coefficient names before asking the numerical core to reject its estimability. No automatic reorder, ordinal fallback or compatibility map was introduced.

The historical `matrix_lmfit`, `matrix_ebayes` and block callers inherit ambiguous-axis rejection through `Matrix::read`; they do not invoke the named sample-alignment helper. This matches the disclosed scope. Their numerical test success must not be described as named sample-identity qualification. Expected-output matrices also receive the shared axis hygiene check; the full-suite evidence shows that the existing references satisfy it.

## Evidence independently checked

| Check | Reviewer observation |
|---|---|
| Focused execution | `cargo +1.95.0 test --locked --test fixture_identity --test fresh_reference`: **9 passed, 0 failed, 0 ignored**, observed process duration **0.11950037100177724 seconds** |
| Runtime selection | Existing isolated Rust 1.95.0 toolchain; both `CARGO_HOME` and `RUSTUP_HOME` set; both corpus-override variables explicitly unset |
| Frozen data | Independently recomputed all **563** file hashes under the three reference/input roots; exact match to the before manifest, with no file-set change |
| Source integrity | All three current source hashes match `integrity-after.json`; inspected original R recipe hashes also match |
| Case identity | Independently parsed all **11** input/design/contrast triples: four fresh cases have exact named alignment; seven historical cases retain ordinal design rows |
| Production/dependency boundary | `git diff --name-only -- src Cargo.toml Cargo.lock` is empty |
| Recorded full suites | Independently summed the implementation logs: **113 passed, 0 failed, 0 ignored** in each debug and release run |
| Failure evidence | Read the original-reader failure on six ambiguous-axis cases, the original historical-loader acceptance of corrupted copies, and the corrected reader's explicit empty-ID rejection |

The independent command/elapsed duration and complete output are retained in [independent-targeted.json](independent-targeted.json) and [independent-targeted.log](independent-targeted.log). This reviewer did not repeat the entire debug/release suites, Clippy, formatting or R regeneration. The implementation's recorded full-suite/fmt/Clippy results were inspected rather than relabelled as new independent executions.

The permanent identity tests reject reordered input samples, reordered design rows, reordered contrast rows, foreign sample/coefficient names and unmapped ordinal designs. They check the expected alignment failure message. A valid quoted-identifier/missing-value control confirms that intended data representation is preserved. The duplicate/empty/blank tests demonstrated failures before correction and now pass.

The compound historical corruption experiment fails after correction at its empty row identifier. It establishes the new ambiguous-ID boundary only. The evidence README correctly states that it does not prove historical sample-order validation. The initial formatting/toolchain-installation failure is retained alongside the corrected environment's passing checks; no failure was silently discarded.

## Ponytail assessment and remaining boundaries

The change uses the standard-library `HashSet`, the existing CSV dependency and the existing numerical reference harness. It adds no runtime dependency, hashing engine, compatibility framework, new data format or numerical algorithm. The temporary test-fixture utility is small and confined to the tests. **Complexity-only result: Lean already. Ship.**

Preserving historical source bytes while stating their limitation is appropriate for this increment. The original R recipes explain how those positional inputs were constructed; they do not recover an independent named sample manifest. A future production consumer or historical-provenance track must supply its own evidence if it needs that claim.

This acceptance is limited to harness identifier hygiene and the fresh named fixtures. It does not validate arbitrary user-supplied matrices, methylation units, downstream clinical analyses, the full proposed consumer API or an official GitHub Actions run on the owner's Mac/Windows devices. No performance claim follows from the observed focused-test duration.

## Reviewed source identity

| File | SHA-256 |
|---|---|
| `tests/common/matrix.rs` | `5a7a392185a4d2ac51914b625ec1d676f29a2c7e8d135602e08833edef834058` |
| `tests/fresh_reference.rs` | `b7aecf0bee90fde096a15b2f016c76197b08b82a2952f385199bc9a9af54a08a` |
| `tests/fixture_identity.rs` | `86564fd5db8ba68b5dd897e52a5c97de4f1c5004cd09f31665da247974afc03f` |

The implementation evidence's file manifests retain the complete reference and recipe identities. No implementation or reference file was edited by this reviewer.
