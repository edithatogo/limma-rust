# LM02b — Named numerical consumer and synthetic exchange

Review date: 23 November 2026, Australia/Brisbane. The track suffix and instrument
receipts use UTC 22 November.

**Status: completed and archived for the bounded producer.** All 29 strict checks
passed with 121 Rust tests per profile and 25 Python tests; two independent source
reviews accepted this scope. Atlas immutable-pin receiving and the actual publishing
hook commit remain separate coordinating follow-ups.

This bounded increment adds the v1 named Rust boundary and a synthetic reference
exporter. Its actual coordinating caller is Atlas's `scripts/qualify_limma.py`.
It does not close parent LM02 production exchange, historical sample provenance,
LM01 owner-device execution or LM03 scientific admission.

- [Specification](spec.md)
- [Executable plan](plan.md)
- [Metadata](metadata.json)
- [Authoritative numeric contract](../../../docs/named-consumer-v1.md)
- [Executed provider evidence](../../evidence/lm02b-named-consumer-2026-11-22.md)
- [Parent LM02](../../tracks/lm02_consumer_20261122/index.md)

The archived acceptance covers the named API and synthetic producer, including its
mandatory file round-trip tests. The actual Atlas receiving result remains parent
LM02-5 work and requires a published provider source pin. Development receiving
has passed, but a provider-only acceptance is not immutable-pin acknowledgement.
The default tool's exporter line-coverage exclusion is explicit in the contract
and evidence; no numerical, dependency or tolerance waiver was introduced.
