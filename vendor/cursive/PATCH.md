# Vendored `cursive 0.16.4-alpha.0` + `cursive_core 0.2.2` (patched)

This directory contains a minimal, vendorized copy of
[`cursive 0.16.4-alpha.0`](https://github.com/gyscos/cursive)
(`cursive/` + `cursive-core/`), taken from the git revision puppy already
used (`52188d3`), so that puppy can drop dependencies with unfixable
vulnerabilities without changing any TUI behavior.

## Why this exists

1. `owning_ref 0.4` (used by `cursive_core`) is unsound and unmaintained
   ([RUSTSEC-2022-0040](https://rustsec.org/advisories/RUSTSEC-2022-0040)),
   and no patched release exists. The maintained fork
   [`safer_owning_ref 0.5`](https://crates.io/crates/safer_owning_ref)
   is API-compatible, so `cursive-core/Cargo.toml` redirects the
   dependency to it via a Cargo rename:

   ```toml
   # cursive-core/Cargo.toml (before)
   owning_ref = "0.4"
   # cursive-core/Cargo.toml (after)
   owning_ref = { package = "safer_owning_ref", version = "0.5" }
   ```

   The Rust sources keep using `use owning_ref::{...};` unchanged; only
   the package providing that crate name changed.

2. `ncurses` (the default backend of `cursive`) is unmaintained
   (RUSTSEC-2019-0006 / RUSTSEC-2025-0108). Puppy's `Cargo.toml` already
   selects `default-features = false, features = ["termion-backend"]`
   instead, so neither `ncurses` nor `term_size` / `crossterm 0.19`
   (and its `mio 0.7` / `instant` chain) is built anymore.

## The patch

Only three lines of Rust were touched, all required because
`safer_owning_ref` spells the elided lifetime of `owning_ref 0.4`
explicitly (`RcRef<'u, T>` / `ArcRef<'u, T>` instead of `RcRef<T>` /
`ArcRef<T>`). The owned data outlives the handle in both cases, so
`'static` is the correct (and behavior-preserving) annotation:

- `cursive-core/src/views/named_view.rs`:
  `RcRef<RefCell<V>>` -> `RcRef<'static, RefCell<V>>`
- `cursive-core/src/views/text_view.rs` (2 places):
  `ArcRef<Mutex<TextContentInner>>` -> `ArcRef<'static, Mutex<TextContentInner>>`

Plus two packaging-only tweaks so the vendored crates build standalone:

- `cursive/Cargo.toml`: `readme = "../Readme.md"` -> `readme = "Readme.md"`
  (with `Readme.md` + `LICENSE` copied in), matching the layout of the
  published crate.
- `cursive-core/Readme.md`: copied in for the same reason.
- `cursive-core/Cargo.toml`: `chrono = "0.4"` ->
  `chrono = { version = "0.4", default-features = false, features = ["clock", "std"] }`.
  `chrono`'s default features (`wasmbind`, which pulls `wasm-bindgen` for the
  `wasm32` target and thus transitively `bumpalo`, plus `iana-time-zone`)
  are not needed: `cursive-core` only uses `chrono::DateTime`,
  `chrono::Utc`, and `chrono::Local` in its debug logger view.
  Same set puppy itself enables in its own `Cargo.toml`.

No other behavior was changed. After this change `Cargo.lock` contains
`safer_owning_ref 0.5.1` and no longer contains `owning_ref`, `ncurses`,
`term_size`, `crossterm 0.19`, `mio 0.7`, or `instant`.
