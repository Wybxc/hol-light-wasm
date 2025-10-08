main.wat: main.wasm
	wasm-tools print $< > $@

main.wasm: main.js
	cp main.assets/code.wasm main.wasm

main.js: main.byte
	wasm_of_ocaml --enable wasi --enable exnref $< -o $@

main.byte: main.ml
	dune build
	rm -f $@
	cp _build/default/main.bc $@

switch:
	@echo "Creating and switching to OCaml 5.2.0 environment with necessary packages..."
	opam switch create . 5.2.0
	opam pin add js_of_ocaml git+https://github.com/ocsigen/js_of_ocaml.git#8c5a20c99bc46681f5a726ef41e5864cc298f159
	opam pin add js_of_ocaml-compiler git+https://github.com/ocsigen/js_of_ocaml.git#8c5a20c99bc46681f5a726ef41e5864cc298f159
	opam pin add wasm_of_ocaml-compiler git+https://github.com/ocsigen/js_of_ocaml.git#8c5a20c99bc46681f5a726ef41e5864cc298f159

hol-light:
	@echo "Building and pinning hol_light package..."
	cd hol-light && make HOLLIGHT_USE_MODULE=1
	opam pin add hol_light ./hol-light --kind=local --confirm-level=yes
	opam reinstall hol_light --verbose --confirm-level=yes

clean:
	rm -rf *.cm* *.o *.wasm *.byte *.wat *.js
	rm -rf *.assets/

.PHONY: clean switch hol-light
