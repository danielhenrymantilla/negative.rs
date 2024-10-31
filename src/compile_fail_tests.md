# The following snippets fail to compile

#### Missing `!`

```rust ,compile_fail
use ::negative::*;

enum Foo {}

#[negative_impl]
impl Unpin for Foo {}
```

#### Missing `!Trait for`

```rust ,compile_fail
use ::negative::*;

enum Foo {}

#[negative_impl]
impl Foo {}
```

#### Non-empty impl body

```rust ,compile_fail
use ::negative::*;

enum Foo {}

#[negative_impl]
impl !Unpin for Foo {
    fn foo() {}
}
```

#### Properly removes trait impl

```rust ,compile_fail
struct Foo;

#[::negative::negative_impl]
impl !Unpin for Foo {}

const _: &dyn Unpin = &Foo;
```

#### Alas, `unsafe` is required for `unsafe trait`s

```rust ,compile_fail
struct Foo;

#[::negative::negative_impl]
impl !Send for Foo {}
```

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->
