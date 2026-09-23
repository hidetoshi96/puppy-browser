# Vendored `rusty_v8 0.22.1` (patched)

This directory contains a minimal, vendorized copy of
[`rusty_v8 0.22.1`](https://crates.io/crates/rusty_v8/0.22.1) so that puppy can be
built and run with a modern Rust toolchain.

## Why this exists

`rusty_v8 0.22.1` was released in 2021, when `std::any::TypeId` was 8 bytes long.
Its `TypeIdHasher` (used for the `HashMap<TypeId, Box<dyn Any>>` that stores the
embedder data of an isolate) asserted that assumption:

```rust
// vendor/rusty_v8/src/support.rs (before the patch)
assert_eq!(size_of::<TypeId>(), size_of::<u64>());
assert_eq!(align_of::<TypeId>(), size_of::<u64>());
```

Since Rust 1.72, `TypeId` is a 128-bit value
([rust-lang/rust#103449](https://github.com/rust-lang/rust/pull/103449)), so
`size_of::<TypeId>()` is 16 and the assertion panics at runtime:

```
thread 'main' panicked at rusty_v8-0.22.1/src/support.rs:556:5:
assertion `left == right` failed
  left: 16
 right: 8
```

The `Hash` implementation for `TypeId` still only feeds the lower 64 bits of the
`TypeId` to `Hasher::write_u64`, which is exactly what `TypeIdHasher` is
optimized for, so hashing itself keeps working as intended.

## The patch

### 1. `src/support.rs`: the `TypeId` assertions (the reason this copy exists)

The two size/alignment assertions above were removed and replaced with a comment
explaining why. No other behavior was changed.

### 2. Fixes for warnings reported by modern rustc

The rest of the changes in this copy only address warnings that recent rustc
versions report for these 2021-era sources. They were mostly applied with
`cargo +1.98.1 fix --lib -p rusty_v8`, and none of them changes behavior:

- `src/lib.rs`: dropped six `pub use <module>::*;` lines (`bigint`, `object`,
  `private`, `proxy`, `symbol`, `template`). Those modules only contain `impl`
  blocks and `extern "C"` declarations (the types themselves are defined in
  `src/data.rs`), so the globs re-exported nothing and are reported as
  `unused_imports`.
- `src/module.rs`, `src/script_compiler.rs`, `src/script_or_module.rs`,
  `src/inspector.rs`, `src/value_serializer.rs`, `src/value_deserializer.rs`:
  added the explicit lifetimes (`'_`, `'s`) that rustc now suggests for
  `mismatched_lifetime_syntaxes` and `elided_named_lifetimes`.
- `src/isolate.rs`, `src/support.rs`: `Any::downcast_ref`, `Any::downcast_mut`
  and `Any::is` are methods on `dyn Any`, so they are now called with the explicit
  `<dyn Any>::` form to silence `bare_trait_objects`.
- `src/scope.rs`: `HandleScope::uninit()` and `TryCatch::uninit()` now return
  `Self([0; 3])` / `Self([0; 6])` instead of
  `Self(MaybeUninit::uninit().assume_init())`. The old code is undefined behavior
  (`invalid_value`: integers must be initialized), and the initial contents are
  irrelevant because `init()` overwrites them before they are read.
- `src/scope.rs`: `drop(Box::from_raw(root))` instead of `Box::from_raw(root);`,
  and `src/isolate.rs`: `let _ = Arc::into_raw(annex_arc.clone());`, both to
  satisfy `unused_must_use`. The reference count bookkeeping is unchanged.

Upgrading to a newer `rusty_v8` is not an option here: the V8 bindings API
(`v8::V8::initialize_platform`, `ScriptOrigin::new`, `v8::READ_ONLY`, ...) changed
significantly after 0.22.x, and puppy's `src/javascript` module targets that old
API.

## Contents

Only the files required to build the crate with a pre-built V8 archive
(`librusty_v8.a`, downloaded by `build.rs` into `target/<profile>/gn_out/obj/`)
are included:

- `Cargo.toml`
- `build.rs`
- `src/` (Rust sources and the `binding.cc` glue code)
- `tools/download_file.py` (fallback downloader used by `build.rs`)

The upstream sources can be obtained again with:

```sh
cargo vendor --versioned-dirs
```
