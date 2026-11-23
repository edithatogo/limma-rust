# LM01b executable plan

| Order | Concrete action | Required evidence | State |
|---|---|---|---|
| A | Confirm remote `main`, clean worktree and open-PR file ownership; install the old native hook using the README command; run `uv run --locked --python 3.15.2 python scripts/verify.py --output artifacts/verification/privacy-pin-baseline-20261123` | Clean base `c6a757779c0f5d9869bf71a55df0c64cc69ac7b5`; real old-pin hook installation; all 29 checks pass | Complete |
| B | Advance both operative pins and current README/workflow guidance; reinstall with the clean central `2efdbd9` checkout and existing isolated tools | Both pins match the checkout; actual clone-local configuration read back; no numerical or verifier change | Complete |
| C | Run `uv run --locked --python 3.15.2 python -m unittest discover -s scripts -p test_hooks.py -v`, then `uv run --locked --python 3.15.2 python scripts/verify.py --output artifacts/verification/privacy-pin-candidate-20261123` | Existing hook behaviours and all strict checks pass; exact source and receipt hashes retained | Complete: 13 hook tests; all 29 strict checks |
| D | Review every original blocking span against its immutable CSV field and generator; the central owner publishes and qualifies exact reviews before adoption | All 123 spans independently accepted; central PR #175 merged as `0be4668d17ac3b6d902d4354bd01f420c7c29272`; qualified full receiving replay has 0 blockers, 123 reviewed matches, 61,671 advisories and the recorded Conductor gap | Complete; original failed report retained |
| D2 | Adopt the qualified merged pin in both operative sites/current guidance; reinstall the hook; run `uv run --locked --python 3.15.2 python scripts/verify.py --output artifacts/verification/privacy-pin-qualified-candidate-20261123` | Actual clean checkout/configuration, complete strict candidate receipt and independent final review | Complete: all 29 strict checks and independent qualified-source review pass |
| D3 | After D2, archive only this bounded maintenance acceptance and validate its links/statuses | Parent LM01, LM02 production/historical work and scientific admission remain open; final publishing gates stay explicit | Complete: bounded scanner integration archived |
| Integration | Stage the whole reviewed candidate and commit through the installed native hook; run the pinned central guard with `--mode full` on the resulting complete clone | Real staged scan and full strict verifier both pass for the unchanged candidate; exact committed-source full result/advisory review; commit/tree handed to coordinator | Required before publication/merge; actual outcomes belong to the commit and receiving receipts |

The implementation owner resolves failed local gates before integration. The coordinator
owns publication and checks on the published revision. Server settings and the
actual owner-device lane retain their existing owners and revisit conditions;
this scanner adoption cannot close them.

The central review matched the complete 123 identities, not only the original
truncated report's visible examples. Its 21 earlier entries remain unchanged.
No numerical fixture, local exclusion, severity rule or scanner threshold changed.
Preserve the 61,671 baseline advisories and Conductor submodule coverage limit;
record the actual final-commit counts rather than inferring them from the baseline.
