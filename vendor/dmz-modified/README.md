> "GC bugs combine two nasty properties: they cannot be studied in isolation and they trigger depending on a complex set of conditions that cannot be inferred by solely looking at the buggy code." - Frédéric Bour [CAMLroot](https://arxiv.org/abs/1812.04905)

DMZ is a foreign function interface between Rust and OCaml that eliminates the need for packing and unpacking buffers in order to avoid memory corruption. It does so by modeling the OCaml garbage collector in Rust so that the borrow checker ensures values aren't referenced at points when they would have been collected.

Rust functions are provided to allocate OCaml values, e.g. `alloc_string`, which must be called inside the `call!` macro in order to register them with the garbage collector. In addition, Rust functions can be wrapped in the `printmod!` macro to automatically generate corresponding OCaml externals. Code on the OCaml side requires no special stylistic conventions.

To use you need to build `allocpair.c` and `printmod.ml` with ocamlopt. Dune stanzas are provided for both in `example/src/dune`.

This library was originally written by Stephen Dolan. The name has been changed to avoid collision. For more information see his [paper](https://docs.google.com/viewer?a=v&pid=sites&srcid=ZGVmYXVsdGRvbWFpbnxtbHdvcmtzaG9wcGV8Z3g6NDNmNDlmNTcxMDk1YTRmNg) and [talk at ICFP 2018.](https://www.youtube.com/watch?v=UXfcENNM_ts)

