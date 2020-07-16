.PHONY: bench-rust-to-caml bench-ocaml-to-rust

_build/default/callers/ocaml/ocaml_rust_caller.exe:
	cd callers/ocaml; dune clean
	dune build callers/ocaml/ocaml_rust_caller.exe

bench-ocaml-to-rust: _build/default/callers/ocaml/ocaml_rust_caller.exe
	dune exec callers/ocaml/ocaml_rust_caller.exe

bench-rust-to-ocaml:
	cargo bench
