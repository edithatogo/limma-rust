# limma-core: qualified Rust statistical core

## Purpose

Maintain the existing small Rust implementation of limma's linear-model and empirical-Bayes core, and establish whether a precisely bounded interface can support the rare-omics-atlas methylation workflow with the required numerical behaviour.

This is a brownfield fork of MassDynamics/limma-rust. It already has substantial numerical source and a committed R reference corpus. The immediate work is to verify and document that implementation, not to recreate limma or inherit defects from a similarly named repository.

## Initial outcomes

1. Correct source identity, capability claims and reproducible project context.
2. Replay the committed reference comparisons on an exact source revision, then establish a regenerable independent R oracle and evidence from the intended official local runners.
3. Define a small, explicit consumer boundary and repair only demonstrated gaps.
4. Evaluate methylation-specific agreement on separately qualified atlas inputs.
5. Consider dependency replacements or method extensions only after a concrete need has been demonstrated.

Success means another researcher can identify exactly what ran, reproduce the reference comparison and understand the supported data and methods. It does not mean every R limma function has been reimplemented.

## Relationship to rare-omics-atlas

The atlas owns acquisition, permissions and redistribution decisions, raw-array preprocessing, ontology mapping, batch/design decisions and downstream pathway/network analyses. This crate owns fitting and inference for its documented numerical contract. Arrow/Parquet belong at the consumer boundary; no lakehouse belongs inside the statistical kernel.

The atlas can continue with its established reference backend while this optional Rust implementation is qualified. Its clinical analyses must not depend on completing a speculative kernel rewrite.

## Publication

Validation and engineering work are usually bundled into an atlas analysis or reproducibility supplement. A separate software paper is considered only if a substantive validated contribution remains after a new literature and software check. Reimplementation in Rust alone is not a novelty claim.
