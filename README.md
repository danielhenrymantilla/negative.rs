

<span style="text-align: center;">

![image](https://gist.github.com/user-attachments/assets/8dc46e20-90d6-431a-8805-665549e02650)

</span>

# `::negative`

Negative `impl`s in stable Rust.

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/negative.rs)
[![Latest version](https://img.shields.io/crates/v/negative.svg)](
https://crates.io/crates/negative)
[![Documentation](https://docs.rs/negative/badge.svg)](
https://docs.rs/negative)
[![MSRV](https://img.shields.io/badge/MSRV-1.56.0-white)](
https://gist.github.com/danielhenrymantilla/9b59de4db8e5f2467ed008b3c450527b)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](
https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/negative.svg)](
https://github.com/danielhenrymantilla/negative.rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/negative.rs/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/negative.rs/actions)
[![no_std compatible](https://img.shields.io/badge/no__std-compatible-success.svg)](
https://github.com/rust-secure-code/safety-dance/)

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->

---

It can be useful to avoid the need to include an ugly `PhantomData`:

```rust ,compile_fail
/// Unit struct!
struct SyncButNotSend;

#[::negative::negative_impl]
unsafe impl !Sync for Foo {}

const _OK: &dyn Sync = &SyncButNotSend;

/// Error, `Send` is not implemented!
const _COMPILE_ERROR: &dyn Send = &SyncButNotSend;
```

But most importantly, it does change the semantics with regards to _coherence_ (overlapping `impl`s checker), since it makes the trait be explictly unimplemented, _becoming a semver promise_, rather than the default "this thing happens to be carrying a field type which itself happens not to be implementing said auto-trait _yet_, so let's make it so the resulting type does not implement saif auto-trait _yet_ either".

```rust
struct Foo;

#[::negative::negative_impl]
impl !Unpin for Foo {}

trait Demo {}

impl<T : Unpin> Demo for T {}

impl Demo for Foo {} // <- does not overlap!
```

Indeed, the following, on the other hand, would fail to compile:

```rust ,compile_fail
struct Foo(::core::marker::PhantomPinned);

trait Demo {}

impl<T : Unpin> Demo for T {}

impl Demo for Foo {} // <- Error, may overlap!
```

This is because the way stable Rust traditionally opts out of auto-traits is _via_ lack-of-auto-trait-impl structural infection from `Phantom` types.

But these types, as far as the coherence checker is concerned, just happen not to be implementing said traits _in the current version of the stdlib_, that is, they conservatively assume a future, semver-compatible, bump of the standard library may decide to add:

> `impl Unpin for PhantomPinned {}`

Thus, it considers that:

 1. in current Rust, `PhantomPinned : Unpin` does _not_ hold;

 1. but in some future version of Rust, `PhantomPinned : Unpin` _could hold_.

Thus, as far as the coherence checker is concerned, rather than:

> ~~`PhantomPinned : !Unpin`~~

we actually have:

```rust ,ignore
PhantomPinned : ?Unpin
```

  - (and so on for all the other auto-traits.)

And thence, `Foo : ?Unpin`, rather than `Foo : !Unpin`.

Hence why `<T : Unpin>` and `Foo` are conservatively deemed to be able to overlap.
