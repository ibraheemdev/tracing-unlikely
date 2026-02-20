# `tracing-unlikely`

A wrapper around [`tracing`](https://github.com/tokio-rs/tracing/) that minimizes overhead when tracing
is disabled.

This crate exports API-compatible tracing macros that avoid inlining debug machinery into the hot path
when any relevant tracing levels are disabled. This is useful for crates where tracing is used heavily
for debugging but rarely enabled in practice.

Note that the rest of the `tracing` public API is also re-exported, and all crate-level features are mirrored,
so this crate serves as a drop-in replacement for the entire `tracing` crate.
