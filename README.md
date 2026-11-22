# limma-rust

A pure-Rust port of the limma linear-model and empirical-Bayes core (`lmFit`, `contrasts.fit`,
`eBayes`, `topTable`, `decideTests`) with R-faithful nmath and lowess. The crate is
`limma-core`.

It was split out of `md-limma` (`crates/limma-core` at md-limma `main` a4c3893) with its history.
The Python binding and the Mass Dynamics job layer stay in md-limma.

## Test

    cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test

The tests compare against the R goldens in `tests-validation/` (inputs in `data/`, R's results in
`expected/`, the R scripts in `scripts/`; see its README). Set `MD_GOLDEN_CORPUS_DIR` to run them
against another corpus.

## Owner-fork qualification update

The [fresh independent reference](tests-validation/FRESH_REFERENCE.md) is now independently reproducible from public hash-pinned R/limma/statmod sources. Four synthetic cases regenerate 117 files byte-identically; the new slice compares 30,840 values (including 249 matching nonfinite values), and both cloud debug/release suites pass 110 tests. [Independent review](conductor/evidence/fresh-reference-ponytail-review-2026-11-22.md) accepted this bounded increment; [LM01a is archived](conductor/archive/lm01a_reference_20261122/index.md). Production numerical source and historical fixture tolerances remain unchanged.

This is not methylation/clinical admission or a run on the owner's Mac/Windows devices. Consumer identity contracts and the official local-runner receipts remain in Conductor. sinmojito/limma-rust is a separate, optional comparator for a named method; neither agreement between ports nor language choice replaces the independent R oracle.
