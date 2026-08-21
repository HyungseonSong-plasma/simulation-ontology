//! Protocol-facing MockAdapter reference behavior for the published SOL adapter contract.
//!
//! M0.4 phases add deterministic in-process reference operations here while preserving
//! the pre-existing Rust-only helper API in `lib.rs` as a separate compatibility layer.
//!
//! This module is intentionally transport-independent and solver-independent.
