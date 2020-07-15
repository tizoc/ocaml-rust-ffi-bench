# Changelog

## [0.2.0]

### New Types

- `bytes` for mutable Rust `String`, whereas `string` becomes `&str`
- `int64` for Rust `i64` vs. 63-bit OCaml `int`
- `Bigstring.t` with custom allocator
- `Triple` that calls `caml_alloc_ntuple` in allocpair.c

### Administrative Changes

- Changed name from caml-oxide to dmz to avoid confusion with formal semantics project and others
- Unit test using Alcotest
- Build OCaml files with Dune
- Restructed directories to separate examples and tests from core Rust library

## [0.2.1]

### New Types

- `('a, string) result` for Rust `Result<A>`
- `bool`
- `as_vec()` function for `List<A>`

### Administrative Changes

- Optional `type_def` method to `MLType` to print defintions of abstract types and records
- `alloc_string_newtype()` for newtype wrappers around `String`

## To Do

- `alloc_list()`
- `alloc_tuple` for arbitrary sized n-tuples
- `Z.t` for Rust `num::BigInt`
- `camlmod!` also prints an .mli file using optional `interface` method for `MLType` (maybe unnecessary)
- Deriving macro to generate `MLType`,`ValExt` and newtype impls as well as allocation functions for newtypes
- Somehow package allocpair.c and printmod.ml with rust crate
- Known bug: won't generate type definitions when wrapped in another type, e.g. `Option<Foo>` results in "Unbound type constructor foo"
