# Central scanner pin reconciliation

Date: 23 November 2026, Australia/Brisbane. This is a bounded scanner-adoption
record; numerical, scientific and owner-device acceptance remain separate.

## Verified starting point

Remote `main` and the clean local source were
`c6a757779c0f5d9869bf71a55df0c64cc69ac7b5`, tree
`2674ffebeb2061c6ff9256129833bdbef9c7b83e`. A fresh Git fetch and the GitHub
branch response agreed. The only open pull request was StepSecurity #1, adding
`.pre-commit-config.yaml`; it did not overlap these pin/guidance files and was
not applied. No competing writer owned this Limma checkout.

The old native hook was actually installed and its four local configuration
values read back before tracked edits. It used the clean central checkout at
`b30b07bcdaa6aebad2c6a57a4d0adc3616dabfe4`, isolated Python 3.15.2 and Gitleaks
8.30.1. Pinned tool bootstrap and `uv sync --locked --python 3.15.2` passed.

The full baseline command was:

```sh
uv run --locked --python 3.15.2 python scripts/verify.py --output artifacts/verification/privacy-pin-baseline-20261123
```

All **29 checks passed**, including **121 Rust tests** in each debug, release
and instrumented run and **25 Python tests**. The retained receipt is
`artifacts/verification/privacy-pin-baseline-20261123/receipt.json`, SHA-256
`275644c6c8655270e39591f7c138983fbc30e0c996ba6f924bfd7f650ad44637`.
The canonical compact, sorted-key JSON encoding of its source-hash map has
SHA-256 `1e5d0cbf998b86e4a34d1378787e976873fced74dbf9b3ecfe117e5f4ee8f769`.
The run took 63.22 seconds on Linux x86_64; this is cloud execution.

## Initial candidate and adoption boundary

The initial hook and hosted-action candidate advanced together to central revision
[`2efdbd988064f6857795b9fa9ed4e1d08d18791d`](https://github.com/edithatogo/repository-standards/tree/2efdbd988064f6857795b9fa9ed4e1d08d18791d),
tree `7caade42e1ef76e438c07b9aed4b82a2e8f3227a`.
Current pins subsequently advance to the qualified resolution recorded below;
the earlier [hook receipt](native-hooks-2026-11-23.md) remains unchanged as
historical evidence.

Independent central-delta review found the wrapper, composite action, installer,
dependency lock and hook policy unchanged. The adopted scanner changes cover
merge-aware secret-history diffs, deadlock-resistant blob input and exact
committed-source review identities. Those review identities do not excuse staged
content or sensitive-profile privacy matches. Public-attribution advisories and
scope gaps remain visible. The isolated runtime and native staged-index wrapper
are reused; no policy is copied into this repository.

## Initial candidate disposition

The new native hook was actually installed, with `core.hooksPath = .githooks` and
the central checkout at the exact new revision. The isolated Python and Gitleaks
paths were reused, outside this repository. All four local values were read back.
The existing **13 hook regressions passed** in 2.683 seconds; their retained log
is `artifacts/verification/privacy-pin-hook-tests-20261123.log`, SHA-256
`5bff86119c8454a97f993b9acf90470922654a27f47ba90ec25cf43d9e2074d2`.

The complete strict candidate passed **29/29 checks** in 49.26 seconds, with
**121 Rust tests per profile and 25 Python tests**. Branch-aware patch coverage
was **1/1 changed executable line, 100%**, against `c6a7577`; no threshold changed.
The retained receipt is
`artifacts/verification/privacy-pin-candidate-20261123/receipt.json`, SHA-256
`d58c0ab92845655aedee7712802c54db0d2dac15ca8e1b52a308e2b7d64a227f`.
Its canonical source-hash-map SHA-256 is
`24232ad6dadb4e79b0c2b43df55310195572d7f48372ad8e46f613019691fa1c`.
This receipt precedes the present result/status documentation; a final hook must
verify the complete staged candidate again before any integration commit.

Independent peer source review accepted both operative pins, current guidance,
actual isolated hook configuration and the unchanged historical, numerical,
verifier and workflow-permission boundaries. That source acceptance does not
replace the separate full privacy result below.

## Full central privacy failure and revisit condition

The new pinned central guard was actually invoked with `--mode full` against
the unchanged committed base `c6a757779c0f5d9869bf71a55df0c64cc69ac7b5`, using
the installed isolated Python and Gitleaks tools. Full mode reads that committed source
and reachable history; it does not certify these uncommitted documentation edits.
The command returned **exit 1**. Its value-free report is retained unchanged at
`artifacts/verification/privacy-pin-central-before-commit-20261123.jsonl`, SHA-256
`6f047155898047d2196334f5beb485deeaab6ea6ceb8a8899b620d20395a6230`.

| Gate | Actual outcome |
|---|---|
| Reachable secret history | Clean: zero detected findings and zero reviewed nonsecret matches |
| Current committed tracked-text privacy | Failed: 123 blocking findings and 61,671 advisories; zero reviewed nonidentifier matches |
| Selection and scope | 756 files scanned of 757 selected; the Conductor submodule is skipped; coverage is explicitly incomplete |
| Report detail | Finding groups are truncated: 52,001 omitted; aggregate blocking/advisory counters remain available |

Visible blockers include phone-number recognizer matches in existing numerical
CSV fixtures. Those examples do not establish that all 123 matches are false
positives. The central privacy-policy owner must review the exact source/blob and
entity identities, retaining the failed report and all unreviewed findings.
Revisit after a complete central disposition and a passing unchanged-policy
replay, or an explicitly reviewed central revision for coordinated adoption.
No numerical fixture, threshold, exclusion or local scanner policy was changed.

### Complete source review prepared

A read-only diagnostic called the unchanged pinned scanner's blob-selection,
reading and per-blob analysis helpers, preserving its selection, skip rules,
recognizers, profile and central review registry. It reconciled all **53,001
finding groups**, the full **123 blocking / 61,671 advisory** counts, and the
756/757 file selection. Exact spans were then recovered only for the blocking
blobs/entities and reconciled against every original per-blob group/count.

Every blocking match is `PHONE_NUMBER`. Each of the **123 exact spans** lies
entirely within **one finite numeric CSV measurement field**; no span crosses
fields or enters the row-label column. Standard-library CSV parsing, serialized
field offsets, every match digest and the expected matrix shapes were checked.
All nine affected paths contain blobs unchanged from the audited upstream base
`ba41fb6bbf6e3f2524bd44f5be017c827d5c4aaa`.

| Source under `tests-validation/data/` | Exact matches | Source field |
|---|---:|---|
| `block/bojkova_pairs/input_log2.csv` | 20 | Numeric log2-intensity matrix |
| `block/techrep_mixed/input_log2.csv` | 4 | Seeded synthetic numeric matrix |
| `block/techrep_synth/input_log2.csv` | 4 | Seeded synthetic numeric matrix |
| `block/techrep_synth_na/input_log2.csv` | 3 | Seeded synthetic numeric matrix |
| `matrix/conditions_categorical_numeric_covariates/input_log2.csv` | 30 | Numeric log2-intensity matrix |
| `matrix/conditions_numeric_covariate/input_log2.csv` | 30 | Numeric log2-intensity matrix |
| `matrix/conditions_only/input_log2.csv` | 30 | Numeric log2-intensity matrix |
| `matrix/conditions_categorical_numeric_covariates/design.csv` | 1 | Seeded `numcov` value |
| `matrix/conditions_numeric_covariate/design.csv` | 1 | Seeded `numcov` value |

The pinned [block recipe](https://github.com/edithatogo/limma-rust/blob/c6a757779c0f5d9869bf71a55df0c64cc69ac7b5/tests-validation/scripts/block_goldens.R)
and [matrix recipe](https://github.com/edithatogo/limma-rust/blob/c6a757779c0f5d9869bf71a55df0c64cc69ac7b5/tests-validation/scripts/scalar_goldens.R)
provide the actual field/generation semantics. This review did not regenerate
the historical corpus or infer biological identities or scientific admission.

The ignored diagnostic directory is
`artifacts/verification/privacy-pin-inventory-20261123/`. Its receipts contain
paths, blob IDs, line/span offsets, digests, parsed field roles and immutable
source/generator links; they contain no matched values or row snippets:

- `inventory.json`: SHA-256 `ac5a1f69a68927eeeef79c52142d62d5eb4af0570b7eac62890c737e4c6e2241`.
- `classification.json`: SHA-256 `c691cb7e6f12fb4f150f5739b1e2a15cc1680348c7720df91ee04e2beef8ea3f`.
- `proposed-exact-reviews.json`: SHA-256 `3e3075404f857d80348e853eff6b2454141c051274fb7d6de878de1eed5d8c15`.

These **123 exact review proposals were submitted for central-owner acceptance**.
The diagnostic did not change the existing failed result. Its prospective registry,
including its existing 21 entries, is 144 entries: **126,103 bytes compact or
143,971 bytes indented**, exceeded the then-current 65,536-byte bound. The
capacity/representation decision was handled by the central policy change below;
no local override, size-limit bypass or consumer exclusion was introduced.

## Qualified central resolution and receiving replay

Independent exact-span review separately re-read all immutable bytes and identity
fields and checked the CSV boundaries. It accepted all 123 classifications:
121 fractional-digit matches within numeric log2 measurements and two complete
noninteger decimal values in the seeded `numcov` field. No cross-field, header or
row-label case was accepted by inference.

[Central PR #175](https://github.com/edithatogo/repository-standards/pull/175)
merged as `0be4668d17ac3b6d902d4354bd01f420c7c29272`, tree
`dea1ead6d63eee76de88e7605b6a9d04e98e50c5`. The published PR head, tested merge
and merged source have that same tree. The change retains all 21 earlier exact
reviews unchanged, adds the 123 accepted identities and raises the bounded
registry limit from 64 to **128 KiB**. The resulting registry is **126,835 bytes**.
Detection/severity rules and staged-review exclusions are unchanged; no reviewed
identity can excuse staged content or sensitive-profile privacy matches.

All ten provider PR checks passed. Its actual candidate-source
[behavioral and canary run](https://github.com/edithatogo/repository-standards/actions/runs/45116735670)
executed **47 privacy tests and 39 Gitleaks tests with zero skips**. The canary
invoked the candidate workspace action and scanned **6/6 files**, with zero
secret, blocking or advisory findings and no skipped files. The separately
old-pinned self-scan was not used as candidate-source qualification. The retained
provider receipt is `hosted-pr175/receipt.json` under the coordinating central
`artifacts/verification/limma-numeric-review/` directory, SHA-256
`8d6ebc6ecec94eb187c57dc938f42ab58c34f4750170d999f8a044c832e4792b`.

The coordinator's full receiving replay used the unchanged Limma commit
`c6a757779c0f5d9869bf71a55df0c64cc69ac7b5` and the exact accepted scanner
runtime/registry hashes, which were rechecked against the clean merged checkout.
It returned **exit 0 in 223.98 seconds**: zero Gitleaks findings, zero blocking
privacy findings, **123 reviewed nonidentifier matches and 61,671 advisories**.
It scanned 756/757 selected files; the Conductor submodule remained skipped.
The report correctly retains **`status: incomplete`, `coverage_complete: false`**
and 52,001 omitted finding groups. Passing the blocking gate is not complete
privacy coverage or clinical de-identification.

The central `receiving-receipt.json` SHA-256 is
`bcac52017a96801ad8719e7410be2d1d7ebbb7de4bb7ce225e4c1ee43c6b48e3`;
its retained `candidate-full-limma.log` SHA-256 is
`1a153dd6b67d7ed4c5d9f96d6f6658f802ddd014143ba0d6293f15a7eb26e2cc`.
The original failed scan and diagnostic receipts remain unchanged.

## Final consumer acceptance

Both operative pins and current installation guidance now select the clean
merged central source `0be4668d17ac3b6d902d4354bd01f420c7c29272`. The hook was
actually reinstalled with that clean checkout and the unchanged isolated Python
and Gitleaks tools, then all four configuration values were read back. The retained
`artifacts/verification/privacy-pin-qualified-hook-install-20261123.json` has
SHA-256 `21002f4fdb7175e304f0a1a879dc15227bae0341cb2877ca80b1428f529f4657`.

The qualified consumer's full strict gate passed **29/29 checks in 55.12 seconds**:
121 Rust tests in each debug/release/instrumented run, 25 Python tests including
the existing 13 hook regressions, and **1/1 changed executable line covered
(100%)** under the existing branch-aware patch policy. Its retained receipt is
`artifacts/verification/privacy-pin-qualified-candidate-20261123/receipt.json`,
SHA-256 `3df37bf4676fd1e273f9b04493bb57f921ba1ba87e7d1cda49f9325842d848e8`;
the canonical source-hash-map SHA-256 is
`80db2713fdf2d3d96d6e46039110ec2a951ee7681068ec5f281599b27ef16404`.

Independent final review accepted both operative pins, actual hook configuration,
qualified source and preserved scope; it separately matched all 762 receipt source
identities to the frozen candidate. The [bounded LM01b maintenance
acceptance](../archive/lm01b_privacy_pin_20261123/index.md) is archived. This receipt
precedes the integration commit and final archive-documentation byte checks.
The actual commit must run the installed native hook over the complete staged
candidate, and its exact committed source must receive a full central scan before
merge. The coordinator records that commit/tree, hook receipt, actual final
advisory counts and coverage gaps separately, and owns remote publication and
hosted-check observation. Parent LM01 owner-device, LM02 production/historical
and scientific gates remain open.
