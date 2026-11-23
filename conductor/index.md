# Project Context

## Definition

- [Product Definition](./product.md)
- [Product Guidelines](./product-guidelines.md)
- [Tech Stack](./tech-stack.md)

## Workflow

- [Workflow](./workflow.md)
- [Code Style Guides](./code_styleguides/)
- [Dependency and boundary decisions](./decisions.md)
- [Canonical named consumer v1 contract](../docs/named-consumer-v1.md)

## Evidence

- [Strict local/hosted CI implementation and executed receipt](./evidence/strict-ci-2026-11-22.md)
- [Accepted LM02b named consumer and synthetic producer](./archive/lm02b_named_consumer_20261122/index.md)
- [Accepted pinned Atlas synthetic receiving receipt](https://github.com/edithatogo/rare-omics-atlas/blob/4162111f38e63ca21d811d7df2e628239297bc8d/research/qualification/contracts-hooks-2026-11-23.md)
- [Installed native hooks and real central scanner proof](./evidence/native-hooks-2026-11-23.md)
- [Central scanner pin reconciliation](./evidence/privacy-pin-2026-11-23.md)
- [Source audit and provenance](./evidence/source-audit-2026-11-22.md)
- [Context setup receipt](./evidence/setup-2026-11-22.json)
- [Executable baseline and prerequisites](./evidence/baseline-status-2026-11-22.md)
- [Cloud committed-reference replay receipt](./evidence/cloud-corpus-replay-2026-11-22.json)
- [Cloud replay log](./evidence/cloud-corpus-replay-2026-11-22.log)
- [Fixture hygiene and fresh named-identity evidence](./evidence/fixture-identity-2026-11-22/README.md)
- [Fixture-identity independent Ponytail review](./evidence/fixture-identity-2026-11-22/independent-ponytail-review.md)

## Tracks

- [Tracks Registry](./tracks.md)
- [Tracks Directory](./tracks/)

- [Manual official local workflow setup](evidence/local-workflow-setup-2026-11-22.md)
- [Additional cloud formatting/clippy/release receipt](evidence/cloud-static-release-checks-2026-11-22.json)

- [Fresh independent R reference](../tests-validation/FRESH_REFERENCE.md)
- [Fresh-reference independent review](evidence/fresh-reference-ponytail-review-2026-11-22.md)
- [Completed track archive](archive/README.md)
- [Completed LM02a bounded fixture increment](archive/lm02a_fixture_identity_20261122/index.md)

## Repository integration

Limma owns numerical contracts and independent reference checks. Atlas owns the
actual receiver, provenance, preprocessing/storage and scientific admission.
Central repository standards own shared privacy policy and its pinned scanner.
Use the single numeric contract above and receiving links; do not create diverging
copies or combine these repositories solely to share a contract.

The named producer was published through [Limma PR #6](https://github.com/edithatogo/limma-rust/pull/6)
at `c08c53943a7acb9c6a1152df8d79e27cffe7199b` and merged at
`a8f6caf30b6b1cafe31240d3bfe1e355805a9a84`. [Atlas PR #10](https://github.com/edithatogo/rare-omics-atlas/pull/10)
merged the receiving receipt above at `4162111f38e63ca21d811d7df2e628239297bc8d`,
using that immutable producer pin. Bounded synthetic publication and receiving
are complete. Parent LM02 production exchange and historical provenance, LM01
actual owner-device acceptance and LM03 scientific admission remain open. Local
hook installation and server-side required contexts remain distinct facts.
