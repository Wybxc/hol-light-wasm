example.wat: example.wasm
	wasm-tools print $< > $@

example.wasm: example.js
	cp example.assets/code.wasm example.wasm

example.js: example.byte
	wasm_of_ocaml --enable wasi --enable exnref $< -o $@

example.byte: example.ml
	ocamlc -o $@ $<

opam:
	opam switch create . 5.2.0
	opam pin add js_of_ocaml git+https://github.com/ocsigen/js_of_ocaml.git#8c5a20c99bc46681f5a726ef41e5864cc298f159
	opam pin add js_of_ocaml-compiler git+https://github.com/ocsigen/js_of_ocaml.git#8c5a20c99bc46681f5a726ef41e5864cc298f159
	opam pin add wasm_of_ocaml-compiler git+https://github.com/ocsigen/js_of_ocaml.git#8c5a20c99bc46681f5a726ef41e5864cc298f159
	opam pin add hol_light ./hol-light/ --kind=local

clean:
	rm -rf *.cm* *.o *.wasm *.byte *.wat *.js
	rm -rf *.assets/

.PHONY: clean opam
