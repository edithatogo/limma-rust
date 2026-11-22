# LM04 — Conditional numerical dependency review

## Trigger

Start only after an observed maintenance issue or measured resource problem justifies a particular replacement. This track is off the critical path for the first atlas analysis. A library being written in Rust is not sufficient justification.

## Bounded candidate

Inspect current primary documentation and source for one candidate at an exact revision. A generic dense-kernel replacement using faer or nalgebra is a candidate experiment, not a default architecture. These libraries are not current dependencies. Do not replace R LINPACK pivoting or R nmath solely by matching function names.

Record versions/features, licence, advisories, MSRV, transitive dependency/build cost, affected callers, existing alternative, migration benefit and rollback. Prefer one internal replacement to a new generic backend framework.

## Acceptance

- A baseline and a stated maintenance/resource objective exist before the prototype.
- All affected block, contrast, fit and empirical-Bayes/F callers pass existing and independent reference cases.
- Rank/pivot, non-estimable state, extreme/log tails, missingness and deterministic output semantics remain correct for the changed area.
- Public interface compatibility and the named consumer remain accepted.
- Any performance claim uses identical data and measured end-to-end/resource evidence; compile cost is included.
- The candidate is retained only if the benefit outweighs dependency/ownership cost. Rejection and retention of the current implementation are valid results.

## Out of scope

Blanket kernel rewrites, simultaneous dependency changes, mandatory GPU/AD layers and loosening tolerances to make a candidate pass.
