main: externals
	dune build example/src/main.exe

externals:
	cd core && cargo build
	cd example && cargo build
	dune build example/src/printmod.exe

test: externals
	dune runtest

clean:
	cd core && cargo clean
	cd example && cargo clean
	dune clean
