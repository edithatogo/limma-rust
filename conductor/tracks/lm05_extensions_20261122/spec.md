# LM05 — Consumer-led extensions

## Trigger

A named consumer needs a method the current supported interface cannot provide, and reusing a qualified existing implementation is insufficient. This track may resolve an LM03 admission blocker, but no extension is required by default.

## Scope decision

First distinguish a small boundary addition from a new statistical method. Named export fields, existing moderated SE and strict identity validation are small interface work, usually LM02. Weighted fitting, TREAT and camera require distinct method specifications and reference cases. They are not quick additions inferred from generic matrix or probability libraries.

Search current Rust candidates and their primary source, also considering Julia/Python/R implementations under the project hierarchy. Decide whether to reuse, make a small upstream contribution, implement a bounded missing capability or retain the reference backend. Identify licence and redistribution constraints separately.

## Acceptance

Each chosen extension has an explicit scientific contract, allowed inputs, outputs/errors, independent reference, tolerances, integration boundary and rollback. A failing reference/contract test precedes implementation. Existing supported methods remain unchanged. The named consumer and relevant device/resource gates pass before an advertised capability is added.

Novelty and publication value are reviewed against literature before and after development. A package port or an automatic-differentiation experiment alone is not assumed to warrant a paper. Prefer bundling useful methods and validation with the actual analysis.

## Out of scope

Complete R limma parity, speculative raw-array preprocessing and building a general modelling platform inside this crate.
